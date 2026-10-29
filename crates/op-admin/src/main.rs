//! Binário principal do servidor OpenPresenter (Rust + Axum).

mod online_lyrics;
mod routes;
mod session;
mod state;
mod static_files;
mod ws;

use std::net::SocketAddr;

use axum::Router;
use op_core::OutputsConfig;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    routes::create_routes, session::load_session, state::AppState,
    static_files::static_fallback_handler, ws::create_ws_router,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializa logs estruturados
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "open_presenter=info,tower_http=info".into()),
        )
        .init();

    info!("Iniciando {} v{}", op_core::APP_NAME, op_core::VERSION);

    // Carrega a sessão e a conexão SQLite
    let session = load_session();
    let outputs = if let Some(ref db) = session.db {
        db.load_outputs().unwrap_or_default()
    } else {
        OutputsConfig::default()
    };

    // Inicializa o estado compartilhado com o banco de dados
    let state = AppState::new(session.view, outputs, session.db);

    // Configura CORS permissivo para facilidade de desenvolvimento local
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Constrói o roteador principal
    let app = Router::new()
        .merge(create_routes(state.clone()))
        .merge(create_ws_router(state.clone()))
        .fallback(static_fallback_handler)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("\n╔════════════════════════════════════════════════════════════════════╗");
    println!("║       OpenPresenter Minimal — Servidor Web Ativo                   ║");
    println!("╠════════════════════════════════════════════════════════════════════╣");
    println!(
        "║  • Painel Admin:   http://localhost:{port:<5}                          ║",
        port = port
    );
    println!(
        "║  • Tela Pública:   http://localhost:{port:<5}/public                   ║",
        port = port
    );
    println!(
        "║  • Tela do Palco:  http://localhost:{port:<5}/stage                    ║",
        port = port
    );
    println!(
        "║  • API Status:     http://localhost:{port:<5}/api/status               ║",
        port = port
    );
    println!(
        "║  • WebSocket:      ws://localhost:{port:<5}/ws                         ║",
        port = port
    );
    println!("╚════════════════════════════════════════════════════════════════════╝\n");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    fn mock_state() -> AppState {
        let dummy_view = session::SessionView {
            app_version: "0.4.0".into(),
            tagline: "Projeção".into(),
            stack: "Rust + Axum".into(),
            db_path: ":memory:".into(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 2,
            theme_label: "Sistema".into(),
            locale: "pt-BR".into(),
            first_run_at: "agora".into(),
            last_run_at: "agora".into(),
            is_first_run: true,
        };

        AppState::new(dummy_view, OutputsConfig::default(), None)
    }

    #[tokio::test]
    async fn status_endpoint_returns_json() {
        let state = mock_state();
        let app = create_routes(state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/status")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn media_endpoint_returns_sample_media() {
        let state = mock_state();
        let app = create_routes(state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/media")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn live_slide_selection_and_blackout_work() {
        let state = mock_state();
        let app = create_routes(state.clone());

        // Dispara slide
        let payload = serde_json::to_string(&routes::SelectSlideRequest {
            media_id: "song-1".into(),
            slide_index: 1,
        })
        .unwrap();

        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/api/presentation/live")
                    .body(axum::body::Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let pres = state.presentation.read().await;
        assert_eq!(pres.active_media_id, Some("song-1".into()));
        assert_eq!(pres.active_slide_index, Some(1));
        assert_eq!(pres.display_mode, op_core::DisplayMode::Live);
        drop(pres);

        // Alterna blackout
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/presentation/blackout")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let pres = state.presentation.read().await;
        assert_eq!(pres.display_mode, op_core::DisplayMode::Blackout);
    }

    #[tokio::test]
    async fn fallback_serves_spa_routes() {
        let uri: axum::http::Uri = "/public".parse().unwrap();
        let resp = static_fallback_handler(uri).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    fn mock_state_with_db() -> (AppState, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = op_db::Database::open(&db_path).unwrap();
        let _ = db
            .begin_session(op_core::VERSION, chrono::Utc::now())
            .unwrap();

        let dummy_view = session::SessionView {
            app_version: "0.4.0".into(),
            tagline: "Projeção".into(),
            stack: "Rust + Axum".into(),
            db_path: db_path.display().to_string(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 3,
            theme_label: "Sistema".into(),
            locale: "pt-BR".into(),
            first_run_at: "agora".into(),
            last_run_at: "agora".into(),
            is_first_run: true,
        };

        (
            AppState::new(dummy_view, OutputsConfig::default(), Some(db)),
            dir,
        )
    }

    #[tokio::test]
    async fn save_song_and_search_endpoints_work() {
        let (state, _dir) = mock_state_with_db();
        let app = create_routes(state.clone());

        let song = op_core::Song {
            id: "song-test-1".into(),
            title: "Manso e Suave".into(),
            artist: "Will Thompson".into(),
            album: None,
            composer: None,
            song_key: Some("Ab".into()),
            bpm: Some(72),
            sections: vec![],
            lyrics_raw: "## Verso 1\nManso e suave Jesus tá chamando\n\n## Refrão\nVem já, vem já"
                .into(),
        };

        let payload = serde_json::to_string(&routes::SaveSongPayload {
            song: song.clone(),
            add_to_setlist: true,
        })
        .unwrap();

        let save_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/api/songs")
                    .body(axum::body::Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(save_resp.status(), StatusCode::CREATED);

        // Verifica se foi adicionado ao setlist
        let items = state.media_items.read().await;
        assert!(items.iter().any(|m| m.id == "song-test-1"));
        drop(items);

        // Busca via FTS5
        let search_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/songs?q=suave")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(search_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(search_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let results: Vec<op_core::SongSummary> = serde_json::from_slice(&body).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Manso e Suave");
    }
}

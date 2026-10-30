//! Binário principal do servidor OpenPresenter (Rust + Axum).

mod online_lyrics;
mod routes;
mod session;
mod state;
mod static_files;
mod tray;
mod updater;
mod ws;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, extract::DefaultBodyLimit};
use muda::MenuEvent;
use op_core::OutputsConfig;
use tao::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tray_icon::TrayIconEvent;

use crate::{
    routes::create_routes,
    session::load_session,
    state::AppState,
    static_files::static_fallback_handler,
    tray::{init_tray, prompt_change_port, show_native_alert},
    ws::create_ws_router,
};

#[derive(Debug)]
enum UserEvent {
    Tray(TrayIconEvent),
    Menu(MenuEvent),
}

fn resolve_server_port(db_opt: Option<&Arc<tokio::sync::Mutex<op_db::Database>>>) -> u16 {
    // 1. Variável de ambiente PORT
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        // 2. Configuração salva no banco SQLite
        .or_else(|| {
            db_opt.and_then(|db_mutex| {
                db_mutex
                    .try_lock()
                    .ok()?
                    .get_setting("server_port")
                    .ok()?
                    .and_then(|s| s.parse::<u16>().ok())
            })
        })
        // 3. Porta padrão exclusiva para evitar conflitos (18080)
        .unwrap_or(18080)
}

fn main() -> anyhow::Result<()> {
    // Inicializa logs estruturados
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "open_presenter=info,tower_http=info".into()),
        )
        .init();

    info!("Iniciando {} v{}", op_core::APP_NAME, op_core::VERSION);

    // Inicializa runtime Tokio multithread e entra no seu contexto
    let rt = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?,
    );
    let _guard = rt.enter();

    // Carrega a sessão e a conexão SQLite
    let session = load_session();
    let outputs = if let Some(ref db) = session.db {
        db.load_outputs().unwrap_or_default()
    } else {
        OutputsConfig::default()
    };

    let state = AppState::new(session.view, outputs, session.db);
    let db_ref = state.db.clone();
    let port = resolve_server_port(db_ref.as_ref());
    let args: Vec<String> = std::env::args().collect();
    let is_headless = args.iter().any(|a| a == "--headless" || a == "-h")
        || std::env::var("OPENPRESENTER_HEADLESS").is_ok();

    // Inicia o servidor Axum em background no runtime Tokio
    let state_for_server = state.clone();
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let server_rt = rt.clone();
    server_rt.spawn(async move {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = Router::new()
            .merge(create_routes(state_for_server.clone()))
            .merge(create_ws_router(state_for_server.clone()))
            .fallback(static_fallback_handler)
            .layer(cors)
            .layer(DefaultBodyLimit::max(128 * 1024 * 1024))
            .layer(TraceLayer::new_for_http());

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                println!(
                    "\n╔════════════════════════════════════════════════════════════════════╗"
                );
                println!(
                    "║       OpenPresenter Minimal v{:<5} — Servidor Web Ativo             ║",
                    op_core::VERSION
                );
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
                    "║  • Controle Remoto:http://localhost:{port:<5}/remote                   ║",
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
                println!(
                    "╚════════════════════════════════════════════════════════════════════╝\n"
                );

                let _ = axum::serve(
                    listener,
                    app.into_make_service_with_connect_info::<SocketAddr>(),
                )
                .with_graceful_shutdown(async move {
                    let _ = shutdown_rx.await;
                })
                .await;
            }
            Err(e) => {
                eprintln!("Erro ao vincular listener na porta {}: {}", port, e);
            }
        }
    });

    if is_headless {
        info!("Modo headless ativo. Pressione Ctrl+C para encerrar.");
        rt.block_on(async {
            let _ = tokio::signal::ctrl_c().await;
            info!("Sinal de interrupção recebido. Encerrando servidor.");
        });
        let _ = shutdown_tx.send(());
        return Ok(());
    }

    // Modo Desktop com System Tray (Menu Bar / Bandeja do Sistema)
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let proxy_tray = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        let _ = proxy_tray.send_event(UserEvent::Tray(event));
    }));

    let proxy_menu = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy_menu.send_event(UserEvent::Menu(event));
    }));

    let mut tray_holder = None;
    let mut opened_browser_on_start = false;
    let mut shutdown_tx_opt = Some(shutdown_tx);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                match init_tray(port) {
                    Ok(state) => {
                        tray_holder = Some(state);
                        info!("System Tray inicializado com sucesso.");
                    }
                    Err(e) => {
                        warn!("Não foi possível inicializar o System Tray: {}", e);
                    }
                }

                if !opened_browser_on_start {
                    opened_browser_on_start = true;
                    let url = format!("http://localhost:{}", port);
                    let _ = open::that(&url);
                }
            }

            Event::UserEvent(UserEvent::Menu(menu_event)) => {
                if let Some(ref tray_st) = tray_holder {
                    let ids = &tray_st.menu_ids;

                    if menu_event.id == ids.open_admin {
                        let _ = open::that(format!("http://localhost:{}", port));
                    } else if menu_event.id == ids.open_public {
                        let _ = open::that(format!("http://localhost:{}/public", port));
                    } else if menu_event.id == ids.open_stage {
                        let _ = open::that(format!("http://localhost:{}/stage", port));
                    } else if menu_event.id == ids.open_remote {
                        let _ = open::that(format!("http://localhost:{}/remote", port));
                    } else if menu_event.id == ids.change_port {
                        if let Some(new_p) = prompt_change_port(port).filter(|&p| p != port) {
                            if let Some(db) = db_ref.as_ref().and_then(|m| m.try_lock().ok()) {
                                let _ = db.set_setting("server_port", &new_p.to_string());
                            }
                            show_native_alert(
                                "Porta Configurada",
                                &format!(
                                    "A porta {} foi salva com sucesso. Reinicie o OpenPresenter para aplicar a nova porta.",
                                    new_p
                                ),
                            );
                        }
                    } else if menu_event.id == ids.quit {
                        info!("Encerrando OpenPresenter via menu do System Tray.");
                        if let Some(tx) = shutdown_tx_opt.take() {
                            let _ = tx.send(());
                        }
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }

            Event::UserEvent(UserEvent::Tray(_tray_event)) => {
                // Eventos de interação ou foco no ícone da bandeja não reabrem o navegador;
                // a abertura sob demanda é feita pelo menu do tray ("Abrir Painel de Controle")
                // ou automaticamente apenas uma única vez na inicialização da aplicação.
            }

            _ => {}
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    fn mock_state() -> AppState {
        let dummy_view = session::SessionView {
            app_version: op_core::VERSION.into(),
            tagline: "Projeção".into(),
            stack: "Rust + Axum".into(),
            db_path: ":memory:".into(),
            media_path: "/dummy/media".into(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 5,
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
            app_version: op_core::VERSION.into(),
            tagline: "Projeção".into(),
            stack: "Rust + Axum".into(),
            db_path: db_path.display().to_string(),
            media_path: dir.path().join("media").display().to_string(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 5,
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

    #[tokio::test]
    async fn system_paths_and_media_folders_work() {
        let (state, dir) = mock_state_with_db();
        let app = create_routes(state);

        // Testa GET /api/system/paths
        let paths_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/system/paths")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(paths_resp.status(), StatusCode::OK);

        // Testa GET /api/media/folders
        let folders_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/media/folders")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(folders_resp.status(), StatusCode::OK);

        // Testa POST /api/media/folders com diretório do tempdir
        let sample_dir = dir.path().join("imagens-culto");
        let sub_dir = sample_dir.join("avisos");
        std::fs::create_dir_all(&sub_dir).unwrap();
        std::fs::write(sub_dir.join("slide1.png"), b"fake-png-data").unwrap();

        let post_folder_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/media/folders")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(
                        serde_json::to_string(&serde_json::json!({
                            "name": "Culto Teste",
                            "path": sample_dir.display().to_string(),
                            "kind": "images"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(post_folder_resp.status(), StatusCode::OK);

        // Verifica se sub-pastas e imagens aninhadas são descobertas
        let folders_list_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/media/folders")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let folders_bytes = axum::body::to_bytes(folders_list_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let folders_json: Vec<op_core::MediaFolder> =
            serde_json::from_slice(&folders_bytes).unwrap();
        assert_eq!(folders_json.len(), 1);
        assert_eq!(folders_json[0].subfolders.len(), 1);
        assert_eq!(folders_json[0].subfolders[0].name, "avisos");
        assert_eq!(folders_json[0].subfolders[0].count, 1);

        // Testa GET /api/media/images com imagem na sub-pasta
        let images_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/media/images")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(images_resp.status(), StatusCode::OK);
        let img_bytes = axum::body::to_bytes(images_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let img_json: serde_json::Value = serde_json::from_slice(&img_bytes).unwrap();
        let arr = img_json.as_array().unwrap();
        let found = arr.iter().find(|i| i["name"] == "slide1.png");
        assert!(found.is_some());
        assert_eq!(found.unwrap()["subfolder"], "avisos");

        // Testa POST /api/system/open-path com caminho inexistente (retorna 404)
        let open_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/system/open-path")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(
                        serde_json::to_string(&serde_json::json!({
                            "path": "/caminho/completamente/invalido/para/teste"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(open_resp.status(), StatusCode::NOT_FOUND);

        // Testa POST /api/system/open-path com caminho existente
        let open_valid_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/system/open-path")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(
                        serde_json::to_string(&serde_json::json!({
                            "path": sample_dir.display().to_string()
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(open_valid_resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn stop_presentation_and_set_wallpaper_work() {
        let state = mock_state();
        let app = create_routes(state.clone());

        // Inicia slide
        let payload = serde_json::to_string(&routes::SelectSlideRequest {
            media_id: "song-1".into(),
            slide_index: 0,
        })
        .unwrap();

        let _ = app
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

        assert_eq!(
            state.presentation.read().await.active_media_id,
            Some("song-1".into())
        );

        // Altera wallpaper
        let wall_payload = serde_json::to_string(&serde_json::json!({
            "path": "linear-gradient(135deg, #1e1b4b, #0f172a)",
            "mode": "cover"
        }))
        .unwrap();

        let wall_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/api/presentation/wallpaper")
                    .body(axum::body::Body::from(wall_payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(wall_resp.status(), StatusCode::OK);
        assert_eq!(
            state.presentation.read().await.wallpaper_path,
            Some("linear-gradient(135deg, #1e1b4b, #0f172a)".into())
        );

        // Para a apresentação
        let stop_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/presentation/stop")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(stop_resp.status(), StatusCode::OK);

        let pres = state.presentation.read().await;
        assert_eq!(pres.active_media_id, None);
        assert_eq!(pres.active_slide_index, None);
        // Ao parar, o wallpaper deve retornar para o setlist_wallpaper_path (repouso)
        assert_eq!(pres.wallpaper_path, pres.setlist_wallpaper_path);
    }

    #[tokio::test]
    async fn timer_and_app_settings_endpoints_work() {
        let (state, _dir) = mock_state_with_db();
        let app = create_routes(state.clone());

        // GET /api/timer
        let timer_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/timer")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(timer_resp.status(), StatusCode::OK);

        // POST /api/timer/set
        let set_timer_payload = serde_json::to_string(&serde_json::json!({
            "duration_seconds": 300,
            "label": "Pregação"
        }))
        .unwrap();

        let set_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/timer/set")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(set_timer_payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(set_resp.status(), StatusCode::OK);
        assert_eq!(state.presentation.read().await.timer.duration_seconds, 300);

        // POST /api/timer/start
        let start_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/timer/start")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(start_resp.status(), StatusCode::OK);
        assert!(state.presentation.read().await.timer.running);

        // POST /api/timer/pause
        let pause_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/timer/pause")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(pause_resp.status(), StatusCode::OK);
        assert!(!state.presentation.read().await.timer.running);

        // POST /api/settings/public_display
        let setting_payload = serde_json::to_string(&serde_json::json!({
            "value": "{\"fill_space\": true, \"word_wrap\": false}"
        }))
        .unwrap();

        let save_setting_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/settings/public_display")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(setting_payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(save_setting_resp.status(), StatusCode::OK);

        // GET /api/settings/public_display
        let get_setting_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/settings/public_display")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_setting_resp.status(), StatusCode::OK);

        // GET /api/system/network-info
        let net_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/system/network-info")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(net_resp.status(), StatusCode::OK);

        // POST /api/outputs/reload
        let mut rx = state.broadcast_tx.subscribe();
        let reload_payload = serde_json::to_string(&serde_json::json!({
            "target": "public"
        }))
        .unwrap();

        let reload_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/outputs/reload")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(reload_payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(reload_resp.status(), StatusCode::OK);

        let ws_msg = rx.recv().await.unwrap();
        assert_eq!(
            ws_msg,
            op_core::WsMessage::ReloadClients {
                target: Some("public".into())
            }
        );

        // POST /api/presentation/quick-media
        let quick_payload = serde_json::to_string(&serde_json::json!({
            "item": {
                "id": "quick-img-1",
                "title": "Imagem de Teste",
                "kind": "image",
                "slides": [{
                    "id": "s-1",
                    "label": "Slide 1",
                    "content": "/media/images/teste.jpg"
                }]
            }
        }))
        .unwrap();

        let quick_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/presentation/quick-media")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(quick_payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(quick_resp.status(), StatusCode::OK);
        assert_eq!(
            state.presentation.read().await.active_media_id.as_deref(),
            Some("quick-img-1")
        );

        // Timer Stop at Zero
        state
            .set_timer(1, Some("Teste Zero".into()), Some(false))
            .await;
        state.start_timer().await;
        assert!(state.presentation.read().await.timer.running);
        state.tick_timer().await;
        let pres = state.presentation.read().await;
        assert_eq!(pres.timer.remaining_seconds, 0);
        assert!(!pres.timer.running); // Parou no zero!
    }

    #[tokio::test]
    async fn bibles_endpoints_and_parse_work() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("open-presenter.db");
        let db = op_db::Database::open(&db_path).unwrap();

        // Cria arquivo de bíblia SQLite de teste
        let test_bible_path = dir.path().join("ARA.sqlite");
        {
            let conn = rusqlite::Connection::open(&test_bible_path).unwrap();
            conn.execute_batch(
                r#"
                CREATE TABLE metadata (key VARCHAR(255) PRIMARY KEY, value VARCHAR(255));
                INSERT INTO metadata VALUES ('name', 'Almeida Revista e Atualizada');
                CREATE TABLE book (id INTEGER PRIMARY KEY, book_reference_id INTEGER, testament_reference_id INTEGER, name VARCHAR(50));
                INSERT INTO book VALUES (43, 43, 2, 'João');
                CREATE TABLE verse (id INTEGER PRIMARY KEY, book_id INTEGER, chapter INTEGER, verse INTEGER, text TEXT);
                INSERT INTO verse VALUES (1, 43, 3, 16, 'Porque Deus amou o mundo');
                "#,
            )
            .unwrap();
        }

        let dummy_view = crate::session::SessionView {
            app_version: op_core::VERSION.to_string(),
            tagline: op_core::TAGLINE.to_string(),
            stack: op_core::STACK.to_string(),
            db_path: db_path.to_string_lossy().to_string(),
            media_path: dir.path().join("media").to_string_lossy().to_string(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 7,
            theme_label: "Sistema".into(),
            locale: "pt-BR".into(),
            first_run_at: "agora".into(),
            last_run_at: "agora".into(),
            is_first_run: true,
        };

        unsafe {
            std::env::set_var("OPENPRESENTER_DATA_DIR", dir.path());
        }

        let state = AppState::new(dummy_view, OutputsConfig::default(), Some(db));
        let app = create_routes(state);

        // GET /api/bibles (inicialmente vazio)
        let resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/bibles")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // POST /api/bibles/import com source_path
        let import_payload = serde_json::to_string(&serde_json::json!({
            "name": "Almeida Revista e Atualizada",
            "abbreviation": "ARA",
            "source_path": test_bible_path.to_string_lossy().to_string(),
            "filename": "ARA.sqlite"
        }))
        .unwrap();

        let import_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/bibles/import")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(import_payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(import_resp.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(import_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let imported: op_core::BibleVersionSummary = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(imported.abbreviation, "ARA");
        assert!(imported.is_default);

        // GET /api/bibles/:id/books
        let books_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri(format!("/api/bibles/{}/books", imported.id))
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(books_resp.status(), StatusCode::OK);

        // GET /api/bibles/parse-reference?q=joao+3.16
        let parse_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/bibles/parse-reference?q=joao+3.16")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(parse_resp.status(), StatusCode::OK);

        // GET /api/bibles/parse-reference?q=Salmos+91+ARA (com versão)
        let parse_v_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/bibles/parse-reference?q=Salmos+91+ARA")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(parse_v_resp.status(), StatusCode::OK);
        let parse_v_bytes = axum::body::to_bytes(parse_v_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let parsed_ref: Option<op_core::ParsedReference> =
            serde_json::from_slice(&parse_v_bytes).unwrap();
        assert_eq!(parsed_ref.unwrap().version, Some("ARA".to_string()));

        // GET /api/bibles/:id/search?q="amou" (busca com aspas)
        let search_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .uri(format!("/api/bibles/{}/search?q=%22amou%22", imported.id))
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(search_resp.status(), StatusCode::OK);
        let search_bytes = axum::body::to_bytes(search_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let search_hits: Vec<op_core::BibleSearchResult> =
            serde_json::from_slice(&search_bytes).unwrap();
        assert_eq!(search_hits.len(), 1);
        assert_eq!(search_hits[0].verse, 16);
    }

    fn encode_base64_test(bytes: &[u8]) -> String {
        const TABLE: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
        for chunk in bytes.chunks(3) {
            let b0 = chunk[0];
            let b1 = chunk.get(1).copied().unwrap_or(0);
            let b2 = chunk.get(2).copied().unwrap_or(0);
            out.push(TABLE[(b0 >> 2) as usize] as char);
            out.push(TABLE[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
            if chunk.len() > 1 {
                out.push(TABLE[(((b1 & 15) << 2) | (b2 >> 6)) as usize] as char);
            } else {
                out.push('=');
            }
            if chunk.len() > 2 {
                out.push(TABLE[(b2 & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
        out
    }

    #[tokio::test]
    async fn large_bible_base64_import_works_exceeding_default_2mb_limit() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test_large_bible.db");
        let db = op_db::Database::open(&db_path).unwrap();
        let _ = db
            .begin_session(op_core::VERSION, chrono::Utc::now())
            .unwrap();

        let dummy_view = session::SessionView {
            app_version: op_core::VERSION.into(),
            tagline: "Projeção".into(),
            stack: "Rust + Axum".into(),
            db_path: db_path.to_string_lossy().to_string(),
            media_path: dir.path().join("media").to_string_lossy().to_string(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 7,
            theme_label: "Sistema".into(),
            locale: "pt-BR".into(),
            first_run_at: "2026-10-29T10:00:00.000Z".into(),
            last_run_at: "2026-10-29T10:00:00.000Z".into(),
            is_first_run: true,
        };

        unsafe {
            std::env::set_var("OPENPRESENTER_DATA_DIR", dir.path());
        }

        let state = AppState::new(dummy_view, OutputsConfig::default(), Some(db));
        let app = create_routes(state);

        let test_bible_path = dir.path().join("Large_Bible.sqlite");

        // Se o arquivo de teste real do usuário estiver presente, usamos ele diretamente (4.5MB)
        let bytes = if let Ok(real_bytes) =
            std::fs::read("/Users/claudio/Downloads/bibles_ARA.sqlite")
        {
            real_bytes
        } else {
            let conn = rusqlite::Connection::open(&test_bible_path).unwrap();
            conn.execute_batch(
                    r#"
                    CREATE TABLE metadata (key VARCHAR(50) PRIMARY KEY, value TEXT);
                    INSERT INTO metadata VALUES ('name', 'Bíblia de Teste Grande');
                    CREATE TABLE book (id INTEGER PRIMARY KEY, book_reference_id INTEGER, testament_reference_id INTEGER, name VARCHAR(50));
                    INSERT INTO book VALUES (1, 1, 1, 'Gênesis');
                    CREATE TABLE verse (id INTEGER PRIMARY KEY, book_id INTEGER, chapter INTEGER, verse INTEGER, text TEXT);
                    INSERT INTO verse VALUES (1, 1, 1, 1, 'No princípio...');
                    CREATE TABLE padding (id INTEGER PRIMARY KEY, content TEXT);
                    "#,
                )
                .unwrap();
            let chunk = "A".repeat(100_000);
            for i in 0..26 {
                conn.execute(
                    "INSERT INTO padding (id, content) VALUES (?1, ?2)",
                    rusqlite::params![i, chunk],
                )
                .unwrap();
            }
            drop(conn);
            std::fs::read(&test_bible_path).unwrap()
        };

        assert!(
            bytes.len() > 2_500_000,
            "Arquivo deve ter mais de 2.5MB para testar limite do Axum"
        );
        let b64 = encode_base64_test(&bytes);

        let import_payload = serde_json::to_string(&serde_json::json!({
            "name": "Bíblia Grande Importada",
            "abbreviation": "BGI",
            "content_base64": b64,
            "filename": "Large_Bible.sqlite"
        }))
        .unwrap();

        let import_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/bibles/import")
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(import_payload))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(import_resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn clear_setlist_items_works() {
        let (state, _dir) = mock_state_with_db();
        let app = create_routes(state.clone());

        let initial_len = state.media_items.read().await.len();
        assert!(initial_len > 0);

        // Adiciona um item na setlist
        let item = op_core::MediaItem {
            id: "item-to-clear".into(),
            title: "Hino da Vitória".into(),
            artist_or_ref: None,
            kind: op_core::MediaKind::Song,
            slides: vec![op_core::Slide {
                id: "slide-1".into(),
                label: "Verso 1".into(),
                content: "Quem é que pode impedir".into(),
                align_h: None,
                align_v: None,
            }],
            align_h: None,
            align_v: None,
        };
        state.add_media_item(item).await;
        assert_eq!(state.media_items.read().await.len(), initial_len + 1);

        // Dispara live do item
        state.select_slide("item-to-clear".into(), 0).await;
        assert_eq!(
            state.presentation.read().await.active_media_id.as_deref(),
            Some("item-to-clear")
        );

        // Chama POST /api/setlists/items/clear
        let clear_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/setlists/items/clear")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(clear_resp.status(), StatusCode::OK);
        assert_eq!(state.media_items.read().await.len(), 0);
        assert!(state.presentation.read().await.active_media_id.is_none());
    }

    #[tokio::test]
    async fn test_updater_endpoints_work() {
        let dummy_view = crate::session::SessionView {
            app_version: op_core::VERSION.to_string(),
            tagline: op_core::TAGLINE.to_string(),
            stack: op_core::STACK.to_string(),
            db_path: ":memory:".into(),
            media_path: "/tmp/media".into(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 7,
            theme_label: "Sistema".into(),
            locale: "pt-BR".into(),
            first_run_at: "agora".into(),
            last_run_at: "agora".into(),
            is_first_run: true,
        };

        let state = AppState::new(dummy_view, OutputsConfig::default(), None);
        let app = create_routes(state.clone());

        // 1. GET /api/updater/status inicial deve ser Idle
        let status_resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/updater/status")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(status_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(status_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let status_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(status_json["stage"]["type"], "Idle");

        // 2. POST /api/updater/install sem asset_name e download_url deve retornar 400 Bad Request
        let install_resp = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/updater/install")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(r#"{}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(install_resp.status(), StatusCode::BAD_REQUEST);

        // 3. Atualizando o estado do updater em memória
        {
            let mut st = state.updater.write().await;
            st.stage = crate::updater::UpdateStage::ReadyToRestart {
                new_version: "0.2.0".into(),
            };
        }

        let updated_resp = create_routes(state.clone())
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/updater/status")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(updated_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(updated_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let status_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(status_json["stage"]["type"], "ReadyToRestart");
        assert_eq!(status_json["stage"]["new_version"], "0.2.0");
    }

    #[tokio::test]
    async fn test_connections_endpoint_and_state_work() {
        let dummy_view = crate::session::SessionView {
            app_version: op_core::VERSION.to_string(),
            tagline: op_core::TAGLINE.to_string(),
            stack: op_core::STACK.to_string(),
            db_path: ":memory:".into(),
            media_path: "/tmp/media".into(),
            db_ready: true,
            db_error: String::new(),
            schema_version: 7,
            theme_label: "Sistema".into(),
            locale: "pt-BR".into(),
            first_run_at: "agora".into(),
            last_run_at: "agora".into(),
            is_first_run: true,
        };

        let state = AppState::new(dummy_view, OutputsConfig::default(), None);
        let app = create_routes(state.clone());

        // Inicialmente vazio
        let resp = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/connections")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let conns: Vec<op_core::ClientConnection> = serde_json::from_slice(&body).unwrap();
        assert!(conns.is_empty());

        // Adiciona conexões
        state
            .add_or_update_connection("c1", "192.168.1.50", "/public", "public")
            .await;
        state
            .add_or_update_connection("c2", "192.168.1.55", "/stage", "stage")
            .await;

        let resp2 = create_routes(state.clone())
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/connections")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body2 = axum::body::to_bytes(resp2.into_body(), usize::MAX)
            .await
            .unwrap();
        let conns2: Vec<op_core::ClientConnection> = serde_json::from_slice(&body2).unwrap();
        assert_eq!(conns2.len(), 2);
        assert_eq!(conns2[0].client_type, "public");
        assert_eq!(conns2[0].ip, "192.168.1.50");
        assert_eq!(conns2[0].route, "/public");

        // Remove c1
        state.remove_connection("c1").await;
        let remaining = state.get_connections().await;
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "c2");
    }
}

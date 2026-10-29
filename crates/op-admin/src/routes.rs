use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use op_core::{
    DisplayMode, MediaItem, OnlineLyricHit, OutputKind, OutputsConfig, PresentationState, Song,
    SongSummary, VERSION,
};
use serde::{Deserialize, Serialize};

use crate::{session::SessionView, state::AppState};

#[derive(Serialize)]
pub struct StatusResponse {
    pub app_name: &'static str,
    pub version: &'static str,
    pub session: SessionView,
    pub presentation: PresentationState,
    pub outputs: OutputsConfig,
}

#[derive(Serialize, Deserialize)]
pub struct SelectSlideRequest {
    pub media_id: String,
    pub slide_index: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SetModeRequest {
    pub mode: DisplayMode,
}

#[derive(Deserialize)]
pub struct SongQuery {
    pub q: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveSongPayload {
    pub song: Song,
    #[serde(default)]
    pub add_to_setlist: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OnlineSearchQuery {
    pub q: String,
}

#[derive(Serialize, Deserialize)]
pub struct OnlineFetchQuery {
    pub provider: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddToSetlistPayload {
    pub song_id: Option<String>,
    pub item: Option<MediaItem>,
}

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/status", get(get_status))
        .route("/api/media", get(get_media))
        .route("/api/presentation/live", post(select_live_slide))
        .route("/api/presentation/mode", post(set_display_mode))
        .route("/api/presentation/blackout", post(toggle_blackout))
        .route("/api/presentation/clear", post(toggle_clear_text))
        .route("/api/outputs/{kind}/toggle", post(toggle_output))
        .route("/api/songs", get(get_songs).post(save_song))
        .route("/api/songs/{id}", get(get_song))
        .route("/api/songs/online/search", get(search_online_lyrics))
        .route("/api/songs/online/fetch", get(fetch_online_lyrics))
        .route("/api/setlist/add", post(add_to_setlist))
        .with_state(state)
}

async fn get_status(State(state): State<AppState>) -> Json<StatusResponse> {
    let presentation = state.presentation.read().await.clone();
    let outputs = state.outputs.read().await.clone();

    Json(StatusResponse {
        app_name: op_core::APP_NAME,
        version: VERSION,
        session: state.session.clone(),
        presentation,
        outputs,
    })
}

async fn get_media(State(state): State<AppState>) -> Json<Vec<MediaItem>> {
    let items = state.media_items.read().await.clone();
    Json(items)
}

async fn select_live_slide(
    State(state): State<AppState>,
    Json(payload): Json<SelectSlideRequest>,
) -> Json<PresentationState> {
    let updated = state
        .select_slide(payload.media_id, payload.slide_index)
        .await;
    Json(updated)
}

async fn set_display_mode(
    State(state): State<AppState>,
    Json(payload): Json<SetModeRequest>,
) -> Json<PresentationState> {
    let updated = state.set_display_mode(payload.mode).await;
    Json(updated)
}

async fn toggle_blackout(State(state): State<AppState>) -> Json<PresentationState> {
    let updated = state.toggle_blackout().await;
    Json(updated)
}

async fn toggle_clear_text(State(state): State<AppState>) -> Json<PresentationState> {
    let updated = state.toggle_clear_text().await;
    Json(updated)
}

async fn toggle_output(
    State(state): State<AppState>,
    Path(kind): Path<String>,
) -> Result<Json<OutputsConfig>, impl IntoResponse> {
    let output_kind = match OutputKind::from_db_str(&kind) {
        Some(k) => k,
        None => return Err((StatusCode::BAD_REQUEST, "Tipo de saída inválido")),
    };

    let mut outputs = state.outputs.write().await;
    let cfg = outputs.get_mut(output_kind);
    cfg.enabled = !cfg.enabled;
    let updated = outputs.clone();

    Ok(Json(updated))
}

async fn get_songs(
    State(state): State<AppState>,
    Query(query): Query<SongQuery>,
) -> Json<Vec<SongSummary>> {
    let q = query.q.unwrap_or_default();
    let songs = if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        db_guard.search_songs(&q).unwrap_or_default()
    } else {
        Vec::new()
    };
    Json(songs)
}

async fn get_song(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Song>, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let db_guard = db.lock().await;
    match db_guard.get_song(&id) {
        Ok(Some(song)) => Ok(Json(song)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Música não encontrada".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn save_song(
    State(state): State<AppState>,
    Json(mut payload): Json<SaveSongPayload>,
) -> Result<(StatusCode, Json<Song>), (StatusCode, String)> {
    if payload.song.sections.is_empty() {
        let (parsed_sections, _) = op_core::parse_raw_lyrics(&payload.song.lyrics_raw);
        payload.song.sections = parsed_sections;
    }

    if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        db_guard
            .save_song(&payload.song, chrono::Utc::now())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    if payload.add_to_setlist {
        let media_item = payload.song.clone().into_media_item();
        state.add_media_item(media_item).await;
    }

    Ok((StatusCode::CREATED, Json(payload.song)))
}

async fn search_online_lyrics(
    State(state): State<AppState>,
    Query(query): Query<OnlineSearchQuery>,
) -> Json<Vec<OnlineLyricHit>> {
    let hits = crate::online_lyrics::search_online(&state.http_client, &query.q).await;
    Json(hits)
}

async fn fetch_online_lyrics(
    State(state): State<AppState>,
    Query(query): Query<OnlineFetchQuery>,
) -> Result<Json<Song>, (StatusCode, String)> {
    match crate::online_lyrics::fetch_online_song(&state.http_client, &query.provider, &query.id)
        .await
    {
        Ok(fetched) => Ok(Json(fetched.into_song())),
        Err(e) => Err((StatusCode::BAD_GATEWAY, e.to_string())),
    }
}

async fn add_to_setlist(
    State(state): State<AppState>,
    Json(payload): Json<AddToSetlistPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Some(item) = payload.item {
        state.add_media_item(item).await;
        return Ok(StatusCode::OK);
    }

    if let Some(song_id) = payload.song_id {
        let db = state.db.as_ref().ok_or((
            StatusCode::SERVICE_UNAVAILABLE,
            "Banco de dados indisponível".into(),
        ))?;

        let db_guard = db.lock().await;
        match db_guard.get_song(&song_id) {
            Ok(Some(song)) => {
                let media_item = song.into_media_item();
                drop(db_guard);
                state.add_media_item(media_item).await;
                return Ok(StatusCode::OK);
            }
            Ok(None) => return Err((StatusCode::NOT_FOUND, "Música não encontrada".into())),
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        "Parâmetro 'song_id' ou 'item' obrigatório".into(),
    ))
}

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use op_core::{DisplayMode, MediaItem, OutputKind, OutputsConfig, PresentationState, VERSION};
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

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/status", get(get_status))
        .route("/api/media", get(get_media))
        .route("/api/presentation/live", post(select_live_slide))
        .route("/api/presentation/mode", post(set_display_mode))
        .route("/api/presentation/blackout", post(toggle_blackout))
        .route("/api/presentation/clear", post(toggle_clear_text))
        .route("/api/outputs/{kind}/toggle", post(toggle_output))
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

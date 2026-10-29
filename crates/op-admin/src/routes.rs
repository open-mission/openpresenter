use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use op_core::{
    DisplayMode, MediaItem, OnlineLyricHit, OutputKind, OutputsConfig, PresentationState, Setlist,
    SetlistSummary, Song, SongSummary, VERSION,
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

#[derive(Serialize, Deserialize)]
pub struct RemoveItemPayload {
    pub item_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReorderItemsPayload {
    pub items: Vec<MediaItem>,
}

#[derive(Serialize, Deserialize)]
pub struct RenameItemPayload {
    pub item_id: String,
    pub new_title: String,
}

#[derive(Serialize, Deserialize)]
pub struct UploadMediaPayload {
    pub filename: String,
    pub content_base64: String,
}

#[derive(Serialize, Deserialize)]
pub struct UploadMediaResponse {
    pub url: String,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct WallpaperItem {
    pub name: String,
    pub url: String,
    pub is_default: bool,
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
        .route("/api/setlists", get(list_setlists).post(save_setlist))
        .route(
            "/api/setlists/{id}",
            get(get_setlist).delete(delete_setlist),
        )
        .route("/api/setlists/{id}/active", post(activate_setlist))
        .route("/api/setlists/items/remove", post(remove_setlist_item))
        .route("/api/setlists/items/reorder", post(reorder_setlist_items))
        .route("/api/setlists/items/rename", post(rename_setlist_item))
        .route("/api/media/wallpapers", get(list_wallpapers))
        .route("/api/media/upload", post(upload_media))
        .route("/api/media/files/{*subpath}", get(serve_media_file))
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

async fn list_setlists(State(state): State<AppState>) -> Json<Vec<SetlistSummary>> {
    let db = match state.db.as_ref() {
        Some(db) => db,
        None => return Json(Vec::new()),
    };
    let db_guard = db.lock().await;
    let list = db_guard.list_setlists().unwrap_or_default();
    Json(list)
}

async fn save_setlist(
    State(state): State<AppState>,
    Json(setlist): Json<Setlist>,
) -> Result<(StatusCode, Json<Setlist>), (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;
    let mut db_guard = db.lock().await;
    if let Err(e) = db_guard.save_setlist(&setlist) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }
    drop(db_guard);

    if setlist.is_active {
        state.switch_active_setlist(&setlist.id).await;
    }

    Ok((StatusCode::OK, Json(setlist)))
}

async fn get_setlist(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Setlist>, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;
    let db_guard = db.lock().await;
    match db_guard.get_setlist(&id) {
        Ok(Some(s)) => Ok(Json(s)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Setlist não encontrada".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_setlist(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;
    let mut db_guard = db.lock().await;
    match db_guard.delete_setlist(&id) {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((StatusCode::NOT_FOUND, "Setlist não encontrada".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn activate_setlist(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Setlist>, (StatusCode, String)> {
    match state.switch_active_setlist(&id).await {
        Some(s) => Ok(Json(s)),
        None => Err((StatusCode::NOT_FOUND, "Setlist não encontrada".into())),
    }
}

async fn remove_setlist_item(
    State(state): State<AppState>,
    Json(payload): Json<RemoveItemPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    if state.remove_media_item(&payload.item_id).await {
        Ok(StatusCode::OK)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "Item não encontrado na setlist".into(),
        ))
    }
}

async fn reorder_setlist_items(
    State(state): State<AppState>,
    Json(payload): Json<ReorderItemsPayload>,
) -> StatusCode {
    state.reorder_media_items(payload.items).await;
    StatusCode::OK
}

async fn rename_setlist_item(
    State(state): State<AppState>,
    Json(payload): Json<RenameItemPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    if state
        .rename_media_item(&payload.item_id, &payload.new_title)
        .await
    {
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "Item não encontrado".into()))
    }
}

async fn list_wallpapers() -> Json<Vec<WallpaperItem>> {
    let mut items = Vec::new();
    if let Ok(media_dir) = crate::session::resolve_media_dir() {
        let wall_dir = media_dir.join("wallpapers");
        if let Ok(entries) = std::fs::read_dir(wall_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && let Some(name) = path.file_name().and_then(|n| n.to_str())
                {
                    let lower = name.to_lowercase();
                    if lower.ends_with(".jpg")
                        || lower.ends_with(".jpeg")
                        || lower.ends_with(".png")
                        || lower.ends_with(".webp")
                        || lower.ends_with(".svg")
                    {
                        items.push(WallpaperItem {
                            name: name.to_string(),
                            url: format!("/api/media/files/wallpapers/{name}"),
                            is_default: name == "dark-ambient.svg"
                                || name == "worship-glow.svg"
                                || name == "cross-subtle.svg",
                        });
                    }
                }
            }
        }
    }
    items.sort_by(|a, b| b.is_default.cmp(&a.is_default).then(a.name.cmp(&b.name)));
    Json(items)
}

async fn upload_media(
    Json(payload): Json<UploadMediaPayload>,
) -> Result<Json<UploadMediaResponse>, (StatusCode, String)> {
    let media_dir = crate::session::resolve_media_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let wallpapers_dir = media_dir.join("wallpapers");
    let _ = std::fs::create_dir_all(&wallpapers_dir);

    let bytes = decode_base64(&payload.content_base64)
        .ok_or((StatusCode::BAD_REQUEST, "Base64 inválido".into()))?;

    let raw_name = std::path::Path::new(&payload.filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("wallpaper.png");
    let sanitized_name: String = raw_name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    let target_path = wallpapers_dir.join(&sanitized_name);
    tokio::fs::write(&target_path, bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UploadMediaResponse {
        url: format!("/api/media/files/wallpapers/{sanitized_name}"),
        filename: sanitized_name,
    }))
}

async fn serve_media_file(Path(subpath): Path<String>) -> impl IntoResponse {
    if subpath.contains("..") || subpath.starts_with('/') {
        return (StatusCode::BAD_REQUEST, "Caminho inválido").into_response();
    }

    let media_dir = match crate::session::resolve_media_dir() {
        Ok(dir) => dir,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao resolver pasta de mídias",
            )
                .into_response();
        }
    };

    let file_path = media_dir.join(&subpath);
    if !file_path.exists() || !file_path.is_file() {
        return (StatusCode::NOT_FOUND, "Arquivo não encontrado").into_response();
    }

    let content_type = mime_guess::from_path(&file_path)
        .first_or_octet_stream()
        .to_string();

    match tokio::fs::read(&file_path).await {
        Ok(bytes) => (
            StatusCode::OK,
            [
                (axum::http::header::CONTENT_TYPE, content_type),
                (
                    axum::http::header::CACHE_CONTROL,
                    "public, max-age=3600".into(),
                ),
            ],
            bytes,
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao ler arquivo").into_response(),
    }
}

fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let clean = if let Some(idx) = input.find(";base64,") {
        &input[idx + 8..]
    } else {
        input.trim()
    };
    let clean = clean.replace(['\r', '\n', ' '], "");
    let mut out = Vec::with_capacity((clean.len() * 3) / 4);
    let mut buf: u32 = 0;
    let mut bits = 0;

    for b in clean.bytes() {
        let val = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => continue,
        } as u32;

        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    Some(out)
}

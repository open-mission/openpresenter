use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use op_core::{
    BibleBook, BibleSearchResult, BibleVerse, BibleVersionSummary, DisplayMode, MediaItem,
    OnlineLyricHit, OutputKind, OutputsConfig, ParsedReference, PresentationState, Setlist,
    SetlistSummary, Song, SongSummary, VERSION, WsMessage,
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
    #[serde(default)]
    pub is_video: bool,
}

#[derive(Serialize)]
pub struct SystemPathsResponse {
    pub data_dir: String,
    pub db_path: String,
    pub images_path: String,
    pub wallpapers_path: String,
    pub videos_path: String,
    pub fonts_path: String,
}

#[derive(Deserialize)]
pub struct CreateMediaFolderPayload {
    pub name: String,
    pub path: String,
    pub kind: Option<String>,
}

#[derive(Serialize)]
pub struct ImageMediaItem {
    pub id: String,
    pub name: String,
    pub path: String,
    pub folder_id: String,
    pub folder_name: String,
    #[serde(default)]
    pub subfolder: Option<String>,
    pub url: String,
    pub is_external: bool,
    pub size_bytes: u64,
}

#[derive(Deserialize)]
pub struct ExternalFileQuery {
    pub folder_id: String,
    pub file: String,
}

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/status", get(get_status))
        .route("/api/system/paths", get(get_system_paths))
        .route("/api/system/pick-folder", post(pick_folder))
        .route("/api/system/open-path", post(open_system_path))
        .route("/api/media", get(get_media))
        .route("/api/presentation/live", post(select_live_slide))
        .route("/api/presentation/mode", post(set_display_mode))
        .route("/api/presentation/blackout", post(toggle_blackout))
        .route("/api/presentation/clear", post(toggle_clear_text))
        .route("/api/presentation/stop", post(stop_presentation))
        .route("/api/presentation/quick-media", post(quick_present_media))
        .route(
            "/api/presentation/wallpaper",
            post(set_presentation_wallpaper),
        )
        .route("/api/system/network-info", get(get_network_info))
        .route("/api/timer", get(get_timer))
        .route("/api/timer/start", post(start_timer))
        .route("/api/timer/pause", post(pause_timer))
        .route("/api/timer/reset", post(reset_timer))
        .route("/api/timer/set", post(set_timer))
        .route("/api/timer/overtime", post(set_timer_overtime))
        .route("/api/timer/message", post(set_stage_message))
        .route(
            "/api/settings/{key}",
            get(get_app_setting).post(save_app_setting),
        )
        .route("/api/outputs/reload", post(reload_outputs))
        .route("/api/church/upload-logo", post(upload_church_logo))
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
        .route("/api/setlists/items/clear", post(clear_setlist_items))
        .route("/api/setlists/items/reorder", post(reorder_setlist_items))
        .route("/api/setlists/items/rename", post(rename_setlist_item))
        .route(
            "/api/media/folders",
            get(list_media_folders).post(create_media_folder),
        )
        .route(
            "/api/media/folders/{id}",
            axum::routing::delete(delete_media_folder),
        )
        .route("/api/media/wallpapers", get(list_wallpapers))
        .route("/api/media/images", get(list_images))
        .route("/api/media/images/upload", post(upload_image))
        .route("/api/media/upload", post(upload_media))
        .route("/api/media/external-file", get(serve_external_file))
        .route("/api/media/files/{*subpath}", get(serve_media_file))
        .route("/api/bibles", get(list_bibles))
        .route("/api/bibles/import", post(import_bible))
        .route(
            "/api/bibles/{id}",
            axum::routing::put(update_bible).delete(delete_bible),
        )
        .route("/api/bibles/{id}/books", get(get_bible_books))
        .route(
            "/api/bibles/{id}/books/{book_id}/chapters",
            get(get_bible_chapters),
        )
        .route(
            "/api/bibles/{id}/books/{book_id}/chapters/{chapter}/verses",
            get(get_bible_verses),
        )
        .route("/api/bibles/{id}/search", get(search_bible))
        .route("/api/bibles/parse-reference", get(parse_reference_endpoint))
        .route("/api/updater/check", get(check_updater))
        .route("/api/updater/status", get(get_updater_status))
        .route("/api/updater/install", post(install_updater))
        .layer(DefaultBodyLimit::max(128 * 1024 * 1024))
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

#[derive(Deserialize)]
pub struct SetWallpaperRequest {
    pub path: Option<String>,
    pub mode: Option<String>,
}

async fn stop_presentation(State(state): State<AppState>) -> Json<PresentationState> {
    let updated = state.stop_presentation().await;
    Json(updated)
}

async fn set_presentation_wallpaper(
    State(state): State<AppState>,
    Json(payload): Json<SetWallpaperRequest>,
) -> Json<PresentationState> {
    let updated = state.set_wallpaper(payload.path, payload.mode).await;
    Json(updated)
}

#[derive(Deserialize)]
pub struct QuickMediaRequest {
    pub item: MediaItem,
    #[serde(default)]
    pub slide_index: Option<usize>,
}

async fn quick_present_media(
    State(state): State<AppState>,
    Json(payload): Json<QuickMediaRequest>,
) -> Json<PresentationState> {
    let updated = state
        .quick_present_media_with_slide(payload.item, payload.slide_index.unwrap_or(0))
        .await;
    Json(updated)
}

#[derive(Debug, Serialize)]
pub struct NetworkInfoResponse {
    pub local_ip: String,
    pub port: u16,
    pub public_url: String,
    pub stage_url: String,
}

pub fn get_local_ip() -> String {
    let try_get_ip = || -> Option<String> {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
        socket.connect("8.8.8.8:80").ok()?;
        let addr = socket.local_addr().ok()?;
        let ip = addr.ip().to_string();
        if ip.starts_with("127.") {
            None
        } else {
            Some(ip)
        }
    };
    try_get_ip().unwrap_or_else(|| "127.0.0.1".to_string())
}

async fn get_network_info(State(state): State<AppState>) -> Json<NetworkInfoResponse> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .or_else(|| {
            state.db.as_ref().and_then(|db_mutex| {
                db_mutex
                    .try_lock()
                    .ok()?
                    .get_setting("server_port")
                    .ok()?
                    .and_then(|s| s.parse::<u16>().ok())
            })
        })
        .unwrap_or(18080);
    let local_ip = get_local_ip();
    let public_url = format!("http://{}:{}/public", local_ip, port);
    let stage_url = format!("http://{}:{}/stage", local_ip, port);

    Json(NetworkInfoResponse {
        local_ip,
        port,
        public_url,
        stage_url,
    })
}

#[derive(Deserialize)]
pub struct SetTimerRequest {
    pub duration_seconds: i64,
    pub label: Option<String>,
    pub allow_overtime: Option<bool>,
}

#[derive(Deserialize)]
pub struct SetTimerOvertimeRequest {
    pub allow_overtime: bool,
}

#[derive(Deserialize)]
pub struct StageMessageRequest {
    pub message: Option<String>,
}

#[derive(Deserialize)]
pub struct SaveSettingRequest {
    pub value: String,
}

#[derive(Serialize)]
pub struct SettingResponse {
    pub key: String,
    pub value: Option<String>,
}

async fn get_timer(State(state): State<AppState>) -> Json<op_core::TimerState> {
    let pres = state.presentation.read().await;
    Json(pres.timer.clone())
}

async fn start_timer(State(state): State<AppState>) -> Json<PresentationState> {
    let updated = state.start_timer().await;
    Json(updated)
}

async fn pause_timer(State(state): State<AppState>) -> Json<PresentationState> {
    let updated = state.pause_timer().await;
    Json(updated)
}

async fn reset_timer(State(state): State<AppState>) -> Json<PresentationState> {
    let updated = state.reset_timer().await;
    Json(updated)
}

async fn set_timer(
    State(state): State<AppState>,
    Json(payload): Json<SetTimerRequest>,
) -> Json<PresentationState> {
    let updated = state
        .set_timer(
            payload.duration_seconds,
            payload.label,
            payload.allow_overtime,
        )
        .await;
    Json(updated)
}

async fn set_timer_overtime(
    State(state): State<AppState>,
    Json(payload): Json<SetTimerOvertimeRequest>,
) -> Json<PresentationState> {
    let updated = state.set_timer_overtime(payload.allow_overtime).await;
    Json(updated)
}

async fn set_stage_message(
    State(state): State<AppState>,
    Json(payload): Json<StageMessageRequest>,
) -> Json<PresentationState> {
    let updated = state.set_stage_message(payload.message).await;
    Json(updated)
}

async fn get_app_setting(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<SettingResponse>, (StatusCode, String)> {
    let val = if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        db_guard
            .get_setting(&key)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else {
        None
    };
    Ok(Json(SettingResponse { key, value: val }))
}

async fn save_app_setting(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(payload): Json<SaveSettingRequest>,
) -> Result<Json<SettingResponse>, (StatusCode, String)> {
    if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        db_guard
            .set_setting(&key, &payload.value)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    // Transmite a alteração para todas as telas conectadas via WebSocket em tempo real
    let _ = state.broadcast_tx.send(WsMessage::SettingUpdated {
        key: key.clone(),
        value: Some(payload.value.clone()),
    });

    Ok(Json(SettingResponse {
        key,
        value: Some(payload.value),
    }))
}

#[derive(Debug, Deserialize)]
pub struct ReloadOutputsRequest {
    #[serde(default)]
    pub target: Option<String>,
}

async fn reload_outputs(
    State(state): State<AppState>,
    Json(payload): Json<ReloadOutputsRequest>,
) -> StatusCode {
    let _ = state.broadcast_tx.send(WsMessage::ReloadClients {
        target: payload.target,
    });
    StatusCode::OK
}

async fn upload_church_logo(
    State(state): State<AppState>,
    Json(payload): Json<UploadMediaPayload>,
) -> Result<Json<UploadMediaResponse>, (StatusCode, String)> {
    let media_dir = std::path::PathBuf::from(&state.session.media_path);
    let brand_dir = media_dir.join("church_brand");
    tokio::fs::create_dir_all(&brand_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let bytes = decode_base64(&payload.content_base64)
        .ok_or((StatusCode::BAD_REQUEST, "Base64 inválido".into()))?;

    let sanitized_name: String = payload
        .filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    let target_path = brand_dir.join(&sanitized_name);
    tokio::fs::write(&target_path, bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UploadMediaResponse {
        url: format!("/api/media/files/church_brand/{sanitized_name}"),
        filename: sanitized_name,
    }))
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

async fn clear_setlist_items(State(state): State<AppState>) -> StatusCode {
    state.clear_active_setlist_items().await;
    StatusCode::OK
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
                    let is_image = lower.ends_with(".jpg")
                        || lower.ends_with(".jpeg")
                        || lower.ends_with(".png")
                        || lower.ends_with(".webp")
                        || lower.ends_with(".svg");
                    let is_video = lower.ends_with(".mp4")
                        || lower.ends_with(".webm")
                        || lower.ends_with(".mov")
                        || lower.ends_with(".mkv");
                    if is_image || is_video {
                        items.push(WallpaperItem {
                            name: name.to_string(),
                            url: format!("/api/media/files/wallpapers/{name}"),
                            is_default: name == "dark-ambient.svg"
                                || name == "worship-glow.svg"
                                || name == "cross-subtle.svg",
                            is_video,
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

async fn get_system_paths(State(state): State<AppState>) -> Json<SystemPathsResponse> {
    let db_path = std::path::PathBuf::from(&state.session.db_path);
    let data_dir = db_path.parent().unwrap_or(std::path::Path::new("."));
    let media_dir = data_dir.join("media");
    Json(SystemPathsResponse {
        data_dir: data_dir.display().to_string(),
        db_path: db_path.display().to_string(),
        images_path: media_dir.join("images").display().to_string(),
        wallpapers_path: media_dir.join("wallpapers").display().to_string(),
        videos_path: media_dir.join("videos").display().to_string(),
        fonts_path: data_dir.join("fonts").display().to_string(),
    })
}

#[derive(Serialize)]
pub struct PickFolderResponse {
    pub path: Option<String>,
    pub name: Option<String>,
}

async fn pick_folder() -> Json<PickFolderResponse> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg("POSIX path of (choose folder with prompt \"Selecione a pasta de imagens para o OpenPresenter\")")
            .output();

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if out.status.success() && !stdout.is_empty() {
                let name = std::path::Path::new(&stdout)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Pasta")
                    .to_string();
                return Json(PickFolderResponse {
                    path: Some(stdout),
                    name: Some(name),
                });
            }
        }
    }

    Json(PickFolderResponse {
        path: None,
        name: None,
    })
}

#[derive(Deserialize)]
pub struct OpenPathPayload {
    pub path: String,
}

async fn open_system_path(
    Json(payload): Json<OpenPathPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let raw = payload.path.trim();
    if raw.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Caminho não fornecido".into()));
    }
    let p = std::path::Path::new(raw);
    if !p.exists() {
        return Err((StatusCode::NOT_FOUND, "Caminho não existe".into()));
    }

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(raw).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer").arg(raw).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(raw).spawn();
    }

    Ok(StatusCode::OK)
}

async fn list_media_folders(State(state): State<AppState>) -> Json<Vec<op_core::MediaFolder>> {
    let db = match state.db.as_ref() {
        Some(db) => db,
        None => return Json(Vec::new()),
    };
    let db_guard = db.lock().await;
    let mut list = db_guard.list_media_folders().unwrap_or_default();
    for f in &mut list {
        let p = std::path::Path::new(&f.path);
        if p.is_dir() {
            f.subfolders = discover_subfolders(p, 4);
        }
    }
    Json(list)
}

async fn create_media_folder(
    State(state): State<AppState>,
    Json(payload): Json<CreateMediaFolderPayload>,
) -> Result<Json<op_core::MediaFolder>, (StatusCode, String)> {
    let raw_path = payload.path.trim();
    if raw_path.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Caminho da pasta é obrigatório".into(),
        ));
    }
    let p = std::path::Path::new(raw_path);
    if !p.exists() || !p.is_dir() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Diretório não existe no computador".into(),
        ));
    }

    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;
    let folder = op_core::MediaFolder {
        id: format!("folder-{}", chrono::Utc::now().timestamp_millis()),
        name: if payload.name.trim().is_empty() {
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Pasta")
                .to_string()
        } else {
            payload.name.trim().to_string()
        },
        path: raw_path.to_string(),
        kind: payload.kind.unwrap_or_else(|| "images".to_string()),
        created_at: chrono::Utc::now().to_rfc3339(),
        subfolders: discover_subfolders(p, 4),
    };

    let db_guard = db.lock().await;
    db_guard
        .save_media_folder(&folder)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(folder))
}

async fn delete_media_folder(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;
    let db_guard = db.lock().await;
    match db_guard.delete_media_folder(&id) {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((StatusCode::NOT_FOUND, "Pasta não encontrada".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

fn is_image_file(path: &std::path::Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let lower = ext.to_lowercase();
        lower == "jpg"
            || lower == "jpeg"
            || lower == "png"
            || lower == "webp"
            || lower == "svg"
            || lower == "gif"
    } else {
        false
    }
}

fn url_encode(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

struct ScannedImage {
    pub path: std::path::PathBuf,
    pub filename: String,
    pub subfolder: Option<String>,
    pub rel_path: String,
    pub size_bytes: u64,
}

fn scan_images_recursively(
    root: &std::path::Path,
    current: &std::path::Path,
    depth: usize,
    max_depth: usize,
    out: &mut Vec<ScannedImage>,
) {
    if depth > max_depth {
        return;
    }
    let entries = match std::fs::read_dir(current) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let fname = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if fname.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            scan_images_recursively(root, &path, depth + 1, max_depth, out);
        } else if path.is_file() && is_image_file(&path) {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let (subfolder, rel_path) = if let Ok(rel) = path.strip_prefix(root) {
                let sub = rel.parent().and_then(|p| {
                    let s = p.to_string_lossy().replace('\\', "/");
                    if s.is_empty() { None } else { Some(s) }
                });
                (sub, rel.to_string_lossy().replace('\\', "/"))
            } else {
                (None, fname.clone())
            };
            out.push(ScannedImage {
                path,
                filename: fname,
                subfolder,
                rel_path,
                size_bytes: size,
            });
        }
    }
}

fn discover_subfolders(root: &std::path::Path, max_depth: usize) -> Vec<op_core::MediaSubfolder> {
    let mut scanned = Vec::new();
    scan_images_recursively(root, root, 1, max_depth, &mut scanned);

    use std::collections::BTreeMap;
    let mut sub_counts: BTreeMap<String, usize> = BTreeMap::new();

    fn walk_dirs(
        root: &std::path::Path,
        current: &std::path::Path,
        depth: usize,
        max_depth: usize,
        counts: &mut BTreeMap<String, usize>,
    ) {
        if depth > max_depth {
            return;
        }
        if let Ok(entries) = std::fs::read_dir(current) {
            for entry in entries.flatten() {
                let path = entry.path();
                let fname = match path.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n,
                    None => continue,
                };
                if fname.starts_with('.') {
                    continue;
                }
                if path.is_dir() {
                    if let Ok(rel) = path.strip_prefix(root) {
                        let rel_str = rel.to_string_lossy().replace('\\', "/");
                        counts.entry(rel_str).or_insert(0);
                    }
                    walk_dirs(root, &path, depth + 1, max_depth, counts);
                }
            }
        }
    }
    walk_dirs(root, root, 1, max_depth, &mut sub_counts);

    for img in &scanned {
        if let Some(ref sub) = img.subfolder {
            *sub_counts.entry(sub.clone()).or_insert(0) += 1;
        }
    }

    sub_counts
        .into_iter()
        .map(|(relative_path, count)| {
            let name = relative_path
                .split('/')
                .next_back()
                .unwrap_or(&relative_path)
                .to_string();
            op_core::MediaSubfolder {
                relative_path,
                name,
                count,
            }
        })
        .collect()
}

async fn list_images(State(state): State<AppState>) -> Json<Vec<ImageMediaItem>> {
    let mut items = Vec::new();

    // 1. Imagens internas do app (<data_dir>/media/images) com suporte a sub-pastas
    if let Ok(media_dir) = crate::session::resolve_media_dir() {
        let app_img_dir = media_dir.join("images");
        let _ = std::fs::create_dir_all(&app_img_dir);
        let mut scanned = Vec::new();
        scan_images_recursively(&app_img_dir, &app_img_dir, 1, 5, &mut scanned);
        for item in scanned {
            let enc_rel = item
                .rel_path
                .split('/')
                .map(url_encode)
                .collect::<Vec<_>>()
                .join("/");
            items.push(ImageMediaItem {
                id: format!("app-img-{}", item.rel_path.replace('/', "-")),
                name: item.filename,
                path: item.path.display().to_string(),
                folder_id: "app".to_string(),
                folder_name: "Pasta do App".to_string(),
                subfolder: item.subfolder,
                url: format!("/api/media/files/images/{enc_rel}"),
                is_external: false,
                size_bytes: item.size_bytes,
            });
        }
    }

    // 2. Pastas externas mapeadas com suporte a sub-pastas recursivas
    if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        if let Ok(folders) = db_guard.list_media_folders() {
            for f in folders {
                let p = std::path::Path::new(&f.path);
                if p.is_dir() {
                    let mut scanned = Vec::new();
                    scan_images_recursively(p, p, 1, 5, &mut scanned);
                    for item in scanned {
                        let enc_rel = item
                            .rel_path
                            .split('/')
                            .map(url_encode)
                            .collect::<Vec<_>>()
                            .join("/");
                        items.push(ImageMediaItem {
                            id: format!("ext-{}-{}", f.id, item.rel_path.replace('/', "-")),
                            name: item.filename,
                            path: item.path.display().to_string(),
                            folder_id: f.id.clone(),
                            folder_name: f.name.clone(),
                            subfolder: item.subfolder,
                            url: format!(
                                "/api/media/external-file?folder_id={}&file={enc_rel}",
                                f.id
                            ),
                            is_external: true,
                            size_bytes: item.size_bytes,
                        });
                    }
                }
            }
        }
    }

    items.sort_by_key(|a| a.name.to_lowercase());
    Json(items)
}

async fn upload_image(
    Json(payload): Json<UploadMediaPayload>,
) -> Result<Json<UploadMediaResponse>, (StatusCode, String)> {
    let media_dir = crate::session::resolve_media_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let images_dir = media_dir.join("images");
    let _ = std::fs::create_dir_all(&images_dir);

    let bytes = decode_base64(&payload.content_base64)
        .ok_or((StatusCode::BAD_REQUEST, "Base64 inválido".into()))?;

    let raw_name = std::path::Path::new(&payload.filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("image.png");
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

    let target_path = images_dir.join(&sanitized_name);
    tokio::fs::write(&target_path, bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UploadMediaResponse {
        url: format!("/api/media/files/images/{sanitized_name}"),
        filename: sanitized_name,
    }))
}

async fn serve_external_file(
    State(state): State<AppState>,
    Query(query): Query<ExternalFileQuery>,
) -> impl IntoResponse {
    let clean_file = query.file.replace('\\', "/");
    if clean_file.contains("..") || clean_file.starts_with('/') {
        return (StatusCode::BAD_REQUEST, "Arquivo inválido").into_response();
    }

    let db = match state.db.as_ref() {
        Some(db) => db,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Banco de dados indisponível",
            )
                .into_response();
        }
    };

    let folder_path = {
        let db_guard = db.lock().await;
        let folders = db_guard.list_media_folders().unwrap_or_default();
        match folders.into_iter().find(|f| f.id == query.folder_id) {
            Some(f) => std::path::PathBuf::from(f.path),
            None => return (StatusCode::NOT_FOUND, "Pasta não cadastrada").into_response(),
        }
    };

    let file_path = folder_path.join(&clean_file);
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

// ==========================================
// MÓDULO DE BÍBLIAS (SQLITE)
// ==========================================

#[derive(Deserialize)]
pub struct ImportBiblePayload {
    pub name: Option<String>,
    pub abbreviation: Option<String>,
    pub source_path: Option<String>,
    pub filename: Option<String>,
    pub content_base64: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateBiblePayload {
    pub name: String,
    pub abbreviation: String,
    #[serde(default)]
    pub is_default: Option<bool>,
}

#[derive(Deserialize)]
pub struct BibleSearchQuery {
    pub q: String,
    pub limit: Option<usize>,
    pub exact: Option<bool>,
}

#[derive(Deserialize)]
pub struct ParseReferenceQuery {
    pub q: String,
}

async fn list_bibles(
    State(state): State<AppState>,
) -> Result<Json<Vec<BibleVersionSummary>>, (StatusCode, String)> {
    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;
    let db = db_opt.lock().await;
    let mut bibles = db
        .list_bibles()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let bibles_dir = crate::session::resolve_bibles_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for bible in &mut bibles {
        let path = bibles_dir.join(&bible.filename);
        if path.exists()
            && let Ok(reader) = op_db::BibleReader::open(&path)
            && let Ok((books, verses)) = reader.get_stats()
        {
            bible.book_count = books;
            bible.verse_count = verses;
        }
    }

    Ok(Json(bibles))
}

async fn import_bible(
    State(state): State<AppState>,
    Json(payload): Json<ImportBiblePayload>,
) -> Result<Json<BibleVersionSummary>, (StatusCode, String)> {
    let bibles_dir = crate::session::resolve_bibles_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;

    let id = format!("bible-{}", chrono::Utc::now().timestamp_millis());
    let target_filename = format!("{id}.sqlite");
    let target_path = bibles_dir.join(&target_filename);

    if let Some(source_path_str) = payload.source_path.filter(|p| !p.trim().is_empty()) {
        let src = std::path::PathBuf::from(source_path_str.trim());
        if !src.exists() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arquivo de origem não encontrado".into(),
            ));
        }
        std::fs::copy(&src, &target_path).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao copiar arquivo de bíblia: {e}"),
            )
        })?;
    } else if let Some(base64_data) = payload.content_base64.filter(|b| !b.trim().is_empty()) {
        let bytes = decode_base64(&base64_data)
            .ok_or((StatusCode::BAD_REQUEST, "Base64 inválido".into()))?;
        std::fs::write(&target_path, bytes).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao gravar arquivo de bíblia: {e}"),
            )
        })?;
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Forneça o arquivo (base64) ou o caminho local do arquivo SQLite".into(),
        ));
    }

    // Valida o arquivo SQLite
    let reader = op_db::BibleReader::open(&target_path).map_err(|e| {
        let _ = std::fs::remove_file(&target_path);
        (
            StatusCode::BAD_REQUEST,
            format!("Não foi possível abrir o arquivo SQLite: {e}"),
        )
    })?;

    if let Err(e) = reader.validate_schema() {
        let _ = std::fs::remove_file(&target_path);
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Estrutura incompatível: o arquivo deve conter tabelas 'book' e 'verse' ({e})"),
        ));
    }

    let _ = reader.ensure_indexes();

    let (book_count, verse_count) = reader.get_stats().unwrap_or((66, 31104));

    // Determina o nome
    let mut name = payload.name.unwrap_or_default().trim().to_string();
    if name.is_empty() {
        if let Ok(Some(meta_name)) = reader.get_metadata("name") {
            name = meta_name;
        } else if let Some(ref fname) = payload.filename {
            name = fname
                .trim_end_matches(".sqlite")
                .trim_end_matches(".db")
                .replace('_', " ");
        } else {
            name = "Bíblia Sagrada".to_string();
        }
    }

    // Determina a sigla / abreviação (ex: ARA, NVI)
    let mut abbr = payload
        .abbreviation
        .unwrap_or_default()
        .trim()
        .to_uppercase();
    if abbr.is_empty() {
        if let Ok(Some(meta_abbr)) = reader
            .get_metadata("abbreviation")
            .or_else(|_| reader.get_metadata("version"))
        {
            abbr = meta_abbr.to_uppercase();
        } else {
            let combined =
                format!("{} {}", name, payload.filename.as_deref().unwrap_or("")).to_uppercase();
            if combined.contains("ARA") {
                abbr = "ARA".into();
            } else if combined.contains("NVI") {
                abbr = "NVI".into();
            } else if combined.contains("ACF") {
                abbr = "ACF".into();
            } else if combined.contains("NVT") {
                abbr = "NVT".into();
            } else if combined.contains("ARC") {
                abbr = "ARC".into();
            } else if combined.contains("KJV") {
                abbr = "KJV".into();
            } else if combined.contains("NAA") {
                abbr = "NAA".into();
            } else {
                let initials: String = name
                    .split_whitespace()
                    .filter_map(|w| w.chars().next())
                    .filter(|c| c.is_alphabetic())
                    .take(4)
                    .collect::<String>()
                    .to_uppercase();
                abbr = if initials.is_empty() {
                    "BIB".into()
                } else {
                    initials
                };
            }
        }
    }

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let mut db = db_opt.lock().await;
    let is_first = db.list_bibles().map(|l| l.is_empty()).unwrap_or(true);

    let summary = BibleVersionSummary {
        id,
        name,
        abbreviation: abbr,
        filename: target_filename,
        is_default: is_first,
        book_count,
        verse_count,
        created_at: now.clone(),
        updated_at: now,
    };

    db.save_bible(&summary)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(summary))
}

async fn update_bible(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBiblePayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;
    let mut db = db_opt.lock().await;

    let updated = db
        .update_bible(
            &id,
            &payload.name,
            &payload.abbreviation,
            payload.is_default.unwrap_or(false),
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if updated {
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "Bíblia não encontrada".into()))
    }
}

async fn delete_bible(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;

    let bibles_dir = crate::session::resolve_bibles_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut db = db_opt.lock().await;
    if let Ok(Some(bible)) = db.get_bible(&id) {
        let file_path = bibles_dir.join(&bible.filename);
        let _ = std::fs::remove_file(file_path);
    }

    let deleted = db
        .delete_bible(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "Bíblia não encontrada".into()))
    }
}

async fn get_bible_books(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<BibleBook>>, (StatusCode, String)> {
    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;
    let db = db_opt.lock().await;
    let bible = db
        .get_bible(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Bíblia não encontrada".into()))?;

    let bibles_dir = crate::session::resolve_bibles_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let path = bibles_dir.join(&bible.filename);

    let reader = op_db::BibleReader::open(&path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao abrir bíblia: {e}"),
        )
    })?;

    let books = reader
        .get_books()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(books))
}

async fn get_bible_chapters(
    State(state): State<AppState>,
    Path((id, book_id)): Path<(String, i64)>,
) -> Result<Json<Vec<i64>>, (StatusCode, String)> {
    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;
    let db = db_opt.lock().await;
    let bible = db
        .get_bible(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Bíblia não encontrada".into()))?;

    let bibles_dir = crate::session::resolve_bibles_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let path = bibles_dir.join(&bible.filename);

    let reader = op_db::BibleReader::open(&path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao abrir bíblia: {e}"),
        )
    })?;

    let chapters = reader
        .get_chapters(book_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(chapters))
}

async fn get_bible_verses(
    State(state): State<AppState>,
    Path((id, book_id, chapter)): Path<(String, i64, i64)>,
) -> Result<Json<Vec<BibleVerse>>, (StatusCode, String)> {
    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;
    let db = db_opt.lock().await;
    let bible = db
        .get_bible(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Bíblia não encontrada".into()))?;

    let bibles_dir = crate::session::resolve_bibles_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let path = bibles_dir.join(&bible.filename);

    let reader = op_db::BibleReader::open(&path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao abrir bíblia: {e}"),
        )
    })?;

    let verses = reader
        .get_verses(book_id, chapter)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(verses))
}

async fn search_bible(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<BibleSearchQuery>,
) -> Result<Json<Vec<BibleSearchResult>>, (StatusCode, String)> {
    let db_opt = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;
    let db = db_opt.lock().await;
    let bible = db
        .get_bible(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Bíblia não encontrada".into()))?;

    let bibles_dir = crate::session::resolve_bibles_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let path = bibles_dir.join(&bible.filename);

    let reader = op_db::BibleReader::open(&path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao abrir bíblia: {e}"),
        )
    })?;

    // Verifica se a consulta é uma referência bíblica
    if let Some(parsed) = op_core::parse_bible_reference(&query.q) {
        let start = parsed.verse_start.unwrap_or(1);
        let end = parsed.verse_end.unwrap_or(start);
        if let Ok(verses) = reader.get_verse_range(parsed.book_id, parsed.chapter, start, end)
            && !verses.is_empty()
        {
            let mapped = verses
                .into_iter()
                .map(|v| BibleSearchResult {
                    book_id: v.book_id,
                    book_name: parsed.book_name.clone(),
                    book_abbreviation: parsed.book_abbreviation.clone(),
                    chapter: v.chapter,
                    verse: v.verse,
                    text: v.text,
                })
                .collect();
            return Ok(Json(mapped));
        }
    }

    // Busca textual livre (com suporte a palavra exata)
    let is_exact = query.exact.unwrap_or(false);
    let results = reader
        .search_verses(&query.q, query.limit.unwrap_or(50), is_exact)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results))
}

async fn parse_reference_endpoint(
    Query(query): Query<ParseReferenceQuery>,
) -> Json<Option<ParsedReference>> {
    let res = op_core::parse_bible_reference(&query.q);
    Json(res)
}

#[derive(Serialize)]
pub struct UpdaterStatusPayload {
    pub stage: crate::updater::UpdateStage,
    pub last_check: Option<crate::updater::UpdateCheckResponse>,
}

async fn check_updater(
    State(state): State<AppState>,
) -> Result<Json<crate::updater::UpdateCheckResponse>, (StatusCode, String)> {
    let current_version = op_core::VERSION;
    match crate::updater::check_for_updates(&state.http_client, current_version).await {
        Ok(info) => {
            let mut st = state.updater.write().await;
            st.last_check = Some(info.clone());
            Ok(Json(info))
        }
        Err(e) => {
            // Se já tínhamos uma checagem em cache, devolvemos ela
            let st = state.updater.read().await;
            if let Some(ref cached) = st.last_check {
                return Ok(Json(cached.clone()));
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao consultar atualizações: {}", e),
            ))
        }
    }
}

async fn get_updater_status(State(state): State<AppState>) -> Json<UpdaterStatusPayload> {
    let st = state.updater.read().await;
    Json(UpdaterStatusPayload {
        stage: st.stage.clone(),
        last_check: st.last_check.clone(),
    })
}

#[derive(Deserialize)]
pub struct InstallUpdatePayload {
    pub download_url: Option<String>,
    pub asset_name: Option<String>,
    pub target_version: Option<String>,
}

async fn install_updater(
    State(state): State<AppState>,
    Json(payload): Json<InstallUpdatePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let (url, name, version) = {
        let st = state.updater.read().await;
        let last = st.last_check.as_ref();
        let url = payload
            .download_url
            .or_else(|| last.and_then(|l| l.download_url.clone()));
        let name = payload
            .asset_name
            .or_else(|| last.and_then(|l| l.asset_name.clone()));
        let version = payload
            .target_version
            .or_else(|| last.map(|l| l.latest_version.clone()));
        (url, name, version)
    };

    let (download_url, asset_name, new_version) = match (url, name, version) {
        (Some(u), Some(n), Some(v)) => (u, n, v),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "URL de download ou asset não especificado. Realize uma verificação prévia.".into(),
            ));
        }
    };

    let client = state.http_client.clone();
    let updater_ref = state.updater.clone();

    tokio::spawn(async move {
        crate::updater::start_background_update(
            client,
            updater_ref,
            download_url,
            asset_name,
            new_version,
        )
        .await;
    });

    Ok(Json(serde_json::json!({
        "status": "started",
        "message": "Download e atualização iniciados em segundo plano"
    })))
}

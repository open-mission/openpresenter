use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use op_core::{
    BackupExportOptions, BackupInspectResult, BackupRestoreResult, BibleBook, BibleSearchResult,
    BibleVerse, BibleVersionSummary, DisplayMode, MediaItem, OnlineLyricHit, OutputKind,
    OutputsConfig, ParsedReference, PresentationState, ServiceRoster, ServiceRosterItem,
    ServiceRosterSummary, Setlist, SetlistSummary, Song, SongSummary, TeamDepartment, TeamMember,
    UpsertServiceRoster, UpsertServiceRosterItem, UpsertTeamDepartment, UpsertTeamMember, VERSION,
    WsMessage,
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
    pub tag: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct HarpaItemRaw {
    pub id: Option<serde_json::Value>,
    pub title: String,
    pub artist: Option<String>,
    pub author: Option<String>,
    pub key: Option<String>,
    pub bpm: Option<f64>,
    pub lyrics: Option<HarpaLyricsRaw>,
}

#[derive(Deserialize)]
pub struct HarpaLyricsRaw {
    pub full_text: Option<String>,
    pub paragraphs: Option<Vec<HarpaParagraphRaw>>,
}

#[derive(Deserialize)]
pub struct HarpaParagraphRaw {
    pub number: Option<i32>,
    pub description: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize)]
pub struct HarpaInstallResponse {
    pub success: bool,
    pub installed_count: usize,
    pub message: String,
}

#[derive(Deserialize)]
pub struct ImportSongsPayload {
    #[serde(default)]
    pub format: Option<String>,
    pub content: Option<String>,
    pub filename: Option<String>,
    pub songs: Option<Vec<Song>>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub lines_per_slide: Option<usize>,
}

#[derive(Serialize)]
pub struct ImportSongsResponse {
    pub success: bool,
    pub imported_count: usize,
    pub message: String,
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
    #[serde(default)]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub subfolder: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AudioMediaItem {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VideoMediaItem {
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
        // Documentação da API (100% Offline via Scalar e Swagger UI)
        .route("/api/openapi.json", get(crate::api_docs::get_openapi_spec))
        .route("/docs", get(crate::api_docs::scalar_handler))
        .route("/scalar", get(crate::api_docs::scalar_handler))
        .route("/swagger", get(crate::api_docs::swagger_handler))
        .route(
            "/docs/assets/{*path}",
            get(crate::api_docs::serve_api_docs_asset),
        )
        // Módulo Teams (Departamentos, Membros e Escalas de Culto)
        .route(
            "/api/teams/departments",
            get(list_team_departments).post(create_team_department),
        )
        .route(
            "/api/teams/departments/{id}",
            get(get_team_department)
                .put(update_team_department)
                .delete(delete_team_department),
        )
        .route(
            "/api/teams/members",
            get(list_team_members).post(create_team_member),
        )
        .route(
            "/api/teams/members/{id}",
            get(get_team_member)
                .put(update_team_member)
                .delete(delete_team_member),
        )
        .route(
            "/api/teams/rosters",
            get(list_service_rosters).post(create_service_roster),
        )
        .route("/api/teams/rosters/active", get(get_active_service_roster))
        .route(
            "/api/teams/rosters/{id}",
            get(get_service_roster)
                .put(update_service_roster)
                .delete(delete_service_roster),
        )
        .route(
            "/api/teams/rosters/{id}/active",
            post(set_active_service_roster),
        )
        .route(
            "/api/teams/rosters/{id}/items",
            post(add_service_roster_item),
        )
        .route(
            "/api/teams/rosters/{id}/items/{item_id}",
            axum::routing::patch(update_service_roster_item_status)
                .delete(remove_service_roster_item),
        )
        // Rotas Gerais e de Mídia
        .route("/api/status", get(get_status))
        .route("/api/system/paths", get(get_system_paths))
        .route("/api/system/metrics", get(get_system_metrics))
        .route("/api/system/pick-folder", post(pick_folder))
        .route("/api/system/open-path", post(open_system_path))
        .route("/api/system/restart", post(restart_system))
        .route("/api/media", get(get_media))
        .route("/api/presentation/live", post(select_live_slide))
        .route("/api/presentation/mode", post(set_display_mode))
        .route("/api/presentation/blackout", post(toggle_blackout))
        .route("/api/presentation/clear", post(toggle_clear_text))
        .route("/api/presentation/stop", post(stop_presentation))
        .route("/api/presentation/quick-media", post(quick_present_media))
        .route(
            "/api/presentation/audio-playback",
            post(update_audio_playback),
        )
        .route(
            "/api/presentation/audio-playback/clear",
            post(clear_audio_playback),
        )
        .route(
            "/api/presentation/wallpaper",
            post(set_presentation_wallpaper),
        )
        .route("/api/system/network-info", get(get_network_info))
        .route("/api/connections", get(get_connections_endpoint))
        .route("/api/logs", get(get_logs).delete(clear_logs))
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
        .route("/api/songs/{id}", get(get_song).delete(delete_song))
        .route("/api/songs/tags", get(get_song_tags))
        .route("/api/songs/import", post(import_songs))
        .route("/api/songs/harpa/install", post(install_harpa_crista))
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
            "/api/media/folders/create-dir",
            post(create_media_directory),
        )
        .route("/api/media/app-subfolders", get(list_app_subfolders))
        .route(
            "/api/media/folders/{id}",
            axum::routing::delete(delete_media_folder),
        )
        .route("/api/media/wallpapers", get(list_wallpapers))
        .route(
            "/api/media/wallpapers/{filename}",
            axum::routing::delete(delete_wallpaper),
        )
        .route("/api/media/images", get(list_images))
        .route("/api/media/images/upload", post(upload_image))
        .route("/api/media/videos", get(list_videos))
        .route("/api/media/videos/upload", post(upload_video))
        .route("/api/media/audios", get(list_audios))
        .route("/api/media/audios/upload", post(upload_audio))
        .route("/api/media/upload", post(upload_media))
        .route("/api/media/external-file", get(serve_external_file))
        .route("/api/media/files/{*subpath}", get(serve_media_file))
        .route("/media/{*subpath}", get(serve_media_file))
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
        .route("/api/updater/restart", post(restart_system))
        .route("/api/backup/export", get(export_backup))
        .route(
            "/api/backup/inspect",
            post(inspect_backup).layer(DefaultBodyLimit::max(1024 * 1024 * 1024)),
        )
        .route(
            "/api/backup/import",
            post(import_backup).layer(DefaultBodyLimit::max(1024 * 1024 * 1024)),
        )
        .layer(axum::middleware::from_fn(cors_header_middleware))
        .layer(DefaultBodyLimit::max(128 * 1024 * 1024))
        .with_state(state)
}

/// Middleware que injeta cabeçalhos CORS irrestritos para Web Receivers do Google Cast e clientes HTTP locais.
pub async fn cors_header_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut resp = next.run(req).await;
    let headers = resp.headers_mut();
    if !headers.contains_key(axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN) {
        headers.insert(
            axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            axum::http::HeaderValue::from_static("*"),
        );
    }
    if !headers.contains_key(axum::http::header::ACCESS_CONTROL_ALLOW_METHODS) {
        headers.insert(
            axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
            axum::http::HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS, HEAD"),
        );
    }
    if !headers.contains_key(axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS) {
        headers.insert(
            axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
            axum::http::HeaderValue::from_static("*"),
        );
    }
    resp
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

async fn get_connections_endpoint(
    State(state): State<AppState>,
) -> Json<Vec<op_core::ClientConnection>> {
    let conns = state.get_connections().await;
    Json(conns)
}

#[derive(Debug, Deserialize)]
pub struct GetLogsQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub category: Option<String>,
}

async fn get_logs(
    State(state): State<AppState>,
    Query(query): Query<GetLogsQuery>,
) -> Json<Vec<op_core::ActionLog>> {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);
    let logs = state
        .get_action_logs(limit, offset, query.category.as_deref())
        .await;
    Json(logs)
}

async fn clear_logs(State(state): State<AppState>) -> StatusCode {
    if state.clear_action_logs().await {
        state
            .log_action(
                "logs_cleared",
                "settings",
                "Histórico de logs foi limpo pelo operador",
                None,
            )
            .await;
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
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
    pub remote_url: String,
    pub lower_third_url: String,
}

pub fn get_local_ip() -> String {
    let try_get_ip = || -> Option<String> {
        // Tenta múltiplos alvos de sonda na tabela de rotas do SO sem emitir tráfego real.
        // Cobre conexões ativas com internet (8.8.8.8, 1.1.1.1), sub-redes comuns de roteadores (192.168.x, 10.x, 172.x) e multicast (224.0.0.1)
        let targets = [
            "8.8.8.8:80",
            "1.1.1.1:80",
            "192.168.1.1:80",
            "192.168.0.1:80",
            "10.0.0.1:80",
            "172.16.0.1:80",
            "224.0.0.1:80",
        ];
        for target in targets {
            if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0")
                && socket.connect(target).is_ok()
                && let Ok(addr) = socket.local_addr()
            {
                let ip = addr.ip().to_string();
                if !ip.starts_with("127.") && ip != "0.0.0.0" {
                    return Some(ip);
                }
            }
        }
        None
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
    let remote_url = format!("http://{}:{}/remote", local_ip, port);
    let lower_third_url = format!("http://{}:{}/lower-third", local_ip, port);

    Json(NetworkInfoResponse {
        local_ip,
        port,
        public_url,
        stage_url,
        remote_url,
        lower_third_url,
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

#[derive(Deserialize)]
pub struct AudioPlaybackRequest {
    pub is_playing: bool,
    #[serde(deserialize_with = "op_core::deserialize_f64_to_i64")]
    pub current_time_seconds: i64,
    #[serde(deserialize_with = "op_core::deserialize_f64_to_i64")]
    pub duration_seconds: i64,
    pub title: String,
    pub artist_or_ref: Option<String>,
}

async fn update_audio_playback(
    State(state): State<AppState>,
    Json(payload): Json<AudioPlaybackRequest>,
) -> Json<PresentationState> {
    let updated = state
        .update_audio_playback(
            payload.is_playing,
            payload.current_time_seconds,
            payload.duration_seconds,
            payload.title,
            payload.artist_or_ref,
        )
        .await;
    Json(updated)
}

async fn clear_audio_playback(State(state): State<AppState>) -> Json<PresentationState> {
    let updated = state.clear_audio_playback().await;
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

async fn get_configured_lines_per_slide(state: &AppState) -> usize {
    if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        if let Ok(Some(val)) = db_guard.get_setting("song_settings")
            && let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&val)
            && let Some(lines) = parsed.get("lines_per_slide").and_then(|v| v.as_u64())
        {
            return lines as usize;
        }
    }
    2
}

async fn get_song_tags(State(state): State<AppState>) -> Json<Vec<String>> {
    let tags = if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        db_guard.list_all_song_tags().unwrap_or_default()
    } else {
        Vec::new()
    };
    Json(tags)
}

async fn get_songs(
    State(state): State<AppState>,
    Query(query): Query<SongQuery>,
) -> Json<Vec<SongSummary>> {
    let q = query.q.unwrap_or_default();
    let tag_filter = query.tag.filter(|t| !t.trim().is_empty());
    let mut songs = if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        db_guard.search_songs(&q).unwrap_or_default()
    } else {
        Vec::new()
    };
    if let Some(target_tag) = tag_filter {
        songs.retain(|s| s.tags.iter().any(|t| t.eq_ignore_ascii_case(&target_tag)));
    }
    if let Some(lim) = query.limit
        && lim > 0
        && lim < songs.len()
    {
        songs.truncate(lim);
    }
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

async fn delete_song(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let db_guard = db.lock().await;
    match db_guard.delete_song(&id) {
        Ok(true) => {
            drop(db_guard);
            let details = format!("Música excluída do catálogo (ID: {})", id);
            state
                .log_action("song_deleted", "songs", &details, None)
                .await;
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => Err((StatusCode::NOT_FOUND, "Música não encontrada".into())),
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

    let is_already_in_setlist = state
        .media_items
        .read()
        .await
        .iter()
        .any(|m| m.id == payload.song.id);

    if payload.add_to_setlist || is_already_in_setlist {
        let media_item = payload.song.clone().into_media_item();
        state.add_media_item(media_item).await;
    }

    let details = format!("Música salva no catálogo: '{}'", payload.song.title);
    state
        .log_action("song_saved", "songs", &details, None)
        .await;

    Ok((StatusCode::CREATED, Json(payload.song)))
}

fn parse_imported_text_song(
    filename: &str,
    content: &str,
    lines_per_slide: usize,
    default_tags: &[String],
) -> Song {
    let clean = content.replace("\r\n", "\n").replace('\r', "\n");
    let mut title = String::new();
    let mut artist = String::new();
    let mut composer: Option<String> = None;
    let mut song_key: Option<String> = None;
    let mut tags: Vec<String> = default_tags.to_vec();
    let mut lyrics_lines = Vec::new();

    let mut in_header = true;
    for line in clean.lines() {
        let trimmed = line.trim();

        if in_header {
            if let Some(rest) = trimmed.strip_prefix("# ") {
                title = rest.trim().to_string();
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Title:") {
                title = rest.trim().to_string();
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Título:") {
                title = rest.trim().to_string();
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Artist:") {
                artist = rest.trim().to_string();
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Artista:") {
                artist = rest.trim().to_string();
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Author:") {
                composer = Some(rest.trim().to_string());
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Autor:") {
                composer = Some(rest.trim().to_string());
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Tom:") {
                song_key = Some(rest.trim().to_string());
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Key:") {
                song_key = Some(rest.trim().to_string());
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("Tags:") {
                for t in rest.split(',') {
                    let ct = t.trim();
                    if !ct.is_empty()
                        && !tags
                            .iter()
                            .any(|existing| existing.eq_ignore_ascii_case(ct))
                    {
                        tags.push(ct.to_string());
                    }
                }
                continue;
            }

            if trimmed.is_empty() {
                if !title.is_empty() {
                    in_header = false;
                }
                continue;
            }

            if title.is_empty() {
                title = trimmed.to_string();
                continue;
            }

            in_header = false;
        }

        lyrics_lines.push(line);
    }

    if title.is_empty() {
        title = filename
            .trim_end_matches(".txt")
            .trim_end_matches(".md")
            .replace(['_', '-'], " ")
            .trim()
            .to_string();
    }
    if title.is_empty() {
        title = "Música Sem Título".to_string();
    }

    let mut lyrics_raw = lyrics_lines.join("\n").trim().to_string();
    if lines_per_slide > 0 {
        lyrics_raw = op_core::format_lyrics_lines_per_slide(&lyrics_raw, lines_per_slide);
    }

    let (sections, _) = op_core::parse_raw_lyrics(&lyrics_raw);
    let id = format!("song-{}", crate::online_lyrics::uuid_simple());

    Song {
        id,
        title,
        artist,
        album: None,
        composer,
        song_key,
        bpm: None,
        sections,
        lyrics_raw,
        tags,
        audio_url: None,
    }
}

async fn install_harpa_crista(
    State(state): State<AppState>,
) -> Result<Json<HarpaInstallResponse>, (StatusCode, String)> {
    let lines_per_slide = get_configured_lines_per_slide(&state).await;

    // 1. Tenta carregar do disco local (Downloads ou assets/songs)
    let possible_paths = [
        "/Users/claudio/Downloads/harpa-crista.json",
        "assets/songs/harpa-crista.json",
        "../assets/songs/harpa-crista.json",
    ];

    let mut json_content: Option<String> = None;
    for path in possible_paths {
        if let Ok(content) = tokio::fs::read_to_string(path).await {
            json_content = Some(content);
            break;
        }
    }

    // 2. Se não estiver no disco, tenta baixar da URL pública do GitHub
    if json_content.is_none() {
        let github_url = "https://raw.githubusercontent.com/open-mission/openpresenter/main/assets/songs/harpa-crista.json";
        if let Ok(resp) = state.http_client.get(github_url).send().await
            && resp.status().is_success()
            && let Ok(text) = resp.text().await
        {
            json_content = Some(text);
        }
    }

    let json_text = json_content.ok_or((
        StatusCode::NOT_FOUND,
        "Arquivo harpa-crista.json não encontrado no sistema local nem no GitHub".into(),
    ))?;

    let items: Vec<HarpaItemRaw> = serde_json::from_str(&json_text).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Falha ao ler JSON da Harpa: {e}"),
        )
    })?;

    let mut songs_to_save: Vec<Song> = Vec::with_capacity(items.len());

    for (idx, item) in items.into_iter().enumerate() {
        let raw_title = item.title.trim().to_string();
        if raw_title.is_empty() {
            continue;
        }

        let artist = item
            .artist
            .unwrap_or_else(|| format!("Harpa Cristã - {}", idx + 1));
        let composer = item.author.filter(|a| !a.trim().is_empty());
        let song_key = item.key.filter(|k| !k.trim().is_empty());
        let bpm = item.bpm.map(|b| b as i32);

        let mut raw_lyrics = String::new();
        if let Some(lyrics) = item.lyrics {
            if let Some(paragraphs) = lyrics.paragraphs
                && !paragraphs.is_empty()
            {
                let mut blocks = Vec::new();
                for p in paragraphs {
                    let mut block = String::new();
                    if let Some(ref desc) = p.description
                        && !desc.trim().is_empty()
                    {
                        block.push_str(&format!("## {}\n", desc.trim()));
                    } else if let Some(num) = p.number
                        && num > 0
                    {
                        block.push_str(&format!("## Verso {}\n", num));
                    }
                    if let Some(t) = p.text {
                        let clean_t = t.trim().trim_end_matches('.').trim();
                        block.push_str(clean_t);
                    }
                    if !block.trim().is_empty() {
                        blocks.push(block);
                    }
                }
                raw_lyrics = blocks.join("\n\n");
            } else if let Some(full) = lyrics.full_text {
                raw_lyrics = full.trim().trim_end_matches('.').trim().to_string();
            }
        }

        if lines_per_slide > 0 {
            raw_lyrics = op_core::format_lyrics_lines_per_slide(&raw_lyrics, lines_per_slide);
        }

        let (sections, _) = op_core::parse_raw_lyrics(&raw_lyrics);
        let song_id = format!("harpa-{}", idx + 1);

        songs_to_save.push(Song {
            id: song_id,
            title: raw_title,
            artist,
            album: Some("Harpa Cristã".into()),
            composer,
            song_key,
            bpm,
            sections,
            lyrics_raw: raw_lyrics,
            tags: vec!["Harpa Cristã".into()],
            audio_url: None,
        });
    }

    let installed_count = songs_to_save.len();

    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let db_guard = db.lock().await;
    db_guard
        .save_songs_batch(&songs_to_save, chrono::Utc::now())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let details =
        format!("Instalação da Harpa Cristã concluída ({installed_count} hinos importados)");
    state
        .log_action("harpa_installed", "songs", &details, None)
        .await;

    Ok(Json(HarpaInstallResponse {
        success: true,
        installed_count,
        message: format!("{installed_count} hinos da Harpa Cristã importados com sucesso!"),
    }))
}

async fn import_songs(
    State(state): State<AppState>,
    Json(payload): Json<ImportSongsPayload>,
) -> Result<Json<ImportSongsResponse>, (StatusCode, String)> {
    let default_lines = get_configured_lines_per_slide(&state).await;
    let lines_per_slide = payload.lines_per_slide.unwrap_or(default_lines);

    let mut songs_to_save: Vec<Song> = Vec::new();

    // Caso 1: Lista direta de músicas estruturadas
    if let Some(list) = payload.songs {
        for mut s in list {
            for t in &payload.tags {
                if !s.tags.contains(t) {
                    s.tags.push(t.clone());
                }
            }
            if lines_per_slide > 0 {
                s.lyrics_raw =
                    op_core::format_lyrics_lines_per_slide(&s.lyrics_raw, lines_per_slide);
                let (sections, _) = op_core::parse_raw_lyrics(&s.lyrics_raw);
                s.sections = sections;
            }
            if s.id.is_empty() {
                s.id = format!("song-{}", crate::online_lyrics::uuid_simple());
            }
            songs_to_save.push(s);
        }
    }

    // Caso 2: Conteúdo bruto textual (JSON, Text ou Markdown)
    if let Some(content) = payload.content {
        let filename = payload.filename.as_deref().unwrap_or("import.txt");
        let format_hint = payload.format.as_deref().unwrap_or("");

        let trimmed_content = content.trim();
        let is_json = format_hint == "json"
            || filename.ends_with(".json")
            || trimmed_content.starts_with('[')
            || trimmed_content.starts_with('{');

        if is_json {
            if let Ok(list) = serde_json::from_str::<Vec<Song>>(trimmed_content) {
                for mut s in list {
                    for t in &payload.tags {
                        if !s.tags.contains(t) {
                            s.tags.push(t.clone());
                        }
                    }
                    if lines_per_slide > 0 {
                        s.lyrics_raw =
                            op_core::format_lyrics_lines_per_slide(&s.lyrics_raw, lines_per_slide);
                        let (sections, _) = op_core::parse_raw_lyrics(&s.lyrics_raw);
                        s.sections = sections;
                    }
                    if s.id.is_empty() {
                        s.id = format!("song-{}", crate::online_lyrics::uuid_simple());
                    }
                    songs_to_save.push(s);
                }
            } else if let Ok(single) = serde_json::from_str::<Song>(trimmed_content) {
                let mut s = single;
                for t in &payload.tags {
                    if !s.tags.contains(t) {
                        s.tags.push(t.clone());
                    }
                }
                if lines_per_slide > 0 {
                    s.lyrics_raw =
                        op_core::format_lyrics_lines_per_slide(&s.lyrics_raw, lines_per_slide);
                    let (sections, _) = op_core::parse_raw_lyrics(&s.lyrics_raw);
                    s.sections = sections;
                }
                if s.id.is_empty() {
                    s.id = format!("song-{}", crate::online_lyrics::uuid_simple());
                }
                songs_to_save.push(s);
            } else if let Ok(harpa_list) =
                serde_json::from_str::<Vec<HarpaItemRaw>>(trimmed_content)
            {
                for (idx, item) in harpa_list.into_iter().enumerate() {
                    let title = item.title.trim().to_string();
                    if title.is_empty() {
                        continue;
                    }
                    let artist = item
                        .artist
                        .unwrap_or_else(|| format!("Harpa Cristã - {}", idx + 1));
                    let mut raw_lyrics = String::new();
                    if let Some(lyrics) = item.lyrics {
                        if let Some(paras) = lyrics.paragraphs
                            && !paras.is_empty()
                        {
                            let mut blocks = Vec::new();
                            for p in paras {
                                let mut b = String::new();
                                if let Some(d) = p.description
                                    && !d.trim().is_empty()
                                {
                                    b.push_str(&format!("## {}\n", d.trim()));
                                } else if let Some(n) = p.number
                                    && n > 0
                                {
                                    b.push_str(&format!("## Verso {}\n", n));
                                }
                                if let Some(t) = p.text {
                                    b.push_str(t.trim().trim_end_matches('.').trim());
                                }
                                if !b.trim().is_empty() {
                                    blocks.push(b);
                                }
                            }
                            raw_lyrics = blocks.join("\n\n");
                        } else if let Some(f) = lyrics.full_text {
                            raw_lyrics = f.trim().trim_end_matches('.').trim().to_string();
                        }
                    }
                    if lines_per_slide > 0 {
                        raw_lyrics =
                            op_core::format_lyrics_lines_per_slide(&raw_lyrics, lines_per_slide);
                    }
                    let (sections, _) = op_core::parse_raw_lyrics(&raw_lyrics);
                    let mut tags = payload.tags.clone();
                    if !tags.iter().any(|t| t.eq_ignore_ascii_case("Harpa Cristã")) {
                        tags.push("Harpa Cristã".into());
                    }
                    songs_to_save.push(Song {
                        id: format!("harpa-{}", idx + 1),
                        title,
                        artist,
                        album: Some("Harpa Cristã".into()),
                        composer: item.author.filter(|a| !a.trim().is_empty()),
                        song_key: item.key.filter(|k| !k.trim().is_empty()),
                        bpm: item.bpm.map(|b| b as i32),
                        sections,
                        lyrics_raw: raw_lyrics,
                        tags,
                        audio_url: None,
                    });
                }
            } else if let Ok(generic_list) =
                serde_json::from_str::<Vec<serde_json::Value>>(trimmed_content)
            {
                for item in generic_list {
                    let title = item
                        .get("title")
                        .or_else(|| item.get("nome"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if title.is_empty() {
                        continue;
                    }
                    let artist = item
                        .get("artist")
                        .or_else(|| item.get("artista"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    let lyrics = item
                        .get("lyrics_raw")
                        .or_else(|| item.get("lyrics"))
                        .or_else(|| item.get("letra"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    let mut lyrics_raw = lyrics;
                    if lines_per_slide > 0 {
                        lyrics_raw =
                            op_core::format_lyrics_lines_per_slide(&lyrics_raw, lines_per_slide);
                    }
                    let (sections, _) = op_core::parse_raw_lyrics(&lyrics_raw);
                    let mut tags = payload.tags.clone();
                    if let Some(t_arr) = item.get("tags").and_then(|t| t.as_array()) {
                        for t in t_arr {
                            if let Some(ts) = t.as_str()
                                && !tags
                                    .iter()
                                    .any(|existing| existing.eq_ignore_ascii_case(ts))
                            {
                                tags.push(ts.to_string());
                            }
                        }
                    }
                    let audio_url = item
                        .get("audio_url")
                        .or_else(|| item.get("audio"))
                        .and_then(|v| v.as_str())
                        .map(String::from);
                    songs_to_save.push(Song {
                        id: format!("song-{}", crate::online_lyrics::uuid_simple()),
                        title,
                        artist,
                        album: None,
                        composer: None,
                        song_key: None,
                        bpm: None,
                        sections,
                        lyrics_raw,
                        tags,
                        audio_url,
                    });
                }
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Formato JSON não reconhecido para importação".into(),
                ));
            }
        } else {
            // Text ou Markdown
            let song =
                parse_imported_text_song(filename, trimmed_content, lines_per_slide, &payload.tags);
            songs_to_save.push(song);
        }
    }

    if songs_to_save.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Nenhuma música válida encontrada para importar".into(),
        ));
    }

    let imported_count = songs_to_save.len();
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let db_guard = db.lock().await;
    db_guard
        .save_songs_batch(&songs_to_save, chrono::Utc::now())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let details = format!("Importação de músicas concluída ({imported_count} importadas)");
    state
        .log_action("songs_imported", "songs", &details, None)
        .await;

    Ok(Json(ImportSongsResponse {
        success: true,
        imported_count,
        message: format!("{imported_count} música(s) importada(s) com sucesso!"),
    }))
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
    let lines_per_slide = get_configured_lines_per_slide(&state).await;
    match crate::online_lyrics::fetch_online_song(&state.http_client, &query.provider, &query.id)
        .await
    {
        Ok(fetched) => {
            let mut song = fetched.into_song();
            if lines_per_slide > 0 {
                let formatted =
                    op_core::format_lyrics_lines_per_slide(&song.lyrics_raw, lines_per_slide);
                let (sections, _) = op_core::parse_raw_lyrics(&formatted);
                song.lyrics_raw = formatted;
                song.sections = sections;
            }
            Ok(Json(song))
        }
        Err(e) => Err((StatusCode::BAD_GATEWAY, e.to_string())),
    }
}

async fn add_to_setlist(
    State(state): State<AppState>,
    Json(payload): Json<AddToSetlistPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    if let Some(item) = payload.item {
        let details = format!("Item '{}' adicionado à setlist", item.title);
        state
            .log_action("setlist_item_added", "setlists", &details, None)
            .await;
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
        Some(s) => {
            let details = format!("Setlist '{}' ativada", s.title);
            state
                .log_action("setlist_activated", "setlists", &details, None)
                .await;
            Ok(Json(s))
        }
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
    state
        .log_action(
            "setlist_cleared",
            "setlists",
            "Todos os itens da setlist foram removidos",
            None,
        )
        .await;
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

async fn delete_wallpaper(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let media_dir = state.resolve_media_dir();
    let wallpapers_dir = media_dir.join("wallpapers");

    let clean_name = std::path::Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or((StatusCode::BAD_REQUEST, "Nome de arquivo inválido".into()))?;

    if clean_name != filename || clean_name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Nome de arquivo inválido".into()));
    }

    if clean_name == "dark-ambient.svg"
        || clean_name == "worship-glow.svg"
        || clean_name == "cross-subtle.svg"
    {
        return Err((
            StatusCode::FORBIDDEN,
            "Papéis de parede padrão do sistema não podem ser removidos".into(),
        ));
    }

    let file_path = wallpapers_dir.join(clean_name);
    if !file_path.exists() || !file_path.is_file() {
        return Err((
            StatusCode::NOT_FOUND,
            "Papel de parede não encontrado".into(),
        ));
    }

    tokio::fs::remove_file(&file_path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Falha ao remover arquivo: {e}"),
        )
    })?;

    let removed_url = format!("/api/media/files/wallpapers/{clean_name}");
    {
        let mut pres = state.presentation.write().await;
        let mut changed = false;
        if pres.wallpaper_path.as_deref() == Some(&removed_url) {
            pres.wallpaper_path = Some("/api/media/files/wallpapers/dark-ambient.svg".into());
            changed = true;
        }
        if pres.lyrics_background_path.as_deref() == Some(&removed_url) {
            pres.lyrics_background_path =
                Some("/api/media/files/wallpapers/dark-ambient.svg".into());
            changed = true;
        }
        if changed {
            let updated = pres.clone();
            drop(pres);
            let _ = state.broadcast_tx.send(WsMessage::StateUpdated {
                presentation: updated,
            });
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn serve_media_file(
    Path(subpath): Path<String>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    if subpath.contains("..") || subpath.starts_with('/') {
        return (
            StatusCode::BAD_REQUEST,
            [
                (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                (
                    axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET, HEAD, OPTIONS",
                ),
                (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
            ],
            "Caminho inválido",
        )
            .into_response();
    }

    let media_dir = match crate::session::resolve_media_dir() {
        Ok(dir) => dir,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [
                    (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                    (
                        axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                        "GET, HEAD, OPTIONS",
                    ),
                    (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
                ],
                "Erro ao resolver pasta de mídias",
            )
                .into_response();
        }
    };

    // Suporta resolução direta ou sob subpastas conhecidas (wallpapers, images, videos, audios, church_brand)
    let candidates = [
        media_dir.join(&subpath),
        media_dir.join("wallpapers").join(&subpath),
        media_dir.join("images").join(&subpath),
        media_dir.join("videos").join(&subpath),
        media_dir.join("audios").join(&subpath),
        media_dir.join("church_brand").join(&subpath),
    ];

    let found_path = candidates.into_iter().find(|p| p.is_file());

    let file_path = match found_path {
        Some(p) => p,
        None => {
            return (
                StatusCode::NOT_FOUND,
                [
                    (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                    (
                        axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                        "GET, HEAD, OPTIONS",
                    ),
                    (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
                ],
                "Arquivo não encontrado",
            )
                .into_response();
        }
    };

    use tower::ServiceExt;
    let serve_file = tower_http::services::ServeFile::new(file_path);
    match serve_file.oneshot(req).await {
        Ok(mut resp) => {
            resp.headers_mut().insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                axum::http::HeaderValue::from_static("*"),
            );
            resp.headers_mut().insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                axum::http::HeaderValue::from_static("GET, HEAD, OPTIONS"),
            );
            resp.headers_mut().insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                axum::http::HeaderValue::from_static("*"),
            );
            resp.into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [
                (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                (
                    axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET, HEAD, OPTIONS",
                ),
                (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
            ],
            "Erro ao servir arquivo",
        )
            .into_response(),
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

async fn get_system_metrics(State(state): State<AppState>) -> Json<op_core::SystemMetrics> {
    let mut collector = state.metrics.lock().await;
    let metrics = collector.collect();
    Json(metrics)
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
            f.subfolders = discover_subfolders_for_kind(p, 4, f.kind == "videos");
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

#[derive(Debug, Clone, Deserialize)]
pub struct AppSubfoldersQuery {
    #[serde(default = "default_media_kind")]
    pub kind: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDirectoryPayload {
    #[serde(default = "default_media_kind")]
    pub kind: String,
    #[serde(default = "default_app_folder")]
    pub folder_id: String,
    #[serde(default)]
    pub parent_subfolder: Option<String>,
    pub name: String,
}

fn default_media_kind() -> String {
    "images".to_string()
}

fn default_app_folder() -> String {
    "app".to_string()
}

async fn list_app_subfolders(
    Query(query): Query<AppSubfoldersQuery>,
) -> Result<Json<Vec<op_core::MediaSubfolder>>, (StatusCode, String)> {
    let media_dir = crate::session::resolve_media_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let kind = if query.kind == "videos" {
        "videos"
    } else {
        "images"
    };
    let app_dir = media_dir.join(kind);
    let _ = std::fs::create_dir_all(&app_dir);
    let is_video = kind == "videos";
    let subfolders = discover_subfolders_for_kind(&app_dir, 4, is_video);
    Ok(Json(subfolders))
}

async fn create_media_directory(
    State(state): State<AppState>,
    Json(payload): Json<CreateDirectoryPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let raw_name = payload.name.trim();
    if raw_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Nome da pasta não pode ser vazio".into(),
        ));
    }
    if raw_name.contains('/') || raw_name.contains('\\') || raw_name.contains("..") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Nome de pasta contém caracteres inválidos".into(),
        ));
    }

    let sanitized_name: String = raw_name
        .chars()
        .filter(|c| {
            !c.is_control()
                && *c != '/'
                && *c != '\\'
                && *c != ':'
                && *c != '*'
                && *c != '?'
                && *c != '"'
                && *c != '<'
                && *c != '>'
                && *c != '|'
        })
        .collect();

    let clean_name = sanitized_name.trim();
    if clean_name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Nome de pasta inválido".into()));
    }

    let kind = if payload.kind == "videos" {
        "videos"
    } else {
        "images"
    };

    let target_base: std::path::PathBuf =
        if payload.folder_id == "app" || payload.folder_id.is_empty() {
            let media_dir = crate::session::resolve_media_dir()
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            media_dir.join(kind)
        } else {
            let db = state.db.as_ref().ok_or((
                StatusCode::SERVICE_UNAVAILABLE,
                "Banco de dados indisponível".into(),
            ))?;
            let db_guard = db.lock().await;
            let folders = db_guard.list_media_folders().unwrap_or_default();
            let folder = folders
                .into_iter()
                .find(|f| f.id == payload.folder_id)
                .ok_or((StatusCode::NOT_FOUND, "Pasta mapeada não encontrada".into()))?;
            std::path::PathBuf::from(folder.path)
        };

    let mut target_dir = target_base;
    let mut clean_rel_path = clean_name.to_string();

    if let Some(parent) = payload
        .parent_subfolder
        .as_deref()
        .filter(|s| !s.trim().is_empty())
    {
        let clean_parent = parent.trim().replace('\\', "/");
        for part in clean_parent.split('/') {
            let trimmed = part.trim();
            if trimmed.is_empty() || trimmed == "." {
                continue;
            }
            if trimmed == ".." {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Caminho inválido (..) detectado".into(),
                ));
            }
            target_dir.push(trimmed);
        }
        clean_rel_path = format!("{clean_parent}/{clean_name}");
    }
    target_dir.push(clean_name);

    std::fs::create_dir_all(&target_dir).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Falha ao criar diretório físico no disco: {e}"),
        )
    })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "name": clean_name,
        "relative_path": clean_rel_path,
        "path": target_dir.display().to_string(),
        "kind": kind,
        "folder_id": payload.folder_id
    })))
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

fn is_audio_file(path: &std::path::Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let lower = ext.to_lowercase();
        matches!(
            lower.as_str(),
            "mp3" | "wav" | "ogg" | "aac" | "m4a" | "flac" | "wma" | "weba"
        )
    } else {
        false
    }
}

fn is_video_file(path: &std::path::Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let lower = ext.to_lowercase();
        matches!(
            lower.as_str(),
            "mp4" | "webm" | "mov" | "mkv" | "avi" | "m4v"
        )
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
    scan_media_recursively(root, current, depth, max_depth, is_image_file, out);
}

fn scan_media_recursively(
    root: &std::path::Path,
    current: &std::path::Path,
    depth: usize,
    max_depth: usize,
    is_valid_file: fn(&std::path::Path) -> bool,
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
            scan_media_recursively(root, &path, depth + 1, max_depth, is_valid_file, out);
        } else if path.is_file() && is_valid_file(&path) {
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

fn discover_subfolders_for_kind(
    root: &std::path::Path,
    max_depth: usize,
    is_video: bool,
) -> Vec<op_core::MediaSubfolder> {
    let mut scanned = Vec::new();
    if is_video {
        scan_media_recursively(root, root, 1, max_depth, is_video_file, &mut scanned);
    } else {
        scan_images_recursively(root, root, 1, max_depth, &mut scanned);
    }

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

    for item in &scanned {
        if let Some(ref sub) = item.subfolder {
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

fn discover_subfolders(root: &std::path::Path, max_depth: usize) -> Vec<op_core::MediaSubfolder> {
    discover_subfolders_for_kind(root, max_depth, false)
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
    State(state): State<AppState>,
    Json(payload): Json<UploadMediaPayload>,
) -> Result<Json<UploadMediaResponse>, (StatusCode, String)> {
    let media_dir = crate::session::resolve_media_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut images_dir = media_dir.join("images");

    let is_app = payload.folder_id.as_deref().unwrap_or("app") == "app";
    let mut clean_subfolder = String::new();

    if is_app {
        if let Some(ref sub) = payload.subfolder.filter(|s| !s.trim().is_empty()) {
            for part in sub.trim().replace('\\', "/").split('/') {
                let trimmed = part.trim();
                if trimmed.is_empty() || trimmed == "." {
                    continue;
                }
                if trimmed == ".." {
                    return Err((StatusCode::BAD_REQUEST, "Subpasta inválida".into()));
                }
                images_dir.push(trimmed);
            }
            clean_subfolder = sub.trim().replace('\\', "/");
        }
    } else if let Some(ref fid) = payload.folder_id {
        let db = state.db.as_ref().ok_or((
            StatusCode::SERVICE_UNAVAILABLE,
            "Banco de dados indisponível".into(),
        ))?;
        let db_guard = db.lock().await;
        let folders = db_guard.list_media_folders().unwrap_or_default();
        let folder = folders
            .into_iter()
            .find(|f| f.id == *fid)
            .ok_or((StatusCode::NOT_FOUND, "Pasta mapeada não encontrada".into()))?;
        images_dir = std::path::PathBuf::from(folder.path);
        if let Some(ref sub) = payload.subfolder.filter(|s| !s.trim().is_empty()) {
            for part in sub.trim().replace('\\', "/").split('/') {
                let trimmed = part.trim();
                if trimmed.is_empty() || trimmed == "." {
                    continue;
                }
                if trimmed == ".." {
                    return Err((StatusCode::BAD_REQUEST, "Subpasta inválida".into()));
                }
                images_dir.push(trimmed);
            }
            clean_subfolder = sub.trim().replace('\\', "/");
        }
    }

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

    let url = if is_app {
        if clean_subfolder.is_empty() {
            format!("/api/media/files/images/{sanitized_name}")
        } else {
            let enc_sub = clean_subfolder
                .split('/')
                .map(url_encode)
                .collect::<Vec<_>>()
                .join("/");
            format!("/api/media/files/images/{enc_sub}/{sanitized_name}")
        }
    } else {
        let fid = payload.folder_id.unwrap_or_default();
        let file_param = if clean_subfolder.is_empty() {
            sanitized_name.clone()
        } else {
            format!("{clean_subfolder}/{sanitized_name}")
        };
        let enc_file = file_param
            .split('/')
            .map(url_encode)
            .collect::<Vec<_>>()
            .join("/");
        format!("/api/media/external-file?folder_id={fid}&file={enc_file}")
    };

    Ok(Json(UploadMediaResponse {
        url,
        filename: sanitized_name,
    }))
}

async fn list_videos(State(state): State<AppState>) -> Json<Vec<VideoMediaItem>> {
    let mut items = Vec::new();

    // 1. Vídeos internos do app (<data_dir>/media/videos)
    if let Ok(media_dir) = crate::session::resolve_media_dir() {
        let app_video_dir = media_dir.join("videos");
        let _ = std::fs::create_dir_all(&app_video_dir);
        let mut scanned = Vec::new();
        scan_media_recursively(
            &app_video_dir,
            &app_video_dir,
            1,
            5,
            is_video_file,
            &mut scanned,
        );
        for item in scanned {
            let enc_rel = item
                .rel_path
                .split('/')
                .map(url_encode)
                .collect::<Vec<_>>()
                .join("/");
            items.push(VideoMediaItem {
                id: format!("app-video-{}", item.rel_path.replace('/', "-")),
                name: item.filename,
                path: item.path.display().to_string(),
                folder_id: "app".to_string(),
                folder_name: "Pasta do App".to_string(),
                subfolder: item.subfolder,
                url: format!("/api/media/files/videos/{enc_rel}"),
                is_external: false,
                size_bytes: item.size_bytes,
            });
        }
    }

    // 2. Pastas externas mapeadas
    if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        if let Ok(folders) = db_guard.list_media_folders() {
            for f in folders {
                let p = std::path::Path::new(&f.path);
                if p.is_dir() {
                    let mut scanned = Vec::new();
                    scan_media_recursively(p, p, 1, 5, is_video_file, &mut scanned);
                    for item in scanned {
                        let enc_rel = item
                            .rel_path
                            .split('/')
                            .map(url_encode)
                            .collect::<Vec<_>>()
                            .join("/");
                        items.push(VideoMediaItem {
                            id: format!("ext-video-{}-{}", f.id, item.rel_path.replace('/', "-")),
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

async fn upload_video(
    State(state): State<AppState>,
    Json(payload): Json<UploadMediaPayload>,
) -> Result<Json<UploadMediaResponse>, (StatusCode, String)> {
    let media_dir = crate::session::resolve_media_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut videos_dir = media_dir.join("videos");

    let is_app = payload.folder_id.as_deref().unwrap_or("app") == "app";
    let mut clean_subfolder = String::new();

    if is_app {
        if let Some(ref sub) = payload.subfolder.filter(|s| !s.trim().is_empty()) {
            for part in sub.trim().replace('\\', "/").split('/') {
                let trimmed = part.trim();
                if trimmed.is_empty() || trimmed == "." {
                    continue;
                }
                if trimmed == ".." {
                    return Err((StatusCode::BAD_REQUEST, "Subpasta inválida".into()));
                }
                videos_dir.push(trimmed);
            }
            clean_subfolder = sub.trim().replace('\\', "/");
        }
    } else if let Some(ref fid) = payload.folder_id {
        let db = state.db.as_ref().ok_or((
            StatusCode::SERVICE_UNAVAILABLE,
            "Banco de dados indisponível".into(),
        ))?;
        let db_guard = db.lock().await;
        let folders = db_guard.list_media_folders().unwrap_or_default();
        let folder = folders
            .into_iter()
            .find(|f| f.id == *fid)
            .ok_or((StatusCode::NOT_FOUND, "Pasta mapeada não encontrada".into()))?;
        videos_dir = std::path::PathBuf::from(folder.path);
        if let Some(ref sub) = payload.subfolder.filter(|s| !s.trim().is_empty()) {
            for part in sub.trim().replace('\\', "/").split('/') {
                let trimmed = part.trim();
                if trimmed.is_empty() || trimmed == "." {
                    continue;
                }
                if trimmed == ".." {
                    return Err((StatusCode::BAD_REQUEST, "Subpasta inválida".into()));
                }
                videos_dir.push(trimmed);
            }
            clean_subfolder = sub.trim().replace('\\', "/");
        }
    }

    let _ = std::fs::create_dir_all(&videos_dir);

    let bytes = decode_base64(&payload.content_base64)
        .ok_or((StatusCode::BAD_REQUEST, "Base64 inválido".into()))?;

    let raw_name = std::path::Path::new(&payload.filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("video.mp4");
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

    let target_path = videos_dir.join(&sanitized_name);
    tokio::fs::write(&target_path, bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let url = if is_app {
        if clean_subfolder.is_empty() {
            format!("/api/media/files/videos/{sanitized_name}")
        } else {
            let enc_sub = clean_subfolder
                .split('/')
                .map(url_encode)
                .collect::<Vec<_>>()
                .join("/");
            format!("/api/media/files/videos/{enc_sub}/{sanitized_name}")
        }
    } else {
        let fid = payload.folder_id.unwrap_or_default();
        let file_param = if clean_subfolder.is_empty() {
            sanitized_name.clone()
        } else {
            format!("{clean_subfolder}/{sanitized_name}")
        };
        let enc_file = file_param
            .split('/')
            .map(url_encode)
            .collect::<Vec<_>>()
            .join("/");
        format!("/api/media/external-file?folder_id={fid}&file={enc_file}")
    };

    Ok(Json(UploadMediaResponse {
        url,
        filename: sanitized_name,
    }))
}

async fn list_audios(State(state): State<AppState>) -> Json<Vec<AudioMediaItem>> {
    let mut items = Vec::new();

    // 1. Áudios internos do app (<data_dir>/media/audios)
    if let Ok(media_dir) = crate::session::resolve_media_dir() {
        let app_audio_dir = media_dir.join("audios");
        let _ = std::fs::create_dir_all(&app_audio_dir);
        let mut scanned = Vec::new();
        scan_media_recursively(
            &app_audio_dir,
            &app_audio_dir,
            1,
            5,
            is_audio_file,
            &mut scanned,
        );
        for item in scanned {
            let enc_rel = item
                .rel_path
                .split('/')
                .map(url_encode)
                .collect::<Vec<_>>()
                .join("/");
            items.push(AudioMediaItem {
                id: format!("app-audio-{}", item.rel_path.replace('/', "-")),
                name: item.filename,
                path: item.path.display().to_string(),
                folder_id: "app".to_string(),
                folder_name: "Pasta do App".to_string(),
                subfolder: item.subfolder,
                url: format!("/api/media/files/audios/{enc_rel}"),
                is_external: false,
                size_bytes: item.size_bytes,
            });
        }
    }

    // 2. Pastas externas mapeadas
    if let Some(ref db) = state.db {
        let db_guard = db.lock().await;
        if let Ok(folders) = db_guard.list_media_folders() {
            for f in folders {
                let p = std::path::Path::new(&f.path);
                if p.is_dir() {
                    let mut scanned = Vec::new();
                    scan_media_recursively(p, p, 1, 5, is_audio_file, &mut scanned);
                    for item in scanned {
                        let enc_rel = item
                            .rel_path
                            .split('/')
                            .map(url_encode)
                            .collect::<Vec<_>>()
                            .join("/");
                        items.push(AudioMediaItem {
                            id: format!("ext-audio-{}-{}", f.id, item.rel_path.replace('/', "-")),
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

async fn upload_audio(
    Json(payload): Json<UploadMediaPayload>,
) -> Result<Json<UploadMediaResponse>, (StatusCode, String)> {
    let media_dir = crate::session::resolve_media_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let audios_dir = media_dir.join("audios");
    let _ = std::fs::create_dir_all(&audios_dir);

    let bytes = decode_base64(&payload.content_base64)
        .ok_or((StatusCode::BAD_REQUEST, "Base64 inválido".into()))?;

    let raw_name = std::path::Path::new(&payload.filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("audio.mp3");
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

    let target_path = audios_dir.join(&sanitized_name);
    tokio::fs::write(&target_path, bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UploadMediaResponse {
        url: format!("/api/media/files/audios/{sanitized_name}"),
        filename: sanitized_name,
    }))
}

async fn serve_external_file(
    State(state): State<AppState>,
    Query(query): Query<ExternalFileQuery>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    let clean_file = query.file.replace('\\', "/");
    if clean_file.contains("..") || clean_file.starts_with('/') {
        return (
            StatusCode::BAD_REQUEST,
            [
                (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                (
                    axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET, HEAD, OPTIONS",
                ),
                (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
            ],
            "Arquivo inválido",
        )
            .into_response();
    }

    let db = match state.db.as_ref() {
        Some(db) => db,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                [
                    (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                    (
                        axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                        "GET, HEAD, OPTIONS",
                    ),
                    (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
                ],
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
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    [
                        (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                        (
                            axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                            "GET, HEAD, OPTIONS",
                        ),
                        (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
                    ],
                    "Pasta não cadastrada",
                )
                    .into_response();
            }
        }
    };

    let file_path = folder_path.join(&clean_file);
    if !file_path.exists() || !file_path.is_file() {
        return (
            StatusCode::NOT_FOUND,
            [
                (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                (
                    axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET, HEAD, OPTIONS",
                ),
                (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
            ],
            "Arquivo não encontrado",
        )
            .into_response();
    }

    use tower::ServiceExt;
    let serve_file = tower_http::services::ServeFile::new(file_path);
    match serve_file.oneshot(req).await {
        Ok(mut resp) => {
            resp.headers_mut().insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                axum::http::HeaderValue::from_static("*"),
            );
            resp.headers_mut().insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                axum::http::HeaderValue::from_static("GET, HEAD, OPTIONS"),
            );
            resp.headers_mut().insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                axum::http::HeaderValue::from_static("*"),
            );
            resp.into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [
                (axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                (
                    axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET, HEAD, OPTIONS",
                ),
                (axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
            ],
            "Erro ao servir arquivo",
        )
            .into_response(),
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

static BIBLE_IMPORT_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

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

    let counter = BIBLE_IMPORT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let id = format!("bible-{}-{counter}", chrono::Utc::now().timestamp_millis());
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

async fn restart_system(State(state): State<AppState>) -> Json<serde_json::Value> {
    state
        .log_action(
            "restart_requested",
            "system",
            "Solicitação de reinicialização da aplicação / servidor recebida",
            None,
        )
        .await;

    // Spawna uma tarefa assíncrona para não bloquear a resposta HTTP
    tokio::spawn(async move {
        #[cfg(not(test))]
        {
            // Breve delay de 500ms para permitir a transmissão completa do HTTP 200 ao cliente
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            if let Err(e) = crate::process::restart_application() {
                tracing::error!("Falha ao spawnar novo processo para reinicialização: {}", e);
            }
            std::process::exit(0);
        }
    });

    Json(serde_json::json!({
        "status": "restarting",
        "message": "Reinicialização da aplicação e servidor agendada com sucesso"
    }))
}

// ==========================================
// MÓDULO TEAMS (ROTAS & HANDLERS)
// ==========================================

#[derive(Deserialize)]
pub struct ListMembersQuery {
    pub include_inactive: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateTeamDepartmentRequest {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
}

async fn list_team_departments(State(state): State<AppState>) -> Json<Vec<TeamDepartment>> {
    let db = match state.db.as_ref() {
        Some(db) => db,
        None => return Json(Vec::new()),
    };
    let db_guard = db.lock().await;
    let list = db_guard.list_team_departments().unwrap_or_default();
    Json(list)
}

async fn get_team_department(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TeamDepartment>, StatusCode> {
    let db = state.db.as_ref().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let db_guard = db.lock().await;
    match db_guard.get_team_department(&id) {
        Ok(Some(dept)) => Ok(Json(dept)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_team_department(
    State(state): State<AppState>,
    Json(payload): Json<CreateTeamDepartmentRequest>,
) -> Result<(StatusCode, Json<TeamDepartment>), (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let id = payload.id.unwrap_or_else(|| {
        let slug = payload.name.to_lowercase().replace(' ', "-");
        let clean: String = slug
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();
        if clean.is_empty() {
            format!("dept-{}", chrono::Utc::now().timestamp_millis())
        } else {
            clean
        }
    });

    let upsert = UpsertTeamDepartment {
        name: payload.name,
        description: payload.description,
        color: payload.color,
        icon: payload.icon,
        display_order: payload.display_order,
    };

    let mut db_guard = db.lock().await;
    let dept = db_guard
        .save_team_department(&id, &upsert)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "department".into(),
        action: "created".into(),
    });
    let _ = state
        .log_action(
            "department_created",
            "teams",
            &format!("Departamento {} criado", dept.name),
            None,
        )
        .await;

    Ok((StatusCode::CREATED, Json(dept)))
}

async fn update_team_department(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpsertTeamDepartment>,
) -> Result<Json<TeamDepartment>, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    let dept = db_guard
        .save_team_department(&id, &payload)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "department".into(),
        action: "updated".into(),
    });
    let _ = state
        .log_action(
            "department_updated",
            "teams",
            &format!("Departamento {} atualizado", dept.name),
            None,
        )
        .await;

    Ok(Json(dept))
}

async fn delete_team_department(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    match db_guard.delete_team_department(&id) {
        Ok(true) => {
            drop(db_guard);
            let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
                entity: "department".into(),
                action: "deleted".into(),
            });
            let _ = state
                .log_action(
                    "department_deleted",
                    "teams",
                    &format!("Departamento {} excluído", id),
                    None,
                )
                .await;
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => Err((StatusCode::NOT_FOUND, "Departamento não encontrado".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn list_team_members(
    State(state): State<AppState>,
    Query(query): Query<ListMembersQuery>,
) -> Json<Vec<TeamMember>> {
    let db = match state.db.as_ref() {
        Some(db) => db,
        None => return Json(Vec::new()),
    };
    let db_guard = db.lock().await;
    let list = db_guard
        .list_team_members(query.include_inactive.unwrap_or(false))
        .unwrap_or_default();
    Json(list)
}

async fn get_team_member(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TeamMember>, StatusCode> {
    let db = state.db.as_ref().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let db_guard = db.lock().await;
    match db_guard.get_team_member(&id) {
        Ok(Some(member)) => Ok(Json(member)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_team_member(
    State(state): State<AppState>,
    Json(payload): Json<UpsertTeamMember>,
) -> Result<(StatusCode, Json<TeamMember>), (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    let member = db_guard
        .save_team_member(None, &payload)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "member".into(),
        action: "created".into(),
    });
    let _ = state
        .log_action(
            "member_created",
            "teams",
            &format!("Voluntário {} cadastrado", member.name),
            None,
        )
        .await;

    Ok((StatusCode::CREATED, Json(member)))
}

async fn update_team_member(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpsertTeamMember>,
) -> Result<Json<TeamMember>, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    let member = db_guard
        .save_team_member(Some(&id), &payload)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "member".into(),
        action: "updated".into(),
    });
    let _ = state
        .log_action(
            "member_updated",
            "teams",
            &format!("Voluntário {} atualizado", member.name),
            None,
        )
        .await;

    Ok(Json(member))
}

async fn delete_team_member(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    match db_guard.delete_team_member(&id) {
        Ok(true) => {
            drop(db_guard);
            let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
                entity: "member".into(),
                action: "deleted".into(),
            });
            let _ = state
                .log_action(
                    "member_deleted",
                    "teams",
                    &format!("Voluntário {} excluído", id),
                    None,
                )
                .await;
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => Err((StatusCode::NOT_FOUND, "Voluntário não encontrado".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn list_service_rosters(State(state): State<AppState>) -> Json<Vec<ServiceRosterSummary>> {
    let db = match state.db.as_ref() {
        Some(db) => db,
        None => return Json(Vec::new()),
    };
    let db_guard = db.lock().await;
    let list = db_guard.list_service_rosters().unwrap_or_default();
    Json(list)
}

async fn get_active_service_roster(
    State(state): State<AppState>,
) -> Result<Json<ServiceRoster>, StatusCode> {
    let db = state.db.as_ref().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let db_guard = db.lock().await;
    match db_guard.get_active_service_roster() {
        Ok(Some(roster)) => Ok(Json(roster)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_service_roster(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ServiceRoster>, StatusCode> {
    let db = state.db.as_ref().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let db_guard = db.lock().await;
    match db_guard.get_service_roster(&id) {
        Ok(Some(roster)) => Ok(Json(roster)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_service_roster(
    State(state): State<AppState>,
    Json(payload): Json<UpsertServiceRoster>,
) -> Result<(StatusCode, Json<ServiceRoster>), (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    let roster = db_guard
        .save_service_roster(None, &payload)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "roster".into(),
        action: "created".into(),
    });
    let _ = state
        .log_action(
            "roster_created",
            "teams",
            &format!("Escala {} criada", roster.title),
            None,
        )
        .await;

    Ok((StatusCode::CREATED, Json(roster)))
}

async fn update_service_roster(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpsertServiceRoster>,
) -> Result<Json<ServiceRoster>, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    let roster = db_guard
        .save_service_roster(Some(&id), &payload)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "roster".into(),
        action: "updated".into(),
    });
    let _ = state
        .log_action(
            "roster_updated",
            "teams",
            &format!("Escala {} atualizada", roster.title),
            None,
        )
        .await;

    Ok(Json(roster))
}

async fn delete_service_roster(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    match db_guard.delete_service_roster(&id) {
        Ok(true) => {
            drop(db_guard);
            let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
                entity: "roster".into(),
                action: "deleted".into(),
            });
            let _ = state
                .log_action(
                    "roster_deleted",
                    "teams",
                    &format!("Escala {} excluída", id),
                    None,
                )
                .await;
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => Err((StatusCode::NOT_FOUND, "Escala não encontrada".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn set_active_service_roster(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    db_guard
        .set_active_service_roster(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db_guard);

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "roster".into(),
        action: "activated".into(),
    });
    let _ = state
        .log_action(
            "roster_activated",
            "teams",
            &format!("Escala {} ativada para o culto corrente", id),
            None,
        )
        .await;

    Ok(StatusCode::OK)
}

async fn add_service_roster_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpsertServiceRosterItem>,
) -> Result<(StatusCode, Json<ServiceRosterItem>), (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    let item = db_guard
        .assign_service_roster_item(&id, &payload)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
        entity: "roster".into(),
        action: "item_added".into(),
    });

    Ok((StatusCode::CREATED, Json(item)))
}

async fn remove_service_roster_item(
    State(state): State<AppState>,
    Path((id, item_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    match db_guard.remove_service_roster_item(&id, &item_id) {
        Ok(true) => {
            let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
                entity: "roster".into(),
                action: "item_removed".into(),
            });
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => Err((StatusCode::NOT_FOUND, "Item não encontrado".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateRosterItemStatusPayload {
    pub status: String,
}

async fn update_service_roster_item_status(
    State(state): State<AppState>,
    Path((id, item_id)): Path<(String, String)>,
    Json(payload): Json<UpdateRosterItemStatusPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados indisponível".into(),
    ))?;

    let mut db_guard = db.lock().await;
    match db_guard.update_service_roster_item_status(&id, &item_id, &payload.status) {
        Ok(true) => {
            let _ = state.broadcast_tx.send(WsMessage::TeamUpdated {
                entity: "roster".into(),
                action: "item_updated".into(),
            });
            Ok(StatusCode::OK)
        }
        Ok(false) => Err((StatusCode::NOT_FOUND, "Item não encontrado".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// ==========================================
// MÓDULO BACKUP & MIGRAÇÃO (ROTAS & HANDLERS)
// ==========================================

#[derive(Debug, Deserialize)]
pub struct BackupUploadPayload {
    pub filename: Option<String>,
    pub content_base64: Option<String>,
    pub source_path: Option<String>,
}

fn extract_backup_data(
    bytes: axum::body::Bytes,
) -> Result<(std::fs::File, String, u64), (StatusCode, String)> {
    // 1. Se os bytes começam com magic bytes de ZIP ('PK', [0x50, 0x4b])
    if bytes.starts_with(b"PK") {
        let len = bytes.len() as u64;
        let mut temp_file = tempfile::NamedTempFile::new().map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao criar arquivo temporário: {e}"),
            )
        })?;
        std::io::Write::write_all(&mut temp_file, &bytes).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao gravar arquivo temporário: {e}"),
            )
        })?;
        let file = temp_file.reopen().map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao reabrir arquivo temporário: {e}"),
            )
        })?;
        return Ok((file, "backup.opbackup".to_string(), len));
    }

    // 2. Tenta desserializar como JSON BackupUploadPayload
    if let Ok(payload) = serde_json::from_slice::<BackupUploadPayload>(&bytes) {
        let filename = payload
            .filename
            .unwrap_or_else(|| "backup.opbackup".to_string());
        if let Some(source_path) = payload.source_path.filter(|p| !p.trim().is_empty()) {
            let path = std::path::PathBuf::from(source_path.trim());
            if !path.exists() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Arquivo não encontrado no caminho: {}", path.display()),
                ));
            }
            let file = std::fs::File::open(&path).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Falha ao abrir arquivo de backup: {e}"),
                )
            })?;
            let len = file.metadata().map(|m| m.len()).unwrap_or(0);
            return Ok((file, filename, len));
        }

        if let Some(base64_str) = payload.content_base64.filter(|b| !b.trim().is_empty()) {
            let decoded = decode_base64(&base64_str).ok_or((
                StatusCode::BAD_REQUEST,
                "Base64 do arquivo de backup inválido".into(),
            ))?;
            let len = decoded.len() as u64;
            let mut temp_file = tempfile::NamedTempFile::new().map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Falha ao criar arquivo temporário: {e}"),
                )
            })?;
            std::io::Write::write_all(&mut temp_file, &decoded).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Falha ao gravar arquivo temporário: {e}"),
                )
            })?;
            let file = temp_file.reopen().map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Falha ao reabrir arquivo temporário: {e}"),
                )
            })?;
            return Ok((file, filename, len));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        "Formato de arquivo ou payload de backup inválido. Envie o arquivo ZIP (.opbackup) diretamente ou JSON com content_base64 ou source_path.".into(),
    ))
}

async fn export_backup(
    State(state): State<AppState>,
    Query(options): Query<BackupExportOptions>,
) -> Result<axum::response::Response, (StatusCode, String)> {
    let db_mutex = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;

    let data_dir = crate::session::resolve_data_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut temp_file = tempfile::NamedTempFile::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Falha ao criar arquivo temporário de exportação: {e}"),
        )
    })?;

    let db_guard = db_mutex.lock().await;
    let manifest =
        crate::backup::export_backup_package(&mut temp_file, &data_dir, &db_guard, &options)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Falha ao gerar pacote de backup: {e}"),
                )
            })?;
    drop(db_guard);

    state
        .log_action(
            "backup_exported",
            "system",
            &format!(
                "Backup exportado com sucesso: {} arquivos ({} músicas, {} setlists)",
                manifest.stats.total_files_count,
                manifest.stats.songs_count,
                manifest.stats.setlists_count
            ),
            None,
        )
        .await;

    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    let filename = format!("open-presenter-backup-{timestamp}.opbackup");

    let mut file = temp_file.reopen().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Falha ao reabrir arquivo temporário de backup: {e}"),
        )
    })?;
    let mut buffer = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut buffer).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Falha ao ler conteúdo do backup: {e}"),
        )
    })?;

    let response = axum::response::Response::builder()
        .header(axum::http::header::CONTENT_TYPE, "application/zip")
        .header(
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(buffer))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(response)
}

async fn inspect_backup(
    bytes: axum::body::Bytes,
) -> Result<Json<BackupInspectResult>, (StatusCode, String)> {
    let (reader, filename, len) = extract_backup_data(bytes)?;
    let result = crate::backup::inspect_backup_package(reader, &filename, len).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Falha ao inspecionar pacote de backup: {e}"),
        )
    })?;

    Ok(Json(result))
}

async fn import_backup(
    State(state): State<AppState>,
    bytes: axum::body::Bytes,
) -> Result<Json<BackupRestoreResult>, (StatusCode, String)> {
    let db_mutex = state.db.as_ref().ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Banco de dados não disponível".into(),
    ))?;

    let data_dir = crate::session::resolve_data_dir()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (reader, _filename, _len) = extract_backup_data(bytes)?;

    let mut db_guard = db_mutex.lock().await;
    let result =
        crate::backup::restore_backup_package(reader, &data_dir, &mut db_guard).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Falha ao restaurar dados do backup: {e}"),
            )
        })?;
    drop(db_guard);

    // Recarrega o estado em memória e notifica clientes via WebSocket
    state.reload_from_database().await;

    state
        .log_action("backup_restored", "system", &result.message, None)
        .await;

    Ok(Json(result))
}

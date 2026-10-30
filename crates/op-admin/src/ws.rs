use axum::{
    Router,
    extract::{
        ConnectInfo, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::HeaderMap,
    response::IntoResponse,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use op_core::WsMessage;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::{debug, error, info, warn};

use crate::state::AppState;

static NEXT_CONN_ID: AtomicU64 = AtomicU64::new(1);

pub fn create_ws_router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|| addr.ip().to_string());

    ws.on_upgrade(move |socket| handle_socket(socket, state, client_ip))
}

async fn handle_socket(socket: WebSocket, state: AppState, client_ip: String) {
    let (mut sender, mut receiver) = socket.split();
    let conn_id = format!("conn-{}", NEXT_CONN_ID.fetch_add(1, Ordering::Relaxed));

    // 1. Envia estado inicial completo ao cliente conectado
    let presentation = state.presentation.read().await.clone();
    let media_items = state.media_items.read().await.clone();

    let initial_sync = WsMessage::SyncState {
        presentation,
        media_items,
    };

    if let Ok(json) = serde_json::to_string(&initial_sync)
        && let Err(err) = sender.send(Message::Text(json.into())).await
    {
        warn!("Falha ao enviar estado inicial para cliente WebSocket: {err}");
        return;
    }

    // Envia lista atual de conexões
    let current_conns = state.get_connections().await;
    let conns_sync = WsMessage::ConnectionsUpdated {
        connections: current_conns,
    };
    if let Ok(json) = serde_json::to_string(&conns_sync) {
        let _ = sender.send(Message::Text(json.into())).await;
    }

    // 2. Cria canal de escuta do broadcast interno do servidor
    let mut broadcast_rx = state.broadcast_tx.subscribe();

    // 3. Tarefa para transmitir mensagens de broadcast ao cliente
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg)
                && sender.send(Message::Text(json.into())).await.is_err()
            {
                break;
            }
        }
    });

    let mut client_identity: Option<String> = None;
    let state_for_cleanup = state.clone();
    let conn_id_for_cleanup = conn_id.clone();

    // 4. Loop de leitura de mensagens enviadas pelo cliente
    while let Some(result) = receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                debug!("Mensagem WebSocket recebida: {text}");
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(WsMessage::Identify { client_type, route }) => {
                        let active_route = route.unwrap_or_else(|| match client_type.as_str() {
                            "public" => "/public".to_string(),
                            "stage" => "/stage".to_string(),
                            "remote" => "/remote".to_string(),
                            _ => "/".to_string(),
                        });
                        info!(
                            "Cliente WebSocket identificado como '{client_type}' rota '{active_route}' ip '{client_ip}'"
                        );
                        state.set_connection(&client_type, true).await;
                        state
                            .add_or_update_connection(
                                &conn_id,
                                &client_ip,
                                &active_route,
                                &client_type,
                            )
                            .await;
                        client_identity = Some(client_type);
                    }
                    Ok(WsMessage::SelectSlide {
                        media_id,
                        slide_index,
                    }) => {
                        state.select_slide(media_id, slide_index).await;
                    }
                    Ok(WsMessage::SetDisplayMode { mode }) => {
                        state.set_display_mode(mode).await;
                    }
                    Ok(WsMessage::StopPresentation) => {
                        state.stop_presentation().await;
                    }
                    Ok(WsMessage::SetWallpaper { path, mode }) => {
                        state.set_wallpaper(path, mode).await;
                    }
                    Ok(WsMessage::StartTimer) => {
                        state.start_timer().await;
                    }
                    Ok(WsMessage::PauseTimer) => {
                        state.pause_timer().await;
                    }
                    Ok(WsMessage::ResetTimer) => {
                        state.reset_timer().await;
                    }
                    Ok(WsMessage::SetTimer {
                        duration_seconds,
                        label,
                        allow_overtime,
                    }) => {
                        state
                            .set_timer(duration_seconds, label, allow_overtime)
                            .await;
                    }
                    Ok(WsMessage::SetTimerOvertime { allow_overtime }) => {
                        state.set_timer_overtime(allow_overtime).await;
                    }
                    Ok(WsMessage::QuickPresentMedia { item, slide_index }) => {
                        state
                            .quick_present_media_with_slide(item, slide_index.unwrap_or(0))
                            .await;
                    }
                    Ok(WsMessage::SetStageMessage { message }) => {
                        state.set_stage_message(message).await;
                    }
                    Ok(WsMessage::ReloadClients { target }) => {
                        let _ = state.broadcast_tx.send(WsMessage::ReloadClients { target });
                    }
                    Ok(WsMessage::Ping) => {
                        let _ = state.broadcast_tx.send(WsMessage::Pong);
                    }
                    Ok(_) => {}
                    Err(err) => {
                        warn!("Falha ao deserializar WsMessage: {err} (conteúdo: {text})");
                    }
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Ok(Message::Ping(_data)) => {
                // Axum responde pong automaticamente
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Binary(_)) => {}
            Err(err) => {
                error!("Erro na conexão WebSocket: {err}");
                break;
            }
        }
    }

    // Cancelar tarefa de envio
    send_task.abort();

    // 5. Limpeza de conexão
    state_for_cleanup
        .remove_connection(&conn_id_for_cleanup)
        .await;
    if let Some(ident) = client_identity {
        info!("Cliente '{ident}' desconectado do WebSocket");
        state_for_cleanup.set_connection(&ident, false).await;
    }
}

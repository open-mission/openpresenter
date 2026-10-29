use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use op_core::WsMessage;
use tracing::{debug, error, info, warn};

use crate::state::AppState;

pub fn create_ws_router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

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

    // 4. Loop de leitura de mensagens enviadas pelo cliente
    while let Some(result) = receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                debug!("Mensagem WebSocket recebida: {text}");
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(WsMessage::Identify { client_type }) => {
                        info!("Cliente WebSocket identificado como '{client_type}'");
                        state.set_connection(&client_type, true).await;
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
    if let Some(ident) = client_identity {
        info!("Cliente '{ident}' desconectado do WebSocket");
        state_for_cleanup.set_connection(&ident, false).await;
    }
}

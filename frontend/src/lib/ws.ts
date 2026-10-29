import { writable } from 'svelte/store';
import type { DisplayMode, MediaItem, PresentationState, WsMessage } from './types';

export const presentationState = writable<PresentationState>({
  active_media_id: null,
  active_slide_index: null,
  display_mode: 'live',
  public_connected: false,
  stage_connected: false,
});

export const mediaItemsList = writable<MediaItem[]>([]);
export const isWsConnected = writable<boolean>(false);

let socket: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
let pingInterval: ReturnType<typeof setInterval> | null = null;

export function initWebSocket(clientType: 'admin' | 'public' | 'stage' = 'admin') {
  if (socket && (socket.readyState === WebSocket.OPEN || socket.readyState === WebSocket.CONNECTING)) {
    return;
  }

  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const host = window.location.host;
  const wsUrl = `${protocol}//${host}/ws`;

  try {
    socket = new WebSocket(wsUrl);

    socket.onopen = () => {
      isWsConnected.set(true);
      if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
      }

      // Identifica o tipo do cliente
      sendWsMessage({
        type: 'identify',
        payload: { client_type: clientType },
      });

      // Configura ping periódico
      if (pingInterval) clearInterval(pingInterval);
      pingInterval = setInterval(() => {
        sendWsMessage({ type: 'ping' });
      }, 15000);
    };

    socket.onmessage = (event) => {
      try {
        const msg: WsMessage = JSON.parse(event.data);
        handleIncomingMessage(msg);
      } catch (err) {
        console.error('Erro ao processar mensagem WebSocket:', err);
      }
    };

    socket.onclose = () => {
      isWsConnected.set(false);
      if (pingInterval) clearInterval(pingInterval);
      // Tenta reconectar a cada 2 segundos
      if (!reconnectTimer) {
        reconnectTimer = setTimeout(() => {
          reconnectTimer = null;
          initWebSocket(clientType);
        }, 2000);
      }
    };

    socket.onerror = (err) => {
      console.warn('WebSocket error, tentando reconectar...', err);
      socket?.close();
    };
  } catch (err) {
    console.error('Falha ao instanciar WebSocket:', err);
  }
}

function handleIncomingMessage(msg: WsMessage) {
  if (msg.type === 'sync_state') {
    presentationState.set(msg.payload.presentation);
    mediaItemsList.set(msg.payload.media_items);
  } else if (msg.type === 'state_updated') {
    presentationState.set(msg.payload.presentation);
  }
}

export function sendWsMessage(msg: WsMessage) {
  if (socket && socket.readyState === WebSocket.OPEN) {
    socket.send(JSON.stringify(msg));
  }
}

export function selectSlide(mediaId: string, slideIndex: number) {
  sendWsMessage({
    type: 'select_slide',
    payload: { media_id: mediaId, slide_index: slideIndex },
  });

  // Também dispara fetch HTTP como fallback garantido
  fetch('/api/presentation/live', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ media_id: mediaId, slide_index: slideIndex }),
  }).catch(() => {});
}

export function setDisplayMode(mode: DisplayMode) {
  sendWsMessage({
    type: 'set_display_mode',
    payload: { mode },
  });

  fetch('/api/presentation/mode', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ mode }),
  }).catch(() => {});
}

export function toggleBlackout() {
  fetch('/api/presentation/blackout', { method: 'POST' }).catch(() => {});
}

export function toggleClearText() {
  fetch('/api/presentation/clear', { method: 'POST' }).catch(() => {});
}

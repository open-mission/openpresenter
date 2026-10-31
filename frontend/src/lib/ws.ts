import { writable } from 'svelte/store';
import type { DisplayMode, MediaItem, PresentationState, Setlist, WsMessage, ClientConnection, ActionLog } from './types';
import { handleSettingBroadcast } from './displaySettings';
import { getInstanceId, getDeviceInfo } from './instance';
import { handleActionLogNotification } from './toast';

export const presentationState = writable<PresentationState>({
  active_media_id: null,
  active_slide_index: null,
  display_mode: 'live',
  public_connected: false,
  stage_connected: false,
  active_setlist_id: undefined,
  wallpaper_path: undefined,
  wallpaper_mode: 'cover',
  timer: {
    running: false,
    duration_seconds: 600,
    remaining_seconds: 600,
    label: 'Cronômetro',
    stage_message: undefined,
    warning_threshold_seconds: 60,
  },
});

export const mediaItemsList = writable<MediaItem[]>([]);
export const activeSetlist = writable<Setlist | null>(null);
export const isWsConnected = writable<boolean>(false);
export const connectedClients = writable<ClientConnection[]>([]);
export const actionLogsStore = writable<ActionLog[]>([]);

let socket: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
let pingInterval: ReturnType<typeof setInterval> | null = null;

export function initWebSocket(clientType: 'admin' | 'public' | 'stage' | 'remote' | 'lower-third' = 'admin') {
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

      // Identifica o tipo do cliente e rota ativa
      const currentRoute = typeof window !== 'undefined' ? window.location.pathname : `/${clientType}`;
      sendWsMessage({
        type: 'identify',
        payload: {
          client_type: clientType,
          route: currentRoute,
          instance_id: getInstanceId(),
          device_info: getDeviceInfo(),
        },
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
  } else if (msg.type === 'media_items_updated') {
    mediaItemsList.set(msg.payload.media_items);
  } else if (msg.type === 'setlist_updated') {
    activeSetlist.set(msg.payload.setlist);
    mediaItemsList.set(msg.payload.setlist.items);
  } else if (msg.type === 'setting_updated') {
    handleSettingBroadcast(msg.payload.key, msg.payload.value);
  } else if (msg.type === 'connections_updated') {
    connectedClients.set(msg.payload.connections);
  } else if (msg.type === 'action_logged') {
    actionLogsStore.update((logs) => [msg.payload.log, ...logs.slice(0, 99)]);
    handleActionLogNotification(msg.payload.log);
  } else if (msg.type === 'reload_clients') {
    const target = msg.payload?.target || 'all';
    if (typeof window !== 'undefined') {
      const currentPath = window.location.pathname;
      const isPublic = currentPath.includes('/public');
      const isStage = currentPath.includes('/stage');
      if (
        target === 'all' ||
        (target === 'public' && isPublic) ||
        (target === 'stage' && isStage)
      ) {
        window.location.reload();
      }
    }
  }
}

export function reloadOutputs(target: 'all' | 'public' | 'stage' = 'all') {
  sendWsMessage({
    type: 'reload_clients',
    payload: { target },
  });
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

export function stopPresentation() {
  sendWsMessage({
    type: 'stop_presentation',
  });

  fetch('/api/presentation/stop', {
    method: 'POST',
  }).catch(() => {});
}

export function setPresentationWallpaper(path?: string, mode: string = 'cover') {
  sendWsMessage({
    type: 'set_wallpaper',
    payload: { path, mode },
  });

  fetch('/api/presentation/wallpaper', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ path, mode }),
  }).catch(() => {});
}

export function toggleBlackout() {
  fetch('/api/presentation/blackout', { method: 'POST' }).catch(() => {});
}

export function toggleClearText() {
  fetch('/api/presentation/clear', { method: 'POST' }).catch(() => {});
}

export async function removeSetlistItem(itemId: string) {
  try {
    await fetch('/api/setlists/items/remove', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ item_id: itemId }),
    });
  } catch (err) {
    console.error('Erro ao remover item da setlist:', err);
  }
}

export async function clearSetlistItems() {
  try {
    await fetch('/api/setlists/items/clear', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (err) {
    console.error('Erro ao limpar itens da setlist:', err);
  }
}

export async function renameSetlistItem(itemId: string, newTitle: string) {
  try {
    await fetch('/api/setlists/items/rename', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ item_id: itemId, new_title: newTitle }),
    });
  } catch (err) {
    console.error('Erro ao renomear item da setlist:', err);
  }
}

export async function reorderSetlistItems(items: MediaItem[]) {
  try {
    await fetch('/api/setlists/items/reorder', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ items }),
    });
  } catch (err) {
    console.error('Erro ao reordenar setlist:', err);
  }
}

export async function switchActiveSetlist(setlistId: string) {
  try {
    const res = await fetch(`/api/setlists/${setlistId}/active`, {
      method: 'POST',
    });
    if (res.ok) {
      const data: Setlist = await res.json();
      activeSetlist.set(data);
      mediaItemsList.set(data.items);
    }
  } catch (err) {
    console.error('Erro ao ativar setlist:', err);
  }
}

export function startTimer() {
  sendWsMessage({ type: 'start_timer' });
  fetch('/api/timer/start', { method: 'POST' }).catch(() => {});
}

export function pauseTimer() {
  sendWsMessage({ type: 'pause_timer' });
  fetch('/api/timer/pause', { method: 'POST' }).catch(() => {});
}

export function resetTimer() {
  sendWsMessage({ type: 'reset_timer' });
  fetch('/api/timer/reset', { method: 'POST' }).catch(() => {});
}

export function setTimer(durationSeconds: number, label?: string, allowOvertime?: boolean) {
  sendWsMessage({
    type: 'set_timer',
    payload: { duration_seconds: durationSeconds, label, allow_overtime: allowOvertime },
  });
  fetch('/api/timer/set', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ duration_seconds: durationSeconds, label, allow_overtime: allowOvertime }),
  }).catch(() => {});
}

export function setTimerOvertime(allowOvertime: boolean) {
  sendWsMessage({
    type: 'set_timer_overtime',
    payload: { allow_overtime: allowOvertime },
  });
  fetch('/api/timer/overtime', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ allow_overtime: allowOvertime }),
  }).catch(() => {});
}

export function quickPresentMedia(item: MediaItem, slideIndex: number = 0) {
  sendWsMessage({
    type: 'quick_present_media',
    payload: { item, slide_index: slideIndex },
  });
  fetch('/api/presentation/quick-media', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ item, slide_index: slideIndex }),
  }).catch(() => {});
}

export function setStageMessage(message?: string) {
  sendWsMessage({
    type: 'set_stage_message',
    payload: { message },
  });
  fetch('/api/timer/message', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ message }),
  }).catch(() => {});
}

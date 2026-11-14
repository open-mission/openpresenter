import { writable } from 'svelte/store';
import type { DisplayMode, MediaItem, PresentationState, Setlist, WsMessage, ClientConnection, ActionLog } from './types';
import { handleSettingBroadcast } from './displaySettings';
import { getInstanceId, getDeviceInfo } from './instance';
import { handleActionLogNotification, notifyError } from './toast';

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
export const teamUpdatedStore = writable<{ entity: string; action: string; timestamp: number } | null>(null);
export const audioControlCommandStore = writable<{
  action: 'play' | 'pause' | 'stop' | 'seek';
  url?: string;
  title?: string;
  artist_or_ref?: string;
  seek_seconds?: number;
  timestamp: number;
} | null>(null);

let socket: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
let pingInterval: ReturnType<typeof setInterval> | null = null;
let reconnectAttempts = 0;
let activeClientType: 'admin' | 'public' | 'stage' | 'remote' | 'lower-third' = 'admin';
let visibilityListenerAttached = false;
let shouldReconnect = true;

function scheduleReconnect() {
  if (!shouldReconnect || reconnectTimer) return;
  if (typeof document !== 'undefined' && document.hidden) return;
  const delayMs = Math.min(2000 * Math.pow(2, reconnectAttempts), 15000);
  reconnectAttempts = Math.min(reconnectAttempts + 1, 4);
  reconnectTimer = setTimeout(() => {
    reconnectTimer = null;
    initWebSocket(activeClientType);
  }, delayMs);
}

function attachVisibilityResume() {
  if (visibilityListenerAttached || typeof document === 'undefined') return;
  visibilityListenerAttached = true;
  document.addEventListener('visibilitychange', () => {
    if (!document.hidden && shouldReconnect && (!socket || socket.readyState === WebSocket.CLOSED)) {
      reconnectAttempts = 0;
      initWebSocket(activeClientType);
    }
  });
}

export function closeWebSocket() {
  shouldReconnect = false;
  if (reconnectTimer) {
    clearTimeout(reconnectTimer);
    reconnectTimer = null;
  }
  if (pingInterval) {
    clearInterval(pingInterval);
    pingInterval = null;
  }
  if (socket) {
    socket.onclose = null;
    socket.close();
    socket = null;
  }
  isWsConnected.set(false);
}

export function initWebSocket(clientType: 'admin' | 'public' | 'stage' | 'remote' | 'lower-third' = 'admin') {
  activeClientType = clientType;
  shouldReconnect = true;
  attachVisibilityResume();

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
      reconnectAttempts = 0;
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
      if (pingInterval) {
        clearInterval(pingInterval);
        pingInterval = null;
      }
      scheduleReconnect();
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
  } else if (msg.type === 'team_updated') {
    teamUpdatedStore.set({
      entity: msg.payload.entity,
      action: msg.payload.action,
      timestamp: Date.now(),
    });
  } else if (msg.type === 'control_audio_playback') {
    audioControlCommandStore.set({
      ...msg.payload,
      timestamp: Date.now(),
    });
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

let lastCommandErrorAt = 0;

function reportCommandFailure(label: string) {
  const now = Date.now();
  if (now - lastCommandErrorAt < 4000) return;
  lastCommandErrorAt = now;
  notifyError('Falha ao comunicar com o servidor', `Não foi possível executar: ${label}. Verifique se o backend está online.`);
}

async function postCommand(url: string, body?: unknown, label = 'comando') {
  try {
    const res = await fetch(url, {
      method: 'POST',
      headers: body !== undefined ? { 'Content-Type': 'application/json' } : undefined,
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });
    if (!res.ok) {
      reportCommandFailure(label);
    }
  } catch {
    reportCommandFailure(label);
  }
}

export function reconnectWebSocket(clientType: 'admin' | 'public' | 'stage' | 'remote' | 'lower-third' = 'admin') {
  if (reconnectTimer) {
    clearTimeout(reconnectTimer);
    reconnectTimer = null;
  }
  initWebSocket(clientType);
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
  void postCommand('/api/presentation/live', { media_id: mediaId, slide_index: slideIndex }, 'selecionar slide');
}

export function setDisplayMode(mode: DisplayMode) {
  sendWsMessage({
    type: 'set_display_mode',
    payload: { mode },
  });

  void postCommand('/api/presentation/mode', { mode }, `alterar saída (${mode})`);
}

export function stopPresentation() {
  sendWsMessage({
    type: 'stop_presentation',
  });

  void postCommand('/api/presentation/stop', undefined, 'parar projeção');
}

export function setPresentationWallpaper(path?: string, mode: string = 'cover') {
  sendWsMessage({
    type: 'set_wallpaper',
    payload: { path, mode },
  });

  void postCommand('/api/presentation/wallpaper', { path, mode }, 'alterar papel de parede');
}

export function toggleBlackout() {
  void postCommand('/api/presentation/blackout', undefined, 'alternar blackout');
}

export function toggleClearText() {
  void postCommand('/api/presentation/clear', undefined, 'limpar texto');
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
    const res = await fetch('/api/setlists/items/reorder', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ items }),
    });
    if (!res.ok) reportCommandFailure('reordenar setlist');
  } catch (err) {
    console.error('Erro ao reordenar setlist:', err);
    reportCommandFailure('reordenar setlist');
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
    } else {
      reportCommandFailure('trocar setlist');
    }
  } catch (err) {
    console.error('Erro ao ativar setlist:', err);
    reportCommandFailure('trocar setlist');
  }
}

export function startTimer() {
  sendWsMessage({ type: 'start_timer' });
  void postCommand('/api/timer/start', undefined, 'iniciar cronômetro');
}

export function pauseTimer() {
  sendWsMessage({ type: 'pause_timer' });
  void postCommand('/api/timer/pause', undefined, 'pausar cronômetro');
}

export function resetTimer() {
  sendWsMessage({ type: 'reset_timer' });
  void postCommand('/api/timer/reset', undefined, 'reiniciar cronômetro');
}

export function setTimer(durationSeconds: number, label?: string, allowOvertime?: boolean) {
  sendWsMessage({
    type: 'set_timer',
    payload: { duration_seconds: durationSeconds, label, allow_overtime: allowOvertime },
  });
  void postCommand('/api/timer/set', { duration_seconds: durationSeconds, label, allow_overtime: allowOvertime }, 'definir cronômetro');
}

export function setTimerOvertime(allowOvertime: boolean) {
  sendWsMessage({
    type: 'set_timer_overtime',
    payload: { allow_overtime: allowOvertime },
  });
  void postCommand('/api/timer/overtime', { allow_overtime: allowOvertime }, 'alternar hora extra');
}

export function quickPresentMedia(item: MediaItem, slideIndex: number = 0) {
  sendWsMessage({
    type: 'quick_present_media',
    payload: { item, slide_index: slideIndex },
  });
  void postCommand('/api/presentation/quick-media', { item, slide_index: slideIndex }, 'apresentar mídia rápida');
}

export function setStageMessage(message?: string) {
  sendWsMessage({
    type: 'set_stage_message',
    payload: { message },
  });
  void postCommand('/api/timer/message', { message }, 'enviar mensagem de palco');
}

export function updateAudioPlayback(
  isPlaying: boolean,
  currentTimeSeconds: number,
  durationSeconds: number,
  title: string,
  artistOrRef?: string
) {
  sendWsMessage({
    type: 'update_audio_playback',
    payload: {
      is_playing: isPlaying,
      current_time_seconds: currentTimeSeconds,
      duration_seconds: durationSeconds,
      title,
      artist_or_ref: artistOrRef,
    },
  });
  void postCommand('/api/presentation/audio-playback', {
    is_playing: isPlaying,
    current_time_seconds: currentTimeSeconds,
    duration_seconds: durationSeconds,
    title,
    artist_or_ref: artistOrRef,
  }, 'atualizar reprodução de áudio');
}

export function clearAudioPlayback() {
  sendWsMessage({
    type: 'clear_audio_playback',
  });
  void postCommand('/api/presentation/audio-playback/clear', undefined, 'limpar reprodução de áudio');
}

export function controlAudioPlayback(
  action: 'play' | 'pause' | 'stop' | 'seek',
  options?: {
    url?: string;
    title?: string;
    artist_or_ref?: string;
    seek_seconds?: number;
  }
) {
  sendWsMessage({
    type: 'control_audio_playback',
    payload: {
      action,
      url: options?.url,
      title: options?.title,
      artist_or_ref: options?.artist_or_ref,
      seek_seconds: options?.seek_seconds ? Math.round(options.seek_seconds) : undefined,
    },
  });
}

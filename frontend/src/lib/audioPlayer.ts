/**
 * Gerenciador global de reprodução de áudio e playback de músicas (OpenPresenter).
 * Controla a instância de áudio local no console do operador, sincroniza em tempo real
 * com o backend via WebSocket (alimentando o Monitor de Palco e a Projeção) e reage
 * a comandos remotos disparados via WebSocket pelo Remoto Mobile (/remote).
 */
import { writable, get } from 'svelte/store';
import { updateAudioPlayback, clearAudioPlayback, audioControlCommandStore } from './ws';

export interface AudioTrack {
  id?: string;
  title: string;
  url: string;
  artist_or_ref?: string;
}

export const currentTrackStore = writable<AudioTrack | null>(null);
export const isAudioPlayingStore = writable<boolean>(false);
export const audioCurrentTimeStore = writable<number>(0);
export const audioDurationStore = writable<number>(0);
export const audioVolumeStore = writable<number>(1.0);
export const isAudioMutedStore = writable<boolean>(false);
export const isAudioLoopingStore = writable<boolean>(false);
export const audioErrorStore = writable<string | null>(null);

let audioElement: HTMLAudioElement | null = null;
let lastWsSyncSec = -1;
let isInitialized = false;

function ensureAudioElement(): HTMLAudioElement {
  if (!audioElement && typeof window !== 'undefined') {
    audioElement = new Audio();
    audioElement.preload = 'metadata';
    audioElement.loop = get(isAudioLoopingStore);
    audioElement.volume = get(audioVolumeStore);
    audioElement.muted = get(isAudioMutedStore);

    audioElement.ontimeupdate = () => {
      if (!audioElement) return;
      const cur = audioElement.currentTime;
      const dur = audioElement.duration || 0;
      audioCurrentTimeStore.set(cur);
      audioDurationStore.set(dur);

      const secFloor = Math.floor(cur);
      if (secFloor !== lastWsSyncSec) {
        lastWsSyncSec = secFloor;
        const track = get(currentTrackStore);
        if (track) {
          updateAudioPlayback(
            !audioElement.paused,
            cur,
            dur,
            track.title,
            track.artist_or_ref
          );
        }
      }
    };

    audioElement.onloadedmetadata = () => {
      if (!audioElement) return;
      const dur = audioElement.duration || 0;
      audioDurationStore.set(dur);
    };

    audioElement.ondurationchange = () => {
      if (!audioElement) return;
      const dur = audioElement.duration || 0;
      audioDurationStore.set(dur);
    };

    audioElement.onplay = () => {
      audioErrorStore.set(null);
      isAudioPlayingStore.set(true);
      const track = get(currentTrackStore);
      if (track && audioElement) {
        updateAudioPlayback(
          true,
          audioElement.currentTime,
          audioElement.duration || 0,
          track.title,
          track.artist_or_ref
        );
      }
    };

    audioElement.onpause = () => {
      isAudioPlayingStore.set(false);
      const track = get(currentTrackStore);
      if (track && audioElement) {
        updateAudioPlayback(
          false,
          audioElement.currentTime,
          audioElement.duration || 0,
          track.title,
          track.artist_or_ref
        );
      }
    };

    audioElement.onended = () => {
      const isLoop = get(isAudioLoopingStore);
      if (isLoop && audioElement) {
        audioElement.currentTime = 0;
        audioElement.play().catch(() => {});
      } else {
        isAudioPlayingStore.set(false);
        audioCurrentTimeStore.set(0);
        clearAudioPlayback();
      }
    };

    audioElement.onerror = () => {
      isAudioPlayingStore.set(false);
      audioErrorStore.set('Não foi possível carregar ou reproduzir o arquivo de áudio selecionado.');
      clearAudioPlayback();
    };
  }
  return audioElement!;
}

export function initAudioPlayer() {
  if (isInitialized || typeof window === 'undefined') return;
  isInitialized = true;
  ensureAudioElement();

  // Escuta comandos remotos disparados via WebSocket (ex: /remote)
  let lastTimestamp = 0;
  audioControlCommandStore.subscribe((cmd) => {
    if (!cmd || cmd.timestamp === lastTimestamp) return;
    lastTimestamp = cmd.timestamp;

    if (cmd.action === 'play') {
      if (cmd.url) {
        playTrack(cmd.url, cmd.title || 'Playback', cmd.artist_or_ref);
      } else {
        resumeTrack();
      }
    } else if (cmd.action === 'pause') {
      pauseTrack();
    } else if (cmd.action === 'stop') {
      stopTrack();
    } else if (cmd.action === 'seek' && typeof cmd.seek_seconds === 'number') {
      seekTrack(cmd.seek_seconds);
    }
  });
}

export function prepareTrack(
  trackOrUrl: AudioTrack | string,
  title?: string,
  artist_or_ref?: string
) {
  const el = ensureAudioElement();
  if (get(isAudioPlayingStore)) return; // não interrompe áudio em reprodução

  const track: AudioTrack =
    typeof trackOrUrl === 'string'
      ? {
          url: trackOrUrl,
          title: title || trackOrUrl.split('/').pop() || 'Áudio',
          artist_or_ref,
        }
      : trackOrUrl;

  if (!track.url) return;

  const current = get(currentTrackStore);
  if (current?.url === track.url && (el.src === track.url || el.src.endsWith(track.url))) {
    return;
  }

  currentTrackStore.set(track);
  el.src = track.url;
  el.currentTime = 0;
  audioCurrentTimeStore.set(0);
}

export function playTrack(
  trackOrUrl: AudioTrack | string,
  title?: string,
  artist_or_ref?: string
) {
  const el = ensureAudioElement();
  const track: AudioTrack =
    typeof trackOrUrl === 'string'
      ? {
          url: trackOrUrl,
          title: title || trackOrUrl.split('/').pop() || 'Áudio',
          artist_or_ref,
        }
      : trackOrUrl;

  if (!track.url) {
    console.warn('URL de áudio inválida para reprodução:', track);
    return;
  }

  const current = get(currentTrackStore);

  // Se já for a mesma faixa e estiver pausada, retoma
  if (current?.url === track.url && (el.src === track.url || el.src.endsWith(track.url))) {
    el.play().catch((err) => console.warn('Falha ao reproduzir áudio:', err));
    return;
  }

  audioErrorStore.set(null);
  currentTrackStore.set(track);
  el.src = track.url;
  el.currentTime = 0;
  audioCurrentTimeStore.set(0);
  lastWsSyncSec = -1;

  el.play()
    .then(() => {
      updateAudioPlayback(true, 0, el.duration || 0, track.title, track.artist_or_ref);
    })
    .catch((err) => {
      console.warn('Falha ao iniciar reprodução:', err);
      isAudioPlayingStore.set(false);
      audioErrorStore.set('Falha ao iniciar reprodução de áudio.');
    });
}

export function togglePlayTrack(
  trackOrUrl?: AudioTrack | string,
  title?: string,
  artist_or_ref?: string
) {
  const el = ensureAudioElement();
  const isPlaying = get(isAudioPlayingStore);
  const current = get(currentTrackStore);

  if (trackOrUrl) {
    const trackUrl = typeof trackOrUrl === 'string' ? trackOrUrl : trackOrUrl.url;
    if (!current || current.url !== trackUrl) {
      playTrack(trackOrUrl, title, artist_or_ref);
      return;
    }
  }

  if (isPlaying) {
    pauseTrack();
  } else {
    resumeTrack();
  }
}

export function pauseTrack() {
  if (!audioElement) return;
  audioElement.pause();
  isAudioPlayingStore.set(false);
}

export function resumeTrack() {
  const el = ensureAudioElement();
  if (el.src) {
    el.play().catch((err) => console.warn('Falha ao retomar áudio:', err));
  }
}

export function stopTrack() {
  if (!audioElement) return;
  audioElement.pause();
  audioElement.currentTime = 0;
  isAudioPlayingStore.set(false);
  audioCurrentTimeStore.set(0);
  clearAudioPlayback();
}

export function resetTrack() {
  if (audioElement) {
    audioElement.pause();
    audioElement.removeAttribute('src');
    audioElement.load();
  }
  isAudioPlayingStore.set(false);
  audioCurrentTimeStore.set(0);
  audioDurationStore.set(0);
  currentTrackStore.set(null);
  clearAudioPlayback();
}

export function seekTrack(seconds: number) {
  if (!audioElement) return;
  const clamped = Math.max(0, Math.min(seconds, audioElement.duration || 0));
  audioElement.currentTime = clamped;
  audioCurrentTimeStore.set(clamped);
  const track = get(currentTrackStore);
  if (track) {
    updateAudioPlayback(
      !audioElement.paused,
      clamped,
      audioElement.duration || 0,
      track.title,
      track.artist_or_ref
    );
  }
}

export function setTrackVolume(vol: number) {
  const clamped = Math.max(0, Math.min(vol, 1.0));
  audioVolumeStore.set(clamped);
  if (clamped === 0) {
    isAudioMutedStore.set(true);
  } else if (get(isAudioMutedStore)) {
    isAudioMutedStore.set(false);
  }
  if (audioElement) {
    audioElement.volume = clamped;
    audioElement.muted = clamped === 0;
  }
}

export function toggleTrackMute() {
  const muted = !get(isAudioMutedStore);
  isAudioMutedStore.set(muted);
  if (!muted && get(audioVolumeStore) === 0) {
    audioVolumeStore.set(0.5);
    if (audioElement) audioElement.volume = 0.5;
  }
  if (audioElement) {
    audioElement.muted = muted;
  }
}

export function toggleTrackLoop(): boolean {
  const nextLoop = !get(isAudioLoopingStore);
  isAudioLoopingStore.set(nextLoop);
  if (audioElement) {
    audioElement.loop = nextLoop;
  }
  return nextLoop;
}

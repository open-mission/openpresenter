import { readHttpError } from './media';
import type { AudioPlaybackState, MediaItem } from './types';

export const REMOTE_SAFE_DISPATCH_KEY = 'openpresenter:remote-safe-dispatch';

export async function fetchJsonWithError<T>(
  url: string,
  fallbackError: string,
  init?: RequestInit
): Promise<T> {
  const res = await fetch(url, init);
  if (!res.ok) {
    throw new Error(await readHttpError(res, fallbackError));
  }
  return (await res.json()) as T;
}

export async function verifyRemoteOperatorPin(
  inputPin: string
): Promise<{ ok: boolean; error?: string }> {
  const cleanPin = inputPin.trim();
  if (!cleanPin) {
    return { ok: false, error: 'Digite o PIN numérico do operador.' };
  }

  try {
    const res = await fetch('/api/settings/remote_operator_pin');
    let expectedPin = '1234';

    if (res.ok) {
      const data = (await res.json()) as { value?: string | null };
      const serverValue = data?.value?.trim();
      if (serverValue) {
        expectedPin = serverValue;
      }
    } else if (res.status !== 404) {
      return {
        ok: false,
        error: await readHttpError(res, 'Não foi possível validar o PIN no servidor.'),
      };
    }

    if (cleanPin === expectedPin) {
      return { ok: true };
    }
    return { ok: false, error: 'PIN incorreto. Confirme o código com o operador da mesa.' };
  } catch {
    return {
      ok: false,
      error: 'Servidor da mesa inacessível para validar o PIN. Verifique a conexão Wi-Fi.',
    };
  }
}

export function getInitials(name: string): string {
  const parts = name.trim().split(/\s+/).filter(Boolean);
  if (parts.length === 0) return 'OP';
  if (parts.length === 1) return parts[0].substring(0, 2).toUpperCase();
  return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
}

export function formatMediaTime(seconds?: number): string {
  if (!seconds || isNaN(seconds) || seconds < 0) return '00:00';
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
}

export function isSameAudioTrack(
  playback: AudioPlaybackState | null | undefined,
  item: Pick<MediaItem, 'title' | 'artist_or_ref' | 'audio_url'>
): boolean {
  if (!playback) return false;
  if (playback.url && item.audio_url) {
    return playback.url === item.audio_url;
  }
  if (playback.title !== item.title) return false;
  if (playback.artist_or_ref && item.artist_or_ref) {
    return playback.artist_or_ref === item.artist_or_ref;
  }
  return true;
}

export function getStoredSafeDispatch(): boolean {
  if (typeof window === 'undefined') return true;
  const raw = localStorage.getItem(REMOTE_SAFE_DISPATCH_KEY);
  return raw === null ? true : raw !== 'false';
}

export function setStoredSafeDispatch(enabled: boolean): void {
  if (typeof window === 'undefined') return;
  localStorage.setItem(REMOTE_SAFE_DISPATCH_KEY, String(enabled));
}

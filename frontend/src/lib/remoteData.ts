import { readHttpError } from './media';
import { parseRawLyrics } from './songParser';
import type { AudioPlaybackState, MediaItem, Slide } from './types';

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

/**
 * Dispara vibração tátil suave no dispositivo móvel para confirmação física de toques.
 */
export function triggerHaptic(type: 'light' | 'medium' | 'warning' = 'light'): void {
  if (typeof window === 'undefined' || !('vibrate' in navigator)) return;
  try {
    if (type === 'light') {
      navigator.vibrate(12);
    } else if (type === 'medium') {
      navigator.vibrate([18, 20, 18]);
    } else if (type === 'warning') {
      navigator.vibrate([35, 30, 45]);
    }
  } catch {
    // Ignorado em ambientes sem suporte
  }
}

/**
 * Extrai de forma resiliente os slides de uma música retornada pela API ou setlist,
 * suportando `slides`, `lyrics_raw`, `sections` e `lyrics`.
 */
export function extractSongSlides(song: unknown, fallbackId = 'song'): Slide[] {
  if (!song || typeof song !== 'object') return [];

  const candidate = song as {
    id?: string;
    slides?: Slide[];
    lyrics_raw?: string;
    lyrics?: string;
    sections?: Array<{
      label?: string;
      section_type?: string;
      lines?: string[];
    }>;
  };

  const idPrefix = candidate.id || fallbackId;

  // 1. Já possui slides previamente mapeados
  if (Array.isArray(candidate.slides) && candidate.slides.length > 0) {
    return candidate.slides;
  }

  // 2. Possui texto raw em Markdown (lyrics_raw ou lyrics)
  const rawText = candidate.lyrics_raw || candidate.lyrics;
  if (rawText && typeof rawText === 'string' && rawText.trim() !== '') {
    const parsed = parseRawLyrics(rawText);
    if (parsed.slides.length > 0) {
      return parsed.slides.map((s, idx) => ({
        ...s,
        id: s.id || `${idPrefix}-s-${idx}`,
      }));
    }
  }

  // 3. Possui array estruturado de seções
  if (Array.isArray(candidate.sections) && candidate.sections.length > 0) {
    const slides: Slide[] = [];
    let slideIdx = 0;
    for (const sec of candidate.sections) {
      const lines = Array.isArray(sec.lines) ? sec.lines : [];
      if (lines.length > 0) {
        slides.push({
          id: `${idPrefix}-sec-${slideIdx}`,
          label: sec.label || (sec.section_type ? sec.section_type.toUpperCase() : `Estrofe ${slideIdx + 1}`),
          content: lines.join('\n'),
        });
        slideIdx += 1;
      }
    }
    if (slides.length > 0) return slides;
  }

  return [];
}

import type { MediaItem, Song } from '$lib/types';
import type { AlignH, AlignV, SongEditorMode } from './types';

export interface EditorInitialValues {
  id: string;
  title: string;
  artist: string;
  composer: string;
  songKey: string;
  bpm: number | undefined;
  tags: string[];
  audioUrl: string;
  lyricsRaw: string;
  alignH: AlignH;
  alignV: AlignV;
}

const DEFAULT_SONG_LYRICS = `## Verso 1
Primeira estrofe da canção
Segunda linha aqui

Outro slide deste verso
Linha vazia separou

## Refrão
Porque Ele vive, posso crer no amanhã
Porque Ele vive, temor não há`;

const DEFAULT_TEXT_LYRICS = `## Boas-vindas
Sejam bem-vindos à nossa reunião!
Estamos felizes com sua presença.

## Próximo Evento
Domingo às 19h00: Culto de Celebração.
Traga sua família e convidados!`;

export function getInitialValues(
  mode: SongEditorMode,
  initialSong: Partial<Song> | null,
  initialItem: Partial<MediaItem> | null
): EditorInitialValues {
  const stamp = Date.now();

  if (mode === 'song') {
    if (initialSong) {
      return {
        id: initialSong.id || `song-${stamp}`,
        title: initialSong.title || '',
        artist: initialSong.artist || '',
        composer: initialSong.composer || '',
        songKey: initialSong.song_key || '',
        bpm: initialSong.bpm,
        tags: initialSong.tags ? [...initialSong.tags] : [],
        audioUrl: initialSong.audio_url || '',
        lyricsRaw: initialSong.lyrics_raw || '',
        alignH: 'center',
        alignV: 'center',
      };
    }

    return {
      id: `song-${stamp}`,
      title: '',
      artist: '',
      composer: '',
      songKey: '',
      bpm: undefined,
      tags: [],
      audioUrl: '',
      lyricsRaw: DEFAULT_SONG_LYRICS,
      alignH: 'center',
      alignV: 'center',
    };
  }

  if (initialItem) {
    return {
      id: initialItem.id || `text-${stamp}`,
      title: initialItem.title || '',
      artist: initialItem.artist_or_ref || '',
      composer: '',
      songKey: '',
      bpm: undefined,
      tags: [],
      audioUrl: initialItem.audio_url || '',
      lyricsRaw: initialItem.slides?.map((slide) => slide.content).join('\n\n') || '',
      alignH: initialItem.align_h || 'center',
      alignV: initialItem.align_v || 'center',
    };
  }

  return {
    id: `text-${stamp}`,
    title: '',
    artist: 'Avisos da Semana',
    composer: '',
    songKey: '',
    bpm: undefined,
    tags: [],
    audioUrl: '',
    lyricsRaw: DEFAULT_TEXT_LYRICS,
    alignH: 'center',
    alignV: 'center',
  };
}

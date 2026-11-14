import type { MediaItem, Slide, Song, SongSection } from '$lib/types';
import type { AlignH, AlignV } from './types';

export interface SongPayloadInput {
  id: string;
  title: string;
  artist: string;
  composer: string;
  songKey: string;
  bpm: number | undefined;
  lyricsRaw: string;
  tags: string[];
  audioUrl: string;
}

export function buildSongPayload(input: SongPayloadInput, sections: SongSection[]): Song {
  return {
    id: input.id || `song-${Date.now()}`,
    title: input.title.trim(),
    artist: input.artist.trim(),
    composer: input.composer.trim() || undefined,
    song_key: input.songKey.trim() || undefined,
    bpm: input.bpm ? Number(input.bpm) : undefined,
    sections,
    lyrics_raw: input.lyricsRaw,
    tags: input.tags,
    audio_url: input.audioUrl.trim() || undefined,
  };
}

export function buildTextItemPayload(
  input: Pick<SongPayloadInput, 'id' | 'title' | 'artist' | 'audioUrl'>,
  slides: Slide[],
  alignH: AlignH,
  alignV: AlignV
): MediaItem {
  return {
    id: input.id || `text-${Date.now()}`,
    title: input.title.trim(),
    artist_or_ref: input.artist.trim() || undefined,
    kind: 'announcement',
    slides: slides.map((slide) => ({ ...slide, align_h: alignH, align_v: alignV })),
    align_h: alignH,
    align_v: alignV,
    audio_url: input.audioUrl.trim() || undefined,
  };
}

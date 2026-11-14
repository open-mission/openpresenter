import type {
  BibleSearchResult,
  BibleVerse,
  MediaItem,
  ParsedReference,
  Slide,
} from '../../types';

function scriptureSlide(label: string, content: string): Slide {
  return {
    id: `v-${label}`,
    label,
    content,
    align_h: 'center',
    align_v: 'center',
  };
}

function scriptureItem(title: string, abbr: string, slides: Slide[]): MediaItem {
  return {
    id: `scripture-${Date.now()}`,
    title,
    artist_or_ref: abbr,
    kind: 'scripture',
    slides,
    align_h: 'center',
    align_v: 'center',
  };
}

export function verseToMediaItem(
  verse: BibleVerse,
  passageRef: ParsedReference | null,
  abbr: string
): MediaItem {
  const bookName = passageRef?.book_name || 'Bíblia';
  const chapter = passageRef?.chapter || verse.chapter;
  const label = `${bookName} ${chapter}:${verse.verse}`;
  return scriptureItem(label, abbr, [scriptureSlide(label, verse.text)]);
}

export function passageToMediaItem(
  verses: BibleVerse[],
  passageRef: ParsedReference,
  abbr: string
): MediaItem {
  const first = verses[0].verse;
  const last = verses[verses.length - 1].verse;
  const range = first === last ? `${first}` : `${first}-${last}`;
  const title = `${passageRef.book_name} ${passageRef.chapter}:${range}`;
  const slides = verses.map((verse) =>
    scriptureSlide(`${passageRef.book_name} ${passageRef.chapter}:${verse.verse}`, verse.text)
  );
  return scriptureItem(title, abbr, slides);
}

export function searchResultToMediaItem(result: BibleSearchResult, abbr: string): MediaItem {
  const label = `${result.book_name} ${result.chapter}:${result.verse}`;
  return scriptureItem(label, abbr, [scriptureSlide(label, result.text)]);
}

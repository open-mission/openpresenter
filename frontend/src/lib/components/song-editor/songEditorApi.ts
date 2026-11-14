import type { MediaItem, Song } from '$lib/types';

async function postJson(url: string, body: unknown, fallbackMessage: string): Promise<Response> {
  const response = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(errorText || fallbackMessage);
  }

  return response;
}

export async function saveSong(song: Song, addToSetlist: boolean): Promise<Song> {
  const response = await postJson(
    '/api/songs',
    { song, add_to_setlist: addToSetlist },
    'Erro ao salvar música'
  );
  return (await response.json()) as Song;
}

export async function saveTextPresentation(item: MediaItem): Promise<void> {
  await postJson('/api/setlist/add', { item }, 'Erro ao salvar texto na setlist');
}

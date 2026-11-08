import type { ImageMediaItem, AudioMediaItem, VideoMediaItem, MediaSubfolder } from './types';

export async function fetchImages(): Promise<ImageMediaItem[]> {
  const res = await fetch('/api/media/images');
  if (!res.ok) throw new Error('Falha ao carregar imagens');
  return res.json();
}

export async function fetchAppSubfolders(kind: 'images' | 'videos' = 'images'): Promise<MediaSubfolder[]> {
  try {
    const res = await fetch(`/api/media/app-subfolders?kind=${kind}`);
    if (!res.ok) return [];
    return res.json();
  } catch {
    return [];
  }
}

export async function createMediaDirectory(
  name: string,
  kind: 'images' | 'videos' = 'images',
  folderId: string = 'app',
  parentSubfolder?: string | null
): Promise<{ success: boolean; name: string; relative_path: string; path: string }> {
  const res = await fetch('/api/media/folders/create-dir', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      kind,
      folder_id: folderId,
      parent_subfolder: parentSubfolder || undefined,
      name,
    }),
  });
  if (!res.ok) {
    const err = await res.text();
    throw new Error(err || 'Falha ao criar pasta');
  }
  return res.json();
}

export async function uploadImage(
  filename: string,
  contentBase64: string,
  folderId?: string,
  subfolder?: string
): Promise<{ url: string; filename: string }> {
  const res = await fetch('/api/media/images/upload', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      filename,
      content_base64: contentBase64,
      folder_id: folderId,
      subfolder,
    }),
  });
  if (!res.ok) throw new Error('Falha ao enviar imagem');
  return res.json();
}

export async function fetchVideos(): Promise<VideoMediaItem[]> {
  const res = await fetch('/api/media/videos');
  if (!res.ok) throw new Error('Falha ao carregar vídeos');
  return res.json();
}

export async function uploadVideo(
  filename: string,
  contentBase64: string,
  folderId?: string,
  subfolder?: string
): Promise<{ url: string; filename: string }> {
  const res = await fetch('/api/media/videos/upload', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      filename,
      content_base64: contentBase64,
      folder_id: folderId,
      subfolder,
    }),
  });
  if (!res.ok) throw new Error('Falha ao enviar vídeo');
  return res.json();
}

export async function fetchAudios(): Promise<AudioMediaItem[]> {
  const res = await fetch('/api/media/audios');
  if (!res.ok) throw new Error('Falha ao carregar áudios');
  return res.json();
}

export async function uploadAudio(
  filename: string,
  contentBase64: string
): Promise<{ url: string; filename: string }> {
  const res = await fetch('/api/media/audios/upload', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ filename, content_base64: contentBase64 }),
  });
  if (!res.ok) throw new Error('Falha ao enviar áudio');
  return res.json();
}

export function formatBytes(bytes: number): string {
  if (bytes <= 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

export function formatDuration(seconds: number): string {
  if (isNaN(seconds) || seconds < 0) return '00:00';
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
}

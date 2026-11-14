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
    throw new Error(await readHttpError(res, 'Falha ao criar pasta'));
  }
  return res.json();
}

export function readFileAsDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const result = typeof reader.result === 'string' ? reader.result : '';
      if (!result) {
        reject(new Error(`Arquivo "${file.name}" vazio ou ilegível.`));
        return;
      }
      resolve(result);
    };
    reader.onerror = () => {
      reject(new Error(`Falha ao ler arquivo "${file.name}".`));
    };
    reader.readAsDataURL(file);
  });
}

export function readFileAsBase64(file: File): Promise<string> {
  return readFileAsDataUrl(file).then((result) => {
    const b64 = result.includes(',') ? result.split(',')[1] : result;
    if (!b64) {
      throw new Error(`Arquivo "${file.name}" vazio ou ilegível.`);
    }
    return b64;
  });
}

export async function readHttpError(res: Response, fallback: string): Promise<string> {
  try {
    const txt = (await res.text()).trim();
    if (!txt || txt.startsWith('<')) {
      return `${fallback} (HTTP ${res.status})`;
    }
    return txt.length > 220 ? `${txt.slice(0, 220)}…` : txt;
  } catch {
    return `${fallback} (HTTP ${res.status})`;
  }
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
  if (!res.ok) throw new Error(await readHttpError(res, 'Falha ao enviar imagem'));
  return res.json();
}

export async function fetchVideos(): Promise<VideoMediaItem[]> {
  const res = await fetch('/api/media/videos');
  if (!res.ok) throw new Error(await readHttpError(res, 'Falha ao carregar vídeos'));
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
  if (!res.ok) throw new Error(await readHttpError(res, 'Falha ao enviar vídeo'));
  return res.json();
}

export async function fetchAudios(): Promise<AudioMediaItem[]> {
  const res = await fetch('/api/media/audios');
  if (!res.ok) throw new Error(await readHttpError(res, 'Falha ao carregar áudios'));
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
  if (!res.ok) throw new Error(await readHttpError(res, 'Falha ao enviar áudio'));
  return res.json();
}

export async function uploadWallpaper(
  filename: string,
  contentBase64: string
): Promise<{ url: string; filename: string }> {
  const res = await fetch('/api/media/upload', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ filename, content_base64: contentBase64 }),
  });
  if (!res.ok) throw new Error(await readHttpError(res, 'Falha ao enviar fundo'));
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

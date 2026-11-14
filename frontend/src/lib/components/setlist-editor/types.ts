import type { Setlist, WallpaperItem } from '$lib/types';

export type WallpaperMode = 'cover' | 'contain' | 'center' | 'repeat';

export interface SetlistEditorCallbacks {
  onclose: () => void;
  onsave: (saved: Setlist) => void;
}

export interface SetlistModalProps {
  open: boolean;
  initialSetlist?: Setlist | null;
  onclose: () => void;
  onsave: (saved: Setlist) => void;
}

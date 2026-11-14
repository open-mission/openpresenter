import type { MediaItem } from '$lib/types';

export type ImageSelectorMode = 'single' | 'folder';

export interface ImageSelectorCallbacks {
  onclose: () => void;
  onadditems?: (items: MediaItem[]) => void;
  onselectimage?: (url: string) => void;
}

export interface ImageSelectorProps {
  open: boolean;
  initialMode?: ImageSelectorMode;
  title?: string;
  subtitle?: string;
  allowFolders?: boolean;
  onclose: () => void;
  onadditems?: (items: MediaItem[]) => void;
  onselectimage?: (url: string) => void;
}

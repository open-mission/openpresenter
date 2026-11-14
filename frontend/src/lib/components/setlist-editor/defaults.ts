import type { WallpaperMode } from './types';

export interface WallpaperModeOption {
  id: WallpaperMode;
  label: string;
}

export const WALLPAPER_MODES: readonly WallpaperModeOption[] = [
  { id: 'cover', label: 'Preencher' },
  { id: 'contain', label: 'Ajustar' },
  { id: 'center', label: 'Centralizar' },
  { id: 'repeat', label: 'Repetir' },
] as const;

export const DEFAULT_WALLPAPER = '/api/media/files/wallpapers/dark-ambient.svg';

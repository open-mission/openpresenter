import type { MediaItem } from '$lib/types';

export type QuickItemMode = 'header' | 'announcement';

export interface QuickItemCallbacks {
  onclose: () => void;
  onsave: (item: MediaItem) => void;
}

export interface QuickItemModalProps extends QuickItemCallbacks {
  open: boolean;
  mode: QuickItemMode;
  initialItem?: MediaItem | null;
}

export interface QuickItemModeConfig {
  titleNew: string;
  titleEdit: string;
  description: string;
  titleLabel: string;
  titlePlaceholder: string;
  kindLabel: string;
}

import type { MediaItem } from '$lib/types';

export interface TimerPreset {
  label: string;
  mins: number;
  secs: number;
}

export interface TimerEditorCallbacks {
  onclose: () => void;
  onsave: (item: MediaItem) => void;
  onaddtosetlist?: (item: MediaItem) => void;
}

export interface TimerEditorModalProps extends TimerEditorCallbacks {
  open: boolean;
  initialItem?: MediaItem | null;
}

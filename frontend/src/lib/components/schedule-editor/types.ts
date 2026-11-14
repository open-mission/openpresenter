import type { MediaItem, ScheduleEvent } from '$lib/types';

export interface ScheduleEditorCallbacks {
  close: () => void;
  save: (item: MediaItem) => void;
}

export interface SchedulePresetItem {
  dayOrDate: string;
  time: string;
  title: string;
  details?: string;
}

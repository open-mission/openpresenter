export type MediaKind = 'song' | 'scripture' | 'announcement' | 'media' | 'header' | 'image' | 'text';

export interface Slide {
  id: string;
  label: string;
  content: string;
  align_h?: 'left' | 'center' | 'right' | 'justify';
  align_v?: 'top' | 'center' | 'bottom';
}

export interface MediaItem {
  id: string;
  title: string;
  artist_or_ref?: string;
  kind: MediaKind;
  slides: Slide[];
  align_h?: 'left' | 'center' | 'right' | 'justify';
  align_v?: 'top' | 'center' | 'bottom';
}

export interface MediaFolder {
  id: string;
  name: string;
  path: string;
  kind: string;
  created_at: string;
}

export interface ImageMediaItem {
  id: string;
  name: string;
  path: string;
  folder_id: string;
  folder_name: string;
  url: string;
  is_external: boolean;
  size_bytes: number;
}

export interface SystemPaths {
  data_dir: string;
  db_path: string;
  images_path: string;
  wallpapers_path: string;
  videos_path: string;
  fonts_path: string;
}

export interface Setlist {
  id: string;
  title: string;
  description?: string;
  wallpaper_path?: string;
  wallpaper_mode?: string; // "cover" | "contain" | "center" | "repeat"
  items: MediaItem[];
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface SetlistSummary {
  id: string;
  title: string;
  description?: string;
  wallpaper_path?: string;
  wallpaper_mode?: string;
  item_count: number;
  is_active: boolean;
  updated_at: string;
}

export interface WallpaperItem {
  name: string;
  url: string;
  is_default: boolean;
}

export type DisplayMode = 'live' | 'blackout' | 'cleartext';

export interface PresentationState {
  active_media_id: string | null;
  active_slide_index: number | null;
  display_mode: DisplayMode;
  public_connected: boolean;
  stage_connected: boolean;
  active_setlist_id?: string;
  wallpaper_path?: string;
  wallpaper_mode?: string;
}

export interface OutputConfig {
  kind: 'public' | 'stage';
  enabled: boolean;
  mode: string;
}

export interface OutputsConfig {
  public: OutputConfig;
  stage: OutputConfig;
}

export interface SessionView {
  app_version: string;
  tagline: string;
  stack: string;
  db_path: string;
  media_path?: string;
  db_ready: boolean;
  db_error: string;
  schema_version: number;
  theme_label: string;
  locale: string;
  first_run_at: string;
  last_run_at: string;
  is_first_run: boolean;
}

export interface StatusResponse {
  app_name: string;
  version: string;
  session: SessionView;
  presentation: PresentationState;
  outputs: OutputsConfig;
}

export type WsMessage =
  | { type: 'sync_state'; payload: { presentation: PresentationState; media_items: MediaItem[] } }
  | { type: 'select_slide'; payload: { media_id: string; slide_index: number } }
  | { type: 'set_display_mode'; payload: { mode: DisplayMode } }
  | { type: 'identify'; payload: { client_type: string } }
  | { type: 'state_updated'; payload: { presentation: PresentationState } }
  | { type: 'media_items_updated'; payload: { media_items: MediaItem[] } }
  | { type: 'setlist_updated'; payload: { setlist: Setlist } }
  | { type: 'ping' }
  | { type: 'pong' };

export type SectionType =
  | 'verse'
  | 'pre-chorus'
  | 'chorus'
  | 'bridge'
  | 'intro'
  | 'outro'
  | 'instrumental'
  | 'tag'
  | 'ending'
  | 'other';

export interface SongSection {
  section_type?: SectionType;
  label: string;
  lines: string[];
}

export interface Song {
  id: string;
  title: string;
  artist: string;
  album?: string;
  composer?: string;
  song_key?: string;
  bpm?: number;
  sections: SongSection[];
  lyrics_raw: string;
}

export interface SongSummary {
  id: string;
  title: string;
  artist: string;
  song_key?: string;
  bpm?: number;
  composer?: string;
  album?: string;
  preview_snippet?: string;
}

export interface OnlineLyricHit {
  track_name: string;
  artist_name: string;
  album_name?: string;
  provider?: string;
  provider_id?: string;
}

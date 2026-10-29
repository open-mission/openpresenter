export type MediaKind = 'song' | 'scripture' | 'announcement' | 'media';

export interface Slide {
  id: string;
  label: string;
  content: string;
}

export interface MediaItem {
  id: string;
  title: string;
  artist_or_ref?: string;
  kind: MediaKind;
  slides: Slide[];
}

export type DisplayMode = 'live' | 'blackout' | 'cleartext';

export interface PresentationState {
  active_media_id: string | null;
  active_slide_index: number | null;
  display_mode: DisplayMode;
  public_connected: boolean;
  stage_connected: boolean;
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
  | { type: 'ping' }
  | { type: 'pong' };

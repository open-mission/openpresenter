export type MediaKind = 'song' | 'scripture' | 'announcement' | 'media' | 'header' | 'image' | 'text' | 'schedule';

export interface ScheduleEvent {
  id: string;
  dayOrDate: string;
  time: string;
  title: string;
  details?: string;
}

export interface Slide {
  id: string;
  label: string;
  content: string;
  align_h?: 'left' | 'center' | 'right' | 'justify';
  align_v?: 'top' | 'center' | 'bottom';
  lower_content?: string;
}

export interface MediaItem {
  id: string;
  title: string;
  artist_or_ref?: string;
  kind: MediaKind;
  slides: Slide[];
  align_h?: 'left' | 'center' | 'right' | 'justify';
  align_v?: 'top' | 'center' | 'bottom';
  lower_image_url?: string;
}

export interface MediaSubfolder {
  relative_path: string;
  name: string;
  count: number;
}

export interface MediaFolder {
  id: string;
  name: string;
  path: string;
  kind: string;
  created_at: string;
  subfolders?: MediaSubfolder[];
}

export interface ImageMediaItem {
  id: string;
  name: string;
  path: string;
  folder_id: string;
  folder_name: string;
  subfolder?: string;
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

export interface BibleVersionSummary {
  id: string;
  name: string;
  abbreviation: string;
  filename: string;
  is_default: boolean;
  book_count: number;
  verse_count: number;
  created_at: string;
  updated_at: string;
}

export interface BibleBook {
  id: number;
  book_reference_id: number;
  testament_reference_id: number; // 1 = AT, 2 = NT
  name: string;
  abbreviation: string;
  chapter_count: number;
}

export interface BibleVerse {
  id: number;
  book_id: number;
  chapter: number;
  verse: number;
  text: string;
}

export interface BibleSearchResult {
  book_id: number;
  book_name: string;
  book_abbreviation: string;
  chapter: number;
  verse: number;
  text: string;
}

export interface ParsedReference {
  book_id: number;
  book_name: string;
  book_abbreviation: string;
  chapter: number;
  verse_start?: number;
  verse_end?: number;
  version?: string;
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
  is_video?: boolean;
}

export type DisplayMode = 'live' | 'blackout' | 'cleartext';

export interface TimerState {
  running: boolean;
  duration_seconds: number;
  remaining_seconds: number;
  label?: string;
  stage_message?: string;
  warning_threshold_seconds: number;
  is_overtime?: boolean;
  allow_overtime?: boolean;
}

export interface PresentationState {
  active_media_id: string | null;
  active_slide_index: number | null;
  display_mode: DisplayMode;
  public_connected: boolean;
  stage_connected: boolean;
  active_setlist_id?: string;
  wallpaper_path?: string;
  wallpaper_mode?: string;
  setlist_wallpaper_path?: string;
  setlist_wallpaper_mode?: string;
  lyrics_background_path?: string;
  lyrics_background_mode?: string;
  timer: TimerState;
  stage_message?: string;
  quick_media?: MediaItem;
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

export interface ClientConnection {
  id: string;
  ip: string;
  route: string;
  client_type: string;
  connected_at: string;
  instance_id?: string;
  device_info?: string;
}

export interface ActionLog {
  id: number;
  timestamp: string;
  action: string;
  category: string;
  details: string;
  instance_id?: string;
  client_type?: string;
  device_info?: string;
  ip_address?: string;
}

export type WsMessage =
  | { type: 'sync_state'; payload: { presentation: PresentationState; media_items: MediaItem[] } }
  | { type: 'select_slide'; payload: { media_id: string; slide_index: number } }
  | { type: 'set_display_mode'; payload: { mode: DisplayMode } }
  | { type: 'stop_presentation' }
  | { type: 'set_wallpaper'; payload: { path?: string; mode?: string } }
  | { type: 'start_timer' }
  | { type: 'pause_timer' }
  | { type: 'reset_timer' }
  | { type: 'set_timer'; payload: { duration_seconds: number; label?: string; allow_overtime?: boolean } }
  | { type: 'set_timer_overtime'; payload: { allow_overtime: boolean } }
  | { type: 'quick_present_media'; payload: { item: MediaItem; slide_index?: number } }
  | { type: 'set_stage_message'; payload: { message?: string } }
  | { type: 'identify'; payload: { client_type: string; route?: string; instance_id?: string; device_info?: string } }
  | { type: 'action_logged'; payload: { log: ActionLog } }
  | { type: 'connections_updated'; payload: { connections: ClientConnection[] } }
  | { type: 'state_updated'; payload: { presentation: PresentationState } }
  | { type: 'media_items_updated'; payload: { media_items: MediaItem[] } }
  | { type: 'setlist_updated'; payload: { setlist: Setlist } }
  | { type: 'setting_updated'; payload: { key: string; value: string | null } }
  | { type: 'reload_clients'; payload: { target?: string } }
  | { type: 'ping' }
  | { type: 'pong' };

export type ImageFitMode = 'contain' | 'cover' | 'fill' | 'center';

export interface PublicDisplaySettings {
  fill_space: boolean;
  word_wrap: boolean;
  show_metadata: boolean;
  margin_top: number;
  margin_bottom: number;
  margin_horizontal: number;
  image_fit?: ImageFitMode;
  ignore_margins_for_media?: boolean;
  image_fade_enabled?: boolean;
  image_fade_duration_ms?: number;
  darken_background?: boolean;
  text_box_background?: boolean;
  min_font_size?: number;
  max_font_size?: number;

  // Encaixe e Margens do Plano de Fundo (Wallpaper)
  wallpaper_fit?: ImageFitMode;
  wallpaper_safe_margins?: boolean;

  // Letras de Músicas
  lyrics_align_h?: 'left' | 'center' | 'right' | 'justify';
  lyrics_align_v?: 'top' | 'center' | 'bottom';
  lyrics_min_font_size?: number;
  lyrics_max_font_size?: number;

  // Textos Bíblicos
  scripture_align_h?: 'left' | 'center' | 'right' | 'justify';
  scripture_align_v?: 'top' | 'center' | 'bottom';
  scripture_min_font_size?: number;
  scripture_max_font_size?: number;
  scripture_fill_space?: boolean;
  scripture_show_reference?: boolean;
  scripture_reference_position?: 'footer' | 'below_text' | 'none';
  scripture_reference_font_size?: number;

  // Agenda / Programação na Tela Pública
  schedule_box_background?: boolean;
  schedule_bg_opacity?: number;
  schedule_max_width_percent?: number;
  schedule_padding?: 'compact' | 'normal' | 'spacious';
  schedule_title_min_font_size?: number;
  schedule_title_max_font_size?: number;
  schedule_day_font_size?: number;
  schedule_time_font_size?: number;
  schedule_details_font_size?: number;
  schedule_show_badges?: boolean;
  schedule_show_footer?: boolean;
  schedule_align_h?: 'left' | 'center' | 'right';
  schedule_day_color?: string;
  schedule_time_color?: string;
  schedule_badge_bg?: boolean;
}

export type ImageDisplayMode = 'fullscreen' | 'lower_third' | 'hide';

export interface LowerThirdSettings {
  enabled: boolean;
  theme: 'gradient' | 'bar' | 'floating' | 'minimal';
  use_public_bg: boolean;
  bg_color: string;
  bg_opacity: number;
  text_color: string;
  font_size: number;
  scripture_font_size?: number;
  font_weight: 'normal' | 'semibold' | 'bold' | 'extrabold';
  text_align: 'left' | 'center' | 'right';
  shadow_strength: 'none' | 'subtle' | 'medium' | 'strong';
  margin_bottom: number;
  max_width_percent: number;
  animation: 'fade' | 'slide' | 'zoom' | 'none';
  show_reference: boolean;
  image_display_mode?: ImageDisplayMode;
  image_fit?: 'contain' | 'cover';
  box_bg_type?: 'color' | 'image';
  box_bg_image_url?: string;
  box_border_radius?: number;
  box_edge_fade?: boolean;
  box_edge_fade_amount?: number;
}

export interface NetworkInfoResponse {
  local_ip: string;
  port: number;
  public_url: string;
  stage_url: string;
  remote_url: string;
  lower_third_url: string;
}

export interface ChurchBrandSettings {
  name: string;
  logo_primary?: string;
  logo_dark?: string;
  logo_light?: string;
  logo_full?: string;
  show_logo_on_public: boolean;
  logo_variant: 'primary' | 'dark' | 'light' | 'full';
  logo_position: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';
  logo_opacity: number;
  logo_size: number;
  logo_margin_horizontal?: number;
  logo_margin_vertical?: number;
}

export type StagePreset = 'split-50-50' | 'modern-grid' | 'video-focus' | 'horizontal-stack';

export interface StageDisplaySettings {
  preset: StagePreset;
  stage_layout?: 'stacked-rows' | 'side-column';
  current_text_color: string;
  next_text_color: string;
  current_border_color: string;
  next_border_color: string;
  timer_color: string;
  clock_color: string;
  font_size_scale?: number;
  min_font_size?: number;
  current_slide_percent?: number;
  next_slide_percent?: number;
  footer_time_percent?: number;

  // Letras de Músicas no Palco
  lyrics_min_font_size?: number;

  // Textos Bíblicos no Palco
  scripture_min_font_size?: number;
  scripture_text_color?: string;
  scripture_border_color?: string;

  // Exibição do Relógio (Segundos e Data)
  clock_show_seconds?: boolean;
  clock_show_date?: boolean;

  // Tamanhos de Fonte das Informações de Palco
  clock_font_size?: number;
  timer_font_size?: number;
  alert_font_size?: number;

  // Aviso para o Palco em Tela Cheia Temporária
  alert_fullscreen_flash?: boolean;
  alert_flash_duration_seconds?: number;

  // Agenda / Programação no Palco
  stage_schedule_title_min_font_size?: number;
  stage_schedule_title_max_font_size?: number;
  stage_schedule_day_font_size?: number;
  stage_schedule_time_font_size?: number;
  stage_schedule_details_font_size?: number;
  stage_schedule_next_min_font_size?: number;
  stage_schedule_next_max_font_size?: number;
  stage_schedule_day_color?: string;
  stage_schedule_time_color?: string;
  stage_schedule_badge_bg?: boolean;
}

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
  tags?: string[];
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
  tags?: string[];
}

export interface SongSettings {
  lines_per_slide: number;
}

export interface OnlineLyricHit {
  track_name: string;
  artist_name: string;
  album_name?: string;
  provider?: string;
  provider_id?: string;
}

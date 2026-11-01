import { writable } from 'svelte/store';
import type {
  PublicDisplaySettings,
  ChurchBrandSettings,
  StageDisplaySettings,
  LowerThirdSettings,
  SongSettings,
} from './types';
import type { BackgroundStylesMap } from './backgrounds';

// Valores padrão
export const DEFAULT_PUBLIC_SETTINGS: PublicDisplaySettings = {
  fill_space: true,
  word_wrap: true,
  show_metadata: true,
  margin_top: 32,
  margin_bottom: 32,
  margin_horizontal: 48,
  image_fit: 'contain',
  wallpaper_fit: 'contain',
  wallpaper_safe_margins: false,
  ignore_margins_for_media: true,
  darken_background: false,
  text_box_background: false,
  min_font_size: 24,
  max_font_size: 128,

  // Letras de Músicas
  lyrics_align_h: 'center',
  lyrics_align_v: 'center',
  lyrics_min_font_size: 24,
  lyrics_max_font_size: 128,

  // Textos Bíblicos
  scripture_align_h: 'center',
  scripture_align_v: 'center',
  scripture_min_font_size: 28,
  scripture_max_font_size: 140,
  scripture_fill_space: true,
  scripture_show_reference: true,
  scripture_reference_position: 'footer',
  scripture_reference_font_size: 28,

  // Agenda / Programação na Tela Pública
  schedule_box_background: true,
  schedule_bg_opacity: 65,
  schedule_max_width_percent: 80,
  schedule_padding: 'normal',
  schedule_title_min_font_size: 32,
  schedule_title_max_font_size: 110,
  schedule_day_font_size: 20,
  schedule_time_font_size: 18,
  schedule_details_font_size: 20,
  schedule_show_badges: true,
  schedule_show_footer: true,
  schedule_align_h: 'center',
};

export const DEFAULT_CHURCH_BRAND: ChurchBrandSettings = {
  name: 'Igreja Local',
  logo_primary: '',
  logo_dark: '',
  logo_light: '',
  logo_full: '',
  show_logo_on_public: false,
  logo_variant: 'primary',
  logo_position: 'top-right',
  logo_opacity: 85,
  logo_size: 64,
  logo_margin_horizontal: 24,
  logo_margin_vertical: 24,
};

export const DEFAULT_STAGE_SETTINGS: StageDisplaySettings = {
  preset: 'modern-grid',
  stage_layout: 'stacked-rows',
  current_text_color: '#FFFFFF',
  next_text_color: '#FACC15',
  current_border_color: '#3B82F6',
  next_border_color: '#EAB308',
  timer_color: '#EF4444',
  clock_color: '#E5E7EB',
  font_size_scale: 1.0,
  min_font_size: 24,
  current_slide_percent: 50,
  next_slide_percent: 30,
  footer_time_percent: 20,

  // Letras de Músicas no Palco
  lyrics_min_font_size: 24,

  // Textos Bíblicos no Palco
  scripture_min_font_size: 24,
  scripture_text_color: '#FFFFFF',
  scripture_border_color: '#3B82F6',

  // Tamanhos de Fonte das Informações de Palco
  clock_font_size: 48,
  timer_font_size: 56,
  alert_font_size: 28,

  // Aviso para o Palco em Tela Cheia Temporária
  alert_fullscreen_flash: true,
  alert_flash_duration_seconds: 5,

  // Agenda / Programação no Palco
  stage_schedule_title_min_font_size: 28,
  stage_schedule_title_max_font_size: 180,
  stage_schedule_day_font_size: 18,
  stage_schedule_time_font_size: 16,
  stage_schedule_details_font_size: 18,
  stage_schedule_next_min_font_size: 18,
  stage_schedule_next_max_font_size: 120,
};

export const DEFAULT_LOWER_THIRD_SETTINGS: LowerThirdSettings = {
  enabled: true,
  theme: 'gradient',
  use_public_bg: true,
  bg_color: '#000000',
  bg_opacity: 75,
  text_color: '#ffffff',
  font_size: 32,
  font_weight: 'bold',
  text_align: 'center',
  shadow_strength: 'strong',
  margin_bottom: 36,
  max_width_percent: 85,
  animation: 'fade',
  show_reference: true,
  image_display_mode: 'fullscreen',
  image_fit: 'contain',
};

export const DEFAULT_BACKGROUND_STYLES: BackgroundStylesMap = {
  defaultBackgroundId: undefined,
  customPresets: [],
  overrides: {},
};

export const DEFAULT_SONG_SETTINGS: SongSettings = {
  lines_per_slide: 2,
};

// Funções utilitárias de carregamento inicial do localStorage
function loadLocal<T>(key: string, fallback: T): T {
  if (typeof window === 'undefined') return fallback;
  try {
    const raw = localStorage.getItem(`open_presenter_${key}`);
    if (raw) return { ...fallback, ...JSON.parse(raw) };
  } catch (err) {
    console.warn(`Erro ao ler ${key} do localStorage:`, err);
  }
  return fallback;
}

function saveLocal<T>(key: string, value: T) {
  if (typeof window === 'undefined') return;
  try {
    localStorage.setItem(`open_presenter_${key}`, JSON.stringify(value));
  } catch (err) {
    console.warn(`Erro ao salvar ${key} no localStorage:`, err);
  }
}

// Stores reativas globais
export const publicSettingsStore = writable<PublicDisplaySettings>(
  loadLocal('public_settings', DEFAULT_PUBLIC_SETTINGS)
);

export const churchBrandStore = writable<ChurchBrandSettings>(
  loadLocal('church_brand', DEFAULT_CHURCH_BRAND)
);

export const stageSettingsStore = writable<StageDisplaySettings>(
  loadLocal('stage_settings', DEFAULT_STAGE_SETTINGS)
);

export const backgroundStylesStore = writable<BackgroundStylesMap>(
  loadLocal('background_styles', DEFAULT_BACKGROUND_STYLES)
);

export const lowerThirdSettingsStore = writable<LowerThirdSettings>(
  loadLocal('lower_third_settings', DEFAULT_LOWER_THIRD_SETTINGS)
);

export const songSettingsStore = writable<SongSettings>(
  loadLocal('song_settings', DEFAULT_SONG_SETTINGS)
);

// Sincronização com o Backend
export async function syncDisplaySettingsFromBackend() {
  if (typeof window === 'undefined') return;

  try {
    // 1. Public Settings
    const pubRes = await fetch('/api/settings/public_display');
    if (pubRes.ok) {
      const data = await pubRes.json();
      if (data.value) {
        const parsed = JSON.parse(data.value);
        publicSettingsStore.set({ ...DEFAULT_PUBLIC_SETTINGS, ...parsed });
        saveLocal('public_settings', parsed);
      }
    }

    // 2. Church Brand
    const brandRes = await fetch('/api/settings/church_brand');
    if (brandRes.ok) {
      const data = await brandRes.json();
      if (data.value) {
        const parsed = JSON.parse(data.value);
        churchBrandStore.set({ ...DEFAULT_CHURCH_BRAND, ...parsed });
        saveLocal('church_brand', parsed);
      }
    }

    // 3. Stage Settings
    const stageRes = await fetch('/api/settings/stage_display');
    if (stageRes.ok) {
      const data = await stageRes.json();
      if (data.value) {
        const parsed = JSON.parse(data.value);
        stageSettingsStore.set({ ...DEFAULT_STAGE_SETTINGS, ...parsed });
        saveLocal('stage_settings', parsed);
      }
    }

    // 4. Background Styles & Custom CSS
    const bgRes = await fetch('/api/settings/background_styles_config');
    if (bgRes.ok) {
      const data = await bgRes.json();
      if (data.value) {
        const parsed = JSON.parse(data.value);
        backgroundStylesStore.set({ ...DEFAULT_BACKGROUND_STYLES, ...parsed });
        saveLocal('background_styles', parsed);
      }
    }

    // 5. Lower Third Settings
    const ltRes = await fetch('/api/settings/lower_third_display');
    if (ltRes.ok) {
      const data = await ltRes.json();
      if (data.value) {
        const parsed = JSON.parse(data.value);
        lowerThirdSettingsStore.set({ ...DEFAULT_LOWER_THIRD_SETTINGS, ...parsed });
        saveLocal('lower_third_settings', parsed);
      }
    }

    // 6. Song Settings
    const songRes = await fetch('/api/settings/song_settings');
    if (songRes.ok) {
      const data = await songRes.json();
      if (data.value) {
        const parsed = JSON.parse(data.value);
        songSettingsStore.set({ ...DEFAULT_SONG_SETTINGS, ...parsed });
        saveLocal('song_settings', parsed);
      }
    }
  } catch (err) {
    console.warn('Falha na sincronização inicial de settings:', err);
  }
}

// Funções de salvamento que atualizam Store, localStorage e Backend
export async function savePublicDisplaySettings(settings: PublicDisplaySettings) {
  publicSettingsStore.set(settings);
  saveLocal('public_settings', settings);

  fetch('/api/settings/public_display', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ value: JSON.stringify(settings) }),
  }).catch(() => {});
}

export async function saveChurchBrandSettings(settings: ChurchBrandSettings) {
  churchBrandStore.set(settings);
  saveLocal('church_brand', settings);

  fetch('/api/settings/church_brand', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ value: JSON.stringify(settings) }),
  }).catch(() => {});
}

export async function saveStageDisplaySettings(settings: StageDisplaySettings) {
  stageSettingsStore.set(settings);
  saveLocal('stage_settings', settings);

  fetch('/api/settings/stage_display', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ value: JSON.stringify(settings) }),
  }).catch(() => {});
}

export async function saveBackgroundStyles(styles: BackgroundStylesMap) {
  backgroundStylesStore.set(styles);
  saveLocal('background_styles', styles);

  fetch('/api/settings/background_styles_config', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ value: JSON.stringify(styles) }),
  }).catch(() => {});
}

export async function saveLowerThirdSettings(settings: LowerThirdSettings) {
  lowerThirdSettingsStore.set(settings);
  saveLocal('lower_third_settings', settings);

  fetch('/api/settings/lower_third_display', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ value: JSON.stringify(settings) }),
  }).catch(() => {});
}

export async function saveSongSettings(settings: SongSettings) {
  songSettingsStore.set(settings);
  saveLocal('song_settings', settings);

  fetch('/api/settings/song_settings', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ value: JSON.stringify(settings) }),
  }).catch(() => {});
}

// Aplica atualizações de configuração recebidas via WebSocket em tempo real
export function handleSettingBroadcast(key: string, value: string | null) {
  if (!value) return;
  try {
    const parsed = JSON.parse(value);
    if (key === 'public_display') {
      publicSettingsStore.set({ ...DEFAULT_PUBLIC_SETTINGS, ...parsed });
      saveLocal('public_settings', parsed);
    } else if (key === 'church_brand') {
      churchBrandStore.set({ ...DEFAULT_CHURCH_BRAND, ...parsed });
      saveLocal('church_brand', parsed);
    } else if (key === 'stage_display') {
      stageSettingsStore.set({ ...DEFAULT_STAGE_SETTINGS, ...parsed });
      saveLocal('stage_settings', parsed);
    } else if (key === 'background_styles_config') {
      backgroundStylesStore.set({ ...DEFAULT_BACKGROUND_STYLES, ...parsed });
      saveLocal('background_styles', parsed);
    } else if (key === 'lower_third_display') {
      lowerThirdSettingsStore.set({ ...DEFAULT_LOWER_THIRD_SETTINGS, ...parsed });
      saveLocal('lower_third_settings', parsed);
    } else if (key === 'song_settings') {
      songSettingsStore.set({ ...DEFAULT_SONG_SETTINGS, ...parsed });
      saveLocal('song_settings', parsed);
    }
  } catch (e) {
    console.error('Falha ao aplicar configuração recebida via WebSocket:', e);
  }
}

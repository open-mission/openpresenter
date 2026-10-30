import { writable } from 'svelte/store';
import type {
  PublicDisplaySettings,
  ChurchBrandSettings,
  StageDisplaySettings,
} from './types';

// Valores padrão
export const DEFAULT_PUBLIC_SETTINGS: PublicDisplaySettings = {
  fill_space: true,
  word_wrap: true,
  show_metadata: true,
  margin_top: 32,
  margin_bottom: 32,
  margin_horizontal: 48,
  image_fit: 'contain',
  ignore_margins_for_media: true,
  darken_background: false,
  text_box_background: false,
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
  current_text_color: '#FFFFFF',
  next_text_color: '#FACC15',
  current_border_color: '#3B82F6',
  next_border_color: '#EAB308',
  timer_color: '#EF4444',
  clock_color: '#E5E7EB',
  font_size_scale: 1.0,
  min_font_size: 24,
  current_slide_percent: 60,
  next_slide_percent: 30,
  footer_time_percent: 10,
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
    }
  } catch (e) {
    console.error('Falha ao aplicar configuração recebida via WebSocket:', e);
  }
}

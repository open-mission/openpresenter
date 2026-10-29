import { writable } from 'svelte/store';

export type ThemeMode = 'system' | 'dark' | 'light';

export type BaseColor = 'neutral' | 'stone' | 'zinc' | 'mauve' | 'olive' | 'mist' | 'taupe';
export type NeutralFamily = BaseColor; // retrocompatibilidade

export type AccentTheme =
  | 'neutral'
  | 'amber'
  | 'blue'
  | 'cyan'
  | 'emerald'
  | 'fuchsia'
  | 'green'
  | 'indigo'
  | 'lime'
  | 'orange'
  | 'pink'
  | 'purple'
  | 'rose'
  | 'violet'
  | 'yellow';

export type WallpaperFitMode = 'cover' | 'contain' | 'center' | 'repeat';

export interface BaseColorPreset {
  id: BaseColor;
  label: string;
  description: string;
  previewColor: string;
}

export const BASE_COLOR_PRESETS: BaseColorPreset[] = [
  { id: 'neutral', label: 'Neutral', description: 'Tons neutros puros e equilibrados (Padrão)', previewColor: '#171717' },
  { id: 'stone', label: 'Stone', description: 'Tons quentes e acolhedores', previewColor: '#1c1917' },
  { id: 'zinc', label: 'Zinc', description: 'Tons neutros modernos ligeiramente azulados', previewColor: '#18181b' },
  { id: 'mauve', label: 'Mauve', description: 'Tons sutis com nuances de ardósia e malva', previewColor: '#282338' },
  { id: 'olive', label: 'Olive', description: 'Tons neutros com nuances de oliva terrosa', previewColor: '#222b25' },
  { id: 'mist', label: 'Mist', description: 'Tons acinzentados frios e arejados', previewColor: '#202936' },
  { id: 'taupe', label: 'Taupe', description: 'Tons terrosos clássicos e sóbrios', previewColor: '#2e2b26' },
];

export const NEUTRAL_PRESETS = BASE_COLOR_PRESETS;

export interface AccentThemePreset {
  id: AccentTheme;
  label: string;
  previewColor: string;
  primaryColor: string;
  primaryForeground: string;
  ringColor: string;
}

export const ACCENT_THEME_PRESETS: AccentThemePreset[] = [
  { id: 'neutral', label: 'Neutral', previewColor: '#e5e5e5', primaryColor: '#fafafa', primaryForeground: '#0a0a0a', ringColor: '#a3a3a3' },
  { id: 'amber', label: 'Amber', previewColor: '#f59e0b', primaryColor: '#f59e0b', primaryForeground: '#000000', ringColor: '#f59e0b' },
  { id: 'blue', label: 'Blue', previewColor: '#3b82f6', primaryColor: '#3b82f6', primaryForeground: '#ffffff', ringColor: '#3b82f6' },
  { id: 'cyan', label: 'Cyan', previewColor: '#06b6d4', primaryColor: '#06b6d4', primaryForeground: '#000000', ringColor: '#06b6d4' },
  { id: 'emerald', label: 'Emerald', previewColor: '#10b981', primaryColor: '#10b981', primaryForeground: '#000000', ringColor: '#10b981' },
  { id: 'fuchsia', label: 'Fuchsia', previewColor: '#d946ef', primaryColor: '#d946ef', primaryForeground: '#ffffff', ringColor: '#d946ef' },
  { id: 'green', label: 'Green', previewColor: '#22c55e', primaryColor: '#22c55e', primaryForeground: '#000000', ringColor: '#22c55e' },
  { id: 'indigo', label: 'Indigo', previewColor: '#6366f1', primaryColor: '#6366f1', primaryForeground: '#ffffff', ringColor: '#6366f1' },
  { id: 'lime', label: 'Lime', previewColor: '#84cc16', primaryColor: '#84cc16', primaryForeground: '#000000', ringColor: '#84cc16' },
  { id: 'orange', label: 'Orange', previewColor: '#f97316', primaryColor: '#f97316', primaryForeground: '#ffffff', ringColor: '#f97316' },
  { id: 'pink', label: 'Pink', previewColor: '#ec4899', primaryColor: '#ec4899', primaryForeground: '#ffffff', ringColor: '#ec4899' },
  { id: 'purple', label: 'Purple', previewColor: '#a855f7', primaryColor: '#a855f7', primaryForeground: '#ffffff', ringColor: '#a855f7' },
  { id: 'rose', label: 'Rose', previewColor: '#f43f5e', primaryColor: '#f43f5e', primaryForeground: '#ffffff', ringColor: '#f43f5e' },
  { id: 'violet', label: 'Violet', previewColor: '#8b5cf6', primaryColor: '#8b5cf6', primaryForeground: '#ffffff', ringColor: '#8b5cf6' },
  { id: 'yellow', label: 'Yellow', previewColor: '#eab308', primaryColor: '#eab308', primaryForeground: '#000000', ringColor: '#eab308' },
];

export interface CustomSectionConfig {
  id: string;
  name: string;
  shortcut: string;
  colorName: string;
  bg: string;
  text: string;
  border: string;
  accent: string;
}

export const COLOR_PRESETS = [
  {
    name: 'Esmeralda',
    id: 'emerald',
    bg: 'bg-emerald-950/40',
    text: 'text-emerald-300',
    border: 'border-emerald-600/40',
    accent: 'bg-emerald-500',
  },
  {
    name: 'Azul',
    id: 'blue',
    bg: 'bg-blue-950/40',
    text: 'text-blue-300',
    border: 'border-blue-600/40',
    accent: 'bg-blue-500',
  },
  {
    name: 'Âmbar',
    id: 'amber',
    bg: 'bg-amber-950/40',
    text: 'text-amber-300',
    border: 'border-amber-600/40',
    accent: 'bg-amber-500',
  },
  {
    name: 'Ciano',
    id: 'cyan',
    bg: 'bg-cyan-950/40',
    text: 'text-cyan-300',
    border: 'border-cyan-600/40',
    accent: 'bg-cyan-500',
  },
  {
    name: 'Teal',
    id: 'teal',
    bg: 'bg-teal-950/40',
    text: 'text-teal-300',
    border: 'border-teal-600/40',
    accent: 'bg-teal-500',
  },
  {
    name: 'Roxo',
    id: 'purple',
    bg: 'bg-purple-950/40',
    text: 'text-purple-300',
    border: 'border-purple-600/40',
    accent: 'bg-purple-500',
  },
  {
    name: 'Rosa',
    id: 'rose',
    bg: 'bg-rose-950/40',
    text: 'text-rose-300',
    border: 'border-rose-600/40',
    accent: 'bg-rose-500',
  },
  {
    name: 'Índigo',
    id: 'indigo',
    bg: 'bg-indigo-950/40',
    text: 'text-indigo-300',
    border: 'border-indigo-600/40',
    accent: 'bg-indigo-500',
  },
];

export const DEFAULT_SECTIONS: CustomSectionConfig[] = [
  {
    id: 'verse',
    name: 'Verso',
    shortcut: 'v',
    colorName: 'Azul',
    bg: 'bg-blue-950/40',
    text: 'text-blue-300',
    border: 'border-blue-600/40',
    accent: 'bg-blue-500',
  },
  {
    id: 'chorus',
    name: 'Refrão',
    shortcut: 'r',
    colorName: 'Esmeralda',
    bg: 'bg-emerald-950/40',
    text: 'text-emerald-300',
    border: 'border-emerald-600/40',
    accent: 'bg-emerald-500',
  },
  {
    id: 'pre-chorus',
    name: 'Pré-Refrão',
    shortcut: 'c',
    colorName: 'Teal',
    bg: 'bg-teal-950/40',
    text: 'text-teal-300',
    border: 'border-teal-600/40',
    accent: 'bg-teal-500',
  },
  {
    id: 'bridge',
    name: 'Ponte',
    shortcut: 'p',
    colorName: 'Âmbar',
    bg: 'bg-amber-950/40',
    text: 'text-amber-300',
    border: 'border-amber-600/40',
    accent: 'bg-amber-500',
  },
  {
    id: 'intro',
    name: 'Intro',
    shortcut: 'i',
    colorName: 'Ciano',
    bg: 'bg-cyan-950/40',
    text: 'text-cyan-300',
    border: 'border-cyan-600/40',
    accent: 'bg-cyan-500',
  },
  {
    id: 'outro',
    name: 'Final',
    shortcut: 'f',
    colorName: 'Roxo',
    bg: 'bg-purple-950/40',
    text: 'text-purple-300',
    border: 'border-purple-600/40',
    accent: 'bg-purple-500',
  },
  {
    id: 'instrumental',
    name: 'Instrumental',
    shortcut: 'm',
    colorName: 'Rosa',
    bg: 'bg-rose-950/40',
    text: 'text-rose-300',
    border: 'border-rose-600/40',
    accent: 'bg-rose-500',
  },
  {
    id: 'tag',
    name: 'Tag',
    shortcut: 't',
    colorName: 'Índigo',
    bg: 'bg-indigo-950/40',
    text: 'text-indigo-300',
    border: 'border-indigo-600/40',
    accent: 'bg-indigo-500',
  },
];

const STORAGE_THEME_KEY = 'open_presenter_theme';
const STORAGE_BASE_COLOR_KEY = 'open_presenter_base_color';
const STORAGE_ACCENT_THEME_KEY = 'open_presenter_accent_theme';
const STORAGE_SECTIONS_KEY = 'open_presenter_sections';
const STORAGE_WALLPAPER_FIT_KEY = 'open_presenter_wallpaper_fit';

function loadInitialTheme(): ThemeMode {
  if (typeof window === 'undefined') return 'system';
  const saved = localStorage.getItem(STORAGE_THEME_KEY);
  if (saved === 'dark' || saved === 'light' || saved === 'system') {
    return saved;
  }
  return 'system';
}

function loadInitialBaseColor(): BaseColor {
  if (typeof window === 'undefined') return 'neutral';
  const saved = localStorage.getItem(STORAGE_BASE_COLOR_KEY) || localStorage.getItem('open_presenter_neutral_family');
  if (
    saved === 'neutral' ||
    saved === 'stone' ||
    saved === 'zinc' ||
    saved === 'mauve' ||
    saved === 'olive' ||
    saved === 'mist' ||
    saved === 'taupe'
  ) {
    return saved as BaseColor;
  }
  return 'neutral';
}

function loadInitialAccentTheme(): AccentTheme {
  if (typeof window === 'undefined') return 'neutral';
  const saved = localStorage.getItem(STORAGE_ACCENT_THEME_KEY);
  if (
    saved === 'neutral' ||
    saved === 'amber' ||
    saved === 'blue' ||
    saved === 'cyan' ||
    saved === 'emerald' ||
    saved === 'fuchsia' ||
    saved === 'green' ||
    saved === 'indigo' ||
    saved === 'lime' ||
    saved === 'orange' ||
    saved === 'pink' ||
    saved === 'purple' ||
    saved === 'rose' ||
    saved === 'violet' ||
    saved === 'yellow'
  ) {
    return saved as AccentTheme;
  }
  return 'neutral';
}

function loadInitialWallpaperFit(): WallpaperFitMode {
  if (typeof window === 'undefined') return 'cover';
  const saved = localStorage.getItem(STORAGE_WALLPAPER_FIT_KEY);
  if (saved === 'cover' || saved === 'contain' || saved === 'center' || saved === 'repeat') {
    return saved;
  }
  return 'cover';
}

function loadInitialSections(): CustomSectionConfig[] {
  if (typeof window === 'undefined') return DEFAULT_SECTIONS;
  try {
    const saved = localStorage.getItem(STORAGE_SECTIONS_KEY);
    if (saved) {
      const parsed = JSON.parse(saved);
      if (Array.isArray(parsed) && parsed.length > 0) {
        return parsed;
      }
    }
  } catch {
    // fallback para padrão
  }
  return DEFAULT_SECTIONS;
}

export const themeStore = writable<ThemeMode>(loadInitialTheme());
export const baseColorStore = writable<BaseColor>(loadInitialBaseColor());
export const neutralFamilyStore = baseColorStore; // alias
export const accentThemeStore = writable<AccentTheme>(loadInitialAccentTheme());
export const wallpaperFitStore = writable<WallpaperFitMode>(loadInitialWallpaperFit());
export const sectionsStore = writable<CustomSectionConfig[]>(loadInitialSections());

export function applyBaseColor(color: BaseColor, isDark = true) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  root.setAttribute('data-base-color', color);

  if (color === 'neutral') {
    root.style.setProperty('--background', isDark ? '#0a0a0a' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#141414' : '#fafafa');
    root.style.setProperty('--border', isDark ? '#262626' : '#e5e5e5');
    root.style.setProperty('--muted', isDark ? '#262626' : '#f5f5f5');
    root.style.setProperty('--muted-foreground', isDark ? '#a3a3a3' : '#737373');
    root.style.setProperty('--foreground', isDark ? '#fafafa' : '#0a0a0a');
  } else if (color === 'stone') {
    root.style.setProperty('--background', isDark ? '#0c0a09' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#1c1917' : '#fafaf9');
    root.style.setProperty('--border', isDark ? '#292524' : '#e7e5e4');
    root.style.setProperty('--muted', isDark ? '#292524' : '#f5f5f4');
    root.style.setProperty('--muted-foreground', isDark ? '#a8a29e' : '#78716c');
    root.style.setProperty('--foreground', isDark ? '#f5f5f4' : '#0c0a09');
  } else if (color === 'zinc') {
    root.style.setProperty('--background', isDark ? '#09090b' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#18181b' : '#fafafa');
    root.style.setProperty('--border', isDark ? '#27272a' : '#e4e4e7');
    root.style.setProperty('--muted', isDark ? '#27272a' : '#f4f4f5');
    root.style.setProperty('--muted-foreground', isDark ? '#a1a1aa' : '#71717a');
    root.style.setProperty('--foreground', isDark ? '#f4f4f5' : '#09090b');
  } else if (color === 'mauve') {
    root.style.setProperty('--background', isDark ? '#0d0b12' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#181522' : '#faf9fc');
    root.style.setProperty('--border', isDark ? '#282338' : '#e5e1ed');
    root.style.setProperty('--muted', isDark ? '#282338' : '#f4f2f8');
    root.style.setProperty('--muted-foreground', isDark ? '#9e97af' : '#736d85');
    root.style.setProperty('--foreground', isDark ? '#f5f3fa' : '#0d0b12');
  } else if (color === 'olive') {
    root.style.setProperty('--background', isDark ? '#0b0e0c' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#141a16' : '#f9faf9');
    root.style.setProperty('--border', isDark ? '#222b25' : '#e1e8e3');
    root.style.setProperty('--muted', isDark ? '#222b25' : '#f1f5f2');
    root.style.setProperty('--muted-foreground', isDark ? '#98a69c' : '#6e7d72');
    root.style.setProperty('--foreground', isDark ? '#f3f7f4' : '#0b0e0c');
  } else if (color === 'mist') {
    root.style.setProperty('--background', isDark ? '#0a0d10' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#141920' : '#f8fafc');
    root.style.setProperty('--border', isDark ? '#202936' : '#e2e8f0');
    root.style.setProperty('--muted', isDark ? '#202936' : '#f1f5f9');
    root.style.setProperty('--muted-foreground', isDark ? '#94a1b2' : '#64748b');
    root.style.setProperty('--foreground', isDark ? '#f4f7fa' : '#020617');
  } else if (color === 'taupe') {
    root.style.setProperty('--background', isDark ? '#0e0d0b' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#1b1916' : '#faf9f7');
    root.style.setProperty('--border', isDark ? '#2e2b26' : '#e8e4de');
    root.style.setProperty('--muted', isDark ? '#2e2b26' : '#f5f3ef');
    root.style.setProperty('--muted-foreground', isDark ? '#aba49b' : '#787268');
    root.style.setProperty('--foreground', isDark ? '#f7f6f4' : '#0e0d0b');
  }
}

export function applyAccentTheme(theme: AccentTheme, isDark = true) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  root.setAttribute('data-accent-theme', theme);

  const preset = ACCENT_THEME_PRESETS.find((p) => p.id === theme) || ACCENT_THEME_PRESETS[0];
  if (theme === 'neutral') {
    root.style.setProperty('--primary', isDark ? '#fafafa' : '#171717');
    root.style.setProperty('--primary-foreground', isDark ? '#0a0a0a' : '#fafafa');
    root.style.setProperty('--ring', isDark ? '#a3a3a3' : '#737373');
  } else {
    root.style.setProperty('--primary', preset.primaryColor);
    root.style.setProperty('--primary-foreground', preset.primaryForeground);
    root.style.setProperty('--ring', preset.ringColor);
  }
}

export function setBaseColor(color: BaseColor) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_BASE_COLOR_KEY, color);
  }
  baseColorStore.set(color);
  const isDark =
    document.documentElement.classList.contains('dark') || !document.documentElement.classList.contains('light');
  applyBaseColor(color, isDark);
}

export function setNeutralFamily(family: NeutralFamily) {
  setBaseColor(family);
}

export function applyNeutralFamily(family: NeutralFamily, isDark = true) {
  applyBaseColor(family, isDark);
}

export function setAccentTheme(theme: AccentTheme) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_ACCENT_THEME_KEY, theme);
  }
  accentThemeStore.set(theme);
  const isDark =
    document.documentElement.classList.contains('dark') || !document.documentElement.classList.contains('light');
  applyAccentTheme(theme, isDark);
}

export function setWallpaperFit(fit: WallpaperFitMode) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_WALLPAPER_FIT_KEY, fit);
  }
  wallpaperFitStore.set(fit);
}

export function applyTheme(mode: ThemeMode) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;

  let isDark = true;
  if (mode === 'system') {
    isDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
  } else if (mode === 'light') {
    isDark = false;
  }

  if (isDark) {
    root.classList.add('dark');
    root.setAttribute('data-theme', 'dark');
  } else {
    root.classList.remove('dark');
    root.setAttribute('data-theme', 'light');
  }

  applyBaseColor(loadInitialBaseColor(), isDark);
  applyAccentTheme(loadInitialAccentTheme(), isDark);
}

export function initAppTheme() {
  if (typeof window === 'undefined') return;
  const theme = loadInitialTheme();
  applyTheme(theme);
}

export function setTheme(mode: ThemeMode) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_THEME_KEY, mode);
  }
  themeStore.set(mode);
  applyTheme(mode);
}

export function saveSections(configs: CustomSectionConfig[]) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_SECTIONS_KEY, JSON.stringify(configs));
  }
  sectionsStore.set(configs);
}

export function resetSectionsToDefault() {
  saveSections(DEFAULT_SECTIONS);
}

export function getSectionStyle(
  labelOrType: string,
  configs: CustomSectionConfig[] = DEFAULT_SECTIONS
): { bg: string; text: string; border: string; accent: string } {
  const lower = labelOrType.toLowerCase().trim();

  for (const cfg of configs) {
    const cfgNameLower = cfg.name.toLowerCase();
    const cfgIdLower = cfg.id.toLowerCase();
    if (
      lower.includes(cfgNameLower) ||
      lower.includes(cfgIdLower) ||
      (cfg.id === 'verse' && (lower.includes('verso') || lower.includes('estrofe'))) ||
      (cfg.id === 'chorus' && (lower.includes('refr') || lower.includes('coro'))) ||
      (cfg.id === 'bridge' && lower.includes('ponte')) ||
      (cfg.id === 'intro' && (lower.includes('intro') || lower.includes('início'))) ||
      (cfg.id === 'outro' && (lower.includes('final') || lower.includes('encerramento')))
    ) {
      return {
        bg: cfg.bg,
        text: cfg.text,
        border: cfg.border,
        accent: cfg.accent,
      };
    }
  }

  // Padrão (Verso)
  return {
    bg: 'bg-blue-950/40',
    text: 'text-blue-300',
    border: 'border-blue-600/40',
    accent: 'bg-blue-500',
  };
}

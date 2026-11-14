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

export type WallpaperFitMode = 'contain' | 'cover' | 'fill' | 'center' | 'repeat';
export type SetlistViewMode = 'item' | 'flow';
export type ToastPosition = 'top-left' | 'top-right' | 'bottom-right' | 'bottom-left';

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
    bg: 'bg-emerald-500/15',
    text: 'text-emerald-800 dark:text-emerald-300',
    border: 'border-emerald-600/35 dark:border-emerald-500/40',
    accent: 'bg-emerald-500',
  },
  {
    name: 'Azul',
    id: 'blue',
    bg: 'bg-blue-500/15',
    text: 'text-blue-800 dark:text-blue-300',
    border: 'border-blue-600/35 dark:border-blue-500/40',
    accent: 'bg-blue-500',
  },
  {
    name: 'Âmbar',
    id: 'amber',
    bg: 'bg-amber-500/15',
    text: 'text-amber-800 dark:text-amber-300',
    border: 'border-amber-600/35 dark:border-amber-500/40',
    accent: 'bg-amber-500',
  },
  {
    name: 'Ciano',
    id: 'cyan',
    bg: 'bg-cyan-500/15',
    text: 'text-cyan-800 dark:text-cyan-300',
    border: 'border-cyan-600/35 dark:border-cyan-500/40',
    accent: 'bg-cyan-500',
  },
  {
    name: 'Teal',
    id: 'teal',
    bg: 'bg-teal-500/15',
    text: 'text-teal-800 dark:text-teal-300',
    border: 'border-teal-600/35 dark:border-teal-500/40',
    accent: 'bg-teal-500',
  },
  {
    name: 'Roxo',
    id: 'purple',
    bg: 'bg-purple-500/15',
    text: 'text-purple-800 dark:text-purple-300',
    border: 'border-purple-600/35 dark:border-purple-500/40',
    accent: 'bg-purple-500',
  },
  {
    name: 'Rosa',
    id: 'rose',
    bg: 'bg-rose-500/15',
    text: 'text-rose-800 dark:text-rose-300',
    border: 'border-rose-600/35 dark:border-rose-500/40',
    accent: 'bg-rose-500',
  },
  {
    name: 'Índigo',
    id: 'indigo',
    bg: 'bg-indigo-500/15',
    text: 'text-indigo-800 dark:text-indigo-300',
    border: 'border-indigo-600/35 dark:border-indigo-500/40',
    accent: 'bg-indigo-500',
  },
];

export const DEFAULT_SECTIONS: CustomSectionConfig[] = [
  {
    id: 'verse',
    name: 'Verso',
    shortcut: 'v',
    colorName: 'Azul',
    bg: 'bg-blue-500/15',
    text: 'text-blue-800 dark:text-blue-300',
    border: 'border-blue-600/35 dark:border-blue-500/40',
    accent: 'bg-blue-500',
  },
  {
    id: 'chorus',
    name: 'Refrão',
    shortcut: 'r',
    colorName: 'Esmeralda',
    bg: 'bg-emerald-500/15',
    text: 'text-emerald-800 dark:text-emerald-300',
    border: 'border-emerald-600/35 dark:border-emerald-500/40',
    accent: 'bg-emerald-500',
  },
  {
    id: 'pre-chorus',
    name: 'Pré-Refrão',
    shortcut: 'c',
    colorName: 'Teal',
    bg: 'bg-teal-500/15',
    text: 'text-teal-800 dark:text-teal-300',
    border: 'border-teal-600/35 dark:border-teal-500/40',
    accent: 'bg-teal-500',
  },
  {
    id: 'bridge',
    name: 'Ponte',
    shortcut: 'p',
    colorName: 'Âmbar',
    bg: 'bg-amber-500/15',
    text: 'text-amber-800 dark:text-amber-300',
    border: 'border-amber-600/35 dark:border-amber-500/40',
    accent: 'bg-amber-500',
  },
  {
    id: 'intro',
    name: 'Intro',
    shortcut: 'i',
    colorName: 'Ciano',
    bg: 'bg-cyan-500/15',
    text: 'text-cyan-800 dark:text-cyan-300',
    border: 'border-cyan-600/35 dark:border-cyan-500/40',
    accent: 'bg-cyan-500',
  },
  {
    id: 'outro',
    name: 'Final',
    shortcut: 'f',
    colorName: 'Roxo',
    bg: 'bg-purple-500/15',
    text: 'text-purple-800 dark:text-purple-300',
    border: 'border-purple-600/35 dark:border-purple-500/40',
    accent: 'bg-purple-500',
  },
  {
    id: 'instrumental',
    name: 'Instrumental',
    shortcut: 'm',
    colorName: 'Rosa',
    bg: 'bg-rose-500/15',
    text: 'text-rose-800 dark:text-rose-300',
    border: 'border-rose-600/35 dark:border-rose-500/40',
    accent: 'bg-rose-500',
  },
  {
    id: 'tag',
    name: 'Tag',
    shortcut: 't',
    colorName: 'Índigo',
    bg: 'bg-indigo-500/15',
    text: 'text-indigo-800 dark:text-indigo-300',
    border: 'border-indigo-600/35 dark:border-indigo-500/40',
    accent: 'bg-indigo-500',
  },
];

const STORAGE_THEME_KEY = 'open_presenter_theme';
const STORAGE_BASE_COLOR_KEY = 'open_presenter_base_color';
const STORAGE_ACCENT_THEME_KEY = 'open_presenter_accent_theme';
const STORAGE_SECTIONS_KEY = 'open_presenter_sections';
const STORAGE_WALLPAPER_FIT_KEY = 'open_presenter_wallpaper_fit';
const STORAGE_PREVIEW_LABELS_KEY = 'open_presenter_preview_labels';
const STORAGE_SETLIST_VIEW_MODE_KEY = 'open_presenter_setlist_view_mode';
const STORAGE_TOAST_DND_KEY = 'open_presenter_toast_dnd';
const STORAGE_TOAST_POSITION_KEY = 'open_presenter_toast_position';

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
  if (saved === 'cover' || saved === 'contain' || saved === 'fill' || saved === 'center' || saved === 'repeat') {
    return saved;
  }
  return 'cover';
}

function loadInitialPreviewLabels(): boolean {
  if (typeof window === 'undefined') return true;
  const saved = localStorage.getItem(STORAGE_PREVIEW_LABELS_KEY);
  return saved !== 'false';
}

function loadInitialSetlistViewMode(): SetlistViewMode {
  if (typeof window === 'undefined') return 'item';
  const saved = localStorage.getItem(STORAGE_SETLIST_VIEW_MODE_KEY);
  if (saved === 'flow' || saved === 'item') {
    return saved;
  }
  return 'item';
}

function normalizeSectionThemeClasses(sec: CustomSectionConfig): CustomSectionConfig {
  const matchedPreset = COLOR_PRESETS.find((p) => p.name === sec.colorName || sec.bg?.includes(p.id));
  if (!matchedPreset || (sec.text.includes('dark:') && !sec.bg.includes('-950/'))) {
    return sec;
  }
  return {
    ...sec,
    bg: matchedPreset.bg,
    text: matchedPreset.text,
    border: matchedPreset.border,
    accent: matchedPreset.accent,
  };
}

function loadInitialSections(): CustomSectionConfig[] {
  if (typeof window === 'undefined') return DEFAULT_SECTIONS;
  try {
    const saved = localStorage.getItem(STORAGE_SECTIONS_KEY);
    if (saved) {
      const parsed = JSON.parse(saved);
      if (Array.isArray(parsed) && parsed.length > 0) {
        return parsed.map(normalizeSectionThemeClasses);
      }
    }
  } catch {
    // fallback para padrão
  }
  return DEFAULT_SECTIONS;
}

export function loadInitialToastDnd(): boolean {
  if (typeof window === 'undefined') return false;
  return localStorage.getItem(STORAGE_TOAST_DND_KEY) === 'true';
}

export function loadInitialToastPosition(): ToastPosition {
  if (typeof window === 'undefined') return 'top-right';
  const saved = localStorage.getItem(STORAGE_TOAST_POSITION_KEY) as ToastPosition | null;
  if (saved && ['top-left', 'top-right', 'bottom-right', 'bottom-left'].includes(saved)) {
    return saved;
  }
  return 'top-right';
}

export const themeStore = writable<ThemeMode>(loadInitialTheme());
export const baseColorStore = writable<BaseColor>(loadInitialBaseColor());
export const neutralFamilyStore = baseColorStore; // alias
export const accentThemeStore = writable<AccentTheme>(loadInitialAccentTheme());
export const wallpaperFitStore = writable<WallpaperFitMode>(loadInitialWallpaperFit());
export const previewLabelsStore = writable<boolean>(loadInitialPreviewLabels());
export const setlistViewModeStore = writable<SetlistViewMode>(loadInitialSetlistViewMode());
export const sectionsStore = writable<CustomSectionConfig[]>(loadInitialSections());
export const toastDndStore = writable<boolean>(loadInitialToastDnd());
export const toastPositionStore = writable<ToastPosition>(loadInitialToastPosition());

export function setToastDnd(enabled: boolean) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_TOAST_DND_KEY, String(enabled));
  }
  toastDndStore.set(enabled);
}

export function setToastPosition(position: ToastPosition) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_TOAST_POSITION_KEY, position);
  }
  toastPositionStore.set(position);
}

export function setSetlistViewMode(mode: SetlistViewMode) {
  setlistViewModeStore.set(mode);
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_SETLIST_VIEW_MODE_KEY, mode);
  }
}

const BASE_COLOR_TOKENS: Record<
  BaseColor,
  {
    dark: { bg: string; card: string; border: string; muted: string; mutedFg: string; fg: string };
    light: { bg: string; card: string; border: string; muted: string; mutedFg: string; fg: string };
  }
> = {
  neutral: {
    dark: { bg: '#0a0a0a', card: '#141414', border: '#262626', muted: '#262626', mutedFg: '#a3a3a3', fg: '#fafafa' },
    light: { bg: '#ffffff', card: '#fafafa', border: '#e5e5e5', muted: '#f5f5f5', mutedFg: '#525252', fg: '#0a0a0a' },
  },
  stone: {
    dark: { bg: '#0c0a09', card: '#1c1917', border: '#292524', muted: '#292524', mutedFg: '#a8a29e', fg: '#f5f5f4' },
    light: { bg: '#ffffff', card: '#fafaf9', border: '#e7e5e4', muted: '#f5f5f4', mutedFg: '#57534e', fg: '#0c0a09' },
  },
  zinc: {
    dark: { bg: '#09090b', card: '#18181b', border: '#27272a', muted: '#27272a', mutedFg: '#a1a1aa', fg: '#f4f4f5' },
    light: { bg: '#ffffff', card: '#fafafa', border: '#e4e4e7', muted: '#f4f4f5', mutedFg: '#52525b', fg: '#09090b' },
  },
  mauve: {
    dark: { bg: '#0d0b12', card: '#181522', border: '#282338', muted: '#282338', mutedFg: '#a69fb7', fg: '#f5f3fa' },
    light: { bg: '#ffffff', card: '#faf9fc', border: '#e5e1ed', muted: '#f4f2f8', mutedFg: '#554f68', fg: '#0d0b12' },
  },
  olive: {
    dark: { bg: '#0b0e0c', card: '#141a16', border: '#222b25', muted: '#222b25', mutedFg: '#9fae9f', fg: '#f3f7f4' },
    light: { bg: '#ffffff', card: '#f9faf9', border: '#e1e8e3', muted: '#f1f5f2', mutedFg: '#48594d', fg: '#0b0e0c' },
  },
  mist: {
    dark: { bg: '#0a0d10', card: '#141920', border: '#202936', muted: '#202936', mutedFg: '#9daab9', fg: '#f4f7fa' },
    light: { bg: '#ffffff', card: '#f8fafc', border: '#e2e8f0', muted: '#f1f5f9', mutedFg: '#475569', fg: '#020617' },
  },
  taupe: {
    dark: { bg: '#0e0d0b', card: '#1b1916', border: '#2e2b26', muted: '#2e2b26', mutedFg: '#aba49b', fg: '#f7f6f4' },
    light: { bg: '#ffffff', card: '#faf9f7', border: '#e8e4de', muted: '#f5f3ef', mutedFg: '#57524a', fg: '#0e0d0b' },
  },
};

export function applyBaseColor(color: BaseColor, isDark = true) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  root.setAttribute('data-base-color', color);

  const family = BASE_COLOR_TOKENS[color] ?? BASE_COLOR_TOKENS.neutral;
  const palette = isDark ? family.dark : family.light;

  root.style.setProperty('--background', palette.bg);
  root.style.setProperty('--card', palette.card);
  root.style.setProperty('--card-foreground', palette.fg);
  root.style.setProperty('--popover', isDark ? palette.card : '#ffffff');
  root.style.setProperty('--popover-foreground', palette.fg);
  root.style.setProperty('--secondary', palette.muted);
  root.style.setProperty('--secondary-foreground', palette.fg);
  root.style.setProperty('--muted', palette.muted);
  root.style.setProperty('--muted-foreground', palette.mutedFg);
  root.style.setProperty('--accent', palette.muted);
  root.style.setProperty('--accent-foreground', palette.fg);
  root.style.setProperty('--border', palette.border);
  root.style.setProperty('--input', palette.border);
  root.style.setProperty('--foreground', palette.fg);
  root.style.setProperty('--destructive', isDark ? '#dc2626' : '#b91c1c');
  root.style.setProperty('--destructive-foreground', '#ffffff');
}

const LIGHT_MODE_ACCESSIBLE_PRIMARIES: Partial<Record<AccentTheme, string>> = {
  blue: '#2563eb',
  fuchsia: '#a21caf',
  indigo: '#4f46e5',
  orange: '#c2410c',
  pink: '#be185d',
  purple: '#7e22ce',
  rose: '#be123c',
  violet: '#6d28d9',
};

export function applyAccentTheme(theme: AccentTheme, isDark = true) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  root.setAttribute('data-accent-theme', theme);

  const preset = ACCENT_THEME_PRESETS.find((p) => p.id === theme) || ACCENT_THEME_PRESETS[0];
  if (theme === 'neutral') {
    root.style.setProperty('--primary', isDark ? '#fafafa' : '#171717');
    root.style.setProperty('--primary-foreground', isDark ? '#0a0a0a' : '#fafafa');
    root.style.setProperty('--ring', isDark ? '#a3a3a3' : '#525252');
  } else {
    const primary = isDark ? preset.primaryColor : (LIGHT_MODE_ACCESSIBLE_PRIMARIES[theme] ?? preset.primaryColor);
    root.style.setProperty('--primary', primary);
    root.style.setProperty('--primary-foreground', preset.primaryForeground);
    root.style.setProperty('--ring', primary);
  }
}

export function setBaseColor(color: BaseColor) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_BASE_COLOR_KEY, color);
  }
  baseColorStore.set(color);
  const isDark = document.documentElement.classList.contains('dark');
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
  const isDark = document.documentElement.classList.contains('dark');
  applyAccentTheme(theme, isDark);
}

export function setWallpaperFit(fit: WallpaperFitMode) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_WALLPAPER_FIT_KEY, fit);
  }
  wallpaperFitStore.set(fit);
}

export function setPreviewLabels(show: boolean) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_PREVIEW_LABELS_KEY, String(show));
  }
  previewLabelsStore.set(show);
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
    bg: 'bg-blue-500/15',
    text: 'text-blue-800 dark:text-blue-300',
    border: 'border-blue-600/35 dark:border-blue-500/40',
    accent: 'bg-blue-500',
  };
}

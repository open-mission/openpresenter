import { writable } from 'svelte/store';

export type ThemeMode = 'system' | 'dark' | 'light';

export type NeutralFamily = 'neutral' | 'zinc' | 'slate' | 'stone' | 'gray';

export type WallpaperFitMode = 'cover' | 'contain' | 'center' | 'repeat';

export interface NeutralPreset {
  id: NeutralFamily;
  label: string;
  description: string;
  previewColor: string;
}

export const NEUTRAL_PRESETS: NeutralPreset[] = [
  { id: 'neutral', label: 'Neutro (Padrão)', description: 'Tons neutros puros e elegantes', previewColor: '#171717' },
  { id: 'zinc', label: 'Zinc', description: 'Tons neutros modernos ligeiramente azulados', previewColor: '#18181b' },
  { id: 'slate', label: 'Slate', description: 'Tons frios contemporâneos', previewColor: '#0f172a' },
  { id: 'stone', label: 'Stone', description: 'Tons quentes e acolhedores', previewColor: '#1c1917' },
  { id: 'gray', label: 'Gray', description: 'Cinza clássico balanceado', previewColor: '#111827' },
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
const STORAGE_SECTIONS_KEY = 'open_presenter_sections';
const STORAGE_NEUTRAL_KEY = 'open_presenter_neutral_family';
const STORAGE_WALLPAPER_FIT_KEY = 'open_presenter_wallpaper_fit';

function loadInitialTheme(): ThemeMode {
  if (typeof window === 'undefined') return 'system';
  const saved = localStorage.getItem(STORAGE_THEME_KEY);
  if (saved === 'dark' || saved === 'light' || saved === 'system') {
    return saved;
  }
  return 'system';
}

function loadInitialNeutralFamily(): NeutralFamily {
  if (typeof window === 'undefined') return 'neutral';
  const saved = localStorage.getItem(STORAGE_NEUTRAL_KEY);
  if (saved === 'neutral' || saved === 'zinc' || saved === 'slate' || saved === 'stone' || saved === 'gray') {
    return saved;
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
export const sectionsStore = writable<CustomSectionConfig[]>(loadInitialSections());
export const neutralFamilyStore = writable<NeutralFamily>(loadInitialNeutralFamily());
export const wallpaperFitStore = writable<WallpaperFitMode>(loadInitialWallpaperFit());

export function applyNeutralFamily(family: NeutralFamily, isDark = true) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  root.setAttribute('data-neutral-family', family);

  if (family === 'neutral') {
    root.style.setProperty('--background', isDark ? '#0a0a0a' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#141414' : '#fafafa');
    root.style.setProperty('--border', isDark ? '#262626' : '#e5e5e5');
    root.style.setProperty('--muted', isDark ? '#262626' : '#f5f5f5');
    root.style.setProperty('--muted-foreground', isDark ? '#a3a3a3' : '#737373');
    root.style.setProperty('--foreground', isDark ? '#fafafa' : '#0a0a0a');
  } else if (family === 'zinc') {
    root.style.setProperty('--background', isDark ? '#09090b' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#18181b' : '#fafafa');
    root.style.setProperty('--border', isDark ? '#27272a' : '#e4e4e7');
    root.style.setProperty('--muted', isDark ? '#27272a' : '#f4f4f5');
    root.style.setProperty('--muted-foreground', isDark ? '#a1a1aa' : '#71717a');
    root.style.setProperty('--foreground', isDark ? '#f4f4f5' : '#09090b');
  } else if (family === 'slate') {
    root.style.setProperty('--background', isDark ? '#020617' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#0f172a' : '#f8fafc');
    root.style.setProperty('--border', isDark ? '#1e293b' : '#e2e8f0');
    root.style.setProperty('--muted', isDark ? '#1e293b' : '#f1f5f9');
    root.style.setProperty('--muted-foreground', isDark ? '#94a3b8' : '#64748b');
    root.style.setProperty('--foreground', isDark ? '#f8fafc' : '#020617');
  } else if (family === 'stone') {
    root.style.setProperty('--background', isDark ? '#0c0a09' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#1c1917' : '#fafaf9');
    root.style.setProperty('--border', isDark ? '#292524' : '#e7e5e4');
    root.style.setProperty('--muted', isDark ? '#292524' : '#f5f5f4');
    root.style.setProperty('--muted-foreground', isDark ? '#a8a29e' : '#78716c');
    root.style.setProperty('--foreground', isDark ? '#f5f5f4' : '#0c0a09');
  } else if (family === 'gray') {
    root.style.setProperty('--background', isDark ? '#030712' : '#ffffff');
    root.style.setProperty('--card', isDark ? '#111827' : '#f9fafb');
    root.style.setProperty('--border', isDark ? '#1f2937' : '#e5e7eb');
    root.style.setProperty('--muted', isDark ? '#1f2937' : '#f3f4f6');
    root.style.setProperty('--muted-foreground', isDark ? '#9ca3af' : '#6b7280');
    root.style.setProperty('--foreground', isDark ? '#f9fafb' : '#030712');
  }
}

export function setNeutralFamily(family: NeutralFamily) {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_NEUTRAL_KEY, family);
  }
  neutralFamilyStore.set(family);
  const isDark = document.documentElement.classList.contains('dark') || !document.documentElement.classList.contains('light');
  applyNeutralFamily(family, isDark);
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

  applyNeutralFamily(loadInitialNeutralFamily(), isDark);
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

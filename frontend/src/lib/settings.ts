import { writable } from 'svelte/store';

export type ThemeMode = 'system' | 'dark' | 'light';

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

function loadInitialTheme(): ThemeMode {
  if (typeof window === 'undefined') return 'system';
  const saved = localStorage.getItem(STORAGE_THEME_KEY);
  if (saved === 'dark' || saved === 'light' || saved === 'system') {
    return saved;
  }
  return 'system';
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

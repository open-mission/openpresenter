import {
  Camera,
  Calendar,
  CheckCircle2,
  Clock,
  Mic,
  MonitorPlay,
  Music,
  Radio,
  Smartphone,
  Sparkles,
  Users,
  Video,
  Volume2,
  XCircle,
} from 'lucide-svelte';
import type { RosterStatus } from '$lib/types';

type DeptIcon = typeof Users;

export type TeamsTabId = 'roster' | 'members' | 'departments';

export const TEAMS_TABS: { id: TeamsTabId; label: string; icon: typeof Calendar }[] = [
  { id: 'roster', label: 'Escala do Culto', icon: Calendar },
  { id: 'members', label: 'Membros', icon: Users },
  { id: 'departments', label: 'Departamentos', icon: Sparkles },
];

export type ConfirmTarget =
  | { kind: 'roster'; id: string; label: string }
  | { kind: 'member'; id: string; label: string }
  | { kind: 'department'; id: string; label: string }
  | { kind: 'item'; rosterId: string; itemId: string; label: string };

export function getConfirmCopy(target: ConfirmTarget | null): { title: string; message: string } {
  if (!target) return { title: '', message: '' };
  switch (target.kind) {
    case 'roster':
      return {
        title: 'Excluir escala',
        message: `Deseja realmente excluir a escala "${target.label}"? Os voluntários alocados serão desvinculados.`,
      };
    case 'member':
      return {
        title: 'Excluir voluntário',
        message: `Deseja realmente excluir o voluntário "${target.label}"? Esta ação também o remove das escalas.`,
      };
    case 'department':
      return {
        title: 'Excluir departamento',
        message: `Deseja realmente excluir o departamento "${target.label}"? Os vínculos dos voluntários serão removidos.`,
      };
    case 'item':
      return {
        title: 'Remover da escala',
        message: `Deseja remover ${target.label} desta escala?`,
      };
  }
}


export interface PresetIcon {
  name: string;
  label: string;
  icon: DeptIcon;
}

export const PRESET_COLORS = [
  '#3b82f6', // Azul
  '#10b981', // Esmeralda
  '#ec4899', // Rosa
  '#8b5cf6', // Roxo
  '#f59e0b', // Âmbar
  '#06b6d4', // Ciano
  '#ef4444', // Vermelho
  '#6366f1', // Índigo
] as const;

export const PRESET_ICONS: PresetIcon[] = [
  { name: 'MonitorPlay', label: 'Projeção', icon: MonitorPlay },
  { name: 'Camera', label: 'Foto', icon: Camera },
  { name: 'Smartphone', label: 'Redes', icon: Smartphone },
  { name: 'Video', label: 'Transmissão', icon: Video },
  { name: 'Volume2', label: 'Som', icon: Volume2 },
  { name: 'Users', label: 'Equipe', icon: Users },
  { name: 'Radio', label: 'Rádio', icon: Radio },
  { name: 'Mic', label: 'Vocal', icon: Mic },
  { name: 'Music', label: 'Louvor', icon: Music },
  { name: 'Sparkles', label: 'Especial', icon: Sparkles },
];

/** Aceita tanto o nome canônico do ícone quanto apelidos salvos por versões antigas. */
const ICON_ALIASES: Record<string, string> = {
  monitor: 'MonitorPlay',
  'monitor-play': 'MonitorPlay',
  camera: 'Camera',
  foto: 'Camera',
  smartphone: 'Smartphone',
  mobile: 'Smartphone',
  video: 'Video',
  audio: 'Volume2',
  'audio-lines': 'Volume2',
  volume: 'Volume2',
  users: 'Users',
  equipe: 'Users',
  radio: 'Radio',
  mic: 'Mic',
  music: 'Music',
  musica: 'Music',
  sparkles: 'Sparkles',
};

function findPresetIcon(iconName?: string | null): PresetIcon | undefined {
  if (!iconName) return undefined;
  const key = iconName.trim().toLowerCase();
  const canonical = (ICON_ALIASES[key] ?? iconName).trim().toLowerCase();
  return PRESET_ICONS.find((item) => item.name.toLowerCase() === canonical);
}

export function getDeptIconComponent(iconName?: string | null): DeptIcon {
  return findPresetIcon(iconName)?.icon ?? Users;
}

export function getDeptIconLabel(iconName?: string | null): string {
  return findPresetIcon(iconName)?.label ?? '';
}

/** Escolhe texto claro ou escuro com base na luminância da cor de fundo. */
export function getContrastTextColor(hex?: string | null): string {
  const value = (hex ?? '').trim().replace('#', '');
  const full =
    value.length === 3
      ? value
          .split('')
          .map((char) => char + char)
          .join('')
      : value;
  if (!/^[0-9a-fA-F]{6}$/.test(full)) return '#ffffff';

  const channels = [0, 2, 4].map((offset) => {
    const channel = parseInt(full.slice(offset, offset + 2), 16) / 255;
    return channel <= 0.03928 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4;
  });
  const luminance = 0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2];

  const contrastWithWhite = 1.05 / (luminance + 0.05);
  const contrastWithBlack = (luminance + 0.05) / 0.05;
  return contrastWithWhite >= contrastWithBlack ? '#ffffff' : '#000000';
}

export function getInitials(name?: string | null): string {
  if (!name) return '?';
  const parts = name.trim().split(/\s+/).filter(Boolean);
  if (parts.length === 0) return '?';
  if (parts.length === 1) return parts[0].substring(0, 2).toUpperCase();
  return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
}

/** Ciclo de confirmação usado nos cartões e nos nós do fluxo. */
export function getNextRosterStatus(status?: string | null): RosterStatus {
  if (status === 'confirmed') return 'declined';
  if (status === 'declined') return 'pending';
  return 'confirmed';
}

export interface RosterStatusMeta {
  label: string;
  shortLabel: string;
  icon: DeptIcon;
  chipClass: string;
  iconClass: string;
}

const STATUS_META: Record<RosterStatus, RosterStatusMeta> = {
  confirmed: {
    label: 'Confirmado',
    shortLabel: 'Confirmado',
    icon: CheckCircle2,
    chipClass: 'text-emerald-700 dark:text-emerald-300 bg-emerald-500/10 border-emerald-500/35',
    iconClass: 'text-emerald-700 dark:text-emerald-300',
  },
  pending: {
    label: 'Pendente',
    shortLabel: 'Pendente',
    icon: Clock,
    chipClass: 'text-amber-700 dark:text-amber-300 bg-amber-500/10 border-amber-500/35',
    iconClass: 'text-amber-700 dark:text-amber-300',
  },
  declined: {
    label: 'Não comparece',
    shortLabel: 'Ausente',
    icon: XCircle,
    chipClass: 'text-rose-700 dark:text-rose-300 bg-rose-500/10 border-rose-500/35',
    iconClass: 'text-rose-700 dark:text-rose-300',
  },
};

export function getRosterStatusMeta(status?: string | null): RosterStatusMeta {
  if (status === 'confirmed' || status === 'declined') return STATUS_META[status];
  return STATUS_META.pending;
}

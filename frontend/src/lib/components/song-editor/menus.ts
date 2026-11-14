import { Copy, Plus, Tag, Trash2 } from 'lucide-svelte';
import type { MenuItem } from '../ContextMenu.svelte';
import type { SongEditorMode } from './types';

export function getSectionChoices(mode: SongEditorMode): string[] {
  return mode === 'song'
    ? [
        'Verso 1',
        'Verso 2',
        'Verso 3',
        'Pré-Refrão',
        'Refrão',
        'Ponte',
        'Intro',
        'Final',
        'Instrumental',
      ]
    : ['Boas-vindas', 'Aviso', 'Campanha', 'Informações', 'Encerramento'];
}

export function buildInsertSectionMenu(
  mode: SongEditorMode,
  onInsert: (label: string) => void,
  onNewSection: () => void
): MenuItem[] {
  return [
    { id: 'tb-sec-header', label: 'Inserir Seção no Cursor:', disabled: true },
    { id: 'tb-div-0', divider: true },
    ...getSectionChoices(mode).map((section) => ({
      id: `tb-sec-${section}`,
      label: section,
      icon: Tag,
      onclick: () => onInsert(section),
    })),
    { id: 'tb-div-1', divider: true },
    {
      id: 'tb-sec-new',
      label: '+ Nova Seção...',
      icon: Plus,
      onclick: onNewSection,
    },
  ];
}

export function buildSlideContextMenu(
  mode: SongEditorMode,
  slideLabel: string,
  slideIdx: number,
  handlers: {
    onSetSection: (slideIdx: number, label: string) => void;
    onNewSection: (slideIdx: number) => void;
    onDuplicate: (slideIdx: number) => void;
    onDelete: (slideIdx: number) => void;
  }
): MenuItem[] {
  const sectionItems: MenuItem[] = getSectionChoices(mode).map((section) => ({
    id: `sec-${section}`,
    label: slideLabel.startsWith(section) ? `✓ ${section}` : section,
    icon: Tag,
    onclick: () => handlers.onSetSection(slideIdx, section),
  }));

  return [
    { id: 'slide-header', label: `Slide #${slideIdx + 1} (${slideLabel})`, disabled: true },
    { id: 'div-sections', divider: true },
    { id: 'sections-title', label: 'Mudar Seção:', disabled: true },
    ...sectionItems,
    {
      id: 'sec-new-custom',
      label: '+ Nova Seção...',
      icon: Plus,
      onclick: () => handlers.onNewSection(slideIdx),
    },
    { id: 'div-actions', divider: true },
    {
      id: 'dup-slide',
      label: 'Duplicar Slide',
      icon: Copy,
      onclick: () => handlers.onDuplicate(slideIdx),
    },
    {
      id: 'del-slide',
      label: 'Excluir Slide',
      icon: Trash2,
      destructive: true,
      onclick: () => handlers.onDelete(slideIdx),
    },
  ];
}

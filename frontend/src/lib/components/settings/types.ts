export type SettingsTab =
  | 'public'
  | 'lyrics_bg'
  | 'stage'
  | 'lower_third'
  | 'church'
  | 'bibles'
  | 'songs'
  | 'sections'
  | 'appearance'
  | 'files'
  | 'outputs'
  | 'shortcuts'
  | 'logs'
  | 'about';

export type PublicSectionId = 'lyrics' | 'scripture' | 'schedule' | 'timer_audio' | 'general';
export type StageSectionId = 'lyrics' | 'scripture' | 'schedule' | 'general';

export interface SettingsConfirmRequest {
  title: string;
  description: string;
  confirmText?: string;
  variant?: 'danger' | 'warning' | 'default';
  onConfirm: () => Promise<void> | void;
}

export interface SettingsTabMeta {
  id: SettingsTab;
  label: string;
  group: 'outputs' | 'content' | 'system';
  keywords: string;
}

export interface BackupInspection {
  valid?: boolean;
  error?: string;
  filename: string;
  file_size_bytes: number;
  manifest?: {
    created_at: string;
    app_version: string;
    stats: {
      songs_count: number;
      setlists_count: number;
      team_members_count: number;
      images_count: number;
      videos_count: number;
      audios_count: number;
      wallpapers_count: number;
    };
  };
}

export const SETTINGS_TABS: SettingsTabMeta[] = [
  {
    id: 'public',
    label: 'Tela Pública',
    group: 'outputs',
    keywords: 'projeção letras bíblia agenda timer margens overscan wallpaper encaixe',
  },
  {
    id: 'stage',
    label: 'Monitor de Palco',
    group: 'outputs',
    keywords: 'retorno palco stage relógio timer cores aviso fullscreen layout',
  },
  {
    id: 'lower_third',
    label: 'Lower Third (OBS)',
    group: 'outputs',
    keywords: 'obs transmissão legenda câmera barra gradiente sombra',
  },
  {
    id: 'lyrics_bg',
    label: 'Fundos de Letras',
    group: 'outputs',
    keywords: 'css gradiente papel parede animação cor texto',
  },
  {
    id: 'outputs',
    label: 'Saídas & Telas',
    group: 'outputs',
    keywords: 'url navegador telão monitor palco obs lower third wallpaper',
  },
  {
    id: 'church',
    label: 'Marca da Igreja',
    group: 'content',
    keywords: 'logo logotipo congregação marca sobreposição',
  },
  {
    id: 'bibles',
    label: 'Bíblias',
    group: 'content',
    keywords: 'versões sqlite importar xml biblia livro',
  },
  {
    id: 'songs',
    label: 'Músicas & Louvor',
    group: 'content',
    keywords: 'letras linhas slide harpa hinos txt json',
  },
  {
    id: 'sections',
    label: 'Partes da Música',
    group: 'content',
    keywords: 'verso refrão ponte atalhos etiquetas cores',
  },
  {
    id: 'files',
    label: 'Arquivos & Backup',
    group: 'content',
    keywords: 'pastas mídia banco dados exportar restaurar json finder',
  },
  {
    id: 'appearance',
    label: 'Aparência',
    group: 'system',
    keywords: 'tema claro escuro cor base destaque setlist notificações toast',
  },
  {
    id: 'shortcuts',
    label: 'Atalhos Rápidos',
    group: 'system',
    keywords: 'teclado command palette blackout limpar slide',
  },
  {
    id: 'logs',
    label: 'Logs & Auditoria',
    group: 'system',
    keywords: 'histórico eventos registro auditoria limpar',
  },
  {
    id: 'about',
    label: 'Sobre & Updates',
    group: 'system',
    keywords: 'versão atualização reiniciar servidor github documentação',
  },
];

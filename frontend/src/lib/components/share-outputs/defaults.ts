import type { ShareOutputTab } from './types';

export interface TabConfig {
  id: ShareOutputTab;
  label: string;
  description: string;
}

export const SHARE_OUTPUT_TABS: readonly TabConfig[] = [
  {
    id: 'stage',
    label: 'Palco',
    description:
      'Ideal para músicos, pregadores e equipe no palco visualizarem letras e timer em tempo real.',
  },
  {
    id: 'public',
    label: 'Pública',
    description:
      'Ideal para monitores auxiliares, salas de transmissão ou telões secundários.',
  },
  {
    id: 'remote',
    label: 'Remoto',
    description:
      'Ideal para operadores, pregadores e líderes de louvor controlarem as apresentações pelo celular.',
  },
  {
    id: 'lower',
    label: 'OBS',
    description:
      'Transparência alfa nativa para OBS Studio / vMix. Adicione uma fonte de Navegador (Browser Source) com resolução 1920x1080.',
  },
  {
    id: 'cast',
    label: 'Cast',
    description:
      'Projete telas em TVs e projetores sem precisar de cabos HDMI via Google Cast.',
  },
] as const;

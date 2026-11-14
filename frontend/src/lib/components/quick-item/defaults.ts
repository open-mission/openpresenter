import type { QuickItemMode, QuickItemModeConfig } from './types';

export const QUICK_ITEM_CONFIG: Record<QuickItemMode, QuickItemModeConfig> = {
  header: {
    titleNew: 'Nova Seção na Setlist',
    titleEdit: 'Editar Seção na Setlist',
    description: 'Cria um divisor visual para organizar sua programação',
    titleLabel: 'Nome da Seção',
    titlePlaceholder: 'Ex.: Momento de Louvor, Dízimos e Ofertas, Palavra Pastoral...',
    kindLabel: 'Divisor de Seção',
  },
  announcement: {
    titleNew: 'Novo Texto Livre / Aviso',
    titleEdit: 'Editar Texto Livre / Aviso',
    description: 'Cria slides de texto livre ou comunicados da equipe',
    titleLabel: 'Título do Aviso / Texto',
    titlePlaceholder: 'Ex.: Boas-Vindas aos Visitantes, Avisos da Semana...',
    kindLabel: 'Texto / Aviso',
  },
};

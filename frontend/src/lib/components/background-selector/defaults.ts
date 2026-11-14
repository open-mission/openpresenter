import type { BackgroundCategory } from './types';

export interface CategoryItem {
  id: BackgroundCategory;
  label: string;
  fullName: string;
}

export const CATEGORIES: readonly CategoryItem[] = [
  { id: 'all', label: 'Todos', fullName: 'Todos os Fundos' },
  { id: 'animated', label: 'Animados (CSS)', fullName: 'Gradientes Animados' },
  { id: 'mesh', label: 'Mesh Gradients', fullName: 'Mesh Gradients' },
  { id: 'gradients', label: 'Gradientes Lineares', fullName: 'Gradientes Lineares' },
  { id: 'custom', label: 'Customizados', fullName: 'Fundos Customizados' },
  { id: 'local', label: 'Papéis Locais', fullName: 'Papéis de Parede Locais' },
] as const;

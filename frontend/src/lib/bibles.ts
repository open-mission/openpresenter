import { writable } from 'svelte/store';
import type { BibleVersionSummary } from './types';

export const biblesStore = writable<BibleVersionSummary[]>([]);
export const biblesRevision = writable<number>(0);

export async function refreshBibles(): Promise<BibleVersionSummary[]> {
  try {
    const res = await fetch('/api/bibles');
    if (res.ok) {
      const data: BibleVersionSummary[] = await res.json();
      biblesStore.set(data);
      biblesRevision.update((n) => n + 1);
      return data;
    }
  } catch (err) {
    console.error('Erro ao recarregar lista de bíblias:', err);
  }
  return [];
}

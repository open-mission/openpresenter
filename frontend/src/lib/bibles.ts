import { writable } from 'svelte/store';
import type { BibleVersionSummary } from './types';

export const biblesStore = writable<BibleVersionSummary[]>([]);
export const biblesRevision = writable<number>(0);
export const biblesErrorStore = writable<string | null>(null);

export async function refreshBibles(): Promise<BibleVersionSummary[]> {
  biblesErrorStore.set(null);
  try {
    const res = await fetch('/api/bibles');
    if (res.ok) {
      const data: BibleVersionSummary[] = await res.json();
      biblesStore.set(data);
      biblesRevision.update((n) => n + 1);
      return data;
    }
    biblesErrorStore.set(`Falha ao carregar versões da Bíblia (HTTP ${res.status}).`);
  } catch (err) {
    console.error('Erro ao recarregar lista de bíblias:', err);
    biblesErrorStore.set('Não foi possível conectar ao servidor para carregar as bíblias.');
  }
  return [];
}

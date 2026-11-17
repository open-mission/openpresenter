import { writable, get } from 'svelte/store';
import { readHttpError } from './media';
import type { MediaItem } from './types';

const CUSTOM_ITEMS_KEY = 'custom_media_items';

export const DEFAULT_CUSTOM_ITEMS: MediaItem[] = [
  {
    id: 'default-schedule-weekly',
    title: 'Programação Semanal',
    artist_or_ref: '',
    kind: 'schedule',
    slides: [
      {
        id: 'sched-slide-1',
        label: 'Culto de Oração',
        content: 'Momento de oração com toda a igreja e estudo bíblico',
        metadata: {
          schedule_day: 'Quarta-Feira',
          schedule_time: '19:30',
          schedule_details: 'Momento de oração com toda a igreja e estudo bíblico no templo central',
        },
      },
      {
        id: 'sched-slide-2',
        label: 'Culto da Família',
        content: 'Celebração com louvor, ministração da palavra e comunhão',
        metadata: {
          schedule_day: 'Domingo',
          schedule_time: '18:00',
          schedule_details: 'Celebração com louvor, ministração da palavra e comunhão',
        },
      },
      {
        id: 'sched-slide-3',
        label: 'Escola Bíblica',
        content: 'Estudo aprofundado das escrituras para todas as idades',
        metadata: {
          schedule_day: 'Domingo',
          schedule_time: '09:00',
          schedule_details: 'Estudo aprofundado das escrituras para todas as idades',
        },
      },
    ],
  },
  {
    id: 'default-text-welcome',
    title: 'Boas-Vindas aos Visitantes',
    artist_or_ref: 'Recepção',
    kind: 'text',
    slides: [
      {
        id: 'text-slide-1',
        label: 'Slide 1',
        content: 'Sejam bem-vindos à Casa do Senhor!\nÉ uma imensa alegria adorarmos juntos hoje.',
      },
      {
        id: 'text-slide-2',
        label: 'Slide 2',
        content: 'Após o culto, procure nossa equipe de recepção no hall de entrada para recebermos você com carinho.',
      },
    ],
  },
  {
    id: 'default-announcement-general',
    title: 'Avisos Gerais',
    artist_or_ref: 'Comunicação',
    kind: 'announcement',
    slides: [
      {
        id: 'ann-slide-1',
        label: 'Slide 1',
        content: 'Dízimos e Ofertas: Adore a Deus com fidelidade e generosidade através do PIX ou no gazofilácio.',
      },
      {
        id: 'ann-slide-2',
        label: 'Slide 2',
        content: 'Pequenos Grupos: Reúna-se durante a semana para comunhão nos lares. Procure seu líder!',
      },
    ],
  },
  {
    id: 'default-timer-countdown',
    title: 'Contagem Regressiva — Início do Culto',
    artist_or_ref: '05:00',
    kind: 'timer',
    slides: [
      {
        id: 'timer-slide-1',
        label: 'Contagem Regressiva',
        content: '300',
        lower_content: 'O culto começará em instantes. Prepare o seu coração!',
      },
    ],
  },
];

export const customItemsStore = writable<MediaItem[]>([]);
export const customItemsErrorStore = writable<string | null>(null);

let isInitialized = false;
let inFlightLoad: Promise<MediaItem[]> | null = null;
let hasSeededDefaults = false;

export async function loadCustomItems(force = false): Promise<MediaItem[]> {
  if (!force) {
    if (isInitialized && !get(customItemsErrorStore)) {
      return get(customItemsStore);
    }
    if (inFlightLoad) {
      return inFlightLoad;
    }
  }

  inFlightLoad = (async () => {
    customItemsErrorStore.set(null);
    try {
      const res = await fetch(`/api/settings/${CUSTOM_ITEMS_KEY}`);
      if (!res.ok) {
        const message = await readHttpError(res, 'Falha ao carregar catálogo Outros');
        customItemsErrorStore.set(message);
        if (get(customItemsStore).length === 0) {
          customItemsStore.set(DEFAULT_CUSTOM_ITEMS);
        }
        isInitialized = true;
        return get(customItemsStore);
      }

      const data = await res.json();
      const rawValue: unknown = data?.value;

      if (typeof rawValue === 'string' && rawValue.trim() !== '') {
        let parsed: unknown;
        try {
          parsed = JSON.parse(rawValue);
        } catch (err) {
          console.error('Catálogo Outros com conteúdo inválido no servidor:', err);
          customItemsErrorStore.set(
            'Os itens do catálogo Outros estão salvos em formato inválido. Salve um item novamente para corrigir o catálogo.'
          );
          isInitialized = true;
          return get(customItemsStore);
        }

        if (!Array.isArray(parsed)) {
          customItemsErrorStore.set(
            'Os itens do catálogo Outros estão salvos em formato inesperado. Salve um item novamente para corrigir o catálogo.'
          );
          isInitialized = true;
          return get(customItemsStore);
        }

        // Array vazio é um estado válido: o operador removeu todos os itens de propósito.
        customItemsStore.set(parsed as MediaItem[]);
        isInitialized = true;
        return get(customItemsStore);
      }

      // Backend respondeu 200 sem valor salvo ainda: semeia os exemplos padrão uma única vez por sessão.
      if (!hasSeededDefaults) {
        hasSeededDefaults = true;
        try {
          const seeded = await requestCustomItems(
            '/api/custom-items/import',
            { items: DEFAULT_CUSTOM_ITEMS },
            'Falha ao inicializar catálogo Outros'
          );
          return seeded;
        } catch (err) {
          console.error('Falha ao semear catálogo Outros no servidor:', err);
          const message = err instanceof Error ? err.message : 'Falha ao inicializar catálogo Outros.';
          customItemsErrorStore.set(message);
        }
      }

      if (get(customItemsStore).length === 0) {
        customItemsStore.set(DEFAULT_CUSTOM_ITEMS);
      }
      isInitialized = true;
      return get(customItemsStore);
    } catch (err) {
      console.error('Falha ao buscar itens customizados do backend:', err);
      customItemsErrorStore.set('Não foi possível conectar ao servidor para carregar o catálogo.');
      if (get(customItemsStore).length === 0) {
        customItemsStore.set(DEFAULT_CUSTOM_ITEMS);
      }
      isInitialized = true;
      return get(customItemsStore);
    } finally {
      inFlightLoad = null;
    }
  })();

  return inFlightLoad;
}

async function requestCustomItems(
  endpoint: string,
  body: unknown,
  fallbackMessage: string
): Promise<MediaItem[]> {
  const res = await fetch(endpoint, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!res.ok) {
    throw new Error(await readHttpError(res, fallbackMessage));
  }

  const data: unknown = await res.json();
  if (!Array.isArray(data)) {
    throw new Error('Resposta inválida do servidor ao atualizar o catálogo Outros.');
  }

  const items = data as MediaItem[];
  customItemsStore.set(items);
  customItemsErrorStore.set(null);
  isInitialized = true;
  return items;
}

export async function saveOrUpdateCustomItem(item: MediaItem): Promise<void> {
  try {
    await requestCustomItems('/api/custom-items', { item }, 'Falha ao salvar item no catálogo Outros');
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Falha ao salvar item no catálogo Outros.';
    customItemsErrorStore.set(message);
    console.error('Falha ao salvar item no catálogo Outros:', err);
    throw err;
  }
}

export async function deleteCustomItem(id: string): Promise<void> {
  try {
    const res = await fetch(`/api/custom-items/${encodeURIComponent(id)}`, { method: 'DELETE' });
    if (!res.ok) {
      throw new Error(await readHttpError(res, 'Falha ao excluir item do catálogo Outros'));
    }

    const data: unknown = await res.json();
    if (!Array.isArray(data)) {
      throw new Error('Resposta inválida do servidor ao excluir item do catálogo Outros.');
    }

    customItemsStore.set(data as MediaItem[]);
    customItemsErrorStore.set(null);
    isInitialized = true;
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Falha ao excluir item do catálogo Outros.';
    customItemsErrorStore.set(message);
    console.error('Falha ao excluir item do catálogo Outros:', err);
    throw err;
  }
}

export async function importItemsFromSetlist(setlistItems: MediaItem[]): Promise<number> {
  const supportedKinds = new Set(['schedule', 'text', 'announcement', 'header', 'timer']);
  const candidates = setlistItems.filter((i) => supportedKinds.has(i.kind));
  if (candidates.length === 0) return 0;

  const previousIds = new Set(get(customItemsStore).map((i) => i.id));

  try {
    const updated = await requestCustomItems(
      '/api/custom-items/import',
      { items: candidates },
      'Falha ao importar itens para o catálogo Outros'
    );
    return updated.filter((i) => !previousIds.has(i.id)).length;
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Falha ao importar itens para o catálogo Outros.';
    customItemsErrorStore.set(message);
    console.error('Falha ao importar itens para o catálogo Outros:', err);
    throw err;
  }
}

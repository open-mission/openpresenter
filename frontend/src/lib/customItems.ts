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
        const message = `Falha ao carregar catálogo Outros (HTTP ${res.status}).`;
        customItemsErrorStore.set(message);
        if (get(customItemsStore).length === 0) {
          customItemsStore.set(DEFAULT_CUSTOM_ITEMS);
        }
        return get(customItemsStore);
      }

      const data = await res.json();
      if (data?.value) {
        const parsed: MediaItem[] = JSON.parse(data.value);
        if (Array.isArray(parsed) && parsed.length > 0) {
          customItemsStore.set(parsed);
          isInitialized = true;
          return parsed;
        }
      }

      // Backend respondeu 200 sem itens salvos ainda: inicializa exemplos padrão uma única vez
      customItemsStore.set(DEFAULT_CUSTOM_ITEMS);
      isInitialized = true;
      if (!hasSeededDefaults) {
        hasSeededDefaults = true;
        await persistCustomItemsToBackend(DEFAULT_CUSTOM_ITEMS).catch(() => {});
      }
      return DEFAULT_CUSTOM_ITEMS;
    } catch (err) {
      console.error('Falha ao buscar itens customizados do backend:', err);
      customItemsErrorStore.set('Não foi possível conectar ao servidor para carregar o catálogo.');
      if (get(customItemsStore).length === 0) {
        customItemsStore.set(DEFAULT_CUSTOM_ITEMS);
      }
      return get(customItemsStore);
    } finally {
      inFlightLoad = null;
    }
  })();

  return inFlightLoad;
}

async function persistCustomItemsToBackend(items: MediaItem[]): Promise<void> {
  const res = await fetch(`/api/settings/${CUSTOM_ITEMS_KEY}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ value: JSON.stringify(items) }),
  });
  if (!res.ok) {
    throw new Error(await readHttpError(res, 'Falha ao salvar catálogo Outros'));
  }
}

export async function saveCustomItems(items: MediaItem[]): Promise<void> {
  const previous = get(customItemsStore);
  customItemsStore.set(items);
  customItemsErrorStore.set(null);
  try {
    await persistCustomItemsToBackend(items);
    isInitialized = true;
  } catch (err) {
    customItemsStore.set(previous);
    const message = err instanceof Error ? err.message : 'Falha ao salvar itens customizados no servidor.';
    customItemsErrorStore.set(message);
    console.error('Falha ao salvar itens customizados no backend:', err);
    throw err;
  }
}

export async function saveOrUpdateCustomItem(item: MediaItem): Promise<void> {
  const current = get(customItemsStore);
  const idx = current.findIndex((i) => i.id === item.id);
  let updated: MediaItem[];
  if (idx >= 0) {
    updated = [...current];
    updated[idx] = { ...item };
  } else {
    updated = [{ ...item }, ...current];
  }
  await saveCustomItems(updated);
}

export async function deleteCustomItem(id: string): Promise<void> {
  const current = get(customItemsStore);
  const updated = current.filter((i) => i.id !== id);
  await saveCustomItems(updated);
}

export async function importItemsFromSetlist(setlistItems: MediaItem[]): Promise<number> {
  const supportedKinds = new Set(['schedule', 'text', 'announcement', 'header', 'timer']);
  const candidates = setlistItems.filter((i) => supportedKinds.has(i.kind));
  if (candidates.length === 0) return 0;

  const current = get(customItemsStore);
  const existingIds = new Set(current.map((i) => i.id));
  const newItems = candidates.filter((i) => !existingIds.has(i.id));

  if (newItems.length > 0) {
    const updated = [...newItems, ...current];
    await saveCustomItems(updated);
    return newItems.length;
  }
  return 0;
}

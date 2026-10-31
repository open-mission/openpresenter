<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Search,
    Plus,
    Globe,
    Music,
    ArrowRight,
    CornerDownLeft,
    Check,
    Loader2,
    X,
    ChevronRight,
    FileText,
    BookOpen,
    Play,
  } from 'lucide-svelte';
  import type {
    SongSummary,
    OnlineLyricHit,
    Song,
    ParsedReference,
    BibleSearchResult,
    BibleVerse,
    BibleVersionSummary,
    MediaItem,
  } from '../types';
  import { parseBibleReference } from '../bibleParser';
  import { quickPresentMedia } from '../ws';
  import Badge from './ui/Badge.svelte';

  interface Props {
    open: boolean;
    onclose: () => void;
    oncreatenew: () => void;
    onimportsong: (song: Song) => void;
    onopenbible?: (ref: ParsedReference) => void;
    onpresentscripture?: (ref: ParsedReference) => void;
    onopenbibletab?: () => void;
  }

  let {
    open = false,
    onclose,
    oncreatenew,
    onimportsong,
    onopenbible,
    onpresentscripture,
    onopenbibletab,
  }: Props = $props();

  let query = $state('');
  let activeTab = $state<'local' | 'bible' | 'online'>('local');
  let localResults = $state<SongSummary[]>([]);
  let onlineResults = $state<OnlineLyricHit[]>([]);
  let bibles = $state<BibleVersionSummary[]>([]);
  let selectedBibleId = $state<string>('');
  let bibleResults = $state<BibleSearchResult[]>([]);
  let bibleVerses = $state<BibleVerse[]>([]);
  let biblePassageRef = $state<ParsedReference | null>(null);
  let addedVerseKeys = $state<Record<string, boolean>>({});
  let isLoading = $state(false);
  let selectedIndex = $state(0);
  let addedIds = $state<Record<string, boolean>>({});
  let inputElement = $state<HTMLInputElement | null>(null);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let prevOpen = false;

  let searchAbortController: AbortController | null = null;
  let currentSearchId = 0;
  let lastSearchedTab = $state<'local' | 'bible' | 'online'>('local');
  let lastSearchedQuery = $state<string>('');

  function abortCurrentSearch() {
    if (searchAbortController) {
      searchAbortController.abort();
      searchAbortController = null;
    }
  }

  function switchTab(newTab: 'local' | 'bible' | 'online') {
    activeTab = newTab;
    setTimeout(() => {
      inputElement?.focus();
    }, 0);

    if (debounceTimer) clearTimeout(debounceTimer);
    lastSearchedTab = newTab;
    lastSearchedQuery = query;

    if (newTab === 'local') {
      searchLocal(query);
    } else if (newTab === 'bible') {
      searchBible(query);
    } else {
      searchOnline(query);
    }
  }

  const currentBible = $derived(
    bibles.find((b) => b.id === selectedBibleId) || bibles[0]
  );

  const parsedBibleRef = $derived.by<ParsedReference | null>(() => {
    const q = query.trim();
    if (!q || q.length < 2) return null;
    return parseBibleReference(q);
  });

  async function loadBibles() {
    try {
      const resp = await fetch('/api/bibles');
      if (resp.ok) {
        bibles = await resp.json();
        if (bibles.length > 0 && !selectedBibleId) {
          const def = bibles.find((b) => b.is_default) || bibles[0];
          selectedBibleId = def.id;
        }
      }
    } catch {
      // Ignora erro ao carregar bíblias
    }
  }

  // Gerencia abertura/fechamento do Command Palette
  $effect(() => {
    if (open && !prevOpen) {
      query = '';
      activeTab = 'local';
      lastSearchedTab = 'local';
      lastSearchedQuery = '';
      localResults = [];
      onlineResults = [];
      bibleResults = [];
      bibleVerses = [];
      biblePassageRef = null;
      selectedIndex = 0;
      loadBibles();
      searchLocal('');
      setTimeout(() => {
        inputElement?.focus();
      }, 50);
    } else if (!open && prevOpen) {
      if (debounceTimer) clearTimeout(debounceTimer);
      abortCurrentSearch();
    }
    prevOpen = open;
  });

  // Busca reativa com debounce ao digitar
  $effect(() => {
    if (!open) return;
    const currentQuery = query;
    const currentTab = activeTab;

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      if (currentTab === lastSearchedTab && currentQuery === lastSearchedQuery) {
        return;
      }
      lastSearchedTab = currentTab;
      lastSearchedQuery = currentQuery;

      if (currentTab === 'local') {
        searchLocal(currentQuery);
      } else if (currentTab === 'bible') {
        searchBible(currentQuery);
      } else {
        searchOnline(currentQuery);
      }
    }, 150);

    return () => {
      if (debounceTimer) clearTimeout(debounceTimer);
    };
  });

  async function searchLocal(q: string) {
    const clean = q.trim();
    abortCurrentSearch();
    const controller = new AbortController();
    searchAbortController = controller;
    const searchId = ++currentSearchId;

    isLoading = true;
    try {
      const resp = await fetch(`/api/songs?q=${encodeURIComponent(clean)}`, {
        signal: controller.signal,
      });
      if (searchId !== currentSearchId) return;

      if (resp.ok) {
        localResults = await resp.json();
        // Se a busca local não encontrou músicas e há termo digitado (sem ref bíblica),
        // pré-seleciona a opção 2 ("Buscar na internet") para que Enter dispare a busca online diretamente
        if (localResults.length === 0 && clean.length > 0 && !parsedBibleRef) {
          selectedIndex = 2;
        } else {
          selectedIndex = 0;
        }
      }
    } catch (err: any) {
      if (err?.name === 'AbortError') return;
      if (searchId === currentSearchId) {
        localResults = [];
      }
    } finally {
      if (searchId === currentSearchId) {
        isLoading = false;
      }
    }
  }

  async function searchOnline(q: string) {
    const clean = q.trim();
    if (!clean) {
      onlineResults = [];
      isLoading = false;
      return;
    }

    abortCurrentSearch();
    const controller = new AbortController();
    searchAbortController = controller;
    const searchId = ++currentSearchId;

    isLoading = true;
    try {
      const resp = await fetch(`/api/songs/online/search?q=${encodeURIComponent(clean)}`, {
        signal: controller.signal,
      });
      if (searchId !== currentSearchId) return;

      if (resp.ok) {
        onlineResults = await resp.json();
        selectedIndex = 0;
      } else {
        onlineResults = [];
      }
    } catch (err: any) {
      if (err?.name === 'AbortError') return;
      if (searchId === currentSearchId) {
        onlineResults = [];
      }
    } finally {
      if (searchId === currentSearchId) {
        isLoading = false;
      }
    }
  }

  async function searchBible(q: string) {
    const clean = q.trim();
    if (!clean) {
      bibleResults = [];
      bibleVerses = [];
      biblePassageRef = null;
      return;
    }

    if (!selectedBibleId && bibles.length > 0) {
      selectedBibleId = (bibles.find((b) => b.is_default) || bibles[0]).id;
    }
    if (!selectedBibleId) return;

    abortCurrentSearch();
    const controller = new AbortController();
    searchAbortController = controller;
    const searchId = ++currentSearchId;

    isLoading = true;
    try {
      // 1. Tenta identificar se é uma referência bíblica (ex: "João 3:16", "Sl 23", "Rm 8:28-30")
      const parsed = parseBibleReference(clean);
      if (parsed) {
        biblePassageRef = parsed;
        let targetBibleId = selectedBibleId;
        if (parsed.version) {
          const matched = bibles.find(
            (b) =>
              b.abbreviation.toLowerCase() === parsed.version!.toLowerCase() ||
              b.name.toLowerCase().includes(parsed.version!.toLowerCase())
          );
          if (matched) {
            targetBibleId = matched.id;
            selectedBibleId = matched.id;
          }
        }

        const resp = await fetch(
          `/api/bibles/${targetBibleId}/books/${parsed.book_id}/chapters/${parsed.chapter}/verses`,
          { signal: controller.signal }
        );
        if (searchId !== currentSearchId) return;

        if (resp.ok) {
          const allChapterVerses: BibleVerse[] = await resp.json();
          const start = parsed.verse_start || 1;
          const end = parsed.verse_end || (parsed.verse_start ? parsed.verse_start : allChapterVerses.length);
          bibleVerses = allChapterVerses.filter((v) => v.verse >= start && v.verse <= end);
          bibleResults = [];
          selectedIndex = 0;
        }
        return;
      }

      // 2. Não sendo referência, realiza busca textual por palavras-chave
      biblePassageRef = null;
      bibleVerses = [];
      const resp = await fetch(
        `/api/bibles/${selectedBibleId}/search?q=${encodeURIComponent(clean)}&limit=30`,
        { signal: controller.signal }
      );
      if (searchId !== currentSearchId) return;

      if (resp.ok) {
        bibleResults = await resp.json();
        selectedIndex = 0;
      }
    } catch (err: any) {
      if (err?.name === 'AbortError') return;
      if (searchId === currentSearchId) {
        console.error('Erro na busca bíblica no CommandPalette:', err);
        bibleResults = [];
        bibleVerses = [];
      }
    } finally {
      if (searchId === currentSearchId) {
        isLoading = false;
      }
    }
  }

  async function addToSetlist(songId: string, e?: MouseEvent) {
    if (e) e.stopPropagation();
    try {
      const resp = await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ song_id: songId }),
      });
      if (resp.ok) {
        addedIds = { ...addedIds, [songId]: true };
        setTimeout(() => {
          addedIds = { ...addedIds, [songId]: false };
        }, 2000);
      }
    } catch (err) {
      console.error('Erro ao adicionar na setlist', err);
    }
  }

  function presentBibleResult(result: BibleSearchResult) {
    const abbr = currentBible?.abbreviation || result.book_abbreviation || 'Bíblia';
    const mediaItem: MediaItem = {
      id: `scripture-${selectedBibleId}-${result.book_id}-${result.chapter}-${result.verse}-${Date.now()}`,
      title: `${result.book_name} ${result.chapter}:${result.verse}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: [
        {
          id: `v-${result.verse}`,
          label: `${result.book_name} ${result.chapter}:${result.verse}`,
          content: result.text,
          align_h: 'center',
          align_v: 'center',
        },
      ],
      align_h: 'center',
      align_v: 'center',
    };
    quickPresentMedia(mediaItem, 0);
    onclose();
  }

  async function addBibleResultToSetlist(result: BibleSearchResult, e?: MouseEvent) {
    if (e) e.stopPropagation();
    const abbr = currentBible?.abbreviation || result.book_abbreviation || 'Bíblia';
    const key = `res-${result.book_id}-${result.chapter}-${result.verse}`;
    const mediaItem: MediaItem = {
      id: `scripture-${Date.now()}`,
      title: `${result.book_name} ${result.chapter}:${result.verse}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: [
        {
          id: `v-${result.verse}`,
          label: `${result.book_name} ${result.chapter}:${result.verse}`,
          content: result.text,
          align_h: 'center',
          align_v: 'center',
        },
      ],
      align_h: 'center',
      align_v: 'center',
    };
    try {
      const resp = await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item: mediaItem }),
      });
      if (resp.ok) {
        addedVerseKeys = { ...addedVerseKeys, [key]: true };
        setTimeout(() => {
          addedVerseKeys = { ...addedVerseKeys, [key]: false };
        }, 2000);
      }
    } catch (err) {
      console.error('Erro ao adicionar versículo à setlist:', err);
    }
  }

  function presentBibleVerse(verse: BibleVerse) {
    const abbr = currentBible?.abbreviation || 'Bíblia';
    const bookName = biblePassageRef?.book_name || 'Bíblia';
    const chapter = biblePassageRef?.chapter || verse.chapter;
    const mediaItem: MediaItem = {
      id: `scripture-${selectedBibleId}-${verse.book_id}-${chapter}-${verse.verse}-${Date.now()}`,
      title: `${bookName} ${chapter}:${verse.verse}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: [
        {
          id: `v-${verse.verse}`,
          label: `${bookName} ${chapter}:${verse.verse}`,
          content: verse.text,
          align_h: 'center',
          align_v: 'center',
        },
      ],
      align_h: 'center',
      align_v: 'center',
    };
    quickPresentMedia(mediaItem, 0);
    onclose();
  }

  async function addBibleVerseToSetlist(verse: BibleVerse, e?: MouseEvent) {
    if (e) e.stopPropagation();
    const abbr = currentBible?.abbreviation || 'Bíblia';
    const bookName = biblePassageRef?.book_name || 'Bíblia';
    const chapter = biblePassageRef?.chapter || verse.chapter;
    const key = `verse-${verse.book_id}-${chapter}-${verse.verse}`;
    const mediaItem: MediaItem = {
      id: `scripture-${Date.now()}`,
      title: `${bookName} ${chapter}:${verse.verse}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: [
        {
          id: `v-${verse.verse}`,
          label: `${bookName} ${chapter}:${verse.verse}`,
          content: verse.text,
          align_h: 'center',
          align_v: 'center',
        },
      ],
      align_h: 'center',
      align_v: 'center',
    };
    try {
      const resp = await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item: mediaItem }),
      });
      if (resp.ok) {
        addedVerseKeys = { ...addedVerseKeys, [key]: true };
        setTimeout(() => {
          addedVerseKeys = { ...addedVerseKeys, [key]: false };
        }, 2000);
      }
    } catch (err) {
      console.error('Erro ao adicionar versículo à setlist:', err);
    }
  }

  function presentPassageVerses() {
    if (bibleVerses.length === 0 || !biblePassageRef) return;
    const abbr = currentBible?.abbreviation || 'Bíblia';
    const firstV = bibleVerses[0].verse;
    const lastV = bibleVerses[bibleVerses.length - 1].verse;
    const rangeLabel = firstV === lastV ? `${firstV}` : `${firstV}-${lastV}`;
    const mediaItem: MediaItem = {
      id: `scripture-${Date.now()}`,
      title: `${biblePassageRef.book_name} ${biblePassageRef.chapter}:${rangeLabel}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: bibleVerses.map((v) => ({
        id: `v-${v.verse}`,
        label: `${biblePassageRef!.book_name} ${biblePassageRef!.chapter}:${v.verse}`,
        content: v.text,
        align_h: 'center',
        align_v: 'center',
      })),
      align_h: 'center',
      align_v: 'center',
    };
    quickPresentMedia(mediaItem, 0);
    onclose();
  }

  async function addPassageToSetlist(e?: MouseEvent) {
    if (e) e.stopPropagation();
    if (bibleVerses.length === 0 || !biblePassageRef) return;
    const abbr = currentBible?.abbreviation || 'Bíblia';
    const firstV = bibleVerses[0].verse;
    const lastV = bibleVerses[bibleVerses.length - 1].verse;
    const rangeLabel = firstV === lastV ? `${firstV}` : `${firstV}-${lastV}`;
    const key = `passage-${biblePassageRef.book_id}-${biblePassageRef.chapter}`;
    const mediaItem: MediaItem = {
      id: `scripture-${Date.now()}`,
      title: `${biblePassageRef.book_name} ${biblePassageRef.chapter}:${rangeLabel}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: bibleVerses.map((v) => ({
        id: `v-${v.verse}`,
        label: `${biblePassageRef!.book_name} ${biblePassageRef!.chapter}:${v.verse}`,
        content: v.text,
        align_h: 'center',
        align_v: 'center',
      })),
      align_h: 'center',
      align_v: 'center',
    };
    try {
      const resp = await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item: mediaItem }),
      });
      if (resp.ok) {
        addedVerseKeys = { ...addedVerseKeys, [key]: true };
        setTimeout(() => {
          addedVerseKeys = { ...addedVerseKeys, [key]: false };
        }, 2000);
      }
    } catch (err) {
      console.error('Erro ao adicionar passagem à setlist:', err);
    }
  }

  async function importOnlineHit(hit: OnlineLyricHit) {
    if (!hit.provider || !hit.provider_id) return;
    isLoading = true;
    try {
      const resp = await fetch(
        `/api/songs/online/fetch?provider=${encodeURIComponent(hit.provider)}&id=${encodeURIComponent(hit.provider_id)}`
      );
      if (resp.ok) {
        const song: Song = await resp.json();
        onclose();
        onimportsong(song);
      }
    } catch (err) {
      console.error('Erro ao baixar letra online', err);
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!open) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      onclose();
      return;
    }

    if (e.key === 'Tab') {
      e.preventDefault();
      if (activeTab === 'local') {
        switchTab('bible');
      } else if (activeTab === 'bible') {
        switchTab('online');
      } else {
        switchTab('local');
      }
      return;
    }

    const getMaxIndex = () => {
      if (activeTab === 'local') {
        return localResults.length + 3; // +3 opções rápidas
      }
      if (activeTab === 'bible') {
        if (bibleVerses.length > 1) {
          return bibleVerses.length + 1; // +1 card da passagem completa no topo
        }
        if (bibleVerses.length === 1) {
          return 1;
        }
        return bibleResults.length;
      }
      return onlineResults.length;
    };

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      const max = getMaxIndex();
      if (max > 0) {
        selectedIndex = (selectedIndex + 1) % max;
      }
      return;
    }

    if (e.key === 'ArrowUp') {
      e.preventDefault();
      const max = getMaxIndex();
      if (max > 0) {
        selectedIndex = (selectedIndex - 1 + max) % max;
      }
      return;
    }

    // Atalhos quando uma referência bíblica for digitada em aba local (ex: "joao 3.16" ou "Salmos 91")
    if (activeTab === 'local' && parsedBibleRef) {
      if (e.key === 'Enter' && e.shiftKey) {
        e.preventDefault();
        onpresentscripture?.(parsedBibleRef);
        onclose();
        return;
      }
      if (e.key === 'Enter') {
        e.preventDefault();
        onopenbible?.(parsedBibleRef);
        onclose();
        return;
      }
    }

    // Shift + Enter: Adiciona à setlist imediatamente
    if (e.key === 'Enter' && e.shiftKey) {
      e.preventDefault();
      if (activeTab === 'local' && localResults.length > 0) {
        const targetIdx = selectedIndex >= 3 ? selectedIndex - 3 : 0;
        const song = localResults[targetIdx];
        if (song) {
          addToSetlist(song.id);
        }
      } else if (activeTab === 'bible') {
        if (bibleVerses.length > 1) {
          if (selectedIndex === 0) {
            addPassageToSetlist();
          } else {
            const v = bibleVerses[selectedIndex - 1];
            if (v) addBibleVerseToSetlist(v);
          }
        } else if (bibleVerses.length === 1) {
          addBibleVerseToSetlist(bibleVerses[0]);
        } else if (bibleResults.length > 0) {
          const res = bibleResults[selectedIndex];
          if (res) addBibleResultToSetlist(res);
        }
      }
      return;
    }

    // Enter simples
    if (e.key === 'Enter') {
      e.preventDefault();
      if (activeTab === 'local') {
        if (selectedIndex === 0) {
          // Alternar para aba Bíblia Sagrada no Command Palette
          switchTab('bible');
        } else if (selectedIndex === 1) {
          // Criar nova música manualmente
          onclose();
          oncreatenew();
        } else if (selectedIndex === 2) {
          // Buscar na internet
          switchTab('online');
        } else {
          const songIdx = selectedIndex - 3;
          const song = localResults[songIdx];
          if (song) {
            addToSetlist(song.id);
          }
        }
      } else if (activeTab === 'bible') {
        if (bibleVerses.length > 1) {
          if (selectedIndex === 0) {
            presentPassageVerses();
          } else {
            const v = bibleVerses[selectedIndex - 1];
            if (v) presentBibleVerse(v);
          }
        } else if (bibleVerses.length === 1) {
          presentBibleVerse(bibleVerses[0]);
        } else if (bibleResults.length > 0) {
          const res = bibleResults[selectedIndex];
          if (res) presentBibleResult(res);
        }
      } else {
        const hit = onlineResults[selectedIndex];
        if (hit) {
          importOnlineHit(hit);
        } else if (query.trim()) {
          // Se não há item selecionado, força busca online imediata
          searchOnline(query);
        }
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-start justify-center pt-[10vh] px-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={onclose}
    onkeydown={(e) => {
      if (e.key === 'Escape') onclose();
    }}
  >
    <!-- Card do Command Palette -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="w-full max-w-2xl bg-zinc-950 border border-zinc-800 rounded-xl shadow-2xl overflow-hidden flex flex-col text-zinc-100 animate-in zoom-in-95 duration-150"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Barra de Input Superior -->
      <div class="h-14 px-4 border-b border-zinc-800 flex items-center gap-3 bg-zinc-900/60 shrink-0">
        <Search size={18} class="text-zinc-400 shrink-0" />
        <input
          bind:this={inputElement}
          type="text"
          bind:value={query}
          placeholder={activeTab === 'local'
            ? 'Buscar música por título ou letra, ou selecione uma ação...'
            : activeTab === 'bible'
              ? 'Buscar passagem (ex: João 3:16, Sl 23:1-4) ou palavras-chave (ex: luz do mundo)...'
              : 'Buscar na internet (Holyrics, Vagalume)...'}
          class="flex-1 bg-transparent border-0 text-sm text-white placeholder-zinc-500 focus:outline-none focus:ring-0"
        />

        {#if isLoading}
          <Loader2 size={16} class="text-emerald-400 animate-spin shrink-0" />
        {/if}

        <div class="flex items-center gap-1.5 shrink-0">
          <div class="flex items-center bg-zinc-800/80 p-0.5 rounded-lg border border-zinc-700/60">
            <button
              type="button"
              onclick={() => switchTab('local')}
              class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1 {activeTab === 'local' ? 'bg-zinc-700 text-emerald-400 font-semibold shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
              title="Buscar no catálogo local de músicas"
            >
              <Music size={12} />
              <span class="hidden sm:inline">Músicas</span>
            </button>
            <button
              type="button"
              onclick={() => switchTab('bible')}
              class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1 {activeTab === 'bible' ? 'bg-zinc-700 text-amber-400 font-semibold shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
              title="Buscar na Bíblia Sagrada"
            >
              <BookOpen size={12} />
              <span class="hidden sm:inline">Bíblia</span>
            </button>
            <button
              type="button"
              onclick={() => switchTab('online')}
              class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1 {activeTab === 'online' ? 'bg-sky-600/40 text-sky-300 font-semibold border border-sky-500/40 shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
              title="Buscar na internet (Holyrics e Vagalume)"
            >
              <Globe size={12} />
              <span>Web</span>
            </button>
          </div>

          <button
            type="button"
            onclick={onclose}
            class="w-7 h-7 rounded text-zinc-400 hover:text-white hover:bg-zinc-800 flex items-center justify-center transition-colors cursor-pointer"
            aria-label="Fechar"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      <!-- Abas de Navegação -->
      <div class="px-4 py-1.5 bg-zinc-900/40 border-b border-zinc-800/60 flex items-center justify-between text-xs">
        <div class="flex items-center gap-1.5">
          <button
            type="button"
            onclick={() => switchTab('local')}
            class="px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 {activeTab === 'local' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
          >
            <Music size={13} />
            <span>Músicas ({localResults.length})</span>
          </button>

          <button
            type="button"
            onclick={() => switchTab('bible')}
            class="px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 {activeTab === 'bible' ? 'bg-zinc-800 text-amber-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
          >
            <BookOpen size={13} />
            <span>Bíblia Sagrada {bibleVerses.length > 0 ? `(${bibleVerses.length})` : bibleResults.length > 0 ? `(${bibleResults.length})` : ''}</span>
          </button>

          <button
            type="button"
            onclick={() => switchTab('online')}
            class="px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 {activeTab === 'online' ? 'bg-zinc-800 text-sky-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
          >
            <Globe size={13} />
            <span>Online {onlineResults.length > 0 ? `(${onlineResults.length})` : ''}</span>
          </button>
        </div>

        <div class="flex items-center gap-2">
          {#if activeTab === 'bible' && bibles.length > 0}
            <select
              bind:value={selectedBibleId}
              onchange={() => searchBible(query)}
              class="bg-zinc-800 text-zinc-300 text-[11px] rounded px-2 py-0.5 border border-zinc-700/80 focus:outline-none cursor-pointer"
              title="Versão da Bíblia"
            >
              {#each bibles as b (b.id)}
                <option value={b.id}>{b.abbreviation} - {b.name}</option>
              {/each}
            </select>
          {/if}

          <span class="text-[10px] text-zinc-500 hidden sm:inline">
            <kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Tab</kbd> Alternar
          </span>
        </div>
      </div>

      <!-- Lista de Resultados e Ações -->
      <div class="max-h-[60vh] overflow-y-auto p-2 space-y-1">
        {#if parsedBibleRef}
          <!-- Card de Referência Bíblica Detectada pelo Parser -->
          <div class="p-3 mb-2 rounded-xl border border-amber-500/40 bg-amber-500/10 text-white flex items-center justify-between gap-3 animate-in fade-in">
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-8 h-8 rounded-lg bg-amber-500/20 text-amber-400 flex items-center justify-center shrink-0">
                <BookOpen size={16} />
              </div>
              <div class="min-w-0">
                <div class="text-[11px] font-bold text-amber-300 flex items-center gap-2">
                  <span>Passagem Bíblica</span>
                  <Badge variant="outline" class="text-[9px] border-amber-500/50 text-amber-300 font-mono py-0 px-1 uppercase">
                    {parsedBibleRef.book_abbreviation}
                  </Badge>
                </div>
                <div class="text-sm font-extrabold text-white truncate">
                  {parsedBibleRef.book_name} {parsedBibleRef.chapter}:{parsedBibleRef.verse_start || 1}{parsedBibleRef.verse_end && parsedBibleRef.verse_end !== parsedBibleRef.verse_start ? `-${parsedBibleRef.verse_end}` : ''}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-1.5 shrink-0">
              <button
                type="button"
                onclick={() => {
                  onopenbible?.(parsedBibleRef!);
                  onclose();
                }}
                class="px-2.5 py-1.5 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-xs font-semibold text-zinc-200 flex items-center gap-1.5 transition-colors cursor-pointer"
                title="Abrir o texto bíblico (Enter)"
              >
                <span>Abrir Texto</span>
                <kbd class="px-1 py-0.2 rounded bg-black/50 text-[10px] font-mono text-zinc-400">↵</kbd>
              </button>

              <button
                type="button"
                onclick={() => {
                  onpresentscripture?.(parsedBibleRef!);
                  onclose();
                }}
                class="px-2.5 py-1.5 rounded-lg bg-amber-500 hover:bg-amber-600 text-xs font-bold text-black flex items-center gap-1.5 transition-colors cursor-pointer shadow-md"
                title="Projetar na tela imediatamente (Shift+Enter)"
              >
                <span>Projetar Agora</span>
                <kbd class="px-1 py-0.2 rounded bg-black/20 text-[10px] font-mono">⇧↵</kbd>
              </button>
            </div>
          </div>
        {/if}

        {#if activeTab === 'local'}
          <!-- Ações Rápidas (Sempre no Topo da aba Local) -->
          <div class="space-y-0.5 mb-2 pb-2 border-b border-zinc-800/60">
            <!-- Ação 0: Abrir Bíblia Sagrada -->
            <button
              type="button"
              onclick={() => {
                onclose();
                onopenbibletab?.();
              }}
              class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {selectedIndex === 0 ? 'bg-zinc-800/90 text-white' : 'hover:bg-zinc-900/80 text-zinc-300'}"
            >
              <div class="flex items-center gap-2.5">
                <div class="w-6 h-6 rounded bg-amber-500/20 text-amber-400 flex items-center justify-center shrink-0">
                  <BookOpen size={14} />
                </div>
                <div>
                  <div class="text-xs font-semibold text-white">Abrir Bíblia Sagrada</div>
                  <div class="text-[10px] text-zinc-400">Navegar por livros, capítulos e versículos bíblicos</div>
                </div>
              </div>
              <div class="flex items-center gap-1">
                <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700/80 text-[10px] font-mono text-zinc-400">⌘B</kbd>
                <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700/80 text-[10px] font-mono text-zinc-400">F4</kbd>
              </div>
            </button>

            <!-- Ação 1: Criar Música Manualmente -->
            <button
              type="button"
              onclick={() => {
                onclose();
                oncreatenew();
              }}
              class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {selectedIndex === 1 ? 'bg-zinc-800/90 text-white' : 'hover:bg-zinc-900/80 text-zinc-300'}"
            >
              <div class="flex items-center gap-2.5">
                <div class="w-6 h-6 rounded bg-emerald-500/20 text-emerald-400 flex items-center justify-center shrink-0">
                  <Plus size={14} />
                </div>
                <div>
                  <div class="text-xs font-semibold text-white">Criar nova música manualmente</div>
                  <div class="text-[10px] text-zinc-400">Abre o editor completo em tela com prévia de slides</div>
                </div>
              </div>
              <Badge variant="outline" class="text-[10px] font-mono text-zinc-400">Novo</Badge>
            </button>

            <!-- Ação 2: Buscar na Internet -->
            <button
              type="button"
              onclick={() => switchTab('online')}
              class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {selectedIndex === 2 ? 'bg-zinc-800/90 text-white' : 'hover:bg-zinc-900/80 text-zinc-300'}"
            >
              <div class="flex items-center gap-2.5">
                <div class="w-6 h-6 rounded bg-sky-500/20 text-sky-400 flex items-center justify-center shrink-0">
                  <Globe size={14} />
                </div>
                <div>
                  <div class="text-xs font-semibold text-white">
                    Buscar {query ? `"${query}"` : 'música'} na internet
                  </div>
                  <div class="text-[10px] text-zinc-400">Consulta Holyrics e Vagalume com download automático</div>
                </div>
              </div>
              <ChevronRight size={14} class="text-zinc-500" />
            </button>
          </div>

          <!-- Resultados do Banco SQLite (FTS5) -->
          {#if localResults.length > 0}
            <div class="px-2 py-1 text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
              Músicas no Banco ({localResults.length})
            </div>

            {#each localResults as song, idx (song.id)}
              {@const isSelected = selectedIndex === idx + 3}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                role="button"
                tabindex="0"
                onclick={() => addToSetlist(song.id)}
                class="w-full px-3 py-2.5 rounded-lg flex items-center justify-between transition-colors cursor-pointer text-left {isSelected ? 'bg-zinc-800/90 ring-1 ring-emerald-500/50' : 'hover:bg-zinc-900/80'}"
              >
                <div class="flex items-start gap-2.5 min-w-0 pr-2">
                  <div class="w-6 h-6 rounded bg-zinc-800 text-zinc-400 flex items-center justify-center shrink-0 mt-0.5">
                    <Music size={13} class="text-emerald-400" />
                  </div>
                  <div class="min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="text-xs font-medium text-white truncate">{song.title}</span>
                      {#if song.song_key}
                        <Badge variant="outline" class="text-[9px] py-0 px-1 border-zinc-700 text-zinc-300 font-mono">
                          {song.song_key}
                        </Badge>
                      {/if}
                      {#if song.bpm}
                        <Badge variant="outline" class="text-[9px] py-0 px-1 border-zinc-700 text-zinc-400 font-mono">
                          {song.bpm} BPM
                        </Badge>
                      {/if}
                    </div>

                    <div class="text-[11px] text-zinc-400 truncate">
                      {song.artist || 'Sem artista'}
                    </div>

                    {#if song.preview_snippet}
                      <div class="text-[10px] text-zinc-500 line-clamp-1 mt-0.5 font-mono italic">
                        {@html song.preview_snippet}
                      </div>
                    {/if}
                  </div>
                </div>

                <!-- Botão Adicionar à Setlist -->
                <button
                  type="button"
                  onclick={(e) => addToSetlist(song.id, e)}
                  class="px-2.5 py-1 rounded bg-muted hover:bg-emerald-600 text-foreground hover:text-white border border-border text-xs font-medium transition-colors shrink-0 flex items-center gap-1.5 cursor-pointer"
                >
                  {#if addedIds[song.id]}
                    <Check size={12} class="text-emerald-400" />
                    <span>Adicionado</span>
                  {:else}
                    <Plus size={12} />
                    <span>Add na Setlist</span>
                  {/if}
                </button>
              </div>
            {/each}
          {:else if query}
            <div class="p-6 text-center space-y-2">
              <p class="text-xs text-zinc-400">Nenhuma música encontrada no catálogo local para "{query}".</p>
              <button
                type="button"
                onclick={() => {
                  activeTab = 'online';
                  searchOnline(query);
                }}
                class="px-3 py-1.5 rounded-lg bg-emerald-600/20 border border-emerald-500/40 text-emerald-300 hover:bg-emerald-600/30 text-xs font-medium inline-flex items-center gap-1.5 transition-colors cursor-pointer"
              >
                <Globe size={13} />
                Buscar "{query}" na internet
              </button>
            </div>
          {/if}
        {:else if activeTab === 'bible'}
          <!-- ABA BÍBLIA SAGRADA -->
          {#if bibleVerses.length > 0}
            <!-- Passagem por Referência (ex: João 3:16 ou Sl 23:1-4) -->
            {#if bibleVerses.length > 1}
              <!-- Card de Passagem Completa -->
              {@const isPassageSelected = selectedIndex === 0}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                role="button"
                tabindex="0"
                onclick={() => presentPassageVerses()}
                class="w-full p-3 mb-2 rounded-xl border flex items-center justify-between gap-3 transition-colors cursor-pointer text-left {isPassageSelected ? 'bg-amber-500/20 border-amber-500 ring-1 ring-amber-500/50 text-white' : 'bg-amber-500/10 border-amber-500/40 hover:bg-amber-500/15 text-white'}"
              >
                <div class="flex items-center gap-3 min-w-0">
                  <div class="w-8 h-8 rounded-lg bg-amber-500/20 text-amber-400 flex items-center justify-center shrink-0">
                    <BookOpen size={16} />
                  </div>
                  <div class="min-w-0">
                    <div class="text-[11px] font-bold text-amber-300 flex items-center gap-1.5">
                      <span>Passagem Completa ({bibleVerses.length} versículos)</span>
                      <Badge variant="outline" class="text-[9px] border-amber-500/50 text-amber-300 font-mono py-0 px-1 uppercase">
                        {currentBible?.abbreviation || 'Bíblia'}
                      </Badge>
                    </div>
                    <div class="text-sm font-extrabold text-white truncate">
                      {biblePassageRef?.book_name} {biblePassageRef?.chapter}:{bibleVerses[0].verse}-{bibleVerses[bibleVerses.length - 1].verse}
                    </div>
                  </div>
                </div>

                <div class="flex items-center gap-1.5 shrink-0">
                  <button
                    type="button"
                    onclick={(e) => addPassageToSetlist(e)}
                    class="px-2.5 py-1.5 rounded-lg bg-muted hover:bg-amber-600 text-foreground hover:text-white border border-border text-xs font-semibold flex items-center gap-1.5 transition-colors cursor-pointer"
                    title="Adicionar todos os versículos à Setlist (Shift+Enter)"
                  >
                    {#if addedVerseKeys[`passage-${biblePassageRef?.book_id}-${biblePassageRef?.chapter}`]}
                      <Check size={13} class="text-emerald-400" />
                      <span>Adicionado</span>
                    {:else}
                      <Plus size={13} />
                      <span>Add na Setlist</span>
                    {/if}
                  </button>

                  <button
                    type="button"
                    onclick={() => presentPassageVerses()}
                    class="px-2.5 py-1.5 rounded-lg bg-amber-500 hover:bg-amber-400 text-xs font-bold text-black flex items-center gap-1.5 transition-colors cursor-pointer shadow-md"
                    title="Projetar todos os versículos ao vivo (Enter)"
                  >
                    <Play size={13} class="fill-current" />
                    <span>Projetar Passagem</span>
                  </button>
                </div>
              </div>
            {/if}

            <div class="px-2 py-1 text-[10px] font-semibold uppercase tracking-wider text-zinc-500 flex items-center justify-between">
              <span>Versículos ({bibleVerses.length}) - {currentBible?.name} ({currentBible?.abbreviation})</span>
            </div>

            {#each bibleVerses as verse, idx (verse.id || verse.verse)}
              {@const isSelected = selectedIndex === (bibleVerses.length > 1 ? idx + 1 : idx)}
              {@const verseKey = `verse-${verse.book_id}-${biblePassageRef?.chapter || verse.chapter}-${verse.verse}`}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                role="button"
                tabindex="0"
                onclick={() => presentBibleVerse(verse)}
                class="w-full px-3 py-2.5 rounded-lg flex items-center justify-between transition-colors cursor-pointer text-left {isSelected ? 'bg-zinc-800/90 ring-1 ring-amber-500/50' : 'hover:bg-zinc-900/80'}"
              >
                <div class="flex items-start gap-2.5 min-w-0 pr-3">
                  <div class="w-6 h-6 rounded bg-amber-500/10 text-amber-400 flex items-center justify-center shrink-0 mt-0.5 text-xs font-mono font-bold">
                    {verse.verse}
                  </div>
                  <div class="min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="text-xs font-bold text-white truncate">
                        {biblePassageRef?.book_name} {biblePassageRef?.chapter || verse.chapter}:{verse.verse}
                      </span>
                      <Badge variant="outline" class="text-[9px] py-0 px-1 border-amber-500/40 text-amber-300 font-mono">
                        {currentBible?.abbreviation || 'Bíblia'}
                      </Badge>
                    </div>
                    <div class="text-xs text-zinc-300 line-clamp-2 mt-0.5 leading-snug">
                      {verse.text}
                    </div>
                  </div>
                </div>

                <div class="flex items-center gap-1.5 shrink-0">
                  <button
                    type="button"
                    onclick={(e) => addBibleVerseToSetlist(verse, e)}
                    class="px-2.5 py-1 rounded bg-muted hover:bg-amber-600 text-foreground hover:text-white border border-border text-xs font-medium transition-colors flex items-center gap-1.5 cursor-pointer"
                    title="Adicionar versículo à Setlist"
                  >
                    {#if addedVerseKeys[verseKey]}
                      <Check size={12} class="text-emerald-400" />
                      <span>Adicionado</span>
                    {:else}
                      <Plus size={12} />
                      <span>Add</span>
                    {/if}
                  </button>

                  <button
                    type="button"
                    onclick={() => presentBibleVerse(verse)}
                    class="px-2.5 py-1 rounded bg-amber-500 hover:bg-amber-400 text-black text-xs font-bold transition-colors flex items-center gap-1.5 cursor-pointer"
                    title="Projetar este versículo ao vivo"
                  >
                    <Play size={12} class="fill-current" />
                    <span>Ao Vivo</span>
                  </button>
                </div>
              </div>
            {/each}
          {:else if bibleResults.length > 0}
            <!-- Resultados Textuais da Busca por Palavras -->
            <div class="px-2 py-1 text-[10px] font-semibold uppercase tracking-wider text-zinc-500 flex items-center justify-between">
              <span>Versículos Encontrados ({bibleResults.length}) - Versão {currentBible?.abbreviation}</span>
            </div>

            {#each bibleResults as result, idx (`${result.book_id}-${result.chapter}-${result.verse}`)}
              {@const isSelected = selectedIndex === idx}
              {@const resKey = `res-${result.book_id}-${result.chapter}-${result.verse}`}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                role="button"
                tabindex="0"
                onclick={() => presentBibleResult(result)}
                class="w-full px-3 py-2.5 rounded-lg flex items-center justify-between transition-colors cursor-pointer text-left {isSelected ? 'bg-zinc-800/90 ring-1 ring-amber-500/50' : 'hover:bg-zinc-900/80'}"
              >
                <div class="flex items-start gap-2.5 min-w-0 pr-3">
                  <div class="w-6 h-6 rounded bg-amber-500/10 text-amber-400 flex items-center justify-center shrink-0 mt-0.5">
                    <BookOpen size={13} />
                  </div>
                  <div class="min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="text-xs font-bold text-white truncate">
                        {result.book_name} {result.chapter}:{result.verse}
                      </span>
                      <Badge variant="outline" class="text-[9px] py-0 px-1 border-amber-500/40 text-amber-300 font-mono">
                        {result.book_abbreviation || currentBible?.abbreviation}
                      </Badge>
                    </div>
                    <div class="text-xs text-zinc-300 line-clamp-2 mt-0.5 leading-snug">
                      {result.text}
                    </div>
                  </div>
                </div>

                <div class="flex items-center gap-1.5 shrink-0">
                  <button
                    type="button"
                    onclick={(e) => addBibleResultToSetlist(result, e)}
                    class="px-2.5 py-1 rounded bg-muted hover:bg-amber-600 text-foreground hover:text-white border border-border text-xs font-medium transition-colors flex items-center gap-1.5 cursor-pointer"
                    title="Adicionar versículo à Setlist"
                  >
                    {#if addedVerseKeys[resKey]}
                      <Check size={12} class="text-emerald-400" />
                      <span>Adicionado</span>
                    {:else}
                      <Plus size={12} />
                      <span>Add</span>
                    {/if}
                  </button>

                  <button
                    type="button"
                    onclick={() => presentBibleResult(result)}
                    class="px-2.5 py-1 rounded bg-amber-500 hover:bg-amber-400 text-black text-xs font-bold transition-colors flex items-center gap-1.5 cursor-pointer"
                    title="Projetar este versículo ao vivo"
                  >
                    <Play size={12} class="fill-current" />
                    <span>Ao Vivo</span>
                  </button>
                </div>
              </div>
            {/each}
          {:else if query && !isLoading}
            <div class="p-8 text-center text-zinc-500 text-xs space-y-2">
              <p>Nenhum texto bíblico encontrado para "{query}" na versão {currentBible?.abbreviation || ''}.</p>
              <p class="text-[11px] text-zinc-600">
                Tente uma referência como <strong class="text-zinc-400">João 3:16</strong>, <strong class="text-zinc-400">Sl 23:1-4</strong>, ou palavras como <strong class="text-zinc-400">luz do mundo</strong>.
              </p>
            </div>
          {:else if !query}
            <div class="p-8 text-center text-zinc-500 text-xs space-y-2">
              <BookOpen size={24} class="mx-auto text-amber-500/40 mb-2" />
              <p class="font-medium text-zinc-400">Pesquise textos bíblicos por referência ou palavras-chave</p>
              <p class="text-[11px] text-zinc-500">
                Exemplos: <code class="px-1.5 py-0.5 rounded bg-zinc-900 border border-zinc-800 text-amber-400">João 3:16</code>, <code class="px-1.5 py-0.5 rounded bg-zinc-900 border border-zinc-800 text-amber-400">Sl 23:1-4</code>, <code class="px-1.5 py-0.5 rounded bg-zinc-900 border border-zinc-800 text-amber-400">luz do mundo</code>, <code class="px-1.5 py-0.5 rounded bg-zinc-900 border border-zinc-800 text-amber-400">princípio</code>
              </p>
            </div>
          {/if}
        {:else}
          <!-- ABA ONLINE: Provedores (Holyrics / Vagalume) -->
          {#if onlineResults.length > 0}
            <div class="px-2 py-1 text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
              Resultados da Internet ({onlineResults.length})
            </div>

            {#each onlineResults as hit, idx}
              {@const isSelected = selectedIndex === idx}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                role="button"
                tabindex="0"
                onclick={() => importOnlineHit(hit)}
                class="w-full px-3 py-2.5 rounded-lg flex items-center justify-between transition-colors cursor-pointer text-left {isSelected ? 'bg-zinc-800/90 ring-1 ring-sky-500/50' : 'hover:bg-zinc-900/80'}"
              >
                <div class="flex items-start gap-2.5 min-w-0 pr-2">
                  <div class="w-6 h-6 rounded bg-sky-500/20 text-sky-400 flex items-center justify-center shrink-0 mt-0.5">
                    <Globe size={13} />
                  </div>
                  <div class="min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="text-xs font-medium text-white truncate">{hit.track_name}</span>
                      <Badge
                        variant="secondary"
                        class="text-[9px] py-0 px-1 {hit.provider === 'holyrics' ? 'bg-emerald-950 text-emerald-300 border border-emerald-800/50' : 'bg-blue-950 text-blue-300 border border-blue-800/50'}"
                      >
                        {hit.provider === 'holyrics' ? 'Holyrics' : 'Vagalume'}
                      </Badge>
                    </div>
                    <div class="text-[11px] text-zinc-400 truncate">
                      {hit.artist_name || 'Desconhecido'}
                    </div>
                  </div>
                </div>

                <button
                  type="button"
                  onclick={() => importOnlineHit(hit)}
                  class="px-2.5 py-1 rounded bg-muted hover:bg-sky-600 text-foreground hover:text-white border border-border text-xs font-medium transition-colors shrink-0 flex items-center gap-1.5 cursor-pointer"
                >
                  <FileText size={12} />
                  <span>Importar e Editar</span>
                </button>
              </div>
            {/each}
          {:else if query && !isLoading}
            <div class="p-8 text-center text-zinc-500 text-xs">
              Nenhum resultado online para "{query}". Tente buscar por outros termos ou crie manualmente.
            </div>
          {:else if !query}
            <div class="p-8 text-center text-zinc-500 text-xs">
              Digite o nome de um louvor, artista ou trecho para pesquisar online.
            </div>
          {/if}
        {/if}
      </div>

      <!-- Rodapé de Dicas de Teclado -->
      <div class="h-9 px-4 border-t border-zinc-800/80 bg-zinc-900/40 flex items-center justify-between text-[11px] text-zinc-500 shrink-0">
        <div class="flex items-center gap-3">
          <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">↑↓</kbd> Navegar</span>
          <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Enter</kbd> Selecionar</span>
          <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Shift+Enter</kbd> Add Setlist</span>
        </div>
        <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Esc</kbd> Fechar</span>
      </div>
    </div>
  </div>
{/if}

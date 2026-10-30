<script lang="ts">
  import { onMount } from 'svelte';
  import { parseRawLyrics } from '$lib/songParser';
  import { parseBibleReference } from '$lib/bibleParser';
  import {
    presentationState,
    isWsConnected,
    initWebSocket,
    selectSlide,
    setDisplayMode,
    stopPresentation,
    quickPresentMedia,
  } from '$lib/ws';
  import type {
    MediaItem,
    Setlist,
    Song,
    SongSummary,
    BibleVersionSummary,
    BibleBook,
    BibleVerse,
    Slide,
  } from '$lib/types';
  import {
    ListMusic,
    Music,
    BookOpen,
    ChevronLeft,
    ChevronRight,
    ChevronDown,
    ChevronUp,
    Square,
    EyeOff,
    Search,
    Play,
    RefreshCw,
    X,
    Check,
    Layers,
  } from 'lucide-svelte';

  // Navegação mobile entre as 3 abas principais
  type RemoteTab = 'setlist' | 'songs' | 'bible';
  let activeTab = $state<RemoteTab>('setlist');

  // Estado da Setlist
  interface SetlistListItem {
    id: string;
    title: string;
    description: string | null;
    item_count: number;
    is_active: boolean;
  }
  let availableSetlists = $state<SetlistListItem[]>([]);
  let activeSetlistData = $state<Setlist | null>(null);
  let isLoadingSetlist = $state(false);
  let expandedSetlistItemId = $state<string | null>(null);
  let isSetlistDrawerOpen = $state(false);

  // Estado do Card de Controle ao Vivo (Colapsável)
  let isLiveCardCollapsed = $state(false);

  // Estado das Músicas
  let searchQuery = $state('');
  let songsList = $state<SongSummary[]>([]);
  let isLoadingSongs = $state(false);
  let selectedSong = $state<Song | null>(null);
  let isLoadingSongDetail = $state(false);
  let songSearchTimeout: ReturnType<typeof setTimeout> | null = null;

  // Estado da Bíblia
  type BibleNavTab = 'books' | 'chapters' | 'verses';
  let bibleNavTab = $state<BibleNavTab>('verses');
  let bibles = $state<BibleVersionSummary[]>([]);
  let activeBibleId = $state<string>('');
  let books = $state<BibleBook[]>([]);
  let selectedBook = $state<BibleBook | null>(null);
  let selectedChapter = $state<number>(1);
  let verses = $state<BibleVerse[]>([]);
  let isLoadingBible = $state(false);
  let bibleQuery = $state('');
  let bibleSearchResults = $state<any[]>([]);
  let isSearchingBible = $state(false);
  let testamentFilter = $state<'all' | 'ot' | 'nt'>('all');
  let isBibleDrawerOpen = $state(false);

  // Derivados do estado de apresentação
  const isLive = $derived(
    $presentationState.display_mode === 'live' && !!$presentationState.active_media_id
  );
  const isBlackout = $derived($presentationState.display_mode === 'blackout');
  const isClear = $derived($presentationState.display_mode === 'cleartext');

  // Slide atual ao vivo
  const currentMediaTitle = $derived.by(() => {
    if (!$presentationState.active_media_id) return 'Nenhuma apresentação ativa';
    if ($presentationState.quick_media) {
      if (currentSlide?.label) {
        return currentSlide.label;
      }
      return $presentationState.quick_media.title;
    }
    const item = (activeSetlistData?.items ?? []).find(
      (i) => i.id === $presentationState.active_media_id
    );
    if (currentSlide?.label && item) {
      return `${item.title} — ${currentSlide.label}`;
    }
    return item?.title || 'Apresentação ativa';
  });

  const currentMediaSlides = $derived.by(() => {
    if ($presentationState.quick_media) {
      return $presentationState.quick_media.slides || [];
    }
    const item = (activeSetlistData?.items ?? []).find(
      (i) => i.id === $presentationState.active_media_id
    );
    return item?.slides || [];
  });

  const currentSlideIndex = $derived($presentationState.active_slide_index ?? 0);
  const totalSlides = $derived(currentMediaSlides.length);
  const currentSlide = $derived(
    totalSlides > 0 && currentSlideIndex >= 0 && currentSlideIndex < totalSlides
      ? currentMediaSlides[currentSlideIndex]
      : null
  );

  // Sigla da Bíblia ativa para o Input Group
  const activeBibleSummary = $derived(
    bibles.find((b) => b.id === activeBibleId) || bibles[0]
  );
  const activeBibleAbbr = $derived(
    activeBibleSummary?.abbreviation || activeBibleSummary?.name || 'Bíblia'
  );

  // Livros filtrados por testamento
  const filteredBooks = $derived(
    books.filter((b) => {
      if (testamentFilter === 'ot' && b.testament_reference_id !== 1) return false;
      if (testamentFilter === 'nt' && b.testament_reference_id !== 2) return false;
      return true;
    })
  );

  onMount(() => {
    if (typeof document !== 'undefined') {
      document.title = 'OpenPresenter — Controle Remoto';
    }
    initWebSocket('remote');
    loadSetlistsAndActive();
    loadSongs();
    loadBibles();
  });

  // Reagir a mudanças no active_setlist_id do WebSocket
  $effect(() => {
    if ($presentationState.active_setlist_id) {
      loadSingleSetlistDetail($presentationState.active_setlist_id);
    }
  });

  // Carregar lista de setlists e obter o detalhe da ativa
  async function loadSetlistsAndActive() {
    isLoadingSetlist = true;
    try {
      const res = await fetch('/api/setlists');
      if (res.ok) {
        availableSetlists = await res.json();
        const target =
          availableSetlists.find((s) => s.id === $presentationState.active_setlist_id || s.is_active) ||
          availableSetlists[0] ||
          null;
        if (target) {
          await loadSingleSetlistDetail(target.id);
        } else {
          activeSetlistData = null;
        }
      }
    } catch (err) {
      console.error('Erro ao listar setlists:', err);
    } finally {
      isLoadingSetlist = false;
    }
  }

  // Carregar os itens completos da setlist pelo ID
  async function loadSingleSetlistDetail(setlistId: string) {
    isLoadingSetlist = true;
    try {
      const res = await fetch(`/api/setlists/${setlistId}`);
      if (res.ok) {
        const data: Setlist = await res.json();
        activeSetlistData = data;
        if (!expandedSetlistItemId && $presentationState.active_media_id) {
          expandedSetlistItemId = $presentationState.active_media_id;
        }
      }
    } catch (err) {
      console.error('Erro ao buscar detalhes da setlist:', err);
    } finally {
      isLoadingSetlist = false;
    }
  }

  // Carregar Músicas
  async function loadSongs(q: string = '') {
    isLoadingSongs = true;
    try {
      const url = q.trim() ? `/api/songs?q=${encodeURIComponent(q.trim())}` : '/api/songs';
      const res = await fetch(url);
      if (res.ok) {
        songsList = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar músicas:', err);
    } finally {
      isLoadingSongs = false;
    }
  }

  function handleSongSearchInput(e: Event) {
    const target = e.target as HTMLInputElement;
    searchQuery = target.value;
    if (songSearchTimeout) clearTimeout(songSearchTimeout);
    songSearchTimeout = setTimeout(() => {
      loadSongs(searchQuery);
    }, 250);
  }

  async function openSongDetail(songSummary: SongSummary) {
    isLoadingSongDetail = true;
    try {
      const res = await fetch(`/api/songs/${songSummary.id}`);
      if (res.ok) {
        selectedSong = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar detalhes da música:', err);
    } finally {
      isLoadingSongDetail = false;
    }
  }

  function songToSlides(song: Song): Slide[] {
    if (song.lyrics_raw && song.lyrics_raw.trim()) {
      const parsed = parseRawLyrics(song.lyrics_raw);
      if (parsed.slides && parsed.slides.length > 0) {
        return parsed.slides.map((s, idx) => ({
          ...s,
          id: `${song.id}-${idx}`,
        }));
      }
    }
    if (song.sections && song.sections.length > 0) {
      return song.sections.map((sec, idx) => ({
        id: `${song.id}-${idx}`,
        label: sec.label || `Parte ${idx + 1}`,
        content: sec.lines.join('\n'),
      }));
    }
    const blocks = (song.lyrics_raw || '').split(/\n\n+/).filter((b) => b.trim());
    return blocks.map((b, idx) => ({
      id: `${song.id}-${idx}`,
      label: `Parte ${idx + 1}`,
      content: b.trim(),
    }));
  }

  // Carregar Bíblias
  async function loadBibles() {
    isLoadingBible = true;
    try {
      const res = await fetch('/api/bibles');
      if (res.ok) {
        bibles = await res.json();
        const active = bibles.find((b) => b.is_default) || bibles[0];
        if (active) {
          activeBibleId = active.id;
          await loadBooks(active.id);
        }
      }
    } catch (err) {
      console.error('Erro ao carregar bíblias:', err);
    } finally {
      isLoadingBible = false;
    }
  }

  async function loadBooks(bibleId: string) {
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books`);
      if (res.ok) {
        books = await res.json();
        if (books.length > 0 && !selectedBook) {
          selectedBook = books[0];
          selectedChapter = 1;
          await loadVerses(bibleId, books[0].id, 1);
        }
      }
    } catch (err) {
      console.error('Erro ao carregar livros da bíblia:', err);
    }
  }

  async function selectBook(book: BibleBook) {
    selectedBook = book;
    selectedChapter = 1;
    if (activeBibleId) {
      await loadVerses(activeBibleId, book.id, 1);
    }
  }

  async function selectChapter(ch: number) {
    selectedChapter = ch;
    if (activeBibleId && selectedBook) {
      await loadVerses(activeBibleId, selectedBook.id, ch);
    }
  }

  async function loadVerses(bibleId: string, bookId: number, chapter: number) {
    isLoadingBible = true;
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books/${bookId}/chapters/${chapter}/verses`);
      if (res.ok) {
        verses = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar versículos:', err);
    } finally {
      isLoadingBible = false;
    }
  }

  async function searchBible(query: string) {
    const clean = query.trim();
    if (!clean || !activeBibleId) {
      bibleSearchResults = [];
      isSearchingBible = false;
      return;
    }

    // 1. Tenta interpretar como referência bíblica direta (ex: "Jo 3:16", "Sl 23", "Gênesis 1")
    const parsed = parseBibleReference(clean);
    if (parsed) {
      const targetBook = books.find(
        (b) =>
          b.id === parsed.book_id ||
          b.name.toLowerCase() === parsed.book_name.toLowerCase() ||
          b.abbreviation.toLowerCase() === parsed.book_abbreviation.toLowerCase()
      );
      if (targetBook) {
        selectedBook = targetBook;
        selectedChapter = parsed.chapter;
        await loadVerses(activeBibleId, targetBook.id, parsed.chapter);
        bibleNavTab = 'verses';
        bibleSearchResults = [];

        if (parsed.verse_start) {
          const verseObj = verses.find((v) => v.verse === parsed.verse_start);
          if (verseObj) {
            presentVerse(verseObj);
          }
        }
        return;
      }
    }

    // 2. Busca textual FTS no endpoint oficial: /api/bibles/{id}/search
    isSearchingBible = true;
    try {
      const res = await fetch(
        `/api/bibles/${activeBibleId}/search?q=${encodeURIComponent(clean)}&limit=50`
      );
      if (res.ok) {
        bibleSearchResults = await res.json();
      } else {
        bibleSearchResults = [];
      }
    } catch (err) {
      console.error('Erro ao pesquisar texto bíblico:', err);
      bibleSearchResults = [];
    } finally {
      isSearchingBible = false;
    }
  }

  // Ações de Apresentação
  function handleNextSlide() {
    if (totalSlides > 0 && currentSlideIndex < totalSlides - 1) {
      const mediaId = $presentationState.active_media_id;
      if (mediaId) {
        selectSlide(mediaId, currentSlideIndex + 1);
      }
    }
  }

  function handlePrevSlide() {
    if (totalSlides > 0 && currentSlideIndex > 0) {
      const mediaId = $presentationState.active_media_id;
      if (mediaId) {
        selectSlide(mediaId, currentSlideIndex - 1);
      }
    }
  }

  function handleSelectSlide(mediaId: string, slideIdx: number) {
    selectSlide(mediaId, slideIdx);
  }

  function handleToggleBlackout() {
    if (isBlackout) {
      setDisplayMode('live');
    } else {
      setDisplayMode('blackout');
    }
  }

  function handleToggleClear() {
    if (isClear) {
      setDisplayMode('live');
    } else {
      setDisplayMode('cleartext');
    }
  }

  function handleStop() {
    stopPresentation();
  }

  // Apresentar Música diretamente
  function presentSong(song: Song, slideIdx: number = 0) {
    const slides = songToSlides(song);
    const mediaItem: MediaItem = {
      id: song.id,
      kind: 'song',
      title: song.title,
      artist_or_ref: song.artist,
      slides,
    };
    quickPresentMedia(mediaItem, slideIdx);
  }

  // Apresentar Versículo Bíblico: projeta o capítulo inteiro com versículos nos slides
  function presentVerse(verse: BibleVerse) {
    if (!selectedBook || verses.length === 0) return;
    const currentBook = selectedBook;
    const bookName = currentBook.name;
    const targetIdx = verses.findIndex((v) => v.verse === verse.verse);
    const initialSlide = targetIdx >= 0 ? targetIdx : 0;

    const mediaItem: MediaItem = {
      id: `scripture-${activeBibleId}-${currentBook.id}-${selectedChapter}`,
      kind: 'scripture',
      title: `${bookName} ${selectedChapter}`,
      artist_or_ref: activeBibleAbbr.toUpperCase(),
      slides: verses.map((v) => ({
        id: `scripture-${currentBook.id}-${selectedChapter}-${v.verse}`,
        label: `${bookName} ${selectedChapter}:${v.verse}`,
        content: v.text,
      })),
    };
    quickPresentMedia(mediaItem, initialSlide);
  }

  // Apresentar resultado da busca bíblica: carrega capítulo completo com versículos nos slides
  async function presentSearchResult(hit: any) {
    const bookId = hit.book_id;
    const ch = hit.chapter;
    const vNum = hit.verse;

    const targetBook = books.find((b) => b.id === bookId);
    if (targetBook) {
      selectedBook = targetBook;
      selectedChapter = ch;
    }

    try {
      const res = await fetch(`/api/bibles/${activeBibleId}/books/${bookId}/chapters/${ch}/verses`);
      if (res.ok) {
        const chapterVerses: BibleVerse[] = await res.json();
        verses = chapterVerses;
        bibleNavTab = 'verses';
        bibleSearchResults = [];

        const targetIdx = chapterVerses.findIndex((v) => v.verse === vNum);
        const initialSlide = targetIdx >= 0 ? targetIdx : 0;
        const bookName = hit.book_name || targetBook?.name || 'Bíblia';

        const mediaItem: MediaItem = {
          id: `scripture-${activeBibleId}-${bookId}-${ch}`,
          kind: 'scripture',
          title: `${bookName} ${ch}`,
          artist_or_ref: (hit.bible_version || activeBibleAbbr).toUpperCase(),
          slides: chapterVerses.map((v) => ({
            id: `scripture-${bookId}-${ch}-${v.verse}`,
            label: `${bookName} ${ch}:${v.verse}`,
            content: v.text,
          })),
        };
        quickPresentMedia(mediaItem, initialSlide);
      }
    } catch (err) {
      console.error('Erro ao carregar versículos do resultado de busca:', err);
    }
  }
</script>

<div class="flex flex-col h-screen w-screen max-w-md mx-auto bg-neutral-950 text-neutral-100 select-none overflow-hidden font-sans">
  <!-- TOP BAR: Status, Logotipo Oficial e Ações Rápidas de Emergência com Safe-Area -->
  <header
    class="flex items-center justify-between px-3 py-2.5 bg-neutral-900 border-b border-neutral-800 shrink-0"
    style="padding-top: max(env(safe-area-inset-top), 10px);"
  >
    <!-- Logotipo Oficial OpenPresenter Minimal -->
    <div class="flex items-center gap-2">
      <img
        src="/branding/open-presenter-icon-dark.svg"
        alt="OpenPresenter"
        class="w-7 h-7 rounded-lg shadow-sm shrink-0 border border-neutral-700/50"
      />
      <div class="flex items-center gap-1.5">
        <span class="font-bold text-xs tracking-tight text-white flex items-center">
          Open<span class="text-neutral-400 font-medium">Presenter</span>
        </span>
        <span
          class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full text-[9px] font-bold {$isWsConnected ? 'bg-emerald-950 text-emerald-400 border border-emerald-800' : 'bg-red-950 text-red-400 border border-red-800'}"
        >
          <span
            class="w-1.5 h-1.5 rounded-full {$isWsConnected ? 'bg-emerald-400 animate-pulse' : 'bg-red-400'}"
          ></span>
          {$isWsConnected ? 'Remoto' : 'Offline'}
        </span>
      </div>
    </div>

    <!-- Botões de Ação de Projeção (Tamanho adequado para toque) -->
    <div class="flex items-center gap-1.5">
      <button
        type="button"
        onclick={handleToggleBlackout}
        class="h-9 px-2.5 text-xs font-bold rounded-lg flex items-center gap-1.5 transition-all active:scale-95 cursor-pointer {isBlackout ? 'bg-red-600 text-white shadow-lg shadow-red-950' : 'bg-neutral-800 hover:bg-neutral-700 text-neutral-200'}"
        title="Blackout (Tela Preta)"
      >
        <span class="w-2 h-2 rounded-full bg-red-500"></span>
        <span>Preto</span>
      </button>

      <button
        type="button"
        onclick={handleToggleClear}
        class="h-9 px-2.5 text-xs font-bold rounded-lg flex items-center gap-1.5 transition-all active:scale-95 cursor-pointer {isClear ? 'bg-amber-600 text-white shadow-lg shadow-amber-950' : 'bg-neutral-800 hover:bg-neutral-700 text-neutral-200'}"
        title="Limpar Texto da Tela"
      >
        <EyeOff size={13} />
        <span>Limpar</span>
      </button>

      <button
        type="button"
        onclick={handleStop}
        class="h-9 w-9 flex items-center justify-center text-neutral-400 hover:text-white bg-neutral-800 hover:bg-neutral-700 active:scale-95 rounded-lg transition-all cursor-pointer"
        title="Parar Apresentação"
        aria-label="Parar Apresentação"
      >
        <Square size={14} />
      </button>
    </div>
  </header>

  <!-- LIVE CONTROL CARD: Fixado no topo, com Opção de Colapsar e botões para o polegar -->
  <section class="p-2.5 bg-neutral-900/80 border-b border-neutral-800 shrink-0 transition-all duration-200">
    <div class="flex items-center justify-between text-xs mb-1.5">
      <div class="flex items-center gap-1.5 font-bold truncate text-emerald-400 min-w-0 pr-2">
        <span class="w-2 h-2 rounded-full shrink-0 {isLive ? 'bg-emerald-400 animate-ping' : 'bg-neutral-600'}"></span>
        <span class="truncate">{currentMediaTitle}</span>
      </div>

      <div class="flex items-center gap-1.5 shrink-0">
        {#if totalSlides > 0}
          <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 font-bold">
            {currentSlideIndex + 1}/{totalSlides}
          </span>
        {/if}

        <!-- Botão de Colapsar / Expandir -->
        <button
          type="button"
          onclick={() => (isLiveCardCollapsed = !isLiveCardCollapsed)}
          class="p-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-400 hover:text-white transition-colors cursor-pointer"
          title={isLiveCardCollapsed ? 'Expandir Card de Controle' : 'Recolher Card de Controle'}
          aria-label={isLiveCardCollapsed ? 'Expandir Card de Controle' : 'Recolher Card de Controle'}
        >
          {#if isLiveCardCollapsed}
            <ChevronDown size={14} />
          {:else}
            <ChevronUp size={14} />
          {/if}
        </button>
      </div>
    </div>

    <!-- Modo Recolhido / Compacto -->
    {#if isLiveCardCollapsed}
      <div class="flex items-center justify-between gap-2 pt-1">
        <p class="text-xs text-neutral-300 truncate flex-1 font-medium italic">
          {currentSlide ? currentSlide.content.replace(/\n/g, ' ') : 'Nenhum slide ativo'}
        </p>

        <div class="flex items-center gap-1 shrink-0">
          <button
            type="button"
            onclick={handlePrevSlide}
            disabled={totalSlides === 0 || currentSlideIndex <= 0}
            class="h-8 px-2.5 rounded-lg bg-neutral-800 hover:bg-neutral-700 disabled:opacity-30 disabled:pointer-events-none text-neutral-200 font-bold text-xs flex items-center gap-1 cursor-pointer"
          >
            <ChevronLeft size={14} />
            <span>Ant.</span>
          </button>
          <button
            type="button"
            onclick={handleNextSlide}
            disabled={totalSlides === 0 || currentSlideIndex >= totalSlides - 1}
            class="h-8 px-2.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 disabled:opacity-30 disabled:pointer-events-none text-white font-bold text-xs flex items-center gap-1 cursor-pointer"
          >
            <span>Próx.</span>
            <ChevronRight size={14} />
          </button>
        </div>
      </div>
    {:else}
      <!-- Modo Expandido: Prévia do Slide Ativo e Botões Gigantes para o Polegar -->
      <div class="p-2.5 rounded-xl bg-black/60 border border-neutral-800/80 mb-2 min-h-[52px] flex items-center justify-center text-center">
        {#if currentSlide}
          <p class="text-xs font-semibold leading-relaxed line-clamp-2 text-neutral-200">
            {currentSlide.content.replace(/\n/g, ' ')}
          </p>
        {:else}
          <span class="text-xs text-neutral-500 italic">
            {isBlackout ? '⚫ Projeção em Blackout' : isClear ? '👁️ Texto ocultado' : 'Nenhum slide em exibição'}
          </span>
        {/if}
      </div>

      <div class="grid grid-cols-2 gap-2">
        <button
          type="button"
          onclick={handlePrevSlide}
          disabled={totalSlides === 0 || currentSlideIndex <= 0}
          class="h-12 rounded-xl bg-neutral-800 hover:bg-neutral-700 active:bg-neutral-600 disabled:opacity-40 disabled:pointer-events-none text-neutral-200 font-bold text-sm flex items-center justify-center gap-1.5 transition-all shadow active:scale-95 cursor-pointer"
        >
          <ChevronLeft size={18} />
          <span>Anterior</span>
        </button>

        <button
          type="button"
          onclick={handleNextSlide}
          disabled={totalSlides === 0 || currentSlideIndex >= totalSlides - 1}
          class="h-12 rounded-xl bg-emerald-600 hover:bg-emerald-500 active:bg-emerald-700 disabled:opacity-40 disabled:pointer-events-none text-white font-extrabold text-sm flex items-center justify-center gap-1.5 transition-all shadow-lg shadow-emerald-950 active:scale-95 cursor-pointer"
        >
          <span>Próximo</span>
          <ChevronRight size={18} />
        </button>
      </div>
    {/if}
  </section>

  <!-- CONTEÚDO PRINCIPAL: Scrollável conforme a aba selecionada -->
  <main class="flex-1 overflow-y-auto overflow-x-hidden min-h-0 bg-neutral-950 p-2.5">
    <!-- ABA 1: SETLIST -->
    {#if activeTab === 'setlist'}
      <div class="space-y-2">
        <div class="flex items-center justify-between px-1 text-xs text-neutral-400 font-medium">
          <!-- Botão para alternar entre setlists se houver mais de uma -->
          <button
            type="button"
            onclick={() => (isSetlistDrawerOpen = true)}
            class="flex items-center gap-1 font-bold text-neutral-200 hover:text-white transition-colors cursor-pointer"
          >
            <span class="truncate max-w-[240px]">{activeSetlistData?.title || 'Setlist Ativa'}</span>
            <ChevronDown size={14} class="text-neutral-400" />
          </button>

          <button
            type="button"
            onclick={loadSetlistsAndActive}
            class="p-1 hover:text-white transition-colors cursor-pointer"
            title="Atualizar Setlist"
            aria-label="Atualizar Setlist"
          >
            <RefreshCw size={13} class={isLoadingSetlist ? 'animate-spin' : ''} />
          </button>
        </div>

        {#if !activeSetlistData || (activeSetlistData.items ?? []).length === 0}
          <div class="p-8 text-center text-neutral-500 text-xs">
            <ListMusic size={32} class="mx-auto mb-2 opacity-40" />
            <p>Nenhum item na setlist ativa.</p>
            <p class="text-[10px] mt-1 text-neutral-600">
              Adicione músicas ou passagens pelo painel do operador ou selecione outra setlist.
            </p>
          </div>
        {:else}
          <div class="space-y-1.5">
            {#each activeSetlistData.items as item, itemIdx}
              {@const isItemActive = $presentationState.active_media_id === item.id}
              {@const isExpanded = expandedSetlistItemId === item.id}
              <div
                class="rounded-xl border transition-all overflow-hidden {isItemActive ? 'bg-neutral-900 border-emerald-500 shadow-lg shadow-emerald-950/20' : 'bg-neutral-900/60 border-neutral-800/80'}"
              >
                <!-- Cabeçalho do item da setlist -->
                <button
                  type="button"
                  onclick={() => {
                    expandedSetlistItemId = isExpanded ? null : item.id;
                  }}
                  class="w-full px-3 py-2.5 flex items-center justify-between text-left cursor-pointer"
                >
                  <div class="flex items-center gap-2 min-w-0 pr-2">
                    <span
                      class="w-6 h-6 rounded-full text-xs font-bold flex items-center justify-center shrink-0 {isItemActive ? 'bg-emerald-500 text-black shadow' : 'bg-neutral-800 text-neutral-400'}"
                    >
                      {itemIdx + 1}
                    </span>
                    <div class="truncate">
                      <p class="text-xs font-bold truncate text-neutral-200 {isItemActive ? 'text-emerald-400' : ''}">
                        {item.title}
                      </p>
                      {#if item.artist_or_ref}
                        <p class="text-[10px] text-neutral-400 truncate">{item.artist_or_ref}</p>
                      {/if}
                    </div>
                  </div>

                  <div class="flex items-center gap-1.5 shrink-0">
                    {#if isItemActive}
                      <span class="px-1.5 py-0.5 rounded text-[9px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40 animate-pulse">
                        Ao Vivo
                      </span>
                    {/if}
                    <span class="text-[10px] text-neutral-500 font-mono">
                      {item.slides.length} slides
                    </span>
                  </div>
                </button>

                <!-- Slides expandidos com toque direto para projetar -->
                {#if isExpanded}
                  <div class="px-2 pb-2 pt-1 border-t border-neutral-800/60 space-y-1 bg-black/40">
                    {#each item.slides as slide, sIdx}
                      {@const isCurrentSlideLive = isItemActive && currentSlideIndex === sIdx && isLive}
                      <button
                        type="button"
                        onclick={() => handleSelectSlide(item.id, sIdx)}
                        class="w-full text-left p-2.5 rounded-lg text-xs flex items-start gap-2 transition-all cursor-pointer {isCurrentSlideLive ? 'bg-emerald-600/30 border border-emerald-500 text-white font-semibold ring-1 ring-emerald-500' : 'bg-neutral-900/80 hover:bg-neutral-800 text-neutral-300 border border-neutral-800/40'}"
                      >
                        <span
                          class="text-[9px] px-1.5 py-0.5 rounded font-mono shrink-0 mt-0.5 {isCurrentSlideLive ? 'bg-emerald-500 text-black font-extrabold' : 'bg-neutral-800 text-neutral-400'}"
                        >
                          {slide.label || `${sIdx + 1}`}
                        </span>
                        <p class="line-clamp-2 leading-relaxed flex-1">
                          {slide.content.replace(/\n/g, ' ')}
                        </p>
                      </button>
                    {/each}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>

    <!-- ABA 2: MÚSICAS -->
    {:else if activeTab === 'songs'}
      <div class="space-y-2">
        <!-- Barra de busca de músicas com botão de recarregar -->
        <div class="flex items-center gap-1.5">
          <div class="relative flex-1">
            <Search size={14} class="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400 pointer-events-none" />
            <input
              type="text"
              value={searchQuery}
              oninput={handleSongSearchInput}
              placeholder="Buscar por título ou trecho da letra..."
              class="w-full pl-8 pr-8 py-2.5 rounded-xl bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder:text-neutral-500 focus:outline-none focus:border-emerald-500 transition-colors"
            />
            {#if searchQuery}
              <button
                type="button"
                onclick={() => {
                  searchQuery = '';
                  loadSongs('');
                }}
                class="absolute right-2.5 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-white p-1 cursor-pointer"
              >
                <X size={13} />
              </button>
            {/if}
          </div>

          <button
            type="button"
            onclick={() => loadSongs(searchQuery)}
            class="h-10 w-10 rounded-xl bg-neutral-900 border border-neutral-800 hover:bg-neutral-800 active:scale-95 text-neutral-300 hover:text-white flex items-center justify-center shrink-0 transition-all cursor-pointer"
            title="Recarregar catálogo de músicas"
            aria-label="Recarregar catálogo de músicas"
          >
            <RefreshCw size={15} class={isLoadingSongs ? 'animate-spin text-emerald-400' : ''} />
          </button>
        </div>

        <!-- Visualização detalhada da música selecionada -->
        {#if isLoadingSongDetail}
          <div class="p-8 text-center text-neutral-500 text-xs">
            <RefreshCw size={20} class="animate-spin mx-auto mb-2" />
            <p>Carregando letra...</p>
          </div>
        {:else if selectedSong}
          {@const songSlides = songToSlides(selectedSong)}
          {@const isSongLive = $presentationState.active_media_id === selectedSong.id && isLive}
          <div class="p-3 rounded-xl bg-neutral-900 border border-neutral-800 space-y-2.5 animate-in fade-in duration-150">
            <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
              <div>
                <div class="flex items-center gap-2">
                  <h3 class="text-sm font-bold text-neutral-100">{selectedSong.title}</h3>
                  {#if isSongLive}
                    <span class="px-1.5 py-0.2 rounded text-[9px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40 animate-pulse">
                      Ao Vivo
                    </span>
                  {/if}
                </div>
                {#if selectedSong.artist}
                  <p class="text-xs text-neutral-400">{selectedSong.artist}</p>
                {/if}
              </div>
              <button
                type="button"
                onclick={() => (selectedSong = null)}
                class="p-1.5 rounded-lg bg-neutral-800 text-neutral-400 hover:text-white cursor-pointer"
              >
                <X size={14} />
              </button>
            </div>

            <!-- Botão Apresentar Música Completa -->
            <button
              type="button"
              onclick={() => presentSong(selectedSong!)}
              class="w-full py-2.5 rounded-xl text-white font-bold text-xs flex items-center justify-center gap-1.5 shadow-lg transition-colors cursor-pointer {isSongLive ? 'bg-emerald-700 hover:bg-emerald-600 shadow-emerald-950' : 'bg-emerald-600 hover:bg-emerald-500 shadow-emerald-950'}"
            >
              <Play size={14} />
              <span>{isSongLive ? 'Reiniciar Música do Início' : 'Apresentar Música Completa'}</span>
            </button>

            <!-- Slides / Estrofes da música com Active State -->
            <div class="space-y-1 pt-1">
              <span class="text-[10px] font-semibold text-neutral-400 uppercase tracking-wider">
                Toque em uma estrofe para projetar:
              </span>
              {#each songSlides as slide, sIdx}
                {@const isSlideLive = isSongLive && currentSlideIndex === sIdx}
                <button
                  type="button"
                  onclick={() => presentSong(selectedSong!, sIdx)}
                  class="w-full text-left p-2.5 rounded-xl border transition-all cursor-pointer {isSlideLive ? 'bg-emerald-950/50 border-emerald-500 ring-1 ring-emerald-500 text-white font-semibold' : 'bg-black/40 hover:bg-neutral-800 text-neutral-200 border-neutral-800/80'}"
                >
                  <div class="flex items-center justify-between mb-1">
                    <span
                      class="text-[9px] font-bold px-1.5 py-0.5 rounded {isSlideLive ? 'bg-emerald-500 text-black font-extrabold' : 'bg-neutral-800 text-neutral-400'}"
                    >
                      {slide.label}
                    </span>
                    {#if isSlideLive}
                      <span class="text-[9px] font-bold text-emerald-400 animate-pulse">Ao Vivo</span>
                    {/if}
                  </div>
                  <p class="leading-relaxed text-xs text-neutral-300">
                    {slide.content}
                  </p>
                </button>
              {/each}
            </div>
          </div>
        {:else}
          <!-- Lista de músicas filtradas -->
          <div class="space-y-1.5">
            {#if isLoadingSongs}
              <div class="p-8 text-center text-neutral-500 text-xs">
                <RefreshCw size={20} class="animate-spin mx-auto mb-2" />
                <p>Carregando músicas...</p>
              </div>
            {:else if songsList.length === 0}
              <div class="p-8 text-center text-neutral-500 text-xs">
                <Music size={24} class="mx-auto mb-2 opacity-40" />
                <p>Nenhuma música encontrada.</p>
              </div>
            {:else}
              {#each songsList as song}
                {@const isSongActive = $presentationState.active_media_id === song.id && isLive}
                <button
                  type="button"
                  onclick={() => openSongDetail(song)}
                  class="w-full px-3 py-2.5 rounded-xl border flex items-center justify-between text-left transition-colors cursor-pointer {isSongActive ? 'bg-neutral-900 border-emerald-500 shadow-md' : 'bg-neutral-900/80 hover:bg-neutral-850 active:bg-neutral-800 border-neutral-800/60'}"
                >
                  <div class="min-w-0 pr-2">
                    <p class="text-xs font-bold truncate {isSongActive ? 'text-emerald-400' : 'text-neutral-200'}">
                      {song.title}
                    </p>
                    {#if song.artist}
                      <p class="text-[10px] text-neutral-400 truncate">{song.artist}</p>
                    {/if}
                  </div>
                  <div class="flex items-center gap-1.5 shrink-0">
                    {#if isSongActive}
                      <span class="px-1.5 py-0.5 rounded text-[9px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40">
                        Ao Vivo
                      </span>
                    {/if}
                    <span class="text-[10px] text-neutral-500 font-mono">
                      Abrir
                    </span>
                  </div>
                </button>
              {/each}
            {/if}
          </div>
        {/if}
      </div>

    <!-- ABA 3: BÍBLIA -->
    {:else if activeTab === 'bible'}
      <div class="space-y-2.5">
        <!-- Input Group: Seletor de Versão (Drawer) + Campo de Busca Bíblica -->
        <div class="flex items-center rounded-xl bg-neutral-900 border border-neutral-800 focus-within:border-emerald-500 overflow-hidden shadow-sm">
          <!-- Botão de Tradução (abre Drawer do Shadcn) -->
          <button
            type="button"
            onclick={() => (isBibleDrawerOpen = true)}
            class="h-10 px-3 bg-neutral-850 hover:bg-neutral-800 text-neutral-200 font-bold text-xs flex items-center gap-1.5 border-r border-neutral-800 shrink-0 transition-colors cursor-pointer"
            title="Alternar Tradução Bíblica"
          >
            <span class="text-emerald-400 font-extrabold">{activeBibleAbbr}</span>
            <ChevronDown size={14} class="text-neutral-400" />
          </button>

          <!-- Input de Pesquisa -->
          <div class="relative flex-1 flex items-center">
            <button
              type="button"
              onclick={() => searchBible(bibleQuery)}
              class="absolute left-2.5 text-neutral-400 hover:text-white p-0.5 cursor-pointer transition-colors"
              title="Pesquisar"
              aria-label="Pesquisar"
            >
              <Search size={14} />
            </button>
            <input
              type="text"
              bind:value={bibleQuery}
              onkeydown={(e) => {
                if (e.key === 'Enter') searchBible(bibleQuery);
              }}
              placeholder="Ex: Jo 3:16 ou amor..."
              class="w-full h-10 pl-8 pr-8 bg-transparent text-xs text-neutral-100 placeholder:text-neutral-500 focus:outline-none"
            />
            {#if bibleQuery}
              <button
                type="button"
                onclick={() => {
                  bibleQuery = '';
                  bibleSearchResults = [];
                }}
                class="absolute right-2.5 p-1 text-neutral-400 hover:text-white cursor-pointer"
              >
                <X size={13} />
              </button>
            {/if}
          </div>
        </div>

        <!-- Resultados da pesquisa de versículos -->
        {#if isSearchingBible}
          <div class="p-6 text-center text-neutral-500 text-xs">
            <RefreshCw size={18} class="animate-spin mx-auto mb-1" />
            <span>Pesquisando na Bíblia...</span>
          </div>
        {:else if bibleSearchResults.length > 0}
          <div class="p-2 rounded-xl bg-neutral-900 border border-neutral-800 space-y-1">
            <div class="flex items-center justify-between pb-1 px-1 border-b border-neutral-800 text-[10px] text-neutral-400 font-semibold">
              <span>{bibleSearchResults.length} resultados encontrados</span>
              <button
                type="button"
                onclick={() => (bibleSearchResults = [])}
                class="text-neutral-400 hover:text-white cursor-pointer"
              >
                Fechar
              </button>
            </div>
            {#each bibleSearchResults as hit}
              <button
                type="button"
                onclick={() => presentSearchResult(hit)}
                class="w-full text-left p-2.5 rounded-lg bg-black/40 hover:bg-neutral-800 active:bg-neutral-700 text-xs border border-neutral-800/60 transition-colors cursor-pointer"
              >
                <div class="flex items-center justify-between font-bold text-emerald-400 text-xs mb-0.5">
                  <span>{hit.book_name} {hit.chapter}:{hit.verse}</span>
                  <span class="text-[9px] uppercase font-mono text-neutral-500">{hit.bible_version || activeBibleAbbr}</span>
                </div>
                <p class="text-neutral-300 leading-snug line-clamp-2">{hit.text}</p>
              </button>
            {/each}
          </div>
        {:else}
          <!-- NAVEGADOR BÍBLICO EM 3 SUB-ABAS: Livros | Capítulos | Versículos -->
          <div class="grid grid-cols-3 gap-1 p-0.5 rounded-xl bg-neutral-900 border border-neutral-800">
            <button
              type="button"
              onclick={() => (bibleNavTab = 'books')}
              class="py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer flex items-center justify-center gap-1 {bibleNavTab === 'books' ? 'bg-emerald-600 text-white shadow' : 'text-neutral-400 hover:text-white'}"
            >
              <span>Livros</span>
            </button>

            <button
              type="button"
              onclick={() => (bibleNavTab = 'chapters')}
              class="py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer flex items-center justify-center gap-1 {bibleNavTab === 'chapters' ? 'bg-emerald-600 text-white shadow' : 'text-neutral-400 hover:text-white'}"
            >
              <span>Capítulos</span>
            </button>

            <button
              type="button"
              onclick={() => (bibleNavTab = 'verses')}
              class="py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer flex items-center justify-center gap-1 {bibleNavTab === 'verses' ? 'bg-emerald-600 text-white shadow' : 'text-neutral-400 hover:text-white'}"
            >
              <span>Versículos</span>
            </button>
          </div>

          <!-- SUB-ABA 1: LIVROS EM GRADE -->
          {#if bibleNavTab === 'books'}
            <div class="space-y-2">
              <!-- Filtro de Testamento -->
              <div class="flex items-center gap-1">
                <button
                  type="button"
                  onclick={() => (testamentFilter = 'all')}
                  class="flex-1 py-1 text-[11px] font-bold rounded-lg transition-colors cursor-pointer {testamentFilter === 'all' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-900'}"
                >
                  Todos ({books.length})
                </button>
                <button
                  type="button"
                  onclick={() => (testamentFilter = 'ot')}
                  class="flex-1 py-1 text-[11px] font-bold rounded-lg transition-colors cursor-pointer {testamentFilter === 'ot' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-900'}"
                >
                  Antigo (39)
                </button>
                <button
                  type="button"
                  onclick={() => (testamentFilter = 'nt')}
                  class="flex-1 py-1 text-[11px] font-bold rounded-lg transition-colors cursor-pointer {testamentFilter === 'nt' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-900'}"
                >
                  Novo (27)
                </button>
              </div>

              <!-- Grade de Livros (3 colunas, botões altos confortáveis) -->
              <div class="grid grid-cols-3 gap-1.5 pb-2">
                {#each filteredBooks as book}
                  {@const isSelected = selectedBook?.id === book.id}
                  <button
                    type="button"
                    onclick={() => {
                      selectBook(book);
                      bibleNavTab = 'chapters';
                    }}
                    class="h-12 px-2 rounded-xl flex flex-col items-center justify-center border transition-all text-center cursor-pointer active:scale-95 {isSelected ? 'bg-emerald-600 border-emerald-500 text-white font-bold shadow' : 'bg-neutral-900 border-neutral-800 text-neutral-200 hover:bg-neutral-850'}"
                  >
                    <span class="text-xs font-semibold leading-tight line-clamp-1">{book.name}</span>
                    <span class="text-[9px] opacity-70 font-mono mt-0.5">{book.chapter_count} cap</span>
                  </button>
                {/each}
              </div>
            </div>

          <!-- SUB-ABA 2: CAPÍTULOS EM GRADE -->
          {:else if bibleNavTab === 'chapters'}
            <div class="space-y-2.5">
              <!-- Cabeçalho do Livro Selecionado -->
              <div class="flex items-center justify-between p-2.5 rounded-xl bg-neutral-900 border border-neutral-800">
                <div>
                  <span class="text-[10px] text-neutral-400 uppercase font-bold block">Livro Selecionado:</span>
                  <h4 class="text-sm font-bold text-white">{selectedBook?.name || 'Selecione um Livro'}</h4>
                </div>
                <button
                  type="button"
                  onclick={() => (bibleNavTab = 'books')}
                  class="px-2.5 py-1 text-xs font-bold rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-300 transition-colors cursor-pointer"
                >
                  Trocar Livro
                </button>
              </div>

              <span class="text-[10px] font-bold uppercase text-neutral-400 px-1 block">
                Selecione o Capítulo:
              </span>

              <!-- Grade de Capítulos (5 colunas com botões amplos para o toque) -->
              <div class="grid grid-cols-5 gap-2 pb-2">
                {#each Array.from({ length: selectedBook?.chapter_count || 1 }, (_, i) => i + 1) as ch}
                  {@const isSelected = selectedChapter === ch}
                  <button
                    type="button"
                    onclick={() => {
                      selectChapter(ch);
                      bibleNavTab = 'verses';
                    }}
                    class="h-12 rounded-xl flex items-center justify-center font-bold text-sm transition-all border cursor-pointer active:scale-95 {isSelected ? 'bg-emerald-600 border-emerald-500 text-white shadow-lg' : 'bg-neutral-900 border-neutral-800 text-neutral-200 hover:bg-neutral-850'}"
                  >
                    {ch}
                  </button>
                {/each}
              </div>
            </div>

          <!-- SUB-ABA 3: VERSÍCULOS COM TEXTO COMPLETO -->
          {:else if bibleNavTab === 'verses'}
            <div class="space-y-2">
              <!-- Navegação Rápida entre Capítulos -->
              <div class="flex items-center justify-between p-2 rounded-xl bg-neutral-900 border border-neutral-800">
                <button
                  type="button"
                  onclick={() => {
                    if (selectedChapter > 1) selectChapter(selectedChapter - 1);
                  }}
                  disabled={selectedChapter <= 1}
                  class="h-8 px-2 rounded-lg bg-neutral-800 hover:bg-neutral-700 disabled:opacity-30 disabled:pointer-events-none text-neutral-200 text-xs font-bold flex items-center gap-1 cursor-pointer"
                >
                  <ChevronLeft size={14} />
                  <span>Cap. {selectedChapter - 1}</span>
                </button>

                <button
                  type="button"
                  onclick={() => (bibleNavTab = 'chapters')}
                  class="font-extrabold text-xs text-emerald-400 hover:underline flex items-center gap-1 cursor-pointer"
                >
                  <span>{selectedBook?.name} {selectedChapter}</span>
                  <ChevronDown size={12} />
                </button>

                <button
                  type="button"
                  onclick={() => {
                    if (selectedBook && selectedChapter < (selectedBook.chapter_count || 1)) {
                      selectChapter(selectedChapter + 1);
                    }
                  }}
                  disabled={!selectedBook || selectedChapter >= (selectedBook.chapter_count || 1)}
                  class="h-8 px-2 rounded-lg bg-neutral-800 hover:bg-neutral-700 disabled:opacity-30 disabled:pointer-events-none text-neutral-200 text-xs font-bold flex items-center gap-1 cursor-pointer"
                >
                  <span>Cap. {selectedChapter + 1}</span>
                  <ChevronRight size={14} />
                </button>
              </div>

              <!-- Lista de Versículos -->
              <div class="space-y-1.5 pb-2">
                {#if isLoadingBible}
                  <div class="p-8 text-center text-neutral-500 text-xs">
                    <RefreshCw size={18} class="animate-spin mx-auto mb-1" />
                    <span>Carregando versículos...</span>
                  </div>
                {:else}
                  {#each verses as v, vIdx}
                    {@const isChapterLive =
                      !!selectedBook &&
                      ($presentationState.active_media_id ===
                        `scripture-${activeBibleId}-${selectedBook.id}-${selectedChapter}` ||
                        $presentationState.active_media_id ===
                          `scripture-${selectedBook.id}-${selectedChapter}` ||
                        $presentationState.active_media_id ===
                          `scripture-${selectedBook.id}-${selectedChapter}-${v.verse}`) &&
                      isLive}
                    {@const isVerseLive =
                      isChapterLive &&
                      (currentSlideIndex === vIdx ||
                        (!!selectedBook &&
                          $presentationState.active_media_id ===
                            `scripture-${selectedBook.id}-${selectedChapter}-${v.verse}`))}
                    <button
                      type="button"
                      onclick={() => presentVerse(v)}
                      class="w-full text-left p-3 rounded-xl border transition-all flex items-start gap-3 cursor-pointer active:scale-[0.99] {isVerseLive ? 'bg-emerald-950/60 border-emerald-500 ring-1 ring-emerald-500 text-white shadow-lg' : 'bg-neutral-900/70 hover:bg-neutral-800 active:bg-neutral-700 border-neutral-800/80'}"
                    >
                      <div class="flex flex-col items-center gap-1 shrink-0 mt-0.5">
                        <span
                          class="w-6 h-6 rounded-full font-bold text-xs flex items-center justify-center {isVerseLive ? 'bg-emerald-500 text-black' : 'bg-neutral-800 text-emerald-400'}"
                        >
                          {v.verse}
                        </span>
                        {#if isVerseLive}
                          <span class="text-[8px] font-bold text-emerald-400 uppercase tracking-tighter animate-pulse">
                            Ao Vivo
                          </span>
                        {/if}
                      </div>

                      <p class="text-xs text-neutral-200 leading-relaxed flex-1 {isVerseLive ? 'font-semibold text-white' : ''}">
                        {v.text}
                      </p>
                    </button>
                  {/each}
                {/if}
              </div>
            </div>
          {/if}
        {/if}
      </div>
    {/if}
  </main>

  <!-- BOTTOM NAVIGATION BAR (Thumb-friendly, 3 abas principais com Safe-Area para iOS) -->
  <nav
    class="bg-neutral-900 border-t border-neutral-800 grid grid-cols-3 shrink-0"
    style="padding-bottom: max(env(safe-area-inset-bottom), 16px);"
  >
    <button
      type="button"
      onclick={() => (activeTab = 'setlist')}
      class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab === 'setlist' ? 'text-emerald-400 font-extrabold' : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <ListMusic size={20} class={activeTab === 'setlist' ? 'text-emerald-400' : 'text-neutral-400'} />
      <span class="text-[11px]">Setlist</span>
    </button>

    <button
      type="button"
      onclick={() => (activeTab = 'songs')}
      class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab === 'songs' ? 'text-emerald-400 font-extrabold' : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <Music size={20} class={activeTab === 'songs' ? 'text-emerald-400' : 'text-neutral-400'} />
      <span class="text-[11px]">Músicas</span>
    </button>

    <button
      type="button"
      onclick={() => (activeTab = 'bible')}
      class="h-14 flex flex-col items-center justify-center gap-1 transition-all active:scale-95 cursor-pointer {activeTab === 'bible' ? 'text-emerald-400 font-extrabold' : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <BookOpen size={20} class={activeTab === 'bible' ? 'text-emerald-400' : 'text-neutral-400'} />
      <span class="text-[11px]">Bíblia</span>
    </button>
  </nav>

  <!-- DRAWER SHADCN: Seleção de Tradução Bíblica -->
  {#if isBibleDrawerOpen}
    <!-- Backdrop com blur -->
    <button
      type="button"
      class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm transition-opacity w-full h-full border-0 p-0 m-0 cursor-pointer"
      onclick={() => (isBibleDrawerOpen = false)}
      aria-label="Fechar Gaveta"
    ></button>

    <!-- Superfície do Drawer deslizando da base -->
    <div
      class="fixed inset-x-0 bottom-0 z-50 max-w-md mx-auto rounded-t-3xl bg-neutral-900 border-t border-neutral-800 p-4 shadow-2xl flex flex-col max-h-[80vh] font-sans"
      style="padding-bottom: max(env(safe-area-inset-bottom), 24px);"
    >
      <!-- Drag Handle -->
      <div class="w-12 h-1.5 rounded-full bg-neutral-700 mx-auto mb-3 shrink-0"></div>

      <!-- Cabeçalho do Drawer -->
      <div class="flex items-center justify-between pb-3 border-b border-neutral-800 shrink-0">
        <div>
          <h3 class="text-sm font-bold text-white flex items-center gap-2">
            <BookOpen size={16} class="text-emerald-400" />
            <span>Tradução da Bíblia</span>
          </h3>
          <p class="text-[11px] text-neutral-400">Selecione a versão para projeção e leitura</p>
        </div>
        <button
          type="button"
          onclick={() => (isBibleDrawerOpen = false)}
          class="p-1.5 rounded-full bg-neutral-800 text-neutral-400 hover:text-white cursor-pointer"
        >
          <X size={15} />
        </button>
      </div>

      <!-- Lista de Traduções Disponíveis -->
      <div class="space-y-1.5 py-3 overflow-y-auto min-h-0 flex-1">
        {#each bibles as b}
          {@const isSelected = b.id === activeBibleId}
          <button
            type="button"
            onclick={() => {
              activeBibleId = b.id;
              if (selectedBook) {
                loadVerses(b.id, selectedBook.id, selectedChapter);
              }
              isBibleDrawerOpen = false;
            }}
            class="w-full p-3 rounded-xl border flex items-center justify-between text-left transition-all cursor-pointer {isSelected ? 'bg-emerald-950/40 border-emerald-500 text-white' : 'bg-neutral-850/80 border-neutral-800 text-neutral-300 hover:bg-neutral-800'}"
          >
            <div>
              <div class="flex items-center gap-2">
                <span
                  class="px-2 py-0.5 rounded text-xs font-extrabold font-mono {isSelected ? 'bg-emerald-500 text-black' : 'bg-neutral-800 text-neutral-300'}"
                >
                  {b.abbreviation || b.name}
                </span>
                <span class="text-xs font-bold text-white">{b.name}</span>
              </div>
              <span class="text-[10px] text-neutral-400 mt-0.5 block">{b.book_count || 66} livros</span>
            </div>
            {#if isSelected}
              <Check size={16} class="text-emerald-400 shrink-0" />
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- DRAWER SHADCN: Seleção de Setlist -->
  {#if isSetlistDrawerOpen}
    <button
      type="button"
      class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm transition-opacity w-full h-full border-0 p-0 m-0 cursor-pointer"
      onclick={() => (isSetlistDrawerOpen = false)}
      aria-label="Fechar Gaveta"
    ></button>

    <div
      class="fixed inset-x-0 bottom-0 z-50 max-w-md mx-auto rounded-t-3xl bg-neutral-900 border-t border-neutral-800 p-4 shadow-2xl flex flex-col max-h-[80vh] font-sans"
      style="padding-bottom: max(env(safe-area-inset-bottom), 24px);"
    >
      <div class="w-12 h-1.5 rounded-full bg-neutral-700 mx-auto mb-3 shrink-0"></div>

      <div class="flex items-center justify-between pb-3 border-b border-neutral-800 shrink-0">
        <div>
          <h3 class="text-sm font-bold text-white flex items-center gap-2">
            <Layers size={16} class="text-emerald-400" />
            <span>Alternar Setlist</span>
          </h3>
          <p class="text-[11px] text-neutral-400">Selecione o culto ou reunião a ser exibido</p>
        </div>
        <button
          type="button"
          onclick={() => (isSetlistDrawerOpen = false)}
          class="p-1.5 rounded-full bg-neutral-800 text-neutral-400 hover:text-white cursor-pointer"
        >
          <X size={15} />
        </button>
      </div>

      <div class="space-y-1.5 py-3 overflow-y-auto min-h-0 flex-1">
        {#each availableSetlists as s}
          {@const isSelected = activeSetlistData?.id === s.id}
          <button
            type="button"
            onclick={() => {
              loadSingleSetlistDetail(s.id);
              isSetlistDrawerOpen = false;
            }}
            class="w-full p-3 rounded-xl border flex items-center justify-between text-left transition-all cursor-pointer {isSelected ? 'bg-emerald-950/40 border-emerald-500 text-white' : 'bg-neutral-850/80 border-neutral-800 text-neutral-300 hover:bg-neutral-800'}"
          >
            <div>
              <p class="text-xs font-bold text-white">{s.title}</p>
              <p class="text-[10px] text-neutral-400 mt-0.5">{s.item_count} itens cadastrados</p>
            </div>
            {#if isSelected}
              <Check size={16} class="text-emerald-400 shrink-0" />
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

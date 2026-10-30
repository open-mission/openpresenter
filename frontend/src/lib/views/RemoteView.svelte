<script lang="ts">
  import { onMount } from 'svelte';
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
    Square,
    EyeOff,
    Search,
    Radio,
    Play,
    RefreshCw,
    X,
  } from 'lucide-svelte';

  // Navegação mobile entre as 3 abas principais
  type RemoteTab = 'setlist' | 'songs' | 'bible';
  let activeTab = $state<RemoteTab>('setlist');

  // Estado da Setlist
  let activeSetlistData = $state<Setlist | null>(null);
  let isLoadingSetlist = $state(false);
  let expandedSetlistItemId = $state<string | null>(null);

  // Estado das Músicas
  let searchQuery = $state('');
  let songsList = $state<SongSummary[]>([]);
  let isLoadingSongs = $state(false);
  let selectedSong = $state<Song | null>(null);
  let isLoadingSongDetail = $state(false);
  let songSearchTimeout: ReturnType<typeof setTimeout> | null = null;

  // Estado da Bíblia
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

  // Derivados do estado de apresentação
  const isLive = $derived(
    $presentationState.display_mode === 'live' && !!$presentationState.active_media_id
  );
  const isBlackout = $derived($presentationState.display_mode === 'blackout');
  const isClear = $derived($presentationState.display_mode === 'cleartext');

  // Slide atual ao vivo
  const currentMediaTitle = $derived.by(() => {
    if (!$presentationState.active_media_id) return 'Nenhuma apresentação ao vivo';
    if ($presentationState.quick_media) {
      return $presentationState.quick_media.title;
    }
    const item = activeSetlistData?.items.find((i) => i.id === $presentationState.active_media_id);
    return item?.title || 'Apresentação ativa';
  });

  const currentMediaSlides = $derived.by(() => {
    if ($presentationState.quick_media) {
      return $presentationState.quick_media.slides || [];
    }
    const item = activeSetlistData?.items.find((i) => i.id === $presentationState.active_media_id);
    return item?.slides || [];
  });

  const currentSlideIndex = $derived($presentationState.active_slide_index ?? 0);
  const totalSlides = $derived(currentMediaSlides.length);
  const currentSlide = $derived(
    totalSlides > 0 && currentSlideIndex >= 0 && currentSlideIndex < totalSlides
      ? currentMediaSlides[currentSlideIndex]
      : null
  );

  onMount(() => {
    initWebSocket('remote');
    loadActiveSetlist();
    loadSongs();
    loadBibles();
  });

  // Reagir a mudanças no active_setlist_id do WebSocket
  $effect(() => {
    if ($presentationState.active_setlist_id) {
      loadActiveSetlist();
    }
  });

  // Carregar a Setlist ativa
  async function loadActiveSetlist() {
    isLoadingSetlist = true;
    try {
      const res = await fetch('/api/setlists');
      if (res.ok) {
        const setlists: Setlist[] = await res.json();
        const active =
          setlists.find((s) => s.id === $presentationState.active_setlist_id || s.is_active) ||
          setlists[0] ||
          null;
        activeSetlistData = active;
        if (active && !expandedSetlistItemId && $presentationState.active_media_id) {
          expandedSetlistItemId = $presentationState.active_media_id;
        }
      }
    } catch (err) {
      console.error('Erro ao carregar setlist:', err);
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
    if (!query.trim() || !activeBibleId) {
      bibleSearchResults = [];
      isSearchingBible = false;
      return;
    }
    isSearchingBible = true;
    try {
      const res = await fetch(
        `/api/bibles/search?q=${encodeURIComponent(query.trim())}&bible_id=${activeBibleId}&exact=false`
      );
      if (res.ok) {
        bibleSearchResults = await res.json();
      }
    } catch (err) {
      console.error('Erro ao pesquisar texto bíblico:', err);
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

  // Apresentar Versículo Bíblico diretamente
  function presentVerse(verse: BibleVerse) {
    if (!selectedBook) return;
    const bookName = selectedBook.name;
    const mediaItem: MediaItem = {
      id: `scripture-${selectedBook.id}-${selectedChapter}-${verse.verse}`,
      kind: 'scripture',
      title: `${bookName} ${selectedChapter}:${verse.verse}`,
      artist_or_ref: activeBibleId.toUpperCase(),
      slides: [
        {
          id: `scripture-${selectedBook.id}-${selectedChapter}-${verse.verse}-0`,
          label: `${bookName} ${selectedChapter}:${verse.verse}`,
          content: verse.text,
        },
      ],
    };
    quickPresentMedia(mediaItem, 0);
  }

  // Apresentar resultado da busca bíblica
  function presentSearchResult(hit: any) {
    const mediaItem: MediaItem = {
      id: `scripture-${hit.book_id}-${hit.chapter}-${hit.verse}`,
      kind: 'scripture',
      title: `${hit.book_name} ${hit.chapter}:${hit.verse}`,
      artist_or_ref: (hit.bible_version || activeBibleId).toUpperCase(),
      slides: [
        {
          id: `scripture-${hit.book_id}-${hit.chapter}-${hit.verse}-0`,
          label: `${hit.book_name} ${hit.chapter}:${hit.verse}`,
          content: hit.text,
        },
      ],
    };
    quickPresentMedia(mediaItem, 0);
  }
</script>

<div class="flex flex-col h-screen w-screen max-w-md mx-auto bg-neutral-950 text-neutral-100 select-none overflow-hidden font-sans">
  <!-- TOP BAR: Status e Ações Rápidas de Emergência -->
  <header class="flex items-center justify-between px-3 py-2 bg-neutral-900 border-b border-neutral-800 shrink-0">
    <div class="flex items-center gap-2">
      <div class="w-7 h-7 rounded-lg bg-emerald-600/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400">
        <Radio size={16} />
      </div>
      <div>
        <h1 class="text-xs font-bold leading-tight flex items-center gap-1.5">
          OpenPresenter
          <span class="inline-flex items-center gap-1 px-1.5 py-0.2 rounded-full text-[9px] font-semibold {$isWsConnected ? 'bg-emerald-950 text-emerald-400 border border-emerald-800' : 'bg-red-950 text-red-400 border border-red-800'}">
            <span class="w-1.5 h-1.5 rounded-full {$isWsConnected ? 'bg-emerald-400 animate-pulse' : 'bg-red-400'}"></span>
            {$isWsConnected ? 'Remoto' : 'Offline'}
          </span>
        </h1>
        <p class="text-[10px] text-neutral-400">Controle na Palma da Mão</p>
      </div>
    </div>

    <!-- Botões de Ação de Projeção -->
    <div class="flex items-center gap-1">
      <button
        type="button"
        onclick={handleToggleBlackout}
        class="px-2 py-1 text-[10px] font-bold rounded flex items-center gap-1 transition-colors {isBlackout ? 'bg-red-600 text-white shadow-lg shadow-red-950' : 'bg-neutral-800 hover:bg-neutral-700 text-neutral-300'}"
        title="Blackout"
      >
        <span class="w-2 h-2 rounded-full bg-red-500"></span>
        Preto
      </button>

      <button
        type="button"
        onclick={handleToggleClear}
        class="px-2 py-1 text-[10px] font-bold rounded flex items-center gap-1 transition-colors {isClear ? 'bg-amber-600 text-white' : 'bg-neutral-800 hover:bg-neutral-700 text-neutral-300'}"
        title="Limpar Texto"
      >
        <EyeOff size={11} />
        Limpar
      </button>

      <button
        type="button"
        onclick={handleStop}
        class="p-1 text-neutral-400 hover:text-white bg-neutral-800 hover:bg-neutral-700 rounded transition-colors cursor-pointer"
        title="Parar Apresentação"
        aria-label="Parar Apresentação"
      >
        <Square size={13} />
      </button>
    </div>
  </header>

  <!-- LIVE CONTROL CARD: Fixado no topo, com botões ergonômicos de Anterior / Próximo -->
  <section class="p-3 bg-neutral-900/60 border-b border-neutral-800 shrink-0">
    <div class="flex items-center justify-between text-[11px] mb-1">
      <div class="flex items-center gap-1 font-semibold truncate text-emerald-400">
        <span class="w-2 h-2 rounded-full {isLive ? 'bg-emerald-400 animate-ping' : 'bg-neutral-600'}"></span>
        <span class="truncate">{currentMediaTitle}</span>
      </div>
      {#if totalSlides > 0}
        <span class="text-[10px] font-mono text-neutral-400 shrink-0">
          Slide {currentSlideIndex + 1}/{totalSlides}
        </span>
      {/if}
    </div>

    <!-- Prévia do Slide Ativo -->
    <div class="p-2.5 rounded-lg bg-black/60 border border-neutral-800/80 mb-2 min-h-[50px] flex items-center justify-center text-center">
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

    <!-- Botões gigantes para o polegar -->
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
  </section>

  <!-- CONTEÚDO PRINCIPAL: Scrollável conforme a aba selecionada -->
  <main class="flex-1 overflow-y-auto overflow-x-hidden min-h-0 bg-neutral-950 p-2">
    <!-- ABA 1: SETLIST -->
    {#if activeTab === 'setlist'}
      <div class="space-y-2">
        <div class="flex items-center justify-between px-1 text-xs text-neutral-400 font-medium">
          <span>Ordem do Culto ({activeSetlistData?.title || 'Setlist Ativa'})</span>
          <button
            type="button"
            onclick={loadActiveSetlist}
            class="p-1 hover:text-white transition-colors cursor-pointer"
            title="Atualizar"
            aria-label="Atualizar Setlist"
          >
            <RefreshCw size={13} class={isLoadingSetlist ? 'animate-spin' : ''} />
          </button>
        </div>

        {#if !activeSetlistData || activeSetlistData.items.length === 0}
          <div class="p-8 text-center text-neutral-500 text-xs">
            <ListMusic size={32} class="mx-auto mb-2 opacity-40" />
            <p>Nenhum item na setlist ativa.</p>
            <p class="text-[10px] mt-1 text-neutral-600">Adicione músicas ou passagens pelo painel do operador.</p>
          </div>
        {:else}
          <div class="space-y-1.5">
            {#each activeSetlistData.items as item, itemIdx}
              {@const isItemActive = $presentationState.active_media_id === item.id}
              {@const isExpanded = expandedSetlistItemId === item.id}
              <div
                class="rounded-xl border transition-all overflow-hidden {isItemActive ? 'bg-neutral-900 border-emerald-500/80 shadow-md' : 'bg-neutral-900/60 border-neutral-800/80'}"
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
                    <span class="w-5 h-5 rounded-full text-[10px] font-bold flex items-center justify-center shrink-0 {isItemActive ? 'bg-emerald-500 text-black' : 'bg-neutral-800 text-neutral-400'}">
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
                      <span class="px-1.5 py-0.5 rounded text-[9px] font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40">
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
                  <div class="px-2 pb-2 pt-1 border-t border-neutral-800/60 space-y-1 bg-black/30">
                    {#each item.slides as slide, sIdx}
                      {@const isCurrentSlideLive = isItemActive && currentSlideIndex === sIdx && isLive}
                      <button
                        type="button"
                        onclick={() => handleSelectSlide(item.id, sIdx)}
                        class="w-full text-left p-2 rounded-lg text-[11px] flex items-start gap-2 transition-all cursor-pointer {isCurrentSlideLive ? 'bg-emerald-600/30 border border-emerald-500 text-white font-semibold' : 'bg-neutral-900/80 hover:bg-neutral-800 text-neutral-300 border border-neutral-800/40'}"
                      >
                        <span class="text-[9px] px-1 rounded font-mono shrink-0 mt-0.5 {isCurrentSlideLive ? 'bg-emerald-500 text-black font-bold' : 'bg-neutral-800 text-neutral-400'}">
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
        <!-- Campo de busca de músicas -->
        <div class="relative">
          <Search size={14} class="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400 pointer-events-none" />
          <input
            type="text"
            value={searchQuery}
            oninput={handleSongSearchInput}
            placeholder="Buscar por título ou trecho da letra..."
            class="w-full pl-8 pr-8 py-2 rounded-xl bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder:text-neutral-500 focus:outline-none focus:border-emerald-500 transition-colors"
          />
          {#if searchQuery}
            <button
              type="button"
              onclick={() => {
                searchQuery = '';
                loadSongs('');
              }}
              class="absolute right-2.5 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-white p-0.5 cursor-pointer"
            >
              <X size={12} />
            </button>
          {/if}
        </div>

        <!-- Visualização detalhada da música selecionada -->
        {#if isLoadingSongDetail}
          <div class="p-8 text-center text-neutral-500 text-xs">
            <RefreshCw size={20} class="animate-spin mx-auto mb-2" />
            <p>Carregando letra...</p>
          </div>
        {:else if selectedSong}
          {@const songSlides = songToSlides(selectedSong)}
          <div class="p-3 rounded-xl bg-neutral-900 border border-neutral-800 space-y-2.5 animate-in fade-in duration-150">
            <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
              <div>
                <h3 class="text-sm font-bold text-neutral-100">{selectedSong.title}</h3>
                {#if selectedSong.artist}
                  <p class="text-xs text-neutral-400">{selectedSong.artist}</p>
                {/if}
              </div>
              <button
                type="button"
                onclick={() => (selectedSong = null)}
                class="p-1 rounded-lg bg-neutral-800 text-neutral-400 hover:text-white cursor-pointer"
              >
                <X size={14} />
              </button>
            </div>

            <button
              type="button"
              onclick={() => presentSong(selectedSong!)}
              class="w-full py-2.5 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-xs flex items-center justify-center gap-1.5 shadow-lg shadow-emerald-950 transition-colors cursor-pointer"
            >
              <Play size={14} />
              <span>Apresentar Música Completa</span>
            </button>

            <!-- Slides / Estrofes da música -->
            <div class="space-y-1 pt-1">
              <span class="text-[10px] font-semibold text-neutral-400 uppercase tracking-wider">
                Toque em uma estrofe para projetar:
              </span>
              {#each songSlides as slide, sIdx}
                <button
                  type="button"
                  onclick={() => presentSong(selectedSong!, sIdx)}
                  class="w-full text-left p-2.5 rounded-lg bg-black/40 hover:bg-neutral-800 text-neutral-200 text-xs border border-neutral-800/80 transition-colors cursor-pointer"
                >
                  <span class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-400 mr-1.5">
                    {slide.label}
                  </span>
                  <p class="mt-1 leading-relaxed text-neutral-300">
                    {slide.content}
                  </p>
                </button>
              {/each}
            </div>
          </div>
        {:else}
          <!-- Lista de músicas filtradas -->
          <div class="space-y-1">
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
                <button
                  type="button"
                  onclick={() => openSongDetail(song)}
                  class="w-full px-3 py-2.5 rounded-xl bg-neutral-900/80 hover:bg-neutral-850 active:bg-neutral-800 border border-neutral-800/60 flex items-center justify-between text-left transition-colors cursor-pointer"
                >
                  <div class="min-w-0 pr-2">
                    <p class="text-xs font-bold text-neutral-200 truncate">{song.title}</p>
                    {#if song.artist}
                      <p class="text-[10px] text-neutral-400 truncate">{song.artist}</p>
                    {/if}
                  </div>
                  <span class="text-[10px] text-neutral-500 font-mono shrink-0">
                    Toque para abrir
                  </span>
                </button>
              {/each}
            {/if}
          </div>
        {/if}
      </div>

    <!-- ABA 3: BÍBLIA -->
    {:else if activeTab === 'bible'}
      <div class="space-y-2">
        <!-- Barra de tradução bíblica e pesquisa rápida -->
        <div class="flex items-center gap-2">
          {#if bibles.length > 0}
            <select
              bind:value={activeBibleId}
              onchange={() => {
                if (selectedBook) {
                  loadVerses(activeBibleId, selectedBook.id, selectedChapter);
                }
              }}
              class="px-2 py-1.5 rounded-xl bg-neutral-900 border border-neutral-800 text-xs text-neutral-200 font-bold focus:outline-none focus:border-emerald-500"
            >
              {#each bibles as b}
                <option value={b.id}>{b.abbreviation || b.name}</option>
              {/each}
            </select>
          {/if}

          <div class="relative flex-1">
            <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-neutral-400" />
            <input
              type="text"
              bind:value={bibleQuery}
              onkeydown={(e) => {
                if (e.key === 'Enter') searchBible(bibleQuery);
              }}
              placeholder="Ex: Jo 3:16 ou amor..."
              class="w-full pl-7 pr-7 py-1.5 rounded-xl bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder:text-neutral-500 focus:outline-none focus:border-emerald-500"
            />
            {#if bibleQuery}
              <button
                type="button"
                onclick={() => {
                  bibleQuery = '';
                  bibleSearchResults = [];
                }}
                class="absolute right-2 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-white p-0.5 cursor-pointer"
              >
                <X size={12} />
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
                class="w-full text-left p-2 rounded-lg bg-black/40 hover:bg-neutral-800 active:bg-neutral-700 text-xs border border-neutral-800/60 transition-colors cursor-pointer"
              >
                <div class="flex items-center justify-between font-bold text-emerald-400 text-[11px] mb-0.5">
                  <span>{hit.book_name} {hit.chapter}:{hit.verse}</span>
                  <span class="text-[9px] uppercase font-mono text-neutral-500">{hit.bible_version || activeBibleId}</span>
                </div>
                <p class="text-neutral-300 leading-snug line-clamp-2">{hit.text}</p>
              </button>
            {/each}
          </div>
        {:else}
          <!-- Navegador de Livros e Capítulos -->
          <div class="space-y-2">
            <!-- Seletor de Livro (Scroll horizontal) -->
            <div>
              <span class="text-[10px] font-bold uppercase text-neutral-400 px-1 block mb-1">
                Livro:
              </span>
              <div class="flex gap-1.5 overflow-x-auto pb-1.5 scrollbar-thin">
                {#each books as book}
                  {@const isSelected = selectedBook?.id === book.id}
                  <button
                    type="button"
                    onclick={() => selectBook(book)}
                    class="px-2.5 py-1 rounded-lg text-xs font-semibold shrink-0 transition-colors cursor-pointer {isSelected ? 'bg-emerald-600 text-white shadow' : 'bg-neutral-900 text-neutral-300 border border-neutral-800 hover:bg-neutral-850'}"
                  >
                    {book.name}
                  </button>
                {/each}
              </div>
            </div>

            <!-- Seletor de Capítulo (Pílulas horizontais) -->
            {#if selectedBook}
              <div>
                <span class="text-[10px] font-bold uppercase text-neutral-400 px-1 block mb-1">
                  Capítulo ({selectedBook.name}):
                </span>
                <div class="flex gap-1 overflow-x-auto pb-1.5 scrollbar-thin">
                  {#each Array.from({ length: selectedBook.chapter_count || 1 }, (_, i) => i + 1) as ch}
                    {@const isSelected = selectedChapter === ch}
                    <button
                      type="button"
                      onclick={() => selectChapter(ch)}
                      class="w-7 h-7 rounded-lg text-xs font-bold shrink-0 flex items-center justify-center transition-colors cursor-pointer {isSelected ? 'bg-emerald-600 text-white' : 'bg-neutral-900 text-neutral-400 border border-neutral-800 hover:bg-neutral-850'}"
                    >
                      {ch}
                    </button>
                  {/each}
                </div>
              </div>

              <!-- Lista de Versículos do Capítulo -->
              <div class="space-y-1 pt-1">
                <span class="text-[10px] font-bold uppercase text-neutral-400 px-1 block">
                  Versículos (Toque para projetar ao vivo):
                </span>
                {#if isLoadingBible}
                  <div class="p-6 text-center text-neutral-500 text-xs">
                    <RefreshCw size={18} class="animate-spin mx-auto mb-1" />
                    <span>Carregando versículos...</span>
                  </div>
                {:else}
                  {#each verses as v}
                    <button
                      type="button"
                      onclick={() => presentVerse(v)}
                      class="w-full text-left p-2.5 rounded-xl bg-neutral-900/70 hover:bg-neutral-800 active:bg-neutral-700 border border-neutral-800/80 transition-colors flex items-start gap-2 cursor-pointer"
                    >
                      <span class="w-5 h-5 rounded-full bg-neutral-800 text-emerald-400 font-bold text-[10px] flex items-center justify-center shrink-0 mt-0.5">
                        {v.verse}
                      </span>
                      <p class="text-xs text-neutral-200 leading-relaxed flex-1">
                        {v.text}
                      </p>
                    </button>
                  {/each}
                {/if}
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  </main>

  <!-- BOTTOM NAVIGATION BAR (Thumb-friendly, 3 abas principais) -->
  <nav class="h-14 bg-neutral-900 border-t border-neutral-800 grid grid-cols-3 shrink-0">
    <button
      type="button"
      onclick={() => (activeTab = 'setlist')}
      class="flex flex-col items-center justify-center gap-1 transition-colors cursor-pointer {activeTab === 'setlist' ? 'text-emerald-400 font-bold' : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <ListMusic size={18} />
      <span class="text-[10px]">Setlist</span>
    </button>

    <button
      type="button"
      onclick={() => (activeTab = 'songs')}
      class="flex flex-col items-center justify-center gap-1 transition-colors cursor-pointer {activeTab === 'songs' ? 'text-emerald-400 font-bold' : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <Music size={18} />
      <span class="text-[10px]">Músicas</span>
    </button>

    <button
      type="button"
      onclick={() => (activeTab = 'bible')}
      class="flex flex-col items-center justify-center gap-1 transition-colors cursor-pointer {activeTab === 'bible' ? 'text-emerald-400 font-bold' : 'text-neutral-400 hover:text-neutral-200'}"
    >
      <BookOpen size={18} />
      <span class="text-[10px]">Bíblia</span>
    </button>
  </nav>
</div>

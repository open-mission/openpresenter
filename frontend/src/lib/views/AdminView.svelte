<script lang="ts">
  import { onMount } from 'svelte';
  import { cn } from '$lib/utils';
  import {
    presentationState,
    mediaItemsList,
    isWsConnected,
    initWebSocket,
    selectSlide,
    setDisplayMode,
    toggleBlackout,
    toggleClearText,
  } from '$lib/ws';
  import type { MediaItem, MediaKind, Slide, Song } from '$lib/types';
  import Button from '$lib/components/ui/Button.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import MediaItemRow from '$lib/components/ui/MediaItemRow.svelte';
  import CommandPalette from '$lib/components/CommandPalette.svelte';
  import SongEditorModal from '$lib/components/SongEditorModal.svelte';
  import SettingsModal from '$lib/components/SettingsModal.svelte';
  import {
    themeStore,
    sectionsStore,
    applyTheme,
    getSectionStyle,
  } from '$lib/settings';
  import {
    Search,
    MonitorPlay,
    Tv,
    EyeOff,
    Slash,
    ExternalLink,
    ChevronLeft,
    ChevronRight,
    Wifi,
    WifiOff,
    Radio,
    Clock,
    Layers,
    Plus,
    Play,
    Square,
    Settings,
  } from 'lucide-svelte';

  let selectedMediaId = $state<string>('song-1');
  let searchQuery = $state<string>('');
  let filterKind = $state<MediaKind | 'all'>('all');
  let stageClock = $state<string>('');
  let isCommandPaletteOpen = $state(false);
  let isSongEditorOpen = $state(false);
  let isSettingsOpen = $state(false);
  let editingSong = $state<Song | null>(null);
  let focusedSlideIndex = $state<number>(0);
  let numberBuffer = $state<string>('');
  let numberBufferTimer: ReturnType<typeof setTimeout> | null = null;
  let prevMediaId = $state<string>('');

  onMount(() => {
    initWebSocket('admin');
    applyTheme($themeStore);

    // Inicializa seleção padrão com a primeira mídia
    if ($mediaItemsList.length > 0 && !selectedMediaId) {
      selectedMediaId = $mediaItemsList[0].id;
    }

    const interval = setInterval(() => {
      const now = new Date();
      stageClock = now.toLocaleTimeString('pt-BR', { hour12: false });
    }, 1000);

    return () => clearInterval(interval);
  });

  // Filtro de mídias por busca e tipo
  const filteredMedia = $derived.by(() => {
    return $mediaItemsList.filter((item) => {
      const matchesSearch =
        searchQuery === '' ||
        item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (item.artist_or_ref && item.artist_or_ref.toLowerCase().includes(searchQuery.toLowerCase()));

      const matchesKind = filterKind === 'all' || item.kind === filterKind;
      return matchesSearch && matchesKind;
    });
  });

  // Mídia selecionada para exibição no painel central
  const selectedMedia = $derived.by<MediaItem | undefined>(() => {
    return $mediaItemsList.find((m) => m.id === selectedMediaId) ?? $mediaItemsList[0];
  });

  // Mídia ativa ao vivo
  const liveMedia = $derived.by<MediaItem | undefined>(() => {
    const p = $presentationState;
    if (!p.active_media_id) return undefined;
    return $mediaItemsList.find((m) => m.id === p.active_media_id);
  });

  // Slide ativo ao vivo
  const liveSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!liveMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    return liveMedia.slides[p.active_slide_index];
  });

  // Próximo slide para o preview do stage
  const nextLiveSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!liveMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    const nextIdx = p.active_slide_index + 1;
    if (nextIdx < liveMedia.slides.length) {
      return liveMedia.slides[nextIdx];
    }
    return undefined;
  });

  // Verifica se a mídia selecionada está ao vivo atualmente
  const isSelectedMediaLive = $derived.by(() => {
    return (
      $presentationState.active_media_id === selectedMedia?.id &&
      $presentationState.active_slide_index !== null &&
      $presentationState.active_slide_index !== undefined &&
      $presentationState.display_mode === 'live'
    );
  });

  // Sincroniza o slide focado quando a mídia selecionada muda
  $effect(() => {
    if (selectedMediaId !== prevMediaId) {
      prevMediaId = selectedMediaId;
      if (
        $presentationState.active_media_id === selectedMediaId &&
        $presentationState.active_slide_index !== null &&
        $presentationState.active_slide_index !== undefined
      ) {
        focusedSlideIndex = $presentationState.active_slide_index;
      } else {
        focusedSlideIndex = 0;
      }
    }
  });

  // Disparo e controle de apresentação
  function selectSlideForFocus(idx: number) {
    focusedSlideIndex = idx;
  }

  function presentSlide(idx: number = focusedSlideIndex) {
    if (!selectedMedia || selectedMedia.slides.length === 0) return;
    const clamped = Math.max(0, Math.min(idx, selectedMedia.slides.length - 1));
    focusedSlideIndex = clamped;
    selectSlide(selectedMedia.id, clamped);
  }

  function stopPresentation() {
    setDisplayMode('cleartext');
  }

  // Navegação de slides
  function nextSlide() {
    if (!selectedMedia || selectedMedia.slides.length === 0) return;
    if (focusedSlideIndex < selectedMedia.slides.length - 1) {
      focusedSlideIndex++;
      if (isSelectedMediaLive) {
        selectSlide(selectedMedia.id, focusedSlideIndex);
      }
    }
  }

  function prevSlide() {
    if (!selectedMedia || selectedMedia.slides.length === 0) return;
    if (focusedSlideIndex > 0) {
      focusedSlideIndex--;
      if (isSelectedMediaLive) {
        selectSlide(selectedMedia.id, focusedSlideIndex);
      }
    }
  }

  // Atalho rápido para seções da música (r = refrao, v = verso, p = ponte, i = intro, f = final, etc.)
  function jumpToSection(key: string): boolean {
    if (!selectedMedia || selectedMedia.slides.length === 0) return false;
    const k = key.toLowerCase();

    const matchingConfig = $sectionsStore.find(
      (s) => s.shortcut && s.shortcut.toLowerCase() === k
    );

    const searchTerms: string[] = [];
    if (matchingConfig) {
      searchTerms.push(matchingConfig.name.toLowerCase(), matchingConfig.id.toLowerCase());
    }
    if (k === 'v') searchTerms.push('verso', 'verse', 'estrofe');
    if (k === 'r') searchTerms.push('refrão', 'refrao', 'chorus', 'coro');
    if (k === 'p') searchTerms.push('ponte', 'bridge');
    if (k === 'i') searchTerms.push('intro', 'início');
    if (k === 'f') searchTerms.push('final', 'outro', 'fim');

    if (searchTerms.length === 0) return false;

    const matchingIndices: number[] = [];
    selectedMedia.slides.forEach((slide, idx) => {
      const label = (slide.label || '').toLowerCase();
      if (searchTerms.some((term) => label.includes(term))) {
        matchingIndices.push(idx);
      }
    });

    if (matchingIndices.length === 0) return false;

    // Se já estiver em um deles, pula para o próximo, ou cicla de volta para o primeiro
    const nextIdx = matchingIndices.find((idx) => idx > focusedSlideIndex) ?? matchingIndices[0];
    focusedSlideIndex = nextIdx;
    if (isSelectedMediaLive) {
      selectSlide(selectedMedia.id, nextIdx);
    }
    return true;
  }

  // Buffer numérico para saltar para slide ordinal (ex: digitar 2 + Enter vai para slide 2)
  function appendNumberKey(digit: string) {
    if (numberBufferTimer) clearTimeout(numberBufferTimer);
    numberBuffer += digit;
    numberBufferTimer = setTimeout(() => {
      numberBuffer = '';
    }, 2500);
  }

  function submitNumberBuffer(): boolean {
    if (!numberBuffer) return false;
    const num = parseInt(numberBuffer, 10);
    numberBuffer = '';
    if (numberBufferTimer) {
      clearTimeout(numberBufferTimer);
      numberBufferTimer = null;
    }
    if (!isNaN(num) && selectedMedia && selectedMedia.slides.length > 0) {
      const targetIdx = Math.max(0, Math.min(num - 1, selectedMedia.slides.length - 1));
      presentSlide(targetIdx);
      return true;
    }
    return false;
  }

  // Atalhos de teclado globais
  function handleKeydown(e: KeyboardEvent) {
    // Atalho global do Command Palette (⌘K / Ctrl+K)
    if ((e.metaKey || e.ctrlKey) && (e.key === 'k' || e.key === 'K')) {
      e.preventDefault();
      isCommandPaletteOpen = !isCommandPaletteOpen;
      return;
    }

    // Se estiver digitando em input ou textarea, ignorar
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

    // Se algum modal estiver aberto
    if (isCommandPaletteOpen || isSongEditorOpen || isSettingsOpen) {
      if (e.key === 'Escape') {
        isCommandPaletteOpen = false;
        isSongEditorOpen = false;
        isSettingsOpen = false;
      }
      return;
    }

    // Tecla Escape: Para a apresentação
    if (e.key === 'Escape') {
      e.preventDefault();
      if (numberBuffer) {
        numberBuffer = '';
        if (numberBufferTimer) clearTimeout(numberBufferTimer);
        return;
      }
      stopPresentation();
      return;
    }

    // Tecla Enter: Submete buffer numérico ou apresenta o slide focado
    if (e.key === 'Enter') {
      e.preventDefault();
      if (numberBuffer) {
        submitNumberBuffer();
      } else {
        presentSlide(focusedSlideIndex);
      }
      return;
    }

    // Teclas 0-9 para buffer de número
    if (/^[0-9]$/.test(e.key)) {
      e.preventDefault();
      appendNumberKey(e.key);
      return;
    }

    // Teclas de letras para salto de seção (sem modificadores)
    if (!e.metaKey && !e.ctrlKey && !e.altKey && /^[a-zA-Z]$/.test(e.key)) {
      if (jumpToSection(e.key)) {
        e.preventDefault();
        return;
      }
    }

    // F1: Blackout, F2: Clear text
    if (e.key === 'F1') {
      e.preventDefault();
      toggleBlackout();
    } else if (e.key === 'F2') {
      e.preventDefault();
      toggleClearText();
    } else if (e.key === 'ArrowRight' || e.key === 'PageDown' || e.key === ' ') {
      e.preventDefault();
      nextSlide();
    } else if (e.key === 'ArrowLeft' || e.key === 'PageUp') {
      e.preventDefault();
      prevSlide();
    }
  }

  function handleOpenCreate() {
    editingSong = null;
    isSongEditorOpen = true;
  }

  function handleImportSong(song: Song) {
    editingSong = song;
    isSongEditorOpen = true;
  }

  function handleSongSaved(song: Song) {
    selectedMediaId = song.id;
  }

  // Abertura de janelas de saída
  function openPublicWindow() {
    window.open('/public', 'OpenPresenterPublic', 'width=1280,height=720,menubar=no,toolbar=no,location=no');
  }

  function openStageWindow() {
    window.open('/stage', 'OpenPresenterStage', 'width=1280,height=720,menubar=no,toolbar=no,location=no');
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="h-screen w-screen flex flex-col bg-zinc-950 text-zinc-100 overflow-hidden font-sans">
  <!-- Top Bar / Header Operacional -->
  <header class="h-14 border-b border-zinc-800/80 bg-zinc-900/60 px-4 flex items-center justify-between shrink-0">
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2.5">
        <img
          src="/branding/open-presenter-app-icon.svg"
          alt="OpenPresenter"
          class="w-7 h-7 rounded-md shadow-sm"
        />
        <img
          src="/branding/open-presenter-wordmark-dark.svg"
          alt="OpenPresenter"
          class="h-5 opacity-90 hidden sm:block"
        />
        <span class="font-bold text-base tracking-tight text-white sm:hidden">OpenPresenter</span>
      </div>
      <span class="text-zinc-600">|</span>
      <span class="text-xs text-zinc-400 hidden xl:inline">Projeção para igrejas e eventos</span>
    </div>

    <!-- Busca Global e Command Palette (⌘K) -->
    <button
      type="button"
      onclick={() => (isCommandPaletteOpen = true)}
      class="flex items-center gap-2 h-8 px-3 rounded-lg bg-zinc-800/80 hover:bg-zinc-800 border border-zinc-700/60 text-xs text-zinc-400 hover:text-zinc-200 transition-colors cursor-pointer w-44 sm:w-64 md:w-80 justify-between mx-2"
    >
      <span class="flex items-center gap-2 truncate">
        <Search size={14} class="text-zinc-400 shrink-0" />
        <span class="truncate">Buscar música ou comando...</span>
      </span>
      <kbd class="px-1.5 py-0.5 rounded bg-zinc-900 border border-zinc-700/80 text-[10px] font-mono text-zinc-300 shrink-0">
        ⌘K
      </kbd>
    </button>

    <!-- Status e Ações Rápidas -->
    <div class="flex items-center gap-2 sm:gap-2.5">
      <!-- Status WebSocket -->
      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-zinc-800/70 border border-zinc-700/50 text-xs">
        {#if $isWsConnected}
          <Wifi size={13} class="text-emerald-400" />
          <span class="text-emerald-400 font-medium">Online</span>
        {:else}
          <WifiOff size={13} class="text-red-400" />
          <span class="text-red-400 font-medium">Reconectando</span>
        {/if}
      </div>

      <!-- Botão Configurações -->
      <Button
        variant="outline"
        size="sm"
        onclick={() => (isSettingsOpen = true)}
        class="gap-1.5 text-xs"
        title="Configurações (Aparência, Atalhos, Cores)"
      >
        <Settings size={14} class="text-zinc-400" />
        <span class="hidden md:inline">Configurações</span>
      </Button>

      <!-- Botões de Abertura das Saídas -->
      <Button variant="outline" size="sm" onclick={openPublicWindow} class="gap-1.5 text-xs">
        <MonitorPlay size={14} class="text-emerald-400" />
        <span class="hidden md:inline">Iniciar</span> Public
        <ExternalLink size={12} class="opacity-60" />
      </Button>

      <Button variant="outline" size="sm" onclick={openStageWindow} class="gap-1.5 text-xs">
        <Tv size={14} class="text-emerald-400" />
        <span class="hidden md:inline">Iniciar</span> Stage
        <ExternalLink size={12} class="opacity-60" />
      </Button>
    </div>
  </header>

  <!-- Workspace Principal de 3 Colunas -->
  <div class="flex-1 flex overflow-hidden">
    <!-- COLUNA 1: SIDEBAR DE MÍDIAS (Esquerda) -->
    <aside class="w-72 lg:w-80 border-r border-zinc-800/80 bg-zinc-900/30 flex flex-col shrink-0">
      <!-- Cabeçalho da Setlist -->
      <div class="p-3 border-b border-zinc-800/60 space-y-2.5">
        <div class="flex items-center justify-between">
          <span class="text-xs font-semibold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5">
            <Layers size={14} class="text-emerald-400" />
            Setlist
          </span>
          <div class="flex items-center gap-1.5">
            <Badge variant="secondary" class="text-[11px] py-0 px-1.5">
              {$mediaItemsList.length} itens
            </Badge>
            <button
              type="button"
              title="Adicionar música à Setlist (⌘K)"
              onclick={() => (isCommandPaletteOpen = true)}
              class="w-6 h-6 rounded bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 border border-emerald-500/30 flex items-center justify-center transition-colors cursor-pointer"
            >
              <Plus size={13} />
            </button>
          </div>
        </div>

        <!-- Barra de Busca -->
        <div class="relative">
          <Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500" />
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Filtrar setlist (ou ⌘K p/ buscar)..."
            class="w-full h-8 pl-8 pr-3 bg-zinc-800/60 border border-zinc-700/60 rounded-md text-xs text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-emerald-500/60 focus:ring-1 focus:ring-emerald-500/40 transition-colors"
          />
        </div>

        <!-- Filtros por Categoria -->
        <div class="flex items-center gap-1 overflow-x-auto pb-0.5">
          {#each [
            { id: 'all', label: 'Todos' },
            { id: 'song', label: 'Louvores' },
            { id: 'scripture', label: 'Bíblia' },
            { id: 'announcement', label: 'Avisos' },
          ] as tab}
            <button
              type="button"
              onclick={() => (filterKind = tab.id as any)}
              class={cn(
                'px-2 py-1 rounded text-[11px] font-medium transition-colors shrink-0 cursor-pointer',
                filterKind === tab.id
                  ? 'bg-zinc-800 text-emerald-400 font-semibold'
                  : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-850'
              )}
            >
              {tab.label}
            </button>
          {/each}
        </div>
      </div>

      <!-- Lista de Itens de Mídia -->
      <div class="flex-1 overflow-y-auto p-2.5 space-y-1.5">
        {#each filteredMedia as item (item.id)}
          <MediaItemRow
            {item}
            selected={selectedMediaId === item.id}
            isLive={$presentationState.active_media_id === item.id}
            onclick={() => (selectedMediaId = item.id)}
          />
        {:else}
          <div class="p-6 text-center text-xs text-zinc-500">
            Nenhuma mídia encontrada para os filtros atuais.
          </div>
        {/each}
      </div>
    </aside>

    <!-- COLUNA 2: PAINEL CENTRAL DE SLIDES / PREVIEW -->
    <main class="flex-1 flex flex-col min-w-0 bg-zinc-950/60 overflow-hidden">
      {#if selectedMedia}
        <!-- Cabeçalho da Mídia Selecionada com Ações de Apresentação -->
        <div class="h-14 px-5 border-b border-zinc-800/80 bg-zinc-900/40 flex items-center justify-between shrink-0">
          <div class="min-w-0">
            <div class="flex items-center gap-2">
              <h2 class="text-base font-bold text-white truncate leading-tight">
                {selectedMedia.title}
              </h2>
              <Badge variant="default" class="text-[10px] py-0 px-1.5">
                {selectedMedia.kind === 'song' ? 'Louvor' : selectedMedia.kind === 'scripture' ? 'Bíblia' : 'Aviso'}
              </Badge>
              {#if isSelectedMediaLive}
                <Badge variant="live" class="text-[10px] py-0 px-1.5 animate-pulse">
                  NO AR
                </Badge>
              {/if}
            </div>
            {#if selectedMedia.artist_or_ref}
              <p class="text-xs text-zinc-400 truncate mt-0.5">
                {selectedMedia.artist_or_ref}
              </p>
            {/if}
          </div>

          <!-- Ações e Controles de Navegação -->
          <div class="flex items-center gap-2 shrink-0">
            {#if isSelectedMediaLive}
              <Button
                variant="destructive"
                size="sm"
                onclick={stopPresentation}
                title="Parar Projeção (Esc)"
                class="gap-1.5 text-xs font-semibold"
              >
                <Square size={13} class="fill-current" />
                Parar (Esc)
              </Button>
            {/if}

            <Button
              variant="live"
              size="sm"
              onclick={() => presentSlide(focusedSlideIndex)}
              title="Apresentar Slide Focado (Enter)"
              class="gap-1.5 text-xs font-semibold shadow-sm"
            >
              <Play size={13} class="fill-current" />
              Apresentar (Enter)
            </Button>

            <div class="h-4 w-[1px] bg-zinc-700/60 mx-1"></div>

            <Button
              variant="secondary"
              size="sm"
              onclick={prevSlide}
              title="Slide Anterior (←)"
              class="gap-1 text-xs"
            >
              <ChevronLeft size={14} /> Anterior
            </Button>
            <Button
              variant="secondary"
              size="sm"
              onclick={nextSlide}
              title="Próximo Slide (→ ou Espaço)"
              class="gap-1 text-xs"
            >
              Próximo <ChevronRight size={14} />
            </Button>
          </div>
        </div>

        <!-- Grade de Slides (Aspect Ratio 16:9) -->
        <div class="flex-1 overflow-y-auto p-5">
          <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
            {#each selectedMedia.slides as slide, idx (slide.id)}
              {@const isLive =
                $presentationState.active_media_id === selectedMedia.id &&
                $presentationState.active_slide_index === idx &&
                $presentationState.display_mode === 'live'}
              {@const isFocused = focusedSlideIndex === idx}
              {@const sectionStyle = getSectionStyle(slide.label, $sectionsStore)}

              <div
                class={cn(
                  'group flex flex-col rounded-lg border transition-all duration-150 overflow-hidden cursor-pointer select-none relative',
                  isLive
                    ? 'border-emerald-500 bg-zinc-900 shadow-lg ring-2 ring-emerald-500/50'
                    : isFocused
                      ? 'border-sky-500 bg-zinc-900 shadow-md ring-2 ring-sky-500/40'
                      : 'border-zinc-800 bg-zinc-900/50 hover:border-zinc-700 hover:bg-zinc-850'
                )}
                onclick={() => selectSlideForFocus(idx)}
                ondblclick={() => presentSlide(idx)}
                role="button"
                tabindex="0"
                onkeydown={(e) => {
                  if (e.key === 'Enter') presentSlide(idx);
                }}
              >
                <!-- Cabeçalho do Cartão do Slide com Cores Customizadas de Seção -->
                <div
                  class={cn(
                    'px-3 py-1.5 border-b flex items-center justify-between text-xs',
                    sectionStyle.bg,
                    sectionStyle.border
                  )}
                >
                  <span class={cn('font-semibold flex items-center gap-1.5', sectionStyle.text)}>
                    <span
                      class="w-4 h-4 rounded-full bg-black/40 text-white flex items-center justify-center text-[10px] font-mono"
                    >
                      {idx + 1}
                    </span>
                    {slide.label}
                  </span>

                  <div class="flex items-center gap-1">
                    {#if isLive}
                      <Badge variant="live" class="text-[10px] py-0 px-1.5 animate-pulse">
                        AO VIVO
                      </Badge>
                    {:else if isFocused}
                      <Badge variant="secondary" class="text-[10px] py-0 px-1.5 border-sky-500/40 text-sky-400 bg-sky-950/40">
                        FOCADO
                      </Badge>
                    {/if}
                  </div>
                </div>

                <!-- Conteúdo Visual do Slide (Proporção 16:9) -->
                <div class="aspect-video w-full p-4 flex flex-col items-center justify-center text-center bg-black/60 relative">
                  <div class="space-y-1.5 max-w-full">
                    {#each slide.content.split('\n') as line}
                      <p
                        class={cn(
                          'text-xs sm:text-sm font-semibold truncate leading-tight',
                          isLive
                            ? 'text-emerald-200'
                            : isFocused
                              ? 'text-white'
                              : 'text-zinc-200 group-hover:text-white'
                        )}
                      >
                        {line}
                      </p>
                    {/each}
                  </div>

                  <span class="absolute bottom-1.5 right-2 text-[10px] text-zinc-600 font-mono">
                    16:9
                  </span>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {:else}
        <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm">
          Selecione uma mídia na setlist para visualizar os slides.
        </div>
      {/if}
    </main>

    <!-- COLUNA 3: PAINEL DE PREVIEWS (PUBLIC E STAGE - 16:9) -->
    <aside class="w-80 lg:w-96 border-l border-zinc-800/80 bg-zinc-900/30 flex flex-col shrink-0 overflow-y-auto">
      <!-- Botões de Controle Global da Projeção -->
      <div class="p-3 border-b border-zinc-800/80 space-y-2 bg-zinc-900/60 shrink-0">
        <span class="text-xs font-semibold text-zinc-400 uppercase tracking-wider block">
          Controle de Saídas
        </span>

        <div class="grid grid-cols-3 gap-1.5">
          <!-- Botão Ao Vivo -->
          <Button
            variant={$presentationState.display_mode === 'live' ? 'live' : 'secondary'}
            size="sm"
            onclick={() => setDisplayMode('live')}
            class="text-xs"
          >
            <Radio size={13} />
            Ao Vivo
          </Button>

          <!-- Botão Blackout -->
          <Button
            variant={$presentationState.display_mode === 'blackout' ? 'destructive' : 'secondary'}
            size="sm"
            onclick={toggleBlackout}
            class="text-xs"
            title="Blackout total (F1)"
          >
            <EyeOff size={13} />
            Blackout
          </Button>

          <!-- Botão Limpar Texto -->
          <Button
            variant={$presentationState.display_mode === 'cleartext' ? 'default' : 'secondary'}
            size="sm"
            onclick={toggleClearText}
            class="text-xs"
            title="Ocultar texto (F2)"
          >
            <Slash size={13} />
            Limpar
          </Button>
        </div>
      </div>

      <!-- PREVIEW PUBLIC (16:9) -->
      <div class="p-3 border-b border-zinc-800/80 space-y-2">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-1.5">
            <MonitorPlay size={14} class="text-emerald-400" />
            <span class="text-xs font-bold text-zinc-200">Public (Projetor)</span>
          </div>

          <Badge variant={$presentationState.public_connected ? 'success' : 'secondary'} class="text-[10px] py-0 px-1.5">
            {$presentationState.public_connected ? 'Conectado' : 'Aguardando'}
          </Badge>
        </div>

        <!-- Caixa de Preview do Public com Aspect Ratio 16:9 -->
        <div class="w-full aspect-video rounded-lg bg-black border border-zinc-800 flex flex-col items-center justify-center p-3 text-center relative overflow-hidden shadow-inner">
          {#if $presentationState.display_mode === 'blackout'}
            <div class="text-[11px] font-bold text-red-500 uppercase tracking-widest">
              Blackout Ativo
            </div>
          {:else if $presentationState.display_mode === 'cleartext'}
            <div class="text-[11px] font-medium text-zinc-600 italic">
              Texto Oculto
            </div>
          {:else if liveSlide}
            <div class="space-y-1 my-auto max-w-full">
              {#each liveSlide.content.split('\n') as line}
                <p class="text-[11px] sm:text-xs font-bold text-white truncate leading-tight">
                  {line}
                </p>
              {/each}
            </div>
            {#if liveMedia?.artist_or_ref}
              <span class="text-[9px] text-zinc-500 truncate mt-auto">
                {liveMedia.title}
              </span>
            {/if}
          {:else}
            <div class="text-[11px] text-zinc-600">
              Nenhum slide ao vivo
            </div>
          {/if}

          <span class="absolute top-1.5 right-2 text-[9px] text-zinc-600 font-mono">16:9</span>
        </div>

        <Button variant="secondary" size="sm" onclick={openPublicWindow} class="w-full text-xs gap-1.5">
          <ExternalLink size={13} />
          Abrir Janela do Public
        </Button>
      </div>

      <!-- PREVIEW STAGE (16:9) -->
      <div class="p-3 space-y-2">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-1.5">
            <Tv size={14} class="text-emerald-400" />
            <span class="text-xs font-bold text-zinc-200">Stage (Retorno Palco)</span>
          </div>

          <Badge variant={$presentationState.stage_connected ? 'success' : 'secondary'} class="text-[10px] py-0 px-1.5">
            {$presentationState.stage_connected ? 'Conectado' : 'Aguardando'}
          </Badge>
        </div>

        <!-- Caixa de Preview do Stage com Aspect Ratio 16:9 -->
        <div class="w-full aspect-video rounded-lg bg-[#070709] border border-zinc-800 p-2.5 flex flex-col justify-between relative overflow-hidden shadow-inner font-mono text-[10px]">
          <!-- Topo do Stage Preview -->
          <div class="flex items-center justify-between text-zinc-400 border-b border-zinc-800/60 pb-1">
            <span class="text-[9px] uppercase font-bold text-emerald-400">Retorno</span>
            <div class="flex items-center gap-1 text-white font-bold">
              <Clock size={10} class="text-emerald-400" />
              <span>{stageClock || '00:00:00'}</span>
            </div>
          </div>

          <!-- Meio do Stage Preview -->
          <div class="my-auto py-1 space-y-1">
            {#if liveSlide}
              <div class="p-1 rounded bg-zinc-900/80 border border-emerald-500/30">
                <span class="text-[8px] text-emerald-400 font-bold block">ATUAL:</span>
                <p class="text-[9px] font-bold text-zinc-100 truncate">
                  {liveSlide.content.split('\n')[0]}
                </p>
              </div>
            {:else}
              <p class="text-zinc-600 text-center text-[9px]">Aguardando slide</p>
            {/if}

            {#if nextLiveSlide}
              <div class="p-1 rounded bg-zinc-900/40 border border-zinc-800">
                <span class="text-[8px] text-zinc-500 font-bold block">A SEGUIR:</span>
                <p class="text-[9px] text-zinc-400 truncate">
                  {nextLiveSlide.content.split('\n')[0]}
                </p>
              </div>
            {/if}
          </div>

          <span class="absolute bottom-1 right-2 text-[9px] text-zinc-700 font-mono">16:9</span>
        </div>

        <Button variant="secondary" size="sm" onclick={openStageWindow} class="w-full text-xs gap-1.5">
          <ExternalLink size={13} />
          Abrir Janela do Stage
        </Button>
      </div>
    </aside>
  </div>

  <!-- Floating Number Buffer HUD -->
  {#if numberBuffer}
    <div
      class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 px-4 py-2 bg-emerald-600 text-white font-mono font-bold text-base rounded-xl shadow-2xl flex items-center gap-2.5 border border-emerald-400/50"
    >
      <span class="text-xs uppercase tracking-wider text-emerald-200">Ir para slide:</span>
      <span class="text-lg underline decoration-2">{numberBuffer}</span>
      <span class="text-xs font-normal opacity-90">(Pressione Enter)</span>
    </div>
  {/if}

  <!-- Modais Operacionais: Command Palette, Editor de Músicas e Configurações -->
  <CommandPalette
    open={isCommandPaletteOpen}
    onclose={() => (isCommandPaletteOpen = false)}
    oncreatenew={handleOpenCreate}
    onimportsong={handleImportSong}
  />

  <SongEditorModal
    open={isSongEditorOpen}
    initialSong={editingSong}
    onclose={() => {
      isSongEditorOpen = false;
      editingSong = null;
    }}
    onsave={handleSongSaved}
  />

  <SettingsModal
    open={isSettingsOpen}
    onclose={() => (isSettingsOpen = false)}
  />
</div>

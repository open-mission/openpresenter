<script lang="ts">
  import {
    AlertTriangle,
    ChevronDown,
    ChevronUp,
    Eye,
    KeyRound,
    ListMusic,
    Music,
    Pause,
    Play,
    RefreshCw,
    Send,
    Square,
    Volume2,
  } from 'lucide-svelte';
  import type { AudioPlaybackState, MediaItem, Setlist } from '$lib/types';
  import { formatMediaTime, isSameAudioTrack } from '$lib/remoteData';
  import { controlAudioPlayback } from '$lib/ws';

  interface Props {
    canControlRemote: boolean;
    safeDispatch: boolean;
    activeSetlistData: Setlist | null;
    activeMediaId: string | null;
    activeSlideIndex: number | null;
    audioPlayback?: AudioPlaybackState | null;
    expandedSetlistItemId: string | null;
    isLoading: boolean;
    error: string | null;
    onOpenSetlistSelector: () => void;
    onRefresh: () => void;
    onToggleExpandItem: (id: string) => void;
    onProjectSlide: (item: MediaItem, slideIndex: number) => void;
    onOpenPinModal: () => void;
  }

  let {
    canControlRemote,
    safeDispatch,
    activeSetlistData,
    activeMediaId,
    activeSlideIndex,
    audioPlayback = null,
    expandedSetlistItemId,
    isLoading,
    error,
    onOpenSetlistSelector,
    onRefresh,
    onToggleExpandItem,
    onProjectSlide,
    onOpenPinModal,
  }: Props = $props();

  let cuedSlideKey = $state<string | null>(null);

  // Pré-calcula numeração sequencial ignorando cabeçalhos de seção (O(n))
  const numberedItems = $derived.by(() => {
    if (!activeSetlistData) return [] as Array<{ item: MediaItem; displayNumber: number }>;
    let seq = 0;
    return activeSetlistData.items.map((item) => {
      if (item.kind !== 'header') seq += 1;
      return { item, displayNumber: seq };
    });
  });

  function handleSlideTap(item: MediaItem, sIdx: number) {
    if (!canControlRemote) return;
    const key = `${item.id}:${sIdx}`;
    if (!safeDispatch || cuedSlideKey === key) {
      cuedSlideKey = null;
      onProjectSlide(item, sIdx);
    } else {
      cuedSlideKey = key;
    }
  }
</script>

<div class="space-y-3 pb-2">
  <h1 class="sr-only">Ordem do Culto e Setlist</h1>

  {#if !canControlRemote}
    <div class="p-3 rounded-2xl bg-amber-500/15 border border-amber-600/35 dark:border-amber-500/40 flex items-center justify-between gap-2.5">
      <div class="flex items-center gap-2 min-w-0 text-amber-800 dark:text-amber-300">
        <Eye size={16} class="shrink-0" aria-hidden="true" />
        <p class="text-xs font-medium">
          Modo Consulta — Você acompanha o roteiro e letras sem alterar o telão.
        </p>
      </div>
      <button
        type="button"
        onclick={onOpenPinModal}
        class="min-h-9 px-3 rounded-xl bg-amber-700 hover:bg-amber-800 text-white text-xs font-bold inline-flex items-center gap-1.5 transition-colors cursor-pointer shrink-0"
      >
        <KeyRound size={13} aria-hidden="true" />
        <span>Operar</span>
      </button>
    </div>
  {/if}

  <!-- Barra de Seleção e Atualização da Setlist -->
  <div class="flex items-center gap-2">
    <button
      type="button"
      onclick={onOpenSetlistSelector}
      aria-haspopup="dialog"
      class="flex-1 min-h-11 px-3.5 py-2 rounded-xl bg-card hover:bg-muted/70 border border-border flex items-center justify-between gap-2 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
    >
      <span class="min-w-0">
        <span class="block text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">
          Setlist Selecionada
        </span>
        <span class="block text-sm font-bold text-foreground truncate">
          {activeSetlistData ? activeSetlistData.title : 'Nenhuma lista carregada'}
        </span>
      </span>
      <ChevronDown size={17} class="text-muted-foreground shrink-0" aria-hidden="true" />
    </button>

    <button
      type="button"
      onclick={onRefresh}
      aria-label="Atualizar setlist do culto"
      title="Atualizar setlist"
      class="h-11 w-11 rounded-xl bg-card hover:bg-muted border border-border text-foreground flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer shrink-0"
    >
      <RefreshCw size={17} class={isLoading ? 'animate-spin' : ''} aria-hidden="true" />
    </button>
  </div>

  <!-- Banner de erro HTTP -->
  {#if error}
    <div
      role="alert"
      class="p-3.5 rounded-2xl bg-red-500/15 border border-red-600/35 dark:border-red-500/40 text-red-700 dark:text-red-300 flex items-center justify-between gap-3"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertTriangle size={16} class="shrink-0" aria-hidden="true" />
        <p class="text-xs font-medium">{error}</p>
      </div>
      <button
        type="button"
        onclick={onRefresh}
        class="min-h-9 px-3 rounded-lg bg-red-700 hover:bg-red-800 text-white text-xs font-bold cursor-pointer shrink-0"
      >
        Tentar
      </button>
    </div>
  {/if}

  <!-- Banner Global de Playback -->
  {#if audioPlayback && (audioPlayback.is_playing || audioPlayback.duration_seconds > 0)}
    <div class="p-3 rounded-2xl bg-emerald-500/15 border border-emerald-600/35 dark:border-emerald-500/40 flex items-center justify-between gap-3">
      <div class="flex items-center gap-2.5 min-w-0">
        <span class="w-9 h-9 rounded-xl bg-emerald-700 text-white flex items-center justify-center shrink-0">
          <Volume2 size={17} aria-hidden="true" />
        </span>
        <div class="min-w-0">
          <p class="text-[11px] font-bold uppercase tracking-wider text-emerald-800 dark:text-emerald-300">
            {audioPlayback.is_playing ? 'Playback Tocando' : 'Playback Pausado'}
          </p>
          <p class="text-xs font-bold text-foreground truncate">{audioPlayback.title}</p>
          <p class="text-[11px] font-mono tabular-nums text-muted-foreground">
            {formatMediaTime(audioPlayback.current_time_seconds)} / {formatMediaTime(audioPlayback.duration_seconds)}
          </p>
        </div>
      </div>

      {#if canControlRemote}
        <div class="flex items-center gap-1.5 shrink-0">
          <button
            type="button"
            onclick={() => {
              if (audioPlayback?.is_playing) {
                controlAudioPlayback('pause');
              } else {
                controlAudioPlayback('play', {
                  title: audioPlayback?.title,
                  artist_or_ref: audioPlayback?.artist_or_ref,
                });
              }
            }}
            aria-label={audioPlayback.is_playing ? 'Pausar playback' : 'Continuar playback'}
            class="h-10 w-10 rounded-xl bg-emerald-700 hover:bg-emerald-800 text-white flex items-center justify-center transition-colors cursor-pointer"
          >
            {#if audioPlayback.is_playing}
              <Pause size={16} aria-hidden="true" />
            {:else}
              <Play size={16} aria-hidden="true" />
            {/if}
          </button>
          <button
            type="button"
            onclick={() => controlAudioPlayback('stop')}
            aria-label="Parar playback"
            class="h-10 w-10 rounded-xl bg-red-500/20 hover:bg-red-500/30 text-red-700 dark:text-red-300 border border-red-600/35 dark:border-red-500/40 flex items-center justify-center transition-colors cursor-pointer"
          >
            <Square size={15} aria-hidden="true" />
          </button>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Itens da Setlist -->
  {#if !activeSetlistData || activeSetlistData.items.length === 0}
    <div class="py-12 px-4 text-center space-y-2 rounded-2xl bg-card border border-border">
      <ListMusic size={28} class="mx-auto text-muted-foreground" aria-hidden="true" />
      <p class="text-sm font-semibold text-foreground">
        {activeSetlistData ? 'Esta setlist está vazia' : 'Nenhuma setlist ativa disponível'}
      </p>
      <p class="text-xs text-muted-foreground">
        Adicione louvores e leituras no painel principal para operar pelo celular.
      </p>
    </div>
  {:else}
    <div class="space-y-2">
      {#each numberedItems as { item, displayNumber } (item.id)}
        {#if item.kind === 'header'}
          <div class="px-2 pt-3 pb-1 flex items-center gap-2">
            <span class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground shrink-0">
              {item.title}
            </span>
            <span class="h-px flex-1 bg-border" aria-hidden="true"></span>
          </div>
        {:else}
          {@const isItemLive = activeMediaId === item.id}
          {@const isExpanded = expandedSetlistItemId === item.id}
          {@const trackPlaying = isSameAudioTrack(audioPlayback, item) && Boolean(audioPlayback?.is_playing)}
          {@const trackMatched = isSameAudioTrack(audioPlayback, item)}

          <div
            class="rounded-2xl border overflow-hidden transition-colors {isItemLive
              ? 'bg-emerald-500/10 border-emerald-600/50 dark:border-emerald-500/50'
              : 'bg-card border-border'}"
          >
            <button
              type="button"
              onclick={() => onToggleExpandItem(item.id)}
              aria-expanded={isExpanded}
              class="w-full min-h-[56px] p-3.5 flex items-center justify-between gap-3 text-left hover:bg-muted/50 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
            >
              <span class="flex items-center gap-3 min-w-0 flex-1">
                <span
                  class="w-7 h-7 rounded-lg text-xs font-bold flex items-center justify-center shrink-0 {isItemLive
                    ? 'bg-emerald-700 text-white'
                    : 'bg-secondary text-secondary-foreground border border-border'}"
                >
                  {displayNumber}
                </span>
                <span class="min-w-0 flex-1">
                  <span class="flex items-center gap-1.5 min-w-0">
                    <span class="text-sm font-bold text-foreground truncate">{item.title}</span>
                    {#if item.audio_url}
                      <Music
                        size={13}
                        class="text-emerald-700 dark:text-emerald-400 shrink-0"
                        aria-label="Com playback vinculado"
                      />
                    {/if}
                  </span>
                  <span class="block text-xs text-muted-foreground truncate mt-0.5">
                    {item.slides.length} {item.slides.length === 1 ? 'slide' : 'slides'}
                    {item.artist_or_ref ? `• ${item.artist_or_ref}` : ''}
                  </span>
                </span>
              </span>

              <span class="flex items-center gap-2 shrink-0">
                {#if isItemLive}
                  <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-emerald-700 text-white">
                    No ar
                  </span>
                {/if}
                {#if isExpanded}
                  <ChevronUp size={17} class="text-muted-foreground" aria-hidden="true" />
                {:else}
                  <ChevronDown size={17} class="text-muted-foreground" aria-hidden="true" />
                {/if}
              </span>
            </button>

            {#if isExpanded}
              <div class="px-3 pb-3 pt-1.5 space-y-2 border-t border-border/70 bg-background/60">
                {#if item.audio_url && canControlRemote}
                  {@const cur = trackMatched ? audioPlayback?.current_time_seconds || 0 : 0}
                  {@const dur = trackMatched ? audioPlayback?.duration_seconds || 0 : 0}
                  {@const pct = dur > 0 ? Math.min(100, Math.round((cur / dur) * 100)) : 0}
                  <div class="p-2.5 rounded-xl bg-card border border-border space-y-2">
                    <div class="flex items-center justify-between gap-2">
                      <div class="flex items-center gap-2 min-w-0">
                        <Music size={14} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
                        <span class="text-xs font-semibold text-foreground truncate">
                          Playback vinculado
                        </span>
                      </div>
                      <div class="flex items-center gap-1.5 shrink-0">
                        <span class="text-[11px] font-mono tabular-nums text-muted-foreground">
                          {formatMediaTime(cur)}{dur > 0 ? ` / ${formatMediaTime(dur)}` : ''}
                        </span>
                        {#if trackPlaying}
                          <button
                            type="button"
                            onclick={() => controlAudioPlayback('pause')}
                            class="min-h-9 px-3 rounded-lg bg-amber-700 hover:bg-amber-800 text-white text-xs font-bold inline-flex items-center gap-1 cursor-pointer"
                          >
                            <Pause size={13} aria-hidden="true" />
                            <span>Pausar</span>
                          </button>
                        {:else}
                          <button
                            type="button"
                            onclick={() =>
                              controlAudioPlayback('play', {
                                url: item.audio_url,
                                title: item.title,
                                artist_or_ref: item.artist_or_ref,
                              })}
                            class="min-h-9 px-3 rounded-lg bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold inline-flex items-center gap-1 cursor-pointer"
                          >
                            <Play size={13} aria-hidden="true" />
                            <span>Tocar</span>
                          </button>
                        {/if}
                      </div>
                    </div>
                    {#if trackMatched && dur > 0}
                      <div class="w-full h-1.5 rounded-full bg-muted overflow-hidden" aria-hidden="true">
                        <div class="h-full bg-emerald-600 transition-all" style="width: {pct}%;"></div>
                      </div>
                    {/if}
                  </div>
                {/if}

                {#if item.slides.length === 0}
                  <p class="text-xs text-muted-foreground text-center py-3">
                    Este item não possui slides de texto.
                  </p>
                {:else}
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-1.5">
                    {#each item.slides as slide, sIdx (slide.id || `${item.id}-${sIdx}`)}
                      {@const isSlideLive = isItemLive && activeSlideIndex === sIdx}
                      {@const slideKey = `${item.id}:${sIdx}`}
                      {@const isCued = canControlRemote && safeDispatch && cuedSlideKey === slideKey}

                      {#if canControlRemote}
                        <div
                          class="rounded-xl border transition-colors flex items-stretch gap-2 {isSlideLive
                            ? 'bg-emerald-500/15 border-emerald-600/60 dark:border-emerald-500'
                            : isCued
                              ? 'bg-amber-500/15 border-amber-600/60 dark:border-amber-500'
                              : 'bg-card border-border'}"
                        >
                          <button
                            type="button"
                            onclick={() => handleSlideTap(item, sIdx)}
                            class="flex-1 min-h-12 px-3 py-2.5 text-left rounded-xl hover:bg-muted/50 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
                          >
                            <span class="flex items-center justify-between gap-2 mb-1">
                              <span class="text-[11px] font-bold uppercase tracking-wider {isSlideLive
                                ? 'text-emerald-800 dark:text-emerald-300'
                                : isCued
                                  ? 'text-amber-800 dark:text-amber-300'
                                  : 'text-muted-foreground'}">
                                {slide.label || `Slide ${sIdx + 1}`}
                              </span>
                              {#if isSlideLive}
                                <span class="text-[11px] font-bold text-emerald-800 dark:text-emerald-300">
                                  Ao Vivo
                                </span>
                              {:else if isCued}
                                <span class="text-[11px] font-bold text-amber-800 dark:text-amber-300">
                                  Pronto
                                </span>
                              {/if}
                            </span>
                            <span class="block text-xs text-foreground leading-snug whitespace-pre-line line-clamp-3">
                              {slide.content}
                            </span>
                          </button>

                          {#if isCued}
                            <button
                              type="button"
                              onclick={() => {
                                cuedSlideKey = null;
                                onProjectSlide(item, sIdx);
                              }}
                              class="m-1.5 min-h-10 px-3 rounded-lg bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer shrink-0 self-center"
                            >
                              <Send size={13} aria-hidden="true" />
                              <span>Projetar</span>
                            </button>
                          {/if}
                        </div>
                      {:else}
                        <div class="p-3 rounded-xl bg-card border border-border space-y-1">
                          <span class="block text-[11px] font-bold uppercase tracking-wider text-muted-foreground">
                            {slide.label || `Slide ${sIdx + 1}`}
                          </span>
                          <p class="text-xs text-foreground leading-snug whitespace-pre-line">
                            {slide.content}
                          </p>
                        </div>
                      {/if}
                    {/each}
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        {/if}
      {/each}
    </div>
  {/if}
</div>

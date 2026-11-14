<script lang="ts">
  import {
    AlertTriangle,
    ArrowLeft,
    ChevronRight,
    Loader2,
    Music,
    Pause,
    Play,
    RefreshCw,
    Search,
    Send,
    X,
  } from 'lucide-svelte';
  import type { AudioPlaybackState, MediaItem, SongSummary } from '$lib/types';
  import { parseRawLyrics } from '$lib/songParser';
  import { formatMediaTime, isSameAudioTrack } from '$lib/remoteData';
  import { controlAudioPlayback } from '$lib/ws';

  interface Props {
    songs: SongSummary[];
    songSearchQuery: string;
    isSearchingSongs: boolean;
    songsError: string | null;
    selectedSong: MediaItem | null;
    activeMediaId: string | null;
    activeSlideIndex: number | null;
    audioPlayback?: AudioPlaybackState | null;
    safeDispatch: boolean;
    onSearchInput: (query: string) => void;
    onRetrySongs: () => void;
    onOpenSong: (summary: SongSummary) => void;
    onCloseSong: () => void;
    onProjectSongSlide: (song: MediaItem, slideIndex: number) => void;
  }

  let {
    songs,
    songSearchQuery,
    isSearchingSongs,
    songsError,
    selectedSong,
    activeMediaId,
    activeSlideIndex,
    audioPlayback = null,
    safeDispatch,
    onSearchInput,
    onRetrySongs,
    onOpenSong,
    onCloseSong,
    onProjectSongSlide,
  }: Props = $props();

  let cuedSlideIndex = $state<number | null>(null);

  const parsedSongSlides = $derived.by(() => {
    if (!selectedSong) return [];
    if (selectedSong.slides && selectedSong.slides.length > 0) return selectedSong.slides;
    const raw = (selectedSong as unknown as { lyrics?: string }).lyrics;
    if (!raw) return [];
    return parseRawLyrics(raw).slides.map((slide, idx) => ({
      ...slide,
      id: `${selectedSong.id}-s-${idx}`,
    }));
  });

  function handleSlideClick(idx: number) {
    if (!selectedSong) return;
    if (!safeDispatch || cuedSlideIndex === idx) {
      cuedSlideIndex = null;
      onProjectSongSlide(selectedSong, idx);
    } else {
      cuedSlideIndex = idx;
    }
  }
</script>

<div class="space-y-3 pb-2">
  <h1 class="sr-only">Músicas e Letras</h1>

  {#if !selectedSong}
    <!-- Barra de Busca de Louvores -->
    <div class="relative">
      <label for="remote-songs-search" class="sr-only">
        Buscar música por título, artista ou letra
      </label>
      <Search
        size={16}
        aria-hidden="true"
        class="absolute left-3.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
      />
      <input
        id="remote-songs-search"
        type="search"
        value={songSearchQuery}
        oninput={(e) => onSearchInput(e.currentTarget.value)}
        placeholder="Buscar louvor por título ou letra..."
        autocomplete="off"
        class="w-full h-11 bg-card border border-border rounded-xl pl-10 pr-11 text-base sm:text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring transition-colors"
      />
      {#if songSearchQuery}
        <button
          type="button"
          onclick={() => onSearchInput('')}
          aria-label="Limpar busca de músicas"
          class="h-9 w-9 rounded-lg absolute right-1 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
        >
          <X size={16} aria-hidden="true" />
        </button>
      {/if}
    </div>

    {#if songsError}
      <div
        role="alert"
        class="p-3.5 rounded-2xl bg-red-500/15 border border-red-600/35 dark:border-red-500/40 text-red-700 dark:text-red-300 space-y-2.5"
      >
        <p class="text-xs font-medium flex items-center gap-2">
          <AlertTriangle size={16} class="shrink-0" aria-hidden="true" />
          <span>{songsError}</span>
        </p>
        <button
          type="button"
          onclick={onRetrySongs}
          class="min-h-9 px-3 rounded-lg bg-red-700 hover:bg-red-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer"
        >
          <RefreshCw size={13} aria-hidden="true" />
          <span>Tentar novamente</span>
        </button>
      </div>
    {:else if isSearchingSongs}
      <div class="py-10 flex items-center justify-center gap-2 text-xs text-muted-foreground" role="status">
        <Loader2 size={18} class="animate-spin text-emerald-700 dark:text-emerald-400" aria-hidden="true" />
        <span>Buscando catálogo de louvores...</span>
      </div>
    {:else if songs.length === 0}
      <div class="py-12 px-4 text-center space-y-2 rounded-2xl bg-card border border-border">
        <Music size={28} class="mx-auto text-muted-foreground" aria-hidden="true" />
        <p class="text-sm font-semibold text-foreground">
          {songSearchQuery ? 'Nenhuma música encontrada' : 'Catálogo sem músicas cadastradas'}
        </p>
        <p class="text-xs text-muted-foreground">
          {songSearchQuery ? 'Tente outro trecho ou título.' : 'Importe louvores ou a Harpa Cristã nas configurações.'}
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
        {#each songs as song (song.id)}
          <button
            type="button"
            onclick={() => {
              cuedSlideIndex = null;
              onOpenSong(song);
            }}
            class="w-full min-h-[60px] p-3.5 rounded-2xl bg-card hover:bg-muted/70 border border-border flex items-center justify-between gap-3 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
          >
            <span class="min-w-0 flex-1">
              <span class="flex items-center gap-1.5 min-w-0">
                <span class="text-sm font-bold text-foreground truncate">{song.title}</span>
                {#if song.audio_url}
                  <span class="px-1.5 py-0.5 rounded-md text-[11px] font-semibold bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border border-emerald-600/35 dark:border-emerald-500/40 shrink-0">
                    Playback
                  </span>
                {/if}
              </span>
              {#if song.artist}
                <span class="block text-xs text-muted-foreground truncate mt-0.5">{song.artist}</span>
              {/if}
            </span>
            <ChevronRight size={17} class="text-muted-foreground shrink-0" aria-hidden="true" />
          </button>
        {/each}
      </div>
    {/if}
  {:else}
    <!-- Detalhes e Estrofes da Música -->
    {@const isSongLive = activeMediaId === selectedSong.id}
    {@const trackMatched = isSameAudioTrack(audioPlayback, selectedSong)}
    {@const trackPlaying = trackMatched && Boolean(audioPlayback?.is_playing)}

    <div class="space-y-3">
      <div class="p-3 rounded-2xl bg-card border border-border flex items-center justify-between gap-2.5">
        <button
          type="button"
          onclick={() => {
            cuedSlideIndex = null;
            onCloseSong();
          }}
          aria-label="Voltar para a lista de músicas"
          class="min-h-10 px-3 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold inline-flex items-center gap-1.5 transition-colors cursor-pointer shrink-0"
        >
          <ArrowLeft size={15} aria-hidden="true" />
          <span>Voltar</span>
        </button>

        <div class="flex-1 min-w-0">
          <h2 class="text-sm font-bold text-foreground truncate">{selectedSong.title}</h2>
          {#if selectedSong.artist_or_ref}
            <p class="text-xs text-muted-foreground truncate">{selectedSong.artist_or_ref}</p>
          {/if}
        </div>
      </div>

      {#if selectedSong.audio_url}
        <div class="p-3 rounded-2xl bg-card border border-border flex items-center justify-between gap-3">
          <div class="min-w-0">
            <p class="text-xs font-bold text-foreground">Playback da música</p>
            <p class="text-[11px] font-mono tabular-nums text-muted-foreground">
              {trackMatched
                ? `${formatMediaTime(audioPlayback?.current_time_seconds)} / ${formatMediaTime(audioPlayback?.duration_seconds)}`
                : 'Pronto para reproduzir na mesa'}
            </p>
          </div>
          {#if trackPlaying}
            <button
              type="button"
              onclick={() => controlAudioPlayback('pause')}
              class="min-h-10 px-3.5 rounded-xl bg-amber-700 hover:bg-amber-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer shrink-0"
            >
              <Pause size={14} aria-hidden="true" />
              <span>Pausar</span>
            </button>
          {:else}
            <button
              type="button"
              onclick={() =>
                controlAudioPlayback('play', {
                  url: selectedSong?.audio_url,
                  title: selectedSong?.title,
                  artist_or_ref: selectedSong?.artist_or_ref,
                })}
              class="min-h-10 px-3.5 rounded-xl bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer shrink-0"
            >
              <Play size={14} aria-hidden="true" />
              <span>Tocar Playback</span>
            </button>
          {/if}
        </div>
      {/if}

      {#if parsedSongSlides.length === 0}
        <div class="p-6 text-center rounded-2xl bg-card border border-border text-xs text-muted-foreground">
          Esta música não possui slides configurados.
        </div>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
          {#each parsedSongSlides as slide, sIdx (slide.id || `${selectedSong.id}-${sIdx}`)}
            {@const isSlideLive = isSongLive && activeSlideIndex === sIdx}
            {@const isCued = safeDispatch && cuedSlideIndex === sIdx}

            <div
              class="rounded-2xl border transition-colors flex items-stretch gap-2 {isSlideLive
                ? 'bg-emerald-500/15 border-emerald-600/60 dark:border-emerald-500'
                : isCued
                  ? 'bg-amber-500/15 border-amber-600/60 dark:border-amber-500'
                  : 'bg-card border-border'}"
            >
              <button
                type="button"
                onclick={() => handleSlideClick(sIdx)}
                class="flex-1 min-h-14 p-3.5 rounded-2xl text-left hover:bg-muted/50 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
              >
                <span class="flex items-center justify-between gap-2 mb-1">
                  <span class="text-[11px] font-bold uppercase tracking-wider {isSlideLive
                    ? 'text-emerald-800 dark:text-emerald-300'
                    : isCued
                      ? 'text-amber-800 dark:text-amber-300'
                      : 'text-muted-foreground'}">
                    {slide.label || `Estrofe ${sIdx + 1}`}
                  </span>
                  {#if isSlideLive}
                    <span class="text-[11px] font-bold text-emerald-800 dark:text-emerald-300">Ao Vivo</span>
                  {:else if isCued}
                    <span class="text-[11px] font-bold text-amber-800 dark:text-amber-300">Selecionado</span>
                  {/if}
                </span>
                <span class="block text-sm text-foreground leading-relaxed whitespace-pre-line">
                  {slide.content}
                </span>
              </button>

              {#if isCued}
                <button
                  type="button"
                  onclick={() => {
                    cuedSlideIndex = null;
                    if (selectedSong) onProjectSongSlide(selectedSong, sIdx);
                  }}
                  class="m-2 min-h-10 px-3 rounded-xl bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer shrink-0 self-center"
                >
                  <Send size={13} aria-hidden="true" />
                  <span>Projetar</span>
                </button>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

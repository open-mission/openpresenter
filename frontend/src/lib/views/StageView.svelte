<script lang="ts">
  import { onMount } from 'svelte';
  import { presentationState, mediaItemsList, initWebSocket } from '$lib/ws';
  import type { MediaItem, Slide } from '$lib/types';
  import { Maximize, Minimize, Clock, ArrowRight } from 'lucide-svelte';

  let currentTime = $state('');
  let currentDate = $state('');
  let isFullscreen = $state(false);

  onMount(() => {
    initWebSocket('stage');

    const updateClock = () => {
      const now = new Date();
      currentTime = now.toLocaleTimeString('pt-BR', { hour12: false });
      currentDate = now.toLocaleDateString('pt-BR', {
        weekday: 'long',
        day: '2-digit',
        month: 'long',
      });
    };

    updateClock();
    const interval = setInterval(updateClock, 1000);

    const handleFullscreenChange = () => {
      isFullscreen = !!document.fullscreenElement;
    };
    document.addEventListener('fullscreenchange', handleFullscreenChange);

    return () => {
      clearInterval(interval);
      document.removeEventListener('fullscreenchange', handleFullscreenChange);
    };
  });

  function toggleFullscreen() {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(() => {});
    } else {
      document.exitFullscreen().catch(() => {});
    }
  }

  // Mídia ativa
  const activeMedia = $derived.by<MediaItem | undefined>(() => {
    const p = $presentationState;
    if (!p.active_media_id) return undefined;
    return $mediaItemsList.find((m) => m.id === p.active_media_id);
  });

  // Slide atual
  const activeSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!activeMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    return activeMedia.slides[p.active_slide_index];
  });

  // Próximo slide
  const nextSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!activeMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    const nextIdx = p.active_slide_index + 1;
    if (nextIdx < activeMedia.slides.length) {
      return activeMedia.slides[nextIdx];
    }
    return undefined;
  });
</script>

<svelte:window onkeydown={(e) => { if (e.key.toLowerCase() === 'f') toggleFullscreen(); }} />

<!-- Monitor de Palco / Retorno do Apresentador -->
<main
  class="relative w-screen h-screen bg-[#070709] text-white flex flex-col p-6 overflow-hidden select-none font-sans"
  ondblclick={toggleFullscreen}
>
  <!-- Barra superior com relógio e data -->
  <header class="flex items-center justify-between pb-4 border-b border-zinc-800/80 shrink-0">
    <div class="flex items-center gap-3">
      <span class="px-2.5 py-1 rounded bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 text-xs font-bold uppercase tracking-wider">
        Retorno de Palco
      </span>
      {#if activeMedia}
        <span class="text-zinc-400 text-sm truncate max-w-md font-medium">
          {activeMedia.title}
          {#if activeMedia.artist_or_ref}
            <span class="text-zinc-600">({activeMedia.artist_or_ref})</span>
          {/if}
        </span>
      {/if}
    </div>

    <div class="flex items-center gap-6">
      <div class="text-right">
        <p class="text-xs text-zinc-400 capitalize">{currentDate}</p>
        <div class="flex items-center gap-2 justify-end">
          <Clock size={18} class="text-emerald-400" />
          <span class="text-2xl sm:text-3xl font-mono font-bold text-white tracking-wider">
            {currentTime}
          </span>
        </div>
      </div>

      <button
        type="button"
        onclick={toggleFullscreen}
        title="Alternar Tela Cheia (F)"
        class="p-2 rounded bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 hover:text-white"
      >
        {#if isFullscreen}
          <Minimize size={18} />
        {:else}
          <Maximize size={18} />
        {/if}
      </button>
    </div>
  </header>

  <!-- Indicador se Blackout ou Limpar Texto estiver ativo -->
  {#if $presentationState.display_mode === 'blackout'}
    <div class="bg-red-500/20 border border-red-500/40 text-red-300 py-1.5 px-4 text-center text-xs font-semibold uppercase tracking-wider rounded mt-3">
      Projeção em Blackout (Tela Preta no Público)
    </div>
  {:else if $presentationState.display_mode === 'cleartext'}
    <div class="bg-amber-500/20 border border-amber-500/40 text-amber-300 py-1.5 px-4 text-center text-xs font-semibold uppercase tracking-wider rounded mt-3">
      Texto Oculto no Público (Clear Text)
    </div>
  {/if}

  <!-- Corpo com Slide Atual e Próximo Slide -->
  <div class="flex-1 grid grid-cols-1 md:grid-cols-12 gap-6 pt-5 min-h-0">
    <!-- Slide Atual (Coluna Principal - 8 colunas) -->
    <div class="md:col-span-8 flex flex-col rounded-xl bg-zinc-900/60 border-2 border-emerald-500/40 p-6 sm:p-8 overflow-hidden shadow-lg relative">
      <div class="flex items-center justify-between pb-4 border-b border-zinc-800 mb-auto">
        <span class="text-xs font-bold uppercase tracking-widest text-emerald-400 flex items-center gap-1.5">
          <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
          Slide Atual no Público
        </span>
        {#if activeSlide}
          <span class="text-xs font-semibold bg-zinc-800 text-zinc-300 px-2 py-0.5 rounded">
            {activeSlide.label}
          </span>
        {/if}
      </div>

      <div class="my-auto py-4">
        {#if activeSlide}
          <div class="space-y-3">
            {#each activeSlide.content.split('\n') as line}
              <p class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-extrabold text-white leading-tight">
                {line}
              </p>
            {/each}
          </div>
        {:else}
          <p class="text-zinc-500 text-xl font-medium text-center">
            Nenhum slide sendo projetado no momento
          </p>
        {/if}
      </div>

      <div class="pt-4 border-t border-zinc-800/80 mt-auto flex justify-between text-xs text-zinc-500 font-mono">
        <span>OpenPresenter Stage</span>
        {#if activeMedia && $presentationState.active_slide_index !== null}
          <span>Slide {$presentationState.active_slide_index + 1} de {activeMedia.slides.length}</span>
        {/if}
      </div>
    </div>

    <!-- Próximo Slide (Coluna Lateral - 4 colunas) -->
    <div class="md:col-span-4 flex flex-col rounded-xl bg-zinc-900/30 border border-zinc-800 p-5 overflow-hidden">
      <div class="flex items-center justify-between pb-3 border-b border-zinc-800/80 mb-3">
        <span class="text-xs font-bold uppercase tracking-widest text-zinc-400 flex items-center gap-1.5">
          <ArrowRight size={14} class="text-emerald-400" />
          A Seguir (Próximo)
        </span>
        {#if nextSlide}
          <span class="text-xs font-medium bg-zinc-800 text-zinc-400 px-2 py-0.5 rounded">
            {nextSlide.label}
          </span>
        {/if}
      </div>

      <div class="my-auto py-2">
        {#if nextSlide}
          <div class="space-y-2 opacity-80">
            {#each nextSlide.content.split('\n') as line}
              <p class="text-lg sm:text-xl font-medium text-zinc-300 leading-snug">
                {line}
              </p>
            {/each}
          </div>
        {:else if activeSlide}
          <p class="text-zinc-600 text-sm italic text-center py-8">
            Fim desta mídia (não há próximo slide)
          </p>
        {:else}
          <p class="text-zinc-600 text-sm text-center py-8">
            Aguardando seleção
          </p>
        {/if}
      </div>
    </div>
  </div>
</main>

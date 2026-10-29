<script lang="ts">
  import { onMount } from 'svelte';
  import { presentationState, mediaItemsList, initWebSocket } from '$lib/ws';
  import type { MediaItem, Slide } from '$lib/types';
  import { Maximize, Minimize } from 'lucide-svelte';

  let isFullscreen = $state(false);

  onMount(() => {
    initWebSocket('public');

    const handleFullscreenChange = () => {
      isFullscreen = !!document.fullscreenElement;
    };

    document.addEventListener('fullscreenchange', handleFullscreenChange);
    return () => {
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

  // Encontra a mídia e o slide ativos
  const activeMedia = $derived.by<MediaItem | undefined>(() => {
    const p = $presentationState;
    if (!p.active_media_id) return undefined;
    return $mediaItemsList.find((m) => m.id === p.active_media_id);
  });

  const activeSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!activeMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    return activeMedia.slides[p.active_slide_index];
  });
</script>

<svelte:window onkeydown={(e) => { if (e.key.toLowerCase() === 'f') toggleFullscreen(); }} />

<!-- Container fullscreen de projeção pública -->
<main
  class="relative w-screen h-screen bg-black text-white flex items-center justify-center overflow-hidden select-none"
  ondblclick={toggleFullscreen}
>
  {#if $presentationState.display_mode === 'blackout'}
    <!-- Blackout total: tela preta absoluta -->
    <div class="absolute inset-0 bg-black z-20"></div>
  {:else if $presentationState.display_mode === 'cleartext'}
    <!-- Clear Text: texto oculto, mantém atmosfera limpa -->
    <div class="text-zinc-700 text-sm font-light select-none opacity-20">
      OpenPresenter
    </div>
  {:else if activeSlide}
    <!-- Slide Ao Vivo -->
    <div class="w-full h-full max-w-[95vw] max-h-[92vh] aspect-video flex flex-col items-center justify-center p-8 sm:p-14 text-center animate-in fade-in duration-150">
      <div class="space-y-4 my-auto">
        {#each activeSlide.content.split('\n') as line}
          <p class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight text-white drop-shadow-[0_4px_16px_rgba(0,0,0,0.95)] leading-tight">
            {line}
          </p>
        {/each}
      </div>

      {#if activeMedia?.artist_or_ref}
        <div class="text-zinc-400 text-sm md:text-base font-medium mt-auto opacity-75">
          {activeMedia.title} — {activeMedia.artist_or_ref}
        </div>
      {/if}
    </div>
  {:else}
    <!-- Estado de espera sem mídia ativa -->
    <div class="flex flex-col items-center justify-center text-center text-zinc-500 gap-3">
      <div class="w-12 h-12 rounded-full border border-zinc-700/50 flex items-center justify-center text-emerald-500 font-bold text-xl">
        OP
      </div>
      <p class="text-lg font-medium text-zinc-400">OpenPresenter</p>
      <p class="text-xs text-zinc-600">Aguardando disparo de slide pelo operador</p>
    </div>
  {/if}

  <!-- Botão sutil de tela cheia no canto inferior direito -->
  <button
    type="button"
    onclick={toggleFullscreen}
    title="Alternar Tela Cheia (Tecla F)"
    class="absolute bottom-3 right-3 p-2 bg-zinc-900/60 hover:bg-zinc-800 text-zinc-400 hover:text-white rounded-md opacity-20 hover:opacity-100 transition-opacity z-30"
  >
    {#if isFullscreen}
      <Minimize size={18} />
    {:else}
      <Maximize size={18} />
    {/if}
  </button>
</main>

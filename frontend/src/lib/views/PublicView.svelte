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

  const wallpaper = $derived($presentationState.wallpaper_path);
  const wallpaperFit = $derived($presentationState.wallpaper_mode || 'cover');
</script>

<svelte:window onkeydown={(e) => { if (e.key.toLowerCase() === 'f') toggleFullscreen(); }} />

<!-- Container fullscreen de projeção pública -->
<main
  class="relative w-screen h-screen bg-black text-white flex items-center justify-center overflow-hidden select-none"
  ondblclick={toggleFullscreen}
>
  {#if $presentationState.display_mode === 'blackout'}
    <!-- Blackout total: tela preta absoluta -->
    <div class="absolute inset-0 bg-black z-30"></div>
  {:else}
    <!-- Fundo / Papel de Parede da Setlist Ativa -->
    {#if wallpaper}
      <div class="absolute inset-0 z-0 overflow-hidden flex items-center justify-center pointer-events-none">
        {#if wallpaperFit === 'repeat'}
          <div
            class="w-full h-full"
            style="background-image: url('{wallpaper}'); background-repeat: repeat;"
          ></div>
        {:else if wallpaperFit === 'contain'}
          <img
            src={wallpaper}
            alt="Plano de Fundo"
            class="w-full h-full object-contain"
          />
        {:else if wallpaperFit === 'center'}
          <img
            src={wallpaper}
            alt="Plano de Fundo"
            class="max-w-full max-h-full object-none"
          />
        {:else}
          <!-- cover (padrão) -->
          <img
            src={wallpaper}
            alt="Plano de Fundo"
            class="w-full h-full object-cover"
          />
        {/if}

        <!-- Overlay escuro sutil para garantir legibilidade quando houver slide -->
        {#if activeSlide}
          <div class="absolute inset-0 bg-black/60 backdrop-blur-[2px]"></div>
        {/if}
      </div>
    {/if}

    <!-- Camada de Conteúdo do Slide ou Repouso -->
    <div class="relative z-10 w-full h-full flex items-center justify-center">
      {#if $presentationState.display_mode === 'cleartext'}
        <!-- Clear Text: texto oculto, mantém papel de parede visível -->
        {#if !wallpaper}
          <div class="text-neutral-700 text-sm font-light select-none opacity-25">
            OpenPresenter
          </div>
        {/if}
      {:else if activeSlide}
        <!-- Slide Ao Vivo -->
        <div class="w-full h-full max-w-[95vw] max-h-[92vh] aspect-video flex flex-col items-center justify-center p-8 sm:p-14 text-center animate-in fade-in duration-150">
          <div class="space-y-4 my-auto">
            {#each activeSlide.content.split('\n') as line}
              <p class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight text-white drop-shadow-[0_4px_20px_rgba(0,0,0,0.95)] leading-tight">
                {line}
              </p>
            {/each}
          </div>

          {#if activeMedia?.artist_or_ref}
            <div class="text-neutral-300 text-sm md:text-base font-medium mt-auto opacity-80 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
              {activeMedia.title} — {activeMedia.artist_or_ref}
            </div>
          {/if}
        </div>
      {:else if !wallpaper}
        <!-- Estado de espera padrão sem mídia ativa nem wallpaper -->
        <div class="flex flex-col items-center justify-center text-center text-neutral-500 gap-3">
          <img
            src="/branding/open-presenter-app-icon.svg"
            alt="OpenPresenter"
            class="w-16 h-16 rounded-2xl shadow-xl opacity-80"
          />
          <img
            src="/branding/open-presenter-wordmark-dark.svg"
            alt="OpenPresenter"
            class="h-6 opacity-75 mt-1"
          />
          <p class="text-xs text-neutral-600 mt-1">Aguardando disparo de slide pelo operador</p>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Botão sutil de tela cheia no canto inferior direito -->
  <button
    type="button"
    onclick={toggleFullscreen}
    title="Alternar Tela Cheia (Tecla F)"
    class="absolute bottom-3 right-3 p-2 bg-neutral-900/60 hover:bg-neutral-800 text-neutral-400 hover:text-white rounded-md opacity-20 hover:opacity-100 transition-opacity z-40 cursor-pointer"
  >
    {#if isFullscreen}
      <Minimize size={18} />
    {:else}
      <Maximize size={18} />
    {/if}
  </button>
</main>

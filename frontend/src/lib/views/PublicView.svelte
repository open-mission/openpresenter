<script lang="ts">
  import { onMount } from 'svelte';
  import { presentationState, mediaItemsList, initWebSocket } from '$lib/ws';
  import type { MediaItem, Slide } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
  import { resolveBackground } from '$lib/backgrounds';
  import {
    publicSettingsStore,
    churchBrandStore,
    syncDisplaySettingsFromBackend,
  } from '$lib/displaySettings';
  import { Maximize, Minimize } from 'lucide-svelte';

  let isFullscreen = $state(false);

  onMount(() => {
    initWebSocket('public');
    syncDisplaySettingsFromBackend();

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

  const alignH = $derived(activeSlide?.align_h || activeMedia?.align_h || 'center');
  const alignV = $derived(activeSlide?.align_v || activeMedia?.align_v || 'center');

  const wallpaper = $derived($presentationState.wallpaper_path);
  const wallpaperFit = $derived($presentationState.wallpaper_mode || 'cover');
  const resolvedBg = $derived(resolveBackground(wallpaper));

  const publicSettings = $derived($publicSettingsStore);
  const churchBrand = $derived($churchBrandStore);

  const activeLogoUrl = $derived.by<string>(() => {
    if (!churchBrand.show_logo_on_public) return '';
    const variant = churchBrand.logo_variant || 'primary';
    if (variant === 'primary' && churchBrand.logo_primary) return churchBrand.logo_primary;
    if (variant === 'dark' && churchBrand.logo_dark) return churchBrand.logo_dark;
    if (variant === 'light' && churchBrand.logo_light) return churchBrand.logo_light;
    if (variant === 'full' && churchBrand.logo_full) return churchBrand.logo_full;
    return churchBrand.logo_primary || churchBrand.logo_light || churchBrand.logo_dark || churchBrand.logo_full || '';
  });

  const logoPositionClasses = $derived.by<string>(() => {
    switch (churchBrand.logo_position) {
      case 'top-left':
        return 'top-6 left-6';
      case 'bottom-left':
        return 'bottom-6 left-6';
      case 'bottom-right':
        return 'bottom-6 right-6';
      case 'top-right':
      default:
        return 'top-6 right-6';
    }
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
    <div class="absolute inset-0 bg-black z-30"></div>
  {:else}
    <!-- Fundo / Papel de Parede da Setlist Ativa -->
    {#if resolvedBg.isCss || resolvedBg.isImage}
      <div class="absolute inset-0 z-0 overflow-hidden flex items-center justify-center pointer-events-none">
        {#if resolvedBg.isCss}
          <div
            class="w-full h-full {resolvedBg.className}"
            style={resolvedBg.style}
          ></div>
        {:else if wallpaperFit === 'repeat'}
          <div
            class="w-full h-full"
            style="background-image: url('{resolvedBg.url}'); background-repeat: repeat;"
          ></div>
        {:else if wallpaperFit === 'contain'}
          <img
            src={resolvedBg.url}
            alt="Plano de Fundo"
            class="w-full h-full object-contain"
          />
        {:else if wallpaperFit === 'center'}
          <img
            src={resolvedBg.url}
            alt="Plano de Fundo"
            class="max-w-full max-h-full object-none"
          />
        {:else}
          <!-- cover (padrão) -->
          <img
            src={resolvedBg.url}
            alt="Plano de Fundo"
            class="w-full h-full object-cover"
          />
        {/if}

        <!-- Overlay escuro sutil para garantir legibilidade quando houver slide -->
        {#if activeSlide && activeMedia?.kind !== 'image'}
          <div class="absolute inset-0 bg-black/60 backdrop-blur-[2px]"></div>
        {/if}
      </div>
    {/if}

    <!-- Logotipo da Igreja sobreposto na tela pública -->
    {#if activeLogoUrl}
      <div
        class="absolute pointer-events-none z-20 {logoPositionClasses} transition-all duration-200"
        style="opacity: {churchBrand.logo_opacity / 100};"
      >
        <img
          src={activeLogoUrl}
          alt={churchBrand.name || 'Logotipo da Igreja'}
          class="object-contain drop-shadow-[0_2px_12px_rgba(0,0,0,0.8)]"
          style="max-width: {churchBrand.logo_size}px; max-height: {churchBrand.logo_size}px;"
        />
      </div>
    {/if}

    <!-- Camada de Conteúdo do Slide ou Repouso com Margens Personalizadas -->
    <div
      class="relative z-10 w-full h-full flex items-center justify-center"
      style="padding-top: {publicSettings.margin_top}px; padding-bottom: {publicSettings.margin_bottom}px; padding-left: {publicSettings.margin_horizontal}px; padding-right: {publicSettings.margin_horizontal}px;"
    >
      {#if $presentationState.display_mode === 'cleartext'}
        <!-- Clear Text: texto oculto, mantém papel de parede visível -->
        {#if !wallpaper}
          <div class="text-neutral-700 text-sm font-light select-none opacity-25">
            OpenPresenter
          </div>
        {/if}
      {:else if activeSlide}
        <!-- Mídia do Tipo Imagem Direta -->
        {#if activeMedia?.kind === 'image'}
          <div class="w-full h-full flex items-center justify-center p-2">
            <img
              src={activeSlide.content}
              alt={activeMedia.title}
              class="max-w-full max-h-full object-contain rounded shadow-2xl animate-in fade-in duration-200"
            />
          </div>
        {:else}
          <!-- Slide de Texto / Música Ao Vivo com Formatação, Margens e Preenchimento -->
          <div
            class="w-full h-full flex flex-col animate-in fade-in duration-150 {alignV === 'top' ? 'justify-start' : alignV === 'bottom' ? 'justify-end' : 'justify-center'} {alignH === 'left' ? 'text-left items-start' : alignH === 'right' ? 'text-right items-end' : alignH === 'justify' ? 'text-justify items-stretch' : 'text-center items-center'}"
          >
            <div class="space-y-4 my-auto {publicSettings.fill_space ? 'w-full max-w-[96vw]' : 'max-w-5xl'}">
              <div
                class="font-bold tracking-tight text-white drop-shadow-[0_4px_24px_rgba(0,0,0,0.95)] leading-tight {publicSettings.fill_space ? 'text-4xl sm:text-6xl md:text-7xl lg:text-8xl' : 'text-3xl sm:text-4xl md:text-5xl lg:text-6xl'} {publicSettings.word_wrap ? 'break-words' : 'whitespace-nowrap overflow-hidden text-ellipsis'}"
              >
                {@html renderFormattedText(activeSlide.content)}
              </div>
            </div>

            {#if publicSettings.show_metadata && (activeMedia?.title || activeMedia?.artist_or_ref)}
              <div class="text-neutral-300 text-sm md:text-base font-medium mt-auto opacity-80 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
                {activeMedia.title}{activeMedia.artist_or_ref ? ` — ${activeMedia.artist_or_ref}` : ''}
              </div>
            {/if}
          </div>
        {/if}
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

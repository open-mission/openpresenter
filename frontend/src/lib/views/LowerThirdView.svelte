<script lang="ts">
  import { onMount } from 'svelte';
  import {
    presentationState,
    initWebSocket,
  } from '$lib/ws';
  import {
    lowerThirdSettingsStore,
    backgroundStylesStore,
    syncDisplaySettingsFromBackend,
  } from '$lib/displaySettings';
  import { resolveBackground } from '$lib/backgrounds';
  import { renderFormattedText } from '$lib/markdown';
  import type { Slide, MediaItem } from '$lib/types';

  onMount(() => {
    initWebSocket('lower-third');
    syncDisplaySettingsFromBackend();
  });

  const lt = $derived($lowerThirdSettingsStore);

  // Determinar se o conteúdo deve estar visível
  const isVisible = $derived.by(() => {
    if (!lt.enabled) return false;
    if ($presentationState.display_mode === 'blackout') return false;
    if ($presentationState.display_mode === 'cleartext') return false;
    if (!$presentationState.active_media_id) return false;
    return true;
  });

  // Slide atual
  const activeMedia = $derived.by<MediaItem | null>(() => {
    if ($presentationState.quick_media) {
      return $presentationState.quick_media;
    }
    return null;
  });

  const currentSlide = $derived.by<Slide | null>(() => {
    if (!isVisible) return null;
    const media = activeMedia;
    const idx = $presentationState.active_slide_index ?? 0;
    if (media && media.slides && media.slides[idx]) {
      return media.slides[idx];
    }
    return null;
  });

  const isScripture = $derived(activeMedia?.kind === 'scripture');

  // Informações de título e referência
  const mediaLabel = $derived.by(() => {
    if (!activeMedia) return '';
    if (isScripture) {
      return activeMedia.title || currentSlide?.label || 'Bíblia';
    }
    return activeMedia.title || '';
  });

  const subLabel = $derived.by(() => {
    if (!activeMedia) return '';
    return activeMedia.artist_or_ref || '';
  });

  // Cor de fundo do Lower Third
  const resolvedBg = $derived(
    resolveBackground($presentationState.wallpaper_path, $backgroundStylesStore)
  );

  const effectiveBgColor = $derived.by(() => {
    return lt.bg_color || '#000000';
  });

  // Converte HEX para RGBA com opacidade
  function hexToRgba(hex: string, opacityPercent: number): string {
    let clean = hex.replace('#', '');
    if (clean.length === 3) {
      clean = clean.split('').map((c) => c + c).join('');
    }
    if (clean.length !== 6) {
      return `rgba(0, 0, 0, ${opacityPercent / 100})`;
    }
    const r = parseInt(clean.substring(0, 2), 16);
    const g = parseInt(clean.substring(2, 4), 16);
    const b = parseInt(clean.substring(4, 6), 16);
    const a = (opacityPercent / 100).toFixed(2);
    return `rgba(${r}, ${g}, ${b}, ${a})`;
  }

  const bgRgba = $derived(hexToRgba(effectiveBgColor, lt.bg_opacity ?? 75));
  const bgRgbaTransparent = $derived(hexToRgba(effectiveBgColor, 0));

  // Estilo de sombra do texto para contraste no vídeo
  const textShadowStyle = $derived.by(() => {
    switch (lt.shadow_strength) {
      case 'none':
        return 'none';
      case 'subtle':
        return '0 2px 4px rgba(0, 0, 0, 0.6)';
      case 'medium':
        return '0 2px 6px rgba(0, 0, 0, 0.85), 0 0 2px rgba(0, 0, 0, 0.9)';
      case 'strong':
      default:
        return '0 3px 12px rgba(0, 0, 0, 0.95), 0 0 4px rgba(0, 0, 0, 1), 0 1px 2px rgba(0, 0, 0, 1)';
    }
  });

  // Animação de transição
  const animationClass = $derived.by(() => {
    if (!isVisible) {
      return 'opacity-0 translate-y-6 pointer-events-none';
    }
    switch (lt.animation) {
      case 'none':
        return 'opacity-100';
      case 'slide':
        return 'opacity-100 translate-y-0 transition-all duration-300 ease-out';
      case 'zoom':
        return 'opacity-100 scale-100 transition-all duration-300 ease-out';
      case 'fade':
      default:
        return 'opacity-100 transition-opacity duration-250 ease-in-out';
    }
  });
</script>

<!--
  Contêiner raiz da saída Lower Third.
  A classe .lower-third-root sinaliza ao app.css para aplicar background: transparent !important.
-->
<div
  class="lower-third-root fixed inset-0 w-screen h-screen overflow-hidden flex flex-col justify-end select-none pointer-events-none font-sans"
  style="background: transparent;"
>
  <!-- Caixa do Lower Third ancorada no terço inferior da tela -->
  <div
    class="w-full flex flex-col items-center justify-end transition-all duration-300"
    style="padding-bottom: {lt.margin_bottom ?? 36}px;"
  >
    <div
      class="w-full max-w-6xl px-6 transition-all duration-300 {animationClass}"
      style="max-width: {lt.max_width_percent ?? 85}%;"
    >
      {#if isVisible && currentSlide}
        <!-- ESTILO 1: GRADIENTE CENTRAL (Padrão de Transmissão Profissional) -->
        {#if lt.theme === 'gradient'}
          <div
            class="relative px-8 py-5 rounded-2xl flex flex-col items-center justify-center text-center overflow-hidden"
            style="background: radial-gradient(ellipse at center, {bgRgba} 0%, {bgRgba} 55%, {bgRgbaTransparent} 100%);"
          >
            <!-- Badge de Referência Bíblica ou Música -->
            {#if lt.show_reference && mediaLabel}
              <div
                class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-2.5 bg-black/60 border border-white/20 text-white shadow-md backdrop-blur-md"
              >
                <span>{isScripture ? '📖' : '🎵'}</span>
                <span>{mediaLabel}</span>
                {#if subLabel}
                  <span class="opacity-70 font-normal">({subLabel})</span>
                {/if}
              </div>
            {/if}

            <!-- Texto Principal do Slide -->
            <div
              class="w-full text-center leading-snug tracking-normal {lt.text_align === 'left' ? 'text-left' : lt.text_align === 'right' ? 'text-right' : 'text-center'}"
              style="font-size: {lt.font_size ?? 32}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
            >
              {@html renderFormattedText(currentSlide.content)}
            </div>
          </div>

        <!-- ESTILO 2: BARRA INFERIOR (Lower Third Estilo Noticiário / TV) -->
        {:else if lt.theme === 'bar'}
          <div
            class="w-full px-8 py-4 rounded-xl border border-white/10 shadow-2xl backdrop-blur-md flex flex-col {lt.text_align === 'left' ? 'items-start text-left' : lt.text_align === 'right' ? 'items-end text-right' : 'items-center text-center'}"
            style="background-color: {bgRgba}; border-left: 6px solid {resolvedBg.textColor || '#3b82f6'};"
          >
            {#if lt.show_reference && mediaLabel}
              <div class="text-xs uppercase font-extrabold tracking-wider mb-1 text-emerald-400 drop-shadow">
                {isScripture ? '📖 ' : '🎵 '}{mediaLabel} {subLabel ? `• ${subLabel}` : ''}
              </div>
            {/if}

            <div
              class="w-full leading-snug"
              style="font-size: {lt.font_size ?? 32}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
            >
              {@html renderFormattedText(currentSlide.content)}
            </div>
          </div>

        <!-- ESTILO 3: CAIXA FLUTUANTE (Glassmorphism Pill) -->
        {:else if lt.theme === 'floating'}
          <div
            class="mx-auto px-8 py-4 rounded-3xl border border-white/20 shadow-2xl backdrop-blur-lg flex flex-col items-center text-center"
            style="background-color: {bgRgba};"
          >
            {#if lt.show_reference && mediaLabel}
              <div class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-bold uppercase mb-2 bg-white/15 text-white/90">
                {isScripture ? '📖 ' : '🎵 '}{mediaLabel}
              </div>
            {/if}

            <div
              class="w-full leading-snug"
              style="font-size: {lt.font_size ?? 32}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
            >
              {@html renderFormattedText(currentSlide.content)}
            </div>
          </div>

        <!-- ESTILO 4: MINIMALISTA (Sem caixa de fundo, apenas tipografia de alto impacto) -->
        {:else}
          <div class="w-full flex flex-col items-center text-center">
            {#if lt.show_reference && mediaLabel}
              <span
                class="text-sm uppercase font-bold tracking-widest mb-1 text-amber-300"
                style="text-shadow: {textShadowStyle};"
              >
                {isScripture ? '📖 ' : '🎵 '}{mediaLabel}
              </span>
            {/if}

            <div
              class="w-full leading-snug"
              style="font-size: {lt.font_size ?? 32}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
            >
              {@html renderFormattedText(currentSlide.content)}
            </div>
          </div>
        {/if}
      {/if}
    </div>
  </div>
</div>

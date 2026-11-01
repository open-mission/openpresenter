<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { presentationState, mediaItemsList, initWebSocket } from '$lib/ws';
  import type { MediaItem, Slide } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
  import { resolveBackground } from '$lib/backgrounds';
  import { autoScale } from '$lib/autoScale';
  import {
    publicSettingsStore,
    churchBrandStore,
    backgroundStylesStore,
    syncDisplaySettingsFromBackend,
  } from '$lib/displaySettings';
  import { resolveMediaUrl } from '$lib/cast';
  import { Maximize, Minimize, Clock } from 'lucide-svelte';

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
    if (p.quick_media && p.quick_media.id === p.active_media_id) {
      return p.quick_media;
    }
    return $mediaItemsList.find((m) => m.id === p.active_media_id);
  });

  const activeSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!activeMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    return activeMedia.slides[p.active_slide_index];
  });

  const isScripture = $derived(activeMedia?.kind === 'scripture');
  const isSchedule = $derived(activeMedia?.kind === 'schedule');

  function parseScheduleContent(label: string, content: string) {
    const lines = (content || '').split('\n').map((l) => l.trim()).filter(Boolean);
    let time = '';
    let title = '';
    let details = '';

    if (lines.length >= 3) {
      time = lines[0];
      title = lines[1];
      details = lines.slice(2).join(' • ');
    } else if (lines.length === 2) {
      if (/\d+[:hH]/.test(lines[0])) {
        time = lines[0];
        title = lines[1];
      } else {
        title = lines[0];
        details = lines[1];
      }
    } else if (lines.length === 1) {
      title = lines[0];
    }

    return {
      day: label || 'Programação',
      time,
      title: title || label,
      details,
    };
  }
  const publicSettings = $derived($publicSettingsStore);
  const churchBrand = $derived($churchBrandStore);

  const alignH = $derived.by<'left' | 'center' | 'right' | 'justify'>(() => {
    if (isScripture) {
      return publicSettings?.scripture_align_h || 'center';
    }
    return (activeSlide?.align_h || activeMedia?.align_h || publicSettings?.lyrics_align_h || 'center') as any;
  });

  const alignV = $derived.by<'top' | 'center' | 'bottom'>(() => {
    if (isScripture) {
      return publicSettings?.scripture_align_v || 'center';
    }
    return (activeSlide?.align_v || activeMedia?.align_v || publicSettings?.lyrics_align_v || 'center') as any;
  });

  const minFontSize = $derived(
    isScripture
      ? (publicSettings?.scripture_min_font_size ?? 28)
      : (publicSettings?.lyrics_min_font_size ?? publicSettings?.min_font_size ?? 24)
  );

  const maxFontSize = $derived(
    isScripture
      ? (publicSettings?.scripture_max_font_size ?? 140)
      : (publicSettings?.lyrics_max_font_size ?? publicSettings?.max_font_size ?? 128)
  );

  const fillSpace = $derived(
    isScripture
      ? (publicSettings?.scripture_fill_space !== false)
      : (publicSettings?.fill_space !== false)
  );

  const wallpaper = $derived($presentationState.wallpaper_path);
  const wallpaperFit = $derived(publicSettings?.wallpaper_fit || $presentationState.wallpaper_mode || 'contain');
  const resolvedBg = $derived(resolveBackground(wallpaper, $backgroundStylesStore));
  const activeTextColor = $derived(resolvedBg.textColor || '#ffffff');

  const marginTop = $derived(Number(publicSettings?.margin_top ?? 32));
  const marginBottom = $derived(Number(publicSettings?.margin_bottom ?? 32));
  const marginHorizontal = $derived(Number(publicSettings?.margin_horizontal ?? 48));

  const isImageMedia = $derived(activeMedia?.kind === 'image');
  const imageFadeDuration = $derived.by(() => {
    if (publicSettings?.image_fade_enabled === false) return 0;
    return publicSettings?.image_fade_duration_ms ?? 500;
  });
  const applyMargins = $derived(!isImageMedia || !publicSettings.ignore_margins_for_media);
  const imageFitClass = $derived.by(() => {
    switch (publicSettings.image_fit) {
      case 'cover':
        return 'w-full h-full object-cover';
      case 'fill':
        return 'w-full h-full object-fill';
      case 'center':
        return 'max-w-full max-h-full object-none';
      case 'contain':
      default:
        return 'w-full h-full object-contain';
    }
  });

  const activeLogoUrl = $derived.by<string>(() => {
    if (!churchBrand.show_logo_on_public) return '';
    const variant = churchBrand.logo_variant || 'primary';
    if (variant === 'primary' && churchBrand.logo_primary) return churchBrand.logo_primary;
    if (variant === 'dark' && churchBrand.logo_dark) return churchBrand.logo_dark;
    if (variant === 'light' && churchBrand.logo_light) return churchBrand.logo_light;
    if (variant === 'full' && churchBrand.logo_full) return churchBrand.logo_full;
    return churchBrand.logo_primary || churchBrand.logo_light || churchBrand.logo_dark || churchBrand.logo_full || '';
  });

  const logoPositionStyles = $derived.by<string>(() => {
    const mx = churchBrand.logo_margin_horizontal ?? 24;
    const my = churchBrand.logo_margin_vertical ?? 24;
    switch (churchBrand.logo_position) {
      case 'top-left':
        return `top: ${my}px; left: ${mx}px;`;
      case 'bottom-left':
        return `bottom: ${my}px; left: ${mx}px;`;
      case 'bottom-right':
        return `bottom: ${my}px; right: ${mx}px;`;
      case 'top-right':
      default:
        return `top: ${my}px; right: ${mx}px;`;
    }
  });

  function getScriptureInfo(label: string | undefined, artistOrRef: string | undefined) {
    let ref = label || activeMedia?.title || 'Bíblia';
    const version = artistOrRef?.trim() || '';
    if (version && ref.endsWith(version)) {
      ref = ref.slice(0, -version.length).trim();
    }
    return { ref, version };
  }
</script>

<svelte:window onkeydown={(e) => { if (e.key.toLowerCase() === 'f') toggleFullscreen(); }} />

<!-- Container fullscreen de projeção pública com estilos críticos inline anti-flicker -->
<main
  class="relative w-full h-full min-w-full min-h-full bg-black text-white flex items-center justify-center overflow-hidden select-none"
  style="background-color: #000; color: #fff; width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; overflow: hidden;"
  ondblclick={toggleFullscreen}
>
  {#if $presentationState.display_mode === 'blackout'}
    <!-- Blackout total: tela preta absoluta -->
    <div class="absolute inset-0 bg-black z-30"></div>
  {:else}
    <!-- Fundo / Papel de Parede da Setlist Ativa -->
    {#if resolvedBg.isCss || resolvedBg.isImage || resolvedBg.isVideo}
      <div
        class="absolute inset-0 z-0 overflow-hidden flex items-center justify-center pointer-events-none"
        style={publicSettings?.wallpaper_safe_margins
          ? `box-sizing: border-box; padding-top: ${marginTop}px; padding-bottom: ${marginBottom}px; padding-left: ${marginHorizontal}px; padding-right: ${marginHorizontal}px;`
          : ''}
      >
        {#if resolvedBg.isCss}
          <div
            class="w-full h-full {resolvedBg.className}"
            style="{resolvedBg.style}; {publicSettings?.wallpaper_safe_margins ? 'border-radius: 1rem;' : ''}"
          ></div>
        {:else if resolvedBg.isVideo}
          <video
            src={resolveMediaUrl(resolvedBg.url)}
            class="w-full h-full {wallpaperFit === 'contain' ? 'object-contain' : wallpaperFit === 'fill' ? 'object-fill' : 'object-cover'} {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
            autoplay
            loop
            muted
            playsinline
          ></video>
        {:else if wallpaperFit === 'repeat'}
          <div
            class="w-full h-full {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl overflow-hidden' : ''}"
            style="background-image: url('{resolveMediaUrl(resolvedBg.url)}'); background-repeat: repeat;"
          ></div>
        {:else if wallpaperFit === 'contain'}
          <img
            src={resolveMediaUrl(resolvedBg.url)}
            alt="Plano de Fundo"
            class="w-full h-full object-contain {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
          />
        {:else if wallpaperFit === 'fill'}
          <img
            src={resolveMediaUrl(resolvedBg.url)}
            alt="Plano de Fundo"
            class="w-full h-full object-fill {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
          />
        {:else if wallpaperFit === 'center'}
          <img
            src={resolveMediaUrl(resolvedBg.url)}
            alt="Plano de Fundo"
            class="max-w-full max-h-full object-none {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
          />
        {:else}
          <!-- cover (padrão) -->
          <img
            src={resolveMediaUrl(resolvedBg.url)}
            alt="Plano de Fundo"
            class="w-full h-full object-cover {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
          />
        {/if}

        <!-- Overlay escuro opcional sobre o plano de fundo geral -->
        {#if publicSettings.darken_background && activeSlide && activeMedia?.kind !== 'image'}
          <div class="absolute inset-0 bg-black/60 backdrop-blur-[2px] transition-all duration-300 {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"></div>
        {/if}
      </div>
    {/if}

    <!-- Logotipo da Igreja sobreposto na tela pública -->
    {#if activeLogoUrl}
      <div
        class="absolute pointer-events-none z-20 transition-all duration-200"
        style="{logoPositionStyles} opacity: {churchBrand.logo_opacity / 100};"
      >
        <img
          src={resolveMediaUrl(activeLogoUrl)}
          alt={churchBrand.name || 'Logotipo da Igreja'}
          class="object-contain drop-shadow-[0_2px_12px_rgba(0,0,0,0.8)]"
          style="max-width: {churchBrand.logo_size}px; max-height: {churchBrand.logo_size}px;"
        />
      </div>
    {/if}

    <!-- Camada de Conteúdo do Slide ou Repouso com Margens Personalizadas -->
    <div
      class="relative z-10 w-full h-full box-border flex flex-col items-center justify-center overflow-hidden"
      style={applyMargins
        ? `box-sizing: border-box; padding-top: ${marginTop}px; padding-bottom: ${marginBottom}px; padding-left: ${marginHorizontal}px; padding-right: ${marginHorizontal}px;`
        : 'box-sizing: border-box;'}
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
          {#key activeSlide.content}
            <div
              transition:fade={{ duration: imageFadeDuration }}
              class="w-full flex-1 min-h-0 flex items-center justify-center overflow-hidden pointer-events-none"
            >
              <img
                src={resolveMediaUrl(activeSlide.content)}
                alt={activeMedia.title}
                class={imageFitClass}
              />
            </div>
          {/key}
        {:else if isSchedule}
          <!-- Agenda / Programação de Cultos e Festividades -->
          {@const ev = parseScheduleContent(activeSlide.label, activeSlide.content)}
          {@const showBoxBg = publicSettings.schedule_box_background ?? true}
          {@const boxOpacity = (publicSettings.schedule_bg_opacity ?? 65) / 100}
          {@const maxWidthPct = publicSettings.schedule_max_width_percent ?? 80}
          {@const padClass = publicSettings.schedule_padding === 'compact'
            ? 'p-5 sm:p-7 md:p-9 gap-4'
            : publicSettings.schedule_padding === 'spacious'
              ? 'p-10 sm:p-16 md:p-20 gap-8'
              : 'p-8 sm:p-12 md:p-16 gap-6'}
          {@const schedAlign = publicSettings.schedule_align_h ?? 'center'}
          <div class="w-full flex-1 h-full min-h-0 min-w-0 flex flex-col items-center justify-center overflow-hidden animate-in fade-in duration-150 select-none">
            <div
              class="w-full rounded-3xl transition-all shadow-2xl flex flex-col justify-center {padClass} {schedAlign === 'left' ? 'items-start text-left' : schedAlign === 'right' ? 'items-end text-right' : 'items-center text-center'}"
              style="max-width: {maxWidthPct}%; {showBoxBg ? `background-color: rgba(0, 0, 0, ${boxOpacity}); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border: 1px solid rgba(255, 255, 255, 0.15);` : 'background: transparent; border: none; box-shadow: none;'}"
            >
              <!-- Badges de Topo: Dia da Semana e Horário -->
              {#if publicSettings.schedule_show_badges !== false}
                <div class="flex items-center gap-3 flex-wrap {schedAlign === 'left' ? 'justify-start' : schedAlign === 'right' ? 'justify-end' : 'justify-center'}">
                  <span
                    class="px-5 py-1.5 rounded-full bg-amber-500/25 text-amber-300 border border-amber-500/50 font-black uppercase tracking-wider shadow-lg"
                    style="font-size: {publicSettings.schedule_day_font_size ?? 20}px;"
                  >
                    {ev.day}
                  </span>
                  {#if ev.time}
                    <span
                      class="px-4 py-1.5 rounded-full bg-white/10 text-white border border-white/20 font-mono font-bold flex items-center gap-2 shadow-lg"
                      style="font-size: {publicSettings.schedule_time_font_size ?? 18}px;"
                    >
                      <Clock size={Math.round((publicSettings.schedule_time_font_size ?? 18) * 1.0)} class="text-amber-400" />
                      <span>{ev.time}</span>
                    </span>
                  {/if}
                </div>
              {/if}

              <!-- Título do Evento com Auto-Scale -->
              <div class="w-full flex-1 min-h-[100px] max-h-[380px] flex items-center {schedAlign === 'left' ? 'justify-start' : schedAlign === 'right' ? 'justify-end' : 'justify-center'} overflow-hidden">
                <div
                  use:autoScale={{
                    minSize: publicSettings.schedule_title_min_font_size ?? 32,
                    maxSize: publicSettings.schedule_title_max_font_size ?? 110,
                  }}
                  style="color: {activeTextColor};"
                  class="w-full font-black tracking-tight leading-tight drop-shadow-[0_4px_24px_rgba(0,0,0,0.95)] {schedAlign === 'left' ? 'text-left' : schedAlign === 'right' ? 'text-right' : 'text-center'}"
                >
                  {ev.title}
                </div>
              </div>

              <!-- Detalhes do Evento -->
              {#if ev.details}
                <div
                  class="text-neutral-300 font-medium tracking-wide opacity-90 drop-shadow-[0_2px_12px_rgba(0,0,0,0.9)] max-w-3xl {schedAlign === 'left' ? 'text-left' : schedAlign === 'right' ? 'text-right' : 'text-center'}"
                  style="font-size: {publicSettings.schedule_details_font_size ?? 20}px;"
                >
                  {ev.details}
                </div>
              {/if}

              <!-- Identificador da Agenda no Rodapé -->
              {#if (publicSettings.schedule_show_footer !== false) && activeMedia?.title}
                <div class="pt-4 border-t border-white/10 text-xs sm:text-sm text-neutral-400 uppercase tracking-widest font-semibold opacity-75 {schedAlign === 'left' ? 'text-left w-full' : schedAlign === 'right' ? 'text-right w-full' : 'text-center'}">
                  {activeMedia.title}
                </div>
              {/if}
            </div>
          </div>
        {:else}
          <!-- Slide de Texto / Música / Bíblia Ao Vivo com Formatação, Margens e Auto-scale -->
          <div
            class="w-full flex-1 h-full min-h-0 min-w-0 flex flex-col overflow-hidden animate-in fade-in duration-150 {alignH === 'left' ? 'text-left items-start' : alignH === 'right' ? 'text-right items-end' : alignH === 'justify' ? 'text-justify items-stretch' : 'text-center items-center'}"
          >
            <div
              class="flex flex-col justify-between w-full h-full flex-1 min-h-0 overflow-hidden {fillSpace ? 'w-full' : 'max-w-5xl'} {publicSettings.text_box_background ? 'bg-black/65 backdrop-blur-md rounded-3xl p-6 sm:p-10 md:p-14 border border-white/10 shadow-2xl transition-all' : ''}"
            >
              <div class="w-full flex-1 min-h-0 flex flex-col overflow-hidden {alignV === 'top' ? 'justify-start' : alignV === 'bottom' ? 'justify-end' : 'justify-center'}">
                <div
                  use:autoScale={{
                    minSize: minFontSize,
                    maxSize: maxFontSize,
                  }}
                  style="color: {activeTextColor};"
                  class="w-full font-bold tracking-tight drop-shadow-[0_4px_24px_rgba(0,0,0,0.95)] {isScripture ? 'leading-snug' : 'leading-tight'} select-none {alignH === 'left' ? 'text-left' : alignH === 'right' ? 'text-right' : alignH === 'justify' ? 'text-justify' : 'text-center'} {publicSettings.word_wrap ? 'whitespace-normal break-words' : 'whitespace-nowrap'}"
                >
                  {@html renderFormattedText(activeSlide.content)}
                </div>

                {#if isScripture && (publicSettings.scripture_show_reference !== false) && publicSettings.scripture_reference_position === 'below_text'}
                  {@const { ref, version } = getScriptureInfo(activeSlide.label, activeMedia?.artist_or_ref)}
                  <div
                    class="mt-4 sm:mt-6 flex items-center {alignH === 'left' ? 'justify-start' : alignH === 'right' ? 'justify-end' : 'justify-center'} gap-3 drop-shadow-[0_2px_12px_rgba(0,0,0,0.9)] shrink-0"
                  >
                    <span
                      style="color: {activeTextColor}; font-size: {Math.max(16, (publicSettings.scripture_reference_font_size ?? 28) * 0.9)}px;"
                      class="font-bold tracking-tight opacity-90"
                    >
                      {ref}
                    </span>
                    {#if version}
                      <span
                        style="font-size: {Math.max(11, Math.round((publicSettings.scripture_reference_font_size ?? 28) * 0.55))}px;"
                        class="px-2.5 py-0.5 rounded-md bg-white/15 border border-white/20 uppercase font-mono font-bold tracking-wider text-neutral-100 shadow-md"
                      >
                        {version}
                      </span>
                    {/if}
                  </div>
                {/if}
              </div>

              {#if isScripture && (publicSettings.scripture_show_reference !== false) && (publicSettings.scripture_reference_position ?? 'footer') === 'footer'}
                {@const { ref, version } = getScriptureInfo(activeSlide.label, activeMedia?.artist_or_ref)}
                <div
                  class="w-full mt-auto pt-4 sm:pt-6 flex items-center justify-between gap-4 border-t border-white/10 shrink-0 drop-shadow-[0_2px_12px_rgba(0,0,0,0.9)]"
                >
                  <span
                    style="color: {activeTextColor}; font-size: {publicSettings.scripture_reference_font_size ?? 28}px;"
                    class="font-bold tracking-tight"
                  >
                    {ref}
                  </span>
                  {#if version}
                    <span
                      style="font-size: {Math.max(12, Math.round((publicSettings.scripture_reference_font_size ?? 28) * 0.55))}px;"
                      class="px-3 py-1 rounded-lg bg-white/15 border border-white/20 uppercase font-mono font-bold tracking-wider text-neutral-100 shadow-md"
                    >
                      {version}
                    </span>
                  {/if}
                </div>
              {:else if !isScripture && publicSettings.show_metadata && (activeMedia?.title || activeMedia?.artist_or_ref)}
                <div class="text-neutral-300 text-sm md:text-base font-medium mt-4 sm:mt-6 opacity-80 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
                  {activeMedia.title}{activeMedia.artist_or_ref ? ` — ${activeMedia.artist_or_ref}` : ''}
                </div>
              {/if}
            </div>
          </div>
        {/if}
      {:else if !wallpaper}
        <!-- Estado de espera padrão sem mídia ativa nem wallpaper -->
        <div class="flex flex-col items-center justify-center text-center text-neutral-500 gap-3">
          <img
            src={resolveMediaUrl('/branding/open-presenter-app-icon.svg')}
            alt="OpenPresenter"
            class="w-16 h-16 rounded-2xl shadow-xl opacity-80"
          />
          <img
            src={resolveMediaUrl('/branding/open-presenter-wordmark-dark.svg')}
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
    class="absolute bottom-3 right-3 p-2 bg-neutral-900/60 hover:bg-neutral-800 text-neutral-400 hover:text-white rounded-md opacity-0 hover:opacity-100 transition-opacity z-40 cursor-pointer hidden md:block"
  >
    {#if isFullscreen}
      <Minimize size={18} />
    {:else}
      <Maximize size={18} />
    {/if}
  </button>
</main>

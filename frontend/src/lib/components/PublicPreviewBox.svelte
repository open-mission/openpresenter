<script lang="ts">
  import type {
    Slide,
    MediaItem,
    PublicDisplaySettings,
    ChurchBrandSettings,
    DisplayMode,
  } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
  import {
    resolveBackground,
    resolveMediaUrl,
    type ResolvedBackground,
  } from '$lib/backgrounds';
  import { backgroundStylesStore } from '$lib/displaySettings';
  import { autoScale } from '$lib/autoScale';
  import { Clock } from 'lucide-svelte';

  let {
    publicSettings,
    churchBrand,
    activeSlide,
    activeMedia,
    displayMode = 'live',
    wallpaperPath = '',
    wallpaperMode = 'cover',
    resolvedWallpaper,
  }: {
    publicSettings: PublicDisplaySettings;
    churchBrand?: ChurchBrandSettings;
    activeSlide?: Slide;
    activeMedia?: MediaItem;
    displayMode?: DisplayMode;
    wallpaperPath?: string;
    wallpaperMode?: string;
    resolvedWallpaper?: ResolvedBackground;
  } = $props();

  let containerWidth = $state<number>(320);
  const scale = $derived(containerWidth > 0 ? containerWidth / 1920 : 0.16666);

  const isScripture = $derived(activeMedia?.kind === 'scripture');
  const isImageMedia = $derived(activeMedia?.kind === 'image');
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

  const resolvedBg = $derived(
    resolvedWallpaper ?? resolveBackground(wallpaperPath, $backgroundStylesStore)
  );
  const activeTextColor = $derived(resolvedBg.textColor || '#ffffff');
  const wallpaperFit = $derived(publicSettings?.wallpaper_fit || wallpaperMode || 'contain');

  const marginTop = $derived(Number(publicSettings?.margin_top ?? 32));
  const marginBottom = $derived(Number(publicSettings?.margin_bottom ?? 32));
  const marginHorizontal = $derived(Number(publicSettings?.margin_horizontal ?? 48));
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
    if (!churchBrand?.show_logo_on_public) return '';
    const variant = churchBrand.logo_variant || 'primary';
    if (variant === 'primary' && churchBrand.logo_primary) return churchBrand.logo_primary;
    if (variant === 'dark' && churchBrand.logo_dark) return churchBrand.logo_dark;
    if (variant === 'light' && churchBrand.logo_light) return churchBrand.logo_light;
    if (variant === 'full' && churchBrand.logo_full) return churchBrand.logo_full;
    return (
      churchBrand.logo_primary ||
      churchBrand.logo_light ||
      churchBrand.logo_dark ||
      churchBrand.logo_full ||
      ''
    );
  });

  const logoPositionStyles = $derived.by<string>(() => {
    const mx = churchBrand?.logo_margin_horizontal ?? 24;
    const my = churchBrand?.logo_margin_vertical ?? 24;
    switch (churchBrand?.logo_position) {
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

<div
  bind:clientWidth={containerWidth}
  class="w-full aspect-video rounded-lg bg-black border border-neutral-800 flex flex-col items-center justify-center relative overflow-hidden shadow-inner select-none font-sans"
>
  <!-- Canvas Virtual 1920x1080 Espelhado 1:1 via CSS Transform -->
  <div
    style="width: 1920px; height: 1080px; transform: scale({scale}); transform-origin: top left; position: absolute; top: 0; left: 0; pointer-events: none;"
    class="relative bg-black text-white overflow-hidden flex items-center justify-center select-none"
  >
    {#if displayMode === 'blackout'}
      <!-- Blackout total da projeção -->
      <div class="absolute inset-0 bg-black z-30 flex items-center justify-center">
        <span class="text-4xl font-black text-red-500 uppercase tracking-widest px-8 py-3 rounded-xl bg-red-950/80 border border-red-800/80 shadow-2xl">
          Blackout
        </span>
      </div>
    {:else}
      <!-- Fundo / Papel de Parede Ativo (Espelhando 100% o PublicView) -->
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
              src={resolvedBg.url}
              class="w-full h-full {wallpaperFit === 'contain' ? 'object-contain' : wallpaperFit === 'fill' ? 'object-fill' : 'object-cover'} {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
              autoplay
              loop
              muted
              playsinline
            ></video>
          {:else if wallpaperFit === 'repeat'}
            <div
              class="w-full h-full {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl overflow-hidden' : ''}"
              style="background-image: url('{resolvedBg.url}'); background-repeat: repeat;"
            ></div>
          {:else if wallpaperFit === 'contain'}
            <img
              src={resolvedBg.url}
              alt="Fundo"
              class="w-full h-full object-contain {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
            />
          {:else if wallpaperFit === 'fill'}
            <img
              src={resolvedBg.url}
              alt="Fundo"
              class="w-full h-full object-fill {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
            />
          {:else if wallpaperFit === 'center'}
            <img
              src={resolvedBg.url}
              alt="Fundo"
              class="max-w-full max-h-full object-none {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
            />
          {:else}
            <!-- cover -->
            <img
              src={resolvedBg.url}
              alt="Fundo"
              class="w-full h-full object-cover {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"
            />
          {/if}

          <!-- Backdrop opcional que escurece o fundo geral -->
          {#if publicSettings.darken_background && activeSlide && !isImageMedia}
            <div class="absolute inset-0 bg-black/60 backdrop-blur-[2px] transition-all duration-300 {publicSettings?.wallpaper_safe_margins ? 'rounded-2xl' : ''}"></div>
          {/if}
        </div>
      {/if}

      <!-- Logotipo da Igreja sobreposto no preview com posição real -->
      {#if activeLogoUrl}
        <div
          class="absolute pointer-events-none z-20"
          style="{logoPositionStyles} opacity: {(churchBrand?.logo_opacity ?? 85) / 100};"
        >
          <img
            src={resolveMediaUrl(activeLogoUrl)}
            alt={churchBrand?.name || 'Logo'}
            class="object-contain drop-shadow-[0_2px_12px_rgba(0,0,0,0.8)]"
            style="max-width: {churchBrand?.logo_size || 64}px; max-height: {churchBrand?.logo_size || 64}px;"
          />
        </div>
      {/if}

      <!-- Camada de Conteúdo do Slide com Margens Proporcionais e Auto-Scale Fiel -->
      <div
        class="relative z-10 w-full h-full box-border flex flex-col items-center justify-center overflow-hidden"
        style={applyMargins
          ? `box-sizing: border-box; padding-top: ${marginTop}px; padding-bottom: ${marginBottom}px; padding-left: ${marginHorizontal}px; padding-right: ${marginHorizontal}px;`
          : 'box-sizing: border-box;'}
      >
        {#if displayMode === 'cleartext'}
          <div class="text-3xl font-light text-neutral-400 italic bg-black/50 px-8 py-3 rounded-2xl border border-white/10 drop-shadow">
            Texto Oculto (Fundo Ativo)
          </div>
        {:else if activeSlide}
          {#if isImageMedia}
            <div class="w-full flex-1 min-h-0 flex items-center justify-center overflow-hidden pointer-events-none">
              <img src={resolveMediaUrl(activeSlide.content)} alt={activeMedia?.title} class="{imageFitClass} animate-in fade-in duration-200" />
            </div>
          {:else if isSchedule}
            <!-- Agenda / Programação de Cultos e Festividades -->
            {@const ev = parseScheduleContent(activeSlide.label, activeSlide.content)}
            <div class="w-full flex-1 h-full min-h-0 min-w-0 flex flex-col items-center justify-center overflow-hidden select-none">
              <div class="w-full max-w-5xl rounded-3xl bg-black/65 backdrop-blur-xl border border-white/15 shadow-2xl p-8 sm:p-12 md:p-16 flex flex-col items-center justify-center text-center gap-6">
                <!-- Badges de Topo: Dia da Semana e Horário -->
                <div class="flex items-center justify-center gap-3 flex-wrap">
                  <span class="px-5 py-1.5 rounded-full bg-amber-500/25 text-amber-300 border border-amber-500/50 text-base sm:text-lg md:text-xl font-black uppercase tracking-wider shadow-lg">
                    {ev.day}
                  </span>
                  {#if ev.time}
                    <span class="px-4 py-1.5 rounded-full bg-white/10 text-white border border-white/20 text-sm sm:text-base md:text-lg font-mono font-bold flex items-center gap-2 shadow-lg">
                      <Clock size={18} class="text-amber-400" />
                      <span>{ev.time}</span>
                    </span>
                  {/if}
                </div>

                <!-- Título do Evento com Auto-Scale -->
                <div class="w-full flex-1 min-h-[120px] max-h-[360px] flex items-center justify-center overflow-hidden">
                  <div
                    use:autoScale={{
                      minSize: 32,
                      maxSize: 110,
                    }}
                    style="color: {activeTextColor};"
                    class="w-full font-black tracking-tight leading-tight drop-shadow-[0_4px_24px_rgba(0,0,0,0.95)]"
                  >
                    {ev.title}
                  </div>
                </div>

                <!-- Detalhes do Evento -->
                {#if ev.details}
                  <div class="text-neutral-300 text-base sm:text-lg md:text-xl font-medium tracking-wide opacity-90 drop-shadow-[0_2px_12px_rgba(0,0,0,0.9)] max-w-3xl">
                    {ev.details}
                  </div>
                {/if}

                <!-- Identificador da Agenda no Rodapé -->
                {#if activeMedia?.title}
                  <div class="pt-4 border-t border-white/10 text-xs sm:text-sm text-neutral-400 uppercase tracking-widest font-semibold opacity-75">
                    {activeMedia.title}
                  </div>
                {/if}
              </div>
            </div>
          {:else}
            <!-- Slide de Texto / Música / Bíblia Fiel 1:1 com PublicView -->
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
                      class="mt-6 flex items-center {alignH === 'left' ? 'justify-start' : alignH === 'right' ? 'justify-end' : 'justify-center'} gap-4 drop-shadow-[0_2px_12px_rgba(0,0,0,0.9)] shrink-0"
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
                          class="px-3 py-1 rounded-md bg-white/15 border border-white/20 uppercase font-mono font-bold tracking-wider text-neutral-100 shadow-md"
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
                    class="w-full mt-auto pt-6 flex items-center justify-between gap-4 border-t border-white/10 shrink-0 drop-shadow-[0_2px_12px_rgba(0,0,0,0.9)]"
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
                        class="px-4 py-1.5 rounded-lg bg-white/15 border border-white/20 uppercase font-mono font-bold tracking-wider text-neutral-100 shadow-md"
                      >
                        {version}
                      </span>
                    {/if}
                  </div>
                {:else if !isScripture && publicSettings.show_metadata && (activeMedia?.title || activeMedia?.artist_or_ref)}
                  <div class="text-neutral-300 text-2xl font-medium mt-6 opacity-80 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
                    {activeMedia.title}{activeMedia.artist_or_ref ? ` — ${activeMedia.artist_or_ref}` : ''}
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        {:else if !wallpaperPath}
          <!-- Sem mídia ao vivo e sem papel de parede -->
          <div class="flex flex-col items-center justify-center text-center text-neutral-500 gap-4">
            <img
              src={resolveMediaUrl('/branding/open-presenter-app-icon.svg')}
              alt="OpenPresenter"
              class="w-20 h-20 rounded-2xl shadow-xl opacity-80"
            />
            <span class="text-xl text-neutral-400 font-medium">Aguardando slide</span>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Indicador 16:9 discreto -->
  <span class="absolute top-1 right-1.5 text-[8px] text-neutral-500 font-mono pointer-events-none opacity-50 z-20">
    16:9
  </span>
</div>

<script lang="ts">
  import type {
    Slide,
    MediaItem,
    PublicDisplaySettings,
    ChurchBrandSettings,
    DisplayMode,
  } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
  import { resolveBackground, type ResolvedBackground } from '$lib/backgrounds';
  import { backgroundStylesStore } from '$lib/displaySettings';

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

  const alignH = $derived(activeSlide?.align_h || activeMedia?.align_h || 'center');
  const alignV = $derived(activeSlide?.align_v || activeMedia?.align_v || 'center');

  const resolvedBg = $derived(
    resolvedWallpaper ?? resolveBackground(wallpaperPath, $backgroundStylesStore)
  );
  const activeTextColor = $derived(resolvedBg.textColor || '#ffffff');
  const isImageMedia = $derived(activeMedia?.kind === 'image');
  const isScripture = $derived(activeMedia?.kind === 'scripture');

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
    const scale = 0.25;
    const mx = Math.round((churchBrand?.logo_margin_horizontal ?? 24) * scale);
    const my = Math.round((churchBrand?.logo_margin_vertical ?? 24) * scale);
    switch (churchBrand?.logo_position) {
      case 'top-left':
        return `top: ${Math.max(2, my)}px; left: ${Math.max(2, mx)}px;`;
      case 'bottom-left':
        return `bottom: ${Math.max(2, my)}px; left: ${Math.max(2, mx)}px;`;
      case 'bottom-right':
        return `bottom: ${Math.max(2, my)}px; right: ${Math.max(2, mx)}px;`;
      case 'top-right':
      default:
        return `top: ${Math.max(2, my)}px; right: ${Math.max(2, mx)}px;`;
    }
  });

  // Margens proporcionais reduzidas para a escala do preview
  const previewPaddingStyle = $derived.by(() => {
    if (isImageMedia && publicSettings.ignore_margins_for_media) return '';
    const top = Math.round((publicSettings.margin_top || 32) * 0.16);
    const bottom = Math.round((publicSettings.margin_bottom || 32) * 0.16);
    const horiz = Math.round((publicSettings.margin_horizontal || 48) * 0.16);
    return `padding: ${top}px ${horiz}px ${bottom}px ${horiz}px;`;
  });
</script>

<div
  class="w-full aspect-video rounded-lg bg-black border border-neutral-800 flex flex-col items-center justify-center relative overflow-hidden shadow-inner select-none font-sans"
>
  {#if displayMode === 'blackout'}
    <!-- Blackout total da projeção -->
    <div class="absolute inset-0 bg-black z-30 flex items-center justify-center">
      <span class="text-[10px] font-black text-red-500 uppercase tracking-widest px-2 py-0.5 rounded bg-red-950/80 border border-red-800/80 shadow-md">
        Blackout
      </span>
    </div>
  {:else}
    <!-- Fundo / Papel de Parede Ativo -->
    {#if resolvedBg.isCss || resolvedBg.isImage || resolvedBg.isVideo}
      <div class="absolute inset-0 z-0 overflow-hidden flex items-center justify-center pointer-events-none">
        {#if resolvedBg.isCss}
          <div class="w-full h-full {resolvedBg.className}" style={resolvedBg.style}></div>
        {:else if resolvedBg.isVideo}
          <video
            src={resolvedBg.url}
            class="w-full h-full object-cover"
            autoplay
            loop
            muted
            playsinline
          ></video>
        {:else if wallpaperMode === 'repeat'}
          <div
            class="w-full h-full"
            style="background-image: url('{resolvedBg.url}'); background-repeat: repeat; background-size: 30%;"
          ></div>
        {:else if wallpaperMode === 'contain'}
          <img src={resolvedBg.url} alt="Fundo" class="w-full h-full object-contain" />
        {:else if wallpaperMode === 'center'}
          <img src={resolvedBg.url} alt="Fundo" class="max-w-full max-h-full object-none" />
        {:else}
          <img src={resolvedBg.url} alt="Fundo" class="w-full h-full object-cover" />
        {/if}

        <!-- Backdrop opcional que escurece o fundo geral -->
        {#if publicSettings.darken_background && activeSlide && !isImageMedia}
          <div class="absolute inset-0 bg-black/60 backdrop-blur-[1px]"></div>
        {/if}
      </div>
    {/if}

    <!-- Logotipo da Igreja sobreposto no preview -->
    {#if activeLogoUrl}
      <div
        class="absolute pointer-events-none z-20"
        style="{logoPositionStyles} opacity: {(churchBrand?.logo_opacity ?? 85) / 100};"
      >
        <img
          src={activeLogoUrl}
          alt={churchBrand?.name || 'Logo'}
          class="object-contain drop-shadow-[0_1px_4px_rgba(0,0,0,0.8)]"
          style="max-width: {Math.max(14, Math.round((churchBrand?.logo_size || 64) * 0.25))}px; max-height: {Math.max(14, Math.round((churchBrand?.logo_size || 64) * 0.25))}px;"
        />
      </div>
    {/if}

    <!-- Camada de Conteúdo do Slide com Margens Proporcionais -->
    <div
      class="relative z-10 w-full h-full flex items-center justify-center overflow-hidden"
      style={previewPaddingStyle}
    >
      {#if displayMode === 'cleartext'}
        <div class="text-[9px] font-medium text-neutral-400 italic bg-black/40 px-2 py-0.5 rounded border border-white/5 drop-shadow">
          Texto Oculto (Fundo Ativo)
        </div>
      {:else if activeSlide}
        {#if isImageMedia}
          <div class="w-full h-full flex items-center justify-center overflow-hidden pointer-events-none">
            <img src={activeSlide.content} alt={activeMedia?.title} class={imageFitClass} />
          </div>
        {:else}
          <!-- Slide de Texto ou Bíblia -->
          <div
            class="w-full h-full flex flex-col {alignV === 'top' ? 'justify-start' : alignV === 'bottom' ? 'justify-end' : 'justify-center'} {alignH === 'left' ? 'text-left items-start' : alignH === 'right' ? 'text-right items-end' : alignH === 'justify' ? 'text-justify items-stretch' : 'text-center items-center'}"
          >
            <div
              class="my-auto {publicSettings.fill_space ? 'w-full' : 'max-w-[90%]'} {publicSettings.text_box_background ? 'bg-black/70 backdrop-blur-sm rounded-md p-1.5 border border-white/15 shadow-lg' : ''}"
            >
              <div
                style="color: {activeTextColor};"
                class="font-extrabold leading-tight drop-shadow-[0_2px_8px_rgba(0,0,0,0.95)] {publicSettings.fill_space ? 'text-[11px] sm:text-[13px]' : 'text-[10px] sm:text-[11px]'} {publicSettings.word_wrap ? 'break-words' : 'whitespace-nowrap overflow-hidden text-ellipsis'}"
              >
                {@html renderFormattedText(activeSlide.content)}
              </div>

              {#if isScripture}
                {@const refLabel = activeSlide.label || activeMedia?.title || 'Bíblia'}
                {@const version = activeMedia?.artist_or_ref || ''}
                {@const cleanRef = version && refLabel.endsWith(version) ? refLabel.slice(0, -version.length).trim() : refLabel}
                <div class="mt-2 pt-1 border-t border-white/10 flex items-center justify-between gap-1 drop-shadow-[0_1px_4px_rgba(0,0,0,0.95)]">
                  <span style="color: {activeTextColor};" class="text-[9px] sm:text-[10px] font-bold tracking-tight">
                    {cleanRef}
                  </span>
                  {#if version}
                    <span class="text-[7px] px-1 py-0.2 rounded bg-white/20 border border-white/25 uppercase font-mono font-bold text-neutral-200">
                      {version}
                    </span>
                  {/if}
                </div>
              {:else if publicSettings.show_metadata && (activeMedia?.title || activeMedia?.artist_or_ref)}
                <div class="text-neutral-300 text-[8px] font-medium mt-1 opacity-80 truncate drop-shadow">
                  {activeMedia.title}{activeMedia.artist_or_ref ? ` — ${activeMedia.artist_or_ref}` : ''}
                </div>
              {/if}
            </div>
          </div>
        {/if}
      {:else if !wallpaperPath}
        <!-- Sem mídia ao vivo e sem papel de parede -->
        <div class="flex flex-col items-center justify-center text-center opacity-60 gap-1">
          <img
            src="/branding/open-presenter-app-icon.svg"
            alt="OpenPresenter"
            class="w-6 h-6 rounded-lg shadow"
          />
          <span class="text-[8px] text-neutral-500 font-medium">Aguardando slide</span>
        </div>
      {/if}
    </div>

    <!-- Indicador 16:9 discreto -->
    <span class="absolute top-1 right-1.5 text-[8px] text-neutral-500 font-mono pointer-events-none opacity-50 z-20">
      16:9
    </span>
  {/if}
</div>

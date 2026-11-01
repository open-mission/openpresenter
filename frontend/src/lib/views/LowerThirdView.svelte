<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import {
    presentationState,
    mediaItemsList,
    activeSetlist,
    initWebSocket,
  } from '$lib/ws';
  import {
    lowerThirdSettingsStore,
    backgroundStylesStore,
    syncDisplaySettingsFromBackend,
  } from '$lib/displaySettings';
  import { resolveBackground } from '$lib/backgrounds';
  import { renderFormattedText } from '$lib/markdown';
  import { resolveMediaUrl } from '$lib/cast';
  import type { Slide, MediaItem } from '$lib/types';
  import { Camera, Layers, CheckCircle2, Tv, Music, BookOpen } from 'lucide-svelte';

  let isPreviewBackground = $state(false);
  let isObsBrowser = $state(false);
  let fetchedSong = $state<MediaItem | null>(null);

  onMount(() => {
    initWebSocket('lower-third');
    syncDisplaySettingsFromBackend();

    // Detecta se está sendo executado no navegador OBS Studio CEF
    isObsBrowser =
      typeof window !== 'undefined' &&
      ((window as any).obsstudio !== undefined ||
        new URLSearchParams(window.location.search).has('obs'));

    // Parâmetro de teste/preview na URL (ex: /lower-third?preview=1 ou /lower-third?bg=dark)
    const params = new URLSearchParams(window.location.search);
    if (params.get('preview') === '1' || params.get('test') === '1' || params.get('bg') === 'dark') {
      isPreviewBackground = true;
    }

    // Se estiver em um navegador de desktop comum (fora do OBS) e sem parâmetro explícito,
    // ativa o modo simulação de câmera para não exibir tela branca vazia no browser
    if (!isObsBrowser && params.get('transparent') !== '1') {
      isPreviewBackground = true;
    }

    // Aplica classes de transparência no html e body para garantir compatibilidade no OBS
    document.documentElement.classList.add('is-lower-third');
    document.body.classList.add('is-lower-third');

    return () => {
      document.documentElement.classList.remove('is-lower-third');
      document.body.classList.remove('is-lower-third');
    };
  });

  const lt = $derived($lowerThirdSettingsStore);

  // Busca mídia ativa em quick_media, mediaItemsList, activeSetlist e fallback de fetch sob demanda
  const activeMedia = $derived.by<MediaItem | null>(() => {
    const p = $presentationState;
    if (!p.active_media_id) return null;

    // 1. Quick Media (Bíblia instantânea, apresentação rápida)
    if (p.quick_media && p.quick_media.id === p.active_media_id) {
      return p.quick_media;
    }

    // 2. Catálogo de mídias da sessão/setlist
    const fromList = $mediaItemsList.find((m) => m.id === p.active_media_id);
    if (fromList) return fromList;

    // 3. Setlist ativa no momento
    if ($activeSetlist && $activeSetlist.items) {
      const fromSetlist = $activeSetlist.items.find((si) => si.id === p.active_media_id);
      if (fromSetlist) return fromSetlist;
    }

    // 4. Mídia sob demanda buscada via fetch
    if (fetchedSong && fetchedSong.id === p.active_media_id) {
      return fetchedSong;
    }

    return null;
  });

  // Fallback assíncrono caso a página tenha sido aberta isoladamente no OBS
  $effect(() => {
    const mediaId = $presentationState.active_media_id;
    if (mediaId && !activeMedia && mediaId.startsWith('song-')) {
      fetch(`/api/songs/${mediaId}`)
        .then((r) => (r.ok ? r.json() : null))
        .then((song) => {
          if (song && song.id === mediaId) {
            fetchedSong = {
              id: song.id,
              title: song.title,
              artist_or_ref: song.artist,
              kind: 'song',
              slides: song.slides || [],
            };
          }
        })
        .catch(() => {});
    }
  });

  // Determinar se o conteúdo deve estar visível
  const isVisible = $derived.by(() => {
    if (!lt.enabled) return false;
    if ($presentationState.display_mode === 'blackout') return false;
    if ($presentationState.display_mode === 'cleartext') return false;
    if (!$presentationState.active_media_id) return false;
    return true;
  });

  // Slide atual
  const currentSlide = $derived.by<Slide | null>(() => {
    if (!isVisible || !activeMedia) return null;
    const idx = $presentationState.active_slide_index ?? 0;
    if (activeMedia.slides && activeMedia.slides[idx]) {
      return activeMedia.slides[idx];
    }
    return null;
  });

  const isScripture = $derived(activeMedia?.kind === 'scripture');

  // Detecta se o slide atual é de mídia/imagem
  const isImageMedia = $derived.by(() => {
    if (activeMedia?.kind === 'image') return true;
    if (!currentSlide?.content) return false;
    const c = currentSlide.content;
    if (c.startsWith('/media/') || c.startsWith('/api/media/')) return true;
    if (/\.(jpg|jpeg|png|webp|gif|svg|bmp|tiff)$/i.test(c)) return true;
    return false;
  });

  // URL efetiva da imagem (prioridade: lower_image_url > lower_content > content)
  const effectiveImageUrl = $derived.by(() => {
    if (!isImageMedia) return '';
    const url = activeMedia?.lower_image_url || currentSlide?.lower_content || currentSlide?.content || '';
    return resolveMediaUrl(url);
  });

  // Modo de exibição da imagem no Lower Third
  const imageDisplayMode = $derived(lt.image_display_mode || 'fullscreen');
  const imageFit = $derived(lt.image_fit || 'contain');

  // Informações de título e referência
  const mediaLabel = $derived.by(() => {
    if (!activeMedia) return '';
    if (isScripture) {
      return currentSlide?.label || activeMedia.title || 'Bíblia';
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

  // Tamanho de fonte dinâmico (louvor vs versículo bíblico)
  const effectiveFontSize = $derived(
    isScripture ? (lt.scripture_font_size ?? lt.font_size ?? 28) : (lt.font_size ?? 32)
  );

  // Fundo com imagem personalizada
  const boxHasImage = $derived(
    lt.box_bg_type === 'image' && Boolean(lt.box_bg_image_url && lt.box_bg_image_url.trim())
  );

  const borderRadiusPx = $derived(lt.box_border_radius ?? (lt.theme === 'floating' ? 24 : 16));
  const fadeAmount = $derived(lt.box_edge_fade_amount ?? 20);

  const edgeFadeMask = $derived(
    lt.box_edge_fade
      ? `mask-image: radial-gradient(ellipse 95% 85% at 50% 50%, black ${Math.max(20, 100 - fadeAmount * 2)}%, transparent 100%); -webkit-mask-image: radial-gradient(ellipse 95% 85% at 50% 50%, black ${Math.max(20, 100 - fadeAmount * 2)}%, transparent 100%);`
      : ''
  );

  const boxImageLayerStyle = $derived(
    boxHasImage
      ? `background-image: url('${lt.box_bg_image_url}'); background-size: cover; background-position: center; opacity: ${(lt.bg_opacity ?? 75) / 100}; border-radius: ${borderRadiusPx}px; ${edgeFadeMask}`
      : ''
  );

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
  class="lower-third-root fixed inset-0 w-screen h-screen overflow-hidden flex flex-col justify-end select-none font-sans"
  style={isPreviewBackground
    ? 'background: radial-gradient(circle at center, #1e293b 0%, #090d16 100%); pointer-events: auto;'
    : 'background: transparent; pointer-events: none;'}
>
  <!-- Barra de utilitários flutuante para operadores testando no navegador (fora do OBS) -->
  {#if !isObsBrowser}
    <aside
      aria-label="Controles de teste e prévia do Lower Third"
      class="absolute top-3 right-3 flex items-center gap-2 z-50 pointer-events-auto bg-neutral-950/85 backdrop-blur-md border border-neutral-800 text-xs px-3 py-1.5 rounded-xl text-neutral-300 shadow-xl opacity-80 hover:opacity-100 transition-opacity"
    >
      <div class="flex items-center gap-1.5 text-[11px] font-medium text-emerald-400">
        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
        <span>OBS Lower Third</span>
      </div>
      <div class="h-3 w-px bg-neutral-700"></div>
      <button
        type="button"
        onclick={() => (isPreviewBackground = !isPreviewBackground)}
        class="flex items-center gap-1.5 px-2 py-0.5 rounded-lg text-[11px] font-semibold transition-colors cursor-pointer {isPreviewBackground
          ? 'bg-blue-600 text-white'
          : 'bg-neutral-800 text-neutral-300 hover:bg-neutral-750'}"
        title="Alterna entre simulação de vídeo escuro e transparência alfa para OBS"
      >
        {#if isPreviewBackground}
          <Camera class="w-3.5 h-3.5" />
          <span>Simulação Vídeo</span>
        {:else}
          <Layers class="w-3.5 h-3.5" />
          <span>Fundo Transparente</span>
        {/if}
      </button>
    </aside>
  {/if}

  <!-- Banner de status quando aberto no navegador comum e nenhum slide estiver ao vivo -->
  {#if !isObsBrowser && isPreviewBackground && (!isVisible || !currentSlide)}
    <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center z-20 pointer-events-none">
      <div class="max-w-md p-6 rounded-2xl bg-neutral-900/90 border border-neutral-800 shadow-2xl backdrop-blur-md space-y-3">
        <div class="w-12 h-12 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 flex items-center justify-center mx-auto">
          <Tv class="w-6 h-6" />
        </div>
        <div>
          <h2 class="text-base font-bold text-white flex items-center justify-center gap-1.5">
            <CheckCircle2 class="w-4 h-4 text-emerald-400" />
            Lower Third Conectado
          </h2>
          <p class="text-xs text-neutral-400 mt-1 leading-relaxed">
            Aguardando projeção ao vivo. Projete uma <strong>música</strong> ou <strong>versículo bíblico</strong> no painel administrativo ou pelo celular para visualizar aqui no terço inferior.
          </p>
        </div>
        <div class="pt-2 border-t border-neutral-800 text-[11px] text-neutral-400 font-mono">
          URL no OBS: {typeof window !== 'undefined' ? `${window.location.origin}/lower-third` : 'http://localhost:18080/lower-third'}
        </div>
      </div>
    </div>
  {/if}

  <!-- ════════════════════════════════════════════════════════════ -->
  <!-- RENDERIZAÇÃO DE IMAGEM EM TELA CHEIA NO LOWER THIRD (OBS)  -->
  <!-- ════════════════════════════════════════════════════════════ -->
  {#if isVisible && currentSlide && isImageMedia && imageDisplayMode === 'fullscreen'}
    <div
      transition:fade={{ duration: 300 }}
      class="fixed inset-0 z-40 flex items-center justify-center bg-black/40"
      style="pointer-events: none;"
    >
      <img
        src={effectiveImageUrl}
        alt={activeMedia?.title || 'Imagem'}
        class="w-full h-full drop-shadow-2xl"
        style="object-fit: {imageFit};"
        onerror={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }}
      />
    </div>
  {/if}

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
        <div transition:fade={{ duration: 250 }} class="w-full flex flex-col items-center">
          {#if isImageMedia}
            <!-- RENDERIZAÇÃO DE IMAGEM NO LOWER THIRD (MODO BANNER) -->
            {#if imageDisplayMode === 'lower_third'}
              <div class="max-h-44 overflow-hidden rounded-xl border border-white/20 shadow-2xl bg-black/60 backdrop-blur-md flex items-center justify-center">
                <img
                  src={effectiveImageUrl}
                  alt={activeMedia?.title || 'Imagem'}
                  class="max-h-44 w-auto mx-auto drop-shadow-lg"
                  style="object-fit: {imageFit};"
                  onerror={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }}
                />
              </div>
            {/if}
          {:else}
            <!-- ESTILO 1: GRADIENTE CENTRAL (Padrão de Transmissão Profissional) -->
            {#if lt.theme === 'gradient'}
              <div
                class="relative px-8 py-5 flex flex-col {lt.text_align === 'left' ? 'items-start text-left' : lt.text_align === 'right' ? 'items-end text-right' : 'items-center text-center'} overflow-hidden w-full"
                style="border-radius: {borderRadiusPx}px; {boxHasImage ? edgeFadeMask : 'mask-image: radial-gradient(ellipse 98% 90% at 50% 50%, black 50%, transparent 100%); -webkit-mask-image: radial-gradient(ellipse 98% 90% at 50% 50%, black 50%, transparent 100%);'}"
              >
                {#if boxHasImage}
                  <div class="absolute inset-0 w-full h-full pointer-events-none z-0" style={boxImageLayerStyle}></div>
                {:else}
                  <div
                    class="absolute inset-0 w-full h-full pointer-events-none z-0"
                    style="background: radial-gradient(ellipse at center, {bgRgba} 0%, {bgRgba} 45%, {bgRgbaTransparent} 92%);"
                  ></div>
                {/if}

                <div class="relative z-10 w-full flex flex-col {lt.text_align === 'left' ? 'items-start text-left' : lt.text_align === 'right' ? 'items-end text-right' : 'items-center text-center'}">
                  <!-- Badge de Referência Bíblica ou Música -->
                  {#if lt.show_reference && mediaLabel}
                    <div
                      class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-2.5 bg-black/60 border border-white/20 text-white shadow-md backdrop-blur-md"
                    >
                      {#if isScripture}
                        <BookOpen class="w-3.5 h-3.5 text-primary" />
                      {:else}
                        <Music class="w-3.5 h-3.5 text-primary" />
                      {/if}
                      <span>{mediaLabel}</span>
                      {#if subLabel}
                        <span class="opacity-70 font-normal">({subLabel})</span>
                      {/if}
                    </div>
                  {/if}

                  <!-- Texto Principal do Slide -->
                  {#key currentSlide.content}
                    <div
                      transition:fade={{ duration: 150 }}
                      class="w-full leading-snug tracking-normal {lt.text_align === 'left' ? 'text-left' : lt.text_align === 'right' ? 'text-right' : 'text-center'}"
                      style="font-size: {effectiveFontSize}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
                    >
                      {@html renderFormattedText(currentSlide.content)}
                    </div>
                  {/key}
                </div>
              </div>

            <!-- ESTILO 2: BARRA INFERIOR (Lower Third Estilo Noticiário / TV) -->
            {:else if lt.theme === 'bar'}
              <div
                class="w-full px-8 py-4 shadow-2xl backdrop-blur-md flex flex-col {lt.text_align === 'left' ? 'items-start text-left' : lt.text_align === 'right' ? 'items-end text-right' : 'items-center text-center'} relative overflow-hidden"
                style="{boxHasImage ? '' : `background-color: ${bgRgba};`} border-radius: {borderRadiusPx}px; border-left: 6px solid {resolvedBg.textColor || '#3b82f6'}; {edgeFadeMask}"
              >
                {#if boxHasImage}
                  <div class="absolute inset-0 w-full h-full pointer-events-none z-0" style={boxImageLayerStyle}></div>
                {/if}

                <div class="relative z-10 w-full flex flex-col {lt.text_align === 'left' ? 'items-start text-left' : lt.text_align === 'right' ? 'items-end text-right' : 'items-center text-center'}">
                  {#if lt.show_reference && mediaLabel}
                    <div class="inline-flex items-center gap-1.5 text-xs uppercase font-extrabold tracking-wider mb-1 text-emerald-400 drop-shadow">
                      {#if isScripture}
                        <BookOpen class="w-3.5 h-3.5" />
                      {:else}
                        <Music class="w-3.5 h-3.5" />
                      {/if}
                      <span>{mediaLabel}</span>
                      {#if subLabel}
                        <span class="opacity-70 font-normal">• {subLabel}</span>
                      {/if}
                    </div>
                  {/if}

                  {#key currentSlide.content}
                    <div
                      transition:fade={{ duration: 150 }}
                      class="w-full leading-snug {lt.text_align === 'left' ? 'text-left' : lt.text_align === 'right' ? 'text-right' : 'text-center'}"
                      style="font-size: {effectiveFontSize}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
                    >
                      {@html renderFormattedText(currentSlide.content)}
                    </div>
                  {/key}
                </div>
              </div>

            <!-- ESTILO 3: CAIXA FLUTUANTE (Glassmorphism Pill) -->
            {:else if lt.theme === 'floating'}
              <div
                class="px-8 py-4 border border-white/20 shadow-2xl backdrop-blur-lg flex flex-col {lt.text_align === 'left' ? 'mr-auto items-start text-left' : lt.text_align === 'right' ? 'ml-auto items-end text-right' : 'mx-auto items-center text-center'} relative overflow-hidden"
                style="{boxHasImage ? '' : `background-color: ${bgRgba};`} border-radius: {borderRadiusPx}px; {edgeFadeMask}"
              >
                {#if boxHasImage}
                  <div class="absolute inset-0 w-full h-full pointer-events-none z-0" style={boxImageLayerStyle}></div>
                {/if}

                <div class="relative z-10 w-full flex flex-col {lt.text_align === 'left' ? 'items-start text-left' : lt.text_align === 'right' ? 'items-end text-right' : 'items-center text-center'}">
                  {#if lt.show_reference && mediaLabel}
                    <div class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-bold uppercase mb-2 bg-white/15 text-white/90">
                      {#if isScripture}
                        <BookOpen class="w-3 h-3 text-primary" />
                      {:else}
                        <Music class="w-3 h-3 text-primary" />
                      {/if}
                      <span>{mediaLabel}</span>
                    </div>
                  {/if}

                  {#key currentSlide.content}
                    <div
                      transition:fade={{ duration: 150 }}
                      class="w-full leading-snug {lt.text_align === 'left' ? 'text-left' : lt.text_align === 'right' ? 'text-right' : 'text-center'}"
                      style="font-size: {effectiveFontSize}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
                    >
                      {@html renderFormattedText(currentSlide.content)}
                    </div>
                  {/key}
                </div>
              </div>

            <!-- ESTILO 4: MINIMALISTA (Sem caixa de fundo, apenas tipografia de alto impacto) -->
            {:else}
              <div class="w-full flex flex-col {lt.text_align === 'left' ? 'items-start text-left' : lt.text_align === 'right' ? 'items-end text-right' : 'items-center text-center'}">
                {#if lt.show_reference && mediaLabel}
                  <div
                    class="inline-flex items-center gap-1.5 text-sm uppercase font-bold tracking-widest mb-1 text-amber-300"
                    style="text-shadow: {textShadowStyle};"
                  >
                    {#if isScripture}
                      <BookOpen class="w-3.5 h-3.5" />
                    {:else}
                      <Music class="w-3.5 h-3.5" />
                    {/if}
                    <span>{mediaLabel}</span>
                  </div>
                {/if}

                {#key currentSlide.content}
                  <div
                    transition:fade={{ duration: 150 }}
                    class="w-full leading-snug {lt.text_align === 'left' ? 'text-left' : lt.text_align === 'right' ? 'text-right' : 'text-center'}"
                    style="font-size: {effectiveFontSize}px; font-weight: {lt.font_weight === 'extrabold' ? '800' : lt.font_weight === 'bold' ? '700' : lt.font_weight === 'semibold' ? '600' : '400'}; color: {lt.text_color || '#ffffff'}; text-shadow: {textShadowStyle};"
                  >
                    {@html renderFormattedText(currentSlide.content)}
                  </div>
                {/key}
              </div>
            {/if}
            <!-- Fim da condicional de imagem -->
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>

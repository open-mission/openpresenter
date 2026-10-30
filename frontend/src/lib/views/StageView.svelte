<script lang="ts">
  import { onMount } from 'svelte';
  import { presentationState, mediaItemsList, activeSetlist, initWebSocket } from '$lib/ws';
  import type { MediaItem, Slide } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
  import { autoScale } from '$lib/autoScale';
  import {
    stageSettingsStore,
    syncDisplaySettingsFromBackend,
  } from '$lib/displaySettings';
  import {
    Maximize,
    Minimize,
    Clock,
    ArrowRight,
    Hourglass,
    MessageSquare,
    AlertTriangle,
  } from 'lucide-svelte';

  let currentTime = $state('');
  let currentDate = $state('');
  let isFullscreen = $state(false);

  onMount(() => {
    initWebSocket('stage');
    syncDisplaySettingsFromBackend();

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
    if (p.quick_media && p.quick_media.id === p.active_media_id) {
      return p.quick_media;
    }
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

  const isCurrentImage = $derived(
    activeMedia?.kind === 'image' || (activeSlide?.content && activeSlide.content.startsWith('/media/'))
  );

  const isNextImage = $derived(
    nextSlide?.content && nextSlide.content.startsWith('/media/')
  );

  const isScripture = $derived(activeMedia?.kind === 'scripture');

  const stageSettings = $derived($stageSettingsStore);
  const timer = $derived($presentationState.timer);
  const stageMessage = $derived($presentationState.stage_message || $presentationState.timer?.stage_message || '');

  // Proporções configuráveis com fallbacks inteligentes
  const currentRatio = $derived(stageSettings.current_slide_percent || 60);
  const nextRatio = $derived(stageSettings.next_slide_percent || 30);
  const footerRatio = $derived(stageSettings.footer_time_percent || 10);

  // Determina se o timer está em Overtime (passou do tempo previsto)
  const timerIsOvertime = $derived(
    !!timer &&
      (Boolean(timer.is_overtime) ||
        (timer.duration_seconds > 0 && timer.remaining_seconds <= 0))
  );

  // Alerta quando restam menos de 60s
  const timerIsNearEnd = $derived(
    !!timer &&
      !timerIsOvertime &&
      timer.duration_seconds > 0 &&
      timer.remaining_seconds <= (timer.warning_threshold_seconds || 60) &&
      timer.remaining_seconds > 0
  );

  // Cor do timer de acordo com o estado
  const timerColorClass = $derived(
    timerIsOvertime
      ? 'text-rose-500 animate-pulse'
      : timerIsNearEnd
        ? 'text-amber-400'
        : 'text-white'
  );

  function formatTime(remaining: number, isOvertime = false): string {
    const isNeg = remaining < 0 || isOvertime;
    const abs = Math.abs(remaining);
    const m = Math.floor(abs / 60);
    const s = abs % 60;
    const str = `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
    return isNeg ? `-${str}` : str;
  }
</script>

<svelte:window onkeydown={(e) => { if (e.key.toLowerCase() === 'f') toggleFullscreen(); }} />

<!-- Monitor de Palco / Retorno do Apresentador -->
<main
  class="relative w-screen h-screen bg-black text-white flex flex-col p-0 overflow-hidden select-none font-sans"
  ondblclick={toggleFullscreen}
>
  <!-- Alerta se Blackout ou Limpar Texto estiver ativo -->
  {#if $presentationState.display_mode === 'blackout'}
    <div class="bg-red-500/20 border border-red-500/40 text-red-300 py-1.5 px-4 text-center text-xs font-semibold uppercase tracking-wider shrink-0">
      Projeção em Blackout (Tela Preta no Público)
    </div>
  {:else if $presentationState.display_mode === 'cleartext'}
    <div class="bg-amber-500/20 border border-amber-500/40 text-amber-300 py-1.5 px-4 text-center text-xs font-semibold uppercase tracking-wider shrink-0">
      Texto Oculto no Público (Clear Text)
    </div>
  {/if}

  <!-- ========================================================================= -->
  <!-- MODO DE DESCANSO / STANDBY (Quando não há slide ativo em apresentação)   -->
  <!-- ========================================================================= -->
  {#if !activeSlide}
    <div class="flex-1 flex flex-col justify-center items-center text-center p-6 sm:p-10 md:p-14 animate-in fade-in zoom-in-95 duration-200 min-h-0 relative">
      <!-- Botão Discreto de Alternar Tela Cheia no Canto Superior Direito -->
      <button
        type="button"
        onclick={toggleFullscreen}
        class="absolute top-5 right-5 p-2 rounded-lg bg-zinc-900/40 border border-zinc-800/60 text-zinc-500 hover:text-white hover:bg-zinc-800 transition-all cursor-pointer opacity-40 hover:opacity-100"
        title="Alternar Tela Cheia (F)"
      >
        {#if isFullscreen}
          <Minimize size={16} />
        {:else}
          <Maximize size={16} />
        {/if}
      </button>

      <!-- Mensagem de Palco em Destaque (se houver) -->
      {#if stageMessage}
        <div class="w-full max-w-3xl mb-8 p-3 sm:p-4 rounded-xl bg-amber-500/15 border border-amber-500/40 text-amber-200 text-base sm:text-lg md:text-xl font-bold text-center shadow-lg animate-pulse shrink-0">
          ⚠️ {stageMessage}
        </div>
      {/if}

      <!-- Centro: Horário em Tipografia Gigante e Imponente -->
      <div class="my-auto flex flex-col items-center justify-center space-y-2 select-none">
        <div
          class="text-7xl sm:text-9xl md:text-[140px] lg:text-[170px] font-mono font-black tracking-tight leading-none drop-shadow-2xl"
          style="color: {stageSettings.current_text_color || '#ffffff'};"
        >
          {currentTime || '00:00:00'}
        </div>
        <div class="text-base sm:text-xl md:text-2xl text-zinc-400 capitalize font-medium tracking-wide">
          {currentDate}
        </div>

        <!-- Bloco do Cronômetro / Timer Centralizado (Apenas se estiver rodando ou em overtime) -->
        {#if timer && (timer.running || timerIsOvertime)}
          <div
            class="mt-6 sm:mt-8 px-8 sm:px-12 py-4 sm:py-6 rounded-3xl bg-zinc-900/70 border-2 transition-all flex flex-col items-center gap-1.5 shadow-2xl {timerIsOvertime ? 'border-rose-500/80 bg-rose-950/40 animate-pulse' : timerIsNearEnd ? 'border-amber-500/80 bg-amber-950/40' : 'border-zinc-800'}"
          >
            <div class="text-xs sm:text-sm uppercase tracking-widest font-bold flex items-center gap-2 {timerIsOvertime ? 'text-rose-300' : timerIsNearEnd ? 'text-amber-300' : 'text-zinc-400'}">
              <Hourglass size={14} class={timer.running ? 'animate-spin' : ''} />
              <span>{timer.label || 'Cronômetro'} {timer.running ? '(Em andamento)' : ''}</span>
            </div>

            <div class="text-5xl sm:text-6xl md:text-7xl font-mono font-black tracking-tight {timerColorClass}">
              {formatTime(timer.remaining_seconds, timerIsOvertime)}
            </div>

            {#if timerIsOvertime}
              <div class="text-xs text-rose-300 font-bold uppercase tracking-wider mt-0.5">
                Tempo Excedido
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>

  <!-- ========================================================================= -->
  <!-- PRESET ÚNICO: GRID MODERNO PRO (Auto-scale, sem gaps, sem border-radius) -->
  <!-- ========================================================================= -->
  {:else}
    <div class="flex-1 flex flex-col md:flex-row gap-0 min-h-0 w-full h-full">
      <!-- Coluna Principal (Slides: Atual em cima, Próximo embaixo) -->
      <div
        class="flex flex-col gap-0 min-h-0 transition-all border-b md:border-b-0 md:border-r border-zinc-800"
        style="flex: {100 - (footerRatio * 2.2)}; min-width: 0;"
      >
        <!-- Slide Atual Grande -->
        <div
          class="flex flex-col rounded-none bg-zinc-950/80 p-3 sm:p-5 overflow-hidden border-b transition-all min-h-0"
          style="flex: {currentRatio}; border-color: {isScripture ? (stageSettings.scripture_border_color || stageSettings.current_border_color || '#3b82f6') : (stageSettings.current_border_color || '#3b82f6')};"
        >
          <div class="flex items-center justify-between pb-2 border-b border-zinc-800 shrink-0">
            <span
              class="text-xs font-bold uppercase tracking-widest flex items-center gap-2"
              style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};"
            >
              <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
              Slide Atual
            </span>
            {#if isScripture && activeSlide}
              {@const refLabel = activeSlide.label || activeMedia?.title || 'Bíblia'}
              {@const version = activeMedia?.artist_or_ref || ''}
              {@const cleanRef = version && refLabel.endsWith(version) ? refLabel.slice(0, -version.length).trim() : refLabel}
              <span class="text-xs font-bold bg-zinc-800 text-zinc-200 border border-zinc-700 px-2.5 py-0.5 rounded flex items-center gap-1.5">
                <span>{cleanRef}</span>
                {#if version}
                  <span class="text-[10px] opacity-75 uppercase font-mono">({version})</span>
                {/if}
              </span>
            {:else if activeSlide}
              <span class="text-xs font-semibold bg-zinc-800 text-zinc-300 px-2 py-0.5 rounded">
                {activeSlide.label}
              </span>
            {/if}
          </div>

          <div class="w-full flex-1 min-h-0 flex items-center justify-center overflow-hidden p-2">
            {#if isCurrentImage && activeSlide}
              <div class="w-full h-full flex flex-col items-center justify-center p-2 min-h-0">
                <img
                  src={activeSlide.content}
                  alt={activeMedia?.title || 'Imagem'}
                  class="max-w-full max-h-[50vh] object-contain rounded shadow-2xl border border-zinc-800"
                />
                <span class="text-xs text-zinc-400 font-mono mt-1.5 truncate max-w-sm">{activeMedia?.title || 'Imagem'}</span>
              </div>
            {:else if activeSlide}
              <div
                use:autoScale={{
                  minSize: isScripture
                    ? (stageSettings.scripture_min_font_size || stageSettings.min_font_size || 24)
                    : (stageSettings.lyrics_min_font_size || stageSettings.min_font_size || 24),
                  maxSize: 240
                }}
                class="w-full font-black leading-tight text-center break-words select-none"
                style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};"
              >
                {@html renderFormattedText(activeSlide.content)}
              </div>
            {:else}
              <p class="text-zinc-600 text-xl font-medium text-center">
                Aguardando seleção de slide
              </p>
            {/if}
          </div>
        </div>

        <!-- Próximo Slide Compacto Abaixo -->
        <div
          class="rounded-none bg-zinc-900/40 p-2 sm:p-3 overflow-hidden border-t flex flex-col shrink-0 transition-all min-h-0"
          style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
        >
          <div class="flex items-center justify-between pb-1 border-b border-zinc-800/80 shrink-0">
            <span
              class="text-[11px] font-bold uppercase tracking-wider flex items-center gap-1.5"
              style="color: {stageSettings.next_text_color || '#facc15'};"
            >
              <ArrowRight size={13} />
              A Seguir (Próximo)
            </span>
            {#if isScripture && nextSlide}
              <span class="text-[10px] font-bold bg-zinc-800 text-zinc-300 border border-zinc-700 px-1.5 py-0.5 rounded flex items-center gap-1">
                <span>{nextSlide.label}</span>
              </span>
            {:else if nextSlide}
              <span class="text-[10px] bg-zinc-800 text-zinc-400 px-1.5 py-0.5 rounded">
                {nextSlide.label}
              </span>
            {/if}
          </div>

          <div class="w-full flex-1 min-h-0 flex items-center justify-center overflow-hidden p-2">
            {#if isNextImage && nextSlide}
              <div class="w-full h-full flex items-center justify-center p-1 min-h-0">
                <img
                  src={nextSlide.content}
                  alt="Próxima imagem"
                  class="max-w-full max-h-[22vh] object-contain rounded opacity-85 shadow border border-zinc-800"
                />
              </div>
            {:else if nextSlide}
              <div
                use:autoScale={{
                  minSize: Math.max(14, Math.round((isScripture ? (stageSettings.scripture_min_font_size || 24) : (stageSettings.lyrics_min_font_size || stageSettings.min_font_size || 24)) * 0.75)),
                  maxSize: 180
                }}
                class="w-full font-black leading-tight opacity-95 text-center break-words select-none"
                style="color: {stageSettings.next_text_color || '#facc15'};"
              >
                {@html renderFormattedText(nextSlide.content)}
              </div>
            {:else}
              <p class="text-zinc-600 text-xs text-center py-2">Fim da mídia</p>
            {/if}
          </div>
        </div>
      </div>

      <!-- Coluna Lateral Direita (Aviso, Relógio e Timer - Sem gaps, sem border-radius) -->
      <div
        class="flex flex-col gap-0 min-h-0 transition-all border-l border-zinc-800"
        style="flex: {footerRatio * 2.2}; min-width: 280px; max-width: 440px;"
      >
        <!-- Caixa de Mensagem / Aviso para o Palco: SEMPRE VISÍVEL -->
        <div
          class="p-4 sm:p-5 rounded-none border-b border-zinc-800 flex flex-col justify-between overflow-hidden transition-all {stageMessage ? 'bg-emerald-950/40 text-emerald-300' : 'bg-zinc-950/40 text-zinc-500'}"
          style="flex: 40; min-height: 0;"
        >
          <div class="flex items-center justify-between text-xs font-bold uppercase tracking-wider shrink-0 {stageMessage ? 'text-emerald-400' : 'text-zinc-500'}">
            <span class="flex items-center gap-1.5">
              <MessageSquare size={15} />
              <span>Aviso para o Palco</span>
            </span>
            {#if stageMessage}
              <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            {/if}
          </div>

          <div class="w-full flex-1 min-h-0 flex items-center justify-center overflow-hidden py-1">
            {#if stageMessage}
              <p class="text-xl sm:text-2xl md:text-3xl font-black leading-tight text-white drop-shadow-[0_2px_12px_rgba(0,0,0,0.8)] break-words text-center">
                {stageMessage}
              </p>
            {:else}
              <div class="flex flex-col items-center justify-center text-center text-zinc-600 space-y-1">
                <span class="text-xs font-medium italic">Nenhum aviso ativo</span>
              </div>
            {/if}
          </div>

          <div class="text-[10px] font-mono uppercase tracking-wider shrink-0 text-right {stageMessage ? 'text-emerald-400/80' : 'text-zinc-600'}">
            {stageMessage ? 'Mensagem do Operador' : 'Canal do Operador'}
          </div>
        </div>

        <!-- Relógio de Parede -->
        <div
          class="p-4 rounded-none bg-zinc-900/60 border-b border-zinc-800 flex flex-col justify-center items-center text-center transition-all overflow-hidden"
          style="flex: 30; min-height: 0;"
        >
          <div class="text-[11px] text-zinc-400 uppercase tracking-widest font-semibold mb-1 flex items-center gap-1.5 shrink-0">
            <Clock size={14} />
            Horário
          </div>
          <div
            class="text-3xl sm:text-4xl md:text-5xl font-mono font-bold tracking-tight my-auto"
            style="color: {stageSettings.clock_color || '#ffffff'};"
          >
            {currentTime}
          </div>
          <div class="text-xs text-zinc-500 capitalize mt-1 truncate shrink-0">{currentDate}</div>
        </div>

        <!-- Timer Dedicado com Overtime -->
        <div
          class="p-4 rounded-none bg-zinc-900/60 flex flex-col justify-center items-center text-center relative overflow-hidden transition-all min-h-0"
          style="flex: 30;"
        >
          <div class="text-xs text-zinc-400 uppercase tracking-widest font-semibold mb-1 flex items-center gap-1.5 shrink-0">
            <Hourglass size={14} />
            {timer?.label || 'Cronômetro'}
          </div>

          {#if timer && timer.duration_seconds > 0}
            <div
              class="text-4xl sm:text-5xl md:text-6xl font-mono font-black tracking-tighter my-auto {timerIsOvertime ? 'text-red-500 animate-pulse' : timerIsNearEnd ? 'text-amber-400' : ''}"
              style="color: {timerIsOvertime ? '#ef4444' : timerIsNearEnd ? '#f59e0b' : stageSettings.timer_color || '#ef4444'};"
            >
              {formatTime(timer.remaining_seconds, timerIsOvertime)}
            </div>

            {#if timerIsOvertime}
              <div class="mt-1 text-xs font-bold uppercase tracking-wider text-red-400 flex items-center gap-1 bg-red-500/20 px-2 py-0.5 rounded-full shrink-0">
                <AlertTriangle size={12} />
                Tempo Excedido
              </div>
            {:else if timerIsNearEnd}
              <div class="mt-1 text-xs font-bold uppercase tracking-wider text-amber-400 bg-amber-500/20 px-2 py-0.5 rounded-full shrink-0">
                Último Minuto
              </div>
            {/if}
          {:else}
            <div class="text-3xl sm:text-4xl font-mono font-bold text-zinc-600 my-auto">
              00:00
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- Botão sutil de tela cheia no canto inferior direito -->
  <button
    type="button"
    onclick={toggleFullscreen}
    title="Alternar Tela Cheia (F)"
    class="absolute bottom-3 right-3 p-2 bg-neutral-900/60 hover:bg-neutral-800 text-neutral-400 hover:text-white rounded-md opacity-20 hover:opacity-100 transition-opacity z-40 cursor-pointer"
  >
    {#if isFullscreen}
      <Minimize size={18} />
    {:else}
      <Maximize size={18} />
    {/if}
  </button>
</main>

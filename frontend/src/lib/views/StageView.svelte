<script lang="ts">
  import { onMount } from 'svelte';
  import { presentationState, mediaItemsList, initWebSocket } from '$lib/ws';
  import type { MediaItem, Slide } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
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

  function formatTime(remaining: number, isOvertime: boolean): string {
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
  class="relative w-screen h-screen bg-[#070709] text-white flex flex-col p-5 overflow-hidden select-none font-sans"
  ondblclick={toggleFullscreen}
>
  <!-- Alerta se Blackout ou Limpar Texto estiver ativo -->
  {#if $presentationState.display_mode === 'blackout'}
    <div class="bg-red-500/20 border border-red-500/40 text-red-300 py-1.5 px-4 text-center text-xs font-semibold uppercase tracking-wider rounded mb-3 shrink-0">
      Projeção em Blackout (Tela Preta no Público)
    </div>
  {:else if $presentationState.display_mode === 'cleartext'}
    <div class="bg-amber-500/20 border border-amber-500/40 text-amber-300 py-1.5 px-4 text-center text-xs font-semibold uppercase tracking-wider rounded mb-3 shrink-0">
      Texto Oculto no Público (Clear Text)
    </div>
  {/if}

  <!-- ========================================================================= -->
  <!-- PRESET 1: SPLIT 50-50 (Divisão Equilibrada) -->
  <!-- ========================================================================= -->
  {#if stageSettings.preset === 'split-50-50'}
    <div class="flex-1 flex flex-col gap-4 min-h-0">
      <!-- Grade dinâmica com dois cartões com bordas e etiquetas -->
      <div
        class="flex-1 flex flex-col md:flex-row gap-4 min-h-0"
        style="flex: {100 - footerRatio};"
      >
        <!-- Slide Atual -->
        <div
          class="flex flex-col rounded-2xl bg-zinc-900/60 p-6 overflow-hidden shadow-2xl relative border-2 transition-all min-h-0 min-w-0"
          style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
        >
          <div class="flex items-center justify-between pb-3 border-b border-zinc-800 shrink-0">
            <span
              class="text-xs font-bold uppercase tracking-widest flex items-center gap-2"
              style="color: {stageSettings.current_text_color || '#ffffff'};"
            >
              <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
              Slide Atual (Público)
            </span>
            {#if activeSlide}
              <span class="text-xs font-semibold bg-zinc-800 text-zinc-300 px-2 py-0.5 rounded">
                {activeSlide.label}
              </span>
            {/if}
          </div>

          <div class="my-auto py-3 overflow-y-auto min-h-0 flex items-center justify-center">
            {#if isCurrentImage && activeSlide}
              <div class="w-full h-full flex flex-col items-center justify-center p-2 min-h-0">
                <img
                  src={activeSlide.content}
                  alt={activeMedia?.title || 'Imagem'}
                  class="max-w-full max-h-[60vh] object-contain rounded-xl shadow-2xl border border-zinc-800"
                />
                <span class="text-xs text-zinc-400 font-mono mt-2 truncate max-w-sm">{activeMedia?.title || 'Imagem'}</span>
              </div>
            {:else if activeSlide}
              <div
                class="text-4xl sm:text-5xl md:text-6xl font-black leading-tight text-center"
                style="color: {stageSettings.current_text_color || '#ffffff'};"
              >
                {@html renderFormattedText(activeSlide.content)}
              </div>
            {:else}
              <p class="text-zinc-600 text-xl font-medium text-center">
                Aguardando início da projeção
              </p>
            {/if}
          </div>

          <div class="pt-3 border-t border-zinc-800 text-xs text-zinc-500 flex justify-between font-mono shrink-0">
            <span>{activeMedia?.title || 'OpenPresenter Stage'}</span>
            {#if activeMedia && $presentationState.active_slide_index !== null}
              <span>Slide {$presentationState.active_slide_index + 1} de {activeMedia.slides.length}</span>
            {/if}
          </div>
        </div>

        <!-- Próximo Slide -->
        <div
          class="flex flex-col rounded-2xl bg-zinc-900/40 p-6 overflow-hidden shadow-2xl relative border-2 transition-all min-h-0 min-w-0"
          style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
        >
          <div class="flex items-center justify-between pb-3 border-b border-zinc-800 shrink-0">
            <span
              class="text-xs font-bold uppercase tracking-widest flex items-center gap-1.5"
              style="color: {stageSettings.next_text_color || '#facc15'};"
            >
              <ArrowRight size={14} />
              A Seguir (Próximo)
            </span>
            {#if nextSlide}
              <span class="text-xs font-semibold bg-zinc-800 text-zinc-400 px-2 py-0.5 rounded">
                {nextSlide.label}
              </span>
            {/if}
          </div>

          <div class="my-auto py-3 overflow-y-auto min-h-0 flex items-center justify-center">
            {#if isNextImage && nextSlide}
              <div class="w-full h-full flex flex-col items-center justify-center p-2 min-h-0">
                <img
                  src={nextSlide.content}
                  alt="Próxima imagem"
                  class="max-w-full max-h-[50vh] object-contain rounded-lg opacity-85 shadow border border-zinc-800"
                />
              </div>
            {:else if nextSlide}
              <div
                class="text-3xl sm:text-4xl md:text-5xl font-extrabold leading-tight opacity-90 text-center"
                style="color: {stageSettings.next_text_color || '#facc15'};"
              >
                {@html renderFormattedText(nextSlide.content)}
              </div>
            {:else if activeSlide}
              <p class="text-zinc-600 text-base italic text-center py-8">
                Fim desta música / apresentação
              </p>
            {:else}
              <p class="text-zinc-600 text-base text-center py-8">
                Nenhum item selecionado
              </p>
            {/if}
          </div>

          <div class="pt-3 border-t border-zinc-800 text-xs text-zinc-600 font-mono shrink-0 text-right">
            Próximo Slide
          </div>
        </div>
      </div>

      <!-- Barra de Rodapé: Relógio, Timer e Mensagem de Palco -->
      <footer
        class="bg-zinc-900/80 rounded-2xl border border-zinc-800 px-6 flex items-center justify-between shrink-0 transition-all"
        style="min-height: clamp(60px, {footerRatio * 1.5}vh, 120px);"
      >
        <!-- Relógio de Horário Atual -->
        <div class="flex items-center gap-3">
          <Clock size={20} class="text-zinc-400" />
          <div>
            <div class="text-[10px] text-zinc-500 uppercase tracking-wider font-semibold">Horário Local</div>
            <div
              class="text-2xl font-mono font-bold tracking-wider"
              style="color: {stageSettings.clock_color || '#ffffff'};"
            >
              {currentTime}
            </div>
          </div>
        </div>

        <!-- Mensagem de Palco ao Centro se houver -->
        {#if stageMessage}
          <div class="px-4 py-1.5 rounded-lg bg-amber-500/15 border border-amber-500/30 text-amber-300 text-sm font-semibold flex items-center gap-2 max-w-md truncate animate-in fade-in">
            <MessageSquare size={16} class="shrink-0" />
            <span class="truncate">{stageMessage}</span>
          </div>
        {/if}

        <!-- Timer / Cronômetro Regressivo -->
        {#if timer && timer.duration_seconds > 0}
          <div class="flex items-center gap-3">
            <div>
              <div class="text-[10px] text-zinc-500 uppercase tracking-wider font-semibold text-right">
                {timer.label || 'Tempo Restante'}
              </div>
              <div
                class="text-3xl font-mono font-extrabold tracking-tight {timerIsOvertime ? 'text-red-500 animate-pulse' : timerIsNearEnd ? 'text-amber-400' : ''}"
                style="color: {timerIsOvertime ? '#ef4444' : timerIsNearEnd ? '#f59e0b' : stageSettings.timer_color || '#ef4444'};"
              >
                {formatTime(timer.remaining_seconds, timerIsOvertime)}
              </div>
            </div>
            <Hourglass
              size={22}
              class={timerIsOvertime ? 'text-red-500 animate-spin' : timerIsNearEnd ? 'text-amber-400' : 'text-zinc-400'}
            />
          </div>
        {/if}
      </footer>
    </div>

  <!-- ========================================================================= -->
  <!-- PRESET 2: MODERN GRID (Slide atual grande, próximo abaixo, lateral com timer) -->
  <!-- ========================================================================= -->
  {:else if stageSettings.preset === 'modern-grid'}
    <div class="flex-1 flex flex-col md:flex-row gap-5 min-h-0">
      <!-- Coluna Principal (Slides) -->
      <div
        class="flex flex-col gap-4 min-h-0 transition-all"
        style="flex: {100 - (footerRatio * 2.2)}; min-width: 0;"
      >
        <!-- Slide Atual Grande -->
        <div
          class="flex flex-col rounded-2xl bg-zinc-900/70 p-3.5 sm:p-5 overflow-hidden shadow-2xl border-2 transition-all min-h-0"
          style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
        >
          <div class="flex items-center justify-between pb-2 border-b border-zinc-800 shrink-0">
            <span
              class="text-xs font-bold uppercase tracking-widest flex items-center gap-2"
              style="color: {stageSettings.current_text_color || '#ffffff'};"
            >
              <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
              Slide Atual
            </span>
            {#if activeSlide}
              <span class="text-xs font-semibold bg-zinc-800 text-zinc-300 px-2 py-0.5 rounded">
                {activeSlide.label}
              </span>
            {/if}
          </div>

          <div class="my-auto py-2 overflow-y-auto min-h-0 flex items-center justify-center">
            {#if isCurrentImage && activeSlide}
              <div class="w-full h-full flex flex-col items-center justify-center p-2 min-h-0">
                <img
                  src={activeSlide.content}
                  alt={activeMedia?.title || 'Imagem'}
                  class="max-w-full max-h-[50vh] object-contain rounded-xl shadow-2xl border border-zinc-800"
                />
                <span class="text-xs text-zinc-400 font-mono mt-1.5 truncate max-w-sm">{activeMedia?.title || 'Imagem'}</span>
              </div>
            {:else if activeSlide}
              <div
                class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black leading-tight text-center"
                style="color: {stageSettings.current_text_color || '#ffffff'};"
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
          class="rounded-2xl bg-zinc-900/40 p-2 sm:p-3 overflow-hidden border flex flex-col shrink-0 transition-all min-h-0"
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
            {#if nextSlide}
              <span class="text-[10px] bg-zinc-800 text-zinc-400 px-1.5 py-0.5 rounded">
                {nextSlide.label}
              </span>
            {/if}
          </div>

          <div class="my-auto overflow-y-auto py-1 min-h-0 flex items-center justify-center">
            {#if isNextImage && nextSlide}
              <div class="w-full h-full flex items-center justify-center p-1 min-h-0">
                <img
                  src={nextSlide.content}
                  alt="Próxima imagem"
                  class="max-w-full max-h-[22vh] object-contain rounded-lg opacity-85 shadow border border-zinc-800"
                />
              </div>
            {:else if nextSlide}
              <div
                class="text-2xl sm:text-3xl md:text-4xl font-extrabold leading-tight opacity-90 text-center"
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

      <!-- Coluna Lateral Direita (Tempo, Avisos e Relógio) -->
      <div
        class="flex flex-col gap-4 min-h-0 transition-all"
        style="flex: {footerRatio * 2.2}; min-width: 280px; max-width: 440px;"
      >
        <!-- Mensagem de Palco com visual proeminente -->
        {#if stageMessage}
          <div class="p-4 rounded-2xl bg-emerald-500/15 border-2 border-emerald-500/40 text-emerald-300 shadow-lg shrink-0">
            <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider mb-1 text-emerald-400">
              <MessageSquare size={14} />
              Aviso para o Palco
            </div>
            <p class="text-sm font-semibold leading-snug">{stageMessage}</p>
          </div>
        {/if}

        <!-- Relógio de Parede -->
        <div class="p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800 flex flex-col justify-center shrink-0">
          <div class="text-[11px] text-zinc-400 uppercase tracking-widest font-semibold mb-1 flex items-center gap-1.5">
            <Clock size={14} />
            Horário
          </div>
          <div
            class="text-4xl sm:text-5xl font-mono font-bold tracking-tight"
            style="color: {stageSettings.clock_color || '#ffffff'};"
          >
            {currentTime}
          </div>
          <div class="text-xs text-zinc-500 capitalize mt-1">{currentDate}</div>
        </div>

        <!-- Timer Grande Dedicado com Overtime Negativo -->
        <div class="flex-1 p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800 flex flex-col justify-center items-center text-center relative overflow-hidden min-h-0">
          <div class="text-xs text-zinc-400 uppercase tracking-widest font-semibold mb-2 flex items-center gap-1.5">
            <Hourglass size={14} />
            {timer?.label || 'Cronômetro'}
          </div>

          {#if timer && timer.duration_seconds > 0}
            <div
              class="text-5xl sm:text-6xl font-mono font-black tracking-tighter {timerIsOvertime ? 'text-red-500 animate-pulse' : timerIsNearEnd ? 'text-amber-400' : ''}"
              style="color: {timerIsOvertime ? '#ef4444' : timerIsNearEnd ? '#f59e0b' : stageSettings.timer_color || '#ef4444'};"
            >
              {formatTime(timer.remaining_seconds, timerIsOvertime)}
            </div>

            {#if timerIsOvertime}
              <div class="mt-2 text-xs font-bold uppercase tracking-wider text-red-400 flex items-center gap-1 bg-red-500/20 px-2.5 py-1 rounded-full">
                <AlertTriangle size={12} />
                Tempo Excedido (Overtime)
              </div>
            {:else if timerIsNearEnd}
              <div class="mt-2 text-xs font-bold uppercase tracking-wider text-amber-400 bg-amber-500/20 px-2.5 py-1 rounded-full">
                Último Minuto
              </div>
            {/if}
          {:else}
            <div class="text-3xl font-mono font-bold text-zinc-600">00:00</div>
            <p class="text-[11px] text-zinc-600 mt-2">Nenhum timer ativo</p>
          {/if}
        </div>
      </div>
    </div>

  <!-- ========================================================================= -->
  <!-- PRESET 3: VIDEO FOCUS (Foco central expandido, barra inferior de tempo) -->
  <!-- ========================================================================= -->
  {:else if stageSettings.preset === 'video-focus'}
    <div class="flex-1 flex flex-col gap-4 min-h-0">
      <!-- Área Central de Foco Máximo -->
      <div
        class="flex flex-col rounded-2xl bg-zinc-900/80 p-8 sm:p-12 overflow-hidden shadow-2xl border-2 justify-center items-center text-center relative transition-all min-h-0"
        style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
      >
        <div class="absolute top-4 left-6 flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
          <span class="text-xs font-bold uppercase tracking-widest text-zinc-400">Projeção Ativa</span>
          {#if activeSlide}
            <span class="text-xs bg-zinc-800 text-zinc-300 px-2 py-0.5 rounded font-semibold ml-2">
              {activeSlide.label}
            </span>
          {/if}
        </div>

        <div class="max-w-4xl overflow-y-auto my-auto w-full">
          {#if isCurrentImage && activeSlide}
            <div class="w-full h-full flex flex-col items-center justify-center p-2 min-h-0">
              <img
                src={activeSlide.content}
                alt={activeMedia?.title || 'Imagem'}
                class="max-w-full max-h-[55vh] object-contain rounded-xl shadow-2xl border border-zinc-800"
              />
              <span class="text-xs text-zinc-400 font-mono mt-2 truncate max-w-sm">{activeMedia?.title || 'Imagem'}</span>
            </div>
          {:else if activeSlide}
            <div
              class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black leading-tight tracking-tight drop-shadow-md text-center"
              style="color: {stageSettings.current_text_color || '#ffffff'};"
            >
              {@html renderFormattedText(activeSlide.content)}
            </div>
          {:else}
            <p class="text-zinc-600 text-2xl font-medium">Aguardando apresentação</p>
          {/if}
        </div>

        {#if stageMessage}
          <div class="absolute bottom-4 left-6 right-6 p-2 rounded-lg bg-emerald-500/20 border border-emerald-500/40 text-emerald-300 text-xs font-semibold truncate text-center">
            {stageMessage}
          </div>
        {/if}
      </div>

      <!-- Barra Inferior com Próximo Slide, Relógio e Timer -->
      <footer
        class="bg-zinc-900/60 rounded-2xl border border-zinc-800 p-4 flex items-center justify-between shrink-0 gap-4 transition-all"
        style="min-height: clamp(68px, {footerRatio * 1.5}vh, 130px);"
      >
        <!-- Próximo Slide Compacto -->
        <div class="flex-1 min-w-0 pr-4 border-r border-zinc-800">
          <div
            class="text-[10px] font-bold uppercase tracking-wider flex items-center gap-1 mb-1"
            style="color: {stageSettings.next_text_color || '#facc15'};"
          >
            <ArrowRight size={12} />
            Próximo
          </div>
          <div
            class="text-xl sm:text-2xl font-bold truncate opacity-90"
            style="color: {stageSettings.next_text_color || '#facc15'};"
          >
            {#if isNextImage && nextSlide}
              <span class="flex items-center gap-2">
                <img src={nextSlide.content} alt="Próxima" class="w-8 h-6 object-cover rounded shrink-0" />
                <span class="truncate">{nextSlide.label}</span>
              </span>
            {:else if nextSlide}
              {nextSlide.content.replace(/\n/g, ' ')}
            {:else}
              <span class="text-zinc-600 italic">Fim da apresentação</span>
            {/if}
          </div>
        </div>

        <!-- Relógio -->
        <div class="px-4 border-r border-zinc-800 text-center shrink-0">
          <div class="text-[10px] text-zinc-500 uppercase tracking-wider font-semibold">Relógio</div>
          <div
            class="text-2xl font-mono font-bold"
            style="color: {stageSettings.clock_color || '#ffffff'};"
          >
            {currentTime}
          </div>
        </div>

        <!-- Timer -->
        <div class="pl-2 text-right shrink-0">
          <div class="text-[10px] text-zinc-500 uppercase tracking-wider font-semibold">
            {timer?.label || 'Cronômetro'}
          </div>
          <div
            class="text-3xl font-mono font-black {timerIsOvertime ? 'text-red-500 animate-pulse' : timerIsNearEnd ? 'text-amber-400' : ''}"
            style="color: {timerIsOvertime ? '#ef4444' : timerIsNearEnd ? '#f59e0b' : stageSettings.timer_color || '#ef4444'};"
          >
            {#if timer && timer.duration_seconds > 0}
              {formatTime(timer.remaining_seconds, timerIsOvertime)}
            {:else}
              00:00
            {/if}
          </div>
        </div>
      </footer>
    </div>

  <!-- ========================================================================= -->
  <!-- PRESET 4: HORIZONTAL STACK (Barra topo, centro gigante, faixa na base) -->
  <!-- ========================================================================= -->
  {:else}
    <div class="flex-1 flex flex-col gap-3 min-h-0">
      <!-- Barra Topo com Relógio, Timer e Mensagem -->
      <header
        class="bg-zinc-900/80 rounded-xl border border-zinc-800 px-6 flex items-center justify-between shrink-0 transition-all"
        style="min-height: clamp(52px, {footerRatio * 1.2}vh, 85px);"
      >
        <div class="flex items-center gap-3">
          <Clock size={18} class="text-zinc-400" />
          <span
            class="text-2xl font-mono font-bold"
            style="color: {stageSettings.clock_color || '#ffffff'};"
          >
            {currentTime}
          </span>
        </div>

        {#if stageMessage}
          <div class="px-4 py-1 rounded bg-amber-500/20 border border-amber-500/40 text-amber-300 text-xs font-bold truncate max-w-lg">
            {stageMessage}
          </div>
        {/if}

        <div class="flex items-center gap-3">
          {#if timer && timer.duration_seconds > 0}
            <span
              class="text-2xl font-mono font-black {timerIsOvertime ? 'text-red-500 animate-pulse' : timerIsNearEnd ? 'text-amber-400' : ''}"
              style="color: {timerIsOvertime ? '#ef4444' : timerIsNearEnd ? '#f59e0b' : stageSettings.timer_color || '#ef4444'};"
            >
              {formatTime(timer.remaining_seconds, timerIsOvertime)}
            </span>
            <Hourglass size={18} class={timerIsOvertime ? 'text-red-500 animate-spin' : 'text-zinc-400'} />
          {:else}
            <span class="text-sm font-mono text-zinc-600">--:--</span>
          {/if}
        </div>
      </header>

      <!-- Centro: Slide Atual em Tipografia Gigante -->
      <div
        class="rounded-2xl bg-zinc-900/50 p-6 sm:p-10 overflow-hidden border-2 flex flex-col justify-center items-center text-center shadow-xl transition-all min-h-0"
        style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
      >
        <div class="my-auto overflow-y-auto max-w-5xl w-full">
          {#if isCurrentImage && activeSlide}
            <div class="w-full h-full flex flex-col items-center justify-center p-2 min-h-0">
              <img
                src={activeSlide.content}
                alt={activeMedia?.title || 'Imagem'}
                class="max-w-full max-h-[55vh] object-contain rounded-xl shadow-2xl border border-zinc-800"
              />
              <span class="text-xs text-zinc-400 font-mono mt-2 truncate max-w-sm">{activeMedia?.title || 'Imagem'}</span>
            </div>
          {:else if activeSlide}
            <div
              class="text-5xl sm:text-6xl md:text-7xl lg:text-8xl font-black leading-tight tracking-tight"
              style="color: {stageSettings.current_text_color || '#ffffff'};"
            >
              {@html renderFormattedText(activeSlide.content)}
            </div>
          {:else}
            <p class="text-zinc-600 text-2xl font-medium">Nenhum slide ativo</p>
          {/if}
        </div>
      </div>

      <!-- Base: Próximo Slide em Faixa Horizontal -->
      <footer
        class="rounded-xl bg-zinc-900/70 p-3 sm:p-4 border flex items-center justify-between shrink-0 px-6 transition-all"
        style="min-height: clamp(60px, {nextRatio * 0.7}vh, 120px); border-color: {stageSettings.next_border_color || '#eab308'};"
      >
        <div class="flex items-center gap-4 min-w-0 flex-1">
          <span
            class="text-xs font-bold uppercase tracking-widest shrink-0 flex items-center gap-1.5"
            style="color: {stageSettings.next_text_color || '#facc15'};"
          >
            <ArrowRight size={14} />
            A Seguir:
          </span>
          <div
            class="text-2xl sm:text-3xl font-extrabold truncate opacity-90"
            style="color: {stageSettings.next_text_color || '#facc15'};"
          >
            {#if isNextImage && nextSlide}
              <span class="flex items-center gap-2">
                <img src={nextSlide.content} alt="Próxima" class="w-10 h-7 object-cover rounded shrink-0 border border-zinc-700" />
                <span class="truncate">{nextSlide.label}</span>
              </span>
            {:else if nextSlide}
              {nextSlide.content.replace(/\n/g, ' ')}
            {:else}
              <span class="text-zinc-600 italic">Fim da mídia</span>
            {/if}
          </div>
        </div>

        {#if nextSlide}
          <span class="text-xs font-semibold bg-zinc-800 text-zinc-400 px-2.5 py-1 rounded ml-4 shrink-0">
            {nextSlide.label}
          </span>
        {/if}
      </footer>
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

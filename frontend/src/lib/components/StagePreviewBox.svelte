<script lang="ts">
  import type { MediaItem, Slide, StageDisplaySettings, TimerState } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
  import { Clock, Hourglass, ArrowRight, MessageSquare, Image as ImageIcon } from 'lucide-svelte';

  let {
    stageSettings,
    activeSlide,
    nextSlide,
    activeMedia,
    timer,
    stageClock,
  }: {
    stageSettings: StageDisplaySettings;
    activeSlide?: Slide;
    nextSlide?: Slide;
    activeMedia?: MediaItem;
    timer: TimerState;
    stageClock: string;
  } = $props();

  const isCurrentMediaImage = $derived(
    activeMedia?.kind === 'image' || (activeSlide?.content && activeSlide.content.startsWith('/media/'))
  );

  const isNextMediaImage = $derived(
    nextSlide?.content && nextSlide.content.startsWith('/media/')
  );

  const isScripture = $derived(activeMedia?.kind === 'scripture');

  // Proporções
  const currentRatio = $derived(stageSettings.current_slide_percent ?? 60);
  const nextRatio = $derived(stageSettings.next_slide_percent ?? 30);
  const footerRatio = $derived(stageSettings.footer_time_percent ?? 10);

  function formatTime(sec: number): string {
    const isNeg = sec < 0;
    const abs = Math.abs(sec);
    const m = Math.floor(abs / 60);
    const s = abs % 60;
    return `${isNeg ? '-' : ''}${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  }
</script>

<div class="w-full aspect-video rounded-lg bg-black border border-neutral-800 p-1.5 flex flex-col justify-between relative overflow-hidden shadow-inner select-none font-sans text-[10px]">
  {#if !activeSlide}
    <!-- MODO DE DESCANSO NO PREVIEW -->
    <div class="flex-1 flex flex-col items-center justify-center p-2 text-center bg-zinc-950/80 rounded border border-zinc-800/80 gap-1.5 animate-in fade-in duration-200">
      <div class="text-[8px] font-bold text-zinc-500 uppercase tracking-widest flex items-center gap-1">
        <Clock size={9} class="text-emerald-400" />
        <span>Modo Descanso (Palco)</span>
      </div>
      <div class="text-xl font-mono font-black text-white tracking-tight">
        {stageClock || '00:00:00'}
      </div>
      <div class="flex items-center gap-1.5 px-2 py-0.5 rounded bg-zinc-900 border border-zinc-800 text-[9px] font-mono font-bold {timer.remaining_seconds < 0 ? 'text-rose-400' : timer.remaining_seconds <= (timer.warning_threshold_seconds || 60) && timer.running ? 'text-amber-400' : 'text-zinc-300'}">
        <Hourglass size={8} class="opacity-70" />
        <span>{timer.label || 'Timer'}: {formatTime(timer.remaining_seconds)}</span>
      </div>
      <div class="text-[7.5px] text-zinc-600 truncate max-w-[90%]">
        Aguardando disparo de slide
      </div>
    </div>
  <!-- PRESET 1: SPLIT 50-50 (LADO A LADO) -->
  {:else if stageSettings.preset === 'split-50-50'}
    <div class="flex-1 flex gap-1 min-h-0" style="flex: {100 - footerRatio};">
      <!-- Slide Atual -->
      <div
        class="flex-1 rounded border p-1 flex flex-col justify-between overflow-hidden bg-zinc-900/60"
        style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
      >
        <span class="text-[8px] font-bold uppercase truncate" style="color: {stageSettings.current_text_color || '#ffffff'};">
          {isScripture ? `📖 ${activeSlide?.label || 'Bíblia'}` : `Atual: ${activeSlide?.label || 'Slide'}`}
        </span>
        <div class="my-auto overflow-hidden">
          {#if isCurrentMediaImage && activeSlide}
            <div class="w-full h-12 flex items-center justify-center">
              <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
            </div>
          {:else if activeSlide}
            <p class="text-[9px] font-bold leading-tight line-clamp-3" style="color: {stageSettings.current_text_color || '#ffffff'};">
              {activeSlide.content.split('\n')[0]}
            </p>
          {:else}
            <span class="text-zinc-600 text-[8px] italic">Sem slide</span>
          {/if}
        </div>
        <span class="text-[7px] text-zinc-500 truncate">{activeMedia?.title || 'OpenPresenter'}</span>
      </div>

      <!-- Próximo Slide -->
      <div
        class="flex-1 rounded border p-1 flex flex-col justify-between overflow-hidden bg-zinc-900/40"
        style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
      >
        <span class="text-[8px] font-bold uppercase truncate" style="color: {stageSettings.next_text_color || '#facc15'};">
          {isScripture && nextSlide ? `📖 ${nextSlide.label}` : 'A Seguir'}
        </span>
        <div class="my-auto overflow-hidden">
          {#if isNextMediaImage && nextSlide}
            <div class="w-full h-12 flex items-center justify-center opacity-80">
              <img src={nextSlide.content} alt="Próximo" class="max-w-full max-h-full object-contain rounded" />
            </div>
          {:else if nextSlide}
            <p class="text-[9px] font-semibold leading-tight line-clamp-3 opacity-90" style="color: {stageSettings.next_text_color || '#facc15'};">
              {nextSlide.content.split('\n')[0]}
            </p>
          {:else}
            <span class="text-zinc-600 text-[8px] italic">Fim</span>
          {/if}
        </div>
        <span class="text-[7px] text-zinc-600 text-right">Próximo</span>
      </div>
    </div>

    <!-- Barra de Rodapé: Relógio e Timer -->
    <div
      class="mt-1 pt-1 border-t border-zinc-800/80 flex items-center justify-between text-[8px] shrink-0 font-mono"
      style="height: {Math.max(16, Math.round(footerRatio * 0.4))}px;"
    >
      <div class="flex items-center gap-1 text-zinc-300">
        <Clock size={9} class="text-zinc-400" />
        <span style="color: {stageSettings.clock_color || '#ffffff'};">{stageClock || '00:00:00'}</span>
      </div>
      {#if timer.stage_message}
        <span class="text-[8px] text-emerald-400 font-bold truncate max-w-[100px]">{timer.stage_message}</span>
      {/if}
      <div class="flex items-center gap-1 font-bold">
        <span style="color: {timer.remaining_seconds <= 0 && timer.running ? '#ef4444' : stageSettings.timer_color || '#ef4444'};">
          {formatTime(timer.remaining_seconds)}
        </span>
      </div>
    </div>

  <!-- PRESET 2: MODERN GRID (ATUAL NO TOPO, PRÓXIMO ABAIXO, COLUNA LATERAL) -->
  {:else if stageSettings.preset === 'modern-grid'}
    <div class="flex-1 flex gap-1 min-h-0">
      <!-- Coluna Principal (Atual em cima, Próximo embaixo) -->
      <div class="flex-1 flex flex-col gap-1 min-h-0 min-w-0" style="flex: {100 - (footerRatio * 1.5)};">
        <!-- Atual -->
        <div
          class="rounded border p-1 overflow-hidden bg-zinc-900/70 flex flex-col justify-between"
          style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
        >
          <span class="text-[8px] font-bold uppercase truncate" style="color: {stageSettings.current_text_color || '#ffffff'};">
            ● {isScripture ? `📖 ${activeSlide?.label || 'Bíblia'}` : `Atual: ${activeSlide?.label || 'Slide'}`}
          </span>
          {#if isCurrentMediaImage && activeSlide}
            <div class="w-full h-10 flex items-center justify-center my-auto">
              <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
            </div>
          {:else if activeSlide}
            <p class="text-[10px] font-extrabold leading-tight line-clamp-2 my-auto" style="color: {stageSettings.current_text_color || '#ffffff'};">
              {activeSlide.content.split('\n')[0]}
            </p>
          {:else}
            <span class="text-zinc-600 text-[8px] italic my-auto">Aguardando slide</span>
          {/if}
        </div>

        <!-- Próximo -->
        <div
          class="rounded border p-1 overflow-hidden bg-zinc-900/40 flex flex-col justify-between"
          style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
        >
          <span class="text-[7px] font-bold uppercase truncate" style="color: {stageSettings.next_text_color || '#facc15'};">
            {isScripture && nextSlide ? `📖 ${nextSlide.label}` : 'A Seguir:'}
          </span>
          {#if isNextMediaImage && nextSlide}
            <div class="w-full h-8 flex items-center justify-center my-auto opacity-85">
              <img src={nextSlide.content} alt="Próximo" class="max-w-full max-h-full object-contain rounded" />
            </div>
          {:else if nextSlide}
            <p class="text-[10px] font-extrabold leading-tight line-clamp-1 my-auto opacity-95" style="color: {stageSettings.next_text_color || '#facc15'};">
              {nextSlide.content.split('\n')[0]}
            </p>
          {:else}
            <span class="text-zinc-600 text-[8px] italic my-auto">Fim</span>
          {/if}
        </div>
      </div>

      <!-- Coluna Lateral (Relógio, Timer, Mensagem) -->
      <div
        class="rounded border border-zinc-800 bg-zinc-900/50 p-1 flex flex-col gap-1 shrink-0 font-mono text-center overflow-hidden"
        style="width: {Math.max(48, Math.round(footerRatio * 4.5))}px;"
      >
        {#if timer.stage_message}
          <!-- Aviso de Palco (60%) -->
          <div
            class="p-0.5 rounded bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 flex flex-col justify-center overflow-hidden"
            style="flex: 60; min-height: 0;"
          >
            <span class="text-[6px] text-emerald-400 uppercase font-bold font-sans">Aviso</span>
            <p class="text-[8px] sm:text-[9px] font-black leading-tight text-white line-clamp-2 my-auto">
              {timer.stage_message}
            </p>
          </div>

          <!-- Horário (20%) -->
          <div
            class="rounded bg-zinc-900/80 p-0.5 flex flex-col justify-center overflow-hidden"
            style="flex: 20; min-height: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans">Hora</span>
            <span class="text-[8px] font-bold block" style="color: {stageSettings.clock_color || '#ffffff'};">
              {stageClock?.substring(0, 5) || '00:00'}
            </span>
          </div>

          <!-- Timer (20%) -->
          <div
            class="rounded bg-zinc-900/80 p-0.5 flex flex-col justify-center overflow-hidden"
            style="flex: 20; min-height: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans">Timer</span>
            <span
              class="text-[9px] font-black block tracking-tight"
              style="color: {timer.remaining_seconds <= 0 && timer.running ? '#ef4444' : stageSettings.timer_color || '#ef4444'};"
            >
              {formatTime(timer.remaining_seconds)}
            </span>
          </div>
        {:else}
          <!-- Horário (50%) -->
          <div
            class="rounded bg-zinc-900/60 p-0.5 flex flex-col justify-center overflow-hidden"
            style="flex: 50; min-height: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans">Hora</span>
            <span class="text-[9px] font-bold block my-auto" style="color: {stageSettings.clock_color || '#ffffff'};">
              {stageClock?.substring(0, 5) || '00:00'}
            </span>
          </div>

          <!-- Timer (50%) -->
          <div
            class="rounded bg-zinc-900/60 p-0.5 flex flex-col justify-center overflow-hidden"
            style="flex: 50; min-height: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans">Timer</span>
            <span
              class="text-[11px] font-black block tracking-tight my-auto"
              style="color: {timer.remaining_seconds <= 0 && timer.running ? '#ef4444' : stageSettings.timer_color || '#ef4444'};"
            >
              {formatTime(timer.remaining_seconds)}
            </span>
          </div>
        {/if}
      </div>
    </div>

  <!-- PRESET 3: VIDEO FOCUS (CENTRO EXPANDIDO, BARRA INFERIOR) -->
  {:else if stageSettings.preset === 'video-focus'}
    <div class="flex-1 flex flex-col gap-1 min-h-0">
      <div
        class="flex-1 rounded border p-2 flex flex-col justify-center items-center text-center bg-zinc-900/80 overflow-hidden"
        style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
      >
        {#if isCurrentMediaImage && activeSlide}
          <div class="w-full h-16 flex items-center justify-center">
            <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
          </div>
        {:else if activeSlide}
          <p class="text-[11px] font-black leading-tight line-clamp-3" style="color: {stageSettings.current_text_color || '#ffffff'};">
            {activeSlide.content.split('\n')[0]}
          </p>
        {:else}
          <span class="text-zinc-600 text-[8px] italic">Aguardando slide</span>
        {/if}
      </div>

      <div
        class="p-1 rounded border border-zinc-800 bg-zinc-900/60 flex items-center justify-between text-[8px] shrink-0 font-mono"
        style="height: {Math.max(18, Math.round(footerRatio * 1.5))}px;"
      >
        <span class="text-[8px] truncate max-w-[120px] font-sans" style="color: {stageSettings.next_text_color || '#facc15'};">
          Próx: {nextSlide ? nextSlide.content.split('\n')[0] : 'Fim'}
        </span>
        <div class="flex items-center gap-2">
          <span style="color: {stageSettings.clock_color || '#ffffff'};">{stageClock?.substring(0, 5) || '00:00'}</span>
          <span class="font-bold" style="color: {stageSettings.timer_color || '#ef4444'};">{formatTime(timer.remaining_seconds)}</span>
        </div>
      </div>
    </div>

  <!-- PRESET 4: HORIZONTAL STACK (HEADER TOPO, CENTRO GIGANTE, PRÓXIMO NA BASE) -->
  {:else}
    <div class="flex-1 flex flex-col gap-1 min-h-0">
      <!-- Topo: Relógio e Timer -->
      <div
        class="p-1 rounded border border-zinc-800 bg-zinc-900/80 flex items-center justify-between text-[8px] shrink-0 font-mono"
        style="height: {Math.max(16, Math.round(footerRatio * 1.2))}px;"
      >
        <div class="flex items-center gap-1">
          <Clock size={8} class="text-zinc-400" />
          <span style="color: {stageSettings.clock_color || '#ffffff'};">{stageClock || '00:00:00'}</span>
        </div>
        {#if timer.stage_message}
          <span class="text-emerald-400 text-[8px] font-bold truncate max-w-[120px]">{timer.stage_message}</span>
        {/if}
        <div class="flex items-center gap-1 font-bold" style="color: {stageSettings.timer_color || '#ef4444'};">
          <Hourglass size={8} />
          <span>{formatTime(timer.remaining_seconds)}</span>
        </div>
      </div>

      <!-- Centro -->
      <div
        class="flex-1 rounded border p-2 flex flex-col justify-center items-center text-center bg-zinc-900/60 overflow-hidden"
        style="flex: {currentRatio}; border-color: {stageSettings.current_border_color || '#3b82f6'};"
      >
        {#if isCurrentMediaImage && activeSlide}
          <div class="w-full h-14 flex items-center justify-center">
            <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
          </div>
        {:else if activeSlide}
          <p class="text-[12px] font-black leading-tight line-clamp-2" style="color: {stageSettings.current_text_color || '#ffffff'};">
            {activeSlide.content.split('\n')[0]}
          </p>
        {:else}
          <span class="text-zinc-600 text-[8px] italic">Aguardando slide</span>
        {/if}
      </div>

      <!-- Base -->
      <div
        class="p-1 rounded border flex items-center justify-between bg-zinc-900/70 text-[8px]"
        style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'}; color: {stageSettings.next_text_color || '#facc15'};"
      >
        <span class="font-bold text-[7px] uppercase tracking-wider shrink-0 mr-1">Próx:</span>
        <span class="truncate flex-1 font-medium">
          {nextSlide ? nextSlide.content.split('\n')[0] : 'Fim da mídia'}
        </span>
      </div>
    </div>
  {/if}
</div>

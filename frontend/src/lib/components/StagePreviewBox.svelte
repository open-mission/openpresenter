<script lang="ts">
  import type { MediaItem, Slide, StageDisplaySettings, TimerState } from '$lib/types';
  import { Clock, Hourglass, ArrowRight, MessageSquare } from 'lucide-svelte';

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

  // Proporções
  const currentRatio = $derived(stageSettings.current_slide_percent ?? 60);
  const nextRatio = $derived(stageSettings.next_slide_percent ?? 30);
  const footerRatio = $derived(stageSettings.footer_time_percent ?? 10);

  // Escala dinâmica de fontes para preview proporcional
  const previewClockSize = $derived(
    Math.max(6, Math.min(14, Math.round(8 * ((stageSettings.clock_font_size || 48) / 48))))
  );
  const previewTimerSize = $derived(
    Math.max(7, Math.min(16, Math.round(9 * ((stageSettings.timer_font_size || 56) / 56))))
  );
  const previewAlertSize = $derived(
    Math.max(6, Math.min(13, Math.round(7.5 * ((stageSettings.alert_font_size || 28) / 28))))
  );

  function formatTime(sec: number): string {
    const isNeg = sec < 0;
    const abs = Math.abs(sec);
    const m = Math.floor(abs / 60);
    const s = abs % 60;
    return `${isNeg ? '-' : ''}${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  }
</script>

<div class="w-full aspect-video rounded bg-black border border-neutral-800 p-0 flex flex-col justify-between relative overflow-hidden shadow-inner select-none font-sans text-[10px]">
  {#if !activeSlide}
    <!-- MODO DE DESCANSO NO PREVIEW -->
    <div class="flex-1 flex flex-col items-center justify-center p-2 text-center bg-zinc-950/80 rounded-none gap-1.5 animate-in fade-in duration-200">
      <div class="text-2xl font-mono font-black text-white tracking-tight">
        {stageClock || '00:00:00'}
      </div>
      {#if timer && (timer.running || timer.remaining_seconds < 0)}
        <div class="flex items-center gap-1.5 px-2 py-0.5 rounded bg-zinc-900 border border-zinc-800 text-[9px] font-mono font-bold {timer.remaining_seconds < 0 ? 'text-rose-400' : timer.remaining_seconds <= (timer.warning_threshold_seconds || 60) && timer.running ? 'text-amber-400' : 'text-zinc-300'}">
          <Hourglass size={8} class="opacity-70" />
          <span>{timer.label || 'Timer'}: {formatTime(timer.remaining_seconds)}</span>
        </div>
      {/if}
    </div>
  {:else}
    <!-- PRESET: MODO COLUNA LATERAL OU MODO 3 LINHAS EMPILHADAS -->
    {#if stageSettings.stage_layout === 'side-column'}
      <div class="flex-1 flex gap-0 min-h-0 w-full h-full">
        <!-- Coluna Principal (Atual em cima, Próximo embaixo) -->
        <div class="flex-1 flex flex-col gap-0 min-h-0 min-w-0 border-r border-zinc-800" style="flex: {100 - (footerRatio * 1.5)};">
          <!-- Atual -->
          <div
            class="relative rounded-none border-b p-1 overflow-hidden bg-zinc-900/70 flex flex-col justify-center"
            style="flex: {currentRatio}; border-color: {isScripture ? (stageSettings.scripture_border_color || stageSettings.current_border_color || '#3b82f6') : (stageSettings.current_border_color || '#3b82f6')};"
          >
            <span class="absolute top-0.5 left-1 text-[7px] font-bold uppercase truncate opacity-75 pointer-events-none" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
              ● {isScripture ? `${activeSlide?.label || 'Bíblia'}` : `Atual: ${activeSlide?.label || 'Slide'}`}
            </span>
            {#if isCurrentMediaImage && activeSlide}
              <div class="w-full h-10 flex items-center justify-center my-auto">
                <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
              </div>
            {:else if isSchedule && activeSlide}
              {@const ev = parseScheduleContent(activeSlide.label, activeSlide.content)}
              <div class="flex flex-col items-center justify-center my-auto px-1 text-center">
                <span
                  class="text-amber-300 font-bold uppercase truncate"
                  style="font-size: {Math.max(6, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 0.38))}px;"
                >
                  {ev.day} {ev.time ? `• ${ev.time}` : ''}
                </span>
                <p
                  class="font-black text-white leading-tight line-clamp-1 truncate"
                  style="font-size: {Math.max(7, Math.round((stageSettings.stage_schedule_title_min_font_size ?? 28) * 0.32))}px;"
                >
                  {ev.title}
                </p>
                {#if ev.details}
                  <span
                    class="text-zinc-400 font-medium truncate"
                    style="font-size: {Math.max(5.5, Math.round((stageSettings.stage_schedule_details_font_size ?? 18) * 0.32))}px;"
                  >
                    {ev.details}
                  </span>
                {/if}
              </div>
            {:else if activeSlide}
              <p class="text-[10px] font-extrabold leading-tight line-clamp-2 my-auto" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
                {activeSlide.content.split('\n')[0]}
              </p>
            {:else}
              <span class="text-zinc-600 text-[8px] italic my-auto">Aguardando slide</span>
            {/if}
          </div>

          <!-- Próximo -->
          <div
            class="relative rounded-none p-1 overflow-hidden bg-zinc-900/40 flex flex-col justify-center"
            style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
          >
            <span class="absolute top-0.5 left-1 text-[6.5px] font-bold uppercase truncate flex items-center gap-0.5 opacity-75 pointer-events-none" style="color: {stageSettings.next_text_color || '#facc15'};">
              <ArrowRight size={7} />
              {isScripture && nextSlide ? `${nextSlide.label}` : 'A Seguir:'}
            </span>
            {#if isNextMediaImage && nextSlide}
              <div class="w-full h-8 flex items-center justify-center my-auto opacity-85">
                <img src={nextSlide.content} alt="Próximo" class="max-w-full max-h-full object-contain rounded" />
              </div>
            {:else if isSchedule && nextSlide}
              {@const nextEv = parseScheduleContent(nextSlide.label, nextSlide.content)}
              <div class="flex flex-col items-center justify-center my-auto px-1 text-center opacity-90">
                <span
                  class="text-yellow-300 font-bold uppercase truncate"
                  style="font-size: {Math.max(5.5, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 0.35))}px;"
                >
                  {nextEv.day} {nextEv.time ? `• ${nextEv.time}` : ''}
                </span>
                <p
                  class="font-extrabold text-white leading-tight line-clamp-1 truncate"
                  style="font-size: {Math.max(6.5, Math.round((stageSettings.stage_schedule_next_min_font_size ?? 18) * 0.35))}px;"
                >
                  {nextEv.title}
                </p>
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

        <!-- Coluna Lateral (Aviso Sempre Visível, Relógio, Timer) -->
        <div
          class="rounded-none border-l border-zinc-800 bg-zinc-900/50 p-0 flex flex-col gap-0 shrink-0 font-mono text-center overflow-hidden"
          style="width: {Math.max(52, Math.round(footerRatio * 4.8))}px;"
        >
          <!-- Aviso de Palco: Sempre Visível -->
          <div
            class="p-0.5 rounded-none border-b border-zinc-800 flex flex-col justify-center overflow-hidden {timer.stage_message ? 'bg-emerald-500/20 text-emerald-300' : 'bg-zinc-950/60 text-zinc-600'}"
            style="flex: 40; min-height: 0;"
          >
            <span class="text-[6px] uppercase font-bold font-sans flex items-center justify-center gap-0.5 {timer.stage_message ? 'text-emerald-400' : 'text-zinc-500'}">
              <MessageSquare size={7} />
              Aviso
            </span>
            {#if timer.stage_message}
              <p class="font-black leading-tight text-white line-clamp-2 my-auto" style="font-size: {previewAlertSize}px;">
                {timer.stage_message}
              </p>
            {:else}
              <span class="text-[6px] text-zinc-600 italic my-auto">-</span>
            {/if}
          </div>

          <!-- Horário -->
          <div
            class="rounded-none bg-zinc-900/80 p-0.5 border-b border-zinc-800 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans">Hora</span>
            <span class="font-bold block my-auto" style="color: {stageSettings.clock_color || '#ffffff'}; font-size: {previewClockSize}px;">
              {stageClock?.substring(0, 5) || '00:00'}
            </span>
          </div>

          <!-- Timer -->
          <div
            class="rounded-none bg-zinc-900/80 p-0.5 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans truncate">{timer?.label || 'Timer'}</span>
            <span
              class="font-black block tracking-tight my-auto"
              style="color: {timer.remaining_seconds <= 0 && timer.running ? '#ef4444' : stageSettings.timer_color || '#ef4444'}; font-size: {previewTimerSize}px;"
            >
              {formatTime(timer.remaining_seconds)}
            </span>
          </div>
        </div>
      </div>
    {:else}
      <!-- MODO NOVO PADRÃO: 3 LINHAS EMPILHADAS (100% DA LARGURA) -->
      <div class="flex-1 flex flex-col gap-0 min-h-0 w-full h-full">
        <!-- Linha 1: Slide Atual -->
        <div
          class="relative rounded-none border-b p-1 overflow-hidden bg-zinc-900/70 flex flex-col justify-center min-h-0"
          style="flex: {currentRatio}; border-color: {isScripture ? (stageSettings.scripture_border_color || stageSettings.current_border_color || '#3b82f6') : (stageSettings.current_border_color || '#3b82f6')};"
        >
          <span class="absolute top-0.5 left-1 text-[7px] font-bold uppercase truncate opacity-75 pointer-events-none" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
            ● {isScripture ? `${activeSlide?.label || 'Bíblia'}` : `Atual: ${activeSlide?.label || 'Slide'}`}
          </span>
          {#if isCurrentMediaImage && activeSlide}
            <div class="w-full h-10 flex items-center justify-center my-auto">
              <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
            </div>
          {:else if isSchedule && activeSlide}
            {@const ev = parseScheduleContent(activeSlide.label, activeSlide.content)}
            <div class="flex flex-col items-center justify-center my-auto px-1 text-center">
              <span
                class="text-amber-300 font-bold uppercase truncate"
                style="font-size: {Math.max(6, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 0.38))}px;"
              >
                {ev.day} {ev.time ? `• ${ev.time}` : ''}
              </span>
              <p
                class="font-black text-white leading-tight line-clamp-1 truncate"
                style="font-size: {Math.max(7, Math.round((stageSettings.stage_schedule_title_min_font_size ?? 28) * 0.32))}px;"
              >
                {ev.title}
              </p>
              {#if ev.details}
                <span
                  class="text-zinc-400 font-medium truncate"
                  style="font-size: {Math.max(5.5, Math.round((stageSettings.stage_schedule_details_font_size ?? 18) * 0.32))}px;"
                >
                  {ev.details}
                </span>
              {/if}
            </div>
          {:else if activeSlide}
            <p class="text-[10px] font-extrabold leading-tight line-clamp-2 my-auto" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
              {activeSlide.content.split('\n')[0]}
            </p>
          {:else}
            <span class="text-zinc-600 text-[8px] italic my-auto">Aguardando slide</span>
          {/if}
        </div>

        <!-- Linha 2: Próximo Slide -->
        <div
          class="relative rounded-none border-b p-1 overflow-hidden bg-zinc-900/40 flex flex-col justify-center min-h-0"
          style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
        >
          <span class="absolute top-0.5 left-1 text-[6.5px] font-bold uppercase truncate flex items-center gap-0.5 opacity-75 pointer-events-none" style="color: {stageSettings.next_text_color || '#facc15'};">
            <ArrowRight size={7} />
            {isScripture && nextSlide ? `${nextSlide.label}` : 'A Seguir:'}
          </span>
          {#if isNextMediaImage && nextSlide}
            <div class="w-full h-8 flex items-center justify-center my-auto opacity-85">
              <img src={nextSlide.content} alt="Próximo" class="max-w-full max-h-full object-contain rounded" />
            </div>
          {:else if isSchedule && nextSlide}
            {@const nextEv = parseScheduleContent(nextSlide.label, nextSlide.content)}
            <div class="flex flex-col items-center justify-center my-auto px-1 text-center opacity-90">
              <span
                class="text-yellow-300 font-bold uppercase truncate"
                style="font-size: {Math.max(5.5, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 0.35))}px;"
              >
                {nextEv.day} {nextEv.time ? `• ${nextEv.time}` : ''}
              </span>
              <p
                class="font-extrabold text-white leading-tight line-clamp-1 truncate"
                style="font-size: {Math.max(6.5, Math.round((stageSettings.stage_schedule_next_min_font_size ?? 18) * 0.35))}px;"
              >
                {nextEv.title}
              </p>
            </div>
          {:else if nextSlide}
            <p class="text-[10px] font-extrabold leading-tight line-clamp-1 my-auto opacity-95" style="color: {stageSettings.next_text_color || '#facc15'};">
              {nextSlide.content.split('\n')[0]}
            </p>
          {:else}
            <span class="text-zinc-600 text-[8px] italic my-auto">Fim</span>
          {/if}
        </div>

        <!-- Linha 3: Barra Inferior Horizontal (Aviso, Hora, Timer) -->
        <div
          class="rounded-none bg-zinc-900/50 p-0 flex flex-row gap-0 shrink-0 font-mono text-center overflow-hidden min-h-0"
          style="flex: {footerRatio};"
        >
          <!-- Aviso de Palco -->
          <div
            class="p-0.5 rounded-none border-r border-zinc-800 flex flex-col justify-center overflow-hidden {timer.stage_message ? 'bg-emerald-500/20 text-emerald-300' : 'bg-zinc-950/60 text-zinc-600'}"
            style="flex: 40; min-height: 0; min-width: 0;"
          >
            <span class="text-[6px] uppercase font-bold font-sans flex items-center justify-center gap-0.5 truncate {timer.stage_message ? 'text-emerald-400' : 'text-zinc-500'}">
              <MessageSquare size={7} />
              Aviso
            </span>
            {#if timer.stage_message}
              <p class="font-black leading-tight text-white line-clamp-1 my-auto" style="font-size: {previewAlertSize}px;">
                {timer.stage_message}
              </p>
            {:else}
              <span class="text-[6px] text-zinc-600 italic my-auto">-</span>
            {/if}
          </div>

          <!-- Horário -->
          <div
            class="rounded-none bg-zinc-900/80 p-0.5 border-r border-zinc-800 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0; min-width: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans">Hora</span>
            <span class="font-bold block my-auto truncate" style="color: {stageSettings.clock_color || '#ffffff'}; font-size: {previewClockSize}px;">
              {stageClock?.substring(0, 5) || '00:00'}
            </span>
          </div>

          <!-- Timer -->
          <div
            class="rounded-none bg-zinc-900/80 p-0.5 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0; min-width: 0;"
          >
            <span class="text-[6px] text-zinc-500 uppercase block font-sans truncate">{timer?.label || 'Timer'}</span>
            <span
              class="font-black block tracking-tight my-auto truncate"
              style="color: {timer.remaining_seconds <= 0 && timer.running ? '#ef4444' : stageSettings.timer_color || '#ef4444'}; font-size: {previewTimerSize}px;"
            >
              {formatTime(timer.remaining_seconds)}
            </span>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>

<script lang="ts">
  import type { MediaItem, Slide, StageDisplaySettings, TimerState } from '$lib/types';
  import { Clock, Hourglass, ArrowRight, MessageSquare, AlertTriangle } from 'lucide-svelte';

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
    activeMedia?.kind === 'image' ||
      (activeSlide?.content &&
        (activeSlide.content.startsWith('/media/') ||
          activeSlide.content.startsWith('/api/media/') ||
          activeSlide.content.startsWith('http://') ||
          activeSlide.content.startsWith('https://') ||
          activeSlide.content.startsWith('data:image/') ||
          /\.(jpg|jpeg|png|webp|gif|svg|bmp|tiff)$/i.test(activeSlide.content)))
  );

  const isNextMediaImage = $derived(
    nextSlide?.content &&
      (nextSlide.content.startsWith('/media/') ||
        nextSlide.content.startsWith('/api/media/') ||
        nextSlide.content.startsWith('http://') ||
        nextSlide.content.startsWith('https://') ||
        nextSlide.content.startsWith('data:image/') ||
        /\.(jpg|jpeg|png|webp|gif|svg|bmp|tiff)$/i.test(nextSlide.content))
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

  let containerWidth = $state<number>(320);
  const scale = $derived(containerWidth > 0 ? containerWidth / 960 : 0.3333);

  const currentDateLabel = $derived.by(() => {
    void stageClock;
    try {
      return new Intl.DateTimeFormat('pt-BR', {
        weekday: 'long',
        day: '2-digit',
        month: 'long',
      }).format(new Date());
    } catch {
      return '';
    }
  });

  // Escala dinâmica de fontes dentro do canvas virtual 960x540 (0.5x de 1920x1080)
  const displayClock = $derived.by(() => {
    if (!stageClock) return stageSettings.clock_show_seconds === false ? '00:00' : '00:00:00';
    if (stageSettings.clock_show_seconds === false) {
      return stageClock.length >= 5 ? stageClock.substring(0, 5) : stageClock;
    }
    return stageClock;
  });

  const previewClockSize = $derived(
    Math.max(18, Math.min(44, Math.round(28 * ((stageSettings.clock_font_size || 48) / 48))))
  );
  const previewTimerSize = $derived(
    Math.max(20, Math.min(48, Math.round(32 * ((stageSettings.timer_font_size || 56) / 56))))
  );
  const previewAlertSize = $derived(
    Math.max(16, Math.min(36, Math.round(22 * ((stageSettings.alert_font_size || 28) / 28))))
  );
  const previewStandbyAlertSize = $derived(
    Math.max(18, Math.min(42, Math.round(26 * ((stageSettings.standby_alert_font_size || stageSettings.alert_font_size || 36) / 36))))
  );

  function formatTime(sec: number): string {
    const isNeg = sec < 0;
    const abs = Math.abs(sec);
    const m = Math.floor(abs / 60);
    const s = abs % 60;
    return `${isNeg ? '-' : ''}${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  }
</script>

<div
  bind:clientWidth={containerWidth}
  class="w-full aspect-video rounded-lg bg-black border border-border p-0 relative overflow-hidden shadow-inner select-none font-sans"
>
  <!-- Canvas Virtual 960x540 Espelhado via CSS Transform -->
  <div
    style="width: 960px; height: 540px; transform: scale({scale}); transform-origin: top left; position: absolute; top: 0; left: 0; pointer-events: none;"
    class="bg-black text-white flex flex-col justify-between overflow-hidden select-none"
  >
    {#if !activeSlide}
      <!-- MODO DE DESCANSO NO PREVIEW -->
      <div class="flex-1 flex flex-col items-center justify-center p-8 text-center bg-zinc-950/90 gap-4 animate-in fade-in duration-200">
        {#if timer?.stage_message}
          <div
            class="px-5 py-2 rounded-xl bg-amber-500/20 border border-amber-500/50 text-amber-200 font-bold flex items-center gap-3 max-w-[90%] truncate mb-2 shadow-sm"
            style="font-size: {previewStandbyAlertSize}px;"
          >
            <AlertTriangle size={Math.max(20, previewStandbyAlertSize - 2)} class="shrink-0 text-amber-300" />
            <span class="truncate">{timer.stage_message}</span>
          </div>
        {/if}
        <div class="text-7xl font-mono font-black text-white tracking-tight">
          {displayClock}
        </div>
        {#if stageSettings.clock_show_date !== false && currentDateLabel}
          <span class="text-2xl text-zinc-300 capitalize">{currentDateLabel}</span>
        {/if}
        {#if timer && (timer.running || timer.remaining_seconds < 0)}
          <div class="flex items-center gap-3 px-4 py-1.5 rounded-lg bg-zinc-900 border border-zinc-800 text-2xl font-mono font-bold {timer.remaining_seconds < 0 ? 'text-rose-400' : timer.remaining_seconds <= (timer.warning_threshold_seconds || 60) && timer.running ? 'text-amber-400' : 'text-zinc-200'}">
            <Hourglass size={22} class="opacity-80" />
            <span>{timer.label || 'Timer'}: {formatTime(timer.remaining_seconds)}</span>
          </div>
        {/if}
      </div>
    {:else if stageSettings.stage_layout === 'side-column'}
      <!-- PRESET: MODO COLUNA LATERAL -->
      <div class="flex-1 flex gap-0 min-h-0 w-full h-full">
        <!-- Coluna Principal (Atual em cima, Próximo embaixo) -->
        <div class="flex-1 flex flex-col gap-0 min-h-0 min-w-0 border-r border-zinc-800" style="flex: {100 - (footerRatio * 1.5)};">
          <!-- Atual -->
          <div
            class="relative border-b p-4 pt-9 overflow-hidden bg-zinc-900/70 flex flex-col justify-center min-h-0"
            style="flex: {isCurrentMediaImage ? (currentRatio + nextRatio) : currentRatio}; border-color: {isScripture ? (stageSettings.scripture_border_color || stageSettings.current_border_color || '#3b82f6') : (stageSettings.current_border_color || '#3b82f6')};"
          >
            <span class="absolute top-2.5 left-3.5 text-sm font-bold uppercase truncate opacity-85" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
              ● {isScripture ? `${activeSlide?.label || 'Bíblia'}` : `Atual: ${activeSlide?.label || 'Slide'}`}
            </span>
            {#if isCurrentMediaImage && activeSlide}
              <div class="w-full h-full flex items-center justify-center my-auto p-1 min-h-0 overflow-hidden">
                <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
              </div>
            {:else if isSchedule && activeSlide}
              {@const ev = parseScheduleContent(activeSlide.label, activeSlide.content)}
              <div class="flex flex-col items-center justify-center my-auto px-3 text-center gap-1">
                <span
                  class="font-bold uppercase truncate"
                  style="color: {stageSettings.stage_schedule_day_color || '#fcd34d'}; font-size: {Math.max(16, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 1.1))}px;"
                >
                  {ev.day} {#if ev.time}<span style="color: {stageSettings.stage_schedule_time_color || '#ffffff'};">• {ev.time}</span>{/if}
                </span>
                <p
                  class="font-black text-white leading-tight line-clamp-1 truncate"
                  style="font-size: {Math.max(22, Math.round((stageSettings.stage_schedule_title_min_font_size ?? 28) * 1.0))}px;"
                >
                  {ev.title}
                </p>
                {#if ev.details}
                  <span
                    class="text-zinc-300 font-medium truncate"
                    style="font-size: {Math.max(16, Math.round((stageSettings.stage_schedule_details_font_size ?? 18) * 0.95))}px;"
                  >
                    {ev.details}
                  </span>
                {/if}
              </div>
            {:else if activeSlide}
              <p class="text-[28px] font-extrabold leading-tight line-clamp-2 my-auto" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
                {activeSlide.content.split('\n')[0]}
              </p>
            {:else}
              <span class="text-zinc-400 text-lg italic my-auto">Aguardando slide</span>
            {/if}
          </div>

          <!-- Próximo (Ocultado quando for imagem em tela cheia) -->
          {#if !isCurrentMediaImage}
            <div
              class="relative p-4 pt-9 overflow-hidden bg-zinc-900/40 flex flex-col justify-center min-h-0"
              style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
            >
              <span class="absolute top-2.5 left-3.5 text-sm font-bold uppercase truncate flex items-center gap-1.5 opacity-85" style="color: {stageSettings.next_text_color || '#facc15'};">
                <ArrowRight size={16} />
                {isScripture && nextSlide ? `${nextSlide.label}` : 'A Seguir:'}
              </span>
              {#if isNextMediaImage && nextSlide}
                <div class="w-full h-24 flex items-center justify-center my-auto opacity-85">
                  <img src={nextSlide.content} alt="Próximo" class="max-w-full max-h-full object-contain rounded" />
                </div>
              {:else if isSchedule && nextSlide}
                {@const nextEv = parseScheduleContent(nextSlide.label, nextSlide.content)}
                <div class="flex flex-col items-center justify-center my-auto px-3 text-center opacity-90 gap-1">
                  <span
                    class="text-yellow-300 font-bold uppercase truncate"
                    style="font-size: {Math.max(15, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 0.95))}px;"
                  >
                    {nextEv.day} {nextEv.time ? `• ${nextEv.time}` : ''}
                  </span>
                  <p
                    class="font-extrabold text-white leading-tight line-clamp-1 truncate"
                    style="font-size: {Math.max(18, Math.round((stageSettings.stage_schedule_next_min_font_size ?? 18) * 1.0))}px;"
                  >
                    {nextEv.title}
                  </p>
                </div>
              {:else if nextSlide}
                <p class="text-2xl font-extrabold leading-tight line-clamp-1 my-auto opacity-95" style="color: {stageSettings.next_text_color || '#facc15'};">
                  {nextSlide.content.split('\n')[0]}
                </p>
              {:else}
                <span class="text-zinc-400 text-lg italic my-auto">Fim</span>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Coluna Lateral (Aviso Sempre Visível, Relógio, Timer) -->
        <div
          class="border-l border-zinc-800 bg-zinc-900/50 p-0 flex flex-col gap-0 shrink-0 font-mono text-center overflow-hidden"
          style="width: {Math.max(190, Math.round(footerRatio * 14))}px;"
        >
          <!-- Aviso de Palco -->
          <div
            class="p-2 border-b border-zinc-800 flex flex-col justify-center overflow-hidden {timer.stage_message ? 'bg-emerald-500/20 text-emerald-200' : 'bg-zinc-950/60 text-zinc-400'}"
            style="flex: 40; min-height: 0;"
          >
            <span class="text-xs uppercase font-bold font-sans flex items-center justify-center gap-1 {timer.stage_message ? 'text-emerald-300' : 'text-zinc-400'}">
              <MessageSquare size={14} />
              Aviso
            </span>
            {#if timer.stage_message}
              <p class="font-black leading-tight text-white line-clamp-2 my-auto" style="font-size: {previewAlertSize}px;">
                {timer.stage_message}
              </p>
            {:else}
              <span class="text-sm text-zinc-400 italic my-auto">—</span>
            {/if}
          </div>

          <!-- Horário -->
          <div
            class="bg-zinc-900/80 p-2 border-b border-zinc-800 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0;"
          >
            <span class="text-xs text-zinc-400 uppercase block font-sans">Hora</span>
            <span class="font-bold block my-auto" style="color: {stageSettings.clock_color || '#ffffff'}; font-size: {previewClockSize}px;">
              {displayClock}
            </span>
          </div>

          <!-- Timer -->
          <div
            class="bg-zinc-900/80 p-2 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0;"
          >
            <span class="text-xs text-zinc-400 uppercase block font-sans truncate">{timer?.label || 'Timer'}</span>
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
          class="relative border-b p-4 pt-9 overflow-hidden bg-zinc-900/70 flex flex-col justify-center min-h-0"
          style="flex: {isCurrentMediaImage ? (currentRatio + nextRatio) : currentRatio}; border-color: {isScripture ? (stageSettings.scripture_border_color || stageSettings.current_border_color || '#3b82f6') : (stageSettings.current_border_color || '#3b82f6')};"
        >
          <span class="absolute top-2.5 left-3.5 text-sm font-bold uppercase truncate opacity-85" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
            ● {isScripture ? `${activeSlide?.label || 'Bíblia'}` : `Atual: ${activeSlide?.label || 'Slide'}`}
          </span>
          {#if isCurrentMediaImage && activeSlide}
            <div class="w-full h-full flex items-center justify-center my-auto p-1 min-h-0 overflow-hidden">
              <img src={activeSlide.content} alt="Atual" class="max-w-full max-h-full object-contain rounded" />
            </div>
          {:else if isSchedule && activeSlide}
            {@const ev = parseScheduleContent(activeSlide.label, activeSlide.content)}
            <div class="flex flex-col items-center justify-center my-auto px-3 text-center gap-1">
              <span
                class="font-bold uppercase truncate"
                style="color: {stageSettings.stage_schedule_day_color || '#fcd34d'}; font-size: {Math.max(16, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 1.1))}px;"
              >
                {ev.day} {#if ev.time}<span style="color: {stageSettings.stage_schedule_time_color || '#ffffff'};">• {ev.time}</span>{/if}
              </span>
              <p
                class="font-black text-white leading-tight line-clamp-1 truncate"
                style="font-size: {Math.max(22, Math.round((stageSettings.stage_schedule_title_min_font_size ?? 28) * 1.0))}px;"
              >
                {ev.title}
              </p>
              {#if ev.details}
                <span
                  class="text-zinc-300 font-medium truncate"
                  style="font-size: {Math.max(16, Math.round((stageSettings.stage_schedule_details_font_size ?? 18) * 0.95))}px;"
                >
                  {ev.details}
                </span>
              {/if}
            </div>
          {:else if activeSlide}
            <p class="text-[28px] font-extrabold leading-tight line-clamp-2 my-auto" style="color: {isScripture ? (stageSettings.scripture_text_color || stageSettings.current_text_color || '#ffffff') : (stageSettings.current_text_color || '#ffffff')};">
              {activeSlide.content.split('\n')[0]}
            </p>
          {:else}
            <span class="text-zinc-400 text-lg italic my-auto">Aguardando slide</span>
          {/if}
        </div>

        <!-- Linha 2: Próximo Slide (Ocultado quando for imagem em tela cheia) -->
        {#if !isCurrentMediaImage}
          <div
            class="relative border-b p-4 pt-9 overflow-hidden bg-zinc-900/40 flex flex-col justify-center min-h-0"
            style="flex: {nextRatio}; border-color: {stageSettings.next_border_color || '#eab308'};"
          >
            <span class="absolute top-2.5 left-3.5 text-sm font-bold uppercase truncate flex items-center gap-1.5 opacity-85" style="color: {stageSettings.next_text_color || '#facc15'};">
              <ArrowRight size={16} />
              {isScripture && nextSlide ? `${nextSlide.label}` : 'A Seguir:'}
            </span>
            {#if isNextMediaImage && nextSlide}
              <div class="w-full h-24 flex items-center justify-center my-auto opacity-85">
                <img src={nextSlide.content} alt="Próximo" class="max-w-full max-h-full object-contain rounded" />
              </div>
            {:else if isSchedule && nextSlide}
              {@const nextEv = parseScheduleContent(nextSlide.label, nextSlide.content)}
              <div class="flex flex-col items-center justify-center my-auto px-3 text-center opacity-90 gap-1">
                <span
                  class="text-yellow-300 font-bold uppercase truncate"
                  style="font-size: {Math.max(15, Math.round((stageSettings.stage_schedule_day_font_size ?? 18) * 0.95))}px;"
                >
                  {nextEv.day} {nextEv.time ? `• ${nextEv.time}` : ''}
                </span>
                <p
                  class="font-extrabold text-white leading-tight line-clamp-1 truncate"
                  style="font-size: {Math.max(18, Math.round((stageSettings.stage_schedule_next_min_font_size ?? 18) * 1.0))}px;"
                >
                  {nextEv.title}
                </p>
              </div>
            {:else if nextSlide}
              <p class="text-2xl font-extrabold leading-tight line-clamp-1 my-auto opacity-95" style="color: {stageSettings.next_text_color || '#facc15'};">
                {nextSlide.content.split('\n')[0]}
              </p>
            {:else}
              <span class="text-zinc-400 text-lg italic my-auto">Fim</span>
            {/if}
          </div>
        {/if}

        <!-- Linha 3: Barra Inferior Horizontal (Aviso, Hora, Timer) -->
        <div
          class="bg-zinc-900/50 p-0 flex flex-row gap-0 shrink-0 font-mono text-center overflow-hidden min-h-0"
          style="flex: {footerRatio};"
        >
          <!-- Aviso de Palco -->
          <div
            class="p-1.5 border-r border-zinc-800 flex flex-col justify-center overflow-hidden {timer.stage_message ? 'bg-emerald-500/20 text-emerald-200' : 'bg-zinc-950/60 text-zinc-400'}"
            style="flex: 40; min-height: 0; min-width: 0;"
          >
            <span class="text-xs uppercase font-bold font-sans flex items-center justify-center gap-1 truncate {timer.stage_message ? 'text-emerald-300' : 'text-zinc-400'}">
              <MessageSquare size={14} />
              Aviso
            </span>
            {#if timer.stage_message}
              <p class="font-black leading-tight text-white line-clamp-1 my-auto" style="font-size: {previewAlertSize}px;">
                {timer.stage_message}
              </p>
            {:else}
              <span class="text-sm text-zinc-400 italic my-auto">—</span>
            {/if}
          </div>

          <!-- Horário -->
          <div
            class="bg-zinc-900/80 p-1.5 border-r border-zinc-800 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0; min-width: 0;"
          >
            <span class="text-xs text-zinc-400 uppercase block font-sans">Hora</span>
            <span class="font-bold block my-auto truncate" style="color: {stageSettings.clock_color || '#ffffff'}; font-size: {previewClockSize}px;">
              {displayClock}
            </span>
          </div>

          <!-- Timer -->
          <div
            class="bg-zinc-900/80 p-1.5 flex flex-col justify-center overflow-hidden"
            style="flex: 30; min-height: 0; min-width: 0;"
          >
            <span class="text-xs text-zinc-400 uppercase block font-sans truncate">{timer?.label || 'Timer'}</span>
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
  </div>
</div>

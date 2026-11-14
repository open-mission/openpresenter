<script lang="ts">
  import { onDestroy } from 'svelte';
  import {
    Play,
    Pause,
    RotateCcw,
    Square,
    Timer as TimerIcon,
    Send,
    X,
    ChevronDown,
    MessageSquare,
    SlidersHorizontal,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import {
    presentationState,
    startTimer,
    pauseTimer,
    resetTimer,
    setTimer,
    setStageMessage,
  } from '../ws';

  let stageMessageInput = $state('');
  let stageMessageInputEl = $state<HTMLInputElement | null>(null);
  let showTimerConfig = $state(false);

  let isTimerCollapsed = $state(
    typeof window !== 'undefined' && localStorage.getItem('timer_section_collapsed') === 'true'
  );
  function toggleTimerCollapse() {
    isTimerCollapsed = !isTimerCollapsed;
    if (typeof window !== 'undefined') {
      localStorage.setItem('timer_section_collapsed', String(isTimerCollapsed));
    }
  }

  let isMessageCollapsed = $state(
    typeof window !== 'undefined' && localStorage.getItem('stage_message_collapsed') === 'true'
  );
  function toggleMessageCollapse() {
    isMessageCollapsed = !isMessageCollapsed;
    if (typeof window !== 'undefined') {
      localStorage.setItem('stage_message_collapsed', String(isMessageCollapsed));
    }
  }

  const timer = $derived($presentationState.timer);
  let timerTitleInput = $state('');

  // Sincroniza com timer.label quando existir
  $effect(() => {
    if (timer?.label && timerTitleInput === '') {
      timerTitleInput = timer.label;
    }
  });

  function handleUpdateTitle(customText?: string) {
    const textToApply = customText !== undefined ? customText : timerTitleInput;
    const title = textToApply.trim() || undefined;
    setTimer(timer.duration_seconds, title);
  }

  let titleDebounceTimeout: ReturnType<typeof setTimeout> | undefined;
  function handleTitleInput() {
    clearTimeout(titleDebounceTimeout);
    titleDebounceTimeout = setTimeout(() => {
      handleUpdateTitle();
    }, 350);
  }

  onDestroy(() => {
    clearTimeout(titleDebounceTimeout);
  });

  // Formata os segundos em MM:SS ou -MM:SS
  function formatSeconds(sec: number): { text: string; isNegative: boolean } {
    const isNegative = sec < 0;
    const abs = Math.abs(sec);
    const m = Math.floor(abs / 60);
    const s = abs % 60;
    const strM = String(m).padStart(2, '0');
    const strS = String(s).padStart(2, '0');
    return {
      text: `${isNegative ? '-' : ''}${strM}:${strS}`,
      isNegative,
    };
  }

  const formattedRemaining = $derived(formatSeconds(timer.remaining_seconds));

  // Estados visuais
  const isOvertime = $derived(timer.remaining_seconds <= 0 && timer.running);
  const isWarning = $derived(
    timer.remaining_seconds > 0 &&
    timer.remaining_seconds <= (timer.warning_threshold_seconds || 60) &&
    timer.running
  );

  function handleSetPreset(mins: number) {
    const currentTitle = timerTitleInput.trim() || `${mins} min`;
    setTimer(mins * 60, currentTitle);
  }

  function handleAddMinutes(mins: number) {
    const newRemaining = timer.remaining_seconds + mins * 60;
    const newDuration = Math.max(timer.duration_seconds, newRemaining);
    const currentTitle = timerTitleInput.trim() || timer.label;
    setTimer(newDuration, currentTitle);
    // Se o timer estiver rodando, reinicia com o novo tempo
    if (timer.running) {
      startTimer();
    }
  }

  function handleSendMessage() {
    if (!stageMessageInput.trim()) return;
    setStageMessage(stageMessageInput.trim());
    stageMessageInput = '';
  }

  function handleClearMessage() {
    setStageMessage(undefined);
  }
</script>

<!-- SEÇÃO 1: CRONÔMETRO -->
<div class="p-3 border-b border-border {isTimerCollapsed ? 'py-2' : 'space-y-2.5'} bg-card/30 shrink-0 transition-all">
  <!-- Cabeçalho do Cronômetro com Toggle de Recolher -->
  <div class="flex items-center justify-between gap-2">
    <button
      type="button"
      onclick={toggleTimerCollapse}
      aria-expanded={!isTimerCollapsed}
      class="min-h-7 px-1 -ml-1 flex items-center gap-1.5 text-xs font-bold text-foreground hover:text-primary transition-colors cursor-pointer select-none group rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title={isTimerCollapsed ? 'Expandir painel do cronômetro' : 'Recolher painel do cronômetro'}
    >
      <TimerIcon size={14} class="text-emerald-700 dark:text-emerald-400 group-hover:text-primary transition-colors" />
      <span>Cronômetro</span>
      <ChevronDown size={13} class="text-muted-foreground group-hover:text-foreground transition-transform duration-200 {isTimerCollapsed ? '-rotate-90' : ''}" />
    </button>

    <div class="flex items-center gap-1.5">
      {#if isTimerCollapsed}
        <span
          class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-[11px] font-mono font-bold {isOvertime
            ? 'bg-red-500/15 text-red-700 dark:text-red-300'
            : timer.running
              ? 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300'
              : 'bg-muted text-foreground'}"
        >
          <TimerIcon size={11} />
          <span>{formattedRemaining.text}</span>
        </span>
      {:else}
        <button
          type="button"
          onclick={() => (showTimerConfig = !showTimerConfig)}
          aria-expanded={showTimerConfig}
          aria-label="Alternar presets de duração e título do cronômetro"
          class="min-h-6 px-2 rounded border border-border text-[11px] font-medium flex items-center gap-1 transition-colors cursor-pointer {showTimerConfig ? 'bg-primary text-primary-foreground border-primary' : 'bg-muted/70 text-muted-foreground hover:text-foreground hover:bg-muted'}"
          title="Configurar duração e título no palco"
        >
          <SlidersHorizontal size={11} />
          <span>Presets</span>
        </button>
      {/if}
    </div>
  </div>

  {#if !isTimerCollapsed}
    <div class="space-y-2 animate-in fade-in duration-150">
      <!-- Display Digital do Timer -->
      <div
        class="w-full py-2 px-3 rounded-lg border flex items-center justify-between gap-2 transition-colors {isOvertime
          ? 'border-red-600 dark:border-red-500 bg-red-500/15 text-red-700 dark:text-red-300 shadow-inner ring-1 ring-red-500/40'
          : isWarning
            ? 'border-amber-600 dark:border-amber-500 bg-amber-500/15 text-amber-800 dark:text-amber-300 ring-1 ring-amber-500/40'
            : 'border-border bg-muted/40 text-foreground'}"
      >
        <div class="min-w-0">
          <div class="flex items-center gap-1.5 flex-wrap">
            <span class="text-[11px] uppercase font-bold tracking-wider text-muted-foreground truncate max-w-[140px]">
              {timer.label || 'Contagem'}
            </span>
            {#if isOvertime}
              <span class="text-[11px] font-bold px-1.5 py-0.5 rounded bg-red-700 text-white">
                TEMPO ESGOTADO
              </span>
            {:else if isWarning}
              <span class="text-[11px] font-bold px-1.5 py-0.5 rounded bg-amber-500 text-black">
                ÚLTIMO MINUTO
              </span>
            {/if}
          </div>

          <!-- Mostrador Digital -->
          <div
            class="text-2xl sm:text-3xl font-mono font-black tracking-tight leading-none mt-1 {isOvertime
              ? 'text-red-700 dark:text-red-400'
              : isWarning
                ? 'text-amber-800 dark:text-amber-300'
                : timer.running
                  ? 'text-foreground'
                  : 'text-muted-foreground'}"
          >
            {formattedRemaining.text}
          </div>
        </div>

        <!-- Botões de Ação do Timer -->
        <div class="flex items-center gap-1 shrink-0">
          {#if timer.running}
            <button
              type="button"
              onclick={pauseTimer}
              aria-label="Pausar cronômetro"
              class="w-8 h-8 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 text-amber-800 dark:text-amber-300 border border-amber-600/40 dark:border-amber-500/40 flex items-center justify-center transition-colors cursor-pointer"
              title="Pausar Timer"
            >
              <Pause size={14} class="fill-current" />
            </button>
          {:else}
            <button
              type="button"
              onclick={startTimer}
              aria-label="Iniciar cronômetro"
              class="w-8 h-8 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white font-bold flex items-center justify-center transition-colors cursor-pointer shadow-sm"
              title="Iniciar Timer"
            >
              <Play size={14} class="fill-current ml-0.5" />
            </button>
          {/if}

          <button
            type="button"
            onclick={resetTimer}
            aria-label="Reiniciar cronômetro para a duração original"
            class="w-8 h-8 rounded-lg bg-muted hover:bg-muted/80 border border-border text-foreground flex items-center justify-center transition-colors cursor-pointer"
            title="Reiniciar para o tempo original"
          >
            <RotateCcw size={13} />
          </button>

          <button
            type="button"
            onclick={() => setTimer(0, timerTitleInput.trim() || timer.label || undefined)}
            aria-label="Zerar cronômetro"
            class="w-8 h-8 rounded-lg bg-muted hover:bg-muted/80 border border-border text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer"
            title="Zerar Timer"
          >
            <Square size={12} />
          </button>
        </div>
      </div>

      <!-- Ajustes Rápidos Sempre Visíveis (+1m/+5m + Presets Principais) -->
      <div class="flex items-center gap-1 flex-wrap">
        {#each [5, 10, 15, 30] as mins}
          <button
            type="button"
            onclick={() => handleSetPreset(mins)}
            class="min-h-6 px-2 py-0.5 rounded text-[11px] font-mono font-semibold bg-muted hover:bg-muted/80 border border-border text-foreground transition-colors cursor-pointer"
          >
            {mins}m
          </button>
        {/each}

        <button
          type="button"
          onclick={() => handleAddMinutes(1)}
          class="min-h-6 px-2 py-0.5 rounded text-[11px] font-mono font-bold bg-primary/15 hover:bg-primary/25 border border-primary/30 text-primary transition-colors cursor-pointer ml-auto"
          title="Adicionar 1 minuto"
        >
          +1m
        </button>
        <button
          type="button"
          onclick={() => handleAddMinutes(5)}
          class="min-h-6 px-2 py-0.5 rounded text-[11px] font-mono font-bold bg-primary/15 hover:bg-primary/25 border border-primary/30 text-primary transition-colors cursor-pointer"
          title="Adicionar 5 minutos"
        >
          +5m
        </button>
      </div>

      <!-- Título e Presets Estendidos (Revelação Progressiva) -->
      {#if showTimerConfig}
        <div class="space-y-1.5 pt-1 border-t border-border/60 animate-in fade-in duration-150">
          <div class="flex items-center justify-between text-[11px] text-muted-foreground font-medium">
            <label for="timer-title-input">Título no Palco (Stage)</label>
            {#if timerTitleInput}
              <button
                type="button"
                onclick={() => {
                  timerTitleInput = '';
                  handleUpdateTitle('');
                }}
                class="min-h-6 px-1.5 text-muted-foreground hover:text-foreground text-[11px] cursor-pointer"
              >
                Limpar
              </button>
            {/if}
          </div>
          <input
            id="timer-title-input"
            type="text"
            bind:value={timerTitleInput}
            oninput={handleTitleInput}
            placeholder="Ex.: Pregação, Oração, Louvor..."
            class="w-full h-8 px-2.5 bg-background border border-border rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring transition-colors"
          />
          <div class="flex items-center gap-1 flex-wrap pt-0.5">
            {#each ['Pregação', 'Louvor', 'Oração', 'Avisos', 'Palavra', 'Ceia'] as quickTitle}
              <button
                type="button"
                onclick={() => {
                  timerTitleInput = quickTitle;
                  handleUpdateTitle(quickTitle);
                }}
                class="min-h-6 px-2 py-0.5 rounded text-[11px] font-medium border transition-colors cursor-pointer {timerTitleInput === quickTitle
                  ? 'border-primary text-primary bg-primary/15 font-bold'
                  : 'border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted'}"
              >
                {quickTitle}
              </button>
            {/each}
            <button
              type="button"
              onclick={() => handleSetPreset(45)}
              class="min-h-6 px-2 py-0.5 rounded text-[11px] font-mono font-semibold bg-muted hover:bg-muted/80 border border-border text-foreground transition-colors cursor-pointer ml-auto"
            >
              45m
            </button>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- SEÇÃO 2: MENSAGEM NO RETORNO (STAGE) -->
<div class="p-3 border-b border-border {isMessageCollapsed ? 'py-2' : 'space-y-2'} bg-card/30 shrink-0 transition-all">
  <div class="flex items-center justify-between gap-2">
    <button
      type="button"
      onclick={toggleMessageCollapse}
      aria-expanded={!isMessageCollapsed}
      class="min-h-7 px-1 -ml-1 flex items-center gap-1.5 text-xs font-bold text-foreground hover:text-primary transition-colors cursor-pointer select-none group rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title={isMessageCollapsed ? 'Expandir mensagem para o palco' : 'Recolher mensagem para o palco'}
    >
      <MessageSquare size={14} class="text-emerald-700 dark:text-emerald-400 group-hover:text-primary transition-colors" />
      <span>Mensagem no Retorno</span>
      <ChevronDown size={13} class="text-muted-foreground group-hover:text-foreground transition-transform duration-200 {isMessageCollapsed ? '-rotate-90' : ''}" />
    </button>

    <div class="flex items-center gap-1.5">
      {#if timer.stage_message}
        <Badge variant="success" class="text-[11px] py-0.5 px-2">
          Ativo
        </Badge>
        <button
          type="button"
          onclick={handleClearMessage}
          aria-label="Limpar mensagem ativa do palco"
          class="min-h-6 px-2 rounded text-[11px] font-medium text-red-700 dark:text-red-400 hover:bg-red-500/15 flex items-center gap-1 cursor-pointer transition-colors"
          title="Limpar mensagem ativa do palco"
        >
          <X size={12} />
          <span>Limpar</span>
        </button>
      {/if}
    </div>
  </div>

  {#if !isMessageCollapsed}
    <div class="space-y-2 animate-in fade-in duration-150">
      {#if timer.stage_message}
        <div class="p-2 rounded-lg bg-emerald-500/15 border border-emerald-600/35 dark:border-emerald-500/40 flex items-center justify-between gap-2 text-xs text-emerald-800 dark:text-emerald-200 shadow-sm">
          <span class="truncate font-semibold">{timer.stage_message}</span>
        </div>
      {/if}

      <form
        onsubmit={(e) => {
          e.preventDefault();
          handleSendMessage();
        }}
        class="flex items-center gap-1.5"
      >
        <label for="stage-message-input" class="sr-only">Mensagem para o monitor de palco</label>
        <input
          id="stage-message-input"
          bind:this={stageMessageInputEl}
          type="text"
          bind:value={stageMessageInput}
          placeholder="Ex.: 5 min restantes, Próximo hino..."
          class="flex-1 h-8 px-2.5 bg-background border border-border rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring transition-colors"
        />
        <Button
          variant="secondary"
          size="sm"
          type="submit"
          disabled={!stageMessageInput.trim()}
          aria-label="Enviar mensagem para o monitor de palco"
          class="h-8 text-xs px-2.5 shrink-0"
          title="Enviar mensagem para o monitor de palco"
        >
          <Send size={13} />
        </Button>
      </form>

      <!-- Chips de Pré-preenchimento Rápido (evita envio acidental em 1 clique) -->
      <div class="flex items-center gap-1 flex-wrap pt-0.5">
        {#each ['5 min restantes', 'Atenção ao tempo', 'Próximo louvor', 'Encerrar'] as quickMsg}
          <button
            type="button"
            onclick={() => {
              stageMessageInput = quickMsg;
              stageMessageInputEl?.focus();
            }}
            class="min-h-6 px-2 py-0.5 rounded text-[11px] font-medium border border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
            title="Preencher mensagem no campo acima (pressione Enter para enviar)"
          >
            {quickMsg}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

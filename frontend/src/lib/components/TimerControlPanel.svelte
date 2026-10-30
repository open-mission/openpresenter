<script lang="ts">
  import {
    Play,
    Pause,
    RotateCcw,
    Square,
    Clock,
    Timer as TimerIcon,
    Send,
    X,
    AlertCircle,
    Plus,
    ChevronDown,
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

  let customMinutes = $state(10);
  let stageMessageInput = $state('');
  let clockTime = $state('');
  let isCollapsed = $state(false);

  // Atualiza relógio local a cada segundo
  $effect(() => {
    const updateClock = () => {
      const now = new Date();
      clockTime = now.toLocaleTimeString('pt-BR', { hour12: false });
    };
    updateClock();
    const interval = setInterval(updateClock, 1000);
    return () => clearInterval(interval);
  });

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

  let titleDebounceTimeout: any = null;
  function handleTitleInput() {
    clearTimeout(titleDebounceTimeout);
    titleDebounceTimeout = setTimeout(() => {
      handleUpdateTitle();
    }, 350);
  }

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

<div class="p-3 border-t border-neutral-800/80 bg-neutral-900/60 space-y-2.5 shrink-0 transition-all">
  <!-- Cabeçalho do Painel com Toggle de Recolher -->
  <div class="flex items-center justify-between">
    <button
      type="button"
      onclick={() => (isCollapsed = !isCollapsed)}
      class="flex items-center gap-1.5 text-left hover:text-white transition-colors cursor-pointer group select-none"
      title={isCollapsed ? 'Expandir painel do cronômetro' : 'Recolher painel do cronômetro'}
    >
      <TimerIcon size={14} class="text-primary" />
      <span class="text-xs font-bold text-neutral-200 group-hover:text-white">Cronômetro & Palco</span>
      <ChevronDown size={13} class="text-neutral-500 group-hover:text-neutral-300 transition-transform duration-200 {isCollapsed ? '-rotate-90' : ''}" />
    </button>

    <div class="flex items-center gap-1.5">
      <!-- Indicador compacto quando recolhido -->
      {#if isCollapsed}
        <div class="flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-mono font-bold {isOvertime ? 'bg-red-500/20 text-red-400' : timer.running ? 'bg-emerald-500/20 text-emerald-400' : 'bg-neutral-800 text-neutral-300'}">
          <TimerIcon size={10} />
          <span>{formattedRemaining.text}</span>
        </div>
      {/if}

      <!-- Relógio em Destaque -->
      <div class="flex items-center gap-1 text-[11px] font-mono text-neutral-400 bg-neutral-950 px-2 py-0.5 rounded border border-neutral-800">
        <Clock size={11} class="text-primary" />
        <span class="font-bold text-neutral-200">{clockTime || '00:00:00'}</span>
      </div>
    </div>
  </div>

  {#if !isCollapsed}

  <!-- Display Digital do Timer -->
  <div
    class="w-full py-2.5 px-3 rounded-lg border flex items-center justify-between transition-colors {isOvertime
      ? 'border-red-500 bg-red-950/40 text-red-400 shadow-inner ring-1 ring-red-500/50'
      : isWarning
        ? 'border-amber-500 bg-amber-950/30 text-amber-300 ring-1 ring-amber-500/40'
        : 'border-neutral-800 bg-black/60 text-white'}"
  >
    <div>
      <div class="flex items-center gap-1.5">
        <span class="text-[9px] uppercase font-bold tracking-wider opacity-75">
          {timer.label || 'Contagem'}
        </span>
        {#if isOvertime}
          <span class="text-[9px] font-bold px-1 py-0.2 rounded bg-red-500 text-white animate-pulse">
            TEMPO ESGOTADO
          </span>
        {:else if isWarning}
          <span class="text-[9px] font-bold px-1 py-0.2 rounded bg-amber-500 text-black">
            ÚLTIMO MINUTO
          </span>
        {/if}
      </div>

      <!-- Mostrador Digital de Números Gigantes -->
      <div
        class="text-2xl sm:text-3xl font-mono font-black tracking-tight leading-none mt-1 {isOvertime
          ? 'animate-pulse text-red-400'
          : isWarning
            ? 'text-amber-300'
            : timer.running
              ? 'text-white'
              : 'text-neutral-400'}"
      >
        {formattedRemaining.text}
      </div>
    </div>

    <!-- Botões de Ação do Timer -->
    <div class="flex items-center gap-1">
      {#if timer.running}
        <button
          type="button"
          onclick={pauseTimer}
          class="w-8 h-8 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 text-amber-400 border border-amber-500/40 flex items-center justify-center transition-colors cursor-pointer"
          title="Pausar Timer"
        >
          <Pause size={14} class="fill-current" />
        </button>
      {:else}
        <button
          type="button"
          onclick={startTimer}
          class="w-8 h-8 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-black font-bold flex items-center justify-center transition-colors cursor-pointer shadow-sm"
          title="Iniciar Timer"
        >
          <Play size={14} class="fill-current ml-0.5" />
        </button>
      {/if}

      <button
        type="button"
        onclick={resetTimer}
        class="w-8 h-8 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-300 flex items-center justify-center transition-colors cursor-pointer"
        title="Reiniciar para o tempo original"
      >
        <RotateCcw size={13} />
      </button>

      <button
        type="button"
        onclick={() => setTimer(0, 'Zerado')}
        class="w-8 h-8 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
        title="Zerar Timer"
      >
        <Square size={12} />
      </button>
    </div>
  </div>

  <!-- Título Configurável do Cronômetro (Exibido no Monitor de Palco / Stage) -->
  <div class="space-y-1">
    <div class="flex items-center justify-between text-[10px] text-neutral-400 font-medium">
      <span>Título no Palco (Stage)</span>
      {#if timerTitleInput}
        <button
          type="button"
          onclick={() => {
            timerTitleInput = '';
            handleUpdateTitle('');
          }}
          class="text-neutral-500 hover:text-neutral-300 text-[10px] cursor-pointer"
        >
          Limpar
        </button>
      {/if}
    </div>
    <input
      type="text"
      bind:value={timerTitleInput}
      oninput={handleTitleInput}
      placeholder="Ex.: Pregação, Oração, Louvor, Avisos..."
      class="w-full h-7 px-2.5 bg-neutral-950 border border-neutral-800 rounded-md text-[11px] text-white placeholder-neutral-500 focus:outline-none focus:border-primary/60 transition-colors"
    />
    <!-- Chips de Sugestão Rápida de Título -->
    <div class="flex items-center gap-1 flex-wrap pt-0.5">
      {#each ['Pregação', 'Louvor', 'Oração', 'Avisos', 'Palavra', 'Ceia'] as quickTitle}
        <button
          type="button"
          onclick={() => {
            timerTitleInput = quickTitle;
            handleUpdateTitle(quickTitle);
          }}
          class="px-1.5 py-0.5 rounded text-[9px] font-medium border border-neutral-800/80 bg-neutral-950/60 text-neutral-400 hover:text-white hover:border-neutral-700 transition-colors cursor-pointer {timerTitleInput === quickTitle ? 'border-primary/60 text-primary bg-primary/10 font-bold' : ''}"
        >
          {quickTitle}
        </button>
      {/each}
    </div>
  </div>

  <!-- Presets Rápidos de Minutos -->
  <div class="flex items-center gap-1 flex-wrap">
    <button
      type="button"
      onclick={() => handleSetPreset(5)}
      class="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-neutral-800/80 hover:bg-neutral-700 text-neutral-300 transition-colors cursor-pointer"
    >
      5m
    </button>
    <button
      type="button"
      onclick={() => handleSetPreset(10)}
      class="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-neutral-800/80 hover:bg-neutral-700 text-neutral-300 transition-colors cursor-pointer"
    >
      10m
    </button>
    <button
      type="button"
      onclick={() => handleSetPreset(15)}
      class="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-neutral-800/80 hover:bg-neutral-700 text-neutral-300 transition-colors cursor-pointer"
    >
      15m
    </button>
    <button
      type="button"
      onclick={() => handleSetPreset(30)}
      class="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-neutral-800/80 hover:bg-neutral-700 text-neutral-300 transition-colors cursor-pointer"
    >
      30m
    </button>
    <button
      type="button"
      onclick={() => handleSetPreset(45)}
      class="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-neutral-800/80 hover:bg-neutral-700 text-neutral-300 transition-colors cursor-pointer"
    >
      45m
    </button>

    <button
      type="button"
      onclick={() => handleAddMinutes(1)}
      class="px-1.5 py-0.5 rounded text-[10px] font-mono font-bold bg-primary/20 hover:bg-primary/30 text-primary transition-colors cursor-pointer ml-auto"
      title="Adicionar 1 minuto"
    >
      +1m
    </button>
    <button
      type="button"
      onclick={() => handleAddMinutes(5)}
      class="px-1.5 py-0.5 rounded text-[10px] font-mono font-bold bg-primary/20 hover:bg-primary/30 text-primary transition-colors cursor-pointer"
      title="Adicionar 5 minutos"
    >
      +5m
    </button>
  </div>

  <!-- Mensagem Instantânea de Palco (Stage Message) -->
  <div class="space-y-1 pt-1.5 border-t border-neutral-800/60">
    <div class="flex items-center justify-between text-[10px] text-neutral-400 font-medium">
      <span>Mensagem no Retorno (Stage)</span>
      {#if timer.stage_message}
        <button
          type="button"
          onclick={handleClearMessage}
          class="text-red-400 hover:text-red-300 flex items-center gap-0.5 cursor-pointer"
        >
          <X size={10} /> Limpar
        </button>
      {/if}
    </div>

    {#if timer.stage_message}
      <div class="p-1.5 rounded bg-emerald-950/40 border border-emerald-500/40 flex items-center justify-between text-xs text-emerald-300">
        <span class="truncate font-medium">{timer.stage_message}</span>
      </div>
    {/if}

    <form
      onsubmit={(e) => {
        e.preventDefault();
        handleSendMessage();
      }}
      class="flex items-center gap-1"
    >
      <input
        type="text"
        bind:value={stageMessageInput}
        placeholder="Ex.: 5 min restantes, Próximo hino..."
        class="flex-1 h-7 px-2 bg-neutral-950 border border-neutral-800 rounded text-[11px] text-white placeholder-neutral-500 focus:outline-none focus:border-primary/60"
      />
      <Button
        variant="secondary"
        size="sm"
        type="submit"
        disabled={!stageMessageInput.trim()}
        class="h-7 text-xs px-2 shrink-0"
      >
        <Send size={11} />
      </Button>
    </form>
  </div>
  {/if}
</div>

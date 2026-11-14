<script lang="ts">
  import {
    AlertTriangle,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Clock,
    Shield,
  } from 'lucide-svelte';
  import type { Slide, TimerState } from '$lib/types';
  import { formatMediaTime } from '$lib/remoteData';

  interface Props {
    canControlRemote: boolean;
    isLive: boolean;
    isBlackout: boolean;
    isClear: boolean;
    currentMediaTitle: string;
    currentMediaRef?: string;
    currentSlide?: Slide | null;
    nextSlide?: Slide | null;
    currentSlideIndex: number;
    totalSlides: number;
    timer: TimerState;
    isCollapsed: boolean;
    safeDispatch: boolean;
    onToggleCollapse: () => void;
    onToggleSafeDispatch: () => void;
    onPrevSlide: () => void;
    onNextSlide: () => void;
  }

  let {
    canControlRemote,
    isLive,
    isBlackout,
    isClear,
    currentMediaTitle,
    currentMediaRef = '',
    currentSlide = null,
    nextSlide = null,
    currentSlideIndex,
    totalSlides,
    timer,
    isCollapsed,
    safeDispatch,
    onToggleCollapse,
    onToggleSafeDispatch,
    onPrevSlide,
    onNextSlide,
  }: Props = $props();

  const hasTimerActivity = $derived(
    timer.running || timer.remaining_seconds !== timer.duration_seconds
  );

  const isTimerUrgent = $derived(
    Boolean(timer.is_overtime) || timer.remaining_seconds <= timer.warning_threshold_seconds
  );
</script>

{#if canControlRemote}
  <section
    aria-label="Controle rápido da projeção ao vivo"
    class="rounded-2xl bg-card border border-border overflow-hidden shadow-xs"
  >
    <div class="px-3.5 py-2.5 bg-muted/45 flex items-center justify-between gap-2 border-b border-border/70">
      <div class="flex items-center gap-2 min-w-0">
        {#if isBlackout}
          <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-amber-500/15 text-amber-800 dark:text-amber-300 border border-amber-600/35 dark:border-amber-500/40 shrink-0">
            BLACKOUT
          </span>
        {:else if isClear}
          <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-sky-500/15 text-sky-800 dark:text-sky-300 border border-sky-600/35 dark:border-sky-500/40 shrink-0">
            SEM TEXTO
          </span>
        {:else if isLive}
          <span class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[11px] font-bold bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border border-emerald-600/35 dark:border-emerald-500/40 shrink-0">
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-600 dark:bg-emerald-400" aria-hidden="true"></span>
            AO VIVO
          </span>
        {:else}
          <span class="px-2 py-0.5 rounded-full text-[11px] font-semibold bg-muted text-muted-foreground border border-border shrink-0">
            LIVRE
          </span>
        {/if}

        <span class="text-xs font-semibold text-foreground truncate">
          {currentMediaTitle}
          {#if currentMediaRef}
            <span class="text-muted-foreground font-normal">• {currentMediaRef}</span>
          {/if}
        </span>
      </div>

      <div class="flex items-center gap-1.5 shrink-0">
        {#if hasTimerActivity}
          <span
            class="px-2 py-0.5 rounded-md text-[11px] font-mono tabular-nums font-bold inline-flex items-center gap-1 border {isTimerUrgent
              ? 'bg-red-500/15 text-red-700 dark:text-red-300 border-red-600/35 dark:border-red-500/40'
              : 'bg-secondary text-foreground border-border'}"
            title="Cronômetro de palco"
          >
            <Clock size={12} aria-hidden="true" />
            {timer.is_overtime ? '-' : ''}{formatMediaTime(Math.abs(timer.remaining_seconds))}
          </span>
        {/if}

        {#if totalSlides > 0}
          <span class="text-[11px] font-mono tabular-nums font-bold px-2 py-0.5 rounded-md bg-secondary text-secondary-foreground border border-border">
            {currentSlideIndex + 1}/{totalSlides}
          </span>
        {/if}

        <button
          type="button"
          onclick={onToggleSafeDispatch}
          aria-pressed={safeDispatch}
          title={safeDispatch
            ? 'Disparo protegido ativo: tocar pré-seleciona antes de enviar ao vivo'
            : 'Disparo direto ativo: 1 toque envia imediatamente ao vivo'}
          class="min-h-8 px-2 rounded-lg text-[11px] font-semibold border inline-flex items-center gap-1 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {safeDispatch
            ? 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border-emerald-600/35 dark:border-emerald-500/40'
            : 'bg-amber-500/15 text-amber-800 dark:text-amber-300 border-amber-600/35 dark:border-amber-500/40'}"
        >
          <Shield size={12} aria-hidden="true" />
          <span class="hidden sm:inline">{safeDispatch ? '2 Toques' : '1 Toque'}</span>
        </button>

        <button
          type="button"
          onclick={onToggleCollapse}
          aria-expanded={!isCollapsed}
          aria-label={isCollapsed ? 'Expandir card de controle ao vivo' : 'Recolher card de controle ao vivo'}
          class="h-8 w-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          {#if isCollapsed}
            <ChevronDown size={16} aria-hidden="true" />
          {:else}
            <ChevronUp size={16} aria-hidden="true" />
          {/if}
        </button>
      </div>
    </div>

    {#if isCollapsed}
      <div class="p-2.5 flex items-center gap-2">
        <div class="flex-1 min-w-0 px-2">
          {#if isBlackout}
            <p class="text-xs font-semibold text-amber-800 dark:text-amber-300 truncate">
              Tela pública em Blackout (oculta para a igreja)
            </p>
          {:else if isClear}
            <p class="text-xs font-semibold text-sky-800 dark:text-sky-300 truncate">
              Texto ocultado na tela pública (fundo visível)
            </p>
          {:else if currentSlide}
            <p class="text-xs text-foreground truncate font-medium">
              {currentSlide.content.replace(/\n/g, ' ')}
            </p>
          {:else}
            <p class="text-xs text-muted-foreground truncate">
              Nenhuma mídia projetada no momento
            </p>
          {/if}
        </div>
        <button
          type="button"
          onclick={onPrevSlide}
          disabled={!isLive && currentSlideIndex <= 0}
          aria-label="Slide anterior"
          class="h-10 w-10 rounded-xl bg-secondary hover:bg-muted disabled:opacity-40 text-foreground flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer shrink-0"
        >
          <ChevronLeft size={18} aria-hidden="true" />
        </button>
        <button
          type="button"
          onclick={onNextSlide}
          disabled={totalSlides > 0 && currentSlideIndex >= totalSlides - 1}
          aria-label="Próximo slide"
          class="h-10 w-10 rounded-xl bg-emerald-700 hover:bg-emerald-800 disabled:opacity-40 text-white flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer shrink-0"
        >
          <ChevronRight size={18} aria-hidden="true" />
        </button>
      </div>
    {:else}
      <div class="p-3.5 space-y-3">
        <!-- Alerta explícito quando Blackout / Limpar está sobrepondo o slide -->
        {#if isBlackout}
          <div
            role="status"
            class="px-3 py-2 rounded-xl bg-amber-500/15 border border-amber-600/35 dark:border-amber-500/40 text-amber-800 dark:text-amber-300 text-xs font-semibold flex items-center gap-2"
          >
            <AlertTriangle size={15} class="shrink-0" aria-hidden="true" />
            <span>TELA EM BLACKOUT — A tela pública está escura para a congregação.</span>
          </div>
        {:else if isClear}
          <div
            role="status"
            class="px-3 py-2 rounded-xl bg-sky-500/15 border border-sky-600/35 dark:border-sky-500/40 text-sky-800 dark:text-sky-300 text-xs font-semibold flex items-center gap-2"
          >
            <AlertTriangle size={15} class="shrink-0" aria-hidden="true" />
            <span>TEXTO OCULTADO — Apenas o papel de parede está visível no telão.</span>
          </div>
        {/if}

        <!-- Prévia do Slide Atual e Próximo -->
        <div class="p-3.5 rounded-xl bg-background/80 border border-border min-h-[76px] flex flex-col justify-center space-y-2">
          {#if currentSlide}
            <div class="flex items-start justify-between gap-2">
              <p class="text-sm leading-relaxed whitespace-pre-line line-clamp-3 font-medium {isBlackout || isClear
                ? 'text-muted-foreground line-through decoration-1'
                : 'text-foreground'}">
                {currentSlide.content}
              </p>
              {#if currentSlide.label}
                <span class="px-2 py-0.5 rounded-md text-[11px] font-semibold bg-secondary text-secondary-foreground border border-border shrink-0">
                  {currentSlide.label}
                </span>
              {/if}
            </div>
            {#if nextSlide}
              <p class="text-xs text-muted-foreground border-t border-border/60 pt-1.5 line-clamp-1">
                <strong class="font-semibold text-foreground/80">Próximo:</strong>
                {nextSlide.content.replace(/\n/g, ' ')}
              </p>
            {/if}
          {:else}
            <p class="text-xs text-muted-foreground text-center py-2">
              Escolha um louvor, setlist ou passagem bíblica para iniciar a projeção.
            </p>
          {/if}
        </div>

        <!-- Botões principais de navegação de slide -->
        <div class="grid grid-cols-2 gap-2.5">
          <button
            type="button"
            onclick={onPrevSlide}
            disabled={!isLive && currentSlideIndex <= 0}
            class="min-h-12 rounded-xl bg-secondary hover:bg-muted disabled:opacity-40 text-foreground font-bold text-sm flex items-center justify-center gap-1.5 border border-border transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
          >
            <ChevronLeft size={18} aria-hidden="true" />
            <span>Anterior</span>
          </button>

          <button
            type="button"
            onclick={onNextSlide}
            disabled={totalSlides > 0 && currentSlideIndex >= totalSlides - 1}
            class="min-h-12 rounded-xl bg-emerald-700 hover:bg-emerald-800 disabled:opacity-40 text-white font-bold text-sm flex items-center justify-center gap-1.5 shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
          >
            <span>Próximo</span>
            <ChevronRight size={18} aria-hidden="true" />
          </button>
        </div>
      </div>
    {/if}
  </section>
{:else if isLive || isBlackout || isClear || hasTimerActivity}
  <!-- Monitor passivo para voluntários acompanharem o culto sem botões de disparo -->
  <section
    aria-label="Monitor de status ao vivo do culto"
    class="rounded-2xl bg-card border border-border px-3.5 py-2.5 flex items-center justify-between gap-2"
  >
    <div class="flex items-center gap-2 min-w-0">
      <span class="px-2 py-0.5 rounded-full text-[11px] font-bold {isBlackout
        ? 'bg-amber-500/15 text-amber-800 dark:text-amber-300'
        : isClear
          ? 'bg-sky-500/15 text-sky-800 dark:text-sky-300'
          : 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300'}">
        {isBlackout ? 'BLACKOUT' : isClear ? 'SEM TEXTO' : 'NO AR'}
      </span>
      <span class="text-xs font-medium text-foreground truncate">{currentMediaTitle}</span>
    </div>
    {#if hasTimerActivity}
      <span class="text-[11px] font-mono tabular-nums font-bold px-2 py-0.5 rounded-md bg-secondary text-foreground border border-border shrink-0">
        {formatMediaTime(Math.abs(timer.remaining_seconds))}
      </span>
    {/if}
  </section>
{/if}

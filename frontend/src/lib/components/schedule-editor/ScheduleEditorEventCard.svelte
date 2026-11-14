<script lang="ts">
  import { ArrowDown, ArrowUp, Calendar, Clock, Trash2 } from 'lucide-svelte';
  import type { ScheduleEvent } from '$lib/types';

  interface Props {
    event: ScheduleEvent;
    index: number;
    isSelected: boolean;
    isFirst: boolean;
    isLast: boolean;
    onselect: () => void;
    onmoveup: () => void;
    onmovedown: () => void;
    onremove: () => void;
  }

  let {
    event = $bindable(),
    index,
    isSelected,
    isFirst,
    isLast,
    onselect,
    onmoveup,
    onmovedown,
    onremove,
  }: Props = $props();

  const titleId = `ev-card-title-${event.id}`;
  const inputClass =
    'w-full px-2.5 py-1.5 rounded-md bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/40 transition-colors';
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="p-3.5 rounded-xl border transition-all duration-150 {isSelected
    ? 'bg-card border-amber-600/60 ring-1 ring-amber-500/30 shadow-md'
    : 'bg-card/70 border-border hover:border-muted-foreground/40'}"
  onclick={onselect}
  onfocusin={onselect}
  role="group"
  aria-labelledby={titleId}
>
  <div class="flex items-center justify-between gap-2 mb-2 pb-2 border-b border-border/80">
    <div class="flex items-center gap-2 min-w-0">
      <span
        class="w-5 h-5 rounded-full bg-muted text-foreground text-[11px] font-mono font-bold flex items-center justify-center shrink-0"
        aria-hidden="true"
      >
        {index + 1}
      </span>
      <span id={titleId} class="text-xs font-semibold text-foreground truncate">
        {event.dayOrDate || 'Sem dia definido'}
      </span>
    </div>

    <!-- Botões de Reordenação e Exclusão -->
    <div class="flex items-center gap-1 shrink-0">
      <button
        type="button"
        disabled={isFirst}
        onclick={(e) => {
          e.stopPropagation();
          onmoveup();
        }}
        class="w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted disabled:opacity-30 disabled:hover:bg-transparent flex items-center justify-center cursor-pointer transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        title="Mover para cima"
        aria-label={`Mover evento ${index + 1} (${event.dayOrDate || 'sem dia'}) para cima`}
      >
        <ArrowUp size={13} aria-hidden="true" />
      </button>
      <button
        type="button"
        disabled={isLast}
        onclick={(e) => {
          e.stopPropagation();
          onmovedown();
        }}
        class="w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted disabled:opacity-30 disabled:hover:bg-transparent flex items-center justify-center cursor-pointer transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        title="Mover para baixo"
        aria-label={`Mover evento ${index + 1} (${event.dayOrDate || 'sem dia'}) para baixo`}
      >
        <ArrowDown size={13} aria-hidden="true" />
      </button>
      <button
        type="button"
        onclick={(e) => {
          e.stopPropagation();
          onremove();
        }}
        class="w-7 h-7 rounded-md text-muted-foreground hover:text-red-700 dark:hover:text-red-400 hover:bg-red-500/10 flex items-center justify-center transition-colors ml-0.5 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        title="Remover evento"
        aria-label={`Remover evento ${index + 1} (${event.dayOrDate || 'sem dia'})`}
      >
        <Trash2 size={13} aria-hidden="true" />
      </button>
    </div>
  </div>

  <!-- Campos de Edição do Evento -->
  <div class="grid grid-cols-1 sm:grid-cols-12 gap-2.5">
    <div class="sm:col-span-5 space-y-1">
      <label
        for={`ev-day-${event.id}`}
        class="text-[11px] font-medium text-muted-foreground flex items-center gap-1"
      >
        <Calendar size={11} class="text-amber-700 dark:text-amber-400" aria-hidden="true" />
        <span>Dia da Semana / Data</span>
      </label>
      <input
        id={`ev-day-${event.id}`}
        type="text"
        bind:value={event.dayOrDate}
        placeholder="Ex: Quarta-feira, 20 de Dezembro"
        class={inputClass}
      />
    </div>

    <div class="sm:col-span-7 space-y-1">
      <label
        for={`ev-time-${event.id}`}
        class="text-[11px] font-medium text-muted-foreground flex items-center gap-1"
      >
        <Clock size={11} class="text-amber-700 dark:text-amber-400" aria-hidden="true" />
        <span>Horário</span>
      </label>
      <input
        id={`ev-time-${event.id}`}
        type="text"
        bind:value={event.time}
        placeholder="Ex: 19h30 ou 09h00 e 18h30"
        class={inputClass}
      />
    </div>

    <div class="sm:col-span-12 space-y-1">
      <label for={`ev-title-${event.id}`} class="text-[11px] font-medium text-muted-foreground block">
        Nome do Culto / Festividade
      </label>
      <input
        id={`ev-title-${event.id}`}
        type="text"
        bind:value={event.title}
        placeholder="Ex: Culto de Oração e Doutrina"
        class="{inputClass} font-medium"
      />
    </div>

    <div class="sm:col-span-12 space-y-1">
      <label for={`ev-det-${event.id}`} class="text-[11px] font-medium text-muted-foreground block">
        Detalhes / Local / Preletores (Opcional)
      </label>
      <input
        id={`ev-det-${event.id}`}
        type="text"
        bind:value={event.details}
        placeholder="Ex: Templo Central • Com Santa Ceia"
        class={inputClass}
      />
    </div>
  </div>
</div>

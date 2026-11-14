<script lang="ts">
  import { Clock, Eye } from 'lucide-svelte';
  import Badge from '../ui/Badge.svelte';
  import type { ScheduleEvent } from '$lib/types';

  interface Props {
    events: ScheduleEvent[];
    selectedIndex: number;
    activeEvent?: ScheduleEvent;
  }

  let { events, selectedIndex, activeEvent }: Props = $props();

  const nextEvent = $derived(events[selectedIndex + 1]);
</script>

<div class="p-4 rounded-xl bg-card border border-border flex flex-col gap-3 sticky top-0 shadow-xs select-none">
  <div class="flex items-center justify-between">
    <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
      <Eye size={14} class="text-amber-700 dark:text-amber-400" aria-hidden="true" />
      <span>Prévia do Slide Selecionado</span>
    </span>
    <Badge variant="outline" class="text-[11px] font-mono text-muted-foreground">
      Slide {selectedIndex + 1} de {events.length}
    </Badge>
  </div>

  <!-- Preview 16:9 Estilizado da Tela Pública -->
  <div
    class="w-full aspect-video rounded-xl bg-muted/30 border border-border p-4 flex flex-col items-center justify-center text-center relative overflow-hidden shadow-inner"
  >
    <!-- Grid sutil de fundo para simular tela de projeção -->
    <div
      class="absolute inset-0 bg-[radial-gradient(currentColor_1px,transparent_1px)] [background-size:12px_12px] opacity-10 pointer-events-none"
      aria-hidden="true"
    ></div>

    {#if activeEvent}
      <div class="relative z-10 max-w-[90%] space-y-2 flex flex-col items-center">
        <!-- Badges superiores de Dia e Horário -->
        <div class="flex items-center gap-1.5 flex-wrap justify-center">
          <span
            class="px-2.5 py-0.5 rounded-full bg-amber-500/15 text-amber-800 dark:text-amber-300 border border-amber-600/35 text-[11px] font-bold uppercase tracking-wider"
          >
            {activeEvent.dayOrDate || 'Dia do Evento'}
          </span>
          {#if activeEvent.time}
            <span
              class="px-2 py-0.5 rounded-full bg-secondary text-secondary-foreground border border-border text-[11px] font-mono flex items-center gap-1"
            >
              <Clock size={11} class="text-amber-700 dark:text-amber-400" aria-hidden="true" />
              <span>{activeEvent.time}</span>
            </span>
          {/if}
        </div>

        <!-- Título com Destaque -->
        <h3 class="text-sm sm:text-base font-black text-foreground leading-tight tracking-tight drop-shadow-xs">
          {activeEvent.title || 'Nome do Culto'}
        </h3>

        <!-- Detalhes do Evento -->
        {#if activeEvent.details}
          <p class="text-[11px] text-muted-foreground font-medium leading-snug line-clamp-2">
            {activeEvent.details}
          </p>
        {/if}
      </div>
    {:else}
      <span class="text-xs text-muted-foreground">Nenhum evento selecionado</span>
    {/if}
  </div>

  <!-- Demonstração de como aparece no Monitor de Palco -->
  <div class="pt-3 border-t border-border space-y-2">
    <span class="text-[11px] font-semibold text-muted-foreground block uppercase tracking-wider">
      Comportamento no Monitor de Palco (/stage):
    </span>
    <div class="grid grid-cols-2 gap-2 text-[11px] font-mono">
      <div class="p-2.5 rounded-lg bg-background border border-border">
        <span class="text-amber-700 dark:text-amber-400 font-bold block mb-0.5 text-[11px]">
          SLIDE ATUAL:
        </span>
        <div class="text-foreground font-semibold truncate text-xs">
          {activeEvent?.dayOrDate || '-'}
        </div>
        <div class="text-muted-foreground truncate text-[11px] mt-0.5">
          {activeEvent?.time ? `${activeEvent.time} • ` : ''}{activeEvent?.title || ''}
        </div>
      </div>
      <div class="p-2.5 rounded-lg bg-background border border-border">
        <span class="text-amber-600 dark:text-yellow-400 font-bold block mb-0.5 text-[11px]">
          A SEGUIR:
        </span>
        {#if nextEvent}
          <div class="text-foreground font-semibold truncate text-xs">
            {nextEvent.dayOrDate}
          </div>
          <div class="text-muted-foreground truncate text-[11px] mt-0.5">
            {nextEvent.time ? `${nextEvent.time} • ` : ''}{nextEvent.title}
          </div>
        {:else}
          <div class="text-muted-foreground italic text-[11px] pt-1">
            Fim da agenda
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

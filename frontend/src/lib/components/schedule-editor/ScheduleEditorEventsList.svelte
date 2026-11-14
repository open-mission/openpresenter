<script lang="ts">
  import { Plus } from 'lucide-svelte';
  import type { ScheduleEvent } from '$lib/types';
  import ScheduleEditorEventCard from './ScheduleEditorEventCard.svelte';

  interface Props {
    events: ScheduleEvent[];
    selectedIndex: number;
    onaddevent: () => void;
    onselect: (index: number) => void;
    onmoveup: (index: number) => void;
    onmovedown: (index: number) => void;
    onremove: (index: number) => void;
  }

  let {
    events = $bindable([]),
    selectedIndex,
    onaddevent,
    onselect,
    onmoveup,
    onmovedown,
    onremove,
  }: Props = $props();
</script>

<div class="space-y-2">
  <div class="flex items-center justify-between text-xs text-muted-foreground select-none">
    <span class="font-semibold uppercase tracking-wider">Cultos e Eventos da Agenda:</span>
    <button
      type="button"
      onclick={onaddevent}
      class="inline-flex items-center gap-1 min-h-[28px] text-amber-700 dark:text-amber-400 hover:text-amber-800 dark:hover:text-amber-300 font-semibold cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring rounded-md px-1"
      aria-label="Adicionar novo evento à agenda"
    >
      <Plus size={14} aria-hidden="true" />
      <span>Novo Evento</span>
    </button>
  </div>

  <div class="space-y-2.5">
    {#each events as event, index (event.id)}
      <ScheduleEditorEventCard
        bind:event={events[index]}
        {index}
        isSelected={selectedIndex === index}
        isFirst={index === 0}
        isLast={index === events.length - 1}
        onselect={() => onselect(index)}
        onmoveup={() => onmoveup(index)}
        onmovedown={() => onmovedown(index)}
        onremove={() => onremove(index)}
      />
    {/each}
  </div>
</div>

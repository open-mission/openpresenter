<script lang="ts">
  import { Sparkles } from 'lucide-svelte';
  import { PRESET_SCHEDULE_DAYS } from './defaults';
  import type { SchedulePresetItem } from './types';

  interface Props {
    onaddpreset: (preset: SchedulePresetItem) => void;
  }

  let { onaddpreset }: Props = $props();
</script>

<div class="space-y-1.5 select-none">
  <div class="flex items-center justify-between">
    <span
      class="text-[11px] font-semibold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5"
    >
      <Sparkles size={12} class="text-amber-700 dark:text-amber-400" aria-hidden="true" />
      <span>Adicionar Atalho Rápido:</span>
    </span>
  </div>
  <div class="flex flex-wrap gap-1.5">
    {#each PRESET_SCHEDULE_DAYS as preset}
      <button
        type="button"
        onclick={() => onaddpreset(preset)}
        class="min-h-[28px] px-2.5 py-1 rounded-lg bg-muted hover:bg-secondary text-foreground text-xs font-medium border border-border hover:border-amber-600/40 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        title={`Adicionar ${preset.dayOrDate} (${preset.title})`}
        aria-label={`Adicionar atalho de ${preset.dayOrDate}`}
      >
        + {preset.dayOrDate.split('-')[0].trim()}
      </button>
    {/each}
  </div>
</div>

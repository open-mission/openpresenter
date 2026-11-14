<script lang="ts">
  import { Clock } from 'lucide-svelte';
  import { TIMER_PRESETS } from './defaults';

  interface Props {
    formattedTime: string;
    minutes: number;
    seconds: number;
    onselectpreset: (mins: number, secs: number) => void;
    onupdateminutes: (mins: number) => void;
    onupdateseconds: (secs: number) => void;
  }

  let {
    formattedTime,
    minutes,
    seconds,
    onselectpreset,
    onupdateminutes,
    onupdateseconds,
  }: Props = $props();

  function handleMinutesInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onupdateminutes(parseInt(target.value, 10));
  }

  function handleSecondsInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onupdateseconds(parseInt(target.value, 10));
  }
</script>

<div class="p-4 rounded-2xl bg-muted/40 border border-border space-y-3.5">
  <div class="flex items-center justify-between">
    <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
      <Clock size={14} class="text-amber-500" aria-hidden="true" />
      <span>Duração da Contagem</span>
    </span>
    <div
      class="text-sm font-mono font-bold text-amber-600 dark:text-amber-400 bg-amber-500/10 px-3 py-1 rounded-lg border border-amber-500/25"
      aria-label={`Tempo total configurado: ${formattedTime}`}
    >
      {formattedTime}
    </div>
  </div>

  <!-- Presets Rápidos -->
  <div>
    <span class="block text-xs text-muted-foreground font-medium mb-1.5">Presets rápidos:</span>
    <div
      role="radiogroup"
      aria-label="Presets rápidos de duração"
      class="flex flex-wrap gap-1.5"
    >
      {#each TIMER_PRESETS as p}
        {@const isSelected = minutes === p.mins && seconds === p.secs}
        <button
          type="button"
          role="radio"
          aria-checked={isSelected}
          tabindex={isSelected ? 0 : -1}
          onclick={() => onselectpreset(p.mins, p.secs)}
          class="px-3 py-1.5 min-h-[30px] rounded-lg text-xs font-semibold transition-all cursor-pointer border focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isSelected
            ? 'bg-amber-500 text-black border-amber-400 font-bold shadow-xs'
            : 'bg-card hover:bg-muted text-muted-foreground hover:text-foreground border-border'}"
        >
          {p.label}
        </button>
      {/each}
    </div>
  </div>

  <!-- Inputs Numéricos (Minutos : Segundos) -->
  <div class="grid grid-cols-2 gap-3 pt-1">
    <div>
      <label for="timer-mins" class="block text-xs font-semibold text-muted-foreground mb-1">
        Minutos
      </label>
      <input
        id="timer-mins"
        type="number"
        min="0"
        max="599"
        value={minutes}
        oninput={handleMinutesInput}
        class="w-full h-10 px-3 bg-background border border-border rounded-xl text-sm font-mono text-foreground text-center focus:outline-none focus:ring-2 focus:ring-ring focus:border-amber-500"
      />
    </div>
    <div>
      <label for="timer-secs" class="block text-xs font-semibold text-muted-foreground mb-1">
        Segundos
      </label>
      <input
        id="timer-secs"
        type="number"
        min="0"
        max="59"
        value={seconds}
        oninput={handleSecondsInput}
        class="w-full h-10 px-3 bg-background border border-border rounded-xl text-sm font-mono text-foreground text-center focus:outline-none focus:ring-2 focus:ring-ring focus:border-amber-500"
      />
    </div>
  </div>
</div>

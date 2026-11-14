<script lang="ts">
  interface Props {
    id: string;
    label: string;
    value: number;
    min: number;
    max: number;
    step?: number;
    unit?: string;
    hint?: string;
    minLabel?: string;
    midLabel?: string;
    maxLabel?: string;
    allowNumberInput?: boolean;
    numberMin?: number;
    numberMax?: number;
    numberStep?: number;
    onchange: (next: number) => void;
  }

  let {
    id,
    label,
    value,
    min,
    max,
    step = 1,
    unit = 'px',
    hint = '',
    minLabel = '',
    midLabel = '',
    maxLabel = '',
    allowNumberInput = false,
    numberMin = min,
    numberMax = max,
    numberStep = step,
    onchange,
  }: Props = $props();
</script>

<div class="p-3 rounded-lg border border-border bg-card/80 space-y-2">
  <div class="flex justify-between items-center gap-2 text-xs">
    <label for={id} class="text-foreground font-medium">{label}</label>

    {#if allowNumberInput}
      <div class="flex items-center gap-1.5 shrink-0">
        <input
          type="number"
          min={numberMin}
          max={numberMax}
          step={numberStep}
          {value}
          aria-label="{label} ({unit})"
          oninput={(e) => {
            const val = Number((e.target as HTMLInputElement).value);
            if (!Number.isNaN(val) && val >= numberMin) onchange(val);
          }}
          class="w-16 h-7 px-1.5 text-right font-mono font-bold text-xs bg-background border border-border rounded text-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        />
        {#if unit}
          <span class="text-[11px] font-mono text-muted-foreground">{unit}</span>
        {/if}
      </div>
    {:else}
      <span class="font-mono text-primary font-bold bg-primary/10 px-2 py-0.5 rounded border border-primary/30 text-[11px] shrink-0">
        {value}{unit}
      </span>
    {/if}
  </div>

  <input
    {id}
    type="range"
    {min}
    {max}
    {step}
    {value}
    oninput={(e) => onchange(Number((e.target as HTMLInputElement).value))}
    class="w-full h-1.5 bg-muted rounded-lg appearance-none cursor-pointer accent-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
  />

  {#if minLabel || midLabel || maxLabel}
    <div class="flex justify-between gap-2 text-[11px] text-muted-foreground font-mono">
      <span>{minLabel}</span>
      {#if midLabel}
        <span>{midLabel}</span>
      {/if}
      <span>{maxLabel}</span>
    </div>
  {:else if hint}
    <p class="text-[11px] text-muted-foreground">{hint}</p>
  {/if}
</div>

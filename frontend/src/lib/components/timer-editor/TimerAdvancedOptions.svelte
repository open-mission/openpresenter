<script lang="ts">
  interface Props {
    allowOvertime: boolean;
    ontoggleovertime: () => void;
  }

  let { allowOvertime, ontoggleovertime }: Props = $props();
</script>

<div class="p-3.5 rounded-xl bg-muted/40 border border-border text-xs flex items-center justify-between gap-4">
  <div class="space-y-0.5">
    <span id="timer-overtime-label" class="font-bold text-foreground block text-xs">
      Permitir Overtime
    </span>
    <p id="timer-overtime-desc" class="text-xs text-muted-foreground">
      Continua contando com sinal negativo após chegar a zero (-00:01, -00:02...)
    </p>
  </div>
  <button
    type="button"
    role="switch"
    id="timer-overtime-switch"
    aria-checked={allowOvertime}
    aria-labelledby="timer-overtime-label"
    aria-describedby="timer-overtime-desc"
    onclick={ontoggleovertime}
    class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {allowOvertime
      ? 'bg-amber-500'
      : 'bg-muted-foreground/30'}"
  >
    <span
      class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white dark:bg-neutral-900 shadow-md ring-0 transition duration-200 ease-in-out {allowOvertime
        ? 'translate-x-5'
        : 'translate-x-0'}"
      aria-hidden="true"
    ></span>
  </button>
</div>

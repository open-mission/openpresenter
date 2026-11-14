<script lang="ts">
  interface Props {
    title?: string;
    artistOrRef?: string;
  }

  let { title = $bindable(''), artistOrRef = $bindable('') }: Props = $props();

  const fieldClass =
    'w-full px-3 py-1.5 rounded-lg bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/40 transition-colors';
</script>

<div class="grid grid-cols-1 sm:grid-cols-3 gap-3 p-3.5 rounded-xl bg-card border border-border shadow-xs">
  <div class="sm:col-span-2 space-y-1">
    <label
      for="sched-title"
      class="text-[11px] font-semibold text-muted-foreground block uppercase tracking-wider"
    >
      Título da Programação <span class="text-amber-700 dark:text-amber-300">*</span>
    </label>
    <input
      id="sched-title"
      type="text"
      bind:value={title}
      placeholder="Ex: Programação Semanal dos Cultos"
      class={fieldClass}
    />
  </div>
  <div class="space-y-1">
    <label
      for="sched-ref"
      class="text-[11px] font-semibold text-muted-foreground block uppercase tracking-wider"
    >
      Categoria / Tag
    </label>
    <input
      id="sched-ref"
      type="text"
      bind:value={artistOrRef}
      placeholder="Ex: Agenda Semanal"
      class={fieldClass}
    />
  </div>
</div>

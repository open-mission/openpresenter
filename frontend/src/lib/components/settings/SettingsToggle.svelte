<script lang="ts">
  interface Props {
    id?: string;
    checked: boolean;
    label?: string;
    description?: string;
    ariaLabel?: string;
    tone?: 'emerald' | 'primary' | 'amber';
    compact?: boolean;
    ontoggle?: () => void;
    onToggle?: () => void;
    icon?: import('svelte').Snippet;
  }

  let {
    id,
    checked,
    label = '',
    description = '',
    ariaLabel,
    tone = 'emerald',
    compact = false,
    ontoggle,
    onToggle,
    icon,
  }: Props = $props();

  function handleToggle() {
    (ontoggle ?? onToggle)?.();
  }

  const activeTrackClass = $derived(
    tone === 'amber'
      ? 'bg-amber-600'
      : tone === 'primary'
        ? 'bg-primary'
        : 'bg-emerald-600'
  );
</script>

{#snippet switchButton()}
  <button
    {id}
    type="button"
    role="switch"
    aria-label={ariaLabel || label}
    aria-checked={checked}
    onclick={handleToggle}
    class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background {checked ? activeTrackClass : 'bg-muted-foreground/35'}"
  >
    <span
      aria-hidden="true"
      class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow-sm ring-0 transition duration-200 ease-in-out {checked ? 'translate-x-5' : 'translate-x-0'}"
    ></span>
  </button>
{/snippet}

{#if label}
  <div
    class="rounded-xl border border-border bg-card/60 flex items-center justify-between gap-4 {compact ? 'p-3' : 'p-3.5'}"
  >
    <div class="min-w-0">
      <div class="text-xs font-semibold text-foreground flex items-center gap-2">
        {#if icon}
          {@render icon()}
        {/if}
        <span>{label}</span>
      </div>
      {#if description}
        <p class="text-[11px] text-muted-foreground mt-0.5 leading-relaxed">
          {description}
        </p>
      {/if}
    </div>

    {@render switchButton()}
  </div>
{:else}
  {@render switchButton()}
{/if}

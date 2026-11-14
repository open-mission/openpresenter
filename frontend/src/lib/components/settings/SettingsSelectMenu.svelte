<script lang="ts">
  import { Check, ChevronDown } from 'lucide-svelte';

  export interface SelectMenuItem {
    id: string;
    label: string;
    subtitle?: string;
    swatchClass?: string;
  }

  interface Props {
    id: string;
    label: string;
    selectedId: string;
    selectedLabel: string;
    selectedSwatchClass?: string;
    items: SelectMenuItem[];
    open: boolean;
    onToggleOpen: () => void;
    onClose: () => void;
    onSelect: (id: string) => void;
  }

  let {
    id,
    label,
    selectedId,
    selectedLabel,
    selectedSwatchClass = '',
    items,
    open,
    onToggleOpen,
    onClose,
    onSelect,
  }: Props = $props();

  let containerEl = $state<HTMLDivElement | null>(null);
  let triggerBtnEl = $state<HTMLButtonElement | null>(null);

  export function focusTrigger() {
    triggerBtnEl?.focus();
  }

  function handleWindowClick(e: MouseEvent) {
    if (!open) return;
    const target = e.target as Node | null;
    if (containerEl && target && !containerEl.contains(target)) {
      onClose();
    }
  }

  function handleListKeydown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      onClose();
      triggerBtnEl?.focus();
      return;
    }

    const options = Array.from(
      containerEl?.querySelectorAll<HTMLButtonElement>('[role="option"]') || []
    );
    if (options.length === 0) return;

    const currentIdx = options.findIndex((el) => el === document.activeElement);
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      const next = options[(currentIdx + 1) % options.length];
      next?.focus();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      const prev = options[(currentIdx - 1 + options.length) % options.length];
      prev?.focus();
    } else if (e.key === 'Home') {
      e.preventDefault();
      options[0]?.focus();
    } else if (e.key === 'End') {
      e.preventDefault();
      options[options.length - 1]?.focus();
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div bind:this={containerEl} class="relative">
  <span id="{id}-label" class="block text-[11px] font-medium text-muted-foreground mb-1.5">
    {label}
  </span>

  <button
    bind:this={triggerBtnEl}
    type="button"
    aria-labelledby="{id}-label"
    aria-haspopup="listbox"
    aria-expanded={open}
    aria-controls="{id}-listbox"
    onclick={onToggleOpen}
    class="w-full px-3.5 py-2.5 rounded-xl border border-border bg-card hover:bg-muted/70 text-left flex items-center justify-between transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
  >
    <div class="flex items-center gap-2.5 min-w-0">
      {#if selectedSwatchClass}
        <span class="w-4 h-4 rounded-full border border-border shadow-sm shrink-0 {selectedSwatchClass}"></span>
      {/if}
      <span class="text-xs font-semibold text-foreground truncate">{selectedLabel}</span>
    </div>
    <ChevronDown size={14} class="text-muted-foreground transition-transform duration-150 shrink-0 {open ? 'rotate-180' : ''}" />
  </button>

  {#if open}
    <div
      id="{id}-listbox"
      role="listbox"
      aria-labelledby="{id}-label"
      tabindex="-1"
      onkeydown={handleListKeydown}
      class="absolute left-0 right-0 top-full mt-1.5 rounded-xl border border-border bg-popover text-popover-foreground p-1.5 shadow-2xl z-40 max-h-60 overflow-y-auto space-y-1"
    >
      {#each items as item (item.id)}
        {@const isSelected = selectedId === item.id}
        <button
          type="button"
          role="option"
          aria-selected={isSelected}
          onclick={() => {
            onSelect(item.id);
            onClose();
            triggerBtnEl?.focus();
          }}
          class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isSelected ? 'bg-muted text-foreground font-semibold' : 'text-foreground/85 hover:bg-muted/60 hover:text-foreground'}"
        >
          <div class="flex items-center gap-2.5 min-w-0">
            {#if item.swatchClass}
              <span class="w-3.5 h-3.5 rounded-full border border-border shrink-0 {item.swatchClass}"></span>
            {/if}
            <div class="flex flex-col min-w-0">
              <span class="text-xs truncate">{item.label}</span>
              {#if item.subtitle}
                <span class="text-[11px] text-muted-foreground truncate">{item.subtitle}</span>
              {/if}
            </div>
          </div>
          {#if isSelected}
            <Check size={14} class="text-primary shrink-0" />
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    title: string;
    description?: string;
    size?: 'sm' | 'md' | 'lg';
    onclose: () => void;
    children: Snippet;
    footer?: Snippet;
  }

  let { open, title, description = '', size = 'md', onclose, children, footer }: Props = $props();

  let dialogEl: HTMLDivElement | null = $state(null);
  let previouslyFocused: HTMLElement | null = null;

  const titleId = `teams-subdialog-title-${Math.random().toString(36).slice(2, 9)}`;
  const sizeClass = $derived(
    size === 'sm' ? 'max-w-sm' : size === 'lg' ? 'max-w-3xl' : 'max-w-lg'
  );

  function getFocusable(): HTMLElement[] {
    if (!dialogEl) return [];
    return Array.from(
      dialogEl.querySelectorAll<HTMLElement>(
        'button:not([disabled]), [href], input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
      )
    ).filter((el) => el.offsetParent !== null);
  }

  function focusFirstField() {
    if (!dialogEl) return;
    const fields = Array.from(
      dialogEl.querySelectorAll<HTMLElement>(
        'input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled])'
      )
    ).filter((el) => el.offsetParent !== null);
    (fields[0] ?? getFocusable()[0] ?? dialogEl).focus();
  }

  $effect(() => {
    if (open) {
      previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
      const timer = setTimeout(focusFirstField, 20);
      return () => clearTimeout(timer);
    }
    if (previouslyFocused) {
      previouslyFocused.focus?.();
      previouslyFocused = null;
    }
  });

  function handleKeyDown(e: KeyboardEvent) {
    if (!open || e.key !== 'Tab') return;
    const focusable = getFocusable();
    if (focusable.length === 0) return;
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    if (e.shiftKey && document.activeElement === first) {
      e.preventDefault();
      last.focus();
    } else if (!e.shiftKey && document.activeElement === last) {
      e.preventDefault();
      first.focus();
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-[45] bg-black/60 flex items-center justify-center p-4"
    role="presentation"
    onclick={onclose}
  >
    <div
      bind:this={dialogEl}
      class="w-full {sizeClass} max-h-[90vh] rounded-2xl border border-border bg-card text-card-foreground shadow-2xl flex flex-col overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeyDown}
      role="dialog"
      aria-modal="true"
      aria-labelledby={titleId}
      tabindex="-1"
    >
      <div class="px-4 py-3 border-b border-border flex items-start justify-between gap-3 shrink-0">
        <div class="min-w-0">
          <h3 id={titleId} class="text-sm font-bold text-foreground truncate">{title}</h3>
          {#if description}
            <p class="text-xs text-muted-foreground mt-0.5 leading-relaxed">{description}</p>
          {/if}
        </div>
        <button
          type="button"
          onclick={onclose}
          aria-label="Fechar {title}"
          class="w-8 h-8 shrink-0 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
        >
          <X size={16} />
        </button>
      </div>

      <div class="flex-1 min-h-0 overflow-y-auto p-4">
        {@render children()}
      </div>

      {#if footer}
        <div class="px-4 py-3 border-t border-border flex items-center justify-end gap-2 shrink-0">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}

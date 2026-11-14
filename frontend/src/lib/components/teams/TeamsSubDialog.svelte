<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { Snippet } from 'svelte';
  import { focusInitialElement, trapTabKey } from '../focusTrap';

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

  const uid = $props.id();
  const titleId = `${uid}-title`;
  const descId = `${uid}-desc`;
  const sizeClass = $derived(
    size === 'sm' ? 'max-w-sm' : size === 'lg' ? 'max-w-3xl' : 'max-w-lg'
  );
  const fieldSelector =
    'input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled])';

  function focusFirstField() {
    if (dialogEl) focusInitialElement(dialogEl, fieldSelector, dialogEl);
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
    if (!open || !dialogEl) return;
    trapTabKey(e, dialogEl);
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
      aria-describedby={description ? descId : undefined}
      tabindex="-1"
    >
      <div class="px-4 py-3 border-b border-border flex items-start justify-between gap-3 shrink-0">
        <div class="min-w-0">
          <h3 id={titleId} class="text-sm font-bold text-foreground truncate">{title}</h3>
          {#if description}
            <p id={descId} class="text-xs text-muted-foreground mt-0.5 leading-relaxed">{description}</p>
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

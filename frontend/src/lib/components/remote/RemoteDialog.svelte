<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    open: boolean;
    title: string;
    subtitle?: string;
    description?: string;
    variant?: 'center' | 'sheet';
    role?: 'dialog' | 'alertdialog';
    closeLabel?: string;
    maxWidthClass?: string;
    onclose: () => void;
    children?: Snippet;
    footer?: Snippet;
  }

  let {
    open,
    title,
    subtitle,
    description,
    variant = 'center',
    role = 'dialog',
    closeLabel = 'Fechar janela',
    maxWidthClass = 'max-w-sm',
    onclose,
    children,
    footer,
  }: Props = $props();

  const uid = $props.id();
  const titleId = `${uid}-title`;
  const descId = `${uid}-desc`;

  let dialogEl = $state<HTMLDivElement | null>(null);
  let previouslyFocused: HTMLElement | null = null;

  $effect(() => {
    if (open) {
      previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      const frame = requestAnimationFrame(() => {
        if (!dialogEl) return;
        const preferred = dialogEl.querySelector<HTMLElement>(
          '[data-autofocus], input:not([disabled]), select:not([disabled]), textarea:not([disabled])'
        );
        const fallback = dialogEl.querySelector<HTMLElement>(
          'button:not([disabled]), [href], [tabindex]:not([tabindex="-1"])'
        );
        (preferred || fallback || dialogEl).focus();
      });

      return () => cancelAnimationFrame(frame);
    } else if (previouslyFocused) {
      const targetToRestore = previouslyFocused;
      previouslyFocused = null;
      requestAnimationFrame(() => {
        if (targetToRestore.isConnected) {
          targetToRestore.focus();
        }
      });
    }
  });

  function handleKeyDown(e: KeyboardEvent) {
    if (!open) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      onclose();
      return;
    }

    if (e.key === 'Tab' && dialogEl) {
      const focusable = Array.from(
        dialogEl.querySelectorAll<HTMLElement>(
          'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
        )
      ).filter((el) => el.offsetParent !== null || el === document.activeElement);

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
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-50 bg-black/65 backdrop-blur-xs flex {variant === 'sheet'
      ? 'items-end sm:items-center justify-center sm:p-4'
      : 'items-center justify-center p-4'}"
    onclick={onclose}
    onkeydown={handleKeyDown}
    role="presentation"
  >
    <div
      bind:this={dialogEl}
      class="w-full {maxWidthClass} bg-card text-card-foreground border border-border shadow-2xl overflow-hidden flex flex-col {variant === 'sheet'
        ? 'rounded-t-2xl sm:rounded-2xl max-h-[82dvh] pb-[env(safe-area-inset-bottom)]'
        : 'rounded-2xl max-h-[88dvh]'}"
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeyDown}
      {role}
      aria-modal="true"
      aria-labelledby={titleId}
      aria-describedby={description ? descId : undefined}
      tabindex="-1"
    >
      <div class="px-4 py-3 border-b border-border bg-muted/40 flex items-center justify-between gap-3 shrink-0">
        <div class="min-w-0">
          <h2 id={titleId} class="text-sm font-bold text-foreground truncate">
            {title}
          </h2>
          {#if subtitle}
            <p class="text-xs text-muted-foreground mt-0.5 truncate">
              {subtitle}
            </p>
          {/if}
        </div>
        <button
          type="button"
          onclick={onclose}
          aria-label={closeLabel}
          class="h-10 w-10 -mr-1.5 rounded-xl flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer shrink-0"
        >
          <X size={18} aria-hidden="true" />
        </button>
      </div>

      {#if description}
        <p id={descId} class="px-4 pt-3 text-xs text-muted-foreground leading-relaxed">
          {description}
        </p>
      {/if}

      <div class="flex-1 min-h-0 overflow-y-auto">
        {#if children}
          {@render children()}
        {/if}
      </div>

      {#if footer}
        <div class="px-4 py-3 border-t border-border bg-muted/25 shrink-0">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}

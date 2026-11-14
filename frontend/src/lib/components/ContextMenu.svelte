<script lang="ts">
  import { tick } from 'svelte';
  import { cn } from '$lib/utils';

  export interface MenuItem {
    id: string;
    label?: string;
    icon?: any;
    shortcut?: string;
    destructive?: boolean;
    disabled?: boolean;
    divider?: boolean;
    onclick?: () => void;
  }

  interface Props {
    x: number;
    y: number;
    items: MenuItem[];
    open: boolean;
    onclose: () => void;
  }

  let { x = 0, y = 0, items = [], open = false, onclose }: Props = $props();

  let menuElement = $state<HTMLDivElement | null>(null);
  let adjustedX = $state(0);
  let adjustedY = $state(0);
  let previousFocusedElement: HTMLElement | null = null;

  function closeMenu() {
    onclose();
    if (previousFocusedElement && document.contains(previousFocusedElement)) {
      previousFocusedElement.focus();
    }
    previousFocusedElement = null;
  }

  $effect(() => {
    if (open) {
      previousFocusedElement = document.activeElement instanceof HTMLElement ? document.activeElement : null;
      const estimatedWidth = 220;
      const estimatedHeight = items.length * 36;
      adjustedX = Math.max(10, Math.min(x, window.innerWidth - estimatedWidth - 10));
      adjustedY = Math.max(10, Math.min(y, window.innerHeight - estimatedHeight - 10));

      void tick().then(() => {
        if (!menuElement || !open) return;
        const rect = menuElement.getBoundingClientRect();
        const maxX = window.innerWidth - rect.width - 10;
        const maxY = window.innerHeight - rect.height - 10;
        adjustedX = Math.max(10, Math.min(x, maxX));
        adjustedY = Math.max(10, Math.min(y, maxY));
        const firstEnabled = menuElement.querySelector<HTMLButtonElement>('[role="menuitem"]:not(:disabled)');
        firstEnabled?.focus();
      });
    }
  });

  function handleBackdrop(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    closeMenu();
  }

  function handleMenuKeydown(e: KeyboardEvent) {
    if (!open || !menuElement) return;
    const enabledItems = Array.from(
      menuElement.querySelectorAll<HTMLButtonElement>('[role="menuitem"]:not(:disabled)')
    );
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      closeMenu();
      return;
    }
    if (enabledItems.length === 0) return;

    const currentIdx = enabledItems.indexOf(document.activeElement as HTMLButtonElement);
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      const next = currentIdx < 0 ? 0 : (currentIdx + 1) % enabledItems.length;
      enabledItems[next].focus();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      const prev = currentIdx <= 0 ? enabledItems.length - 1 : currentIdx - 1;
      enabledItems[prev].focus();
    } else if (e.key === 'Home') {
      e.preventDefault();
      enabledItems[0].focus();
    } else if (e.key === 'End') {
      e.preventDefault();
      enabledItems[enabledItems.length - 1].focus();
    }
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (open && e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      closeMenu();
    }
  }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

{#if open}
  <!-- Backdrop transparente para capturar cliques fora -->
  <div
    class="fixed inset-0 z-50 bg-transparent"
    onclick={handleBackdrop}
    oncontextmenu={handleBackdrop}
    role="presentation"
  ></div>

  <!-- Menu Flutuante -->
  <div
    bind:this={menuElement}
    style="left: {adjustedX}px; top: {adjustedY}px;"
    class="fixed z-50 min-w-[210px] max-h-[85vh] overflow-y-auto rounded-lg border border-border bg-popover/95 backdrop-blur-md p-1.5 shadow-2xl text-xs text-popover-foreground select-none animate-in fade-in zoom-in-95 duration-100"
    onkeydown={handleMenuKeydown}
    role="menu"
    aria-label="Menu de contexto"
    tabindex="-1"
  >
    {#each items as item (item.id)}
      {#if item.divider}
        <div role="separator" class="h-[1px] bg-border my-1 mx-1"></div>
      {:else}
        <button
          type="button"
          role="menuitem"
          disabled={item.disabled}
          onclick={() => {
            if (!item.disabled && item.onclick) {
              item.onclick();
              closeMenu();
            }
          }}
          class={cn(
            'w-full min-h-8 flex items-center justify-between gap-3 px-2.5 py-1.5 rounded-md transition-colors text-left cursor-pointer group focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
            item.disabled
              ? 'opacity-45 cursor-not-allowed text-muted-foreground'
              : item.destructive
                ? 'hover:bg-red-500/15 focus-visible:bg-red-500/15 text-red-700 dark:text-red-400'
                : 'hover:bg-muted focus-visible:bg-muted text-popover-foreground'
          )}
        >
          <div class="flex items-center gap-2">
            {#if item.icon}
              {@const IconComp = item.icon}
              <IconComp
                size={14}
                class={cn(
                  'shrink-0',
                  item.destructive
                    ? 'text-red-700 dark:text-red-400'
                    : 'text-muted-foreground group-hover:text-popover-foreground'
                )}
              />
            {/if}
            <span>{item.label}</span>
          </div>

          {#if item.shortcut}
            <kbd class="text-[11px] font-mono text-muted-foreground px-1.5 py-0.5 rounded bg-muted border border-border">
              {item.shortcut}
            </kbd>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}

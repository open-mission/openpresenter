<script lang="ts">
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

  $effect(() => {
    if (open) {
      const menuWidth = 200;
      const menuHeight = items.length * 36;
      const maxX = window.innerWidth - menuWidth - 10;
      const maxY = window.innerHeight - menuHeight - 10;
      adjustedX = Math.min(x, maxX);
      adjustedY = Math.min(y, maxY);
    }
  });

  function handleBackdrop(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    onclose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.stopPropagation();
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop transparente para capturar cliques fora -->
  <div
    class="fixed inset-0 z-50 bg-transparent"
    onclick={handleBackdrop}
    oncontextmenu={handleBackdrop}
    role="presentation"
  ></div>

  <!-- Menu Flutuante -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    bind:this={menuElement}
    style="left: {adjustedX}px; top: {adjustedY}px;"
    class="fixed z-50 min-w-[200px] rounded-lg border border-neutral-800 bg-neutral-900/95 backdrop-blur-md p-1.5 shadow-2xl text-xs text-neutral-200 select-none animate-in fade-in zoom-in-95 duration-100"
    onclick={(e) => e.stopPropagation()}
    role="menu"
    tabindex="-1"
  >
    {#each items as item (item.id)}
      {#if item.divider}
        <div class="h-[1px] bg-neutral-800/80 my-1 mx-1"></div>
      {:else}
        <button
          type="button"
          disabled={item.disabled}
          onclick={() => {
            if (!item.disabled && item.onclick) {
              item.onclick();
              onclose();
            }
          }}
          class={cn(
            'w-full flex items-center justify-between px-2.5 py-1.5 rounded-md transition-colors text-left cursor-pointer group',
            item.disabled
              ? 'opacity-40 cursor-not-allowed text-neutral-500'
              : item.destructive
                ? 'hover:bg-red-500/20 text-red-400 hover:text-red-300'
                : 'hover:bg-neutral-800 hover:text-white text-neutral-200'
          )}
        >
          <div class="flex items-center gap-2">
            {#if item.icon}
              {@const IconComp = item.icon}
              <IconComp
                size={14}
                class={cn(
                  'shrink-0',
                  item.destructive ? 'text-red-400' : 'text-neutral-400 group-hover:text-white'
                )}
              />
            {/if}
            <span>{item.label}</span>
          </div>

          {#if item.shortcut}
            <kbd class="text-[10px] font-mono text-neutral-500 px-1 py-0.5 rounded bg-neutral-950/60 border border-neutral-800">
              {item.shortcut}
            </kbd>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}

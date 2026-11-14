<script lang="ts">
  import { BookOpen, Globe, Music } from 'lucide-svelte';
  import { COMMAND_TABS, panelId, tabId, type CommandTab } from './types';

  interface Props {
    activeTab: CommandTab;
    localCount: number;
    bibleCount: number;
    onlineCount: number;
    onselect: (tab: CommandTab, keepFocus?: boolean) => void;
  }

  let { activeTab, localCount, bibleCount, onlineCount, onselect }: Props = $props();

  let tablistElement = $state<HTMLDivElement | null>(null);

  const labels: Record<CommandTab, string> = {
    local: 'Músicas',
    bible: 'Bíblia',
    online: 'Internet',
  };

  const icons = { local: Music, bible: BookOpen, online: Globe };

  function countFor(tab: CommandTab): number {
    if (tab === 'local') return localCount;
    if (tab === 'bible') return bibleCount;
    return onlineCount;
  }

  function focusTab(index: number) {
    const next = COMMAND_TABS[index];
    onselect(next, true);
    const buttons = tablistElement?.querySelectorAll<HTMLButtonElement>('[role="tab"]');
    buttons?.[index]?.focus();
  }

  function handleKeydown(event: KeyboardEvent, index: number) {
    let nextIndex: number | null = null;
    if (event.key === 'ArrowRight') nextIndex = (index + 1) % COMMAND_TABS.length;
    else if (event.key === 'ArrowLeft') nextIndex = (index - 1 + COMMAND_TABS.length) % COMMAND_TABS.length;
    else if (event.key === 'Home') nextIndex = 0;
    else if (event.key === 'End') nextIndex = COMMAND_TABS.length - 1;
    if (nextIndex === null) return;
    event.preventDefault();
    focusTab(nextIndex);
  }

  function tabClasses(tab: CommandTab, isActive: boolean): string {
    const accent =
      tab === 'local'
        ? 'text-emerald-700 dark:text-emerald-300'
        : tab === 'bible'
          ? 'text-amber-700 dark:text-amber-300'
          : 'text-sky-700 dark:text-sky-300';
    return isActive
      ? `bg-background font-semibold shadow-sm ring-1 ring-border ${accent}`
      : 'text-muted-foreground hover:bg-background/60 hover:text-foreground';
  }
</script>

<div class="flex items-center gap-1 overflow-x-auto rounded-lg border border-border bg-muted/50 p-0.5">
  <div bind:this={tablistElement} role="tablist" aria-label="Escopo da busca global" class="flex items-center gap-0.5">
    {#each COMMAND_TABS as tab, index (tab)}
      {@const Icon = icons[tab]}
      {@const count = countFor(tab)}
      {@const isActive = activeTab === tab}
      <button
        type="button"
        role="tab"
        id={tabId(tab)}
        aria-selected={isActive}
        aria-controls={panelId(tab)}
        tabindex={isActive ? 0 : -1}
        title={`${labels[tab]} (Ctrl+${index + 1})`}
        onclick={() => onselect(tab)}
        onkeydown={(event) => handleKeydown(event, index)}
        class="flex min-h-7 shrink-0 cursor-pointer items-center gap-1.5 whitespace-nowrap rounded-md px-2.5 text-[11px] transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {tabClasses(
          tab,
          isActive
        )}"
      >
        <Icon size={13} class="shrink-0" aria-hidden="true" />
        <span>{labels[tab]}</span>
        {#if tab === 'local' || count > 0}
          <span class="rounded bg-background/70 px-1 font-mono text-[11px] text-muted-foreground">
            {count}
          </span>
        {/if}
      </button>
    {/each}
  </div>
</div>

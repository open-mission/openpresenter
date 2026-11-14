<script lang="ts">
  import { Tv, MonitorPlay, Smartphone, Video, Cast } from 'lucide-svelte';
  import type { ShareOutputTab } from './types';

  interface Props {
    activeTab: ShareOutputTab;
    onselecttab: (tab: ShareOutputTab) => void;
  }

  let { activeTab, onselecttab }: Props = $props();

  const TABS: { id: ShareOutputTab; label: string; icon: typeof Tv }[] = [
    { id: 'stage', label: 'Palco', icon: Tv },
    { id: 'public', label: 'Pública', icon: MonitorPlay },
    { id: 'remote', label: 'Remoto', icon: Smartphone },
    { id: 'lower', label: 'OBS', icon: Video },
    { id: 'cast', label: 'Cast', icon: Cast },
  ];

  function handleKeydown(e: KeyboardEvent, currentIndex: number) {
    let nextIndex = -1;
    if (e.key === 'ArrowRight') {
      nextIndex = (currentIndex + 1) % TABS.length;
    } else if (e.key === 'ArrowLeft') {
      nextIndex = (currentIndex - 1 + TABS.length) % TABS.length;
    } else if (e.key === 'Home') {
      nextIndex = 0;
    } else if (e.key === 'End') {
      nextIndex = TABS.length - 1;
    }

    if (nextIndex >= 0) {
      e.preventDefault();
      onselecttab(TABS[nextIndex].id);
      const tabButtons = document.querySelectorAll<HTMLButtonElement>('[role="tablist"] [role="tab"]');
      tabButtons[nextIndex]?.focus();
    }
  }
</script>

<div class="p-3.5 pb-2 border-b border-border bg-card/40 shrink-0 select-none">
  <div
    role="tablist"
    aria-label="Opções de saídas para compartilhamento"
    class="grid grid-cols-5 gap-1 p-1 bg-muted/40 rounded-xl border border-border"
  >
    {#each TABS as tab, idx (tab.id)}
      {@const isSelected = activeTab === tab.id}
      {@const Icon = tab.icon}
      <button
        type="button"
        role="tab"
        id={`share-tab-${tab.id}`}
        aria-selected={isSelected}
        aria-controls={`share-panel-${tab.id}`}
        tabindex={isSelected ? 0 : -1}
        onclick={() => onselecttab(tab.id)}
        onkeydown={(e) => handleKeydown(e, idx)}
        class="flex items-center justify-center gap-1.5 py-1.5 px-2 rounded-lg text-xs font-semibold transition-all cursor-pointer min-h-[32px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isSelected
          ? 'bg-primary text-primary-foreground shadow-xs font-bold'
          : 'text-muted-foreground hover:text-foreground hover:bg-muted/50'}"
      >
        <Icon size={14} aria-hidden="true" />
        <span class="truncate">{tab.label}</span>
      </button>
    {/each}
  </div>
</div>

<script lang="ts">
  import { ExternalLink, Maximize2, Minimize2, Users, X } from 'lucide-svelte';
  import { TEAMS_TABS, type TeamsTabId } from './teamConstants';

  interface Props {
    activeTab: TeamsTabId;
    membersCount: number;
    departmentsCount: number;
    isMaximized: boolean;
    onSelectTab: (id: TeamsTabId) => void;
    onToggleMaximize: () => void;
    onClose: () => void;
  }

  let {
    activeTab,
    membersCount,
    departmentsCount,
    isMaximized,
    onSelectTab,
    onToggleMaximize,
    onClose,
  }: Props = $props();

  function handleTabKeydown(e: KeyboardEvent) {
    const index = TEAMS_TABS.findIndex((tab) => tab.id === activeTab);
    let next = index;
    if (e.key === 'ArrowRight') next = (index + 1) % TEAMS_TABS.length;
    else if (e.key === 'ArrowLeft') next = (index - 1 + TEAMS_TABS.length) % TEAMS_TABS.length;
    else if (e.key === 'Home') next = 0;
    else if (e.key === 'End') next = TEAMS_TABS.length - 1;
    else return;
    e.preventDefault();
    const id = TEAMS_TABS[next].id;
    onSelectTab(id);
    document.getElementById(`teams-tab-${id}`)?.focus();
  }

  function tabCount(id: TeamsTabId): number | null {
    if (id === 'members') return membersCount;
    if (id === 'departments') return departmentsCount;
    return null;
  }
</script>

<header
  class="px-4 sm:px-6 py-2.5 border-b border-border flex flex-wrap items-center gap-x-3 gap-y-2 shrink-0 bg-card"
>
  <div class="flex items-center gap-2.5 min-w-0">
    <div
      class="w-9 h-9 rounded-xl bg-primary/10 border border-primary/30 text-primary flex items-center justify-center shrink-0"
    >
      <Users size={18} />
    </div>
    <div class="min-w-0">
      <h2 id="teams-modal-title" class="text-sm sm:text-base font-bold text-foreground truncate">
        Equipe de Mídia &amp; Escalas
      </h2>
      <p class="text-[11px] text-muted-foreground truncate">
        Voluntários, departamentos e escalas dos cultos
      </p>
    </div>
  </div>

  <div class="ml-auto flex flex-wrap items-center gap-2">
    <div
      class="flex items-center gap-0.5 p-0.5 rounded-xl bg-muted border border-border"
      role="tablist"
      aria-label="Seções da equipe"
    >
      {#each TEAMS_TABS as tab (tab.id)}
        {@const TabIcon = tab.icon}
        {@const count = tabCount(tab.id)}
        <button
          type="button"
          role="tab"
          id="teams-tab-{tab.id}"
          aria-selected={activeTab === tab.id}
          aria-controls="teams-panel-{tab.id}"
          tabindex={activeTab === tab.id ? 0 : -1}
          onclick={() => onSelectTab(tab.id)}
          onkeydown={handleTabKeydown}
          class="inline-flex items-center gap-1.5 h-8 px-2.5 rounded-lg text-xs font-semibold transition-colors cursor-pointer {activeTab ===
          tab.id
            ? 'bg-card text-foreground shadow-sm'
            : 'text-muted-foreground hover:text-foreground'}"
        >
          <TabIcon size={13} />
          <span>{tab.label}</span>
          {#if count !== null}
            <span class="text-[11px] text-muted-foreground">({count})</span>
          {/if}
        </button>
      {/each}
    </div>

    <a
      href="/docs"
      target="_blank"
      rel="noopener noreferrer"
      aria-label="Abrir documentação da API em nova aba"
      class="inline-flex items-center gap-1.5 h-8 px-2.5 rounded-lg border border-border bg-background hover:bg-muted text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
      title="Documentação OpenAPI interativa (offline)"
    >
      <ExternalLink size={13} />
      <span class="hidden lg:inline">API Docs</span>
    </a>

    <button
      type="button"
      onclick={onToggleMaximize}
      aria-label={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
      title={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
    >
      {#if isMaximized}
        <Minimize2 size={15} />
      {:else}
        <Maximize2 size={15} />
      {/if}
    </button>

    <button
      type="button"
      onclick={onClose}
      aria-label="Fechar equipe e escalas"
      title="Fechar (Esc)"
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
    >
      <X size={16} />
    </button>
  </div>
</header>

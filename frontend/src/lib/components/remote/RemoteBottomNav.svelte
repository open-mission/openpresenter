<script lang="ts">
  import { BookOpen, Home, KeyRound, ListMusic, Music, Users } from 'lucide-svelte';

  export type RemoteTab = 'home' | 'setlist' | 'songs' | 'bible' | 'team';

  interface Props {
    activeTab: RemoteTab;
    canControlRemote: boolean;
    onSelectTab: (tab: RemoteTab) => void;
    onOpenPinModal: () => void;
  }

  let { activeTab, canControlRemote, onSelectTab, onOpenPinModal }: Props = $props();

  const navItems = $derived.by(() => {
    if (canControlRemote) {
      return [
        { id: 'home' as const, label: 'Início', kind: 'tab' as const },
        { id: 'setlist' as const, label: 'Setlist', kind: 'tab' as const },
        { id: 'songs' as const, label: 'Músicas', kind: 'tab' as const },
        { id: 'bible' as const, label: 'Bíblia', kind: 'tab' as const },
        { id: 'team' as const, label: 'Equipe', kind: 'tab' as const },
      ];
    }
    return [
      { id: 'home' as const, label: 'Início', kind: 'tab' as const },
      { id: 'setlist' as const, label: 'Setlist', kind: 'tab' as const },
      { id: 'team' as const, label: 'Equipe', kind: 'tab' as const },
      { id: 'operator' as const, label: 'Operador', kind: 'action' as const },
    ];
  });

  function handleNavKeyDown(e: KeyboardEvent, index: number) {
    if (e.key !== 'ArrowLeft' && e.key !== 'ArrowRight' && e.key !== 'Home' && e.key !== 'End') {
      return;
    }
    e.preventDefault();
    const currentTarget = e.currentTarget as HTMLElement;
    const buttons = Array.from(
      currentTarget.parentElement?.querySelectorAll<HTMLButtonElement>('button') || []
    );
    if (buttons.length === 0) return;

    let nextIdx = index;
    if (e.key === 'ArrowRight') nextIdx = (index + 1) % buttons.length;
    else if (e.key === 'ArrowLeft') nextIdx = (index - 1 + buttons.length) % buttons.length;
    else if (e.key === 'Home') nextIdx = 0;
    else if (e.key === 'End') nextIdx = buttons.length - 1;

    buttons[nextIdx].focus();
  }
</script>

<nav
  aria-label="Navegação principal do controle remoto"
  class="shrink-0 bg-card/95 backdrop-blur-md border-t border-border pb-[env(safe-area-inset-bottom)]"
>
  <div
    class="max-w-md md:max-w-4xl lg:max-w-5xl mx-auto px-2 py-1 grid {canControlRemote
      ? 'grid-cols-5'
      : 'grid-cols-4'} gap-1"
  >
    {#each navItems as item, idx (item.id)}
      {@const isSelected = item.kind === 'tab' && activeTab === item.id}
      <button
        type="button"
        aria-current={isSelected ? 'page' : undefined}
        onclick={() => {
          if (item.kind === 'tab') {
            onSelectTab(item.id);
          } else {
            onOpenPinModal();
          }
        }}
        onkeydown={(e) => handleNavKeyDown(e, idx)}
        class="min-h-[52px] rounded-xl flex flex-col items-center justify-center gap-1 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {isSelected
          ? 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 font-bold'
          : 'text-muted-foreground hover:text-foreground hover:bg-muted/60 font-medium'}"
      >
        {#if item.id === 'home'}
          <Home size={19} strokeWidth={isSelected ? 2.4 : 1.9} aria-hidden="true" />
        {:else if item.id === 'setlist'}
          <ListMusic size={19} strokeWidth={isSelected ? 2.4 : 1.9} aria-hidden="true" />
        {:else if item.id === 'songs'}
          <Music size={19} strokeWidth={isSelected ? 2.4 : 1.9} aria-hidden="true" />
        {:else if item.id === 'bible'}
          <BookOpen size={19} strokeWidth={isSelected ? 2.4 : 1.9} aria-hidden="true" />
        {:else if item.id === 'team'}
          <Users size={19} strokeWidth={isSelected ? 2.4 : 1.9} aria-hidden="true" />
        {:else}
          <KeyRound size={19} aria-hidden="true" />
        {/if}
        <span class="text-[11px] leading-none">{item.label}</span>
      </button>
    {/each}
  </div>
</nav>

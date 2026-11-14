<script lang="ts">
  import { Check, ListMusic } from 'lucide-svelte';
  import type { Setlist, SetlistSummary } from '$lib/types';
  import RemoteDialog from './RemoteDialog.svelte';
  import { triggerHaptic } from '$lib/remoteData';

  interface Props {
    open: boolean;
    setlists: SetlistSummary[];
    activeSetlist: Setlist | null;
    onSelectSetlist: (id: string) => void;
    onClose: () => void;
  }

  let { open, setlists, activeSetlist, onSelectSetlist, onClose }: Props = $props();

  function handleSelect(id: string) {
    triggerHaptic('light');
    onSelectSetlist(id);
  }
</script>

<RemoteDialog
  {open}
  variant="sheet"
  title="Escolher Setlist"
  closeLabel="Fechar seletor de setlist"
  maxWidthClass="max-w-md md:max-w-lg"
  onclose={onClose}
>
  <div class="p-3 space-y-1.5">
    {#if setlists.length === 0}
      <p class="py-8 text-center text-xs text-muted-foreground">
        Nenhuma setlist salva no computador principal.
      </p>
    {:else}
      {#each setlists as s (s.id)}
        {@const selected = activeSetlist?.id === s.id}
        <button
          type="button"
          onclick={() => handleSelect(s.id)}
          class="w-full min-h-[56px] p-3 rounded-xl border flex items-center justify-between gap-3 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {selected
            ? 'bg-emerald-500/15 border-emerald-600/60 dark:border-emerald-500 text-foreground'
            : 'bg-background hover:bg-muted border-border text-foreground'}"
        >
          <span class="flex items-center gap-3 min-w-0">
            <ListMusic size={17} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
            <span class="min-w-0">
              <span class="block text-sm font-bold truncate">{s.title}</span>
              <span class="block text-xs text-muted-foreground">
                {s.item_count} {s.item_count === 1 ? 'item' : 'itens'}
                {s.is_active ? ' • Ativa no culto' : ''}
              </span>
            </span>
          </span>
          {#if selected}
            <Check size={17} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
          {/if}
        </button>
      {/each}
    {/if}
  </div>
</RemoteDialog>

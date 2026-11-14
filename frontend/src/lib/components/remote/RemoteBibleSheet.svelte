<script lang="ts">
  import { BookOpen, Check } from 'lucide-svelte';
  import type { BibleVersionSummary } from '$lib/types';
  import RemoteDialog from './RemoteDialog.svelte';
  import { triggerHaptic } from '$lib/remoteData';

  interface Props {
    open: boolean;
    bibles: BibleVersionSummary[];
    selectedBibleId: string;
    onSelectBible: (id: string) => void;
    onClose: () => void;
  }

  let { open, bibles, selectedBibleId, onSelectBible, onClose }: Props = $props();

  function handleSelect(id: string) {
    triggerHaptic('light');
    onSelectBible(id);
  }
</script>

<RemoteDialog
  {open}
  variant="sheet"
  title="Versão da Bíblia"
  closeLabel="Fechar seletor de versão bíblica"
  maxWidthClass="max-w-md md:max-w-lg"
  onclose={onClose}
>
  <div class="p-3 space-y-1.5">
    {#if bibles.length === 0}
      <p class="py-8 text-center text-xs text-muted-foreground">
        Nenhuma versão bíblica encontrada no servidor.
      </p>
    {:else}
      {#each bibles as b (b.id)}
        {@const selected = selectedBibleId === b.id}
        <button
          type="button"
          onclick={() => handleSelect(b.id)}
          class="w-full min-h-[56px] p-3 rounded-xl border flex items-center justify-between gap-3 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {selected
            ? 'bg-emerald-500/15 border-emerald-600/60 dark:border-emerald-500 text-foreground'
            : 'bg-background hover:bg-muted border-border text-foreground'}"
        >
          <span class="flex items-center gap-3 min-w-0">
            <BookOpen size={17} class="text-emerald-700 dark:text-emerald-400 shrink-0" aria-hidden="true" />
            <span class="min-w-0">
              <span class="block text-sm font-bold truncate">{b.name}</span>
              <span class="block text-xs text-muted-foreground">{b.abbreviation}</span>
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

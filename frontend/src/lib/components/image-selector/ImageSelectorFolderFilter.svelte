<script lang="ts">
  import { Folder } from 'lucide-svelte';
  import type { MediaFolder } from '$lib/types';

  interface Props {
    selectedFolderFilter: string;
    folders: MediaFolder[];
    totalImagesCount: number;
    appImagesCount: number;
    isLowerThird: boolean;
    isAllSelected: boolean;
    filteredCount: number;
    onselectfolder: (id: string) => void;
    onselectall: () => void;
  }

  let {
    selectedFolderFilter,
    folders,
    totalImagesCount,
    appImagesCount,
    isLowerThird,
    isAllSelected,
    filteredCount,
    onselectfolder,
    onselectall,
  }: Props = $props();
</script>

<div class="flex items-center gap-1.5 overflow-x-auto pb-3 mb-3 border-b border-border text-xs select-none">
  <span class="text-xs text-muted-foreground font-medium flex items-center gap-1 shrink-0">
    <Folder size={14} aria-hidden="true" />
    <span>Pasta:</span>
  </span>

  <button
    type="button"
    onclick={() => onselectfolder('all')}
    class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors cursor-pointer shrink-0 min-h-[28px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedFolderFilter === 'all'
      ? 'bg-primary/15 text-primary font-bold shadow-xs'
      : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
  >
    Todas ({totalImagesCount})
  </button>

  <button
    type="button"
    onclick={() => onselectfolder('app')}
    class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors cursor-pointer shrink-0 min-h-[28px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedFolderFilter === 'app'
      ? 'bg-primary/15 text-primary font-bold shadow-xs'
      : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
  >
    Pasta do App ({appImagesCount})
  </button>

  {#each folders as f (f.id)}
    <button
      type="button"
      onclick={() => onselectfolder(f.id)}
      class="px-2.5 py-1 rounded-md text-xs font-medium transition-colors cursor-pointer shrink-0 min-h-[28px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedFolderFilter === f.id
        ? 'bg-primary/15 text-primary font-bold shadow-xs'
        : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      {f.name}
    </button>
  {/each}

  {#if !isLowerThird && filteredCount > 0}
    <div class="ml-auto shrink-0 pl-2">
      <button
        type="button"
        onclick={onselectall}
        class="text-xs text-primary hover:underline font-semibold cursor-pointer min-h-[28px] px-2 flex items-center focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring rounded"
      >
        {isAllSelected ? 'Desmarcar Todas' : 'Selecionar Todas'}
      </button>
    </div>
  {/if}
</div>

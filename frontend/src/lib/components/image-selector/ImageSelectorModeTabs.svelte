<script lang="ts">
  import { Search, X } from 'lucide-svelte';
  import type { ImageSelectorMode } from './types';

  interface Props {
    mode: ImageSelectorMode;
    allowFolders: boolean;
    isLowerThird: boolean;
    searchQuery: string;
    onselectmode: (mode: ImageSelectorMode) => void;
  }

  let {
    mode,
    allowFolders,
    isLowerThird,
    searchQuery = $bindable(),
    onselectmode,
  }: Props = $props();
</script>

<div class="px-5 pt-3 pb-2.5 border-b border-border bg-muted/20 flex flex-col sm:flex-row sm:items-center justify-between gap-3 shrink-0 select-none">
  {#if allowFolders && !isLowerThird}
    <div
      role="tablist"
      aria-label="Modo de seleção de imagens"
      class="flex items-center gap-1.5 p-1 rounded-lg bg-muted/40 border border-border"
    >
      <button
        type="button"
        role="tab"
        aria-selected={mode === 'single'}
        onclick={() => onselectmode('single')}
        class="px-3 py-1.5 rounded-md text-xs font-semibold transition-all cursor-pointer min-h-[30px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {mode === 'single'
          ? 'bg-primary text-primary-foreground shadow-xs'
          : 'text-muted-foreground hover:text-foreground'}"
      >
        Imagens Individuais
      </button>
      <button
        type="button"
        role="tab"
        aria-selected={mode === 'folder'}
        onclick={() => onselectmode('folder')}
        class="px-3 py-1.5 rounded-md text-xs font-semibold transition-all cursor-pointer min-h-[30px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {mode === 'folder'
          ? 'bg-primary text-primary-foreground shadow-xs'
          : 'text-muted-foreground hover:text-foreground'}"
      >
        Pasta Completa de Imagens
      </button>
    </div>
  {:else}
    <div class="text-xs font-medium text-muted-foreground">
      {isLowerThird ? 'Clique para selecionar a imagem alternativa' : 'Galeria de Imagens'}
    </div>
  {/if}

  {#if mode === 'single'}
    <!-- Filtro de Busca -->
    <div class="relative w-full sm:w-64">
      <label for="image-search-input" class="sr-only">Buscar imagem</label>
      <Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" aria-hidden="true" />
      <input
        id="image-search-input"
        type="search"
        bind:value={searchQuery}
        placeholder="Buscar imagem..."
        class="w-full h-8 pl-8 pr-7 rounded-lg bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary transition-colors"
      />
      {#if searchQuery}
        <button
          type="button"
          onclick={() => (searchQuery = '')}
          class="absolute right-2 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full text-muted-foreground hover:text-foreground flex items-center justify-center cursor-pointer"
          aria-label="Limpar busca"
        >
          <X size={12} aria-hidden="true" />
        </button>
      {/if}
    </div>
  {/if}
</div>

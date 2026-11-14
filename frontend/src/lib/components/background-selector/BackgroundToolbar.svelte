<script lang="ts">
  import { Search, ChevronRight, X } from 'lucide-svelte';
  import type { BackgroundCategory } from './types';
  import { CATEGORIES } from './defaults';

  interface Props {
    searchQuery: string;
    selectedCategory: BackgroundCategory;
  }

  let { searchQuery = $bindable(), selectedCategory }: Props = $props();

  const currentCategoryName = $derived(
    CATEGORIES.find((c) => c.id === selectedCategory)?.fullName || 'Todos os Fundos'
  );
</script>

<div class="px-5 py-2.5 border-b border-border bg-muted/20 flex flex-col sm:flex-row sm:items-center justify-between gap-3 shrink-0">
  <div class="relative flex-1 max-w-md">
    <label for="bg-search-input" class="sr-only">Pesquisar fundos e gradientes</label>
    <Search size={14} class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" aria-hidden="true" />
    <input
      id="bg-search-input"
      type="search"
      bind:value={searchQuery}
      placeholder="Pesquisar fundos e gradientes..."
      class="w-full h-8 pl-9 pr-8 bg-background border border-border rounded-lg text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary transition-colors"
    />
    {#if searchQuery}
      <button
        type="button"
        onclick={() => (searchQuery = '')}
        class="absolute right-2 top-1/2 -translate-y-1/2 w-5 h-5 rounded-full text-muted-foreground hover:text-foreground flex items-center justify-center cursor-pointer"
        aria-label="Limpar pesquisa"
      >
        <X size={12} aria-hidden="true" />
      </button>
    {/if}
  </div>

  <!-- Breadcrumb / Categoria Ativa -->
  <div class="flex items-center gap-1.5 text-xs text-muted-foreground shrink-0 select-none">
    <span>Fundos</span>
    <ChevronRight size={12} aria-hidden="true" />
    <span class="text-foreground font-semibold">
      {currentCategoryName}
    </span>
  </div>
</div>

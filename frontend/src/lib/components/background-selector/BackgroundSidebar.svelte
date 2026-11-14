<script lang="ts">
  import { Layers, Sparkles, Palette, Folder } from 'lucide-svelte';
  import Badge from '../ui/Badge.svelte';
  import type { BackgroundCategory } from './types';

  interface Props {
    selectedCategory: BackgroundCategory;
    totalCount: number;
    animatedCount: number;
    meshCount: number;
    gradientsCount: number;
    customCount: number;
    localCount: number;
    onselectcategory: (cat: BackgroundCategory) => void;
  }

  let {
    selectedCategory,
    totalCount,
    animatedCount,
    meshCount,
    gradientsCount,
    customCount,
    localCount,
    onselectcategory,
  }: Props = $props();
</script>

<aside
  class="w-56 border-r border-border bg-card/60 flex flex-col shrink-0 p-3 overflow-y-auto space-y-1 select-none"
  aria-label="Categorias de planos de fundo"
>
  <span class="text-xs font-bold uppercase tracking-wider text-muted-foreground px-2 py-1 block">
    Categorias
  </span>

  <button
    type="button"
    role="tab"
    aria-selected={selectedCategory === 'all'}
    onclick={() => onselectcategory('all')}
    class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg text-xs font-medium transition-colors cursor-pointer min-h-[32px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedCategory === 'all'
      ? 'bg-primary/10 text-primary font-bold shadow-xs'
      : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
  >
    <div class="flex items-center gap-2">
      <Layers size={15} aria-hidden="true" />
      <span>Todos</span>
    </div>
    <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">
      {totalCount}
    </Badge>
  </button>

  <button
    type="button"
    role="tab"
    aria-selected={selectedCategory === 'animated'}
    onclick={() => onselectcategory('animated')}
    class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg text-xs font-medium transition-colors cursor-pointer min-h-[32px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedCategory === 'animated'
      ? 'bg-primary/10 text-primary font-bold shadow-xs'
      : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
  >
    <div class="flex items-center gap-2">
      <Sparkles size={15} class="text-amber-500 dark:text-amber-400" aria-hidden="true" />
      <span>Animados (CSS)</span>
    </div>
    <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">
      {animatedCount}
    </Badge>
  </button>

  <button
    type="button"
    role="tab"
    aria-selected={selectedCategory === 'mesh'}
    onclick={() => onselectcategory('mesh')}
    class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg text-xs font-medium transition-colors cursor-pointer min-h-[32px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedCategory === 'mesh'
      ? 'bg-primary/10 text-primary font-bold shadow-xs'
      : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
  >
    <div class="flex items-center gap-2">
      <Palette size={15} class="text-fuchsia-500 dark:text-fuchsia-400" aria-hidden="true" />
      <span>Mesh Gradients</span>
    </div>
    <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">
      {meshCount}
    </Badge>
  </button>

  <button
    type="button"
    role="tab"
    aria-selected={selectedCategory === 'gradients'}
    onclick={() => onselectcategory('gradients')}
    class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg text-xs font-medium transition-colors cursor-pointer min-h-[32px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedCategory === 'gradients'
      ? 'bg-primary/10 text-primary font-bold shadow-xs'
      : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
  >
    <div class="flex items-center gap-2">
      <Palette size={15} class="text-sky-500 dark:text-sky-400" aria-hidden="true" />
      <span>Gradientes Lineares</span>
    </div>
    <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">
      {gradientsCount}
    </Badge>
  </button>

  {#if customCount > 0}
    <button
      type="button"
      role="tab"
      aria-selected={selectedCategory === 'custom'}
      onclick={() => onselectcategory('custom')}
      class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg text-xs font-medium transition-colors cursor-pointer min-h-[32px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedCategory === 'custom'
        ? 'bg-primary/10 text-primary font-bold shadow-xs'
        : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      <div class="flex items-center gap-2">
        <Sparkles size={15} class="text-emerald-500 dark:text-emerald-400" aria-hidden="true" />
        <span>Customizados</span>
      </div>
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">
        {customCount}
      </Badge>
    </button>
  {/if}

  <button
    type="button"
    role="tab"
    aria-selected={selectedCategory === 'local'}
    onclick={() => onselectcategory('local')}
    class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg text-xs font-medium transition-colors cursor-pointer min-h-[32px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedCategory === 'local'
      ? 'bg-primary/10 text-primary font-bold shadow-xs'
      : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
  >
    <div class="flex items-center gap-2">
      <Folder size={15} class="text-emerald-500 dark:text-emerald-400" aria-hidden="true" />
      <span>Papéis Locais</span>
    </div>
    <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">
      {localCount}
    </Badge>
  </button>
</aside>

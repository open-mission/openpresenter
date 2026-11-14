<script lang="ts">
  import { SearchX } from 'lucide-svelte';
  import { encodeBackground, type BackgroundPreset } from '../../backgrounds';
  import type { WallpaperItem } from '$lib/types';
  import type { BackgroundCategory } from './types';
  import BackgroundCard from './BackgroundCard.svelte';

  interface Props {
    selectedCategory: BackgroundCategory;
    filteredPresets: BackgroundPreset[];
    filteredLocalWallpapers: WallpaperItem[];
    currentValue: string;
    onselectpreset: (preset: BackgroundPreset) => void;
    onselectlocal: (wall: WallpaperItem) => void;
  }

  let {
    selectedCategory,
    filteredPresets,
    filteredLocalWallpapers,
    currentValue,
    onselectpreset,
    onselectlocal,
  }: Props = $props();

  const showPresets = $derived(selectedCategory !== 'local');
  const showLocals = $derived(selectedCategory === 'all' || selectedCategory === 'local');
  const totalVisible = $derived(
    (showPresets ? filteredPresets.length : 0) + (showLocals ? filteredLocalWallpapers.length : 0)
  );
</script>

<main class="flex-1 p-4 overflow-y-auto bg-background/50">
  {#if totalVisible === 0}
    <div class="h-full min-h-[240px] flex flex-col items-center justify-center text-center p-6 text-muted-foreground animate-in fade-in duration-150">
      <div
        class="w-12 h-12 rounded-2xl bg-muted/60 border border-border flex items-center justify-center mb-3"
        aria-hidden="true"
      >
        <SearchX size={24} class="text-muted-foreground" />
      </div>
      <h3 class="text-sm font-semibold text-foreground">Nenhum fundo encontrado</h3>
      <p class="text-xs text-muted-foreground mt-1 max-w-sm">
        Não encontramos resultados correspondentes à sua pesquisa nesta categoria. Tente usar outros termos ou selecione "Todos".
      </p>
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3.5">
      <!-- Presets em Código CSS -->
      {#if showPresets}
        {#each filteredPresets as preset (preset.id)}
          {@const encoded = encodeBackground(preset)}
          <BackgroundCard
            {preset}
            isSelected={currentValue === encoded}
            onselect={() => onselectpreset(preset)}
          />
        {/each}
      {/if}

      <!-- Papéis de Parede Locais / Imagens do Sistema -->
      {#if showLocals}
        {#each filteredLocalWallpapers as wall (wall.url)}
          <BackgroundCard
            localWallpaper={wall}
            isSelected={currentValue === wall.url}
            onselect={() => onselectlocal(wall)}
          />
        {/each}
      {/if}
    </div>
  {/if}
</main>

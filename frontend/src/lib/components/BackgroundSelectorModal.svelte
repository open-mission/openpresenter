<script lang="ts">
  import { onDestroy } from 'svelte';
  import { cn } from '$lib/utils';
  import { BackgroundSelectorController } from './background-selector/controller.svelte';
  import BackgroundHeader from './background-selector/BackgroundHeader.svelte';
  import BackgroundToolbar from './background-selector/BackgroundToolbar.svelte';
  import BackgroundSidebar from './background-selector/BackgroundSidebar.svelte';
  import BackgroundGrid from './background-selector/BackgroundGrid.svelte';
  import BackgroundFooter from './background-selector/BackgroundFooter.svelte';
  import type { BackgroundSelectorProps } from './background-selector/types';

  let { open = false, currentValue = '', onselect, onclose }: BackgroundSelectorProps = $props();

  const controller = new BackgroundSelectorController({
    onselect: (val) => onselect(val),
    onclose: () => onclose(),
  });

  $effect(() => {
    controller.syncOpen(open, currentValue);
  });

  onDestroy(() => {
    controller.destroy();
  });

  function handleKeydown(e: KeyboardEvent) {
    controller.handleKeydown(e);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm animate-in fade-in duration-150 {controller.isMaximized
      ? 'p-0'
      : 'p-4'}"
    role="presentation"
    onclick={() => controller.close()}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      bind:this={controller.dialogEl}
      class={cn(
        'bg-card text-card-foreground shadow-2xl flex flex-col overflow-hidden animate-in zoom-in-95 duration-150',
        controller.isMaximized
          ? 'w-full h-full max-w-none max-h-none rounded-none border-0'
          : 'w-full max-w-4xl h-[85vh] max-h-[720px] rounded-2xl border border-border'
      )}
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="bg-selector-title"
      aria-describedby="bg-selector-desc"
      tabindex="-1"
    >
      <!-- Cabeçalho -->
      <BackgroundHeader
        isMaximized={controller.isMaximized}
        ontogglemaximize={() => controller.toggleMaximize()}
        onclose={() => controller.close()}
      />

      <!-- Barra de Busca e Breadcrumb -->
      <BackgroundToolbar
        bind:searchQuery={controller.searchQuery}
        selectedCategory={controller.selectedCategory}
      />

      <!-- Corpo Principal (Sidebar + Grade) -->
      <div class="flex-1 flex overflow-hidden">
        <BackgroundSidebar
          selectedCategory={controller.selectedCategory}
          totalCount={controller.totalCount}
          animatedCount={controller.animatedCount}
          meshCount={controller.meshCount}
          gradientsCount={controller.gradientsCount}
          customCount={controller.customCount}
          localCount={controller.localCount}
          onselectcategory={(cat) => (controller.selectedCategory = cat)}
        />

        <BackgroundGrid
          selectedCategory={controller.selectedCategory}
          filteredPresets={controller.filteredPresets}
          filteredLocalWallpapers={controller.filteredLocalWallpapers}
          currentValue={controller.currentValue}
          onselectpreset={(p) => controller.selectPreset(p)}
          onselectlocal={(w) => controller.selectLocal(w)}
        />
      </div>

      <!-- Rodapé -->
      <BackgroundFooter onclose={() => controller.close()} />
    </div>
  </div>
{/if}

<script lang="ts">
  import { onDestroy } from 'svelte';
  import { cn } from '$lib/utils';
  import { ImageSelectorController } from './image-selector/controller.svelte';
  import ImageSelectorHeader from './image-selector/ImageSelectorHeader.svelte';
  import ImageSelectorModeTabs from './image-selector/ImageSelectorModeTabs.svelte';
  import ImageSelectorFolderFilter from './image-selector/ImageSelectorFolderFilter.svelte';
  import ImageSelectorGrid from './image-selector/ImageSelectorGrid.svelte';
  import ImageSelectorFolderList from './image-selector/ImageSelectorFolderList.svelte';
  import ImageSelectorFooter from './image-selector/ImageSelectorFooter.svelte';
  import type { ImageSelectorProps } from './image-selector/types';

  let {
    open = false,
    initialMode = 'single',
    title,
    subtitle,
    allowFolders = true,
    onclose,
    onadditems,
    onselectimage,
  }: ImageSelectorProps = $props();

  const controller = new ImageSelectorController({
    onclose: () => onclose(),
    onadditems: (items) => onadditems?.(items),
    onselectimage: (url) => onselectimage?.(url),
  });

  $effect(() => {
    controller.syncOpen(open, initialMode, allowFolders);
  });

  onDestroy(() => {
    controller.restoreFocus();
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
        'border-border bg-card shadow-2xl flex flex-col overflow-hidden text-card-foreground select-none animate-in zoom-in-95 duration-150',
        controller.isMaximized
          ? 'w-full h-full max-w-none max-h-none rounded-none border-0'
          : 'w-full max-w-3xl h-[80vh] max-h-[640px] rounded-2xl border'
      )}
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="image-selector-title"
      aria-describedby="image-selector-desc"
      tabindex="-1"
    >
      <!-- Cabeçalho -->
      <ImageSelectorHeader
        {title}
        {subtitle}
        isLowerThird={controller.isLowerThird}
        isMaximized={controller.isMaximized}
        ontogglemaximize={() => controller.toggleMaximize()}
        onclose={() => controller.close()}
      />

      <!-- Alternância de Abas e Barra de Busca -->
      <ImageSelectorModeTabs
        mode={controller.mode}
        {allowFolders}
        isLowerThird={controller.isLowerThird}
        bind:searchQuery={controller.searchQuery}
        onselectmode={(mode) => (controller.mode = mode)}
      />

      <!-- Conteúdo Principal -->
      <div class="flex-1 overflow-y-auto p-5 min-h-0">
        {#if controller.mode === 'single'}
          <ImageSelectorFolderFilter
            selectedFolderFilter={controller.selectedFolderFilter}
            folders={controller.folders}
            totalImagesCount={controller.images.length}
            appImagesCount={controller.appImagesCount}
            isLowerThird={controller.isLowerThird}
            isAllSelected={controller.isAllSelected}
            filteredCount={controller.filteredImages.length}
            onselectfolder={(id) => (controller.selectedFolderFilter = id)}
            onselectall={() => controller.selectAll()}
          />

          <ImageSelectorGrid
            images={controller.filteredImages}
            selectedImageIds={controller.selectedImageIds}
            isLowerThird={controller.isLowerThird}
            ontoggle={(id) => controller.toggleImageSelection(id)}
            ondoubleclick={(url) => {
              if (onselectimage) {
                onselectimage(url);
                controller.close();
              }
            }}
          />
        {:else}
          <ImageSelectorFolderList
            folders={controller.folders}
            images={controller.images}
            appImagesCount={controller.appImagesCount}
            selectedFolderId={controller.selectedFolderId}
            feedbackMessage={controller.feedbackMessage}
            onselectfolder={(id) => (controller.selectedFolderId = id)}
          />
        {/if}
      </div>

      <!-- Rodapé -->
      <ImageSelectorFooter
        mode={controller.mode}
        isLowerThird={controller.isLowerThird}
        selectedCount={controller.selectedCount}
        selectedFolderId={controller.selectedFolderId}
        onclose={() => controller.close()}
        onconfirm={() => {
          if (controller.mode === 'single') {
            controller.confirmSingle();
          } else {
            controller.confirmFolder();
          }
        }}
      />
    </div>
  </div>
{/if}

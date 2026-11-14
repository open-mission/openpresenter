<script lang="ts">
  import { cn } from '$lib/utils';
  import Button from './ui/Button.svelte';
  import BackgroundSelectorModal from './BackgroundSelectorModal.svelte';
  import { SetlistController } from './setlist-editor/controller.svelte';
  import SetlistHeader from './setlist-editor/SetlistHeader.svelte';
  import SetlistMetadata from './setlist-editor/SetlistMetadata.svelte';
  import SetlistWallpaperSelector from './setlist-editor/SetlistWallpaperSelector.svelte';
  import SetlistConfirmDiscard from './setlist-editor/SetlistConfirmDiscard.svelte';
  import type { SetlistModalProps } from './setlist-editor/types';

  let { open = false, initialSetlist = null, onclose, onsave }: SetlistModalProps = $props();

  const controller = new SetlistController({
    onclose: () => onclose(),
    onsave: (saved) => onsave(saved),
  });

  $effect(() => {
    controller.syncOpen(open, initialSetlist);
  });

  function handleKeydown(e: KeyboardEvent) {
    controller.handleKeydown(e);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm animate-in fade-in duration-150 {controller.isMaximized ? 'p-0' : 'p-4'}"
    role="presentation"
    onclick={() => controller.requestClose()}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      bind:this={controller.dialogEl}
      class={cn(
        'bg-card text-card-foreground shadow-2xl flex flex-col overflow-hidden transition-all duration-150',
        controller.isMaximized
          ? 'w-full h-full max-w-none max-h-none rounded-none border-0'
          : 'w-full max-w-2xl rounded-xl border border-border'
      )}
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="setlist-dialog-title"
      aria-describedby="setlist-dialog-desc"
      tabindex="-1"
      inert={controller.showDiscardConfirm || undefined}
    >
      <!-- Cabeçalho -->
      <SetlistHeader
        isEdit={controller.isEdit}
        isMaximized={controller.isMaximized}
        ontogglemaximize={() => controller.toggleMaximize()}
        onclose={() => controller.requestClose()}
      />

      <!-- Formulário / Conteúdo Principal -->
      <div
        class="p-6 space-y-5 overflow-y-auto flex-1 min-h-0 {controller.isMaximized ? '' : 'max-h-[75vh]'}"
      >
        <SetlistMetadata
          bind:title={controller.title}
          bind:description={controller.description}
          errorMessage={controller.errorMessage}
        />

        <SetlistWallpaperSelector
          selectedWallpaper={controller.selectedWallpaper}
          wallpaperMode={controller.wallpaperMode}
          resolvedWallpaper={controller.resolvedWallpaper}
          availableWallpapers={controller.availableWallpapers}
          isUploading={controller.isUploading}
          onopenbackgroundmodal={() => (controller.isBackgroundModalOpen = true)}
          onuploadfile={(e) => controller.handleFileUpload(e)}
          onselectwallpaper={(val) => (controller.selectedWallpaper = val)}
          onselectmode={(mode) => (controller.wallpaperMode = mode)}
        />
      </div>

      <!-- Rodapé -->
      <div
        class="h-16 px-6 border-t border-border bg-muted/40 flex items-center justify-end gap-2.5 shrink-0"
      >
        <Button
          variant="secondary"
          size="sm"
          onclick={() => controller.requestClose()}
          class="min-h-[32px] text-xs"
        >
          Cancelar
        </Button>
        <Button
          variant="default"
          size="sm"
          onclick={() => controller.save()}
          disabled={controller.isLoading || !controller.title.trim()}
          class="bg-emerald-600 hover:bg-emerald-500 text-white dark:bg-emerald-500 dark:hover:bg-emerald-400 dark:text-black font-semibold min-h-[32px] text-xs"
        >
          {controller.isLoading
            ? 'Salvando...'
            : controller.isEdit
              ? 'Atualizar Setlist'
              : 'Criar Setlist'}
        </Button>
      </div>
    </div>
  </div>

  <!-- Diálogo Acessível de Confirmação de Descarte -->
  <SetlistConfirmDiscard
    open={controller.showDiscardConfirm}
    onconfirm={() => controller.confirmDiscard()}
    oncancel={() => controller.cancelDiscard()}
  />

  <!-- Modal Secundário de Seleção de Backgrounds/Gradientes -->
  <BackgroundSelectorModal
    open={controller.isBackgroundModalOpen}
    currentValue={controller.selectedWallpaper}
    onselect={(val) => {
      controller.selectedWallpaper = val;
    }}
    onclose={() => {
      controller.isBackgroundModalOpen = false;
    }}
  />
{/if}

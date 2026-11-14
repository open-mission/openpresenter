<script lang="ts">
  import { onDestroy } from 'svelte';
  import { cn } from '$lib/utils';
  import { QuickItemController } from './quick-item/controller.svelte';
  import QuickItemHeader from './quick-item/QuickItemHeader.svelte';
  import QuickItemForm from './quick-item/QuickItemForm.svelte';
  import QuickItemFooter from './quick-item/QuickItemFooter.svelte';
  import QuickItemConfirmDiscard from './quick-item/QuickItemConfirmDiscard.svelte';
  import type { QuickItemModalProps } from './quick-item/types';

  let {
    open = false,
    mode = 'header',
    initialItem = null,
    onclose,
    onsave,
  }: QuickItemModalProps = $props();

  const controller = new QuickItemController({
    onclose: () => onclose(),
    onsave: (item) => onsave(item),
  });

  $effect(() => {
    controller.syncOpen(open, mode, initialItem);
  });

  onDestroy(() => {
    controller.restoreFocus();
  });
</script>

{#if open}
  <div
    role="presentation"
    class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-xs animate-in fade-in duration-150 {controller.isMaximized
      ? 'p-0'
      : 'p-4'}"
    onclick={() => controller.requestClose()}
  >
    <div
      bind:this={controller.dialogEl}
      role="dialog"
      aria-modal="true"
      aria-labelledby="quick-item-title"
      aria-describedby="quick-item-desc"
      tabindex="-1"
      onkeydown={(e) => controller.handleKeydown(e)}
      onclick={(e) => e.stopPropagation()}
      class={cn(
        'bg-card border border-border shadow-2xl flex flex-col overflow-hidden text-card-foreground transition-all duration-150 focus:outline-none',
        controller.isMaximized
          ? 'w-full h-full max-w-none max-h-none rounded-none border-0'
          : 'w-full max-w-md rounded-2xl max-h-[90vh]'
      )}
    >
      <!-- Cabeçalho -->
      <QuickItemHeader
        mode={controller.mode}
        isMaximized={controller.isMaximized}
        isEditing={controller.isEditing}
        ontogglemaximized={() => controller.toggleMaximized()}
        onrequestclose={() => controller.requestClose()}
      />

      <!-- Formulário de Edição -->
      <QuickItemForm
        mode={controller.mode}
        isMaximized={controller.isMaximized}
        title={controller.title}
        content={controller.content}
        slideCount={controller.slideCount}
        onupdatetitle={(val) => (controller.title = val)}
        onupdatecontent={(val) => (controller.content = val)}
      />

      <!-- Rodapé -->
      <QuickItemFooter
        canSave={controller.canSave}
        onrequestclose={() => controller.requestClose()}
        onsave={() => controller.save()}
      />
    </div>
  </div>

  <!-- Diálogo de confirmação de descarte de alterações -->
  <QuickItemConfirmDiscard
    open={controller.isDiscardOpen}
    onconfirm={() => controller.confirmDiscard()}
    oncancel={() => controller.cancelDiscard()}
  />
{/if}

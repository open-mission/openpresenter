<script lang="ts">
  import { Check, X } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import type { MediaItem } from '$lib/types';
  import { ScheduleEditorController } from './schedule-editor/controller.svelte';
  import ScheduleEditorHeader from './schedule-editor/ScheduleEditorHeader.svelte';
  import ScheduleEditorMetadata from './schedule-editor/ScheduleEditorMetadata.svelte';
  import ScheduleEditorPresets from './schedule-editor/ScheduleEditorPresets.svelte';
  import ScheduleEditorEventsList from './schedule-editor/ScheduleEditorEventsList.svelte';
  import ScheduleEditorPreview from './schedule-editor/ScheduleEditorPreview.svelte';
  import ScheduleEditorConfirmDiscard from './schedule-editor/ScheduleEditorConfirmDiscard.svelte';

  interface Props {
    open: boolean;
    initialItem?: MediaItem | null;
    onclose: () => void;
    onsave: (item: MediaItem) => void;
  }

  let { open = false, initialItem = null, onclose, onsave }: Props = $props();

  const controller = new ScheduleEditorController({
    close: () => onclose(),
    save: (item) => onsave(item),
  });

  $effect(() => {
    controller.syncOpen(open, initialItem);
  });
</script>

<svelte:window onkeydown={(event) => controller.handleKeydown(event)} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm animate-in fade-in duration-150 {controller.isMaximized
      ? 'p-0'
      : 'p-3 sm:p-5'}"
    role="presentation"
    onclick={() => controller.requestClose()}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      bind:this={controller.dialogEl}
      class="{controller.isMaximized
        ? 'w-full h-full max-w-none max-h-none rounded-none border-0'
        : 'w-full max-w-4xl max-h-[92vh] rounded-2xl border border-border'} bg-card shadow-2xl flex flex-col overflow-hidden text-foreground transition-all duration-150"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => controller.handleDialogKeydown(e)}
      role="dialog"
      aria-modal="true"
      aria-labelledby="schedule-editor-title"
      aria-describedby="schedule-editor-description"
      inert={controller.showDiscardConfirm}
      tabindex="-1"
    >
      <ScheduleEditorHeader
        isEdit={Boolean(initialItem)}
        eventCount={controller.events.length}
        isMaximized={controller.isMaximized}
        ontogglemaximize={() => (controller.isMaximized = !controller.isMaximized)}
        onclose={() => controller.requestClose()}
      />

      {#if controller.error}
        <div
          role="alert"
          class="px-6 py-2.5 bg-red-500/10 border-b border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-center justify-between shrink-0"
        >
          <span>{controller.error}</span>
          <button
            type="button"
            onclick={() => (controller.error = null)}
            class="w-6 h-6 rounded flex items-center justify-center text-red-700 dark:text-red-300 hover:text-foreground cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
            aria-label="Dispensar aviso de erro"
          >
            <X size={14} aria-hidden="true" />
          </button>
        </div>
      {/if}

      <div class="flex-1 min-h-0 overflow-y-auto p-6 grid grid-cols-1 lg:grid-cols-12 gap-6">
        <!-- Coluna da Esquerda: Formulário e Lista de Dias (7 colunas) -->
        <div class="lg:col-span-7 flex flex-col gap-4">
          <ScheduleEditorMetadata
            bind:title={controller.title}
            bind:artistOrRef={controller.artistOrRef}
          />

          <ScheduleEditorPresets
            onaddpreset={(p) => controller.addEvent(p.dayOrDate, p.time, p.title, p.details)}
          />

          <ScheduleEditorEventsList
            bind:events={controller.events}
            selectedIndex={controller.selectedEventIndex}
            onaddevent={() => controller.addEvent()}
            onselect={(idx) => controller.selectEvent(idx)}
            onmoveup={(idx) => controller.moveEvent(idx, 'up')}
            onmovedown={(idx) => controller.moveEvent(idx, 'down')}
            onremove={(idx) => controller.removeEvent(idx)}
          />
        </div>

        <!-- Coluna da Direita: Prévia Visual ao Vivo (5 colunas) -->
        <div class="lg:col-span-5 flex flex-col gap-4">
          <ScheduleEditorPreview
            events={controller.events}
            selectedIndex={controller.selectedEventIndex}
            activeEvent={controller.activePreviewEvent}
          />
        </div>
      </div>

      <!-- Rodapé com Botões de Ação -->
      <footer
        class="h-16 px-6 border-t border-border flex items-center justify-between shrink-0 bg-muted/40 select-none"
      >
        <span class="text-xs text-muted-foreground hidden sm:inline">
          Dica: Use <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-foreground font-mono text-[11px]">⌘Enter</kbd> / <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-foreground font-mono text-[11px]">Ctrl+Enter</kbd> para salvar
        </span>

        <div class="flex items-center gap-2.5 ml-auto">
          <Button
            variant="ghost"
            size="sm"
            onclick={() => controller.requestClose()}
            class="text-xs min-h-[36px]"
          >
            Cancelar
          </Button>
          <Button
            variant="default"
            size="sm"
            onclick={() => controller.handleSave()}
            class="bg-amber-600 hover:bg-amber-700 text-white font-bold text-xs px-4 min-h-[36px]"
          >
            <Check size={14} class="mr-1.5" aria-hidden="true" />
            <span>Salvar Agenda na Setlist</span>
          </Button>
        </div>
      </footer>
    </div>

    <ScheduleEditorConfirmDiscard
      open={controller.showDiscardConfirm}
      onconfirm={() => controller.confirmDiscard()}
      oncancel={() => controller.cancelDiscard()}
    />
  </div>
{/if}

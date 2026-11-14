<script lang="ts">
  import { onDestroy } from 'svelte';
  import { cn } from '$lib/utils';
  import { TimerEditorController } from './timer-editor/controller.svelte';
  import TimerEditorHeader from './timer-editor/TimerEditorHeader.svelte';
  import TimerDurationSelector from './timer-editor/TimerDurationSelector.svelte';
  import TimerSubtitleField from './timer-editor/TimerSubtitleField.svelte';
  import TimerAdvancedOptions from './timer-editor/TimerAdvancedOptions.svelte';
  import TimerEditorFooter from './timer-editor/TimerEditorFooter.svelte';
  import type { TimerEditorModalProps } from './timer-editor/types';

  let {
    open = false,
    initialItem = null,
    onclose,
    onsave,
    onaddtosetlist,
  }: TimerEditorModalProps = $props();

  const controller = new TimerEditorController({
    onclose: () => onclose(),
    onsave: (item) => onsave(item),
    onaddtosetlist: (item) => onaddtosetlist?.(item),
  });

  $effect(() => {
    controller.syncOpen(open, initialItem);
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
    onclick={() => controller.close()}
  >
    <div
      bind:this={controller.dialogEl}
      role="dialog"
      aria-modal="true"
      aria-labelledby="timer-editor-title"
      aria-describedby="timer-editor-desc"
      tabindex="-1"
      onkeydown={(e) => controller.handleKeydown(e)}
      onclick={(e) => e.stopPropagation()}
      class={cn(
        'bg-card border border-border shadow-2xl flex flex-col text-card-foreground select-none overflow-hidden transition-all duration-150 focus:outline-none',
        controller.isMaximized
          ? 'w-full h-full rounded-none border-0'
          : 'w-full max-w-xl rounded-2xl max-h-[90vh]'
      )}
    >
      <!-- Cabeçalho -->
      <TimerEditorHeader
        isMaximized={controller.isMaximized}
        isEditing={controller.isEditing}
        ontogglemaximized={() => controller.toggleMaximized()}
        onclose={() => controller.close()}
      />

      <!-- Conteúdo do Formulário -->
      <div class="p-6 space-y-5 overflow-y-auto flex-1">
        <!-- Título -->
        <div>
          <label
            for="timer-title"
            class="block text-xs font-bold text-muted-foreground uppercase tracking-wider mb-1.5"
          >
            Título / Identificação do Timer
          </label>
          <input
            id="timer-title"
            type="text"
            bind:value={controller.title}
            placeholder="Ex.: Contagem Regressiva — Início do Culto"
            class="w-full h-10 px-3.5 bg-background border border-border rounded-xl text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:border-amber-500 transition-colors"
          />
        </div>

        <!-- Seletor de Duração -->
        <TimerDurationSelector
          formattedTime={controller.formattedDisplayTime}
          minutes={controller.minutes}
          seconds={controller.seconds}
          onselectpreset={(m, s) => controller.applyPreset(m, s)}
          onupdateminutes={(m) => controller.updateMinutes(m)}
          onupdateseconds={(s) => controller.updateSeconds(s)}
        />

        <!-- Subtítulo / Mensagem de Apoio -->
        <TimerSubtitleField
          subtitle={controller.subtitle}
          onupdatesubtitle={(val) => (controller.subtitle = val)}
        />

        <!-- Opções Avançadas -->
        <TimerAdvancedOptions
          allowOvertime={controller.allowOvertime}
          ontoggleovertime={() => controller.toggleOvertime()}
        />
      </div>

      <!-- Rodapé com Ações -->
      <TimerEditorFooter
        hasAddToSetlist={Boolean(onaddtosetlist)}
        oncancel={() => controller.close()}
        onsave={() => controller.saveOnly()}
        onsaveandadd={() => controller.saveAndAddToSetlist()}
      />
    </div>
  </div>
{/if}

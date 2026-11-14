<script lang="ts">
  import { Hourglass, Maximize2, Minimize2, X } from 'lucide-svelte';

  interface Props {
    isMaximized: boolean;
    isEditing: boolean;
    ontogglemaximized: () => void;
    onclose: () => void;
  }

  let { isMaximized, isEditing, ontogglemaximized, onclose }: Props = $props();
</script>

<div class="h-14 px-6 border-b border-border flex items-center justify-between shrink-0 bg-card/60">
  <div class="flex items-center gap-2.5">
    <div
      class="w-8 h-8 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-600 dark:text-amber-400 flex items-center justify-center shrink-0"
      aria-hidden="true"
    >
      <Hourglass size={16} />
    </div>
    <div>
      <h2 id="timer-editor-title" class="text-sm font-bold text-foreground flex items-center gap-2">
        <span>{isEditing ? 'Editar Cronômetro' : 'Novo Cronômetro / Timer'}</span>
        <span class="text-xs font-semibold bg-amber-500/10 text-amber-600 dark:text-amber-400 border border-amber-500/25 px-1.5 py-0.5 rounded">
          Tela Pública + Palco
        </span>
      </h2>
      <p id="timer-editor-desc" class="text-xs text-muted-foreground">
        Projeta contagem regressiva em destaque no telão sincronizada com o stage
      </p>
    </div>
  </div>

  <div class="flex items-center gap-1">
    <button
      type="button"
      onclick={ontogglemaximized}
      class="w-8 h-8 min-w-[32px] min-h-[32px] rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      aria-label={isMaximized ? 'Restaurar janela' : 'Maximizar janela'}
      title={isMaximized ? 'Restaurar janela' : 'Maximizar'}
    >
      {#if isMaximized}
        <Minimize2 size={15} aria-hidden="true" />
      {:else}
        <Maximize2 size={15} aria-hidden="true" />
      {/if}
    </button>
    <button
      type="button"
      onclick={onclose}
      class="w-8 h-8 min-w-[32px] min-h-[32px] rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      aria-label="Fechar (Esc)"
      title="Fechar (Esc)"
    >
      <X size={16} aria-hidden="true" />
    </button>
  </div>
</div>

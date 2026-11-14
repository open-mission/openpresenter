<script lang="ts">
  import { Sparkles, Maximize2, Minimize2, X } from 'lucide-svelte';

  interface Props {
    isEdit: boolean;
    isMaximized: boolean;
    ontogglemaximize: () => void;
    onclose: () => void;
  }

  let { isEdit, isMaximized, ontogglemaximize, onclose }: Props = $props();
</script>

<div class="h-16 px-6 border-b border-border flex items-center justify-between shrink-0 bg-card select-none">
  <div class="flex items-center gap-3 min-w-0">
    <div
      class="w-10 h-10 rounded-xl bg-emerald-500/15 border border-emerald-600/35 text-emerald-700 dark:text-emerald-300 flex items-center justify-center shrink-0"
      aria-hidden="true"
    >
      <Sparkles size={20} />
    </div>
    <div class="min-w-0">
      <h2 id="setlist-dialog-title" class="text-sm sm:text-base font-bold text-foreground truncate">
        {isEdit ? 'Editar Setlist' : 'Nova Setlist de Programação'}
      </h2>
      <p id="setlist-dialog-desc" class="text-xs text-muted-foreground truncate">
        Configure os detalhes e o papel de parede para a tela do projetor
      </p>
    </div>
  </div>

  <div class="flex items-center gap-1.5 shrink-0">
    <span class="text-[11px] text-muted-foreground hidden md:inline mr-1">
      Salvar:
      <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-muted-foreground font-mono text-[11px]">⌘Enter</kbd>
      /
      <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-muted-foreground font-mono text-[11px]">Ctrl+Enter</kbd>
    </span>

    <button
      type="button"
      onclick={ontogglemaximize}
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
      aria-label={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
    >
      {#if isMaximized}
        <Minimize2 size={16} aria-hidden="true" />
      {:else}
        <Maximize2 size={16} aria-hidden="true" />
      {/if}
    </button>

    <button
      type="button"
      onclick={onclose}
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title="Fechar (Esc)"
      aria-label="Fechar editor de setlist"
    >
      <X size={18} aria-hidden="true" />
    </button>
  </div>
</div>

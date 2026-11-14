<script lang="ts">
  import { Palette, Maximize2, Minimize2, X } from 'lucide-svelte';

  interface Props {
    isMaximized: boolean;
    ontogglemaximize: () => void;
    onclose: () => void;
  }

  let { isMaximized, ontogglemaximize, onclose }: Props = $props();
</script>

<div class="px-5 py-3.5 border-b border-border flex items-center justify-between bg-card shrink-0 select-none">
  <div class="flex items-center gap-3 min-w-0">
    <div
      class="w-9 h-9 rounded-xl bg-primary/10 border border-primary/20 text-primary flex items-center justify-center shrink-0"
      aria-hidden="true"
    >
      <Palette size={18} />
    </div>
    <div class="min-w-0">
      <h2 id="bg-selector-title" class="text-sm sm:text-base font-bold text-foreground truncate">
        Biblioteca de Fundos para Projeção
      </h2>
      <p id="bg-selector-desc" class="text-xs text-muted-foreground truncate">
        Escolha gradientes elegantes, mesh gradients procedurais ou animações CSS
      </p>
    </div>
  </div>

  <div class="flex items-center gap-1.5 shrink-0">
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
      aria-label="Fechar seletor de fundos"
    >
      <X size={18} aria-hidden="true" />
    </button>
  </div>
</div>

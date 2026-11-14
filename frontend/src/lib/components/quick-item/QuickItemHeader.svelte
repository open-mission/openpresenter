<script lang="ts">
  import { Bookmark, FileText, Maximize2, Minimize2, X } from 'lucide-svelte';
  import { QUICK_ITEM_CONFIG } from './defaults';
  import type { QuickItemMode } from './types';

  interface Props {
    mode: QuickItemMode;
    isMaximized: boolean;
    isEditing: boolean;
    ontogglemaximized: () => void;
    onrequestclose: () => void;
  }

  let { mode, isMaximized, isEditing, ontogglemaximized, onrequestclose }: Props = $props();

  const config = $derived(QUICK_ITEM_CONFIG[mode]);
</script>

<div class="h-14 px-5 border-b border-border flex items-center justify-between shrink-0 bg-card/60">
  <div class="flex items-center gap-2.5">
    <div
      class="w-8 h-8 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 dark:text-emerald-400 flex items-center justify-center shrink-0"
      aria-hidden="true"
    >
      {#if mode === 'header'}
        <Bookmark size={16} />
      {:else}
        <FileText size={16} />
      {/if}
    </div>
    <div>
      <h2 id="quick-item-title" class="text-sm font-bold text-foreground">
        {isEditing ? config.titleEdit : config.titleNew}
      </h2>
      <p id="quick-item-desc" class="text-xs text-muted-foreground">
        {config.description}
      </p>
    </div>
  </div>

  <div class="flex items-center gap-1.5">
    <button
      type="button"
      onclick={ontogglemaximized}
      class="w-8 h-8 min-w-[32px] min-h-[32px] rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
      aria-label={isMaximized ? 'Restaurar tamanho da janela' : 'Maximizar janela'}
    >
      {#if isMaximized}
        <Minimize2 size={15} aria-hidden="true" />
      {:else}
        <Maximize2 size={15} aria-hidden="true" />
      {/if}
    </button>

    <button
      type="button"
      onclick={onrequestclose}
      class="w-8 h-8 min-w-[32px] min-h-[32px] rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title="Fechar (Esc)"
      aria-label="Fechar (Esc)"
    >
      <X size={16} aria-hidden="true" />
    </button>
  </div>
</div>

<script lang="ts">
  import { Check, Plus } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import type { ImageSelectorMode } from './types';

  interface Props {
    mode: ImageSelectorMode;
    isLowerThird: boolean;
    selectedCount: number;
    selectedFolderId: string;
    onclose: () => void;
    onconfirm: () => void;
  }

  let {
    mode,
    isLowerThird,
    selectedCount,
    selectedFolderId,
    onclose,
    onconfirm,
  }: Props = $props();

  const isConfirmDisabled = $derived(
    mode === 'single' ? selectedCount === 0 : !selectedFolderId
  );
</script>

<div class="h-16 px-5 border-t border-border bg-card/60 flex items-center justify-between shrink-0 select-none">
  <div class="text-xs text-muted-foreground flex items-center gap-2">
    {#if isLowerThird}
      <span>{selectedCount > 0 ? '1 imagem selecionada (ou clique duplo)' : 'Selecione uma imagem'}</span>
    {:else if mode === 'single'}
      <span>{selectedCount} imagem(ns) selecionada(s)</span>
    {:else}
      <span>Inserir pasta como apresentação de slides</span>
    {/if}

    <span class="text-[11px] text-muted-foreground hidden md:inline ml-2 border-l border-border pl-2">
      Confirmar:
      <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-muted-foreground font-mono text-[11px]">⌘Enter</kbd>
    </span>
  </div>

  <div class="flex items-center gap-2">
    <Button variant="secondary" size="sm" onclick={onclose} class="min-h-[32px] text-xs">
      Cancelar
    </Button>

    {#if isLowerThird}
      <Button
        variant="default"
        size="sm"
        onclick={onconfirm}
        disabled={isConfirmDisabled}
        class="gap-1.5 min-h-[32px] text-xs"
      >
        <Check size={14} aria-hidden="true" />
        <span>Selecionar Imagem</span>
      </Button>
    {:else if mode === 'single'}
      <Button
        variant="default"
        size="sm"
        onclick={onconfirm}
        disabled={isConfirmDisabled}
        class="gap-1.5 min-h-[32px] text-xs"
      >
        <Plus size={14} aria-hidden="true" />
        <span>Adicionar ({selectedCount}) à Setlist</span>
      </Button>
    {:else}
      <Button
        variant="default"
        size="sm"
        onclick={onconfirm}
        disabled={isConfirmDisabled}
        class="gap-1.5 min-h-[32px] text-xs"
      >
        <Plus size={14} aria-hidden="true" />
        <span>Adicionar Pasta à Setlist</span>
      </Button>
    {/if}
  </div>
</div>

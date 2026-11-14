<script lang="ts">
  import { ListPlus } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';

  interface Props {
    hasAddToSetlist: boolean;
    oncancel: () => void;
    onsave: () => void;
    onsaveandadd?: () => void;
  }

  let { hasAddToSetlist, oncancel, onsave, onsaveandadd }: Props = $props();
</script>

<div class="h-16 px-6 border-t border-border bg-card/60 flex items-center justify-between shrink-0">
  <Button variant="ghost" size="sm" onclick={oncancel} class="text-xs cursor-pointer">
    Cancelar
  </Button>

  <div class="flex items-center gap-2">
    <Button
      variant="secondary"
      size="sm"
      onclick={onsave}
      class="text-xs cursor-pointer"
      title="Salvar apenas na biblioteca de mídias"
    >
      Salvar na Biblioteca
    </Button>

    {#if hasAddToSetlist && onsaveandadd}
      <Button
        variant="default"
        size="sm"
        onclick={onsaveandadd}
        class="text-xs bg-amber-500 hover:bg-amber-400 text-black font-bold gap-1.5 shadow-xs cursor-pointer"
        title="Salvar e adicionar imediatamente à setlist ativa (⌘Enter)"
      >
        <ListPlus size={14} aria-hidden="true" />
        <span>Salvar e Adicionar à Setlist</span>
        <kbd class="hidden sm:inline-block ml-1 px-1 py-0.5 text-[10px] bg-black/20 text-black rounded font-mono font-semibold">
          ⌘↵
        </kbd>
      </Button>
    {/if}
  </div>
</div>

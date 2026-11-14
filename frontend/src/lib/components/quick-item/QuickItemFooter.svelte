<script lang="ts">
  import Button from '../ui/Button.svelte';

  interface Props {
    canSave: boolean;
    onrequestclose: () => void;
    onsave: () => void;
  }

  let { canSave, onrequestclose, onsave }: Props = $props();
</script>

<div class="h-14 px-5 border-t border-border bg-card/60 flex items-center justify-end gap-2 shrink-0">
  <Button variant="ghost" size="sm" onclick={onrequestclose} class="text-xs cursor-pointer">
    Cancelar
  </Button>
  <Button
    variant="default"
    size="sm"
    onclick={onsave}
    disabled={!canSave}
    class="text-xs bg-emerald-600 hover:bg-emerald-500 text-white font-semibold gap-1.5 cursor-pointer shadow-xs focus-visible:ring-2 focus-visible:ring-ring"
    title="Salvar item (⌘Enter)"
  >
    <span>Salvar</span>
    <kbd class="hidden sm:inline-block ml-1 px-1 py-0.5 text-[10px] bg-black/20 text-white rounded font-mono font-semibold">
      ⌘↵
    </kbd>
  </Button>
</div>

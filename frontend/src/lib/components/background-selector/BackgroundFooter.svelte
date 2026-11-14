<script lang="ts">
  import Button from '../ui/Button.svelte';

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();
</script>

<div class="px-5 py-3 border-t border-border bg-card flex items-center justify-between shrink-0 select-none">
  <span class="text-xs text-muted-foreground">
    Ao selecionar, o plano de fundo é configurado imediatamente.
  </span>
  <Button variant="outline" size="sm" onclick={onclose} class="text-xs min-h-[32px]">
    Fechar
  </Button>
</div>

<script lang="ts">
  import RemoteDialog from './RemoteDialog.svelte';
  import { triggerHaptic } from '$lib/remoteData';

  interface Props {
    open: boolean;
    onConfirmStop: () => void;
    onClose: () => void;
  }

  let { open, onConfirmStop, onClose }: Props = $props();

  function handleStop() {
    triggerHaptic('warning');
    onConfirmStop();
  }
</script>

<RemoteDialog
  {open}
  role="alertdialog"
  title="Parar projeção ao vivo?"
  description="Esta ação remove imediatamente o slide atual da tela pública da igreja."
  closeLabel="Fechar confirmação de parada"
  onclose={onClose}
>
  <div class="p-4 flex items-center gap-2">
    <button
      type="button"
      data-autofocus
      onclick={onClose}
      class="flex-1 min-h-11 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold transition-colors cursor-pointer"
    >
      Cancelar
    </button>
    <button
      type="button"
      onclick={handleStop}
      class="flex-1 min-h-11 rounded-xl bg-red-700 hover:bg-red-800 text-white text-xs font-bold transition-colors cursor-pointer"
    >
      Parar Projeção
    </button>
  </div>
</RemoteDialog>

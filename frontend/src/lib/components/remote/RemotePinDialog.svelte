<script lang="ts">
  import { AlertTriangle, Eye, EyeOff, Loader2, ShieldCheck } from 'lucide-svelte';
  import RemoteDialog from './RemoteDialog.svelte';
  import { triggerHaptic } from '$lib/remoteData';

  interface Props {
    open: boolean;
    inputPin: string;
    errorMessage: string;
    isVerifying: boolean;
    onPinChange: (value: string) => void;
    onSubmit: () => void;
    onClose: () => void;
  }

  let {
    open,
    inputPin,
    errorMessage,
    isVerifying,
    onPinChange,
    onSubmit,
    onClose,
  }: Props = $props();

  let showPin = $state(false);

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    triggerHaptic('light');
    onSubmit();
  }
</script>

<RemoteDialog
  {open}
  title="Liberar Controle ao Vivo"
  description="Insira o PIN de operador configurado no OpenPresenter para enviar slides e controlar a projeção neste aparelho."
  closeLabel="Fechar modal de PIN"
  onclose={onClose}
>
  <form onsubmit={handleSubmit} class="p-4 space-y-4">
    <div class="flex items-center gap-2 text-xs text-emerald-800 dark:text-emerald-300 bg-emerald-500/15 border border-emerald-600/35 dark:border-emerald-500/40 rounded-xl p-3">
      <ShieldCheck size={16} class="shrink-0" aria-hidden="true" />
      <span>Solicite o PIN ao líder de mídia ou operador titular da mesa.</span>
    </div>

    <div class="space-y-1.5">
      <label for="remote-unlock-pin" class="block text-xs font-semibold text-foreground">
        PIN do Operador
      </label>
      <div class="relative">
        <input
          id="remote-unlock-pin"
          data-autofocus
          type={showPin ? 'text' : 'password'}
          inputmode="numeric"
          value={inputPin}
          oninput={(e) => onPinChange(e.currentTarget.value)}
          placeholder="Digite o PIN numérico"
          autocomplete="one-time-code"
          class="w-full h-12 bg-background border border-border rounded-xl px-4 pr-12 text-center text-base sm:text-lg tracking-widest text-foreground font-mono placeholder:text-xs placeholder:tracking-normal focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring"
        />
        <button
          type="button"
          onclick={() => {
            showPin = !showPin;
            triggerHaptic('light');
          }}
          aria-label={showPin ? 'Ocultar PIN de operador' : 'Mostrar PIN de operador'}
          class="h-10 w-10 rounded-lg absolute right-1 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
        >
          {#if showPin}
            <EyeOff size={18} aria-hidden="true" />
          {:else}
            <Eye size={18} aria-hidden="true" />
          {/if}
        </button>
      </div>

      {#if errorMessage}
        <p
          role="alert"
          class="text-xs text-red-700 dark:text-red-300 flex items-center gap-1.5 pt-1 font-medium"
        >
          <AlertTriangle size={14} aria-hidden="true" />
          <span>{errorMessage}</span>
        </p>
      {/if}
    </div>

    <div class="flex items-center gap-2 pt-1">
      <button
        type="button"
        onclick={onClose}
        class="flex-1 min-h-11 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold transition-colors cursor-pointer"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={isVerifying}
        class="flex-1 min-h-11 rounded-xl bg-emerald-700 hover:bg-emerald-800 disabled:opacity-50 text-white text-xs font-bold flex items-center justify-center gap-2 transition-colors cursor-pointer"
      >
        {#if isVerifying}
          <Loader2 size={15} class="animate-spin" aria-hidden="true" />
        {/if}
        <span>Desbloquear</span>
      </button>
    </div>
  </form>
</RemoteDialog>

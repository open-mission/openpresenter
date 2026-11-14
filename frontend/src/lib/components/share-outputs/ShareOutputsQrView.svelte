<script lang="ts">
  import { Wifi, Copy, Check, RefreshCw } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';

  interface Props {
    qrCodeDataUrl: string;
    description: string;
    activeUrl: string;
    isCopied: boolean;
    oncopy: () => void;
  }

  let { qrCodeDataUrl, description, activeUrl, isCopied, oncopy }: Props = $props();
</script>

<div class="p-6 flex flex-col items-center text-center space-y-4">
  <!-- QR Code renderizado -->
  <div class="p-3 bg-white rounded-2xl shadow-xl flex items-center justify-center border border-border/40">
    {#if qrCodeDataUrl}
      <img
        src={qrCodeDataUrl}
        alt="QR Code da Saída Selecionada"
        class="w-52 h-52 object-contain rounded-lg"
      />
    {:else}
      <div class="w-52 h-52 flex flex-col items-center justify-center text-muted-foreground text-xs gap-2">
        <RefreshCw size={22} class="animate-spin text-primary" aria-hidden="true" />
        <span>Gerando QR Code...</span>
      </div>
    {/if}
  </div>

  <!-- Instruções -->
  <div class="space-y-1.5 max-w-sm">
    <div class="flex items-center justify-center gap-1.5 text-xs text-foreground font-semibold">
      <Wifi size={14} class="text-emerald-600 dark:text-emerald-400" aria-hidden="true" />
      <span>Conecte à mesma rede Wi-Fi e aponte a câmera</span>
    </div>
    <p class="text-xs text-muted-foreground leading-relaxed">
      {description}
    </p>
  </div>

  <!-- Caixa com a URL e botão de Copiar -->
  <div class="w-full flex items-center gap-2 p-2 rounded-xl bg-muted/40 border border-border text-left">
    <div
      class="flex-1 min-w-0 px-2 font-mono text-xs text-foreground truncate select-all"
      title={activeUrl}
    >
      {activeUrl}
    </div>
    <Button
      variant="secondary"
      size="sm"
      onclick={oncopy}
      class="text-xs min-h-[32px] px-3 shrink-0 gap-1.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title="Copiar link para a área de transferência"
      aria-label="Copiar link da saída"
    >
      {#if isCopied}
        <Check size={14} class="text-emerald-600 dark:text-emerald-400" aria-hidden="true" />
        <span class="text-emerald-600 dark:text-emerald-400 font-semibold">Copiado</span>
      {:else}
        <Copy size={14} aria-hidden="true" />
        <span>Copiar</span>
      {/if}
    </Button>
  </div>

  <!-- Live Region para leitores de tela -->
  <div aria-live="polite" class="sr-only">
    {isCopied ? 'Link copiado para a área de transferência com sucesso.' : ''}
  </div>
</div>

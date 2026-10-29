<script lang="ts">
  import { onMount } from 'svelte';
  import { X, QrCode, Tv, MonitorPlay, Copy, Check, ExternalLink, Wifi, RefreshCw } from 'lucide-svelte';
  import QRCode from 'qrcode';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import type { NetworkInfoResponse } from '$lib/types';

  let {
    open = false,
    onclose,
  }: {
    open: boolean;
    onclose: () => void;
  } = $props();

  let activeTab = $state<'stage' | 'public'>('stage');
  let networkInfo = $state<NetworkInfoResponse | null>(null);
  let isLoading = $state(false);
  let qrCodeDataUrl = $state<string>('');
  let isCopied = $state(false);

  const activeUrl = $derived.by(() => {
    if (!networkInfo) return '';
    return activeTab === 'stage' ? networkInfo.stage_url : networkInfo.public_url;
  });

  $effect(() => {
    if (open) {
      loadNetworkInfo();
    }
  });

  $effect(() => {
    if (activeUrl) {
      generateQrCode(activeUrl);
    }
  });

  async function loadNetworkInfo() {
    isLoading = true;
    try {
      const res = await fetch('/api/system/network-info');
      if (res.ok) {
        networkInfo = await res.json();
      } else {
        // Fallback local se o endpoint falhar
        const origin = window.location.origin;
        networkInfo = {
          local_ip: window.location.hostname,
          port: Number(window.location.port) || 3000,
          public_url: `${origin}/public`,
          stage_url: `${origin}/stage`,
        };
      }
    } catch {
      const origin = window.location.origin;
      networkInfo = {
        local_ip: window.location.hostname,
        port: Number(window.location.port) || 3000,
        public_url: `${origin}/public`,
        stage_url: `${origin}/stage`,
      };
    } finally {
      isLoading = false;
    }
  }

  async function generateQrCode(text: string) {
    try {
      qrCodeDataUrl = await QRCode.toDataURL(text, {
        width: 320,
        margin: 2,
        color: {
          dark: '#000000',
          light: '#ffffff',
        },
      });
    } catch (err) {
      console.error('Erro ao gerar QR Code:', err);
    }
  }

  async function handleCopy() {
    if (!activeUrl) return;
    try {
      await navigator.clipboard.writeText(activeUrl);
      isCopied = true;
      setTimeout(() => (isCopied = false), 2000);
    } catch (err) {
      console.error('Falha ao copiar:', err);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) {
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
    aria-labelledby="share-outputs-title"
  >
    <div
      class="bg-neutral-900 border border-neutral-800 rounded-2xl w-full max-w-md flex flex-col shadow-2xl overflow-hidden"
    >
      <!-- Cabeçalho -->
      <div class="px-5 py-3.5 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-950/60">
        <div class="flex items-center gap-2">
          <div class="p-1.5 rounded-lg bg-primary/10 text-primary">
            <QrCode size={18} />
          </div>
          <div>
            <h3 id="share-outputs-title" class="text-sm font-bold text-white">
              Compartilhar Saídas na Rede Local
            </h3>
            <p class="text-[11px] text-neutral-400 mt-0.5">
              Abra o retorno ou projeção em tablets e celulares
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="p-1.5 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
          aria-label="Fechar"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Seletor de Saída (Abas) -->
      <div class="p-4 pb-2 border-b border-neutral-800/80 bg-neutral-900/40">
        <div class="grid grid-cols-2 gap-1.5 p-1 bg-neutral-950 rounded-xl border border-neutral-800">
          <button
            type="button"
            onclick={() => (activeTab = 'stage')}
            class="flex items-center justify-center gap-2 py-2 rounded-lg text-xs font-semibold transition-all cursor-pointer {activeTab === 'stage' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-neutral-400 hover:text-white'}"
          >
            <Tv size={14} />
            <span>Monitor de Palco (/stage)</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'public')}
            class="flex items-center justify-center gap-2 py-2 rounded-lg text-xs font-semibold transition-all cursor-pointer {activeTab === 'public' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-neutral-400 hover:text-white'}"
          >
            <MonitorPlay size={14} />
            <span>Tela Pública (/public)</span>
          </button>
        </div>
      </div>

      <!-- Conteúdo com QR Code e URL -->
      <div class="p-6 flex flex-col items-center text-center space-y-4">
        <!-- QR Code renderizado -->
        <div class="p-3 bg-white rounded-2xl shadow-xl flex items-center justify-center">
          {#if qrCodeDataUrl}
            <img
              src={qrCodeDataUrl}
              alt="QR Code da Saída"
              class="w-56 h-56 object-contain rounded-lg"
            />
          {:else}
            <div class="w-56 h-56 flex flex-col items-center justify-center text-neutral-400 text-xs gap-2">
              <RefreshCw size={20} class="animate-spin text-primary" />
              <span>Gerando QR Code...</span>
            </div>
          {/if}
        </div>

        <!-- Instruções -->
        <div class="space-y-1">
          <div class="flex items-center justify-center gap-1.5 text-xs text-neutral-300 font-medium">
            <Wifi size={13} class="text-emerald-400" />
            <span>Conecte à mesma rede Wi-Fi e aponte a câmera</span>
          </div>
          <p class="text-[11px] text-neutral-400 max-w-xs">
            {activeTab === 'stage'
              ? 'Ideal para músicos, pregadores e equipe no palco visualizarem letras e timer em tempo real.'
              : 'Ideal para monitores auxiliares, salas de transmissão ou telões secundários.'}
          </p>
        </div>

        <!-- Caixa com a URL e botão de Copiar -->
        <div class="w-full flex items-center gap-2 p-2 rounded-xl bg-neutral-950 border border-neutral-800 text-left">
          <div class="flex-1 min-w-0 px-2 font-mono text-xs text-neutral-200 truncate select-all">
            {activeUrl}
          </div>
          <Button
            variant="secondary"
            size="sm"
            onclick={handleCopy}
            class="text-xs h-8 px-3 shrink-0 gap-1.5"
            title="Copiar link para a área de transferência"
          >
            {#if isCopied}
              <Check size={13} class="text-emerald-400" />
              <span class="text-emerald-400 font-semibold">Copiado</span>
            {:else}
              <Copy size={13} />
              <span>Copiar</span>
            {/if}
          </Button>
        </div>
      </div>

      <!-- Rodapé -->
      <div class="px-5 py-3 bg-neutral-950/60 border-t border-neutral-800 flex items-center justify-between text-xs text-neutral-400">
        <span class="text-[11px]">IP da máquina: <strong class="text-neutral-200 font-mono">{networkInfo?.local_ip || '127.0.0.1'}</strong></span>
        <button
          type="button"
          onclick={() => window.open(activeUrl, '_blank')}
          class="text-xs text-primary hover:underline flex items-center gap-1 cursor-pointer font-medium"
        >
          <span>Abrir link</span>
          <ExternalLink size={12} />
        </button>
      </div>
    </div>
  </div>
{/if}

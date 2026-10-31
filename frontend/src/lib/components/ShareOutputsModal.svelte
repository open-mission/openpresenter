<script lang="ts">
  import { onMount } from 'svelte';
  import { X, QrCode, Tv, MonitorPlay, Copy, Check, ExternalLink, Wifi, RefreshCw, Cast, Smartphone, Video } from 'lucide-svelte';
  import QRCode from 'qrcode';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import CastControl from './CastControl.svelte';
  import type { NetworkInfoResponse } from '$lib/types';

  let {
    open = false,
    initialTab = 'stage',
    onclose,
  }: {
    open: boolean;
    initialTab?: 'stage' | 'public' | 'remote' | 'lower' | 'cast';
    onclose: () => void;
  } = $props();

  let activeTab = $state<'stage' | 'public' | 'remote' | 'lower' | 'cast'>('stage');
  let networkInfo = $state<NetworkInfoResponse | null>(null);
  let isLoading = $state(false);
  let qrCodeDataUrl = $state<string>('');
  let isCopied = $state(false);

  const activeUrl = $derived.by(() => {
    if (!networkInfo) return '';
    if (activeTab === 'public') return networkInfo.public_url;
    if (activeTab === 'remote') return networkInfo.remote_url;
    if (activeTab === 'lower') return networkInfo.lower_third_url;
    return networkInfo.stage_url;
  });

  $effect(() => {
    if (open) {
      if (initialTab) {
        activeTab = initialTab;
      }
      loadNetworkInfo();
    }
  });

  $effect(() => {
    if (activeUrl && activeTab !== 'cast') {
      generateQrCode(activeUrl);
    }
  });

  async function loadNetworkInfo() {
    isLoading = true;
    try {
      const res = await fetch('/api/system/network-info');
      if (res.ok) {
        networkInfo = await res.json();
        // Se a API retornou localhost/127.0.0.1 mas o operador está acessando por IP de rede
        if (
          (networkInfo.local_ip === '127.0.0.1' || networkInfo.local_ip === '0.0.0.0') &&
          window.location.hostname !== 'localhost' &&
          window.location.hostname !== '127.0.0.1'
        ) {
          networkInfo.local_ip = window.location.hostname;
          const p = networkInfo.port || 18080;
          networkInfo.public_url = `http://${networkInfo.local_ip}:${p}/public`;
          networkInfo.stage_url = `http://${networkInfo.local_ip}:${p}/stage`;
          networkInfo.remote_url = `http://${networkInfo.local_ip}:${p}/remote`;
          networkInfo.lower_third_url = `http://${networkInfo.local_ip}:${p}/lower-third`;
        }
      } else {
        const origin = window.location.origin;
        networkInfo = {
          local_ip: window.location.hostname,
          port: Number(window.location.port) || 3000,
          public_url: `${origin}/public`,
          stage_url: `${origin}/stage`,
          remote_url: `${origin}/remote`,
          lower_third_url: `${origin}/lower-third`,
        };
      }
    } catch {
      const origin = window.location.origin;
      networkInfo = {
        local_ip: window.location.hostname,
        port: Number(window.location.port) || 3000,
        public_url: `${origin}/public`,
        stage_url: `${origin}/stage`,
        remote_url: `${origin}/remote`,
        lower_third_url: `${origin}/lower-third`,
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
      class="bg-neutral-900 border border-neutral-800 rounded-2xl w-full max-w-lg flex flex-col shadow-2xl overflow-hidden max-h-[90vh]"
    >
      <!-- Cabeçalho -->
      <div class="px-5 py-3.5 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-950/60">
        <div class="flex items-center gap-2">
          <div class="p-1.5 rounded-lg bg-primary/10 text-primary">
            {#if activeTab === 'cast'}
              <Cast size={18} />
            {:else}
              <QrCode size={18} />
            {/if}
          </div>
          <div>
            <h3 id="share-outputs-title" class="text-sm font-bold text-white">
              {activeTab === 'cast' ? 'Transmissão sem Fio (Google Cast / Chromecast)' : 'Compartilhar Saídas na Rede Local'}
            </h3>
            <p class="text-[11px] text-neutral-400 mt-0.5">
              {activeTab === 'cast'
                ? 'Projete telas em TVs e projetores sem precisar de cabos HDMI'
                : 'Abra o retorno ou projeção em tablets, celulares e navegadores'}
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

      <!-- Seletor de Abas -->
      <div class="p-3.5 pb-2 border-b border-neutral-800/80 bg-neutral-900/40 shrink-0">
        <div class="grid grid-cols-5 gap-1 p-1 bg-neutral-950 rounded-xl border border-neutral-800">
          <button
            type="button"
            onclick={() => (activeTab = 'stage')}
            class="flex items-center justify-center gap-1 py-1.5 rounded-lg text-[11px] font-semibold transition-all cursor-pointer {activeTab === 'stage' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-neutral-400 hover:text-white'}"
          >
            <Tv size={12} />
            <span>Palco</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'public')}
            class="flex items-center justify-center gap-1 py-1.5 rounded-lg text-[11px] font-semibold transition-all cursor-pointer {activeTab === 'public' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-neutral-400 hover:text-white'}"
          >
            <MonitorPlay size={12} />
            <span>Pública</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'remote')}
            class="flex items-center justify-center gap-1 py-1.5 rounded-lg text-[11px] font-semibold transition-all cursor-pointer {activeTab === 'remote' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-neutral-400 hover:text-white'}"
          >
            <Smartphone size={12} />
            <span>Remoto</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'lower')}
            class="flex items-center justify-center gap-1 py-1.5 rounded-lg text-[11px] font-semibold transition-all cursor-pointer {activeTab === 'lower' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-neutral-400 hover:text-white'}"
          >
            <Video size={12} />
            <span>OBS</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'cast')}
            class="flex items-center justify-center gap-1 py-1.5 rounded-lg text-[11px] font-semibold transition-all cursor-pointer {activeTab === 'cast' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-neutral-400 hover:text-white'}"
          >
            <Cast size={12} />
            <span>Cast</span>
          </button>
        </div>
      </div>

      <!-- Conteúdo da Aba Ativa -->
      <div class="overflow-y-auto min-h-0 flex-1">
        {#if activeTab === 'cast'}
          <!-- Aba Chromecast -->
          <div class="p-5">
            <CastControl />
          </div>
        {:else}
          <!-- Abas com QR Code e URL (Palco ou Pública) -->
          <div class="p-6 flex flex-col items-center text-center space-y-4">
            <!-- QR Code renderizado -->
            <div class="p-3 bg-white rounded-2xl shadow-xl flex items-center justify-center">
              {#if qrCodeDataUrl}
                <img
                  src={qrCodeDataUrl}
                  alt="QR Code da Saída"
                  class="w-52 h-52 object-contain rounded-lg"
                />
              {:else}
                <div class="w-52 h-52 flex flex-col items-center justify-center text-neutral-400 text-xs gap-2">
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
                {#if activeTab === 'stage'}
                  Ideal para músicos, pregadores e equipe no palco visualizarem letras e timer em tempo real.
                {:else if activeTab === 'remote'}
                  Ideal para operadores, pregadores e líderes de louvor controlarem as apresentações pelo celular.
                {:else if activeTab === 'lower'}
                  Transparência alfa nativa para OBS Studio / vMix. Adicione uma fonte de Navegador (Browser Source) com resolução 1920x1080.
                {:else}
                  Ideal para monitores auxiliares, salas de transmissão ou telões secundários.
                {/if}
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
        {/if}
      </div>

      <!-- Rodapé -->
      <div class="px-5 py-3 bg-neutral-950/60 border-t border-neutral-800 flex items-center justify-between text-xs text-neutral-400 shrink-0">
        <span class="text-[11px]">IP da máquina: <strong class="text-neutral-200 font-mono">{networkInfo?.local_ip || '127.0.0.1'}</strong></span>
        {#if activeTab === 'cast'}
          <span class="text-[11px] text-neutral-400 flex items-center gap-1">
            <Cast size={12} class="text-primary" />
            <span>Google Cast SDK</span>
          </span>
        {:else}
          <button
            type="button"
            onclick={() => window.open(activeUrl, '_blank')}
            class="text-xs text-primary hover:underline flex items-center gap-1 cursor-pointer font-medium"
          >
            <span>Abrir link</span>
            <ExternalLink size={12} />
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}

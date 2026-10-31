<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Cast,
    Tv,
    MonitorPlay,
    Wifi,
    Radio,
    AlertCircle,
    CheckCircle2,
    RefreshCw,
    Info,
    ExternalLink,
    HelpCircle,
    ChevronDown,
    ChevronUp,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import {
    castStore,
    startCasting,
    stopCasting,
    loadCastSdk,
    getCastDiagnostic,
    type CastDiagnostic,
  } from '$lib/cast';

  let showOfflineHelp = $state(false);
  let isCheckingSdk = $state(false);
  let diagnostic = $state<CastDiagnostic | null>(null);

  const cast = $derived($castStore);

  onMount(() => {
    diagnostic = getCastDiagnostic();
    isCheckingSdk = true;
    loadCastSdk().finally(() => {
      isCheckingSdk = false;
    });
  });

  async function handleStartCast(target: 'public' | 'stage') {
    await startCasting(target);
  }

  function handleStopCast() {
    stopCasting();
  }

  function openTab(path: string) {
    if (typeof window !== 'undefined') {
      window.open(path, '_blank');
    }
  }
</script>

<div class="space-y-4 text-left">
  <!-- Estado Conectado Ativo -->
  {#if cast.isConnected}
    <div class="p-4 rounded-xl bg-emerald-950/40 border border-emerald-500/30 text-emerald-200 space-y-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <span class="relative flex h-3 w-3">
            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
            <span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
          </span>
          <div>
            <h4 class="text-xs font-bold text-white flex items-center gap-1.5">
              <span>Transmitindo para</span>
              <span class="text-emerald-400 font-mono">{cast.deviceName || 'Chromecast'}</span>
            </h4>
            <p class="text-[11px] text-emerald-300/80 mt-0.5">
              Conexão WebSocket ativa com latência ultrabaixa (&lt; 30ms)
            </p>
          </div>
        </div>

        <Badge variant="outline" class="bg-emerald-500/10 text-emerald-300 border-emerald-500/30 text-[10px]">
          {cast.activeTarget === 'stage' ? 'Monitor de Palco (/stage)' : 'Tela Pública (/public)'}
        </Badge>
      </div>

      <!-- Ações rápidas quando conectado -->
      <div class="pt-2 border-t border-emerald-500/20 flex items-center justify-between gap-2">
        <div class="flex items-center gap-2">
          {#if cast.activeTarget === 'public'}
            <Button
              variant="outline"
              size="sm"
              onclick={() => handleStartCast('stage')}
              class="text-xs h-7 px-2.5 text-emerald-200 border-emerald-500/30 hover:bg-emerald-500/20"
            >
              <Tv size={13} class="mr-1.5" />
              <span>Mudar para Palco</span>
            </Button>
          {:else}
            <Button
              variant="outline"
              size="sm"
              onclick={() => handleStartCast('public')}
              class="text-xs h-7 px-2.5 text-emerald-200 border-emerald-500/30 hover:bg-emerald-500/20"
            >
              <MonitorPlay size={13} class="mr-1.5" />
              <span>Mudar para Pública</span>
            </Button>
          {/if}
        </div>

        <Button
          variant="destructive"
          size="sm"
          onclick={handleStopCast}
          class="text-xs h-7 px-3 bg-rose-600/80 hover:bg-rose-600 text-white"
        >
          Desconectar
        </Button>
      </div>
    </div>
  {:else}
    <!-- Estado Desconectado / Pronto para Transmitir -->
    <div class="space-y-3">
      <div class="text-xs text-neutral-400 leading-relaxed">
        Selecione qual saída você deseja transmitir sem fio para sua TV, projetor ou Chromecast na rede:
      </div>

      <!-- Grid com Botões de Ação para as duas telas -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
        <!-- Transmitir Palco -->
        <button
          type="button"
          disabled={cast.isConnecting}
          onclick={() => handleStartCast('stage')}
          class="group p-3.5 rounded-xl bg-neutral-950/80 border border-neutral-800 hover:border-primary/60 hover:bg-neutral-900/90 transition-all text-left flex flex-col justify-between gap-3 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed shadow-sm"
        >
          <div class="flex items-center justify-between w-full">
            <div class="p-2 rounded-lg bg-blue-500/10 text-blue-400 group-hover:bg-blue-500/20 transition-colors">
              <Tv size={18} />
            </div>
            <Badge variant="outline" class="text-[10px] text-neutral-400 group-hover:text-neutral-200">
              Sem fio
            </Badge>
          </div>
          <div>
            <div class="text-xs font-bold text-white group-hover:text-primary transition-colors flex items-center gap-1.5">
              <span>Monitor de Palco</span>
              <span class="text-[10px] text-neutral-400 font-mono">(/stage)</span>
            </div>
            <p class="text-[11px] text-neutral-400 mt-0.5 leading-snug">
              Letra atual, próxima e cronômetro em tempo real para os músicos e pregador.
            </p>
          </div>
          <div class="flex items-center gap-1.5 text-[11px] font-semibold text-blue-400 group-hover:translate-x-0.5 transition-transform">
            {#if cast.isConnecting}
              <RefreshCw size={12} class="animate-spin" />
              <span>Conectando...</span>
            {:else}
              <Cast size={13} />
              <span>Transmitir para Palco &rarr;</span>
            {/if}
          </div>
        </button>

        <!-- Transmitir Projeção Pública -->
        <button
          type="button"
          disabled={cast.isConnecting}
          onclick={() => handleStartCast('public')}
          class="group p-3.5 rounded-xl bg-neutral-950/80 border border-neutral-800 hover:border-primary/60 hover:bg-neutral-900/90 transition-all text-left flex flex-col justify-between gap-3 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed shadow-sm"
        >
          <div class="flex items-center justify-between w-full">
            <div class="p-2 rounded-lg bg-purple-500/10 text-purple-400 group-hover:bg-purple-500/20 transition-colors">
              <MonitorPlay size={18} />
            </div>
            <Badge variant="outline" class="text-[10px] text-neutral-400 group-hover:text-neutral-200">
              Sem fio
            </Badge>
          </div>
          <div>
            <div class="text-xs font-bold text-white group-hover:text-primary transition-colors flex items-center gap-1.5">
              <span>Tela Pública</span>
              <span class="text-[10px] text-neutral-400 font-mono">(/public)</span>
            </div>
            <p class="text-[11px] text-neutral-400 mt-0.5 leading-snug">
              Projeta hinos, versículos e papéis de parede para o público com nitidez 1080p.
            </p>
          </div>
          <div class="flex items-center gap-1.5 text-[11px] font-semibold text-purple-400 group-hover:translate-x-0.5 transition-transform">
            {#if cast.isConnecting}
              <RefreshCw size={12} class="animate-spin" />
              <span>Conectando...</span>
            {:else}
              <Cast size={13} />
              <span>Transmitir Projeção &rarr;</span>
            {/if}
          </div>
        </button>
      </div>
    </div>
  {/if}

  <!-- Banner de Erro caso ocorra falha -->
  {#if cast.error}
    <div class="p-3.5 rounded-xl bg-amber-950/40 border border-amber-500/30 text-amber-200 text-xs space-y-2.5">
      <div class="flex items-start gap-2">
        <AlertCircle size={16} class="text-amber-400 shrink-0 mt-0.5" />
        <div class="flex-1">
          <span class="font-semibold text-amber-300">Aviso do Cast:</span>
          <span class="text-amber-200/90 ml-1">{cast.error}</span>
        </div>
      </div>

      <!-- Solução rápida alternativa via aba -->
      <div class="pt-2 border-t border-amber-500/20 flex flex-wrap items-center gap-2">
        <span class="text-[11px] text-amber-300/80 font-medium">Solução rápida via aba:</span>
        <button
          type="button"
          onclick={() => openTab('/public')}
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded bg-amber-500/20 hover:bg-amber-500/30 text-amber-200 border border-amber-500/30 text-[11px] font-semibold transition-colors cursor-pointer"
        >
          <ExternalLink size={12} />
          <span>Abrir /public</span>
        </button>
        <button
          type="button"
          onclick={() => openTab('/stage')}
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded bg-amber-500/20 hover:bg-amber-500/30 text-amber-200 border border-amber-500/30 text-[11px] font-semibold transition-colors cursor-pointer"
        >
          <ExternalLink size={12} />
          <span>Abrir /stage</span>
        </button>
      </div>

      <p class="text-[11px] text-amber-300/70 leading-relaxed">
        <strong>Como transmitir a aba para a TV:</strong> Na aba que abrir, clique com o botão direito &rarr; <em class="text-amber-200">Transmitir...</em> (ou no menu do Chrome nos 3 pontinhos &rarr; <em class="text-amber-200">Transmitir...</em>) e selecione seu Chromecast.
      </p>
    </div>
  {/if}

  <!-- Requisitos e Vantagens da Conexão Direta -->
  <div class="p-3 rounded-xl bg-neutral-950/50 border border-neutral-800/80 text-[11px] space-y-2">
    <div class="flex items-center gap-1.5 font-semibold text-neutral-300">
      <Wifi size={13} class="text-emerald-400 shrink-0" />
      <span>Como funciona a transmissão com o OpenPresenter:</span>
    </div>
    <ul class="space-y-1.5 text-neutral-400 pl-4 list-disc marker:text-neutral-600">
      <li>
        <strong class="text-neutral-300">Latência quase zero (&lt; 30ms):</strong> Diferente de transmissões de vídeo tradicionais (como HLS), os slides usam WebSocket direto na rede local. Ao clicar na próxima estrofe, a TV muda instantaneamente.
      </li>
      <li>
        <strong class="text-neutral-300">Fontes nítidas e 0% de CPU:</strong> O Chromecast renderiza o texto vetorial nativo em tela cheia sem sobrecarregar a placa de vídeo ou processador do seu computador.
      </li>
      <li>
        <strong class="text-neutral-300">Mesma rede Wi-Fi:</strong> O computador e o Chromecast devem estar conectados no mesmo roteador sem isolamento de rede.
      </li>
    </ul>
  </div>

  <!-- Modo Alternativo / Offline (Espelhamento de Aba do Chrome) -->
  <div class="rounded-xl border border-neutral-800 bg-neutral-950/40 overflow-hidden">
    <button
      type="button"
      onclick={() => (showOfflineHelp = !showOfflineHelp)}
      class="w-full px-3.5 py-2.5 flex items-center justify-between text-left text-xs font-semibold text-neutral-300 hover:text-white hover:bg-neutral-800/40 transition-colors cursor-pointer"
    >
      <div class="flex items-center gap-2">
        <HelpCircle size={14} class="text-primary" />
        <span>Templo sem internet? Use o Espelhamento de Aba Nativo</span>
      </div>
      {#if showOfflineHelp}
        <ChevronUp size={14} />
      {:else}
        <ChevronDown size={14} />
      {/if}
    </button>

    {#if showOfflineHelp}
      <div class="px-3.5 py-3 border-t border-neutral-800/80 text-[11px] text-neutral-400 space-y-2 bg-neutral-950/80">
        <p class="leading-relaxed">
          Se o templo estiver sem conexão externa com a internet ou o script do Google Cast não carregar, você pode transmitir 100% offline direto pelo navegador Google Chrome ou Edge:
        </p>
        <ol class="list-decimal pl-4 space-y-1 text-neutral-300 marker:text-neutral-500">
          <li>Abra uma nova aba no Chrome com a saída desejada (<code class="bg-neutral-900 px-1 py-0.5 rounded text-neutral-200">/public</code> ou <code class="bg-neutral-900 px-1 py-0.5 rounded text-neutral-200">/stage</code>).</li>
          <li>Clique nos <strong class="text-white">três pontinhos</strong> do canto superior direito do Chrome &rarr; <strong class="text-white">Transmitir...</strong></li>
          <li>Em <strong class="text-white">Fontes</strong>, escolha <strong class="text-white">Transmitir aba</strong>.</li>
          <li>Selecione sua TV ou Chromecast. O Chrome usará WebRTC local direto e em alta taxa de quadros!</li>
        </ol>
      </div>
    {/if}
  </div>
</div>

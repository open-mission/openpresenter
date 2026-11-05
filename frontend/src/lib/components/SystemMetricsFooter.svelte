<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Cpu,
    Activity,
    Layers,
    Clock,
    ChevronDown,
    ChevronUp,
    X,
    Info,
    CheckCircle2,
    AlertTriangle,
    Wifi,
  } from 'lucide-svelte';
  import type { SystemMetrics } from '$lib/types';
  import { connectedClients, isWsConnected } from '$lib/ws';

  interface Props {
    onOpenSettingsAbout?: () => void;
  }

  let { onOpenSettingsAbout }: Props = $props();

  let metrics = $state<SystemMetrics | null>(null);
  let isDetailsOpen = $state(false);
  let isCollapsed = $state(false);
  let fetchError = $state(false);

  onMount(() => {
    // Restaura preferência de colapso do localStorage
    if (typeof window !== 'undefined') {
      const stored = localStorage.getItem('openpresenter:system-footer-collapsed');
      if (stored === 'true') {
        isCollapsed = true;
      }
    }

    // Carregamento inicial
    loadMetrics();

    // Polling a cada 2.5 segundos (pausa quando aba em segundo plano)
    const interval = setInterval(() => {
      if (typeof document !== 'undefined' && !document.hidden) {
        loadMetrics();
      }
    }, 2500);

    return () => {
      clearInterval(interval);
    };
  });

  async function loadMetrics() {
    try {
      const res = await fetch('/api/system/metrics');
      if (res.ok) {
        metrics = await res.json();
        fetchError = false;
      } else {
        fetchError = true;
      }
    } catch {
      fetchError = true;
    }
  }

  function toggleCollapse() {
    isCollapsed = !isCollapsed;
    if (typeof window !== 'undefined') {
      localStorage.setItem('openpresenter:system-footer-collapsed', String(isCollapsed));
    }
  }

  function formatUptime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m ${secs}s`;
  }

  function formatGigabytes(mb: number): string {
    return (mb / 1024).toFixed(1);
  }

  // Cores dinâmicas por nível de carga
  const cpuColor = $derived.by(() => {
    if (!metrics) return 'text-neutral-400';
    if (metrics.system_cpu_percent >= 85) return 'text-rose-400';
    if (metrics.system_cpu_percent >= 60) return 'text-amber-400';
    return 'text-emerald-400';
  });

  const ramColor = $derived.by(() => {
    if (!metrics) return 'text-neutral-400';
    if (metrics.system_memory_percent >= 90) return 'text-rose-400';
    if (metrics.system_memory_percent >= 75) return 'text-amber-400';
    return 'text-emerald-400';
  });

  const gpuColor = $derived.by(() => {
    if (!metrics || metrics.gpu_usage_percent == null) return 'text-purple-400';
    if (metrics.gpu_usage_percent >= 85) return 'text-rose-400';
    if (metrics.gpu_usage_percent >= 70) return 'text-amber-400';
    return 'text-purple-400';
  });
</script>

<!-- Barra Fixa de Rodapé -->
<footer
  class="relative shrink-0 border-t border-neutral-800/80 bg-neutral-950/90 backdrop-blur-md text-neutral-400 select-none transition-all duration-200 z-30"
>
  {#if isCollapsed}
    <!-- MODO COMPACTO / COLAPSADO -->
    <div class="h-6 px-3 flex items-center justify-between text-[10px]">
      <div class="flex items-center gap-3">
        <span class="inline-flex items-center gap-1 font-semibold text-neutral-300">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
          Sistema Online
        </span>
        {#if metrics}
          <span class="text-neutral-500 font-mono">
            CPU: {metrics.process_cpu_percent}% • RAM: {metrics.process_memory_mb} MB
          </span>
        {/if}
      </div>

      <button
        type="button"
        onclick={toggleCollapse}
        class="inline-flex items-center gap-1 text-neutral-400 hover:text-white transition-colors cursor-pointer"
        title="Expandir barra de telemetria"
      >
        <span>Expandir Telemetria</span>
        <ChevronUp size={12} />
      </button>
    </div>
  {:else}
    <!-- MODO EXPANDIDO / PADRÃO -->
    <div class="h-7 px-3.5 flex items-center justify-between gap-2 text-[11px] overflow-x-auto no-scrollbar">
      <!-- Lado Esquerdo: Telemetria de CPU, RAM e GPU -->
      <div class="flex items-center gap-4 shrink-0">
        <!-- CPU -->
        <button
          type="button"
          onclick={() => (isDetailsOpen = true)}
          class="flex items-center gap-1.5 px-2 py-0.5 rounded-md hover:bg-neutral-800/80 transition-colors cursor-pointer group"
          title="Clique para ver detalhes do processador"
        >
          <Cpu size={12} class={cpuColor} />
          <span class="font-medium text-neutral-300 group-hover:text-white">
            CPU:
          </span>
          {#if metrics}
            <span class="font-mono text-neutral-200 font-semibold">
              {metrics.process_cpu_percent}%
            </span>
            <span class="text-[10px] text-neutral-500 font-mono">
              (Sys: {metrics.system_cpu_percent}%)
            </span>
          {:else}
            <span class="text-neutral-600">...</span>
          {/if}
        </button>

        <div class="w-px h-3 bg-neutral-800"></div>

        <!-- RAM -->
        <button
          type="button"
          onclick={() => (isDetailsOpen = true)}
          class="flex items-center gap-1.5 px-2 py-0.5 rounded-md hover:bg-neutral-800/80 transition-colors cursor-pointer group"
          title="Clique para ver detalhes de memória"
        >
          <Activity size={12} class={ramColor} />
          <span class="font-medium text-neutral-300 group-hover:text-white">
            RAM:
          </span>
          {#if metrics}
            <span class="font-mono text-neutral-200 font-semibold">
              {metrics.process_memory_mb} MB
            </span>
            <span class="text-[10px] text-neutral-500 font-mono">
              (Sys: {formatGigabytes(metrics.system_memory_used_mb)}/{formatGigabytes(metrics.system_memory_total_mb)} GB)
            </span>
          {:else}
            <span class="text-neutral-600">...</span>
          {/if}
        </button>

        <div class="w-px h-3 bg-neutral-800"></div>

        <!-- GPU -->
        <button
          type="button"
          onclick={() => (isDetailsOpen = true)}
          class="flex items-center gap-1.5 px-2 py-0.5 rounded-md hover:bg-neutral-800/80 transition-colors cursor-pointer group"
          title="Clique para ver detalhes da placa de vídeo"
        >
          <Layers size={12} class={gpuColor} />
          <span class="font-medium text-neutral-300 group-hover:text-white">
            GPU:
          </span>
          {#if metrics?.gpu_model}
            <span class="text-neutral-200 font-semibold truncate max-w-[140px]">
              {metrics.gpu_model}
            </span>
            {#if metrics.gpu_usage_percent != null}
              <span class="font-mono text-neutral-400 text-[10px]">
                {metrics.gpu_usage_percent}%
              </span>
            {/if}
          {:else}
            <span class="text-neutral-500">Integrada / Ativa</span>
          {/if}
        </button>
      </div>

      <!-- Lado Direito: Telas Conectadas, Uptime e Botão de Colapso -->
      <div class="flex items-center gap-3 shrink-0 text-neutral-400">
        <!-- Conexões WebSockets Ativas -->
        <div
          class="flex items-center gap-1 px-2 py-0.5 rounded bg-neutral-900 border border-neutral-800/80 text-[10px]"
          title="Dispositivos e telas conectadas em tempo real"
        >
          <Wifi size={10} class={$isWsConnected ? 'text-emerald-400' : 'text-neutral-500'} />
          <span class="font-mono text-neutral-300 font-medium">
            {$connectedClients.length} {$connectedClients.length === 1 ? 'tela' : 'telas'}
          </span>
        </div>

        {#if metrics?.uptime_seconds}
          <!-- Uptime da Aplicação -->
          <div
            class="hidden sm:flex items-center gap-1 text-[10px] text-neutral-500 font-mono"
            title="Tempo de atividade ininterrupta do servidor"
          >
            <Clock size={10} />
            <span>{formatUptime(metrics.uptime_seconds)}</span>
          </div>
        {/if}

        <!-- Botão Detalhes -->
        <button
          type="button"
          onclick={() => (isDetailsOpen = true)}
          class="p-1 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
          title="Ver diagnóstico detalhado do sistema"
        >
          <Info size={12} />
        </button>

        <!-- Botão Recolher -->
        <button
          type="button"
          onclick={toggleCollapse}
          class="p-1 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
          title="Minimizar barra de telemetria"
        >
          <ChevronDown size={12} />
        </button>
      </div>
    </div>
  {/if}
</footer>

<!-- DIÁLOGO / POPOVER DETALHADO DE TELEMETRIA -->
{#if isDetailsOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-xs p-4 animate-in fade-in duration-150"
    role="presentation"
  >
    <div
      class="w-full max-w-lg rounded-2xl border border-neutral-800 bg-neutral-900 shadow-2xl p-6 text-white space-y-5"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => {
        if (e.key === 'Escape') isDetailsOpen = false;
      }}
      role="dialog"
      tabindex="-1"
    >
      <!-- Cabeçalho -->
      <div class="flex items-center justify-between pb-3 border-b border-neutral-800">
        <div class="flex items-center gap-2.5">
          <div
            class="w-8 h-8 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 flex items-center justify-center"
          >
            <Activity size={16} />
          </div>
          <div>
            <h3 class="text-sm font-bold text-white">Telemetria de Hardware & Sistema</h3>
            <p class="text-[11px] text-neutral-400">
              Desempenho da máquina e consumo do OpenPresenter em tempo real
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={() => (isDetailsOpen = false)}
          class="w-7 h-7 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
        >
          <X size={15} />
        </button>
      </div>

      {#if metrics}
        <!-- Grid de Cards de Recursos -->
        <div class="grid grid-cols-2 gap-3.5">
          <!-- Card OpenPresenter (Processo) -->
          <div class="p-3.5 rounded-xl bg-neutral-950/70 border border-neutral-800 space-y-2">
            <span class="text-[10px] font-bold text-emerald-400 uppercase tracking-wider">
              Processo OpenPresenter
            </span>
            <div class="space-y-1 text-xs">
              <div class="flex justify-between">
                <span class="text-neutral-400">Memória Física:</span>
                <span class="font-mono font-bold text-white">{metrics.process_memory_mb} MB</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-400">Uso de CPU:</span>
                <span class="font-mono font-bold text-white">{metrics.process_cpu_percent}%</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-400">Tempo Ativo:</span>
                <span class="font-mono text-neutral-300">{formatUptime(metrics.uptime_seconds)}</span>
              </div>
            </div>
          </div>

          <!-- Card Computador (Sistema Host) -->
          <div class="p-3.5 rounded-xl bg-neutral-950/70 border border-neutral-800 space-y-2">
            <span class="text-[10px] font-bold text-primary uppercase tracking-wider">
              Sistema Operacional (Host)
            </span>
            <div class="space-y-1 text-xs">
              <div class="flex justify-between">
                <span class="text-neutral-400">CPU Total:</span>
                <span class="font-mono font-bold text-white">{metrics.system_cpu_percent}%</span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-400">RAM Ocupada:</span>
                <span class="font-mono font-bold text-white">
                  {formatGigabytes(metrics.system_memory_used_mb)} / {formatGigabytes(metrics.system_memory_total_mb)} GB
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-neutral-400">Uso da RAM:</span>
                <span class="font-mono text-neutral-300">{metrics.system_memory_percent}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Barras de Carga do Sistema -->
        <div class="p-4 rounded-xl bg-neutral-950/70 border border-neutral-800 space-y-3">
          <!-- Barra CPU -->
          <div class="space-y-1.5">
            <div class="flex items-center justify-between text-xs">
              <span class="text-neutral-300 font-medium flex items-center gap-1.5">
                <Cpu size={13} class={cpuColor} />
                Carga de CPU Global
              </span>
              <span class="font-mono font-bold {cpuColor}">
                {metrics.system_cpu_percent}%
              </span>
            </div>
            <div class="w-full h-2 rounded-full bg-neutral-800 overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300 {metrics.system_cpu_percent >= 85 ? 'bg-rose-500' : metrics.system_cpu_percent >= 60 ? 'bg-amber-500' : 'bg-emerald-500'}"
                style="width: {Math.min(metrics.system_cpu_percent, 100)}%"
              ></div>
            </div>
          </div>

          <!-- Barra RAM -->
          <div class="space-y-1.5">
            <div class="flex items-center justify-between text-xs">
              <span class="text-neutral-300 font-medium flex items-center gap-1.5">
                <Activity size={13} class={ramColor} />
                Ocupação da Memória RAM
              </span>
              <span class="font-mono font-bold {ramColor}">
                {metrics.system_memory_percent}%
              </span>
            </div>
            <div class="w-full h-2 rounded-full bg-neutral-800 overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300 {metrics.system_memory_percent >= 90 ? 'bg-rose-500' : metrics.system_memory_percent >= 75 ? 'bg-amber-500' : 'bg-emerald-500'}"
                style="width: {Math.min(metrics.system_memory_percent, 100)}%"
              ></div>
            </div>
          </div>
        </div>

        <!-- Placa de Vídeo (GPU) -->
        <div class="p-3.5 rounded-xl bg-neutral-950/70 border border-neutral-800 flex items-center justify-between">
          <div class="flex items-center gap-2.5">
            <div
              class="w-7 h-7 rounded-lg bg-purple-500/10 border border-purple-500/20 text-purple-400 flex items-center justify-center shrink-0"
            >
              <Layers size={14} />
            </div>
            <div>
              <span class="text-[10px] font-bold text-purple-400 uppercase tracking-wider block">
                Processador Gráfico (GPU)
              </span>
              <span class="text-xs font-semibold text-white">
                {metrics.gpu_model || 'GPU Integrada do Sistema'}
              </span>
            </div>
          </div>

          {#if metrics.gpu_usage_percent != null}
            <div class="text-right">
              <span class="text-[10px] text-neutral-400 block">Uso da GPU</span>
              <span class="font-mono text-xs font-bold text-purple-400">
                {metrics.gpu_usage_percent}%
              </span>
            </div>
          {:else if metrics.gpu_memory_used_mb != null}
            <div class="text-right">
              <span class="text-[10px] text-neutral-400 block">VRAM</span>
              <span class="font-mono text-xs font-bold text-purple-400">
                {formatGigabytes(metrics.gpu_memory_used_mb)} / {formatGigabytes(metrics.gpu_memory_total_mb || 0)} GB
              </span>
            </div>
          {:else}
            <span class="text-[10px] text-neutral-500 font-mono">Renderização Ativa</span>
          {/if}
        </div>

        <!-- Dica de Saúde -->
        <div class="flex items-start gap-2.5 p-3 rounded-xl bg-neutral-950 border border-neutral-800/80 text-[11px] text-neutral-400">
          <CheckCircle2 size={15} class="text-emerald-400 shrink-0 mt-0.5" />
          <p class="leading-relaxed">
            O OpenPresenter gerencia memória de forma nativa e enxuta via Rust, consumindo uma fração mínima dos recursos da sua máquina mesmo durante projeções intensivas.
          </p>
        </div>
      {:else}
        <div class="py-8 text-center text-xs text-neutral-500">
          Carregando telemetria do sistema...
        </div>
      {/if}

      <!-- Rodapé do Diálogo -->
      <div class="flex items-center justify-end pt-2 border-t border-neutral-800">
        <button
          type="button"
          onclick={() => (isDetailsOpen = false)}
          class="px-4 py-1.5 rounded-lg text-xs font-semibold bg-neutral-800 hover:bg-neutral-700 text-white transition-colors cursor-pointer"
        >
          Entendido
        </button>
      </div>
    </div>
  </div>
{/if}

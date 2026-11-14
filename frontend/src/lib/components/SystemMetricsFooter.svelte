<script lang="ts">
  import { onMount, tick } from 'svelte';
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
    RefreshCw,
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
  let isRetrying = $state(false);
  let consecutiveFailures = 0;
  let pollTimeoutId: ReturnType<typeof setTimeout> | null = null;
  let dialogEl = $state<HTMLDivElement | null>(null);
  let previousFocusedElement: HTMLElement | null = null;

  const isSystemHealthy = $derived($isWsConnected && !fetchError);

  function scheduleNextPoll() {
    if (pollTimeoutId) clearTimeout(pollTimeoutId);
    const baseDelay = isCollapsed ? 8000 : 2500;
    const delay =
      consecutiveFailures > 0
        ? Math.min(60000, baseDelay * Math.pow(2, consecutiveFailures))
        : baseDelay;

    pollTimeoutId = setTimeout(async () => {
      if (typeof document !== 'undefined' && !document.hidden) {
        await loadMetrics();
      }
      scheduleNextPoll();
    }, delay);
  }

  onMount(() => {
    if (typeof window !== 'undefined') {
      const stored = localStorage.getItem('openpresenter:system-footer-collapsed');
      if (stored === 'true') {
        isCollapsed = true;
      }
    }

    void loadMetrics().then(() => {
      scheduleNextPoll();
    });

    const unsubWs = isWsConnected.subscribe((connected) => {
      if (connected && consecutiveFailures > 0) {
        consecutiveFailures = 0;
        void loadMetrics().then(() => scheduleNextPoll());
      }
    });

    return () => {
      if (pollTimeoutId) clearTimeout(pollTimeoutId);
      unsubWs();
    };
  });

  async function loadMetrics() {
    try {
      const res = await fetch('/api/system/metrics');
      if (res.ok) {
        metrics = await res.json();
        fetchError = false;
        consecutiveFailures = 0;
      } else {
        fetchError = true;
        consecutiveFailures = Math.min(consecutiveFailures + 1, 5);
      }
    } catch {
      fetchError = true;
      consecutiveFailures = Math.min(consecutiveFailures + 1, 5);
    }
  }

  async function handleManualRetry() {
    isRetrying = true;
    consecutiveFailures = 0;
    await loadMetrics();
    scheduleNextPoll();
    isRetrying = false;
  }

  function openDetails() {
    previousFocusedElement = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    isDetailsOpen = true;
    void tick().then(() => {
      dialogEl?.focus();
    });
  }

  function closeDetails() {
    isDetailsOpen = false;
    if (previousFocusedElement && document.contains(previousFocusedElement)) {
      previousFocusedElement.focus();
    }
    previousFocusedElement = null;
  }

  function handleDialogKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      closeDetails();
      return;
    }
    if (e.key === 'Tab' && dialogEl) {
      const focusable = Array.from(
        dialogEl.querySelectorAll<HTMLElement>('button:not(:disabled), [href], input:not(:disabled), [tabindex]:not([tabindex="-1"])')
      );
      if (focusable.length === 0) return;
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
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

  // Cores dinâmicas por nível de carga com contraste Light + Dark
  const cpuColor = $derived.by(() => {
    if (!metrics) return 'text-muted-foreground';
    if (metrics.system_cpu_percent >= 85) return 'text-red-700 dark:text-rose-400';
    if (metrics.system_cpu_percent >= 60) return 'text-amber-700 dark:text-amber-400';
    return 'text-emerald-700 dark:text-emerald-400';
  });

  const ramColor = $derived.by(() => {
    if (!metrics) return 'text-muted-foreground';
    if (metrics.system_memory_percent >= 90) return 'text-red-700 dark:text-rose-400';
    if (metrics.system_memory_percent >= 75) return 'text-amber-700 dark:text-amber-400';
    return 'text-emerald-700 dark:text-emerald-400';
  });

  const gpuColor = $derived.by(() => {
    if (!metrics || metrics.gpu_usage_percent == null) return 'text-purple-700 dark:text-purple-400';
    if (metrics.gpu_usage_percent >= 85) return 'text-red-700 dark:text-rose-400';
    if (metrics.gpu_usage_percent >= 70) return 'text-amber-700 dark:text-amber-400';
    return 'text-purple-700 dark:text-purple-400';
  });
</script>

<!-- Barra Fixa de Rodapé -->
<footer
  class="relative shrink-0 border-t border-border bg-card/90 backdrop-blur-md text-muted-foreground select-none transition-all duration-200 z-30"
>
  {#if isCollapsed}
    <!-- MODO COMPACTO / COLAPSADO -->
    <div class="h-7 px-3 flex items-center justify-between text-[11px]">
      <div class="flex items-center gap-3">
        <span class="inline-flex items-center gap-1.5 font-semibold text-foreground">
          <span
            class="w-2 h-2 rounded-full {isSystemHealthy
              ? 'bg-emerald-500'
              : fetchError
                ? 'bg-red-500'
                : 'bg-amber-500'}"
          ></span>
          {isSystemHealthy ? 'Sistema Online' : fetchError ? 'Sem telemetria' : 'Reconectando'}
        </span>
        {#if metrics && !fetchError}
          <span class="text-muted-foreground font-mono">
            CPU: {metrics.process_cpu_percent}% • RAM: {metrics.process_memory_mb} MB
          </span>
        {/if}
      </div>

      <button
        type="button"
        onclick={toggleCollapse}
        aria-expanded={!isCollapsed}
        class="min-h-6 px-2 rounded inline-flex items-center gap-1 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title="Expandir barra de telemetria"
      >
        <span>Expandir Telemetria</span>
        <ChevronUp size={12} />
      </button>
    </div>
  {:else}
    <!-- MODO EXPANDIDO / PADRÃO -->
    <div class="h-8 px-3.5 flex items-center justify-between gap-2 text-[11px] overflow-x-auto no-scrollbar">
      <!-- Lado Esquerdo: Telemetria de CPU, RAM e GPU -->
      <div class="flex items-center gap-3 shrink-0">
        <!-- CPU -->
        <button
          type="button"
          onclick={openDetails}
          class="min-h-6 flex items-center gap-1.5 px-2 py-0.5 rounded-md hover:bg-muted transition-colors cursor-pointer group"
          title="Clique para ver detalhes do processador"
        >
          <Cpu size={12} class={cpuColor} />
          <span class="font-medium text-muted-foreground group-hover:text-foreground">
            CPU:
          </span>
          {#if metrics && !fetchError}
            <span class="font-mono text-foreground font-semibold">
              {metrics.process_cpu_percent}%
            </span>
            <span class="text-[11px] text-muted-foreground font-mono">
              (Sys: {metrics.system_cpu_percent}%)
            </span>
          {:else}
            <span class="text-muted-foreground">—</span>
          {/if}
        </button>

        <div class="w-px h-3.5 bg-border"></div>

        <!-- RAM -->
        <button
          type="button"
          onclick={openDetails}
          class="min-h-6 flex items-center gap-1.5 px-2 py-0.5 rounded-md hover:bg-muted transition-colors cursor-pointer group"
          title="Clique para ver detalhes de memória"
        >
          <Activity size={12} class={ramColor} />
          <span class="font-medium text-muted-foreground group-hover:text-foreground">
            RAM:
          </span>
          {#if metrics && !fetchError}
            <span class="font-mono text-foreground font-semibold">
              {metrics.process_memory_mb} MB
            </span>
            <span class="text-[11px] text-muted-foreground font-mono">
              (Sys: {formatGigabytes(metrics.system_memory_used_mb)}/{formatGigabytes(metrics.system_memory_total_mb)} GB)
            </span>
          {:else}
            <span class="text-muted-foreground">—</span>
          {/if}
        </button>

        <div class="w-px h-3.5 bg-border"></div>

        <!-- GPU -->
        <button
          type="button"
          onclick={openDetails}
          class="min-h-6 flex items-center gap-1.5 px-2 py-0.5 rounded-md hover:bg-muted transition-colors cursor-pointer group"
          title="Clique para ver detalhes da placa de vídeo"
        >
          <Layers size={12} class={gpuColor} />
          <span class="font-medium text-muted-foreground group-hover:text-foreground">
            GPU:
          </span>
          {#if metrics?.gpu_model && !fetchError}
            <span class="text-foreground font-semibold truncate max-w-[140px]">
              {metrics.gpu_model}
            </span>
            {#if metrics.gpu_usage_percent != null}
              <span class="font-mono text-muted-foreground text-[11px]">
                {metrics.gpu_usage_percent}%
              </span>
            {/if}
          {:else}
            <span class="text-muted-foreground">{fetchError ? 'Indisponível' : 'Integrada / Ativa'}</span>
          {/if}
        </button>
      </div>

      <!-- Lado Direito: Telas Conectadas, Uptime e Botão de Colapso -->
      <div class="flex items-center gap-2.5 shrink-0 text-muted-foreground">
        <!-- Conexões WebSockets Ativas -->
        <div
          class="min-h-6 flex items-center gap-1.5 px-2 py-0.5 rounded bg-muted/60 border border-border text-[11px]"
          title="Dispositivos e telas conectadas em tempo real"
        >
          <Wifi size={11} class={$isWsConnected ? 'text-emerald-700 dark:text-emerald-400' : 'text-red-700 dark:text-red-400'} />
          <span class="font-mono text-foreground font-medium">
            {$connectedClients.length} {$connectedClients.length === 1 ? 'tela' : 'telas'}
          </span>
        </div>

        {#if metrics?.uptime_seconds && !fetchError}
          <!-- Uptime da Aplicação -->
          <div
            class="hidden sm:flex items-center gap-1 text-[11px] text-muted-foreground font-mono"
            title="Tempo de atividade ininterrupta do servidor"
          >
            <Clock size={11} />
            <span>{formatUptime(metrics.uptime_seconds)}</span>
          </div>
        {/if}

        <!-- Botão Detalhes -->
        <button
          type="button"
          onclick={openDetails}
          aria-label="Ver diagnóstico detalhado do sistema"
          class="w-7 h-7 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
          title="Ver diagnóstico detalhado do sistema"
        >
          <Info size={13} />
        </button>

        <!-- Botão Recolher -->
        <button
          type="button"
          onclick={toggleCollapse}
          aria-expanded={!isCollapsed}
          aria-label="Minimizar barra de telemetria"
          class="w-7 h-7 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
          title="Minimizar barra de telemetria"
        >
          <ChevronDown size={13} />
        </button>
      </div>
    </div>
  {/if}
</footer>

<!-- DIÁLOGO / POPOVER DETALHADO DE TELEMETRIA -->
{#if isDetailsOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/65 backdrop-blur-xs p-4 animate-in fade-in duration-150"
    onclick={closeDetails}
    role="presentation"
  >
    <div
      bind:this={dialogEl}
      class="w-full max-w-lg rounded-2xl border border-border bg-card shadow-2xl p-6 text-foreground space-y-5 focus:outline-none"
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleDialogKeydown}
      role="dialog"
      aria-modal="true"
      aria-labelledby="system-telemetry-title"
      tabindex="-1"
    >
      <!-- Cabeçalho -->
      <div class="flex items-center justify-between pb-3 border-b border-border">
        <div class="flex items-center gap-2.5">
          <div
            class="w-8 h-8 rounded-lg {fetchError
              ? 'bg-red-500/15 border-red-500/30 text-red-700 dark:text-red-400'
              : 'bg-emerald-500/15 border-emerald-500/30 text-emerald-700 dark:text-emerald-400'} border flex items-center justify-center"
          >
            {#if fetchError}
              <AlertTriangle size={16} />
            {:else}
              <Activity size={16} />
            {/if}
          </div>
          <div>
            <h3 id="system-telemetry-title" class="text-sm font-bold text-foreground">
              Telemetria de Hardware & Sistema
            </h3>
            <p class="text-[11px] text-muted-foreground">
              Desempenho da máquina e consumo do OpenPresenter em tempo real
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={closeDetails}
          aria-label="Fechar telemetria do sistema"
          class="w-7 h-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
        >
          <X size={15} />
        </button>
      </div>

      {#if fetchError}
        <div class="p-4 rounded-xl bg-red-500/10 border border-red-500/30 space-y-3 text-xs">
          <div class="flex items-start gap-2.5 text-red-700 dark:text-red-300">
            <AlertTriangle size={16} class="shrink-0 mt-0.5" />
            <div class="space-y-1">
              <p class="font-bold">Não foi possível ler a telemetria do servidor</p>
              <p class="text-muted-foreground">
                Verifique se o backend local está em execução. As tentativas automáticas continuam em intervalo reduzido.
              </p>
            </div>
          </div>
          <div class="flex justify-end">
            <button
              type="button"
              onclick={handleManualRetry}
              disabled={isRetrying}
              class="min-h-8 px-3 py-1.5 rounded-lg bg-muted hover:bg-muted/80 border border-border text-foreground font-semibold inline-flex items-center gap-1.5 cursor-pointer disabled:opacity-50"
            >
              <RefreshCw size={13} class={isRetrying ? 'animate-spin' : ''} />
              <span>{isRetrying ? 'Tentando...' : 'Tentar novamente'}</span>
            </button>
          </div>
        </div>
      {:else if metrics}
        <!-- Grid de Cards de Recursos -->
        <div class="grid grid-cols-2 gap-3.5">
          <!-- Card OpenPresenter (Processo) -->
          <div class="p-3.5 rounded-xl bg-muted/40 border border-border space-y-2">
            <span class="text-[11px] font-bold text-emerald-700 dark:text-emerald-400 uppercase tracking-wider">
              Processo OpenPresenter
            </span>
            <div class="space-y-1 text-xs">
              <div class="flex justify-between">
                <span class="text-muted-foreground">Memória Física:</span>
                <span class="font-mono font-bold text-foreground">{metrics.process_memory_mb} MB</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Uso de CPU:</span>
                <span class="font-mono font-bold text-foreground">{metrics.process_cpu_percent}%</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Tempo Ativo:</span>
                <span class="font-mono text-foreground">{formatUptime(metrics.uptime_seconds)}</span>
              </div>
            </div>
          </div>

          <!-- Card Computador (Sistema Host) -->
          <div class="p-3.5 rounded-xl bg-muted/40 border border-border space-y-2">
            <span class="text-[11px] font-bold text-primary uppercase tracking-wider">
              Sistema Operacional (Host)
            </span>
            <div class="space-y-1 text-xs">
              <div class="flex justify-between">
                <span class="text-muted-foreground">CPU Total:</span>
                <span class="font-mono font-bold text-foreground">{metrics.system_cpu_percent}%</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">RAM Ocupada:</span>
                <span class="font-mono font-bold text-foreground">
                  {formatGigabytes(metrics.system_memory_used_mb)} / {formatGigabytes(metrics.system_memory_total_mb)} GB
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Uso da RAM:</span>
                <span class="font-mono text-foreground">{metrics.system_memory_percent}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Barras de Carga do Sistema -->
        <div class="p-4 rounded-xl bg-muted/40 border border-border space-y-3">
          <!-- Barra CPU -->
          <div class="space-y-1.5">
            <div class="flex items-center justify-between text-xs">
              <span class="text-foreground font-medium flex items-center gap-1.5">
                <Cpu size={13} class={cpuColor} />
                Carga de CPU Global
              </span>
              <span class="font-mono font-bold {cpuColor}">
                {metrics.system_cpu_percent}%
              </span>
            </div>
            <div class="w-full h-2 rounded-full bg-muted overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300 {metrics.system_cpu_percent >= 85 ? 'bg-red-500' : metrics.system_cpu_percent >= 60 ? 'bg-amber-500' : 'bg-emerald-500'}"
                style="width: {Math.min(metrics.system_cpu_percent, 100)}%"
              ></div>
            </div>
          </div>

          <!-- Barra RAM -->
          <div class="space-y-1.5">
            <div class="flex items-center justify-between text-xs">
              <span class="text-foreground font-medium flex items-center gap-1.5">
                <Activity size={13} class={ramColor} />
                Ocupação da Memória RAM
              </span>
              <span class="font-mono font-bold {ramColor}">
                {metrics.system_memory_percent}%
              </span>
            </div>
            <div class="w-full h-2 rounded-full bg-muted overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300 {metrics.system_memory_percent >= 90 ? 'bg-red-500' : metrics.system_memory_percent >= 75 ? 'bg-amber-500' : 'bg-emerald-500'}"
                style="width: {Math.min(metrics.system_memory_percent, 100)}%"
              ></div>
            </div>
          </div>
        </div>

        <!-- Placa de Vídeo (GPU) -->
        <div class="p-3.5 rounded-xl bg-muted/40 border border-border flex items-center justify-between">
          <div class="flex items-center gap-2.5">
            <div
              class="w-7 h-7 rounded-lg bg-purple-500/15 border border-purple-500/30 text-purple-700 dark:text-purple-400 flex items-center justify-center shrink-0"
            >
              <Layers size={14} />
            </div>
            <div>
              <span class="text-[11px] font-bold text-purple-700 dark:text-purple-400 uppercase tracking-wider block">
                Processador Gráfico (GPU)
              </span>
              <span class="text-xs font-semibold text-foreground">
                {metrics.gpu_model || 'GPU Integrada do Sistema'}
              </span>
            </div>
          </div>

          {#if metrics.gpu_usage_percent != null}
            <div class="text-right">
              <span class="text-[11px] text-muted-foreground block">Uso da GPU</span>
              <span class="font-mono text-xs font-bold text-purple-700 dark:text-purple-400">
                {metrics.gpu_usage_percent}%
              </span>
            </div>
          {:else if metrics.gpu_memory_used_mb != null}
            <div class="text-right">
              <span class="text-[11px] text-muted-foreground block">VRAM</span>
              <span class="font-mono text-xs font-bold text-purple-700 dark:text-purple-400">
                {formatGigabytes(metrics.gpu_memory_used_mb)} / {formatGigabytes(metrics.gpu_memory_total_mb || 0)} GB
              </span>
            </div>
          {:else}
            <span class="text-[11px] text-muted-foreground font-mono">Renderização Ativa</span>
          {/if}
        </div>

        <!-- Dica de Saúde -->
        <div class="flex items-start gap-2.5 p-3 rounded-xl bg-muted/40 border border-border text-[11px] text-muted-foreground">
          <CheckCircle2 size={15} class="text-emerald-700 dark:text-emerald-400 shrink-0 mt-0.5" />
          <p class="leading-relaxed">
            O OpenPresenter gerencia memória de forma nativa e enxuta via Rust, consumindo uma fração mínima dos recursos da sua máquina mesmo durante projeções intensivas.
          </p>
        </div>
      {:else}
        <div class="py-8 text-center text-xs text-muted-foreground">
          Carregando telemetria do sistema...
        </div>
      {/if}

      <!-- Rodapé do Diálogo -->
      <div class="flex items-center justify-between pt-2 border-t border-border">
        {#if onOpenSettingsAbout}
          <button
            type="button"
            onclick={() => {
              closeDetails();
              onOpenSettingsAbout();
            }}
            class="min-h-8 px-3 py-1.5 rounded-lg text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
          >
            Sobre o OpenPresenter
          </button>
        {:else}
          <span></span>
        {/if}
        <button
          type="button"
          onclick={closeDetails}
          class="min-h-8 px-4 py-1.5 rounded-lg text-xs font-semibold bg-muted hover:bg-muted/80 border border-border text-foreground transition-colors cursor-pointer"
        >
          Entendido
        </button>
      </div>
    </div>
  </div>
{/if}

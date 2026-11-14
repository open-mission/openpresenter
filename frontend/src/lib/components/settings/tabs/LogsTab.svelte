<script lang="ts">
  import { ScrollText, RefreshCw, Trash2, Laptop, AlertCircle } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import type { ActionLog } from '$lib/types';

  interface Props {
    actionLogs: ActionLog[];
    selectedCategory: string;
    isLoading: boolean;
    error: string;
    onSelectCategory: (category: string) => void;
    onRefresh: () => void;
    onRequestClear: () => void;
  }

  let {
    actionLogs,
    selectedCategory,
    isLoading,
    error,
    onSelectCategory,
    onRefresh,
    onRequestClear,
  }: Props = $props();

  const categories = [
    { id: 'all', label: 'Todas as Ações' },
    { id: 'presentation', label: 'Apresentação' },
    { id: 'bibles', label: 'Bíblia' },
    { id: 'songs', label: 'Músicas' },
    { id: 'setlists', label: 'Setlists' },
    { id: 'timer', label: 'Cronômetro' },
    { id: 'connections', label: 'Conexões' },
    { id: 'settings', label: 'Configurações' },
  ];

  function getCategoryBadgeClass(category: string): string {
    switch (category) {
      case 'presentation':
        return 'bg-emerald-500/15 text-emerald-700 dark:text-emerald-300 border border-emerald-500/35';
      case 'bibles':
        return 'bg-sky-500/15 text-sky-700 dark:text-sky-300 border border-sky-500/35';
      case 'songs':
        return 'bg-purple-500/15 text-purple-700 dark:text-purple-300 border border-purple-500/35';
      case 'setlists':
        return 'bg-amber-500/15 text-amber-800 dark:text-amber-300 border border-amber-500/35';
      case 'timer':
        return 'bg-teal-500/15 text-teal-700 dark:text-teal-300 border border-teal-500/35';
      case 'connections':
        return 'bg-blue-500/15 text-blue-700 dark:text-blue-300 border border-blue-500/35';
      default:
        return 'bg-muted text-muted-foreground border border-border';
    }
  }

  function getCategoryLabel(category: string): string {
    switch (category) {
      case 'presentation':
        return 'Apresentação';
      case 'bibles':
        return 'Bíblia';
      case 'songs':
        return 'Músicas';
      case 'setlists':
        return 'Setlist';
      case 'timer':
        return 'Timer';
      case 'connections':
        return 'Conexões';
      case 'settings':
        return 'Config';
      default:
        return category || 'Sistema';
    }
  }

  function formatTimestamp(ts: string): string {
    try {
      const d = new Date(ts);
      if (Number.isNaN(d.getTime())) return ts;
      return d.toLocaleString('pt-BR', {
        day: '2-digit',
        month: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
      });
    } catch {
      return ts;
    }
  }
</script>

<div class="space-y-6 w-full max-w-6xl">
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
    <div>
      <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
        <ScrollText size={16} class="text-primary" />
        <span>Histórico de Ações e Auditoria</span>
      </h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Registro de ações executadas no sistema, identificando dispositivo, instância e horário.
      </p>
    </div>

    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-xs"
        onclick={onRefresh}
        disabled={isLoading}
      >
        <RefreshCw size={13} class={isLoading ? 'animate-spin' : ''} />
        <span>Atualizar</span>
      </Button>
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-xs text-red-700 dark:text-red-300 border-red-500/35 hover:bg-red-500/10"
        onclick={onRequestClear}
        disabled={isLoading}
      >
        <Trash2 size={13} />
        <span>Limpar Logs</span>
      </Button>
    </div>
  </div>

  {#if error}
    <div
      role="alert"
      class="p-3 rounded-lg bg-red-500/10 border border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-center justify-between gap-3"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertCircle size={14} class="shrink-0" />
        <span>{error}</span>
      </div>
      <Button variant="outline" size="sm" onclick={onRefresh} class="h-7 text-xs shrink-0">
        Tentar novamente
      </Button>
    </div>
  {/if}

  <!-- Filtro de Categorias -->
  <div class="flex items-center gap-1.5 overflow-x-auto pb-1 text-xs" role="group" aria-label="Filtrar logs por categoria">
    {#each categories as cat (cat.id)}
      <button
        type="button"
        aria-pressed={selectedCategory === cat.id}
        onclick={() => onSelectCategory(cat.id)}
        class="min-h-7 px-3 py-1 rounded-md text-xs font-medium transition-colors shrink-0 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {selectedCategory === cat.id ? 'bg-primary text-primary-foreground font-semibold' : 'bg-muted text-muted-foreground hover:text-foreground hover:bg-muted/80'}"
      >
        {cat.label}
      </button>
    {/each}
  </div>

  <!-- Lista de Logs -->
  <div class="rounded-xl border border-border bg-card/60 overflow-hidden">
    {#if isLoading}
      <div class="p-8 text-center text-muted-foreground text-xs">
        <RefreshCw size={18} class="animate-spin mx-auto mb-2" />
        <span>Carregando histórico de ações...</span>
      </div>
    {:else if actionLogs.length === 0}
      <div class="p-8 text-center text-muted-foreground text-xs">
        <ScrollText size={24} class="opacity-40 mx-auto mb-2" />
        <p class="font-medium text-foreground">Nenhum registro de log encontrado</p>
        <p class="text-[11px] text-muted-foreground mt-0.5">As ações realizadas no sistema aparecerão aqui.</p>
      </div>
    {:else}
      <div class="divide-y divide-border max-h-[500px] overflow-y-auto">
        {#each actionLogs as log (log.id)}
          <div class="p-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-2.5 hover:bg-muted/50 transition-colors">
            <div class="flex items-start gap-3">
              <span class="mt-0.5 px-2 py-0.5 rounded text-[11px] font-bold uppercase tracking-wider shrink-0 {getCategoryBadgeClass(log.category)}">
                {getCategoryLabel(log.category)}
              </span>
              <div>
                <p class="text-xs font-semibold text-foreground">{log.details}</p>
                <div class="flex flex-wrap items-center gap-2 mt-1 text-[11px] text-muted-foreground">
                  {#if log.device_info}
                    <span class="flex items-center gap-1 font-mono text-foreground/80">
                      <Laptop size={11} />
                      {log.device_info}
                    </span>
                  {/if}
                  {#if log.ip_address}
                    <span>• IP: {log.ip_address}</span>
                  {/if}
                  {#if log.client_type}
                    <span>• Tipo: {log.client_type}</span>
                  {/if}
                </div>
              </div>
            </div>

            <div class="text-[11px] font-mono text-muted-foreground shrink-0 self-end sm:self-center">
              {formatTimestamp(log.timestamp)}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

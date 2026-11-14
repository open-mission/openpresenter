<script lang="ts">
  import { CalendarDays, Maximize2, Minimize2, X } from 'lucide-svelte';
  import Badge from '../ui/Badge.svelte';

  interface Props {
    isEdit: boolean;
    eventCount: number;
    isMaximized: boolean;
    ontogglemaximize: () => void;
    onclose: () => void;
  }

  let { isEdit, eventCount, isMaximized, ontogglemaximize, onclose }: Props = $props();
</script>

<div
  class="h-16 px-6 border-b border-border flex items-center justify-between shrink-0 bg-card select-none"
>
  <div class="flex items-center gap-3 min-w-0">
    <div
      class="w-10 h-10 rounded-xl bg-amber-500/15 border border-amber-600/35 text-amber-700 dark:text-amber-300 flex items-center justify-center shrink-0"
      aria-hidden="true"
    >
      <CalendarDays size={20} />
    </div>
    <div class="min-w-0">
      <div class="flex items-center gap-2">
        <h2 id="schedule-editor-title" class="text-sm sm:text-base font-bold text-foreground truncate">
          {isEdit ? 'Editar Programação / Agenda' : 'Nova Programação / Agenda'}
        </h2>
        <Badge
          variant="outline"
          class="text-[11px] font-mono text-amber-800 dark:text-amber-300 border-amber-600/35 bg-amber-500/10 shrink-0"
        >
          {eventCount} {eventCount === 1 ? 'evento' : 'eventos'}
        </Badge>
      </div>
      <p id="schedule-editor-description" class="text-xs text-muted-foreground truncate">
        Cadastre os cultos da semana ou eventos futuros. Cada dia vira um slide conectado à tela pública e ao palco.
      </p>
    </div>
  </div>

  <div class="flex items-center gap-1.5 shrink-0">
    <span class="text-[11px] text-muted-foreground hidden md:inline mr-1">
      Salvar:
      <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-muted-foreground font-mono text-[11px]">⌘Enter</kbd>
      /
      <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-muted-foreground font-mono text-[11px]">Ctrl+Enter</kbd>
    </span>

    <button
      type="button"
      onclick={ontogglemaximize}
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
      aria-label={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
    >
      {#if isMaximized}
        <Minimize2 size={16} aria-hidden="true" />
      {:else}
        <Maximize2 size={16} aria-hidden="true" />
      {/if}
    </button>

    <button
      type="button"
      onclick={onclose}
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title="Fechar (Esc)"
      aria-label="Fechar editor de agenda"
    >
      <X size={18} aria-hidden="true" />
    </button>
  </div>
</div>

<script lang="ts">
  import type { CommandTab } from './types';

  interface Props {
    activeTab: CommandTab;
  }

  let { activeTab }: Props = $props();

  const primaryAction: Record<CommandTab, string> = {
    local: 'Adicionar',
    bible: 'Projetar',
    online: 'Importar',
  };

  const kbdClass =
    'rounded border border-border bg-background px-1.5 py-0.5 font-mono text-[11px] text-foreground';
</script>

<div
  class="flex min-h-9 shrink-0 flex-wrap items-center justify-between gap-x-3 gap-y-1 border-t border-border bg-muted/30 px-4 py-1.5 text-[11px] text-muted-foreground"
>
  <div class="flex flex-wrap items-center gap-3">
    <span class="flex items-center gap-1"><kbd class={kbdClass}>↑↓</kbd> Navegar</span>
    <span class="flex items-center gap-1">
      <kbd class={kbdClass}>↵</kbd>
      {primaryAction[activeTab]}
    </span>
    <span class="flex items-center gap-1"><kbd class={kbdClass}>⇧↵</kbd> Setlist</span>
    <span class="hidden items-center gap-1 md:flex"><kbd class={kbdClass}>Ctrl+1..3</kbd> Abas</span>
  </div>
  <div class="flex flex-wrap items-center gap-3">
    <span class="hidden items-center gap-1 sm:flex"><kbd class={kbdClass}>Tab</kbd> Controles</span>
    <span class="flex items-center gap-1"><kbd class={kbdClass}>Esc</kbd> Fechar</span>
  </div>
</div>

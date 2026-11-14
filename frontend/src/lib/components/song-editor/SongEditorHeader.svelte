<script lang="ts">
  import { FileText, Maximize2, Minimize2, Music, X } from 'lucide-svelte';
  import Badge from '../ui/Badge.svelte';
  import type { SongEditorMode } from './types';

  interface Props {
    mode: SongEditorMode;
    title: string;
    slideCount: number;
    isDirty: boolean;
    isMaximized: boolean;
    ontogglemaximize: () => void;
    onclose: () => void;
  }

  let { mode, title, slideCount, isDirty, isMaximized, ontogglemaximize, onclose }: Props =
    $props();

  const displayTitle = $derived(
    title.trim() || (mode === 'song' ? 'Nova Música' : 'Nova Apresentação de Texto')
  );
</script>

<header class="h-14 px-4 sm:px-5 border-b border-border bg-card flex items-center justify-between gap-3 shrink-0">
  <div class="flex items-center gap-3 min-w-0">
    <div
      class="w-8 h-8 rounded-lg border flex items-center justify-center shrink-0 {mode === 'song'
        ? 'bg-emerald-500/15 border-emerald-600/35 text-emerald-700 dark:text-emerald-300'
        : 'bg-amber-500/15 border-amber-600/35 text-amber-700 dark:text-amber-300'}"
      aria-hidden="true"
    >
      {#if mode === 'song'}
        <Music size={18} />
      {:else}
        <FileText size={18} />
      {/if}
    </div>

    <div class="min-w-0">
      <div class="flex items-center gap-2 min-w-0">
        <h2 class="text-sm sm:text-base font-semibold tracking-tight text-foreground truncate">
          {displayTitle}
        </h2>
        <Badge variant="outline" class="text-[11px] text-muted-foreground font-mono shrink-0">
          {slideCount} slides
        </Badge>
        {#if isDirty}
          <Badge
            variant="outline"
            class="text-[11px] text-amber-700 dark:text-amber-300 border-amber-600/40 bg-amber-500/10 shrink-0"
            title="Existem alterações não salvas"
          >
            Não salvo
          </Badge>
        {/if}
      </div>
      <p class="text-[11px] text-muted-foreground truncate">
        {mode === 'song'
          ? 'Editor de letras e partes no padrão Holyrics e ProPresenter'
          : 'Editor de slides rápidos, avisos e comunicados com formatação rica'}
      </p>
    </div>
  </div>

  <div class="flex items-center gap-1.5 shrink-0">
    <span class="text-[11px] text-muted-foreground hidden md:inline">
      Salvar:
      <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-muted-foreground font-mono text-[11px]">⌘S</kbd>
      /
      <kbd class="px-1.5 py-0.5 rounded bg-muted border border-border text-muted-foreground font-mono text-[11px]">Ctrl+S</kbd>
    </span>

    <button
      type="button"
      onclick={ontogglemaximize}
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
      aria-label={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
    >
      {#if isMaximized}
        <Minimize2 size={16} />
      {:else}
        <Maximize2 size={16} />
      {/if}
    </button>

    <button
      type="button"
      onclick={onclose}
      class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      aria-label="Fechar editor"
      title="Fechar (Esc)"
    >
      <X size={18} />
    </button>
  </div>
</header>

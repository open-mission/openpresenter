<script lang="ts">
  import { FileText, Music, Sparkles } from 'lucide-svelte';
  import { renderFormattedText } from '$lib/markdown';
  import { getSectionColor } from '../../songParser';
  import type { Slide } from '$lib/types';
  import type { AlignH, AlignV, SongEditorMode } from './types';

  interface Props {
    mode: SongEditorMode;
    slides: Slide[];
    alignH: AlignH;
    alignV: AlignV;
    onfocusblock: (slideIdx: number) => void;
    onslidecontextmenu: (event: MouseEvent, slide: Slide, slideIdx: number) => void;
  }

  let { mode, slides, alignH, alignV, onfocusblock, onslidecontextmenu }: Props = $props();

  const verticalClass = $derived(
    alignV === 'top' ? 'justify-start' : alignV === 'bottom' ? 'justify-end' : 'justify-center'
  );
  const horizontalClass = $derived(
    alignH === 'left'
      ? 'text-left items-start'
      : alignH === 'right'
        ? 'text-right items-end'
        : alignH === 'justify'
          ? 'text-justify items-stretch'
          : 'text-center items-center'
  );

  const previewFormatOptions = {
    strongClass: 'font-bold text-foreground',
    markClass:
      'bg-yellow-400/30 text-yellow-900 dark:text-yellow-200 px-1 py-0.5 rounded font-semibold',
  };
</script>

<div class="w-full lg:w-1/2 flex flex-col overflow-hidden bg-muted/20 min-h-0">
  <div
    class="h-10 px-4 border-b border-border bg-muted/30 flex items-center justify-between gap-3 shrink-0"
  >
    <span class="text-xs font-semibold text-foreground flex items-center gap-2 min-w-0">
      <Sparkles size={14} class="text-primary shrink-0" aria-hidden="true" />
      <span class="truncate">Prévia dos Slides ({slides.length})</span>
    </span>
    <div class="flex items-center gap-2 text-[11px] text-muted-foreground font-mono shrink-0">
      <span class="hidden sm:inline">Alinhamento: {alignH} / {alignV}</span>
      <span aria-hidden="true" class="hidden sm:inline">•</span>
      <span>16:9</span>
    </div>
  </div>

  <div class="flex-1 min-h-0 p-4 overflow-y-auto space-y-4">
    {#if slides.length === 0}
      <div
        class="min-h-64 h-full flex flex-col items-center justify-center text-center text-muted-foreground text-xs border border-dashed border-border rounded-xl p-6"
      >
        {#if mode === 'song'}
          <Music size={28} class="mb-2 opacity-40 text-primary" aria-hidden="true" />
        {:else}
          <FileText size={28} class="mb-2 opacity-40 text-amber-700 dark:text-amber-300" aria-hidden="true" />
        {/if}
        <p>Nenhum slide gerado ainda.</p>
        <p class="text-[11px] mt-1">
          Digite o conteúdo no painel esquerdo para visualizar os slides aqui.
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
        {#each slides as slide, idx (slide.id)}
          {@const color = getSectionColor(slide.label)}
          <article class="group relative">
            <button
              type="button"
              onclick={() => onfocusblock(idx)}
              oncontextmenu={(event) => onslidecontextmenu(event, slide, idx)}
              class="w-full aspect-video bg-card text-card-foreground rounded-lg border border-border overflow-hidden flex flex-col shadow-sm transition-colors hover:border-primary/60 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer text-left"
              title="Clique para editar este trecho; botão direito para mudar seção, duplicar ou excluir slide"
              aria-label={`Editar texto do slide ${idx + 1}: ${slide.label}`}
            >
              <span
                class="h-6 px-2.5 flex items-center justify-between border-b border-border/70 text-[11px] font-medium {color.bg} shrink-0"
              >
                <span class="flex items-center gap-1.5 min-w-0">
                  <span class="w-2 h-2 rounded-full shrink-0 {color.accent}" aria-hidden="true"></span>
                  <span class="font-semibold truncate {color.text}">{slide.label}</span>
                </span>
                <span class="text-muted-foreground font-mono shrink-0">#{idx + 1}</span>
              </span>

              <span class="flex-1 min-h-0 p-3 flex flex-col {verticalClass} {horizontalClass}">
                <span
                  class="block text-xs sm:text-sm font-semibold text-foreground tracking-wide leading-snug drop-shadow-sm line-clamp-5 max-w-full"
                >
                  {@html renderFormattedText(slide.content, previewFormatOptions)}
                </span>
              </span>
            </button>
          </article>
        {/each}
      </div>
    {/if}
  </div>
</div>

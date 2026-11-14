<script lang="ts">
  import {
    AlignCenter,
    AlignJustify,
    AlignLeft,
    AlignRight,
    AlignVerticalJustifyCenter,
    AlignVerticalJustifyEnd,
    AlignVerticalJustifyStart,
    Bold,
    ChevronDown,
    Hash,
    Highlighter,
    Italic,
    Rows2,
    Underline as UnderlineIcon,
  } from 'lucide-svelte';
  import type { AlignH, AlignV, SongEditorMode } from './types';

  interface Props {
    mode: SongEditorMode;
    alignH?: AlignH;
    alignV?: AlignV;
    onbold: () => void;
    onitalic: () => void;
    onunderline: () => void;
    onhighlight: () => void;
    onformatlines: () => void;
    oninsertsection: (label: string) => void;
    onopensectionmenu: (event: MouseEvent) => void;
  }

  let {
    mode,
    alignH = $bindable('center'),
    alignV = $bindable('center'),
    onbold,
    onitalic,
    onunderline,
    onhighlight,
    onformatlines,
    oninsertsection,
    onopensectionmenu,
  }: Props = $props();

  const iconButtonClass =
    'w-7 h-7 rounded-md bg-muted hover:bg-secondary text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring';
  const textButtonClass =
    'h-7 px-2 rounded-md bg-muted hover:bg-secondary text-muted-foreground hover:text-foreground text-[11px] font-medium flex items-center gap-1.5 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring shrink-0';

  function alignClass(active: boolean): string {
    return active
      ? 'bg-secondary text-foreground'
      : 'text-muted-foreground hover:text-foreground';
  }
</script>

<div
  class="px-3 py-2 border-b border-border bg-muted/30 flex flex-wrap items-center gap-2 text-xs shrink-0 select-none"
  role="toolbar"
  aria-label="Formatação da letra"
>
  <div class="flex items-center gap-1 shrink-0">
    <button type="button" onclick={onbold} class={iconButtonClass} title="Negrito (**texto**)" aria-label="Aplicar negrito">
      <Bold size={13} />
    </button>
    <button type="button" onclick={onitalic} class={iconButtonClass} title="Itálico (*texto*)" aria-label="Aplicar itálico">
      <Italic size={13} />
    </button>
    <button
      type="button"
      onclick={onunderline}
      class={iconButtonClass}
      title="Sublinhado (<u>texto</u>)"
      aria-label="Aplicar sublinhado"
    >
      <UnderlineIcon size={13} />
    </button>
    <button
      type="button"
      onclick={onhighlight}
      class="{iconButtonClass} text-amber-700 hover:text-amber-800 dark:text-amber-300 dark:hover:text-amber-200"
      title="Marca-texto / Highlight (==texto==)"
      aria-label="Aplicar marca-texto"
    >
      <Highlighter size={13} />
    </button>
  </div>

  <span class="h-5 w-px bg-border shrink-0" aria-hidden="true"></span>

  <div class="flex items-center gap-2 shrink-0">
    <div class="flex items-center rounded-md bg-background border border-border p-0.5" role="group" aria-label="Alinhamento horizontal">
      <button
        type="button"
        onclick={() => (alignH = 'left')}
        class="w-7 h-7 rounded flex items-center justify-center transition-colors cursor-pointer {alignClass(alignH === 'left')}"
        title="Alinhar à Esquerda"
        aria-label="Alinhar à esquerda"
        aria-pressed={alignH === 'left'}
      >
        <AlignLeft size={12} />
      </button>
      <button
        type="button"
        onclick={() => (alignH = 'center')}
        class="w-7 h-7 rounded flex items-center justify-center transition-colors cursor-pointer {alignClass(alignH === 'center')}"
        title="Centralizar Horizontalmente"
        aria-label="Centralizar horizontalmente"
        aria-pressed={alignH === 'center'}
      >
        <AlignCenter size={12} />
      </button>
      <button
        type="button"
        onclick={() => (alignH = 'right')}
        class="w-7 h-7 rounded flex items-center justify-center transition-colors cursor-pointer {alignClass(alignH === 'right')}"
        title="Alinhar à Direita"
        aria-label="Alinhar à direita"
        aria-pressed={alignH === 'right'}
      >
        <AlignRight size={12} />
      </button>
      <button
        type="button"
        onclick={() => (alignH = 'justify')}
        class="w-7 h-7 rounded flex items-center justify-center transition-colors cursor-pointer {alignClass(alignH === 'justify')}"
        title="Justificado"
        aria-label="Justificar texto"
        aria-pressed={alignH === 'justify'}
      >
        <AlignJustify size={12} />
      </button>
    </div>

    <div class="flex items-center rounded-md bg-background border border-border p-0.5" role="group" aria-label="Alinhamento vertical">
      <button
        type="button"
        onclick={() => (alignV = 'top')}
        class="w-7 h-7 rounded flex items-center justify-center transition-colors cursor-pointer {alignClass(alignV === 'top')}"
        title="Alinhar ao Topo"
        aria-label="Alinhar ao topo"
        aria-pressed={alignV === 'top'}
      >
        <AlignVerticalJustifyStart size={12} />
      </button>
      <button
        type="button"
        onclick={() => (alignV = 'center')}
        class="w-7 h-7 rounded flex items-center justify-center transition-colors cursor-pointer {alignClass(alignV === 'center')}"
        title="Centralizar Verticalmente"
        aria-label="Centralizar verticalmente"
        aria-pressed={alignV === 'center'}
      >
        <AlignVerticalJustifyCenter size={12} />
      </button>
      <button
        type="button"
        onclick={() => (alignV = 'bottom')}
        class="w-7 h-7 rounded flex items-center justify-center transition-colors cursor-pointer {alignClass(alignV === 'bottom')}"
        title="Alinhar à Base"
        aria-label="Alinhar à base"
        aria-pressed={alignV === 'bottom'}
      >
        <AlignVerticalJustifyEnd size={12} />
      </button>
    </div>
  </div>

  <span class="h-5 w-px bg-border shrink-0" aria-hidden="true"></span>

  {#if mode === 'song'}
    <div class="flex items-center gap-1 shrink-0">
      {#each ['Verso', 'Coro', 'Ponte'] as section (section)}
        <button
          type="button"
          onclick={() => oninsertsection(section)}
          class={textButtonClass}
          title={`Inserir seção ## ${section} no cursor`}
          aria-label={`Inserir seção ${section} no cursor`}
        >
          <span class="font-mono text-muted-foreground" aria-hidden="true">##</span>
          <span>{section}</span>
        </button>
      {/each}
    </div>
  {/if}

  {#if mode === 'song'}
    <button
      type="button"
      onclick={onformatlines}
      class={textButtonClass}
      title="Formatar em 2 linhas por slide preservando seções"
      aria-label="Formatar em 2 linhas por slide"
    >
      <Rows2 size={13} class="text-emerald-700 dark:text-emerald-300" aria-hidden="true" />
      <span>2 Linhas/Slide</span>
    </button>
  {/if}

  <button
    type="button"
    onclick={onopensectionmenu}
    class={textButtonClass}
    title="Inserir tag de seção no cursor"
    aria-label="Abrir menu de inserção de seção"
    aria-haspopup="menu"
  >
    <Hash size={13} class="text-primary" aria-hidden="true" />
    <span>+ Seção</span>
    <ChevronDown size={11} class="text-muted-foreground" aria-hidden="true" />
  </button>
</div>

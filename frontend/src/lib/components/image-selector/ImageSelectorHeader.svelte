<script lang="ts">
  import { ImageIcon, Maximize2, Minimize2, X } from 'lucide-svelte';
  import {
    DEFAULT_IMAGE_SELECTOR_TITLE,
    DEFAULT_IMAGE_SELECTOR_SUBTITLE,
    DEFAULT_LOWER_THIRD_TITLE,
    DEFAULT_LOWER_THIRD_SUBTITLE,
  } from './defaults';

  interface Props {
    title?: string;
    subtitle?: string;
    isLowerThird: boolean;
    isMaximized: boolean;
    ontogglemaximize: () => void;
    onclose: () => void;
  }

  let {
    title,
    subtitle,
    isLowerThird,
    isMaximized,
    ontogglemaximize,
    onclose,
  }: Props = $props();

  const displayTitle = $derived(
    title || (isLowerThird ? DEFAULT_LOWER_THIRD_TITLE : DEFAULT_IMAGE_SELECTOR_TITLE)
  );

  const displaySubtitle = $derived(
    subtitle || (isLowerThird ? DEFAULT_LOWER_THIRD_SUBTITLE : DEFAULT_IMAGE_SELECTOR_SUBTITLE)
  );
</script>

<div class="h-16 px-5 border-b border-border bg-card/60 flex items-center justify-between shrink-0 select-none">
  <div class="flex items-center gap-3 min-w-0">
    <div
      class="w-10 h-10 rounded-xl bg-primary/10 border border-primary/25 text-primary flex items-center justify-center shrink-0"
      aria-hidden="true"
    >
      <ImageIcon size={20} />
    </div>
    <div class="min-w-0">
      <h2 id="image-selector-title" class="text-sm sm:text-base font-bold text-foreground truncate">
        {displayTitle}
      </h2>
      <p id="image-selector-desc" class="text-xs text-muted-foreground truncate">
        {displaySubtitle}
      </p>
    </div>
  </div>

  <div class="flex items-center gap-1.5 shrink-0">
    <button
      type="button"
      onclick={ontogglemaximize}
      class="w-8 h-8 rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
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
      class="w-8 h-8 rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title="Fechar (Esc)"
      aria-label="Fechar seletor de imagens"
    >
      <X size={18} aria-hidden="true" />
    </button>
  </div>
</div>

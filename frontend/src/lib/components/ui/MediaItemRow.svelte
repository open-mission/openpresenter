<script lang="ts">
  import { cn } from '$lib/utils';
  import Badge from './Badge.svelte';
  import {
    Music,
    BookOpen,
    Bell,
    Presentation,
    ChevronRight,
    Bookmark,
    GripVertical,
    ImageIcon,
    FileText,
  } from 'lucide-svelte';
  import type { MediaItem } from '$lib/types';

  interface Props {
    item: MediaItem;
    index?: number;
    selected?: boolean;
    isLive?: boolean;
    onclick?: () => void;
    oncontextmenu?: (e: MouseEvent) => void;
    ondragstartitem?: (e: DragEvent, index: number) => void;
    ondragoveritem?: (e: DragEvent, index: number) => void;
    ondropitem?: (e: DragEvent, index: number) => void;
  }

  let {
    item,
    index = 0,
    selected = false,
    isLive = false,
    onclick,
    oncontextmenu,
    ondragstartitem,
    ondragoveritem,
    ondropitem,
  }: Props = $props();

  let isDragOver = $state(false);

  const kindIcons = {
    song: Music,
    scripture: BookOpen,
    announcement: Bell,
    media: Presentation,
    header: Bookmark,
    image: ImageIcon,
    text: FileText,
  };

  const Icon = $derived(kindIcons[item.kind] ?? Presentation);

  function handleDragStart(e: DragEvent) {
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', String(index));
    }
    ondragstartitem?.(e, index);
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'move';
    }
    isDragOver = true;
    ondragoveritem?.(e, index);
  }

  function handleDragLeave() {
    isDragOver = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    ondropitem?.(e, index);
  }
</script>

{#if item.kind === 'header'}
  <!-- Divisor de Seção na Setlist -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    draggable="true"
    ondragstart={handleDragStart}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    class={cn(
      'pt-3 pb-1 px-1 flex items-center justify-between gap-2 group select-none cursor-grab active:cursor-grabbing transition-all',
      isDragOver && 'border-t-2 border-primary bg-primary/10'
    )}
    oncontextmenu={(e) => {
      e.preventDefault();
      oncontextmenu?.(e);
    }}
    role="separator"
  >
    <div class="flex items-center gap-1.5 min-w-0">
      <GripVertical size={13} class="text-neutral-600 group-hover:text-neutral-400 shrink-0 opacity-60 group-hover:opacity-100" />
      <div class="w-4 h-4 rounded text-emerald-400 flex items-center justify-center">
        <Bookmark size={13} />
      </div>
      <span class="text-[11px] font-bold uppercase tracking-wider text-neutral-400 group-hover:text-neutral-200 truncate">
        {item.title}
      </span>
    </div>
    <div class="h-[1px] flex-1 bg-neutral-800/80 group-hover:bg-neutral-700 transition-colors"></div>
  </div>
{:else}
  <!-- Item Normal de Mídia com Drag Handle e Drop Zone -->
  <div
    draggable="true"
    ondragstart={handleDragStart}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    class={cn(
      'w-full text-left p-2.5 rounded-lg border transition-all duration-150 flex items-center justify-between gap-2 group relative overflow-hidden select-none',
      isDragOver ? 'border-primary ring-2 ring-primary/40 bg-neutral-800' : '',
      isLive
        ? 'border-emerald-500 ring-1 ring-emerald-500/50 bg-emerald-950/20 text-neutral-100 shadow-sm'
        : selected
          ? 'bg-neutral-800/90 border-primary/50 shadow-sm text-neutral-100 ring-1 ring-primary/30'
          : 'bg-neutral-900/40 border-neutral-800/60 hover:bg-neutral-850 hover:border-neutral-700 text-neutral-300'
    )}
    oncontextmenu={(e) => {
      e.preventDefault();
      oncontextmenu?.(e);
    }}
    role="button"
    tabindex="0"
    {onclick}
    onkeydown={(e) => {
      if (e.key === 'Enter') onclick?.();
    }}
  >
    {#if isLive}
      <div class="absolute left-0 top-0 bottom-0 w-1 bg-emerald-500"></div>
    {:else if selected}
      <div class="absolute left-0 top-0 bottom-0 w-1 bg-primary"></div>
    {/if}

    <div class="flex items-center gap-2.5 min-w-0 flex-1">
      <!-- Drag Handle -->
      <div
        class="cursor-grab active:cursor-grabbing text-neutral-600 group-hover:text-neutral-400 p-0.5 hover:bg-neutral-800 rounded transition-colors shrink-0"
        title="Arrastar para reordenar"
      >
        <GripVertical size={14} />
      </div>

      <div
        class={cn(
          'w-8 h-8 rounded-md flex items-center justify-center shrink-0 border transition-colors',
          isLive
            ? 'bg-emerald-500/15 border-emerald-500/40 text-emerald-400'
            : selected
              ? 'bg-primary/10 border-primary/30 text-primary'
              : 'bg-neutral-800/80 border-neutral-700/50 text-neutral-400 group-hover:text-neutral-200'
        )}
      >
        <Icon size={16} />
      </div>

      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-1.5">
          <h4 class="text-sm font-medium truncate {isLive ? 'text-white font-semibold' : 'text-neutral-100'} leading-tight">
            {item.title}
          </h4>
        </div>
        {#if item.artist_or_ref}
          <p class="text-xs text-neutral-400 truncate mt-0.5">
            {item.artist_or_ref}
          </p>
        {/if}
      </div>
    </div>

    <div class="flex items-center gap-1.5 shrink-0">
      {#if isLive}
        <Badge variant="success" class="text-[10px] uppercase font-bold tracking-wider py-0 px-1.5 bg-emerald-500/20 text-emerald-300 border-emerald-500/40">
          Ao Vivo
        </Badge>
      {/if}
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5">
        {item.slides.length} {item.slides.length === 1 ? 'slide' : 'slides'}
      </Badge>
      <ChevronRight
        size={14}
        class={cn(
          'transition-transform text-neutral-500 group-hover:text-neutral-300',
          isLive ? 'text-emerald-400' : selected ? 'text-primary translate-x-0.5' : ''
        )}
      />
    </div>
  </div>
{/if}

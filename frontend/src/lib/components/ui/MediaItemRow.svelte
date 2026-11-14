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
    CalendarDays,
    Video,
    Volume2,
    Timer,
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
    onmoveup?: () => void;
    onmovedown?: () => void;
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
    onmoveup,
    onmovedown,
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
    schedule: CalendarDays,
    audio: Volume2,
    video: Video,
    timer: Timer,
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
  <div
    draggable="true"
    ondragstart={handleDragStart}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    class={cn(
      'pt-3 pb-1 px-1 flex items-center justify-between gap-2 group select-none cursor-grab active:cursor-grabbing transition-all rounded-md',
      isDragOver && 'border-t-2 border-primary bg-primary/10'
    )}
    oncontextmenu={(e) => {
      e.preventDefault();
      oncontextmenu?.(e);
    }}
    role="group"
    aria-label={`Seção: ${item.title}`}
  >
    <div class="flex items-center gap-1.5 min-w-0">
      <GripVertical size={13} class="text-muted-foreground group-hover:text-foreground shrink-0 opacity-70 group-hover:opacity-100" aria-hidden="true" />
      <div class="w-4 h-4 rounded text-emerald-700 dark:text-emerald-400 flex items-center justify-center">
        <Bookmark size={13} />
      </div>
      <span class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground group-hover:text-foreground truncate">
        {item.title}
      </span>
    </div>
    <div class="h-[1px] flex-1 bg-border transition-colors"></div>
  </div>
{:else}
  <!-- Item Normal de Mídia com Drag Handle e Drop Zone -->
  <div
    draggable="true"
    ondragstart={handleDragStart}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    role="listitem"
    oncontextmenu={(e) => {
      e.preventDefault();
      oncontextmenu?.(e);
    }}
    class={cn(
      'w-full rounded-lg border transition-all duration-150 group relative overflow-hidden select-none',
      isDragOver ? 'border-primary ring-2 ring-primary/40 bg-muted' : '',
      isLive
        ? 'border-emerald-600 dark:border-emerald-500 ring-1 ring-emerald-500/40 bg-emerald-500/12 text-foreground shadow-sm'
        : selected
          ? 'bg-muted border-primary/50 shadow-sm text-foreground ring-1 ring-primary/30'
          : 'bg-card/70 border-border hover:bg-muted/60 hover:border-foreground/25 text-foreground'
    )}
  >
    <button
      type="button"
      {onclick}
      aria-current={isLive ? 'true' : undefined}
      onkeydown={(e) => {
        if (e.altKey && e.key === 'ArrowUp' && onmoveup) {
          e.preventDefault();
          onmoveup();
        } else if (e.altKey && e.key === 'ArrowDown' && onmovedown) {
          e.preventDefault();
          onmovedown();
        }
      }}
      class="w-full text-left p-2.5 flex flex-col gap-1.5 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring rounded-lg"
    >
      <!-- Linha 1: Drag handle, Ícone, Título/Artista e Indicador Ao Vivo + Chevron -->
      <div class="flex items-start justify-between gap-2 min-w-0 w-full">
        <div class="flex items-center gap-2 min-w-0 flex-1">
          <!-- Drag Handle -->
          <span
            class="cursor-grab active:cursor-grabbing text-muted-foreground group-hover:text-foreground p-0.5 rounded transition-colors shrink-0"
            title="Arraste ou pressione Alt+↑/↓ para reordenar"
            aria-hidden="true"
          >
            <GripVertical size={13} />
          </span>

          <div
            class={cn(
              'w-7 h-7 rounded-md flex items-center justify-center shrink-0 border transition-colors',
              isLive
                ? 'bg-emerald-500/15 border-emerald-600/40 dark:border-emerald-500/40 text-emerald-700 dark:text-emerald-400'
                : selected
                  ? 'bg-primary/10 border-primary/30 text-primary'
                  : 'bg-muted border-border text-muted-foreground group-hover:text-foreground'
            )}
          >
            <Icon size={14} />
          </div>

          <div class="min-w-0 flex-1">
            <h4 class="text-xs font-bold truncate text-foreground leading-tight">
              {item.title}
            </h4>
            {#if item.artist_or_ref}
              <p class="text-[11px] text-muted-foreground truncate mt-0.5 leading-tight">
                {item.artist_or_ref}
              </p>
            {/if}
          </div>
        </div>

        <div class="flex items-center gap-1 shrink-0 pt-0.5">
          {#if isLive}
            <Badge variant="success" class="text-[11px] uppercase font-bold tracking-wider py-0.5 px-2">
              Ao Vivo
            </Badge>
          {/if}
          <ChevronRight
            size={14}
            class={cn(
              'transition-transform text-muted-foreground group-hover:text-foreground',
              isLive ? 'text-emerald-700 dark:text-emerald-400' : selected ? 'text-primary translate-x-0.5' : ''
            )}
          />
        </div>
      </div>

      <!-- Linha 2: Badges informativos (Playback, Lower, Quantidade de slides) organizados sem encavalar -->
      <div class="flex items-center justify-between gap-1.5 pl-8 w-full text-[11px]">
        <div class="flex items-center gap-1 flex-wrap min-w-0">
          {#if item.audio_url}
            <Badge
              variant="success"
              class="text-[11px] py-0.5 px-1.5 gap-1 flex items-center shrink-0 font-medium"
              title="Possui áudio / playback vinculado"
            >
              <Music size={11} />
              <span>Playback</span>
            </Badge>
          {/if}
          {#if item.lower_image_url}
            <Badge
              variant="success"
              class="text-[11px] py-0.5 px-1.5 gap-1 flex items-center shrink-0 font-medium"
              title="Possui imagem alternativa para Lower Third"
            >
              <ImageIcon size={11} />
              <span>Lower</span>
            </Badge>
          {/if}
        </div>

        <Badge variant="secondary" class="text-[11px] py-0.5 px-2 shrink-0 font-mono">
          {item.slides.length} {item.slides.length === 1 ? 'slide' : 'slides'}
        </Badge>
      </div>
    </button>
  </div>
{/if}

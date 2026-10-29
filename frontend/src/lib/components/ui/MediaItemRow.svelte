<script lang="ts">
  import { cn } from '$lib/utils';
  import Badge from './Badge.svelte';
  import { Music, BookOpen, Bell, Presentation, ChevronRight, Bookmark } from 'lucide-svelte';
  import type { MediaItem } from '$lib/types';

  interface Props {
    item: MediaItem;
    selected?: boolean;
    isLive?: boolean;
    onclick?: () => void;
    oncontextmenu?: (e: MouseEvent) => void;
  }

  let { item, selected = false, isLive = false, onclick, oncontextmenu }: Props = $props();

  const kindIcons = {
    song: Music,
    scripture: BookOpen,
    announcement: Bell,
    media: Presentation,
    header: Bookmark,
  };

  const Icon = $derived(kindIcons[item.kind] ?? Presentation);
</script>

{#if item.kind === 'header'}
  <!-- Divisor de Seção na Setlist -->
  <div
    class="pt-3 pb-1 px-1 flex items-center justify-between gap-2 group select-none cursor-pointer"
    oncontextmenu={(e) => {
      e.preventDefault();
      oncontextmenu?.(e);
    }}
    role="separator"
  >
    <div class="flex items-center gap-1.5 min-w-0">
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
  <!-- Item Normal de Mídia -->
  <button
    type="button"
    {onclick}
    oncontextmenu={(e) => {
      e.preventDefault();
      oncontextmenu?.(e);
    }}
    class={cn(
      'w-full text-left p-2.5 rounded-lg border transition-all duration-150 flex items-center justify-between gap-3 group cursor-pointer relative overflow-hidden',
      selected
        ? 'bg-neutral-800/90 border-emerald-500/50 shadow-sm text-neutral-100 ring-1 ring-emerald-500/30'
        : 'bg-neutral-900/40 border-neutral-800/60 hover:bg-neutral-850 hover:border-neutral-700 text-neutral-300'
    )}
  >
    {#if selected}
      <div class="absolute left-0 top-0 bottom-0 w-1 bg-emerald-500"></div>
    {/if}

    <div class="flex items-center gap-3 min-w-0">
      <div
        class={cn(
          'w-8 h-8 rounded-md flex items-center justify-center shrink-0 border transition-colors',
          selected
            ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-400'
            : 'bg-neutral-800/80 border-neutral-700/50 text-neutral-400 group-hover:text-neutral-200'
        )}
      >
        <Icon size={16} />
      </div>

      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-1.5">
          <h4 class="text-sm font-medium truncate text-neutral-100 leading-tight">
            {item.title}
          </h4>
          {#if isLive}
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-ping"></span>
          {/if}
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
        <Badge variant="live" class="text-[10px] uppercase tracking-wider py-0 px-1.5">
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
          selected ? 'text-emerald-400 translate-x-0.5' : ''
        )}
      />
    </div>
  </button>
{/if}

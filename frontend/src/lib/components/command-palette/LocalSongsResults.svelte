<script lang="ts">
  import { BookOpen, Check, ChevronRight, Globe, Music, Play, Plus, Upload } from 'lucide-svelte';
  import type { ParsedReference, SongSummary } from '../../types';
  import Badge from '../ui/Badge.svelte';
  import { highlightSegments, snippetToHtml } from './highlight';
  import { LISTBOX_ID, optionId } from './types';

  interface Props {
    query: string;
    results: SongSummary[];
    selectedIndex: number;
    addedIds: Record<string, boolean>;
    showQuickActions: boolean;
    internetFallback: boolean;
    detectedReference: ParsedReference | null;
    onselect: (index: number) => void;
    onactivate: (index: number) => void;
    onaddsong: (songId: string) => void;
    onopenreference: () => void;
    onpresentreference: () => void;
  }

  let {
    query,
    results,
    selectedIndex,
    addedIds,
    showQuickActions,
    internetFallback,
    detectedReference,
    onselect,
    onactivate,
    onaddsong,
    onopenreference,
    onpresentreference,
  }: Props = $props();

  const songOffset = $derived(showQuickActions ? 3 : 0);
  const fallbackIndex = $derived(songOffset + results.length);

  const quickActions = [
    {
      title: 'Criar nova música',
      detail: 'Abre o editor completo com prévia de slides',
      key: '⌘N',
      icon: Plus,
      tone: 'bg-emerald-500/15 text-emerald-700 dark:text-emerald-300',
    },
    {
      title: 'Harpa Cristã',
      detail: 'Filtra o catálogo local pelos hinos da Harpa',
      key: '⌘H',
      icon: Music,
      tone: 'bg-amber-500/15 text-amber-700 dark:text-amber-300',
    },
    {
      title: 'Bíblia Sagrada',
      detail: 'Busca por referência ou palavras-chave no texto bíblico',
      key: '⌘B',
      icon: BookOpen,
      tone: 'bg-sky-500/15 text-sky-700 dark:text-sky-300',
    },
  ];

  function optionClasses(index: number): string {
    return selectedIndex === index ? 'bg-muted ring-1 ring-ring/60' : 'hover:bg-muted/60';
  }

  function handleRowKeydown(event: KeyboardEvent, index: number) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onactivate(index);
    }
  }
</script>

{#if detectedReference}
  <div
    class="mb-2 flex flex-wrap items-center justify-between gap-3 rounded-xl border border-amber-500/40 bg-amber-500/10 p-3"
  >
    <div class="flex min-w-0 items-center gap-3">
      <div
        class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-amber-500/15 text-amber-700 dark:text-amber-300"
      >
        <BookOpen size={16} aria-hidden="true" />
      </div>
      <div class="min-w-0">
        <div
          class="flex flex-wrap items-center gap-2 text-[11px] font-bold text-amber-700 dark:text-amber-300"
        >
          <span>Passagem bíblica detectada</span>
          <Badge
            variant="outline"
            class="border-amber-500/40 font-mono uppercase text-amber-700 dark:text-amber-300"
          >
            {detectedReference.book_abbreviation}
          </Badge>
        </div>
        <div class="truncate text-sm font-extrabold text-foreground">
          {detectedReference.book_name}
          {detectedReference.chapter}:{detectedReference.verse_start || 1}
          {detectedReference.verse_end && detectedReference.verse_end !== detectedReference.verse_start
            ? `-${detectedReference.verse_end}`
            : ''}
        </div>
      </div>
    </div>

    <div class="flex shrink-0 items-center gap-1.5">
      <button
        type="button"
        onclick={onopenreference}
        title="Abrir o texto bíblico (Enter)"
        class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded-lg border border-border bg-muted px-2.5 text-[11px] font-semibold text-foreground transition-colors hover:bg-background"
      >
        <span>Abrir texto</span>
        <kbd class="rounded bg-background px-1 py-0.5 font-mono text-[11px] text-muted-foreground">↵</kbd>
      </button>

      <button
        type="button"
        onclick={onpresentreference}
        title="Projetar na tela imediatamente (Shift+Enter)"
        class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded-lg bg-amber-500 px-2.5 text-[11px] font-bold text-amber-950 transition-colors hover:bg-amber-400"
      >
        <Play size={12} class="fill-current" aria-hidden="true" />
        <span>Projetar agora</span>
      </button>
    </div>
  </div>
{/if}

<div id={LISTBOX_ID} role="listbox" aria-label="Músicas locais" class="space-y-0.5">
  {#if showQuickActions}
    <div
      role="group"
      aria-label="Ações rápidas"
      class="mb-2 space-y-0.5 border-b border-border/60 pb-2"
    >
      {#each quickActions as action, index (action.title)}
        {@const Icon = action.icon}
        <div
          id={optionId('local', index)}
          role="option"
          aria-selected={selectedIndex === index}
          tabindex="-1"
          onclick={() => onactivate(index)}
          onkeydown={(event) => handleRowKeydown(event, index)}
          onmouseenter={() => onselect(index)}
          class="flex w-full cursor-pointer items-center justify-between gap-2 rounded-lg px-3 py-2 text-left transition-colors {optionClasses(
            index
          )}"
        >
          <div class="flex min-w-0 items-center gap-2.5">
            <div class="flex h-6 w-6 shrink-0 items-center justify-center rounded {action.tone}">
              <Icon size={14} aria-hidden="true" />
            </div>
            <div class="min-w-0">
              <div class="truncate text-xs font-semibold text-foreground">{action.title}</div>
              <div class="truncate text-[11px] text-muted-foreground">{action.detail}</div>
            </div>
          </div>
          <kbd
            class="shrink-0 rounded border border-border bg-background px-1.5 py-0.5 font-mono text-[11px] text-muted-foreground"
          >
            {action.key}
          </kbd>
        </div>
      {/each}
    </div>
  {/if}

  {#if results.length > 0}
    <div
      role="group"
      aria-label={`Músicas no banco (${results.length})`}
      class="space-y-0.5"
    >
      <div
        aria-hidden="true"
        class="px-2 py-1 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground"
      >
        Músicas no banco ({results.length})
      </div>

      {#each results as song, index (song.id)}
        {@const absoluteIndex = songOffset + index}
        {@const isAdded = addedIds[song.id]}
        <div
          role="group"
          class="flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2.5 transition-colors {optionClasses(
            absoluteIndex
          )}"
          onmouseenter={() => onselect(absoluteIndex)}
        >
          <div
            id={optionId('local', absoluteIndex)}
            role="option"
            aria-selected={selectedIndex === absoluteIndex}
            tabindex="-1"
            onclick={() => onactivate(absoluteIndex)}
            onkeydown={(event) => handleRowKeydown(event, absoluteIndex)}
            class="flex min-w-0 flex-1 cursor-pointer items-start gap-2.5 pr-2 text-left"
          >
            <div class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded bg-muted">
              <Music size={13} class="text-emerald-700 dark:text-emerald-300" aria-hidden="true" />
            </div>
            <div class="min-w-0">
              <div class="flex flex-wrap items-center gap-2">
                <span class="truncate text-xs font-medium text-foreground">
                  {#each highlightSegments(song.title, query) as segment}
                    {#if segment.match}<mark
                        class="rounded-sm bg-amber-500/30 px-0.5 text-foreground">{segment.text}</mark
                      >{:else}{segment.text}{/if}
                  {/each}
                </span>
                {#if song.song_key}
                  <Badge variant="outline" class="font-mono">{song.song_key}</Badge>
                {/if}
                {#if song.bpm}
                  <Badge variant="outline" class="font-mono">{song.bpm} BPM</Badge>
                {/if}
              </div>

              <div class="truncate text-[11px] text-muted-foreground">
                {#each highlightSegments(song.artist || 'Sem artista', query) as segment}
                  {#if segment.match}<mark
                      class="rounded-sm bg-amber-500/30 px-0.5 text-foreground">{segment.text}</mark
                    >{:else}{segment.text}{/if}
                {/each}
              </div>

              {#if song.preview_snippet}
                <div
                  class="mt-0.5 line-clamp-1 font-mono text-[11px] italic text-muted-foreground [&_mark]:bg-amber-500/30 [&_mark]:px-0.5 [&_mark]:text-foreground"
                >
                  {@html snippetToHtml(song.preview_snippet)}
                </div>
              {/if}
            </div>
          </div>

          <button
            type="button"
            onclick={() => onaddsong(song.id)}
            aria-label={isAdded
              ? `${song.title} adicionada à setlist`
              : `Adicionar ${song.title} à setlist`}
            class="flex min-h-7 shrink-0 cursor-pointer items-center gap-1.5 rounded border border-border bg-muted px-2.5 text-[11px] font-medium text-foreground transition-colors hover:bg-emerald-600 hover:text-emerald-50"
          >
            {#if isAdded}
              <Check size={12} class="text-emerald-700 dark:text-emerald-300" aria-hidden="true" />
              <span>Adicionado</span>
            {:else}
              <Plus size={12} aria-hidden="true" />
              <span>Add na Setlist</span>
            {/if}
          </button>
        </div>
      {/each}
    </div>
  {/if}

  {#if internetFallback}
    <div role="group" aria-label="Busca na internet">
      <div
        id={optionId('local', fallbackIndex)}
        role="option"
        aria-selected={selectedIndex === fallbackIndex}
        tabindex="-1"
        onclick={() => onactivate(fallbackIndex)}
        onkeydown={(event) => handleRowKeydown(event, fallbackIndex)}
        onmouseenter={() => onselect(fallbackIndex)}
        class="flex w-full cursor-pointer items-center justify-between gap-3 rounded-lg border border-sky-500/35 bg-sky-500/10 px-3 py-2.5 text-left transition-colors {optionClasses(
          fallbackIndex
        )}"
      >
        <div class="flex min-w-0 items-center gap-2.5">
          <div
            class="flex h-6 w-6 shrink-0 items-center justify-center rounded bg-sky-500/15 text-sky-700 dark:text-sky-300"
          >
            <Globe size={14} aria-hidden="true" />
          </div>
          <div class="min-w-0">
            <div class="truncate text-xs font-semibold text-foreground">
              Nenhuma música local para “{query}”
            </div>
            <div class="truncate text-[11px] text-muted-foreground">
              Buscar na internet (Holyrics e Vagalume) e importar a letra
            </div>
          </div>
        </div>
        <ChevronRight size={14} class="shrink-0 text-muted-foreground" aria-hidden="true" />
      </div>
    </div>
  {/if}
</div>

{#if !showQuickActions && results.length === 0 && !internetFallback}
  <div class="p-8 text-center text-xs text-muted-foreground">
    <Upload size={22} class="mx-auto mb-2 text-muted-foreground/60" aria-hidden="true" />
    Digite parte do título, do artista ou da letra para buscar no catálogo local.
  </div>
{/if}

<script lang="ts">
  import { BookOpen, Check, Play, Plus, RefreshCw } from 'lucide-svelte';
  import type {
    BibleSearchResult,
    BibleVerse,
    BibleVersionSummary,
    ParsedReference,
  } from '../../types';
  import Badge from '../ui/Badge.svelte';
  import { LISTBOX_ID, optionId } from './types';

  interface Props {
    query: string;
    loading: boolean;
    error: string | null;
    bibles: BibleVersionSummary[];
    selectedBibleId: string;
    currentBible: BibleVersionSummary | undefined;
    passageRef: ParsedReference | null;
    verses: BibleVerse[];
    results: BibleSearchResult[];
    selectedIndex: number;
    addedKeys: Record<string, boolean>;
    onversionchange: (bibleId: string) => void;
    onselect: (index: number) => void;
    onactivate: (index: number) => void;
    onaddsecondary: (index: number) => void;
    onretry: () => void;
    onsearch: (query: string) => void;
  }

  let {
    query,
    loading,
    error,
    bibles,
    selectedBibleId,
    currentBible,
    passageRef,
    verses,
    results,
    selectedIndex,
    addedKeys,
    onversionchange,
    onselect,
    onactivate,
    onaddsecondary,
    onretry,
    onsearch,
  }: Props = $props();

  const passageOffset = $derived(verses.length > 1 ? 1 : 0);
  const hasContent = $derived(verses.length > 0 || results.length > 0);

  function optionClasses(index: number): string {
    return selectedIndex === index ? 'bg-muted ring-1 ring-ring/60' : 'hover:bg-muted/60';
  }

  function handleRowKeydown(event: KeyboardEvent, index: number) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onactivate(index);
    }
  }

  function verseKey(verse: BibleVerse): string {
    return `verse-${verse.book_id}-${passageRef?.chapter || verse.chapter}-${verse.verse}`;
  }

  function resultKey(result: BibleSearchResult): string {
    return `res-${result.book_id}-${result.chapter}-${result.verse}`;
  }

  const passageKey = $derived(`passage-${passageRef?.book_id}-${passageRef?.chapter}`);
</script>

{#if bibles.length > 0}
  <div
    class="mb-2 flex flex-wrap items-center justify-between gap-2 rounded-lg border border-border bg-muted/40 px-2.5 py-1.5"
  >
    <label
      for="command-palette-bible-version"
      class="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground"
    >
      Versão da Bíblia
    </label>
    <select
      id="command-palette-bible-version"
      value={selectedBibleId}
      onchange={(event) => onversionchange(event.currentTarget.value)}
      class="min-h-7 shrink-0 cursor-pointer rounded-md border border-border bg-background px-2 text-[11px] font-semibold text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
    >
      {#each bibles as bible (bible.id)}
        <option value={bible.id} title={`${bible.abbreviation} - ${bible.name}`}>
          {bible.abbreviation || bible.name}
        </option>
      {/each}
    </select>
  </div>
{/if}

{#if error}
  <div
    role="alert"
    class="mb-2 flex flex-wrap items-center justify-between gap-2 rounded-lg border border-red-500/35 bg-red-500/10 px-3 py-2 text-[11px] font-medium text-red-700 dark:text-red-300"
  >
    <span class="min-w-0">{error}</span>
    <button
      type="button"
      onclick={onretry}
      class="flex min-h-7 shrink-0 cursor-pointer items-center gap-1.5 rounded border border-red-500/35 px-2 text-[11px] font-semibold text-red-700 transition-colors hover:bg-red-500/15 dark:text-red-300"
    >
      <RefreshCw size={12} aria-hidden="true" />
      Tentar novamente
    </button>
  </div>
{/if}

<div id={LISTBOX_ID} role="listbox" aria-label="Textos bíblicos" class="space-y-0.5">
  {#if verses.length > 0}
    {#if verses.length > 1}
      {@const isPassageSelected = selectedIndex === 0}
      <div
        role="group"
        aria-label="Passagem completa"
        onmouseenter={() => onselect(0)}
        class="mb-2 flex w-full flex-wrap items-center justify-between gap-3 rounded-xl border px-3 py-2.5 transition-colors {isPassageSelected
          ? 'border-amber-500/70 bg-amber-500/20'
          : 'border-amber-500/40 bg-amber-500/10 hover:bg-amber-500/15'}"
      >
        <div
          id={optionId('bible', 0)}
          role="option"
          aria-selected={isPassageSelected}
          tabindex="-1"
          onclick={() => onactivate(0)}
          onkeydown={(event) => handleRowKeydown(event, 0)}
          class="flex min-w-0 flex-1 cursor-pointer items-center gap-3 text-left"
        >
          <div
            class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-amber-500/15 text-amber-700 dark:text-amber-300"
          >
            <BookOpen size={16} aria-hidden="true" />
          </div>
          <div class="min-w-0">
            <div
              class="flex flex-wrap items-center gap-1.5 text-[11px] font-bold text-amber-700 dark:text-amber-300"
            >
              <span>Passagem completa ({verses.length} versículos)</span>
              <Badge
                variant="outline"
                class="border-amber-500/40 font-mono uppercase text-amber-700 dark:text-amber-300"
              >
                {currentBible?.abbreviation || 'Bíblia'}
              </Badge>
            </div>
            <div class="truncate text-sm font-extrabold text-foreground">
              {passageRef?.book_name}
              {passageRef?.chapter}:{verses[0].verse}-{verses[verses.length - 1].verse}
            </div>
          </div>
        </div>

        <div class="flex shrink-0 items-center gap-1.5">
          <button
            type="button"
            onclick={() => onaddsecondary(0)}
            title="Adicionar todos os versículos à Setlist (Shift+Enter)"
            class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded-lg border border-border bg-muted px-2.5 text-[11px] font-semibold text-foreground transition-colors hover:bg-amber-600 hover:text-amber-50"
          >
            {#if addedKeys[passageKey]}
              <Check size={13} class="text-emerald-700 dark:text-emerald-300" aria-hidden="true" />
              <span>Adicionado</span>
            {:else}
              <Plus size={13} aria-hidden="true" />
              <span>Add na Setlist</span>
            {/if}
          </button>

          <button
            type="button"
            onclick={() => onactivate(0)}
            title="Projetar todos os versículos ao vivo (Enter)"
            class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded-lg bg-amber-500 px-2.5 text-[11px] font-bold text-amber-950 shadow-sm transition-colors hover:bg-amber-400"
          >
            <Play size={13} class="fill-current" aria-hidden="true" />
            <span>Projetar Passagem</span>
          </button>
        </div>
      </div>
    {/if}

    <div
      role="group"
      aria-label={`Versículos (${verses.length})`}
      class="space-y-0.5"
    >
      <div
        aria-hidden="true"
        class="px-2 py-1 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground"
      >
        Versículos ({verses.length}) — {currentBible?.name} ({currentBible?.abbreviation})
      </div>

      {#each verses as verse, index (verse.id || verse.verse)}
        {@const absoluteIndex = passageOffset + index}
        {@const isAdded = addedKeys[verseKey(verse)]}
        <div
          role="group"
          onmouseenter={() => onselect(absoluteIndex)}
          class="flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2.5 transition-colors {optionClasses(
            absoluteIndex
          )}"
        >
          <div
            id={optionId('bible', absoluteIndex)}
            role="option"
            aria-selected={selectedIndex === absoluteIndex}
            tabindex="-1"
            onclick={() => onactivate(absoluteIndex)}
            onkeydown={(event) => handleRowKeydown(event, absoluteIndex)}
            class="flex min-w-0 flex-1 cursor-pointer items-start gap-2.5 pr-3 text-left"
          >
            <div
              class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded bg-amber-500/15 font-mono text-[11px] font-bold text-amber-700 dark:text-amber-300"
            >
              {verse.verse}
            </div>
            <div class="min-w-0">
              <div class="flex flex-wrap items-center gap-2">
                <span class="truncate text-xs font-bold text-foreground">
                  {passageRef?.book_name} {passageRef?.chapter || verse.chapter}:{verse.verse}
                </span>
                <Badge
                  variant="outline"
                  class="border-amber-500/40 font-mono text-amber-700 dark:text-amber-300"
                >
                  {currentBible?.abbreviation || 'Bíblia'}
                </Badge>
              </div>
              <div class="mt-0.5 line-clamp-2 text-xs leading-snug text-muted-foreground">
                {verse.text}
              </div>
            </div>
          </div>

          <div class="flex shrink-0 items-center gap-1.5">
            <button
              type="button"
              onclick={() => onaddsecondary(absoluteIndex)}
              title="Adicionar versículo à Setlist"
              class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded border border-border bg-muted px-2.5 text-[11px] font-medium text-foreground transition-colors hover:bg-amber-600 hover:text-amber-50"
            >
              {#if isAdded}
                <Check
                  size={12}
                  class="text-emerald-700 dark:text-emerald-300"
                  aria-hidden="true"
                />
                <span>Adicionado</span>
              {:else}
                <Plus size={12} aria-hidden="true" />
                <span>Add</span>
              {/if}
            </button>

            <button
              type="button"
              onclick={() => onactivate(absoluteIndex)}
              title="Projetar este versículo ao vivo"
              class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded bg-amber-500 px-2.5 text-[11px] font-bold text-amber-950 transition-colors hover:bg-amber-400"
            >
              <Play size={12} class="fill-current" aria-hidden="true" />
              <span>Ao Vivo</span>
            </button>
          </div>
        </div>
      {/each}
    </div>
  {:else if results.length > 0}
    <div
      role="group"
      aria-label={`Versículos encontrados (${results.length})`}
      class="space-y-0.5"
    >
      <div
        aria-hidden="true"
        class="px-2 py-1 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground"
      >
        Versículos encontrados ({results.length}) — Versão {currentBible?.abbreviation}
      </div>

      {#each results as result, index (`${result.book_id}-${result.chapter}-${result.verse}`)}
        {@const isAdded = addedKeys[resultKey(result)]}
        <div
          role="group"
          onmouseenter={() => onselect(index)}
          class="flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2.5 transition-colors {optionClasses(
            index
          )}"
        >
          <div
            id={optionId('bible', index)}
            role="option"
            aria-selected={selectedIndex === index}
            tabindex="-1"
            onclick={() => onactivate(index)}
            onkeydown={(event) => handleRowKeydown(event, index)}
            class="flex min-w-0 flex-1 cursor-pointer items-start gap-2.5 pr-3 text-left"
          >
            <div
              class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded bg-amber-500/15 text-amber-700 dark:text-amber-300"
            >
              <BookOpen size={13} aria-hidden="true" />
            </div>
            <div class="min-w-0">
              <div class="flex flex-wrap items-center gap-2">
                <span class="truncate text-xs font-bold text-foreground">
                  {result.book_name} {result.chapter}:{result.verse}
                </span>
                <Badge
                  variant="outline"
                  class="border-amber-500/40 font-mono text-amber-700 dark:text-amber-300"
                >
                  {result.book_abbreviation || currentBible?.abbreviation}
                </Badge>
              </div>
              <div class="mt-0.5 line-clamp-2 text-xs leading-snug text-muted-foreground">
                {result.text}
              </div>
            </div>
          </div>

          <div class="flex shrink-0 items-center gap-1.5">
            <button
              type="button"
              onclick={() => onaddsecondary(index)}
              title="Adicionar versículo à Setlist"
              class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded border border-border bg-muted px-2.5 text-[11px] font-medium text-foreground transition-colors hover:bg-amber-600 hover:text-amber-50"
            >
              {#if isAdded}
                <Check
                  size={12}
                  class="text-emerald-700 dark:text-emerald-300"
                  aria-hidden="true"
                />
                <span>Adicionado</span>
              {:else}
                <Plus size={12} aria-hidden="true" />
                <span>Add</span>
              {/if}
            </button>

            <button
              type="button"
              onclick={() => onactivate(index)}
              title="Projetar este versículo ao vivo"
              class="flex min-h-7 cursor-pointer items-center gap-1.5 rounded bg-amber-500 px-2.5 text-[11px] font-bold text-amber-950 transition-colors hover:bg-amber-400"
            >
              <Play size={12} class="fill-current" aria-hidden="true" />
              <span>Ao Vivo</span>
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if !hasContent}
  {#if loading}
    <div class="p-8 text-center text-xs text-muted-foreground" role="status">
      Buscando textos bíblicos...
    </div>
  {:else if query}
    <div class="space-y-2 p-8 text-center text-xs text-muted-foreground">
      <p>
        Nenhum texto bíblico encontrado para “{query}” na versão {currentBible?.abbreviation || ''}.
      </p>
      <p class="text-[11px]">
        Tente uma referência como <strong class="text-foreground">João 3:16</strong>,
        <strong class="text-foreground">Sl 23:1-4</strong>, ou palavras como
        <strong class="text-foreground">luz do mundo</strong>.
      </p>
    </div>
  {:else}
    <div class="space-y-2 p-8 text-center text-xs text-muted-foreground">
      <BookOpen size={24} class="mx-auto mb-2 text-amber-500/50" aria-hidden="true" />
      <p class="font-medium text-foreground">Pesquise textos bíblicos por referência ou palavras-chave</p>
      <p class="text-[11px]">Exemplos:</p>
      <div class="flex flex-wrap items-center justify-center gap-1.5">
        {#each ['João 3:16', 'Sl 23:1-4', 'luz do mundo', 'princípio'] as example (example)}
          <button
            type="button"
            onclick={() => onsearch(example)}
            class="min-h-7 cursor-pointer rounded border border-border bg-background px-2 font-mono text-[11px] text-amber-700 transition-colors hover:bg-muted dark:text-amber-300"
          >
            {example}
          </button>
        {/each}
      </div>
    </div>
  {/if}
{/if}

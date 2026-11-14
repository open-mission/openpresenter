<script lang="ts">
  import {
    AlertTriangle,
    BookOpen,
    ChevronDown,
    Loader2,
    RefreshCw,
    Search,
    Send,
    X,
  } from 'lucide-svelte';
  import type { BibleBook, BibleSearchResult, BibleVerse } from '$lib/types';

  interface Props {
    activeBibleAbbr: string;
    activeBibleName: string;
    books: BibleBook[];
    selectedBookId: number;
    selectedBookName: string;
    chapters: number[];
    selectedChapter: number;
    verses: BibleVerse[];
    testamentFilter: 'all' | 'ot' | 'nt';
    bibleSearchQuery: string;
    bibleSearchResults: BibleSearchResult[];
    bibleNavTab: 'verses' | 'books' | 'chapters';
    isLoadingBible: boolean;
    bibleError: string | null;
    activeMediaId: string | null;
    activeSlideIndex: number | null;
    safeDispatch: boolean;
    onOpenVersionSelector: () => void;
    onSearchQueryChange: (q: string) => void;
    onClearSearch: () => void;
    onSubmitSearch: (e: SubmitEvent) => void;
    onSetNavTab: (tab: 'verses' | 'books' | 'chapters') => void;
    onSetTestamentFilter: (filter: 'all' | 'ot' | 'nt') => void;
    onSelectBook: (bookId: number) => void;
    onSelectChapter: (chapter: number) => void;
    onProjectVerse: (verse: BibleVerse) => void;
    onProjectSearchResult: (res: BibleSearchResult) => void;
    onRetryBible: () => void;
  }

  let {
    activeBibleAbbr,
    activeBibleName,
    books,
    selectedBookId,
    selectedBookName,
    chapters,
    selectedChapter,
    verses,
    testamentFilter,
    bibleSearchQuery,
    bibleSearchResults,
    bibleNavTab,
    isLoadingBible,
    bibleError,
    activeMediaId,
    activeSlideIndex,
    safeDispatch,
    onOpenVersionSelector,
    onSearchQueryChange,
    onClearSearch,
    onSubmitSearch,
    onSetNavTab,
    onSetTestamentFilter,
    onSelectBook,
    onSelectChapter,
    onProjectVerse,
    onProjectSearchResult,
    onRetryBible,
  }: Props = $props();

  let bookFilterQuery = $state('');
  let cuedVerseId = $state<number | null>(null);
  let cuedResultKey = $state<string | null>(null);

  const filteredBooks = $derived.by(() => {
    const q = bookFilterQuery.trim().toLowerCase();
    return books.filter((b) => {
      if (testamentFilter === 'ot' && b.testament_reference_id !== 1) return false;
      if (testamentFilter === 'nt' && b.testament_reference_id !== 2) return false;
      if (!q) return true;
      return b.name.toLowerCase().includes(q) || b.abbreviation.toLowerCase().includes(q);
    });
  });

  const subTabs: Array<{ id: 'verses' | 'books' | 'chapters'; label: string }> = [
    { id: 'verses', label: 'Versículos' },
    { id: 'books', label: 'Livros' },
    { id: 'chapters', label: 'Capítulos' },
  ];

  function handleTabKeyDown(e: KeyboardEvent, idx: number) {
    if (e.key !== 'ArrowLeft' && e.key !== 'ArrowRight' && e.key !== 'Home' && e.key !== 'End') {
      return;
    }
    e.preventDefault();
    let nextIdx = idx;
    if (e.key === 'ArrowRight') nextIdx = (idx + 1) % subTabs.length;
    else if (e.key === 'ArrowLeft') nextIdx = (idx - 1 + subTabs.length) % subTabs.length;
    else if (e.key === 'Home') nextIdx = 0;
    else if (e.key === 'End') nextIdx = subTabs.length - 1;

    const target = subTabs[nextIdx];
    onSetNavTab(target.id);
    const parent = (e.currentTarget as HTMLElement).parentElement;
    const buttons = parent?.querySelectorAll<HTMLButtonElement>('[role="tab"]');
    buttons?.[nextIdx]?.focus();
  }
</script>

<div class="space-y-3 pb-2">
  <h1 class="sr-only">Bíblia Sagrada</h1>

  <!-- Cabeçalho Bíblia e Seletor de Versão -->
  <div class="flex items-center justify-between gap-2">
    <div class="min-w-0">
      <p class="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">
        Leitura Atual
      </p>
      <p class="text-sm font-bold text-foreground truncate">
        {selectedBookName} {selectedChapter}
      </p>
    </div>

    <button
      type="button"
      onclick={onOpenVersionSelector}
      aria-haspopup="dialog"
      title={activeBibleName}
      class="min-h-10 px-3 rounded-xl bg-card hover:bg-muted border border-border text-xs font-bold text-emerald-800 dark:text-emerald-300 inline-flex items-center gap-1.5 transition-colors cursor-pointer shrink-0"
    >
      <BookOpen size={14} aria-hidden="true" />
      <span>{activeBibleAbbr}</span>
      <ChevronDown size={14} aria-hidden="true" />
    </button>
  </div>

  <!-- Busca Rápida / Referência (ex: Jo 3:16) -->
  <form onsubmit={onSubmitSearch} class="relative">
    <label for="remote-bible-search" class="sr-only">
      Buscar referência como Jo 3:16 ou palavras-chave bíblicas
    </label>
    <Search
      size={16}
      aria-hidden="true"
      class="absolute left-3.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
    />
    <input
      id="remote-bible-search"
      type="search"
      value={bibleSearchQuery}
      oninput={(e) => onSearchQueryChange(e.currentTarget.value)}
      placeholder="Referência (ex: Jo 3:16) ou palavra..."
      autocomplete="off"
      class="w-full h-11 bg-card border border-border rounded-xl pl-10 pr-24 text-base sm:text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring transition-colors"
    />
    <div class="absolute right-1.5 top-1/2 -translate-y-1/2 flex items-center gap-1">
      {#if bibleSearchQuery || bibleSearchResults.length > 0}
        <button
          type="button"
          onclick={onClearSearch}
          aria-label="Limpar busca bíblica"
          class="h-9 w-9 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
        >
          <X size={16} aria-hidden="true" />
        </button>
      {/if}
      <button
        type="submit"
        class="h-8 px-2.5 rounded-lg bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold transition-colors cursor-pointer"
      >
        Ir
      </button>
    </div>
  </form>

  {#if bibleError}
    <div
      role="alert"
      class="p-3.5 rounded-2xl bg-red-500/15 border border-red-600/35 dark:border-red-500/40 text-red-700 dark:text-red-300 flex items-center justify-between gap-3"
    >
      <p class="text-xs font-medium flex items-center gap-2 min-w-0">
        <AlertTriangle size={16} class="shrink-0" aria-hidden="true" />
        <span>{bibleError}</span>
      </p>
      <button
        type="button"
        onclick={onRetryBible}
        class="min-h-9 px-3 rounded-lg bg-red-700 hover:bg-red-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer shrink-0"
      >
        <RefreshCw size={13} aria-hidden="true" />
        <span>Tentar</span>
      </button>
    </div>
  {/if}

  {#if bibleSearchResults.length > 0}
    <div class="space-y-2">
      <div class="flex items-center justify-between px-1">
        <p class="text-xs font-bold text-foreground">
          {bibleSearchResults.length} versículo{bibleSearchResults.length === 1 ? '' : 's'} encontrado{bibleSearchResults.length === 1 ? '' : 's'}
        </p>
        <button
          type="button"
          onclick={onClearSearch}
          class="min-h-8 px-2.5 rounded-lg text-xs font-semibold text-emerald-800 dark:text-emerald-300 hover:bg-muted cursor-pointer"
        >
          Voltar ao capítulo
        </button>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
        {#each bibleSearchResults as item (`${item.book_id}-${item.chapter}-${item.verse}`)}
          {@const rKey = `${item.book_id}-${item.chapter}-${item.verse}`}
          {@const isCued = safeDispatch && cuedResultKey === rKey}
          <div
            class="rounded-2xl border flex items-stretch gap-2 transition-colors {isCued
              ? 'bg-amber-500/15 border-amber-600/60 dark:border-amber-500'
              : 'bg-card border-border'}"
          >
            <button
              type="button"
              onclick={() => {
                if (!safeDispatch || cuedResultKey === rKey) {
                  cuedResultKey = null;
                  onProjectSearchResult(item);
                } else {
                  cuedResultKey = rKey;
                }
              }}
              class="flex-1 min-h-14 p-3.5 text-left rounded-2xl hover:bg-muted/50 transition-colors cursor-pointer"
            >
              <span class="block text-xs font-bold text-emerald-800 dark:text-emerald-300 mb-1">
                {item.book_abbreviation} {item.chapter}:{item.verse}
              </span>
              <span class="block text-sm text-foreground leading-relaxed">{item.text}</span>
            </button>
            {#if isCued}
              <button
                type="button"
                onclick={() => {
                  cuedResultKey = null;
                  onProjectSearchResult(item);
                }}
                class="m-2 min-h-10 px-3 rounded-xl bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer shrink-0 self-center"
              >
                <Send size={13} aria-hidden="true" />
                <span>Projetar</span>
              </button>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <!-- Sub-abas acessíveis (WAI-ARIA tablist) -->
    <div
      role="tablist"
      aria-label="Navegação na Bíblia"
      class="grid grid-cols-3 p-1 rounded-xl bg-card border border-border gap-1"
    >
      {#each subTabs as tab, idx (tab.id)}
        {@const active = bibleNavTab === tab.id}
        <button
          id="bible-tab-{tab.id}"
          type="button"
          role="tab"
          aria-selected={active}
          aria-controls="bible-panel-{tab.id}"
          tabindex={active ? 0 : -1}
          onclick={() => onSetNavTab(tab.id)}
          onkeydown={(e) => handleTabKeyDown(e, idx)}
          class="min-h-10 rounded-lg text-xs font-bold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {active
            ? 'bg-emerald-700 text-white shadow-xs'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted'}"
        >
          {tab.label}
        </button>
      {/each}
    </div>

    {#if isLoadingBible}
      <div class="py-12 flex items-center justify-center gap-2 text-xs text-muted-foreground" role="status">
        <Loader2 size={18} class="animate-spin text-emerald-700 dark:text-emerald-400" aria-hidden="true" />
        <span>Carregando texto bíblico...</span>
      </div>
    {:else if bibleNavTab === 'books'}
      <div
        id="bible-panel-books"
        role="tabpanel"
        aria-labelledby="bible-tab-books"
        class="space-y-3"
      >
        <div class="flex flex-col sm:flex-row gap-2">
          <div class="grid grid-cols-3 gap-1.5 flex-1">
            {#each [
              { id: 'all' as const, label: 'Todos (66)' },
              { id: 'ot' as const, label: 'Antigo (39)' },
              { id: 'nt' as const, label: 'Novo (27)' },
            ] as tf (tf.id)}
              <button
                type="button"
                aria-pressed={testamentFilter === tf.id}
                onclick={() => onSetTestamentFilter(tf.id)}
                class="min-h-10 px-2 rounded-xl text-xs font-bold border transition-colors cursor-pointer {testamentFilter === tf.id
                  ? 'bg-emerald-700 text-white border-emerald-800'
                  : 'bg-card text-muted-foreground hover:text-foreground border-border'}"
              >
                {tf.label}
              </button>
            {/each}
          </div>

          <input
            type="search"
            bind:value={bookFilterQuery}
            aria-label="Filtrar livros pelo nome"
            placeholder="Filtrar livro..."
            class="sm:w-48 h-10 px-3 rounded-xl bg-card border border-border text-base sm:text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>

        {#if filteredBooks.length === 0}
          <p class="py-8 text-center text-xs text-muted-foreground">
            Nenhum livro encontrado para este filtro.
          </p>
        {:else}
          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
            {#each filteredBooks as book (book.id)}
              {@const isSelected = book.id === selectedBookId}
              <button
                type="button"
                aria-pressed={isSelected}
                onclick={() => onSelectBook(book.id)}
                class="min-h-12 px-3 py-2 rounded-xl border text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {isSelected
                  ? 'bg-emerald-700 text-white border-emerald-800 shadow-xs'
                  : 'bg-card hover:bg-muted border-border text-foreground'}"
              >
                <span class="block text-xs font-bold truncate">{book.name}</span>
                <span class="block text-[11px] opacity-80 mt-0.5">
                  {book.abbreviation} • {book.chapter_count} cap.
                </span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {:else if bibleNavTab === 'chapters'}
      <div
        id="bible-panel-chapters"
        role="tabpanel"
        aria-labelledby="bible-tab-chapters"
        class="grid grid-cols-5 sm:grid-cols-8 md:grid-cols-10 gap-2"
      >
        {#each chapters as ch (ch)}
          {@const isSelected = ch === selectedChapter}
          <button
            type="button"
            aria-pressed={isSelected}
            onclick={() => onSelectChapter(ch)}
            class="min-h-12 rounded-xl border font-mono tabular-nums text-sm font-bold flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer {isSelected
              ? 'bg-emerald-700 text-white border-emerald-800 shadow-xs'
              : 'bg-card hover:bg-muted border-border text-foreground'}"
          >
            {ch}
          </button>
        {/each}
      </div>
    {:else}
      <div
        id="bible-panel-verses"
        role="tabpanel"
        aria-labelledby="bible-tab-verses"
        class="space-y-2"
      >
        {#if verses.length === 0}
          <div class="py-10 text-center rounded-2xl bg-card border border-border text-xs text-muted-foreground">
            Nenhum versículo carregado para {selectedBookName} {selectedChapter}.
          </div>
        {:else}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
            {#each verses as verse, vIdx (verse.id || `${verse.chapter}-${verse.verse}`)}
              {@const isVerseLive =
                activeMediaId === `bible-remote-${selectedBookId}-${selectedChapter}` &&
                activeSlideIndex === vIdx}
              {@const isCued = safeDispatch && cuedVerseId === verse.id}

              <div
                class="rounded-2xl border transition-colors flex items-stretch gap-2 {isVerseLive
                  ? 'bg-emerald-500/15 border-emerald-600/60 dark:border-emerald-500'
                  : isCued
                    ? 'bg-amber-500/15 border-amber-600/60 dark:border-amber-500'
                    : 'bg-card border-border'}"
              >
                <button
                  type="button"
                  onclick={() => {
                    if (!safeDispatch || cuedVerseId === verse.id) {
                      cuedVerseId = null;
                      onProjectVerse(verse);
                    } else {
                      cuedVerseId = verse.id;
                    }
                  }}
                  class="flex-1 min-h-14 p-3.5 rounded-2xl text-left hover:bg-muted/50 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
                >
                  <span class="flex items-center justify-between gap-2 mb-1">
                    <span class="text-xs font-bold {isVerseLive
                      ? 'text-emerald-800 dark:text-emerald-300'
                      : isCued
                        ? 'text-amber-800 dark:text-amber-300'
                        : 'text-emerald-700 dark:text-emerald-400'}">
                      v. {verse.verse}
                    </span>
                    {#if isVerseLive}
                      <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-emerald-700 text-white">
                        Ao Vivo
                      </span>
                    {:else if isCued}
                      <span class="text-[11px] font-bold text-amber-800 dark:text-amber-300">
                        Selecionado
                      </span>
                    {/if}
                  </span>
                  <span class="block text-sm text-foreground leading-relaxed">{verse.text}</span>
                </button>

                {#if isCued}
                  <button
                    type="button"
                    onclick={() => {
                      cuedVerseId = null;
                      onProjectVerse(verse);
                    }}
                    class="m-2 min-h-10 px-3 rounded-xl bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer shrink-0 self-center"
                  >
                    <Send size={13} aria-hidden="true" />
                    <span>Projetar</span>
                  </button>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</div>

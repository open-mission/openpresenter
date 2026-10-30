<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Search,
    BookOpen,
    Play,
    ListPlus,
    Plus,
    Copy,
    Check,
    Layers,
    ChevronDown,
    Settings,
    X,
    Filter,
    ArrowRight,
    Sparkles,
  } from 'lucide-svelte';
  import Badge from './ui/Badge.svelte';
  import Button from './ui/Button.svelte';
  import ContextMenu, { type MenuItem } from './ContextMenu.svelte';
  import type {
    BibleVersionSummary,
    BibleBook,
    BibleVerse,
    BibleSearchResult,
    MediaItem,
    Slide,
    ParsedReference,
  } from '../types';
  import { parseBibleReference } from '../bibleParser';
  import { presentationState, quickPresentMedia } from '../ws';
  import { biblesStore, refreshBibles } from '../bibles';

  interface Props {
    targetPassage?: ParsedReference | null;
    onpresentscripture?: (item: MediaItem, slideIndex: number) => void;
    onaddscripturetosetlist?: (item: MediaItem) => void;
    onopenbibleconfig?: () => void;
  }

  let {
    targetPassage = null,
    onpresentscripture,
    onaddscripturetosetlist,
    onopenbibleconfig,
  }: Props = $props();

  // --- ESTADO DE VERSÕES DA BÍBLIA ---
  let bibles = $state<BibleVersionSummary[]>([]);
  let selectedBibleId = $state<string>('');
  let isLoadingBibles = $state(false);

  // --- NAVEGAÇÃO BÍBLICA ---
  let books = $state<BibleBook[]>([]);
  let selectedBookId = $state<number>(1); // Padrão: Gênesis
  let testamentFilter = $state<'all' | 'ot' | 'nt'>('all');
  let bookViewMode = $state<'names' | 'abbreviations'>('names');
  let bookSearchQuery = $state<string>('');

  let chapters = $state<number[]>([]);
  let selectedChapter = $state<number>(1);

  let verses = $state<BibleVerse[]>([]);
  let isLoadingVerses = $state(false);

  // --- SELEÇÃO MÚLTIPLA & SETLIST ---
  let selectedVerseNumbers = $state<number[]>([]);
  let lastClickedVerseNum = $state<number | null>(null);

  // --- BUSCA / PARSER ---
  let searchInput = $state<string>('');
  let searchResults = $state<BibleSearchResult[]>([]);
  let isSearching = $state(false);
  let hasSearchResults = $state(false);
  let matchExactWord = $state<boolean>(false);

  // --- LARGURAS DAS COLUNAS REDIMENSIONÁVEIS ---
  let col1Width = $state<number>(200); // Livros
  let col2Width = $state<number>(180); // Capítulos (ampliado para navegação mais confortável)
  let isDraggingCol1 = $state(false);
  let isDraggingCol2 = $state(false);

  // Livro atual selecionado
  const currentBook = $derived(
    books.find((b) => b.id === selectedBookId) || {
      id: 1,
      book_reference_id: 1,
      testament_reference_id: 1,
      name: 'Gênesis',
      abbreviation: 'Gn',
      chapter_count: 50,
    }
  );

  // Bíblia ativa
  const currentBible = $derived(
    bibles.find((b) => b.id === selectedBibleId) || bibles[0]
  );

  // Livros filtrados por testamento e busca
  const filteredBooks = $derived(
    books.filter((b) => {
      if (testamentFilter === 'ot' && b.testament_reference_id !== 1) return false;
      if (testamentFilter === 'nt' && b.testament_reference_id !== 2) return false;
      if (!bookSearchQuery.trim()) return true;
      const q = bookSearchQuery.toLowerCase().trim();
      return (
        b.name.toLowerCase().includes(q) ||
        b.abbreviation.toLowerCase().includes(q)
      );
    })
  );

  // Verifica se o slide projetado é deste livro e capítulo
  const isScriptureLive = $derived(
    $presentationState.active_media_id?.startsWith('scripture-')
  );

  onMount(() => {
    refreshBibles();
  });

  // Reage imediatamente a mudanças na lista de bíblias (importações, renomeações, exclusões)
  $effect(() => {
    const list = $biblesStore;
    bibles = list;
    if (list.length > 0) {
      const exists = list.some((b) => b.id === selectedBibleId);
      if (!selectedBibleId || !exists) {
        const def = list.find((b) => b.is_default) || list[0];
        selectedBibleId = def.id;
        loadBooks(selectedBibleId);
      }
    } else {
      selectedBibleId = '';
      books = [];
      chapters = [];
      verses = [];
    }
  });

  async function loadBibles() {
    isLoadingBibles = true;
    try {
      await refreshBibles();
    } finally {
      isLoadingBibles = false;
    }
  }

  async function loadBooks(bibleId: string) {
    if (!bibleId) return;
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books`);
      if (res.ok) {
        books = await res.json();
        if (books.length > 0) {
          if (!books.some((b) => b.id === selectedBookId)) {
            selectedBookId = books[0].id;
          }
          await loadChapters(bibleId, selectedBookId);
        }
      }
    } catch (e) {
      console.error('Erro ao carregar livros:', e);
    }
  }

  async function loadChapters(bibleId: string, bookId: number) {
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books/${bookId}/chapters`);
      if (res.ok) {
        chapters = await res.json();
        if (chapters.length > 0) {
          if (!chapters.includes(selectedChapter)) {
            selectedChapter = chapters[0];
          }
          await loadVerses(bibleId, bookId, selectedChapter);
        }
      }
    } catch (e) {
      console.error('Erro ao carregar capítulos:', e);
    }
  }

  async function loadVerses(bibleId: string, bookId: number, chapter: number) {
    isLoadingVerses = true;
    selectedVerseNumbers = [];
    lastClickedVerseNum = null;
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books/${bookId}/chapters/${chapter}/verses`);
      if (res.ok) {
        verses = await res.json();
      }
    } catch (e) {
      console.error('Erro ao carregar versículos:', e);
    } finally {
      isLoadingVerses = false;
    }
  }

  let prevTargetPassage = '';
  $effect(() => {
    if (targetPassage && bibles.length > 0) {
      const key = `${targetPassage.book_id}:${targetPassage.chapter}:${targetPassage.verse_start || 1}`;
      if (key !== prevTargetPassage) {
        prevTargetPassage = key;
        jumpToPassage(targetPassage);
      }
    }
  });

  async function jumpToPassage(ref: ParsedReference) {
    if (ref.version) {
      const matched = bibles.find(
        (b) =>
          b.abbreviation.toLowerCase() === ref.version!.toLowerCase() ||
          b.name.toLowerCase().includes(ref.version!.toLowerCase())
      );
      if (matched && matched.id !== selectedBibleId) {
        selectedBibleId = matched.id;
        await loadBooks(matched.id);
      }
    }

    hasSearchResults = false;
    selectedBookId = ref.book_id;
    selectedChapter = ref.chapter;
    await loadChapters(selectedBibleId, ref.book_id);
    await loadVerses(selectedBibleId, ref.book_id, ref.chapter);
    const targetVerse = ref.verse_start || 1;
    selectedVerseNumbers = [targetVerse];
    lastClickedVerseNum = targetVerse;
    setTimeout(() => {
      const el = document.getElementById(`verse-slide-${targetVerse}`);
      el?.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }, 120);
  }

  function handleSelectBook(bookId: number) {
    selectedBookId = bookId;
    selectedChapter = 1;
    hasSearchResults = false;
    loadChapters(selectedBibleId, bookId);
  }

  function handleSelectChapter(chapter: number) {
    selectedChapter = chapter;
    hasSearchResults = false;
    loadVerses(selectedBibleId, selectedBookId, chapter);
  }

  // --- SELEÇÃO DE VERSÍCULOS (CLICK NORMAL vs SHIFT+CLICK) ---
  function handleVerseClick(verseNum: number, event: MouseEvent) {
    if (event.shiftKey) {
      // Modo de seleção contínua de intervalo
      if (lastClickedVerseNum !== null) {
        const start = Math.min(lastClickedVerseNum, verseNum);
        const end = Math.max(lastClickedVerseNum, verseNum);
        const range: number[] = [];
        for (let i = start; i <= end; i++) {
          range.push(i);
        }
        selectedVerseNumbers = Array.from(new Set([...selectedVerseNumbers, ...range])).sort(
          (a, b) => a - b
        );
      } else {
        selectedVerseNumbers = [verseNum];
        lastClickedVerseNum = verseNum;
      }
    } else {
      // Clique simples: alterna seleção se já houver itens selecionados, ou marca início
      if (selectedVerseNumbers.length > 0) {
        if (selectedVerseNumbers.includes(verseNum)) {
          selectedVerseNumbers = selectedVerseNumbers.filter((n) => n !== verseNum);
        } else {
          selectedVerseNumbers = [...selectedVerseNumbers, verseNum].sort((a, b) => a - b);
        }
      } else {
        selectedVerseNumbers = [verseNum];
      }
      lastClickedVerseNum = verseNum;
    }
  }

  // --- APRESENTAÇÃO AO VIVO (DUPLO CLIQUE) ---
  function handleVerseDoubleClick(verseNum: number) {
    const slideIdx = verses.findIndex((v) => v.verse === verseNum);
    const targetIdx = slideIdx >= 0 ? slideIdx : 0;

    const abbr = currentBible?.abbreviation || 'Bíblia';
    const mediaItem: MediaItem = {
      id: `scripture-${selectedBibleId}-${selectedBookId}-${selectedChapter}`,
      title: `${currentBook.name} ${selectedChapter}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: verses.map((v) => ({
        id: `v-${v.verse}`,
        label: `${currentBook.name} ${selectedChapter}:${v.verse} ${abbr}`,
        content: v.text,
      })),
    };

    if (onpresentscripture) {
      onpresentscripture(mediaItem, targetIdx);
    } else {
      quickPresentMedia(mediaItem, targetIdx);
    }
  }

  // --- ADICIONAR À SETLIST ---
  function handleAddSelectedToSetlist() {
    if (selectedVerseNumbers.length === 0) return;

    const selectedVerses = verses.filter((v) => selectedVerseNumbers.includes(v.verse));
    if (selectedVerses.length === 0) return;

    const abbr = currentBible?.abbreviation || 'Bíblia';
    const firstV = selectedVerseNumbers[0];
    const lastV = selectedVerseNumbers[selectedVerseNumbers.length - 1];
    const rangeLabel =
      selectedVerseNumbers.length === 1
        ? `${firstV}`
        : `${firstV}-${lastV}`;

    const mediaItem: MediaItem = {
      id: `scripture-${Date.now()}`,
      title: `${currentBook.name} ${selectedChapter}:${rangeLabel}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: selectedVerses.map((v) => ({
        id: `v-${v.verse}`,
        label: `${currentBook.name} ${selectedChapter}:${v.verse} ${abbr}`,
        content: v.text,
      })),
    };

    if (onaddscripturetosetlist) {
      onaddscripturetosetlist(mediaItem);
    } else {
      fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item: mediaItem }),
      }).catch(console.error);
    }

    selectedVerseNumbers = [];
  }

  function handleAddFullChapterToSetlist() {
    if (verses.length === 0) return;
    const abbr = currentBible?.abbreviation || 'Bíblia';
    const mediaItem: MediaItem = {
      id: `scripture-${Date.now()}`,
      title: `${currentBook.name} ${selectedChapter}`,
      artist_or_ref: abbr,
      kind: 'scripture',
      slides: verses.map((v) => ({
        id: `v-${v.verse}`,
        label: `${currentBook.name} ${selectedChapter}:${v.verse} ${abbr}`,
        content: v.text,
      })),
    };

    if (onaddscripturetosetlist) {
      onaddscripturetosetlist(mediaItem);
    } else {
      fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item: mediaItem }),
      }).catch(console.error);
    }
  }

  function handleCopySelectedVerses() {
    const selectedVerses = verses.filter((v) => selectedVerseNumbers.includes(v.verse));
    if (selectedVerses.length === 0) return;
    const abbr = currentBible?.abbreviation || 'Bíblia';
    const firstV = selectedVerseNumbers[0];
    const lastV = selectedVerseNumbers[selectedVerseNumbers.length - 1];
    const rangeLabel = selectedVerseNumbers.length === 1 ? `${firstV}` : `${firstV}-${lastV}`;
    const header = `${currentBook.name} ${selectedChapter}:${rangeLabel} (${abbr})`;
    const body = selectedVerses.map((v) => `${v.verse}. ${v.text}`).join('\n');
    navigator.clipboard.writeText(`${header}\n\n${body}`).catch(() => {});
  }

  // Menu de Contexto nos Versículos
  let contextMenuOpen = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuItems = $state<MenuItem[]>([]);

  function handleVerseContextMenu(e: MouseEvent, verseNum: number) {
    e.preventDefault();
    e.stopPropagation();

    if (!selectedVerseNumbers.includes(verseNum)) {
      selectedVerseNumbers = [verseNum];
      lastClickedVerseNum = verseNum;
    }

    const count = selectedVerseNumbers.length;
    const isSingle = count === 1;
    const rangeText = isSingle
      ? `${currentBook?.abbreviation || ''} ${selectedChapter}:${selectedVerseNumbers[0]}`
      : `${currentBook?.abbreviation || ''} ${selectedChapter}:${selectedVerseNumbers[0]}-${selectedVerseNumbers[count - 1]}`;

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'add_to_setlist',
        label: count > 1 ? `Adicionar ${count} Versículos à Setlist` : `Adicionar à Setlist (${rangeText})`,
        icon: Plus,
        onclick: () => handleAddSelectedToSetlist(),
      },
      {
        id: 'add_chapter_to_setlist',
        label: `Adicionar Capítulo Completo à Setlist (${currentBook?.name} ${selectedChapter})`,
        icon: ListPlus,
        onclick: () => handleAddFullChapterToSetlist(),
      },
      {
        id: 'divider_1',
        divider: true,
      },
      {
        id: 'present_live',
        label: `Apresentar ao Vivo (${rangeText})`,
        icon: Play,
        onclick: () => handleVerseDoubleClick(verseNum),
      },
      {
        id: 'copy_text',
        label: 'Copiar Texto com Referência',
        icon: Copy,
        onclick: () => handleCopySelectedVerses(),
      },
    ];

    contextMenuOpen = true;
  }

  // --- BUSCA E PARSER DE REFERÊNCIAS ---
  async function handleSearchKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      const isShift = e.shiftKey;
      await executeSearch(isShift);
    }
  }

  async function executeSearch(immediatePresent: boolean = false) {
    const q = searchInput.trim();
    if (!q) return;

    // Se o usuário digitou diretamente a sigla de uma versão instalada (ex: "ARA", "NVI")
    const cleanWord = q.replace(/[^a-zA-Z]/g, '').toUpperCase();
    if (cleanWord.length >= 2 && cleanWord.length <= 6) {
      const matchedExactVersion = bibles.find(
        (b) => b.abbreviation.toUpperCase() === cleanWord
      );
      if (matchedExactVersion) {
        if (matchedExactVersion.id !== selectedBibleId) {
          selectedBibleId = matchedExactVersion.id;
          await loadBooks(matchedExactVersion.id);
        }
        searchInput = '';
        return;
      }
    }

    // Primeiro tenta o parser local de referência bíblica (ex: "Salmos 91 ARA", "João 3:16", "Sl 91")
    const parsed = parseBibleReference(q);
    if (parsed) {
      if (parsed.version) {
        const matched = bibles.find(
          (b) =>
            b.abbreviation.toLowerCase() === parsed.version!.toLowerCase() ||
            b.name.toLowerCase().includes(parsed.version!.toLowerCase())
        );
        if (matched && matched.id !== selectedBibleId) {
          selectedBibleId = matched.id;
          await loadBooks(matched.id);
        }
      }

      hasSearchResults = false;
      selectedBookId = parsed.book_id;
      selectedChapter = parsed.chapter;
      await loadChapters(selectedBibleId, parsed.book_id);
      await loadVerses(selectedBibleId, parsed.book_id, parsed.chapter);

      const targetVerse = parsed.verse_start || 1;
      selectedVerseNumbers = [targetVerse];
      lastClickedVerseNum = targetVerse;

      if (immediatePresent) {
        handleVerseDoubleClick(targetVerse);
      }
      return;
    }

    // Se não for referência, realiza busca textual por palavra
    isSearching = true;
    const hasQuotes =
      (q.startsWith('"') && q.endsWith('"') && q.length >= 2) ||
      (q.startsWith('“') && q.endsWith('”') && q.length >= 2);
    const isExact = matchExactWord || hasQuotes;

    try {
      const res = await fetch(
        `/api/bibles/${selectedBibleId}/search?q=${encodeURIComponent(q)}&exact=${isExact}&limit=50`
      );
      if (res.ok) {
        searchResults = await res.json();
        hasSearchResults = true;
      }
    } catch (err) {
      console.error('Erro ao buscar versículos:', err);
    } finally {
      isSearching = false;
    }
  }

  function handleSelectSearchResult(result: BibleSearchResult, immediatePresent: boolean = false) {
    selectedBookId = result.book_id;
    selectedChapter = result.chapter;
    hasSearchResults = false;

    loadChapters(selectedBibleId, result.book_id).then(() => {
      loadVerses(selectedBibleId, result.book_id, result.chapter).then(() => {
        selectedVerseNumbers = [result.verse];
        lastClickedVerseNum = result.verse;
        if (immediatePresent) {
          handleVerseDoubleClick(result.verse);
        }
      });
    });
  }

  // --- REDIMENSIONAMENTO DE COLUNAS COM DRAG ---
  function startDragCol1(e: MouseEvent) {
    isDraggingCol1 = true;
    const startX = e.clientX;
    const startWidth = col1Width;

    function onMouseMove(moveEvent: MouseEvent) {
      const delta = moveEvent.clientX - startX;
      col1Width = Math.max(140, Math.min(380, startWidth + delta));
    }

    function onMouseUp() {
      isDraggingCol1 = false;
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
    }

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  }

  function startDragCol2(e: MouseEvent) {
    isDraggingCol2 = true;
    const startX = e.clientX;
    const startWidth = col2Width;

    function onMouseMove(moveEvent: MouseEvent) {
      const delta = moveEvent.clientX - startX;
      col2Width = Math.max(110, Math.min(320, startWidth + delta));
    }

    function onMouseUp() {
      isDraggingCol2 = false;
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
    }

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden bg-background select-none font-sans">
  <!-- BARRA DE FERRAMENTAS SUPERIOR: VERSÃO, BUSCA/PARSER, TESTAMENTOS E AÇÕES -->
  <header class="p-2 border-b border-border bg-card/60 flex items-center justify-between gap-3 shrink-0 flex-wrap">
    <!-- Seletor de Versão da Bíblia -->
    <div class="flex items-center gap-2">
      <div class="relative">
        <select
          bind:value={selectedBibleId}
          onchange={() => {
            loadBooks(selectedBibleId);
          }}
          class="h-8 pl-8 pr-7 bg-muted/60 border border-border rounded-lg text-xs font-bold text-foreground cursor-pointer focus:outline-none focus:ring-1 focus:ring-primary appearance-none max-w-[200px]"
        >
          {#each bibles as b (b.id)}
            <option value={b.id}>
              {b.abbreviation} — {b.name}
            </option>
          {/each}
        </select>
        <BookOpen size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-primary pointer-events-none" />
        <ChevronDown size={12} class="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
      </div>

      {#if onopenbibleconfig}
        <Button
          variant="ghost"
          size="icon"
          class="h-8 w-8 text-muted-foreground hover:text-foreground"
          onclick={onopenbibleconfig}
          title="Configurações de Bíblias (Importar, Renomear)"
        >
          <Settings size={14} />
        </Button>
      {/if}
    </div>

    <!-- Campo de Busca Inteligente / Parser de Referências (Input Group) -->
    <div class="flex-1 max-w-md flex items-center bg-neutral-900 border border-neutral-800 rounded-lg overflow-hidden focus-within:ring-1 focus-within:ring-primary focus-within:border-primary/60 transition-all">
      <div class="pl-2.5 pr-1.5 flex items-center justify-center text-muted-foreground pointer-events-none">
        <Search size={13} />
      </div>

      <input
        type="text"
        bind:value={searchInput}
        onkeydown={handleSearchKeyDown}
        placeholder="Buscar referência (ex: Sl 91 ARA) ou palavra... [Shift+Enter projeta]"
        class="flex-1 h-8 bg-transparent border-0 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-0 px-1"
      />

      <div class="flex items-center gap-1 pr-1.5 shrink-0">
        <!-- Botão Toggle de Palavra Exata -->
        <button
          type="button"
          onclick={() => (matchExactWord = !matchExactWord)}
          class="px-1.5 py-0.5 rounded text-[10px] font-mono font-bold transition-all border cursor-pointer {matchExactWord ? 'bg-primary text-primary-foreground border-primary shadow-xs' : 'bg-neutral-800/80 text-muted-foreground hover:text-foreground border-neutral-700/60 hover:bg-neutral-800'}"
          title={matchExactWord
            ? 'Palavra exata ATIVA (busca estrita, não encontra amorreus). Clique para desligar.'
            : 'Ativar busca por Palavra Exata (ou use aspas na palavra: "amor")'}
        >
          [Ab]
        </button>

        {#if searchInput}
          <button
            type="button"
            onclick={() => {
              searchInput = '';
              hasSearchResults = false;
            }}
            class="text-muted-foreground hover:text-foreground p-0.5 rounded hover:bg-neutral-800 cursor-pointer transition-colors"
            title="Limpar busca"
          >
            <X size={12} />
          </button>
        {/if}
      </div>
    </div>

    <!-- Filtros de Testamento e Alternância de Visualização de Livros -->
    <div class="flex items-center gap-2">
      <!-- Filtro de Testamento -->
      <div class="flex items-center bg-muted/40 p-0.5 rounded-lg border border-border text-[11px]">
        <button
          type="button"
          onclick={() => (testamentFilter = 'all')}
          class="px-2 py-0.5 rounded transition-colors cursor-pointer {testamentFilter === 'all' ? 'bg-background text-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        >
          Todos
        </button>
        <button
          type="button"
          onclick={() => (testamentFilter = 'ot')}
          class="px-2 py-0.5 rounded transition-colors cursor-pointer {testamentFilter === 'ot' ? 'bg-background text-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        >
          AT
        </button>
        <button
          type="button"
          onclick={() => (testamentFilter = 'nt')}
          class="px-2 py-0.5 rounded transition-colors cursor-pointer {testamentFilter === 'nt' ? 'bg-background text-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        >
          NT
        </button>
      </div>

      <!-- Alternador de Nomes vs Abreviações Canônicas -->
      <div class="flex items-center bg-muted/40 p-0.5 rounded-lg border border-border text-[11px]">
        <button
          type="button"
          onclick={() => (bookViewMode = 'names')}
          class="px-2 py-0.5 rounded transition-colors cursor-pointer {bookViewMode === 'names' ? 'bg-primary/20 text-primary font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          title="Exibir nomes completos dos livros"
        >
          Nomes
        </button>
        <button
          type="button"
          onclick={() => (bookViewMode = 'abbreviations')}
          class="px-2 py-0.5 rounded transition-colors cursor-pointer {bookViewMode === 'abbreviations' ? 'bg-primary/20 text-primary font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          title="Exibir abreviações canônicas (ex: Gn, Ex, Mt)"
        >
          Siglas
        </button>
      </div>
    </div>

    <!-- Barra de Ação de Seleção Múltipla para Setlist -->
    {#if selectedVerseNumbers.length > 0}
      <div class="flex items-center gap-2 bg-primary/10 border border-primary/30 px-3 py-1 rounded-lg animate-in fade-in">
        <span class="text-xs font-bold text-primary">
          {selectedVerseNumbers.length} versículo{selectedVerseNumbers.length > 1 ? 's' : ''} selecionado{selectedVerseNumbers.length > 1 ? 's' : ''}
        </span>
        <Button
          size="sm"
          class="h-6 px-2.5 text-[11px] font-bold gap-1 cursor-pointer bg-primary text-primary-foreground"
          onclick={handleAddSelectedToSetlist}
        >
          <ListPlus size={12} />
          <span>+ Setlist</span>
        </Button>
        <button
          type="button"
          onclick={() => (selectedVerseNumbers = [])}
          class="text-muted-foreground hover:text-foreground p-0.5 cursor-pointer"
          title="Limpar seleção"
        >
          <X size={12} />
        </button>
      </div>
    {/if}
  </header>

  <!-- RESULTADOS DA BUSCA TEXTUAL (SE HOUVER) -->
  {#if hasSearchResults}
    <div class="p-2 border-b border-border bg-neutral-900/90 max-h-48 overflow-y-auto">
      <div class="flex items-center justify-between pb-1.5 mb-1.5 border-b border-border text-xs">
        <span class="font-bold text-foreground">
          Resultados da busca: "{searchInput}" ({searchResults.length})
        </span>
        <button
          type="button"
          onclick={() => (hasSearchResults = false)}
          class="text-xs text-muted-foreground hover:text-foreground cursor-pointer"
        >
          Fechar resultados
        </button>
      </div>

      <div class="space-y-1">
        {#each searchResults as r (r.book_id + '-' + r.chapter + '-' + r.verse)}
          <button
            type="button"
            onclick={() => handleSelectSearchResult(r, false)}
            ondblclick={() => handleSelectSearchResult(r, true)}
            class="w-full p-1.5 rounded text-left hover:bg-neutral-800 transition-colors flex items-start justify-between gap-2 cursor-pointer group"
          >
            <div class="flex-1 min-w-0">
              <span class="text-xs font-bold text-primary mr-1.5">
                {r.book_name} {r.chapter}:{r.verse}
              </span>
              <span class="text-xs text-neutral-300">
                {r.text}
              </span>
            </div>
            <span class="text-[10px] text-muted-foreground group-hover:text-primary shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
              Duplo clique projeta
            </span>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- CORPO PRINCIPAL COM AS 3 COLUNAS REDIMENSIONÁVEIS -->
  <div class="flex-1 flex overflow-hidden">
    <!-- ========================================================================= -->
    <!-- COLUNA 1: LIVROS DA BÍBLIA -->
    <!-- ========================================================================= -->
    <aside
      class="border-r border-border bg-card/30 flex flex-col shrink-0 overflow-hidden"
      style="width: {col1Width}px;"
    >
      <!-- Busca rápida de livros -->
      <div class="p-1.5 border-b border-border shrink-0">
        <input
          type="text"
          bind:value={bookSearchQuery}
          placeholder="Filtrar livro..."
          class="w-full h-6 px-2 bg-neutral-900 border border-neutral-800 rounded text-[11px] text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
        />
      </div>

      <!-- Lista ou Grade de Livros com Scroll -->
      <div class="flex-1 overflow-y-auto p-1">
        {#if bookViewMode === 'abbreviations'}
          <!-- Modo Grade com Siglas/Abreviações -->
          <div class="grid grid-cols-3 sm:grid-cols-4 gap-1">
            {#each filteredBooks as book (book.id)}
              <button
                type="button"
                onclick={() => handleSelectBook(book.id)}
                class="h-8 rounded text-center text-xs font-bold transition-all flex items-center justify-center cursor-pointer {book.id === selectedBookId ? 'bg-primary text-primary-foreground shadow-xs ring-1 ring-primary/40' : 'bg-neutral-900 border border-neutral-800 text-neutral-300 hover:bg-neutral-800 hover:text-white'}"
                title="{book.name} ({book.chapter_count} capítulos)"
              >
                <span>{book.abbreviation}</span>
              </button>
            {/each}
          </div>
        {:else}
          <!-- Modo Lista com Nomes Completos -->
          <div class="space-y-0.5">
            {#each filteredBooks as book (book.id)}
              <button
                type="button"
                onclick={() => handleSelectBook(book.id)}
                class="w-full px-2 py-1 rounded text-left text-xs font-medium transition-colors flex items-center justify-between cursor-pointer {book.id === selectedBookId ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-neutral-300 hover:bg-neutral-800/80 hover:text-white'}"
              >
                <span class="truncate">
                  {book.name}
                </span>
                <span class="text-[9px] opacity-70 shrink-0 font-mono ml-1">
                  {book.chapter_count} cap
                </span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </aside>

    <!-- DIVISOR REDIMENSIONÁVEL (SPLITTER 1) -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      onmousedown={startDragCol1}
      class="w-1 bg-border/60 hover:bg-primary/70 active:bg-primary cursor-col-resize shrink-0 transition-colors"
      title="Arraste para redimensionar coluna de livros"
    ></div>

    <!-- ========================================================================= -->
    <!-- COLUNA 2: CAPÍTULOS -->
    <!-- ========================================================================= -->
    <aside
      class="border-r border-border bg-card/20 flex flex-col shrink-0 overflow-hidden"
      style="width: {col2Width}px;"
    >
      <div class="p-1.5 border-b border-border text-[10px] font-bold uppercase tracking-wider text-muted-foreground shrink-0 text-center truncate">
        {currentBook.name} ({chapters.length})
      </div>

      <!-- Grade de botões numéricos dos capítulos -->
      <div class="flex-1 overflow-y-auto p-1.5">
        <div class="grid grid-cols-4 sm:grid-cols-5 gap-1">
          {#each chapters as ch (ch)}
            <button
              type="button"
              onclick={() => handleSelectChapter(ch)}
              class="h-8 rounded font-mono text-xs font-bold transition-all flex items-center justify-center cursor-pointer {ch === selectedChapter ? 'bg-primary text-primary-foreground shadow-sm ring-1 ring-primary/40' : 'bg-neutral-900 border border-neutral-800 text-neutral-300 hover:bg-neutral-800 hover:text-white'}"
            >
              {ch}
            </button>
          {/each}
        </div>
      </div>
    </aside>

    <!-- DIVISOR REDIMENSIONÁVEL (SPLITTER 2) -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      onmousedown={startDragCol2}
      class="w-1 bg-border/60 hover:bg-primary/70 active:bg-primary cursor-col-resize shrink-0 transition-colors"
      title="Arraste para redimensionar coluna de capítulos"
    ></div>

    <!-- ========================================================================= -->
    <!-- COLUNA 3: VERSÍCULOS EM SLIDES (GRADE 2 COLUNAS) -->
    <!-- ========================================================================= -->
    <main class="flex-1 flex flex-col overflow-hidden bg-neutral-950/40 p-2">
      <!-- Cabeçalho da passagem -->
      <div class="flex items-center justify-between pb-1.5 mb-2 border-b border-border shrink-0">
        <div class="flex items-center gap-2">
          <span class="text-sm font-bold text-foreground">
            {currentBook.name} {selectedChapter}
          </span>
          <Badge variant="outline" class="text-[10px] py-0 px-1 font-mono">
            {verses.length} versículos
          </Badge>
          <span class="text-[10px] text-muted-foreground ml-2 hidden sm:inline">
            Duplo clique para apresentar · Shift+clique para selecionar intervalo
          </span>
        </div>

        {#if bibles.length === 0}
          <div class="text-xs text-amber-400 flex items-center gap-1 font-semibold">
            <span>Nenhuma bíblia importada.</span>
            {#if onopenbibleconfig}
              <button
                type="button"
                onclick={onopenbibleconfig}
                class="underline cursor-pointer hover:text-amber-300"
              >
                Importar agora
              </button>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Grade dos Slides de Versículos (2 Colunas conforme especificado) -->
      <div class="flex-1 overflow-y-auto pr-1">
        {#if isLoadingVerses}
          <div class="h-full flex items-center justify-center text-xs text-muted-foreground">
            Carregando versículos...
          </div>
        {:else if verses.length === 0}
          <div class="h-full flex flex-col items-center justify-center text-center text-muted-foreground p-4">
            <BookOpen size={32} class="opacity-30 mb-2" />
            <p class="text-xs font-semibold">Nenhum versículo encontrado neste capítulo</p>
            <p class="text-[11px] opacity-75 mt-0.5">Selecione outro livro ou capítulo ao lado</p>
          </div>
        {:else}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5 pb-3">
            {#each verses as v (v.id)}
              {@const isSelected = selectedVerseNumbers.includes(v.verse)}
              {@const isLive = isScriptureLive && $presentationState.active_slide_index !== null && verses[$presentationState.active_slide_index]?.verse === v.verse}

              <!-- Cartão de Slide de Versículo -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                id="verse-slide-{v.verse}"
                onclick={(e) => handleVerseClick(v.verse, e)}
                ondblclick={() => handleVerseDoubleClick(v.verse)}
                oncontextmenu={(e) => handleVerseContextMenu(e, v.verse)}
                class="rounded-xl border p-2.5 flex flex-col justify-between transition-all cursor-pointer relative group {isLive ? 'border-emerald-500 bg-emerald-500/10 ring-2 ring-emerald-500/40' : isSelected ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/60 hover:bg-neutral-850 hover:border-neutral-700'}"
              >
                <!-- Topo do Slide: Referência e Indicadores -->
                <div class="flex items-center justify-between pb-1.5 mb-1.5 border-b border-border/50 text-xs shrink-0">
                  <div class="flex items-center gap-1.5">
                    <span class="w-5 h-5 rounded-full bg-neutral-800 text-neutral-300 font-mono text-[10px] font-bold flex items-center justify-center">
                      {v.verse}
                    </span>
                    <span class="font-bold {isLive ? 'text-emerald-400' : 'text-foreground'}">
                      {currentBook.abbreviation} {selectedChapter}:{v.verse}
                    </span>
                  </div>

                  <div class="flex items-center gap-1">
                    {#if isLive}
                      <span class="px-1.5 py-0.5 rounded bg-emerald-500 text-black text-[9px] font-black uppercase tracking-wider">
                        Ao Vivo
                      </span>
                    {/if}
                    <button
                      type="button"
                      onclick={(e) => {
                        e.stopPropagation();
                        handleVerseDoubleClick(v.verse);
                      }}
                      class="p-1 rounded hover:bg-neutral-800 text-neutral-400 hover:text-white opacity-0 group-hover:opacity-100 transition-opacity"
                      title="Apresentar este versículo ao vivo"
                    >
                      <Play size={12} class="fill-current" />
                    </button>
                  </div>
                </div>

                <!-- Corpo do Slide: Texto do Versículo -->
                <p class="text-xs sm:text-sm font-medium leading-relaxed text-neutral-200 line-clamp-4">
                  {v.text}
                </p>

                <!-- Rodapé com Versão -->
                <div class="pt-1.5 mt-1 border-t border-border/30 flex items-center justify-between text-[10px] text-muted-foreground font-mono">
                  <span>{currentBible?.abbreviation || 'Bíblia'}</span>
                  <span class="opacity-0 group-hover:opacity-100 transition-opacity text-primary font-sans font-semibold">
                    Duplo clique projeta
                  </span>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </main>
  </div>

  <ContextMenu
    open={contextMenuOpen}
    x={contextMenuX}
    y={contextMenuY}
    items={contextMenuItems}
    onclose={() => (contextMenuOpen = false)}
  />
</div>

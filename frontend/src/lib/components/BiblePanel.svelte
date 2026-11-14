<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    Search,
    BookOpen,
    Play,
    ListPlus,
    Plus,
    Copy,
    ChevronDown,
    Settings,
    X,
    RefreshCw,
    AlertTriangle,
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
  import { presentationState, quickPresentMedia, mediaItemsList } from '../ws';
  import { biblesStore, biblesErrorStore, refreshBibles } from '../bibles';

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
  let bibleError = $state<string | null>(null);

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

  // Mídia atualmente ao vivo no sistema (seja por quick_media avulsa ou item da Setlist)
  const activeMedia = $derived.by<MediaItem | undefined>(() => {
    const p = $presentationState;
    if (!p.active_media_id) return undefined;
    if (p.quick_media && p.quick_media.id === p.active_media_id) {
      return p.quick_media;
    }
    return $mediaItemsList.find((m) => m.id === p.active_media_id);
  });

  // Slide atualmente ao vivo
  const activeSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!activeMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    return activeMedia.slides[p.active_slide_index];
  });

  // Verifica se o slide projetado é DESTE livro e DESTE capítulo especificamente
  const isScriptureLive = $derived.by<boolean>(() => {
    const p = $presentationState;
    if (!p.active_media_id || p.display_mode !== 'live') return false;

    // 1. Verificação por ID canônico de passagem rápida (ex: scripture-ARA-52-5 ou scripture-52-5)
    const expectedQuickId = `scripture-${selectedBibleId}-${selectedBookId}-${selectedChapter}`;
    if (p.active_media_id === expectedQuickId) return true;
    if (p.active_media_id === `scripture-${selectedBookId}-${selectedChapter}`) return true;
    if (p.active_media_id.startsWith(`scripture-${selectedBookId}-${selectedChapter}-`)) return true;

    // 2. Verificação por mídia ativa (título da mídia ou rótulo do slide ativo)
    if (activeMedia && (activeMedia.kind === 'scripture' || activeMedia.id.startsWith('scripture-'))) {
      const bookName = currentBook.name.toLowerCase();
      const bookAbbr = currentBook.abbreviation.toLowerCase();
      const mediaTitle = (activeMedia.title || '').toLowerCase();

      // Checa título da mídia (ex: "1 Tessalonicenses 5" ou "1Ts 5")
      if (
        mediaTitle.startsWith(`${bookName} ${selectedChapter}`) ||
        mediaTitle.startsWith(`${bookAbbr} ${selectedChapter}`)
      ) {
        return true;
      }

      // Checa rótulo do slide ativo (ex: "1 Tessalonicenses 5:7")
      if (activeSlide) {
        const slideLabel = (activeSlide.label || '').toLowerCase();
        if (
          slideLabel.startsWith(`${bookName} ${selectedChapter}:`) ||
          slideLabel.startsWith(`${bookAbbr} ${selectedChapter}:`)
        ) {
          return true;
        }
      }
    }

    return false;
  });

  // Mantém o versículo ao vivo em vista se o capítulo estiver aberto
  $effect(() => {
    if (isScriptureLive && $presentationState.active_slide_index !== null) {
      const liveVerse = verses[$presentationState.active_slide_index];
      if (liveVerse) {
        const el = document.getElementById(`verse-slide-${liveVerse.verse}`);
        el?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
      }
    }
  });

  onMount(() => {
    loadBibles();
  });

  $effect(() => {
    if ($biblesErrorStore) {
      bibleError = $biblesErrorStore;
    }
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
    bibleError = null;
    try {
      await refreshBibles();
      if ($biblesErrorStore) {
        bibleError = $biblesErrorStore;
      }
    } finally {
      isLoadingBibles = false;
    }
  }

  type NavContext = { requestId: number; signal: AbortSignal };
  let navAbortController: AbortController | null = null;
  let navRequestId = 0;
  let searchAbortController: AbortController | null = null;
  let searchRequestId = 0;

  function beginNavRequest(): NavContext {
    navAbortController?.abort();
    navAbortController = new AbortController();
    return { requestId: ++navRequestId, signal: navAbortController.signal };
  }

  onDestroy(() => {
    navAbortController?.abort();
    searchAbortController?.abort();
  });

  async function loadBooks(
    bibleId: string,
    ctx: NavContext = beginNavRequest(),
    preferredBookId?: number,
    preferredChapter?: number
  ) {
    if (!bibleId) return;
    isLoadingVerses = true;
    bibleError = null;
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books`, { signal: ctx.signal });
      if (ctx.requestId !== navRequestId) return;
      if (!res.ok) {
        bibleError = `Falha ao carregar livros da Bíblia (HTTP ${res.status}).`;
        books = [];
        chapters = [];
        verses = [];
        selectedVerseNumbers = [];
        lastClickedVerseNum = null;
        isLoadingVerses = false;
        return;
      }
      const nextBooks: BibleBook[] = await res.json();
      if (ctx.requestId !== navRequestId) return;
      books = nextBooks;
      if (nextBooks.length > 0) {
        const targetBookId =
          preferredBookId && nextBooks.some((b) => b.id === preferredBookId)
            ? preferredBookId
            : nextBooks.some((b) => b.id === selectedBookId)
              ? selectedBookId
              : nextBooks[0].id;
        selectedBookId = targetBookId;
        await loadChapters(bibleId, targetBookId, ctx, preferredChapter);
      } else {
        chapters = [];
        verses = [];
        selectedVerseNumbers = [];
        lastClickedVerseNum = null;
        isLoadingVerses = false;
      }
    } catch (e: any) {
      if (e?.name === 'AbortError') return;
      console.error('Erro ao carregar livros:', e);
      if (ctx.requestId === navRequestId) {
        bibleError = 'Não foi possível carregar os livros da Bíblia.';
        verses = [];
        isLoadingVerses = false;
      }
    }
  }

  async function loadChapters(
    bibleId: string,
    bookId: number,
    ctx: NavContext = beginNavRequest(),
    preferredChapter?: number
  ) {
    isLoadingVerses = true;
    bibleError = null;
    try {
      const res = await fetch(`/api/bibles/${bibleId}/books/${bookId}/chapters`, { signal: ctx.signal });
      if (ctx.requestId !== navRequestId) return;
      if (!res.ok) {
        bibleError = `Falha ao carregar capítulos (HTTP ${res.status}).`;
        chapters = [];
        verses = [];
        selectedVerseNumbers = [];
        lastClickedVerseNum = null;
        isLoadingVerses = false;
        return;
      }
      const nextChapters: number[] = await res.json();
      if (ctx.requestId !== navRequestId) return;
      chapters = nextChapters;
      if (nextChapters.length > 0) {
        const targetChapter =
          preferredChapter && nextChapters.includes(preferredChapter)
            ? preferredChapter
            : nextChapters.includes(selectedChapter)
              ? selectedChapter
              : nextChapters[0];
        selectedChapter = targetChapter;
        await loadVerses(bibleId, bookId, targetChapter, ctx);
      } else {
        verses = [];
        selectedVerseNumbers = [];
        lastClickedVerseNum = null;
        isLoadingVerses = false;
      }
    } catch (e: any) {
      if (e?.name === 'AbortError') return;
      console.error('Erro ao carregar capítulos:', e);
      if (ctx.requestId === navRequestId) {
        bibleError = 'Não foi possível carregar os capítulos.';
        verses = [];
        isLoadingVerses = false;
      }
    }
  }

  async function loadVerses(
    bibleId: string,
    bookId: number,
    chapter: number,
    ctx: NavContext = beginNavRequest()
  ) {
    isLoadingVerses = true;
    bibleError = null;
    selectedVerseNumbers = [];
    lastClickedVerseNum = null;
    try {
      const res = await fetch(
        `/api/bibles/${bibleId}/books/${bookId}/chapters/${chapter}/verses`,
        { signal: ctx.signal }
      );
      if (ctx.requestId !== navRequestId) return;
      if (res.ok) {
        const nextVerses: BibleVerse[] = await res.json();
        if (ctx.requestId !== navRequestId) return;
        verses = nextVerses;
      } else {
        bibleError = `Falha ao carregar versículos (HTTP ${res.status}).`;
        verses = [];
      }
    } catch (e: any) {
      if (e?.name === 'AbortError') return;
      console.error('Erro ao carregar versículos:', e);
      if (ctx.requestId === navRequestId) {
        bibleError = 'Não foi possível carregar os versículos deste capítulo.';
        verses = [];
      }
    } finally {
      if (ctx.requestId === navRequestId) {
        isLoadingVerses = false;
      }
    }
  }

  async function handleVersionChange(newBibleId: string) {
    if (!newBibleId) return;
    selectedBibleId = newBibleId;
    const wasSearching = hasSearchResults;
    hasSearchResults = false;
    searchAbortController?.abort();
    await loadBooks(newBibleId);
    if (wasSearching && searchInput.trim()) {
      await executeTextSearch(searchInput.trim(), newBibleId);
    }
  }

  let prevTargetPassage = '';
  $effect(() => {
    if (targetPassage && bibles.length > 0) {
      const key = `${targetPassage.book_id}:${targetPassage.chapter}:${targetPassage.verse_start || 1}:${targetPassage.version || ''}`;
      if (key !== prevTargetPassage) {
        prevTargetPassage = key;
        jumpToPassage(targetPassage);
      }
    }
  });

  async function jumpToPassage(ref: ParsedReference) {
    const ctx = beginNavRequest();
    let bibleId = selectedBibleId;
    let versionChanged = false;
    if (ref.version) {
      const matched = bibles.find(
        (b) =>
          b.abbreviation.toLowerCase() === ref.version!.toLowerCase() ||
          b.name.toLowerCase().includes(ref.version!.toLowerCase())
      );
      if (matched && matched.id !== selectedBibleId) {
        selectedBibleId = matched.id;
        bibleId = matched.id;
        versionChanged = true;
      }
    }

    hasSearchResults = false;
    selectedBookId = ref.book_id;
    selectedChapter = ref.chapter;
    if (versionChanged) {
      await loadBooks(bibleId, ctx, ref.book_id, ref.chapter);
    } else {
      await loadChapters(bibleId, ref.book_id, ctx, ref.chapter);
    }
    if (ctx.requestId !== navRequestId) return;

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
    loadChapters(selectedBibleId, bookId, beginNavRequest(), 1);
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
        label: `${currentBook.name} ${selectedChapter}:${v.verse}`,
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
        label: `${currentBook.name} ${selectedChapter}:${v.verse}`,
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
        label: `${currentBook.name} ${selectedChapter}:${v.verse}`,
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

  function presentSelectedOrChapter(verseNum: number) {
    if (selectedVerseNumbers.length > 1 && selectedVerseNumbers.includes(verseNum)) {
      const selectedVerses = verses.filter((v) => selectedVerseNumbers.includes(v.verse));
      if (selectedVerses.length === 0) return;

      const abbr = currentBible?.abbreviation || 'Bíblia';
      const firstV = selectedVerseNumbers[0];
      const lastV = selectedVerseNumbers[selectedVerseNumbers.length - 1];
      const targetIdx = Math.max(
        0,
        selectedVerses.findIndex((v) => v.verse === verseNum)
      );

      const mediaItem: MediaItem = {
        id: `scripture-${selectedBibleId}-${selectedBookId}-${selectedChapter}-${firstV}-${lastV}`,
        title: `${currentBook.name} ${selectedChapter}:${firstV}-${lastV}`,
        artist_or_ref: abbr,
        kind: 'scripture',
        slides: selectedVerses.map((v) => ({
          id: `v-${v.verse}`,
          label: `${currentBook.name} ${selectedChapter}:${v.verse}`,
          content: v.text,
        })),
      };

      if (onpresentscripture) {
        onpresentscripture(mediaItem, targetIdx);
      } else {
        quickPresentMedia(mediaItem, targetIdx);
      }
      return;
    }

    handleVerseDoubleClick(verseNum);
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
        onclick: () => presentSelectedOrChapter(verseNum),
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
    const cleanWord = q.replace(/[^a-zA-Z0-9]/g, '').toUpperCase();
    if (cleanWord.length >= 2 && cleanWord.length <= 6) {
      const matchedExactVersion = bibles.find(
        (b) => b.abbreviation.toUpperCase() === cleanWord
      );
      if (matchedExactVersion) {
        if (matchedExactVersion.id !== selectedBibleId) {
          await handleVersionChange(matchedExactVersion.id);
        }
        searchInput = '';
        return;
      }
    }

    // Primeiro tenta o parser local de referência bíblica (ex: "Salmos 91 ARA", "João 3:16", "Sl 91")
    const parsed = parseBibleReference(q);
    if (parsed) {
      searchAbortController?.abort();
      const ctx = beginNavRequest();
      let bibleId = selectedBibleId;
      let versionChanged = false;
      if (parsed.version) {
        const matched = bibles.find(
          (b) =>
            b.abbreviation.toLowerCase() === parsed.version!.toLowerCase() ||
            b.name.toLowerCase().includes(parsed.version!.toLowerCase())
        );
        if (matched && matched.id !== selectedBibleId) {
          selectedBibleId = matched.id;
          bibleId = matched.id;
          versionChanged = true;
        }
      }

      hasSearchResults = false;
      selectedBookId = parsed.book_id;
      selectedChapter = parsed.chapter;
      if (versionChanged) {
        await loadBooks(bibleId, ctx, parsed.book_id, parsed.chapter);
      } else {
        await loadChapters(bibleId, parsed.book_id, ctx, parsed.chapter);
      }
      if (ctx.requestId !== navRequestId) return;

      const targetVerse = parsed.verse_start || 1;
      selectedVerseNumbers = [targetVerse];
      lastClickedVerseNum = targetVerse;

      if (immediatePresent) {
        handleVerseDoubleClick(targetVerse);
      }
      return;
    }

    await executeTextSearch(q, selectedBibleId);
  }

  async function executeTextSearch(q: string, bibleId: string) {
    if (!bibleId) return;
    searchAbortController?.abort();
    searchAbortController = new AbortController();
    const requestId = ++searchRequestId;

    isSearching = true;
    bibleError = null;
    const hasQuotes =
      (q.startsWith('"') && q.endsWith('"') && q.length >= 2) ||
      (q.startsWith('“') && q.endsWith('”') && q.length >= 2);
    const isExact = matchExactWord || hasQuotes;

    try {
      const res = await fetch(
        `/api/bibles/${bibleId}/search?q=${encodeURIComponent(q)}&exact=${isExact}&limit=50`,
        { signal: searchAbortController.signal }
      );
      if (requestId !== searchRequestId) return;
      if (res.ok) {
        const data: BibleSearchResult[] = await res.json();
        if (requestId !== searchRequestId) return;
        searchResults = data;
        hasSearchResults = true;
      } else {
        bibleError = `Falha na busca bíblica (HTTP ${res.status}).`;
      }
    } catch (err: any) {
      if (err?.name === 'AbortError') return;
      console.error('Erro ao buscar versículos:', err);
      bibleError = 'Não foi possível executar a busca na Bíblia.';
    } finally {
      if (requestId === searchRequestId) {
        isSearching = false;
      }
    }
  }

  async function handleSelectSearchResult(result: BibleSearchResult, immediatePresent: boolean = false) {
    const ctx = beginNavRequest();
    selectedBookId = result.book_id;
    selectedChapter = result.chapter;
    hasSearchResults = false;

    await loadChapters(selectedBibleId, result.book_id, ctx, result.chapter);
    if (ctx.requestId !== navRequestId) return;
    selectedVerseNumbers = [result.verse];
    lastClickedVerseNum = result.verse;
    if (immediatePresent) {
      handleVerseDoubleClick(result.verse);
    }
  }

  function retryBibleLoad() {
    if (bibles.length === 0 || !selectedBibleId) {
      loadBibles();
    } else if (books.length === 0) {
      loadBooks(selectedBibleId);
    } else if (chapters.length === 0) {
      loadChapters(selectedBibleId, selectedBookId);
    } else {
      loadVerses(selectedBibleId, selectedBookId, selectedChapter);
    }
  }

  // --- REDIMENSIONAMENTO DE COLUNAS COM POINTER E TECLADO ---
  function startDragCol(which: 1 | 2, e: PointerEvent) {
    e.preventDefault();
    const target = e.currentTarget as HTMLElement;
    const pointerId = e.pointerId;
    if (target?.setPointerCapture) {
      try {
        target.setPointerCapture(pointerId);
      } catch {
        // fallback window listeners
      }
    }

    if (which === 1) isDraggingCol1 = true;
    else isDraggingCol2 = true;

    const startX = e.clientX;
    const startWidth = which === 1 ? col1Width : col2Width;
    document.body.style.userSelect = 'none';

    function onPointerMove(moveEvent: PointerEvent) {
      const delta = moveEvent.clientX - startX;
      if (which === 1) {
        col1Width = Math.max(150, Math.min(360, startWidth + delta));
      } else {
        col2Width = Math.max(120, Math.min(300, startWidth + delta));
      }
    }

    function stopDrag() {
      isDraggingCol1 = false;
      isDraggingCol2 = false;
      document.body.style.userSelect = '';
      if (target?.releasePointerCapture) {
        try {
          if (target.hasPointerCapture?.(pointerId)) {
            target.releasePointerCapture(pointerId);
          }
        } catch {
          // noop
        }
      }
      window.removeEventListener('pointermove', onPointerMove);
      window.removeEventListener('pointerup', stopDrag);
      window.removeEventListener('pointercancel', stopDrag);
      window.removeEventListener('blur', stopDrag);
    }

    window.addEventListener('pointermove', onPointerMove);
    window.addEventListener('pointerup', stopDrag);
    window.addEventListener('pointercancel', stopDrag);
    window.addEventListener('blur', stopDrag);
  }

  function handleColSeparatorKeyDown(which: 1 | 2, e: KeyboardEvent) {
    const step = e.shiftKey ? 32 : 16;
    if (e.key === 'ArrowLeft') {
      e.preventDefault();
      if (which === 1) col1Width = Math.max(150, col1Width - step);
      else col2Width = Math.max(120, col2Width - step);
    } else if (e.key === 'ArrowRight') {
      e.preventDefault();
      if (which === 1) col1Width = Math.min(360, col1Width + step);
      else col2Width = Math.min(300, col2Width + step);
    } else if (e.key === 'Home') {
      e.preventDefault();
      if (which === 1) col1Width = 150;
      else col2Width = 120;
    } else if (e.key === 'End') {
      e.preventDefault();
      if (which === 1) col1Width = 360;
      else col2Width = 300;
    }
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden bg-background select-none font-sans">
  <h2 class="sr-only">Navegador da Bíblia</h2>

  <!-- BARRA DE FERRAMENTAS SUPERIOR: VERSÃO, BUSCA/PARSER, TESTAMENTOS E AÇÕES -->
  <header class="p-2 border-b border-border bg-card/70 flex items-center justify-between gap-2 shrink-0 flex-wrap">
    <!-- Seletor de Versão da Bíblia -->
    <div class="flex items-center gap-1.5">
      <div class="relative">
        <select
          value={selectedBibleId}
          aria-label="Versão da Bíblia"
          onchange={(e) => {
            const target = e.currentTarget as HTMLSelectElement;
            handleVersionChange(target.value);
          }}
          class="h-8 pl-8 pr-7 bg-background border border-input rounded-lg text-xs font-bold text-foreground cursor-pointer focus:outline-none focus:ring-2 focus:ring-primary/60 appearance-none max-w-[190px]"
        >
          {#each bibles as b (b.id)}
            <option value={b.id} class="bg-popover text-popover-foreground">
              {b.abbreviation} — {b.name}
            </option>
          {/each}
        </select>
        <BookOpen size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-primary pointer-events-none" />
        <ChevronDown size={12} class="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
      </div>

      <Button
        variant="ghost"
        size="icon"
        class="h-8 w-8 text-muted-foreground hover:text-foreground"
        onclick={loadBibles}
        aria-label="Recarregar versões da Bíblia"
        title="Recarregar versões da Bíblia"
      >
        <RefreshCw size={14} class={isLoadingBibles ? 'animate-spin' : ''} />
      </Button>

      {#if onopenbibleconfig}
        <Button
          variant="ghost"
          size="icon"
          class="h-8 w-8 text-muted-foreground hover:text-foreground"
          onclick={onopenbibleconfig}
          aria-label="Configurações de bíblias"
          title="Configurações de Bíblias (Importar, Renomear)"
        >
          <Settings size={14} />
        </Button>
      {/if}
    </div>

    <!-- Campo de Busca Inteligente / Parser de Referências -->
    <div class="flex-1 min-w-[220px] max-w-md flex items-center bg-background border border-input rounded-lg overflow-hidden focus-within:ring-2 focus-within:ring-primary/50 focus-within:border-primary/60 transition-all">
      <div class="pl-2.5 pr-1.5 flex items-center justify-center text-muted-foreground pointer-events-none">
        <Search size={13} />
      </div>

      <input
        type="text"
        bind:value={searchInput}
        onkeydown={handleSearchKeyDown}
        aria-label="Buscar referência ou palavra na Bíblia"
        placeholder="Buscar referência (ex: Sl 91) ou palavra... [Shift+Enter projeta]"
        class="flex-1 min-w-0 h-8 bg-transparent border-0 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-0 px-1"
      />

      <div class="flex items-center gap-1 pr-1 shrink-0">
        <button
          type="button"
          onclick={() => (matchExactWord = !matchExactWord)}
          aria-pressed={matchExactWord}
          aria-label="Alternar busca por palavra exata"
          class="h-7 px-2 rounded text-[11px] font-semibold transition-all border cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary {matchExactWord ? 'bg-primary text-primary-foreground border-primary shadow-xs' : 'bg-muted text-muted-foreground hover:text-foreground border-border hover:bg-muted/80'}"
          title={matchExactWord
            ? 'Palavra exata ativa (não encontra variações). Clique para desligar.'
            : 'Ativar busca por palavra exata (ou use aspas na busca)'}
        >
          Exata
        </button>

        {#if searchInput}
          <button
            type="button"
            onclick={() => {
              searchInput = '';
              hasSearchResults = false;
            }}
            aria-label="Limpar busca bíblica"
            class="w-7 h-7 flex items-center justify-center text-muted-foreground hover:text-foreground rounded hover:bg-muted cursor-pointer transition-colors"
            title="Limpar busca"
          >
            <X size={13} />
          </button>
        {/if}
      </div>
    </div>

    <!-- Filtros de Testamento e Alternância de Visualização de Livros -->
    <div class="flex items-center gap-1.5 flex-wrap">
      <div
        role="group"
        aria-label="Filtro por testamento"
        class="flex items-center bg-muted/60 p-0.5 rounded-lg border border-border text-[11px]"
      >
        <button
          type="button"
          onclick={() => (testamentFilter = 'all')}
          aria-pressed={testamentFilter === 'all'}
          class="h-7 px-2.5 rounded transition-colors cursor-pointer {testamentFilter === 'all' ? 'bg-background text-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
        >
          Todos
        </button>
        <button
          type="button"
          onclick={() => (testamentFilter = 'ot')}
          aria-pressed={testamentFilter === 'ot'}
          class="h-7 px-2 rounded transition-colors cursor-pointer {testamentFilter === 'ot' ? 'bg-background text-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          title="Antigo Testamento"
        >
          AT
        </button>
        <button
          type="button"
          onclick={() => (testamentFilter = 'nt')}
          aria-pressed={testamentFilter === 'nt'}
          class="h-7 px-2 rounded transition-colors cursor-pointer {testamentFilter === 'nt' ? 'bg-background text-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          title="Novo Testamento"
        >
          NT
        </button>
      </div>

      <div
        role="group"
        aria-label="Modo de exibição dos livros"
        class="flex items-center bg-muted/60 p-0.5 rounded-lg border border-border text-[11px]"
      >
        <button
          type="button"
          onclick={() => (bookViewMode = 'names')}
          aria-pressed={bookViewMode === 'names'}
          class="h-7 px-2 rounded transition-colors cursor-pointer {bookViewMode === 'names' ? 'bg-primary/15 text-primary font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          title="Exibir nomes completos dos livros"
        >
          Nomes
        </button>
        <button
          type="button"
          onclick={() => (bookViewMode = 'abbreviations')}
          aria-pressed={bookViewMode === 'abbreviations'}
          class="h-7 px-2 rounded transition-colors cursor-pointer {bookViewMode === 'abbreviations' ? 'bg-primary/15 text-primary font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          title="Exibir abreviações canônicas (ex: Gn, Ex, Mt)"
        >
          Siglas
        </button>
      </div>
    </div>

    <!-- Barra de Ação de Seleção Múltipla para Setlist -->
    {#if selectedVerseNumbers.length > 0}
      <div class="flex items-center gap-1.5 bg-primary/10 border border-primary/30 px-2.5 py-1 rounded-lg animate-in fade-in">
        <span class="text-xs font-bold text-primary">
          {selectedVerseNumbers.length} versículo{selectedVerseNumbers.length > 1 ? 's' : ''}
        </span>
        <Button
          size="sm"
          class="h-7 px-2.5 text-[11px] font-bold gap-1 cursor-pointer bg-primary text-primary-foreground"
          onclick={handleAddSelectedToSetlist}
        >
          <ListPlus size={12} />
          <span>+ Setlist</span>
        </Button>
        <button
          type="button"
          onclick={() => (selectedVerseNumbers = [])}
          aria-label="Limpar versículos selecionados"
          class="w-7 h-7 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
          title="Limpar seleção"
        >
          <X size={13} />
        </button>
      </div>
    {/if}
  </header>

  {#if bibleError}
    <div
      role="alert"
      class="px-3 py-1.5 border-b border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-between gap-2 text-xs shrink-0"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertTriangle size={14} class="shrink-0" />
        <span class="truncate font-medium">{bibleError}</span>
      </div>
      <Button
        variant="outline"
        size="sm"
        onclick={retryBibleLoad}
        class="h-7 px-2.5 text-[11px] shrink-0"
      >
        <RefreshCw size={12} />
        <span>Tentar novamente</span>
      </Button>
    </div>
  {/if}

  <!-- RESULTADOS DA BUSCA TEXTUAL (SE HOUVER) -->
  {#if hasSearchResults}
    <div class="p-2 border-b border-border bg-card/95 max-h-48 overflow-y-auto">
      <div class="flex items-center justify-between pb-1.5 mb-1.5 border-b border-border text-xs">
        <span class="font-bold text-foreground">
          Resultados da busca: "{searchInput}" ({searchResults.length})
        </span>
        <button
          type="button"
          onclick={() => (hasSearchResults = false)}
          class="h-7 px-2 rounded text-xs text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
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
            class="w-full p-2 rounded-lg text-left hover:bg-muted transition-colors flex items-start justify-between gap-2 cursor-pointer group"
          >
            <div class="flex-1 min-w-0">
              <span class="text-xs font-bold text-primary mr-1.5">
                {r.book_name} {r.chapter}:{r.verse}
              </span>
              <span class="text-xs text-foreground">
                {r.text}
              </span>
            </div>
            <span class="text-[11px] text-muted-foreground group-hover:text-primary shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
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
      aria-label="Livros da Bíblia"
      class="border-r border-border bg-card/50 flex flex-col shrink-0 overflow-hidden"
      style="width: {col1Width}px;"
    >
      <!-- Busca rápida de livros -->
      <div class="p-1.5 border-b border-border shrink-0">
        <input
          type="text"
          bind:value={bookSearchQuery}
          aria-label="Filtrar livros da Bíblia"
          placeholder="Filtrar livro..."
          class="w-full h-8 px-2.5 bg-background border border-input rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
        />
      </div>

      <!-- Lista ou Grade de Livros com Scroll -->
      <div class="flex-1 overflow-y-auto p-1">
        {#if bookViewMode === 'abbreviations'}
          <div class="grid grid-cols-3 sm:grid-cols-4 gap-1">
            {#each filteredBooks as book (book.id)}
              <button
                type="button"
                onclick={() => handleSelectBook(book.id)}
                class="h-8 rounded-md text-center text-xs font-bold transition-all flex items-center justify-center cursor-pointer {book.id === selectedBookId ? 'bg-primary text-primary-foreground shadow-xs ring-1 ring-primary/40' : 'bg-background border border-border text-foreground hover:bg-muted'}"
                title="{book.name} ({book.chapter_count} capítulos)"
              >
                <span>{book.abbreviation}</span>
              </button>
            {/each}
          </div>
        {:else}
          <div class="space-y-0.5">
            {#each filteredBooks as book (book.id)}
              <button
                type="button"
                onclick={() => handleSelectBook(book.id)}
                class="w-full min-h-[30px] px-2.5 py-1 rounded-md text-left text-xs font-medium transition-colors flex items-center justify-between cursor-pointer {book.id === selectedBookId ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-foreground hover:bg-muted'}"
              >
                <span class="truncate">
                  {book.name}
                </span>
                <span class="text-[11px] opacity-75 shrink-0 font-mono ml-1">
                  {book.chapter_count} cap
                </span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </aside>

    <!-- DIVISOR REDIMENSIONÁVEL (SPLITTER 1) -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
      role="separator"
      aria-orientation="vertical"
      aria-label="Ajustar largura da coluna de livros (setas esquerda ou direita)"
      aria-valuenow={col1Width}
      aria-valuemin={150}
      aria-valuemax={360}
      tabindex="0"
      onpointerdown={(e) => startDragCol(1, e)}
      onkeydown={(e) => handleColSeparatorKeyDown(1, e)}
      class="group relative w-3 -mx-1.5 flex items-center justify-center cursor-col-resize touch-none shrink-0 z-10 focus-visible:outline-none"
      title="Arraste ou use as setas para redimensionar livros"
    >
      <div
        class="h-full w-px transition-colors {isDraggingCol1 ? 'w-0.5 bg-primary' : 'bg-border group-hover:w-0.5 group-hover:bg-primary/70 group-focus-visible:w-0.5 group-focus-visible:bg-primary'}"
      ></div>
    </div>

    <!-- ========================================================================= -->
    <!-- COLUNA 2: CAPÍTULOS -->
    <!-- ========================================================================= -->
    <aside
      aria-label="Capítulos do livro selecionado"
      class="border-r border-border bg-card/30 flex flex-col shrink-0 overflow-hidden"
      style="width: {col2Width}px;"
    >
      <div class="p-2 border-b border-border text-[11px] font-bold uppercase tracking-wider text-muted-foreground shrink-0 text-center truncate">
        {currentBook.name} ({chapters.length})
      </div>

      <div class="flex-1 overflow-y-auto p-1.5">
        <div class="grid grid-cols-4 sm:grid-cols-5 gap-1">
          {#each chapters as ch (ch)}
            <button
              type="button"
              onclick={() => handleSelectChapter(ch)}
              class="h-8 rounded-md font-mono text-xs font-bold transition-all flex items-center justify-center cursor-pointer {ch === selectedChapter ? 'bg-primary text-primary-foreground shadow-sm ring-1 ring-primary/40' : 'bg-background border border-border text-foreground hover:bg-muted'}"
            >
              {ch}
            </button>
          {/each}
        </div>
      </div>
    </aside>

    <!-- DIVISOR REDIMENSIONÁVEL (SPLITTER 2) -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
      role="separator"
      aria-orientation="vertical"
      aria-label="Ajustar largura da coluna de capítulos (setas esquerda ou direita)"
      aria-valuenow={col2Width}
      aria-valuemin={120}
      aria-valuemax={300}
      tabindex="0"
      onpointerdown={(e) => startDragCol(2, e)}
      onkeydown={(e) => handleColSeparatorKeyDown(2, e)}
      class="group relative w-3 -mx-1.5 flex items-center justify-center cursor-col-resize touch-none shrink-0 z-10 focus-visible:outline-none"
      title="Arraste ou use as setas para redimensionar capítulos"
    >
      <div
        class="h-full w-px transition-colors {isDraggingCol2 ? 'w-0.5 bg-primary' : 'bg-border group-hover:w-0.5 group-hover:bg-primary/70 group-focus-visible:w-0.5 group-focus-visible:bg-primary'}"
      ></div>
    </div>

    <!-- ========================================================================= -->
    <!-- COLUNA 3: VERSÍCULOS EM SLIDES (GRADE 2 COLUNAS) -->
    <!-- ========================================================================= -->
    <main class="flex-1 flex flex-col overflow-hidden bg-muted/20 p-2 min-w-0">
      <div class="flex items-center justify-between pb-1.5 mb-2 border-b border-border shrink-0 gap-2 flex-wrap">
        <div class="flex items-center gap-2">
          <h3 class="text-sm font-bold text-foreground">
            {currentBook.name} {selectedChapter}
          </h3>
          <Badge variant="outline" class="text-[11px] py-0 px-1.5 font-mono">
            {verses.length} versículos
          </Badge>
          <span class="text-[11px] text-muted-foreground ml-1 hidden sm:inline">
            Duplo clique ou Enter apresenta · Shift+clique seleciona intervalo
          </span>
        </div>

        {#if bibles.length === 0}
          <div class="text-xs text-amber-700 dark:text-amber-300 flex items-center gap-1.5 font-semibold">
            <span>Nenhuma bíblia importada.</span>
            {#if onopenbibleconfig}
              <button
                type="button"
                onclick={onopenbibleconfig}
                class="h-7 px-2 rounded-md border border-amber-500/35 bg-amber-500/10 hover:bg-amber-500/20 cursor-pointer underline"
              >
                Importar agora
              </button>
            {/if}
          </div>
        {/if}
      </div>

      <div class="flex-1 overflow-y-auto pr-1">
        {#if isLoadingVerses}
          <div class="min-h-full flex items-center justify-center text-xs text-muted-foreground p-4">
            Carregando versículos...
          </div>
        {:else if bibleError && verses.length === 0}
          <div class="min-h-full flex flex-col items-center justify-center text-center text-muted-foreground p-3 gap-2">
            <AlertTriangle size={28} class="text-red-700 dark:text-red-300 opacity-85 shrink-0" />
            <p class="text-xs font-semibold text-foreground">Não foi possível carregar este capítulo</p>
            <Button variant="outline" size="sm" onclick={retryBibleLoad} class="h-8 text-xs gap-1.5 shrink-0">
              <RefreshCw size={12} />
              <span>Tentar novamente</span>
            </Button>
          </div>
        {:else if verses.length === 0}
          <div class="min-h-full flex flex-col items-center justify-center text-center text-muted-foreground p-4">
            <BookOpen size={32} class="opacity-30 mb-2" />
            <p class="text-xs font-semibold text-foreground">Nenhum versículo encontrado neste capítulo</p>
            <p class="text-xs text-muted-foreground mt-0.5">Selecione outro livro ou capítulo ao lado</p>
          </div>
        {:else}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5 pb-3">
            {#each verses as v (`${selectedBibleId}-${v.book_id}-${v.chapter}-${v.verse}`)}
              {@const isSelected = selectedVerseNumbers.includes(v.verse)}
              {@const isLive =
                isScriptureLive &&
                ((activeSlide &&
                  ((activeSlide.label && activeSlide.label.toLowerCase().endsWith(`:${v.verse}`)) ||
                    activeSlide.id === `v-${v.verse}`)) ||
                  ($presentationState.active_slide_index !== null &&
                    verses[$presentationState.active_slide_index]?.verse === v.verse))}

              <article
                id="verse-slide-{v.verse}"
                class="rounded-xl border flex flex-col transition-all group {isLive ? 'border-emerald-500 bg-emerald-500/10 ring-2 ring-emerald-500/40' : isSelected ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-border bg-card hover:bg-muted/40 hover:border-border'}"
              >
                <!-- Topo do Slide -->
                <div class="flex items-center justify-between px-2.5 pt-2 pb-1.5 border-b border-border/50 text-xs shrink-0 gap-2">
                  <div class="flex items-center gap-1.5 min-w-0">
                    <span class="w-6 h-6 rounded-full bg-muted text-foreground font-mono text-[11px] font-bold flex items-center justify-center shrink-0">
                      {v.verse}
                    </span>
                    <span class="font-bold truncate {isLive ? 'text-emerald-700 dark:text-emerald-300' : 'text-foreground'}">
                      {currentBook.abbreviation} {selectedChapter}:{v.verse}
                    </span>
                  </div>

                  <div class="flex items-center gap-1 shrink-0">
                    {#if isLive}
                      <span class="px-1.5 py-0.5 rounded bg-emerald-600 text-white text-[11px] font-bold uppercase tracking-wider">
                        Ao Vivo
                      </span>
                    {/if}
                    <button
                      type="button"
                      onclick={() => handleVerseDoubleClick(v.verse)}
                      aria-label="Apresentar {currentBook.abbreviation} {selectedChapter}:{v.verse} ao vivo"
                      class="w-7 h-7 rounded-md hover:bg-primary/15 text-muted-foreground hover:text-primary flex items-center justify-center opacity-75 group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary transition-opacity cursor-pointer"
                      title="Apresentar este versículo ao vivo"
                    >
                      <Play size={13} class="fill-current" />
                    </button>
                  </div>
                </div>

                <!-- Corpo Selecionável do Slide (Click/Space seleciona, Duplo Clique/Enter apresenta) -->
                <button
                  type="button"
                  aria-pressed={isSelected}
                  onclick={(e) => handleVerseClick(v.verse, e)}
                  ondblclick={() => handleVerseDoubleClick(v.verse)}
                  onkeydown={(e) => {
                    if (e.key === 'Enter') {
                      e.preventDefault();
                      handleVerseDoubleClick(v.verse);
                    }
                  }}
                  oncontextmenu={(e) => handleVerseContextMenu(e, v.verse)}
                  class="flex-1 text-left px-2.5 py-2 flex flex-col justify-between gap-2 cursor-pointer rounded-b-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
                >
                  <p class="text-xs sm:text-sm font-medium leading-relaxed text-foreground line-clamp-4">
                    {v.text}
                  </p>

                  <div class="w-full pt-1.5 border-t border-border/40 flex items-center justify-between text-[11px] text-muted-foreground font-mono">
                    <span>{currentBible?.abbreviation || 'Bíblia'}</span>
                    <span class="opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 transition-opacity text-primary font-sans font-semibold">
                      Duplo clique projeta
                    </span>
                  </div>
                </button>
              </article>
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

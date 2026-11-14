import type {
  BibleSearchResult,
  BibleVerse,
  BibleVersionSummary,
  MediaItem,
  OnlineLyricHit,
  ParsedReference,
  Song,
  SongSummary,
} from '../../types';
import { parseBibleReference } from '../../bibleParser';
import { quickPresentMedia } from '../../ws';
import { readHttpError } from '../../media';
import { notifyError } from '../../toast';
import { passageToMediaItem, searchResultToMediaItem, verseToMediaItem } from './media';
import { optionId, type CommandTab } from './types';

export interface CommandPaletteCallbacks {
  close: () => void;
  createNew: () => void;
  importSong: (song: Song) => void;
  openBible?: (ref: ParsedReference) => void;
  presentScripture?: (ref: ParsedReference) => void;
}

function isAbort(error: unknown): boolean {
  return (error as { name?: string } | null)?.name === 'AbortError';
}

function errorMessage(error: unknown, fallback: string): string {
  return error instanceof Error && error.message ? error.message : fallback;
}

export class CommandPaletteController {
  query = $state('');
  activeTab = $state<CommandTab>('local');
  localResults = $state<SongSummary[]>([]);
  onlineResults = $state<OnlineLyricHit[]>([]);
  bibles = $state<BibleVersionSummary[]>([]);
  selectedBibleId = $state('');
  bibleResults = $state<BibleSearchResult[]>([]);
  bibleVerses = $state<BibleVerse[]>([]);
  biblePassageRef = $state<ParsedReference | null>(null);
  addedIds = $state<Record<string, boolean>>({});
  addedVerseKeys = $state<Record<string, boolean>>({});
  isLoading = $state(false);
  selectedIndex = $state(0);
  bibleError = $state<string | null>(null);
  onlineError = $state<string | null>(null);

  hasQuery = $derived(this.query.trim().length > 0);
  currentBible = $derived(
    this.bibles.find((bible) => bible.id === this.selectedBibleId) || this.bibles[0]
  );
  parsedBibleRef = $derived.by<ParsedReference | null>(() => {
    const clean = this.query.trim();
    if (clean.length < 2) return null;
    return parseBibleReference(clean);
  });
  showQuickActions = $derived(this.activeTab === 'local' && !this.hasQuery);
  internetFallback = $derived(
    this.activeTab === 'local' &&
      this.hasQuery &&
      !this.isLoading &&
      this.localResults.length === 0
  );
  localOffset = $derived(this.showQuickActions ? 3 : 0);
  localOptionCount = $derived(
    this.localOffset + this.localResults.length + (this.internetFallback ? 1 : 0)
  );
  bibleOptionCount = $derived(
    this.bibleVerses.length > 1
      ? this.bibleVerses.length + 1
      : this.bibleVerses.length === 1
        ? 1
        : this.bibleResults.length
  );
  onlineOptionCount = $derived(this.onlineResults.length);
  optionCount = $derived(
    this.activeTab === 'local'
      ? this.localOptionCount
      : this.activeTab === 'bible'
        ? this.bibleOptionCount
        : this.onlineOptionCount
  );
  activeDescendant = $derived(
    this.optionCount > 0 ? optionId(this.activeTab, this.selectedIndex) : undefined
  );
  bibleCount = $derived(
    this.bibleVerses.length > 0 ? this.bibleVerses.length : this.bibleResults.length
  );

  #callbacks: CommandPaletteCallbacks;
  #debounceTimer: ReturnType<typeof setTimeout> | null = null;
  #searchAbort: AbortController | null = null;
  #searchSeq = 0;
  #lastTab: CommandTab | null = null;
  #lastQuery = '';
  #wasOpen = false;
  #previouslyFocused: HTMLElement | null = null;
  #inputFocus: () => void = () => {};

  constructor(callbacks: CommandPaletteCallbacks) {
    this.#callbacks = callbacks;
  }

  attachInputFocus(focus: () => void) {
    this.#inputFocus = focus;
  }

  #abortSearch() {
    this.#searchAbort?.abort();
    this.#searchAbort = null;
  }

  #beginSearch(): AbortController {
    this.#abortSearch();
    const controller = new AbortController();
    this.#searchAbort = controller;
    this.#searchSeq += 1;
    return controller;
  }

  #cancelPending() {
    this.#abortSearch();
    this.#searchSeq += 1;
  }

  async #loadBibles() {
    try {
      const response = await fetch('/api/bibles');
      if (!response.ok) return;
      this.bibles = await response.json();
      if (this.bibles.length > 0 && !this.selectedBibleId) {
        this.selectedBibleId = (this.bibles.find((bible) => bible.is_default) || this.bibles[0]).id;
      }
    } catch {
      this.bibles = [];
    }
  }

  async #searchLocal(value: string) {
    const clean = value.trim();
    const controller = this.#beginSearch();
    const seq = this.#searchSeq;
    this.isLoading = true;
    try {
      const response = await fetch(`/api/songs?q=${encodeURIComponent(clean)}`, {
        signal: controller.signal,
      });
      if (seq !== this.#searchSeq) return;
      this.localResults = response.ok ? await response.json() : [];
      this.selectedIndex = 0;
    } catch (error) {
      if (isAbort(error)) return;
      if (seq === this.#searchSeq) this.localResults = [];
    } finally {
      if (seq === this.#searchSeq) this.isLoading = false;
    }
  }

  async #searchOnline(value: string) {
    const clean = value.trim();
    this.onlineError = null;
    if (!clean) {
      this.#cancelPending();
      this.onlineResults = [];
      this.isLoading = false;
      return;
    }
    const controller = this.#beginSearch();
    const seq = this.#searchSeq;
    this.isLoading = true;
    try {
      const response = await fetch(`/api/songs/online/search?q=${encodeURIComponent(clean)}`, {
        signal: controller.signal,
      });
      if (seq !== this.#searchSeq) return;
      if (!response.ok) throw new Error(await readHttpError(response, 'Falha na busca online.'));
      this.onlineResults = await response.json();
      this.selectedIndex = 0;
    } catch (error) {
      if (isAbort(error)) return;
      if (seq === this.#searchSeq) {
        this.onlineResults = [];
        this.onlineError = errorMessage(error, 'Não foi possível buscar letras na internet.');
      }
    } finally {
      if (seq === this.#searchSeq) this.isLoading = false;
    }
  }

  async #searchBible(value: string, versionId = this.selectedBibleId) {
    const clean = value.trim();
    this.bibleError = null;
    if (!clean) {
      this.#cancelPending();
      this.bibleResults = [];
      this.bibleVerses = [];
      this.biblePassageRef = null;
      this.isLoading = false;
      return;
    }
    if (!versionId && this.bibles.length > 0) {
      versionId = (this.bibles.find((bible) => bible.is_default) || this.bibles[0]).id;
      this.selectedBibleId = versionId;
    }
    if (!versionId) return;

    const controller = this.#beginSearch();
    const seq = this.#searchSeq;
    this.isLoading = true;
    try {
      const parsed = parseBibleReference(clean);
      if (parsed) {
        this.biblePassageRef = parsed;
        let targetId = versionId;
        if (parsed.version) {
          const matched = this.bibles.find(
            (bible) =>
              bible.abbreviation.toLowerCase() === parsed.version!.toLowerCase() ||
              bible.name.toLowerCase().includes(parsed.version!.toLowerCase())
          );
          if (matched) {
            targetId = matched.id;
            this.selectedBibleId = matched.id;
          }
        }
        const response = await fetch(
          `/api/bibles/${targetId}/books/${parsed.book_id}/chapters/${parsed.chapter}/verses`,
          { signal: controller.signal }
        );
        if (seq !== this.#searchSeq) return;
        if (!response.ok) throw new Error(await readHttpError(response, 'Capítulo indisponível.'));
        const chapter: BibleVerse[] = await response.json();
        const start = parsed.verse_start || 1;
        const end = parsed.verse_end || parsed.verse_start || chapter.length;
        this.bibleVerses = chapter.filter((verse) => verse.verse >= start && verse.verse <= end);
        this.bibleResults = [];
        this.selectedIndex = 0;
        return;
      }

      this.biblePassageRef = null;
      this.bibleVerses = [];
      const response = await fetch(
        `/api/bibles/${versionId}/search?q=${encodeURIComponent(clean)}&limit=30`,
        { signal: controller.signal }
      );
      if (seq !== this.#searchSeq) return;
      if (!response.ok) throw new Error(await readHttpError(response, 'Falha na busca bíblica.'));
      this.bibleResults = await response.json();
      this.selectedIndex = 0;
    } catch (error) {
      if (isAbort(error)) return;
      if (seq === this.#searchSeq) {
        this.bibleResults = [];
        this.bibleVerses = [];
        this.bibleError = errorMessage(error, 'Não foi possível consultar a Bíblia.');
      }
    } finally {
      if (seq === this.#searchSeq) this.isLoading = false;
    }
  }

  #runSearch(tab: CommandTab, value: string) {
    if (tab === 'local') this.#searchLocal(value);
    else if (tab === 'bible') this.#searchBible(value);
    else this.#searchOnline(value);
  }

  switchTab = (tab: CommandTab, keepFocus = false) => {
    if (tab === this.activeTab) {
      if (!keepFocus) this.#inputFocus();
      return;
    }
    this.activeTab = tab;
    this.selectedIndex = 0;
    if (this.#debounceTimer) clearTimeout(this.#debounceTimer);
    this.#lastTab = tab;
    this.#lastQuery = this.query;
    this.#runSearch(tab, this.query);
    if (!keepFocus) setTimeout(() => this.#inputFocus(), 0);
  };

  handleVersionChange = (bibleId: string) => {
    this.selectedBibleId = bibleId;
    if (this.#debounceTimer) clearTimeout(this.#debounceTimer);
    this.#lastTab = 'bible';
    this.#lastQuery = this.query;
    this.#searchBible(this.query, bibleId);
  };

  retry = () => {
    if (this.activeTab === 'bible') this.#searchBible(this.query);
    else if (this.activeTab === 'online') this.#searchOnline(this.query);
  };

  searchExample = (example: string) => {
    this.query = example;
  };

  #resolveQuickAction(index: number) {
    if (index === 0) {
      this.#callbacks.close();
      this.#callbacks.createNew();
    } else if (index === 1) {
      this.switchTab('local');
      this.query = 'harpa';
    } else {
      this.switchTab('bible');
    }
  }

  activateLocal = (index: number) => {
    if (this.showQuickActions) {
      this.#resolveQuickAction(index);
      return;
    }
    const songIndex = index - this.localOffset;
    if (songIndex >= 0 && songIndex < this.localResults.length) {
      this.addToSetlist(this.localResults[songIndex].id);
      return;
    }
    if (this.internetFallback) this.switchTab('online');
  };

  #passageKey(): string {
    return `passage-${this.biblePassageRef?.book_id}-${this.biblePassageRef?.chapter}`;
  }

  #verseKey(verse: BibleVerse): string {
    return `verse-${verse.book_id}-${this.biblePassageRef?.chapter || verse.chapter}-${verse.verse}`;
  }

  #resultKey(result: BibleSearchResult): string {
    return `res-${result.book_id}-${result.chapter}-${result.verse}`;
  }

  #bibleMediaAt(index: number): { item: MediaItem; key: string } | null {
    const abbr = this.currentBible?.abbreviation || 'Bíblia';
    if (this.bibleVerses.length > 1) {
      if (index === 0) {
        return this.biblePassageRef
          ? {
              item: passageToMediaItem(this.bibleVerses, this.biblePassageRef, abbr),
              key: this.#passageKey(),
            }
          : null;
      }
      const verse = this.bibleVerses[index - 1];
      return verse
        ? {
            item: verseToMediaItem(verse, this.biblePassageRef, abbr),
            key: this.#verseKey(verse),
          }
        : null;
    }
    if (this.bibleVerses.length === 1) {
      return {
        item: verseToMediaItem(this.bibleVerses[0], this.biblePassageRef, abbr),
        key: this.#verseKey(this.bibleVerses[0]),
      };
    }
    const result = this.bibleResults[index];
    return result
      ? { item: searchResultToMediaItem(result, abbr), key: this.#resultKey(result) }
      : null;
  }

  handleBibleActivate = (index: number) => {
    const media = this.#bibleMediaAt(index);
    if (!media) return;
    quickPresentMedia(media.item, 0);
    this.#callbacks.close();
  };

  handleBibleAdd = (index: number) => {
    const media = this.#bibleMediaAt(index);
    if (media) this.#addScriptureToSetlist(media.item, media.key);
  };

  activateOnline = (index: number) => {
    const hit = this.onlineResults[index];
    if (hit) this.#importOnlineHit(hit);
  };

  openDetectedReference = () => {
    if (!this.parsedBibleRef) return;
    this.#callbacks.openBible?.(this.parsedBibleRef);
    this.#callbacks.close();
  };

  presentDetectedReference = () => {
    if (!this.parsedBibleRef) return;
    this.#callbacks.presentScripture?.(this.parsedBibleRef);
    this.#callbacks.close();
  };

  #flashAdded(id: string, state: 'song' | 'verse') {
    if (state === 'song') {
      this.addedIds = { ...this.addedIds, [id]: true };
      setTimeout(() => {
        const next = { ...this.addedIds };
        delete next[id];
        this.addedIds = next;
      }, 2000);
      return;
    }
    this.addedVerseKeys = { ...this.addedVerseKeys, [id]: true };
    setTimeout(() => {
      const next = { ...this.addedVerseKeys };
      delete next[id];
      this.addedVerseKeys = next;
    }, 2000);
  }

  addToSetlist = async (songId: string) => {
    try {
      const response = await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ song_id: songId }),
      });
      if (!response.ok) {
        throw new Error(await readHttpError(response, 'Falha ao adicionar à Setlist.'));
      }
      this.#flashAdded(songId, 'song');
    } catch (error) {
      notifyError('Não foi possível adicionar à Setlist', errorMessage(error, 'Tente novamente.'));
    }
  };

  async #addScriptureToSetlist(item: MediaItem, key: string) {
    try {
      const response = await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item }),
      });
      if (!response.ok) {
        throw new Error(await readHttpError(response, 'Falha ao adicionar à Setlist.'));
      }
      this.#flashAdded(key, 'verse');
    } catch (error) {
      notifyError(
        'Não foi possível adicionar o texto à Setlist',
        errorMessage(error, 'Tente novamente.')
      );
    }
  }

  async #importOnlineHit(hit: OnlineLyricHit) {
    if (!hit.provider || !hit.provider_id) return;
    this.onlineError = null;
    this.isLoading = true;
    try {
      const response = await fetch(
        `/api/songs/online/fetch?provider=${encodeURIComponent(hit.provider)}&id=${encodeURIComponent(hit.provider_id)}`
      );
      if (!response.ok) throw new Error(await readHttpError(response, 'Falha ao importar a letra.'));
      const song: Song = await response.json();
      this.#callbacks.close();
      this.#callbacks.importSong(song);
    } catch (error) {
      this.onlineError = errorMessage(error, 'Não foi possível importar esta letra.');
    } finally {
      this.isLoading = false;
    }
  }

  #moveSelection(delta: number) {
    if (this.optionCount <= 0) return;
    this.selectedIndex = (this.selectedIndex + delta + this.optionCount) % this.optionCount;
  }

  #activateCurrent() {
    if (this.activeTab === 'local') {
      if (this.parsedBibleRef) {
        this.openDetectedReference();
        return;
      }
      this.activateLocal(this.selectedIndex);
    } else if (this.activeTab === 'bible') {
      this.handleBibleActivate(this.selectedIndex);
    } else {
      this.activateOnline(this.selectedIndex);
    }
  }

  #addCurrent() {
    if (this.activeTab === 'local') {
      if (this.parsedBibleRef) {
        this.presentDetectedReference();
        return;
      }
      const song = this.localResults[this.selectedIndex - this.localOffset];
      if (song) this.addToSetlist(song.id);
    } else if (this.activeTab === 'bible') {
      this.handleBibleAdd(this.selectedIndex);
    }
  }

  #handleShortcut(event: KeyboardEvent): boolean {
    if (!(event.metaKey || event.ctrlKey) || event.altKey) return false;
    if (event.key === '1') this.switchTab('local');
    else if (event.key === '2') this.switchTab('bible');
    else if (event.key === '3') this.switchTab('online');
    else if (event.key.toLowerCase() === 'n') {
      this.#callbacks.close();
      this.#callbacks.createNew();
    } else if (event.key.toLowerCase() === 'h') {
      this.switchTab('local');
      this.query = 'harpa';
    } else if (event.key.toLowerCase() === 'b') this.switchTab('bible');
    else return false;
    event.preventDefault();
    return true;
  }

  handleKeydown = (event: KeyboardEvent) => {
    const target = event.target as HTMLElement | null;
    const role = target?.getAttribute('role');

    if (event.key === 'Escape') {
      event.preventDefault();
      this.#callbacks.close();
      return;
    }
    if (this.#handleShortcut(event)) return;

    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      if (target?.tagName === 'SELECT' || role === 'tab') return;
      event.preventDefault();
      this.#moveSelection(event.key === 'ArrowDown' ? 1 : -1);
      return;
    }

    if (event.key !== 'Enter') return;
    if (target && (target.tagName === 'BUTTON' || target.tagName === 'A' || target.tagName === 'SELECT')) {
      return;
    }
    event.preventDefault();
    if (event.shiftKey) this.#addCurrent();
    else this.#activateCurrent();
  };

  syncOpen(open: boolean) {
    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
      this.query = '';
      this.activeTab = 'local';
      this.localResults = [];
      this.onlineResults = [];
      this.bibleResults = [];
      this.bibleVerses = [];
      this.biblePassageRef = null;
      this.bibleError = null;
      this.onlineError = null;
      this.selectedIndex = 0;
      this.#lastTab = 'local';
      this.#lastQuery = '';
      this.#loadBibles();
      this.#searchLocal('');
      setTimeout(() => this.#inputFocus(), 50);
    } else if (!open && this.#wasOpen) {
      this.#wasOpen = false;
      this.#abortSearch();
      if (this.#debounceTimer) clearTimeout(this.#debounceTimer);
      const active = typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
      const shouldRestoreFocus =
        !active ||
        active === document.body ||
        Boolean(active.closest('[aria-label="Paleta de comandos"]'));
      if (shouldRestoreFocus) this.#previouslyFocused?.focus();
      this.#previouslyFocused = null;
    }
  }

  syncQuery(open: boolean, tab: CommandTab, value: string): (() => void) | undefined {
    if (!open) return;
    if (tab === this.#lastTab && value === this.#lastQuery) return;
    if (this.#debounceTimer) clearTimeout(this.#debounceTimer);
    this.#debounceTimer = setTimeout(() => {
      this.#lastTab = tab;
      this.#lastQuery = value;
      this.#runSearch(tab, value);
    }, 150);
    return () => {
      if (this.#debounceTimer) clearTimeout(this.#debounceTimer);
    };
  }

  destroy() {
    this.#abortSearch();
    if (this.#debounceTimer) clearTimeout(this.#debounceTimer);
  }
}

import { toast } from '$lib/toast';
import type { MediaItem, Slide, Song } from '$lib/types';
import { formatLyricsLinesPerSlide, parseRawLyrics } from '../../songParser';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import type { MenuItem } from '../ContextMenu.svelte';
import { buildInsertSectionMenu, buildSlideContextMenu } from './menus';
import {
  deleteSlide,
  duplicateSlide,
  getSlideLocations,
  setSlideSection,
} from './slides';
import { getInitialValues } from './defaults';
import { buildSongPayload, buildTextItemPayload } from './payload';
import { saveSong, saveTextPresentation } from './songEditorApi';
import type { AlignH, AlignV, SongEditorMode } from './types';

export interface SongEditorCallbacks {
  close: () => void;
  save?: (song: Song) => void;
  saveItem?: (item: MediaItem) => void;
}

export class SongEditorController {
  open = $state(false);
  id = $state('');
  isMaximized = $state(false);
  isLyricsMaximized = $state(false);
  title = $state('');
  artist = $state('');
  composer = $state('');
  songKey = $state('');
  bpm = $state<number | undefined>(undefined);
  lyricsRaw = $state('');
  tags = $state<string[]>([]);
  audioUrl = $state('');
  addToSetlist = $state(true);
  isSaving = $state(false);
  errorMessage = $state('');
  titleError = $state('');
  showDiscardConfirm = $state(false);
  showSectionName = $state(false);
  sectionName = $state('');
  sectionNameError = $state('');
  alignH = $state<AlignH>('center');
  alignV = $state<AlignV>('center');

  dialogEl = $state<HTMLDivElement | null>(null);
  textareaEl = $state<HTMLTextAreaElement | null>(null);

  contextMenuOpen = $state(false);
  contextMenuX = $state(0);
  contextMenuY = $state(0);
  contextMenuItems = $state<MenuItem[]>([]);

  #callbacks: SongEditorCallbacks;
  #mode: SongEditorMode = 'song';
  #hadExistingSong = false;
  #initialSnapshot = '';
  #wasOpen = false;
  #previouslyFocused: HTMLElement | null = null;
  #sectionNameTarget: number | null = null;

  parsed = $derived(parseRawLyrics(this.lyricsRaw));
  isDirty = $derived(this.open && this.#snapshot() !== this.#initialSnapshot);

  constructor(callbacks: SongEditorCallbacks) {
    this.#callbacks = callbacks;
  }

  syncOpen(
    open: boolean,
    mode: SongEditorMode,
    initialSong: Partial<Song> | null,
    initialItem: Partial<MediaItem> | null
  ) {
    this.open = open;
    this.#mode = mode;

    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      this.isLyricsMaximized = false;
      const values = getInitialValues(mode, initialSong, initialItem);
      this.id = values.id;
      this.title = values.title;
      this.artist = values.artist;
      this.composer = values.composer;
      this.songKey = values.songKey;
      this.bpm = values.bpm;
      this.lyricsRaw = values.lyricsRaw;
      this.tags = values.tags;
      this.audioUrl = values.audioUrl;
      this.alignH = values.alignH;
      this.alignV = values.alignV;
      this.#hadExistingSong = Boolean(initialSong?.id);
      this.errorMessage = '';
      this.titleError = '';
      this.showDiscardConfirm = false;
      this.showSectionName = false;
      this.sectionName = '';
      this.sectionNameError = '';
      this.#sectionNameTarget = null;
      this.contextMenuOpen = false;
      this.#initialSnapshot = this.#snapshot();
      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      setTimeout(() => {
        if (!this.dialogEl || !document.contains(this.dialogEl)) return;
        focusInitialElement(
          this.dialogEl,
          mode === 'song' ? '#song-title-input' : '#text-title-input',
          this.dialogEl
        );
      }, 20);
    } else if (!open && this.#wasOpen) {
      this.#wasOpen = false;
      this.showDiscardConfirm = false;
      this.showSectionName = false;
      this.contextMenuOpen = false;
      if (this.#previouslyFocused && document.contains(this.#previouslyFocused)) {
        this.#previouslyFocused.focus();
      }
      this.#previouslyFocused = null;
    }
  }

  #snapshot(): string {
    return JSON.stringify({
      title: this.title.trim(),
      artist: this.artist.trim(),
      composer: this.composer.trim(),
      songKey: this.songKey.trim(),
      bpm: this.bpm ? Number(this.bpm) : undefined,
      lyricsRaw: this.lyricsRaw.trim(),
      tags: [...this.tags].sort(),
      audioUrl: this.audioUrl.trim(),
      alignH: this.alignH,
      alignV: this.alignV,
    });
  }

  handleDialogKeydown(event: KeyboardEvent) {
    if (!this.open || this.showDiscardConfirm) return;
    if (this.dialogEl) trapTabKey(event, this.dialogEl);
  }

  handleKeydown(event: KeyboardEvent) {
    if (!this.open) return;

    if (event.key === 'Escape') {
      event.preventDefault();
      event.stopPropagation();
      if (this.showDiscardConfirm) this.cancelDiscard();
      else if (this.showSectionName) this.cancelSectionName();
      else if (this.contextMenuOpen) this.closeContextMenu();
      else this.requestClose();
      return;
    }

    if (this.showDiscardConfirm || this.contextMenuOpen) return;

    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 's') {
      event.preventDefault();
      void this.saveContent();
      return;
    }

    const isEditorFocused =
      (event.metaKey || event.ctrlKey) &&
      !event.shiftKey &&
      typeof document !== 'undefined' &&
      document.activeElement === this.textareaEl;
    if (isEditorFocused) {
      const key = event.key.toLowerCase();
      if (key === 'b') {
        event.preventDefault();
        this.applyBold();
      } else if (key === 'i') {
        event.preventDefault();
        this.applyItalic();
      } else if (key === 'u') {
        event.preventDefault();
        this.applyUnderline();
      }
    }
  }

  toggleLyricsMaximized() {
    this.isLyricsMaximized = !this.isLyricsMaximized;
  }

  requestClose() {
    if (this.isDirty) {
      this.showDiscardConfirm = true;
      return;
    }
    this.#callbacks.close();
  }

  confirmDiscard() {
    this.showDiscardConfirm = false;
    this.#callbacks.close();
  }

  cancelDiscard() {
    this.showDiscardConfirm = false;
  }

  #wrapSelection(prefix: string, suffix: string, defaultText: string) {
    if (!this.textareaEl) {
      this.lyricsRaw += `${prefix}${defaultText}${suffix}`;
      return;
    }

    const start = this.textareaEl.selectionStart;
    const end = this.textareaEl.selectionEnd;
    const selected = this.lyricsRaw.slice(start, end);
    const replacement = selected
      ? `${prefix}${selected}${suffix}`
      : `${prefix}${defaultText}${suffix}`;

    this.lyricsRaw = this.lyricsRaw.slice(0, start) + replacement + this.lyricsRaw.slice(end);

    setTimeout(() => {
      if (this.textareaEl) {
        this.textareaEl.focus();
        this.textareaEl.setSelectionRange(
          start + prefix.length,
          start + prefix.length + (selected ? selected.length : defaultText.length)
        );
      }
    }, 10);
  }

  applyBold() {
    this.#wrapSelection('**', '**', 'negrito');
  }

  applyItalic() {
    this.#wrapSelection('*', '*', 'itálico');
  }

  applyUnderline() {
    this.#wrapSelection('<u>', '</u>', 'sublinhado');
  }

  applyHighlight() {
    this.#wrapSelection('==', '==', 'destaque');
  }

  applyLinesPerSlide() {
    this.lyricsRaw = formatLyricsLinesPerSlide(this.lyricsRaw, 2);
    toast.success('Letra reorganizada no padrão de 2 linhas por slide!');
  }

  insertSectionTag(tag: string) {
    if (!this.textareaEl) {
      this.lyricsRaw += `\n\n## ${tag}\n`;
      return;
    }

    const start = this.textareaEl.selectionStart;
    const end = this.textareaEl.selectionEnd;
    let prefix = '## ';
    if (start > 0) {
      if (this.lyricsRaw.slice(0, start).endsWith('\n\n')) prefix = '## ';
      else if (this.lyricsRaw.slice(0, start).endsWith('\n')) prefix = '\n## ';
      else prefix = '\n\n## ';
    }

    const insertion = `${prefix}${tag}\n`;
    this.lyricsRaw = this.lyricsRaw.slice(0, start) + insertion + this.lyricsRaw.slice(end);
    const newCursor = start + insertion.length;

    setTimeout(() => {
      if (this.textareaEl) {
        this.textareaEl.focus();
        this.textareaEl.setSelectionRange(newCursor, newCursor);
      }
    }, 10);
  }

  get sectionNameIsInsert(): boolean {
    return this.#sectionNameTarget === null;
  }

  #openSectionName(target: number | null) {
    this.closeContextMenu();
    this.#sectionNameTarget = target;
    this.sectionName = '';
    this.sectionNameError = '';
    this.showSectionName = true;
  }

  promptInsertNewSection = () => {
    this.#openSectionName(null);
  };

  promptSlideSection = (slideIdx: number) => {
    this.#openSectionName(slideIdx);
  };

  confirmSectionName() {
    const name = this.sectionName.trim();
    if (!name) {
      this.sectionNameError = 'Informe um nome para a seção.';
      return;
    }

    const target = this.#sectionNameTarget;
    this.showSectionName = false;
    this.sectionNameError = '';
    this.#sectionNameTarget = null;

    if (target === null) this.insertSectionTag(name);
    else this.handleSetSlideSection(target, name);
  }

  cancelSectionName() {
    this.showSectionName = false;
    this.sectionName = '';
    this.sectionNameError = '';
    this.#sectionNameTarget = null;
  }

  handleSetSlideSection(slideIdx: number, label: string) {
    const next = setSlideSection(this.lyricsRaw, slideIdx, label);
    if (next !== null) {
      this.lyricsRaw = next;
      toast.success(`Seção alterada para "${label}"!`);
    }
  }

  handleDuplicateSlide(slideIdx: number) {
    const next = duplicateSlide(this.lyricsRaw, slideIdx);
    if (next !== null) {
      this.lyricsRaw = next;
      toast.success('Slide duplicado!');
    }
  }

  handleDeleteSlide(slideIdx: number) {
    const next = deleteSlide(this.lyricsRaw, slideIdx);
    if (next !== null) {
      this.lyricsRaw = next;
      toast.success('Slide removido!');
    }
  }

  focusSlideBlock(slideIdx: number) {
    const location = getSlideLocations(this.lyricsRaw)[slideIdx];
    if (!location || !this.textareaEl) return;
    this.textareaEl.focus({ preventScroll: true });
    this.textareaEl.setSelectionRange(location.blockStart, location.blockEnd);
  }

  #openContextMenu(x: number, y: number, items: MenuItem[]) {
    this.contextMenuX = x;
    this.contextMenuY = y;
    this.contextMenuItems = items;
    this.contextMenuOpen = true;
  }

  handleOpenSectionMenu = (event: MouseEvent) => {
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    this.#openContextMenu(
      rect.left,
      rect.bottom + 4,
      buildInsertSectionMenu(
        this.#mode,
        (label) => this.insertSectionTag(label),
        this.promptInsertNewSection
      )
    );
  };

  #openSlideMenu(x: number, y: number, slide: Slide, slideIdx: number) {
    this.#openContextMenu(
      x,
      y,
      buildSlideContextMenu(this.#mode, slide.label, slideIdx, {
        onSetSection: (idx, label) => this.handleSetSlideSection(idx, label),
        onNewSection: (idx) => this.promptSlideSection(idx),
        onDuplicate: (idx) => this.handleDuplicateSlide(idx),
        onDelete: (idx) => this.handleDeleteSlide(idx),
      })
    );
  }

  handleSlideContextMenu = (event: MouseEvent, slide: Slide, slideIdx: number) => {
    event.preventDefault();
    event.stopPropagation();
    this.#openSlideMenu(event.clientX, event.clientY, slide, slideIdx);
  };

  handleSlideActionsMenu = (event: Event, slide: Slide, slideIdx: number) => {
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    this.#openSlideMenu(rect.left, rect.bottom + 4, slide, slideIdx);
  };

  closeContextMenu() {
    this.contextMenuOpen = false;
  }

  clearTitleError() {
    if (this.titleError) this.titleError = '';
  }

  async saveContent() {
    this.errorMessage = '';

    if (!this.title.trim()) {
      this.titleError =
        this.#mode === 'song'
          ? 'O título da música é obrigatório.'
          : 'O título da apresentação é obrigatório.';
      const titleInput = this.dialogEl?.querySelector<HTMLInputElement>(
        this.#mode === 'song' ? '#song-title-input' : '#text-title-input'
      );
      titleInput?.focus();
      return;
    }

    if (
      this.#mode === 'song' &&
      this.bpm !== undefined &&
      (!Number.isFinite(this.bpm) || this.bpm < 20 || this.bpm > 400)
    ) {
      this.errorMessage = 'O BPM deve estar entre 20 e 400.';
      this.dialogEl?.querySelector<HTMLInputElement>('#song-bpm-input')?.focus();
      return;
    }

    this.isSaving = true;

    try {
      if (this.#mode === 'song') {
        const payload = buildSongPayload(
          {
            id: this.id,
            title: this.title,
            artist: this.artist,
            composer: this.composer,
            songKey: this.songKey,
            bpm: this.bpm,
            lyricsRaw: this.lyricsRaw,
            tags: this.tags,
            audioUrl: this.audioUrl,
          },
          this.parsed.sections
        );
        const saved = await saveSong(payload, this.addToSetlist);
        toast.success(
          this.#hadExistingSong
            ? 'Música atualizada com sucesso!'
            : 'Música salva com sucesso no catálogo!'
        );
        if (typeof window !== 'undefined') {
          window.dispatchEvent(new CustomEvent('openpresenter:songs-changed'));
        }
        this.#callbacks.save?.(saved);
      } else {
        const item = buildTextItemPayload(
          { id: this.id, title: this.title, artist: this.artist, audioUrl: this.audioUrl },
          this.parsed.slides,
          this.alignH,
          this.alignV
        );
        await saveTextPresentation(item);
        toast.success('Apresentação rápida adicionada à setlist!');
        this.#callbacks.saveItem?.(item);
      }
      this.#callbacks.close();
    } catch (err) {
      this.errorMessage = err instanceof Error ? err.message : 'Falha ao salvar o conteúdo';
      toast.error(this.errorMessage);
    } finally {
      this.isSaving = false;
    }
  }
}

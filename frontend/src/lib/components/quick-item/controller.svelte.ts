import type { MediaItem, Slide } from '$lib/types';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import { QUICK_ITEM_CONFIG } from './defaults';
import type { QuickItemCallbacks, QuickItemMode, QuickItemModeConfig } from './types';

export class QuickItemController {
  #callbacks: QuickItemCallbacks = {
    onclose: () => {},
    onsave: () => {},
  };

  open = $state(false);
  mode = $state<QuickItemMode>('header');
  isMaximized = $state(false);
  title = $state('');
  content = $state('');
  initialItem = $state<MediaItem | null>(null);
  isDiscardOpen = $state(false);

  dialogEl = $state<HTMLDivElement | null>(null);
  #previouslyFocused: HTMLElement | null = null;
  #initialSnapshot = '';
  #wasOpen = false;

  config = $derived<QuickItemModeConfig>(QUICK_ITEM_CONFIG[this.mode]);
  isEditing = $derived(this.initialItem !== null);

  slideCount = $derived.by(() => {
    if (this.mode === 'header') return 0;
    const trimmed = this.content.trim();
    if (!trimmed) return 1;
    return trimmed.split(/\n\s*\n/).filter((b) => b.trim().length > 0).length;
  });

  canSave = $derived(this.title.trim().length > 0);

  isDirty = $derived.by(() => {
    if (!this.open) return false;
    return this.getFormSnapshot() !== this.#initialSnapshot;
  });

  constructor(callbacks: QuickItemCallbacks) {
    this.#callbacks = callbacks;
  }

  getFormSnapshot(): string {
    return JSON.stringify({
      title: this.title.trim(),
      content: this.content.trim(),
    });
  }

  syncOpen(open: boolean, mode: QuickItemMode, item?: MediaItem | null) {
    this.open = open;
    this.mode = mode;

    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      this.isDiscardOpen = false;
      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      if (item) {
        this.initialItem = item;
        this.title = item.title;
        this.content = item.slides.map((s) => s.content).join('\n\n');
      } else {
        this.initialItem = null;
        this.title = '';
        this.content = '';
      }

      this.#initialSnapshot = this.getFormSnapshot();
      this.isMaximized = false;

      setTimeout(() => {
        if (this.dialogEl) {
          focusInitialElement(this.dialogEl, '#item-title', this.dialogEl);
        }
      }, 50);
    } else if (!open && this.#wasOpen) {
      this.#wasOpen = false;
      this.isDiscardOpen = false;
      this.restoreFocus();
    }
  }

  toggleMaximized() {
    this.isMaximized = !this.isMaximized;
  }

  requestClose() {
    if (this.isDirty) {
      this.isDiscardOpen = true;
      return;
    }
    this.close();
  }

  confirmDiscard() {
    this.isDiscardOpen = false;
    this.close();
  }

  cancelDiscard() {
    this.isDiscardOpen = false;
    setTimeout(() => {
      if (this.dialogEl) {
        focusInitialElement(this.dialogEl, '#item-title', this.dialogEl);
      }
    }, 30);
  }

  buildMediaItem(): MediaItem {
    const itemTitle = this.title.trim();
    let slides: Slide[] = [];

    if (this.mode === 'header') {
      slides = [];
    } else {
      const blocks = this.content.trim() ? this.content.split(/\n\s*\n/) : [itemTitle];
      slides = blocks
        .map((b) => b.trim())
        .filter((b) => b.length > 0)
        .map((block, idx) => ({
          id: `slide-${Date.now()}-${idx + 1}`,
          label: `Slide ${idx + 1}`,
          content: block,
        }));
    }

    return {
      id: this.initialItem?.id || `item-${Date.now()}`,
      title: itemTitle,
      artist_or_ref: this.config.kindLabel,
      kind: this.mode,
      slides,
    };
  }

  save() {
    if (!this.canSave) return;
    const item = this.buildMediaItem();
    this.#callbacks.onsave(item);
    this.close();
  }

  handleKeydown(e: KeyboardEvent) {
    if (!this.open) return;
    if (this.isDiscardOpen) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      this.requestClose();
      return;
    }

    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey || this.mode === 'header')) {
      e.preventDefault();
      e.stopPropagation();
      this.save();
      return;
    }

    if (this.dialogEl) {
      trapTabKey(e, this.dialogEl);
    }
  }

  close() {
    this.#callbacks.onclose();
  }

  restoreFocus() {
    if (this.#previouslyFocused && document.contains(this.#previouslyFocused)) {
      this.#previouslyFocused.focus();
    }
    this.#previouslyFocused = null;
  }
}

import type { MediaItem, Slide } from '$lib/types';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import type { TimerEditorCallbacks } from './types';
import { DEFAULT_TIMER_TITLE, DEFAULT_TIMER_SUBTITLE } from './defaults';

export class TimerEditorController {
  #callbacks: TimerEditorCallbacks = {
    onclose: () => {},
    onsave: () => {},
  };

  open = $state(false);
  isMaximized = $state(false);
  title = $state('');
  minutes = $state(5);
  seconds = $state(0);
  subtitle = $state('');
  allowOvertime = $state(true);
  initialItem = $state<MediaItem | null>(null);

  dialogEl = $state<HTMLDivElement | null>(null);
  #previouslyFocused: HTMLElement | null = null;
  #wasOpen = false;

  totalDurationSeconds = $derived(Math.max(0, this.minutes * 60 + this.seconds));

  formattedDisplayTime = $derived.by(() => {
    const totSecs = this.totalDurationSeconds;
    const m = Math.floor(totSecs / 60);
    const s = totSecs % 60;
    return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  });

  isEditing = $derived(this.initialItem !== null);

  constructor(callbacks: TimerEditorCallbacks) {
    this.#callbacks = callbacks;
  }

  syncOpen(isOpen: boolean, item?: MediaItem | null) {
    this.open = isOpen;

    if (isOpen && !this.#wasOpen) {
      this.#wasOpen = true;
      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      if (item && item.kind === 'timer') {
        this.initialItem = item;
        this.title = item.title;
        const dur = parseInt(item.slides[0]?.content || '300', 10);
        if (!isNaN(dur) && dur >= 0) {
          this.minutes = Math.floor(dur / 60);
          this.seconds = dur % 60;
        } else {
          this.minutes = 5;
          this.seconds = 0;
        }
        this.subtitle = item.slides[0]?.lower_content || '';
      } else {
        this.initialItem = null;
        this.title = DEFAULT_TIMER_TITLE;
        this.minutes = 5;
        this.seconds = 0;
        this.subtitle = DEFAULT_TIMER_SUBTITLE;
      }
      this.isMaximized = false;
      this.allowOvertime = true;

      setTimeout(() => {
        if (this.dialogEl) {
          focusInitialElement(this.dialogEl, '#timer-title', this.dialogEl);
        }
      }, 50);
    } else if (!isOpen && this.#wasOpen) {
      this.#wasOpen = false;
      this.restoreFocus();
    }
  }

  applyPreset(pMins: number, pSecs: number) {
    this.minutes = pMins;
    this.seconds = pSecs;
  }

  updateMinutes(val: number) {
    if (isNaN(val)) {
      this.minutes = 0;
    } else {
      this.minutes = Math.max(0, Math.min(599, Math.floor(val)));
    }
  }

  updateSeconds(val: number) {
    if (isNaN(val)) {
      this.seconds = 0;
    } else {
      this.seconds = Math.max(0, Math.min(59, Math.floor(val)));
    }
  }

  toggleOvertime() {
    this.allowOvertime = !this.allowOvertime;
  }

  toggleMaximized() {
    this.isMaximized = !this.isMaximized;
  }

  buildMediaItem(): MediaItem {
    const itemTitle = this.title.trim() || 'Contagem Regressiva';
    const totalSecs = Math.max(1, this.minutes * 60 + this.seconds);
    const formatted = this.formattedDisplayTime;

    const slide: Slide = {
      id: `timer-slide-${Date.now()}`,
      label: 'Contagem Regressiva',
      content: String(totalSecs),
      lower_content: this.subtitle.trim(),
    };

    return {
      id: this.initialItem?.id || `timer-${Date.now()}`,
      title: itemTitle,
      artist_or_ref: formatted,
      kind: 'timer',
      slides: [slide],
    };
  }

  saveOnly() {
    const item = this.buildMediaItem();
    this.#callbacks.onsave(item);
    this.close();
  }

  saveAndAddToSetlist() {
    const item = this.buildMediaItem();
    this.#callbacks.onsave(item);
    this.#callbacks.onaddtosetlist?.(item);
    this.close();
  }

  handleKeydown(e: KeyboardEvent) {
    if (!this.open) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      this.close();
      return;
    }

    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      e.preventDefault();
      e.stopPropagation();
      if (this.#callbacks.onaddtosetlist) {
        this.saveAndAddToSetlist();
      } else {
        this.saveOnly();
      }
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

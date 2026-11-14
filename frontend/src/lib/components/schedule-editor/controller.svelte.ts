import type { MediaItem, ScheduleEvent, Slide } from '$lib/types';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import { getDefaultEvents } from './defaults';
import type { ScheduleEditorCallbacks } from './types';

export class ScheduleEditorController {
  open = $state(false);
  isMaximized = $state(false);
  title = $state('Programação Semanal');
  artistOrRef = $state('Agenda');
  events = $state<ScheduleEvent[]>([]);
  selectedEventIndex = $state(0);
  error = $state<string | null>(null);
  showDiscardConfirm = $state(false);

  dialogEl = $state<HTMLDivElement | null>(null);

  #callbacks: ScheduleEditorCallbacks;
  #initialItem: MediaItem | null = null;
  #initialSnapshot = '';
  #wasOpen = false;
  #previouslyFocused: HTMLElement | null = null;

  activePreviewEvent = $derived(this.events[this.selectedEventIndex] || this.events[0]);
  isDirty = $derived(this.open && this.#snapshot() !== this.#initialSnapshot);

  constructor(callbacks: ScheduleEditorCallbacks) {
    this.#callbacks = callbacks;
  }

  #snapshot(): string {
    return JSON.stringify({
      title: this.title.trim(),
      artistOrRef: this.artistOrRef.trim(),
      events: this.events.map((ev) => ({
        dayOrDate: ev.dayOrDate.trim(),
        time: ev.time.trim(),
        title: ev.title.trim(),
        details: (ev.details || '').trim(),
      })),
    });
  }

  syncOpen(open: boolean, initialItem: MediaItem | null = null) {
    this.open = open;
    this.#initialItem = initialItem;

    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      this.error = null;
      this.showDiscardConfirm = false;

      if (initialItem && initialItem.kind === 'schedule') {
        this.title = initialItem.title || 'Programação Semanal';
        this.artistOrRef = initialItem.artist_or_ref || 'Agenda';

        const parsedEvents: ScheduleEvent[] = (initialItem.slides || []).map((s, idx) => {
          const lines = (s.content || '').split('\n').map((l) => l.trim()).filter(Boolean);
          let time = '';
          let eventTitle = '';
          let details = '';

          if (lines.length >= 3) {
            time = lines[0];
            eventTitle = lines[1];
            details = lines.slice(2).join(' • ');
          } else if (lines.length === 2) {
            if (/\d+[:hH]/.test(lines[0])) {
              time = lines[0];
              eventTitle = lines[1];
            } else {
              eventTitle = lines[0];
              details = lines[1];
            }
          } else if (lines.length === 1) {
            eventTitle = lines[0];
          }

          return {
            id: s.id || `event-${Date.now()}-${idx}`,
            dayOrDate: s.label || `Culto ${idx + 1}`,
            time: time || '19h30',
            title: eventTitle || 'Culto de Celebração',
            details: details || '',
          };
        });

        this.events = parsedEvents.length > 0 ? parsedEvents : getDefaultEvents();
      } else {
        this.title = 'Programação Semanal';
        this.artistOrRef = 'Agenda';
        this.events = getDefaultEvents();
      }

      this.selectedEventIndex = 0;
      this.#initialSnapshot = this.#snapshot();
      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      setTimeout(() => {
        if (!this.dialogEl || !document.contains(this.dialogEl)) return;
        focusInitialElement(
          this.dialogEl,
          '#sched-title, input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled])',
          this.dialogEl
        );
      }, 50);
    } else if (!open && this.#wasOpen) {
      this.#wasOpen = false;
      this.showDiscardConfirm = false;
      if (this.#previouslyFocused && document.contains(this.#previouslyFocused)) {
        this.#previouslyFocused.focus();
      }
      this.#previouslyFocused = null;
    }
  }

  addEvent(presetDay?: string, presetTime?: string, presetTitle?: string, presetDetails?: string) {
    this.error = null;
    const newEvent: ScheduleEvent = {
      id: `ev-${Date.now()}-${this.events.length + 1}`,
      dayOrDate: presetDay || 'Novo Culto / Evento',
      time: presetTime || '19h30',
      title: presetTitle || 'Culto de Celebração',
      details: presetDetails || '',
    };
    this.events = [...this.events, newEvent];
    this.selectedEventIndex = this.events.length - 1;
  }

  removeEvent(index: number) {
    if (this.events.length <= 1) {
      this.error = 'A agenda deve conter ao menos um culto ou evento.';
      return;
    }
    this.error = null;
    this.events = this.events.filter((_, idx) => idx !== index);
    if (this.selectedEventIndex >= this.events.length) {
      this.selectedEventIndex = Math.max(0, this.events.length - 1);
    }
  }

  moveEvent(index: number, direction: 'up' | 'down') {
    const targetIdx = direction === 'up' ? index - 1 : index + 1;
    if (targetIdx < 0 || targetIdx >= this.events.length) return;

    const copy = [...this.events];
    const temp = copy[index];
    copy[index] = copy[targetIdx];
    copy[targetIdx] = temp;
    this.events = copy;
    this.selectedEventIndex = targetIdx;
  }

  selectEvent(index: number) {
    if (index >= 0 && index < this.events.length) {
      this.selectedEventIndex = index;
    }
  }

  handleSave() {
    this.error = null;

    if (!this.title.trim()) {
      this.error = 'Informe um título geral para a programação.';
      return;
    }

    if (this.events.length === 0) {
      this.error = 'Adicione ao menos um evento à agenda.';
      return;
    }

    for (let i = 0; i < this.events.length; i++) {
      const ev = this.events[i];
      if (!ev.dayOrDate.trim() || !ev.title.trim()) {
        this.error = `Preencha o dia/data e o título do evento na posição #${i + 1}.`;
        this.selectedEventIndex = i;
        return;
      }
    }

    const slides: Slide[] = this.events.map((ev, idx) => {
      const parts: string[] = [];
      if (ev.time.trim()) parts.push(ev.time.trim());
      parts.push(ev.title.trim());
      if (ev.details?.trim()) parts.push(ev.details.trim());

      return {
        id: ev.id || `slide-${Date.now()}-${idx + 1}`,
        label: ev.dayOrDate.trim(),
        content: parts.join('\n'),
        align_h: 'center',
        align_v: 'center',
      };
    });

    const item: MediaItem = {
      id: this.#initialItem?.id || `schedule-${Date.now()}`,
      title: this.title.trim(),
      artist_or_ref: this.artistOrRef.trim() || 'Agenda',
      kind: 'schedule',
      slides,
      align_h: 'center',
      align_v: 'center',
    };

    this.#callbacks.save(item);
    this.#callbacks.close();
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

  handleKeydown(e: KeyboardEvent) {
    if (!this.open) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      if (this.showDiscardConfirm) {
        this.cancelDiscard();
      } else {
        this.requestClose();
      }
      return;
    }

    if (this.showDiscardConfirm) return;

    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      this.handleSave();
    }
  }

  handleDialogKeydown(e: KeyboardEvent) {
    if (!this.open || this.showDiscardConfirm || !this.dialogEl) return;
    trapTabKey(e, this.dialogEl);
  }
}

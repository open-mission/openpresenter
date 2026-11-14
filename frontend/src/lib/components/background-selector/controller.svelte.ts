import type { WallpaperItem } from '$lib/types';
import {
  encodeBackground,
  getAllBackgrounds,
  type BackgroundPreset,
  type BackgroundStylesMap,
} from '../../backgrounds';
import { backgroundStylesStore, DEFAULT_BACKGROUND_STYLES } from '../../displaySettings';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import type { BackgroundCategory, BackgroundSelectorCallbacks } from './types';

export class BackgroundSelectorController {
  open = $state(false);
  currentValue = $state('');
  selectedCategory = $state<BackgroundCategory>('all');
  searchQuery = $state('');
  localWallpapers = $state<WallpaperItem[]>([]);
  isLoadingLocal = $state(false);
  isMaximized = $state(false);

  dialogEl = $state<HTMLDivElement | null>(null);
  searchInputEl = $state<HTMLInputElement | null>(null);

  #backgroundStyles = $state<BackgroundStylesMap>(DEFAULT_BACKGROUND_STYLES);
  #unsubscribeStore: (() => void) | null = null;
  #callbacks: BackgroundSelectorCallbacks;
  #wasOpen = false;
  #previouslyFocused: HTMLElement | null = null;

  availablePresets = $derived(getAllBackgrounds(this.#backgroundStyles));

  filteredPresets = $derived.by(() => {
    const q = this.searchQuery.trim().toLowerCase();
    return this.availablePresets.filter((p) => {
      const matchesCat = this.selectedCategory === 'all' || p.category === this.selectedCategory;
      const matchesSearch =
        q === '' ||
        p.name.toLowerCase().includes(q) ||
        (p.description && p.description.toLowerCase().includes(q));
      return matchesCat && matchesSearch;
    });
  });

  filteredLocalWallpapers = $derived.by(() => {
    const q = this.searchQuery.trim().toLowerCase();
    return this.localWallpapers.filter((w) => {
      const matchesCat = this.selectedCategory === 'all' || this.selectedCategory === 'local';
      const matchesSearch = q === '' || w.name.toLowerCase().includes(q);
      return matchesCat && matchesSearch;
    });
  });

  totalCount = $derived(this.availablePresets.length + this.localWallpapers.length);
  animatedCount = $derived(this.availablePresets.filter((p) => p.category === 'animated').length);
  meshCount = $derived(this.availablePresets.filter((p) => p.category === 'mesh').length);
  gradientsCount = $derived(this.availablePresets.filter((p) => p.category === 'gradients').length);
  customCount = $derived(this.availablePresets.filter((p) => p.category === 'custom').length);
  localCount = $derived(this.localWallpapers.length);

  constructor(callbacks: BackgroundSelectorCallbacks) {
    this.#callbacks = callbacks;
  }

  syncOpen(open: boolean, currentValue: string = '') {
    this.open = open;
    this.currentValue = currentValue;

    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      this.searchQuery = '';
      this.selectedCategory = 'all';

      if (!this.#unsubscribeStore) {
        this.#unsubscribeStore = backgroundStylesStore.subscribe((val) => {
          this.#backgroundStyles = val;
        });
      }

      this.loadLocalWallpapers();

      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      setTimeout(() => {
        if (this.dialogEl) {
          focusInitialElement(this.dialogEl, '#bg-search-input', this.dialogEl);
        }
      }, 50);
    } else if (!open && this.#wasOpen) {
      this.#wasOpen = false;
      this.restoreFocus();
    }
  }

  destroy() {
    if (this.#unsubscribeStore) {
      this.#unsubscribeStore();
      this.#unsubscribeStore = null;
    }
    this.restoreFocus();
  }

  restoreFocus() {
    if (this.#previouslyFocused && document.contains(this.#previouslyFocused)) {
      this.#previouslyFocused.focus();
    }
    this.#previouslyFocused = null;
  }

  async loadLocalWallpapers() {
    this.isLoadingLocal = true;
    try {
      const res = await fetch('/api/media/wallpapers');
      if (res.ok) {
        this.localWallpapers = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar papéis de parede locais:', err);
    } finally {
      this.isLoadingLocal = false;
    }
  }

  selectPreset(preset: BackgroundPreset) {
    const encoded = encodeBackground(preset);
    this.#callbacks.onselect(encoded);
    this.close();
  }

  selectLocal(wall: WallpaperItem) {
    this.#callbacks.onselect(wall.url);
    this.close();
  }

  toggleMaximize() {
    this.isMaximized = !this.isMaximized;
  }

  close() {
    this.#callbacks.onclose();
  }

  handleKeydown(e: KeyboardEvent) {
    if (!this.open) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      this.close();
      return;
    }

    if (this.dialogEl) {
      trapTabKey(e, this.dialogEl);
    }
  }
}

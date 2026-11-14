import type { Setlist, WallpaperItem } from '$lib/types';
import { resolveBackground } from '../../backgrounds';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import { DEFAULT_WALLPAPER } from './defaults';
import type { SetlistEditorCallbacks, WallpaperMode } from './types';

export class SetlistController {
  open = $state(false);
  isMaximized = $state(false);
  title = $state('');
  description = $state('');
  selectedWallpaper = $state('');
  wallpaperMode = $state<WallpaperMode>('cover');
  availableWallpapers = $state<WallpaperItem[]>([]);
  isLoading = $state(false);
  isUploading = $state(false);
  isBackgroundModalOpen = $state(false);
  errorMessage = $state('');
  showDiscardConfirm = $state(false);

  dialogEl = $state<HTMLDivElement | null>(null);
  fileInputRef = $state<HTMLInputElement | null>(null);

  #callbacks: SetlistEditorCallbacks;
  #initialSetlist: Setlist | null = null;
  #initialSnapshot = '';
  #wasOpen = false;
  #previouslyFocused: HTMLElement | null = null;

  isEdit = $derived(!!this.#initialSetlist);
  resolvedWallpaper = $derived(resolveBackground(this.selectedWallpaper));
  isDirty = $derived(this.open && this.#snapshot() !== this.#initialSnapshot);

  constructor(callbacks: SetlistEditorCallbacks) {
    this.#callbacks = callbacks;
  }

  #snapshot(): string {
    return JSON.stringify({
      title: this.title.trim(),
      description: this.description.trim(),
      selectedWallpaper: this.selectedWallpaper,
      wallpaperMode: this.wallpaperMode,
    });
  }

  syncOpen(open: boolean, initialSetlist: Setlist | null = null) {
    this.open = open;
    this.#initialSetlist = initialSetlist;

    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      this.errorMessage = '';
      this.showDiscardConfirm = false;

      if (initialSetlist) {
        this.title = initialSetlist.title || '';
        this.description = initialSetlist.description || '';
        this.selectedWallpaper = initialSetlist.wallpaper_path || '';
        this.wallpaperMode = (initialSetlist.wallpaper_mode as WallpaperMode) || 'cover';
      } else {
        this.title = '';
        this.description = '';
        this.selectedWallpaper = DEFAULT_WALLPAPER;
        this.wallpaperMode = 'cover';
      }

      this.loadWallpapers();
      this.#initialSnapshot = this.#snapshot();
      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      setTimeout(() => {
        if (this.dialogEl) {
          focusInitialElement(this.dialogEl, '#setlist-title', this.dialogEl);
        }
      }, 50);
    } else if (!open && this.#wasOpen) {
      this.#wasOpen = false;
      this.restoreFocus();
    }
  }

  restoreFocus() {
    if (this.#previouslyFocused && document.contains(this.#previouslyFocused)) {
      this.#previouslyFocused.focus();
    }
    this.#previouslyFocused = null;
  }

  requestClose() {
    if (this.isDirty) {
      this.showDiscardConfirm = true;
      return;
    }
    this.#callbacks.onclose();
  }

  confirmDiscard() {
    this.showDiscardConfirm = false;
    this.#callbacks.onclose();
  }

  cancelDiscard() {
    this.showDiscardConfirm = false;
  }

  toggleMaximize() {
    this.isMaximized = !this.isMaximized;
  }

  async loadWallpapers() {
    try {
      const res = await fetch('/api/media/wallpapers');
      if (res.ok) {
        this.availableWallpapers = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar wallpapers:', err);
    }
  }

  async handleFileUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;
    const file = target.files[0];

    this.isUploading = true;
    this.errorMessage = '';

    try {
      const reader = new FileReader();
      reader.onload = async () => {
        try {
          const base64 = reader.result as string;
          const res = await fetch('/api/media/upload', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              filename: file.name,
              content_base64: base64,
            }),
          });

          if (res.ok) {
            const data = await res.json();
            this.selectedWallpaper = data.url;
            await this.loadWallpapers();
          } else {
            this.errorMessage = 'Falha ao salvar papel de parede na pasta do sistema.';
          }
        } catch {
          this.errorMessage = 'Erro ao processar imagem.';
        } finally {
          this.isUploading = false;
        }
      };
      reader.onerror = () => {
        this.errorMessage = 'Erro ao ler arquivo.';
        this.isUploading = false;
      };
      reader.readAsDataURL(file);
    } catch {
      this.errorMessage = 'Erro ao processar imagem.';
      this.isUploading = false;
    }
  }

  async save() {
    if (!this.title.trim()) {
      this.errorMessage = 'Por favor, insira um título para a Setlist.';
      return;
    }

    this.isLoading = true;
    this.errorMessage = '';

    const payload: Setlist = {
      id: this.#initialSetlist?.id || `setlist-${Date.now()}`,
      title: this.title.trim(),
      description: this.description.trim() || undefined,
      wallpaper_path: this.selectedWallpaper || undefined,
      wallpaper_mode: this.wallpaperMode,
      items: this.#initialSetlist?.items || [],
      is_active: this.#initialSetlist?.is_active ?? true,
      created_at: this.#initialSetlist?.created_at || '',
      updated_at: '',
    };

    try {
      const res = await fetch('/api/setlists', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      if (res.ok) {
        const saved = await res.json();
        this.#callbacks.onsave(saved);
        this.#callbacks.onclose();
      } else {
        this.errorMessage = 'Erro ao salvar Setlist.';
      }
    } catch {
      this.errorMessage = 'Falha na conexão com o servidor.';
    } finally {
      this.isLoading = false;
    }
  }

  handleKeydown(e: KeyboardEvent) {
    if (!this.open) return;

    if (this.showDiscardConfirm) {
      return;
    }

    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      e.preventDefault();
      e.stopPropagation();
      this.save();
      return;
    }

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      this.requestClose();
      return;
    }

    if (this.dialogEl) {
      trapTabKey(e, this.dialogEl);
    }
  }
}

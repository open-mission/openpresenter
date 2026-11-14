import type { ImageMediaItem, MediaFolder, MediaItem } from '$lib/types';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import type { ImageSelectorCallbacks, ImageSelectorMode } from './types';

export class ImageSelectorController {
  open = $state(false);
  mode = $state<ImageSelectorMode>('single');
  images = $state<ImageMediaItem[]>([]);
  folders = $state<MediaFolder[]>([]);
  selectedFolderFilter = $state<string>('all');
  searchQuery = $state<string>('');
  selectedImageIds = $state<Set<string>>(new Set());
  selectedFolderId = $state<string>('');
  isLoading = $state(false);
  isMaximized = $state(false);
  feedbackMessage = $state<string>('');

  dialogEl = $state<HTMLDivElement | null>(null);

  #callbacks: ImageSelectorCallbacks = { onclose: () => {} };
  #wasOpen = false;
  #previouslyFocused: HTMLElement | null = null;

  isLowerThird = $derived(!!this.#callbacks.onselectimage);

  filteredImages = $derived.by(() => {
    const q = this.searchQuery.trim().toLowerCase();
    return this.images.filter((img) => {
      const matchesFolder =
        this.selectedFolderFilter === 'all' ||
        (this.selectedFolderFilter === 'app' && !img.is_external) ||
        (this.selectedFolderFilter !== 'all' &&
          this.selectedFolderFilter !== 'app' &&
          img.folder_id === this.selectedFolderFilter);

      const matchesSearch =
        q === '' ||
        img.name.toLowerCase().includes(q) ||
        (img.subfolder && img.subfolder.toLowerCase().includes(q));

      return matchesFolder && matchesSearch;
    });
  });

  appImagesCount = $derived(this.images.filter((i) => !i.is_external).length);
  selectedCount = $derived(this.selectedImageIds.size);
  isAllSelected = $derived(
    this.filteredImages.length > 0 && this.selectedImageIds.size === this.filteredImages.length
  );

  constructor(callbacks: ImageSelectorCallbacks) {
    this.#callbacks = callbacks;
  }

  syncOpen(
    open: boolean,
    initialMode: ImageSelectorMode = 'single',
    allowFolders: boolean = true
  ) {
    this.open = open;

    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      this.mode = this.isLowerThird || !allowFolders ? 'single' : initialMode;
      this.selectedImageIds = new Set();
      this.searchQuery = '';
      this.selectedFolderFilter = 'all';
      this.feedbackMessage = '';
      this.loadData();

      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      setTimeout(() => {
        if (this.dialogEl) {
          focusInitialElement(this.dialogEl, '#image-search-input', this.dialogEl);
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

  async loadData() {
    this.isLoading = true;
    try {
      const [imgRes, fldRes] = await Promise.all([
        fetch('/api/media/images'),
        fetch('/api/media/folders'),
      ]);
      if (imgRes.ok) this.images = await imgRes.json();
      if (fldRes.ok) this.folders = await fldRes.json();
      if (this.folders.length > 0 && !this.selectedFolderId) {
        this.selectedFolderId = this.folders[0].id;
      }
    } catch (err) {
      console.error('Erro ao carregar imagens:', err);
    } finally {
      this.isLoading = false;
    }
  }

  toggleImageSelection(id: string) {
    this.feedbackMessage = '';
    if (this.isLowerThird) {
      this.selectedImageIds = this.selectedImageIds.has(id) ? new Set() : new Set([id]);
      return;
    }
    const next = new Set(this.selectedImageIds);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    this.selectedImageIds = next;
  }

  selectAll() {
    if (this.isLowerThird) return;
    this.feedbackMessage = '';
    if (this.isAllSelected) {
      this.selectedImageIds = new Set();
    } else {
      this.selectedImageIds = new Set(this.filteredImages.map((img) => img.id));
    }
  }

  confirmSingle() {
    if (this.selectedImageIds.size === 0) return;

    if (this.#callbacks.onselectimage) {
      const id = Array.from(this.selectedImageIds)[0];
      const img = this.images.find((i) => i.id === id);
      if (img) {
        this.#callbacks.onselectimage(img.url);
        this.close();
      }
      return;
    }

    const selectedList = this.images.filter((img) => this.selectedImageIds.has(img.id));
    const items: MediaItem[] = selectedList.map((img, idx) => ({
      id: `img-${Date.now()}-${idx}`,
      title: img.name,
      artist_or_ref: img.folder_name,
      kind: 'image',
      slides: [
        {
          id: `slide-img-${Date.now()}-${idx}`,
          label: 'Imagem',
          content: img.url,
        },
      ],
    }));

    this.#callbacks.onadditems?.(items);
    this.close();
  }

  confirmFolder() {
    let folderImages: ImageMediaItem[] = [];
    let title = 'Pasta de Imagens';
    let artist = 'Imagens';

    if (this.selectedFolderId === 'app') {
      folderImages = this.images.filter((i) => !i.is_external);
      title = 'Imagens do Aplicativo';
      artist = 'Pasta do Sistema';
    } else {
      const f = this.folders.find((fld) => fld.id === this.selectedFolderId);
      if (f) {
        folderImages = this.images.filter((i) => i.folder_id === f.id);
        title = f.name;
        artist = f.path;
      }
    }

    if (folderImages.length === 0) {
      this.feedbackMessage = 'Esta pasta não possui imagens para importar.';
      return;
    }

    const item: MediaItem = {
      id: `folder-img-${Date.now()}`,
      title,
      artist_or_ref: artist,
      kind: 'image',
      slides: folderImages.map((img, idx) => ({
        id: `slide-${Date.now()}-${idx}`,
        label: `Slide ${idx + 1}`,
        content: img.url,
      })),
    };

    this.#callbacks.onadditems?.([item]);
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

    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      e.preventDefault();
      e.stopPropagation();
      if (this.mode === 'single') {
        this.confirmSingle();
      } else {
        this.confirmFolder();
      }
      return;
    }

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

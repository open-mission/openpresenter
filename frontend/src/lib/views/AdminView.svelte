<script lang="ts">
  import { onMount } from 'svelte';
  import { cn } from '$lib/utils';
  import {
    presentationState,
    mediaItemsList,
    activeSetlist,
    isWsConnected,
    initWebSocket,
    selectSlide,
    setDisplayMode,
    stopPresentation as wsStopPresentation,
    toggleBlackout,
    toggleClearText,
    removeSetlistItem,
    clearSetlistItems,
    renameSetlistItem,
    reorderSetlistItems,
    switchActiveSetlist,
    startTimer,
    pauseTimer,
    resetTimer,
    quickPresentMedia,
    reloadOutputs,
  } from '$lib/ws';
  import type {
    MediaItem,
    MediaKind,
    Setlist,
    SetlistSummary,
    Slide,
    Song,
    ImageMediaItem,
    ParsedReference,
    BibleVersionSummary,
    BibleVerse,
  } from '$lib/types';
  import { renderFormattedText } from '$lib/markdown';
  import Button from '$lib/components/ui/Button.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import MediaItemRow from '$lib/components/ui/MediaItemRow.svelte';
  import CommandPalette from '$lib/components/CommandPalette.svelte';
  import SongEditorModal from '$lib/components/SongEditorModal.svelte';
  import SettingsModal, { type SettingsTab } from '$lib/components/SettingsModal.svelte';
  import SetlistModal from '$lib/components/SetlistModal.svelte';
  import QuickItemModal from '$lib/components/QuickItemModal.svelte';
  import BottomPanel from '$lib/components/BottomPanel.svelte';
  import ImageSelectorModal from '$lib/components/ImageSelectorModal.svelte';
  import ImagePreviewModal from '$lib/components/ImagePreviewModal.svelte';
  import ShareOutputsModal from '$lib/components/ShareOutputsModal.svelte';
  import StagePreviewBox from '$lib/components/StagePreviewBox.svelte';
  import PublicPreviewBox from '$lib/components/PublicPreviewBox.svelte';
  import TimerControlPanel from '$lib/components/TimerControlPanel.svelte';
  import ContextMenu, { type MenuItem } from '$lib/components/ContextMenu.svelte';
  import { fetchUpdateCheck, type UpdateCheckResponse } from '$lib/updater';
  import {
    syncDisplaySettingsFromBackend,
    stageSettingsStore,
    publicSettingsStore,
    churchBrandStore,
  } from '$lib/displaySettings';
  import { refreshBibles } from '$lib/bibles';
  import {
    themeStore,
    sectionsStore,
    previewLabelsStore,
    applyTheme,
    initAppTheme,
    getSectionStyle,
  } from '$lib/settings';
  import { resolveBackground } from '$lib/backgrounds';
  import {
    Search,
    MonitorPlay,
    Tv,
    EyeOff,
    Slash,
    ExternalLink,
    ChevronLeft,
    ChevronRight,
    ChevronDown,
    Wifi,
    WifiOff,
    Radio,
    Clock,
    Layers,
    Plus,
    Play,
    Pause,
    RotateCcw,
    RefreshCw,
    Timer,
    Square,
    Settings,
    Music,
    Bookmark,
    FileText,
    BookOpen,
    ArrowUpToLine,
    Pencil,
    Edit3,
    Trash2,
    Check,
    Eraser,
    Filter,
    PanelLeftClose,
    PanelLeftOpen,
    PanelRightClose,
    PanelRightOpen,
    ImageIcon,
    Folder,
    QrCode,
    Sparkles,
  } from 'lucide-svelte';

  let selectedMediaId = $state<string>('song-1');
  let searchQuery = $state<string>('');
  let filterKind = $state<MediaKind | 'all'>('all');
  let showCategoryFilters = $state(false);
  let stageClock = $state<string>('');
  let isCommandPaletteOpen = $state(false);
  let isSongEditorOpen = $state(false);
  let isSettingsOpen = $state(false);
  let isSetlistModalOpen = $state(false);
  let isQuickItemModalOpen = $state(false);
  let quickItemMode = $state<'header' | 'announcement'>('header');
  let isRenameModalOpen = $state(false);
  let isSetlistSelectorOpen = $state(false);
  let isAddMenuOpen = $state(false);
  let isImageSelectorOpen = $state(false);
  let imageSelectorMode = $state<'single' | 'folder'>('single');
  let isShareOutputsOpen = $state(false);
  let previewImage = $state<ImageMediaItem | null>(null);
  let isPreviewImageOpen = $state(false);

  // Controle de Sidebars Colapsáveis
  let isSetlistSidebarOpen = $state(true);
  let isPreviewSidebarOpen = $state(true);

  // Modo do Editor e Mídia em Edição
  let songEditorMode = $state<'song' | 'text'>('song');
  let editingSong = $state<Song | null>(null);
  let editingItem = $state<MediaItem | null>(null);
  let editingSetlist = $state<Setlist | null>(null);
  let editingQuickItem = $state<MediaItem | null>(null);
  let renamingItemId = $state<string>('');
  let renamingItemTitle = $state<string>('');
  let settingsInitialTab = $state<SettingsTab>('appearance');
  let bottomPanelRequestedTab = $state<'songs' | 'images' | 'backgrounds' | 'bible' | null>(null);
  let targetScriptureRef = $state<ParsedReference | null>(null);
  let draggedItemIndex = $state<number | null>(null);

  let setlistsList = $state<SetlistSummary[]>([]);
  let focusedSlideIndex = $state<number>(0);
  let numberBuffer = $state<string>('');
  let numberBufferTimer: ReturnType<typeof setTimeout> | null = null;
  let prevMediaId = $state<string>('');
  let updateInfo = $state<UpdateCheckResponse | null>(null);

  // Menu de Contexto
  let contextMenuOpen = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuItems = $state<MenuItem[]>([]);

  onMount(() => {
    initWebSocket('admin');
    initAppTheme();
    syncDisplaySettingsFromBackend();
    loadSetlists();

    fetchUpdateCheck()
      .then((res) => {
        updateInfo = res;
      })
      .catch(() => {});

    // Inicializa seleção padrão com a primeira mídia
    if ($mediaItemsList.length > 0 && !selectedMediaId) {
      selectedMediaId = $mediaItemsList[0].id;
    }

    const interval = setInterval(() => {
      const now = new Date();
      stageClock = now.toLocaleTimeString('pt-BR', { hour12: false });
    }, 1000);

    return () => clearInterval(interval);
  });

  async function loadSetlists() {
    try {
      const res = await fetch('/api/setlists');
      if (res.ok) {
        setlistsList = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar setlists:', err);
    }
  }

  // Título da Setlist atual
  const currentSetlistTitle = $derived.by(() => {
    if ($activeSetlist) return $activeSetlist.title;
    const found = setlistsList.find((s) => s.id === $presentationState.active_setlist_id || s.is_active);
    return found?.title || 'Culto Principal';
  });

  // Filtro de mídias por busca e tipo
  const filteredMedia = $derived.by(() => {
    return $mediaItemsList.filter((item) => {
      const matchesSearch =
        searchQuery === '' ||
        item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (item.artist_or_ref && item.artist_or_ref.toLowerCase().includes(searchQuery.toLowerCase()));

      const matchesKind = filterKind === 'all' || item.kind === filterKind;
      return matchesSearch && matchesKind;
    });
  });

  // Mídia selecionada para exibição no painel central
  const selectedMedia = $derived.by<MediaItem | undefined>(() => {
    if (
      $presentationState.quick_media &&
      $presentationState.quick_media.id === selectedMediaId
    ) {
      return $presentationState.quick_media;
    }
    return $mediaItemsList.find((m) => m.id === selectedMediaId) ?? $mediaItemsList[0];
  });

  // Mídia ativa ao vivo
  const liveMedia = $derived.by<MediaItem | undefined>(() => {
    const p = $presentationState;
    if (!p.active_media_id) return undefined;
    if (p.quick_media && p.quick_media.id === p.active_media_id) {
      return p.quick_media;
    }
    return $mediaItemsList.find((m) => m.id === p.active_media_id);
  });

  // Slide ativo ao vivo
  const liveSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!liveMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    return liveMedia.slides[p.active_slide_index];
  });

  // Próximo slide para o preview do stage
  const nextLiveSlide = $derived.by<Slide | undefined>(() => {
    const p = $presentationState;
    if (!liveMedia || p.active_slide_index === null || p.active_slide_index === undefined) {
      return undefined;
    }
    const nextIdx = p.active_slide_index + 1;
    if (nextIdx < liveMedia.slides.length) {
      return liveMedia.slides[nextIdx];
    }
    return undefined;
  });

  // Verifica se a mídia selecionada está ao vivo atualmente
  const isSelectedMediaLive = $derived.by(() => {
    return (
      $presentationState.active_media_id === selectedMedia?.id &&
      $presentationState.active_slide_index !== null &&
      $presentationState.active_slide_index !== undefined &&
      $presentationState.display_mode === 'live'
    );
  });

  const resolvedWallpaper = $derived(resolveBackground($presentationState.wallpaper_path));

  // Timer Global no Header
  const timer = $derived($presentationState.timer);
  const isTimerRunning = $derived(timer?.running ?? false);
  const isTimerOvertime = $derived((timer?.remaining_seconds ?? 0) <= 0 && isTimerRunning);
  const isTimerWarning = $derived(
    (timer?.remaining_seconds ?? 0) > 0 &&
      (timer?.remaining_seconds ?? 0) <= (timer?.warning_threshold_seconds || 60) &&
      isTimerRunning
  );

  function formatHeaderTimer(sec: number): { text: string; isNegative: boolean } {
    const isNegative = sec < 0;
    const abs = Math.abs(sec);
    const m = Math.floor(abs / 60);
    const s = abs % 60;
    const strM = String(m).padStart(2, '0');
    const strS = String(s).padStart(2, '0');
    return {
      text: `${isNegative ? '-' : ''}${strM}:${strS}`,
      isNegative,
    };
  }

  const formattedHeaderTimer = $derived(formatHeaderTimer(timer?.remaining_seconds ?? 0));

  // Sincroniza o slide focado quando a mídia selecionada muda
  $effect(() => {
    if (selectedMediaId !== prevMediaId) {
      prevMediaId = selectedMediaId;
      if (
        $presentationState.active_media_id === selectedMediaId &&
        $presentationState.active_slide_index !== null &&
        $presentationState.active_slide_index !== undefined
      ) {
        focusedSlideIndex = $presentationState.active_slide_index;
      } else {
        focusedSlideIndex = 0;
      }
    }
  });

  // Disparo e controle de apresentação
  function selectSlideForFocus(idx: number) {
    focusedSlideIndex = idx;
  }

  function presentSlide(idx: number = focusedSlideIndex) {
    if (!selectedMedia || selectedMedia.slides.length === 0) return;
    const clamped = Math.max(0, Math.min(idx, selectedMedia.slides.length - 1));
    focusedSlideIndex = clamped;
    selectSlide(selectedMedia.id, clamped);
  }

  function stopPresentation() {
    wsStopPresentation();
  }

  // Navegação de slides
  function nextSlide() {
    if (!selectedMedia || selectedMedia.slides.length === 0) return;
    if (focusedSlideIndex < selectedMedia.slides.length - 1) {
      focusedSlideIndex++;
      if (isSelectedMediaLive) {
        selectSlide(selectedMedia.id, focusedSlideIndex);
      }
    }
  }

  function prevSlide() {
    if (!selectedMedia || selectedMedia.slides.length === 0) return;
    if (focusedSlideIndex > 0) {
      focusedSlideIndex--;
      if (isSelectedMediaLive) {
        selectSlide(selectedMedia.id, focusedSlideIndex);
      }
    }
  }

  // Atalho rápido para seções da música
  function jumpToSection(key: string): boolean {
    if (!selectedMedia || selectedMedia.slides.length === 0) return false;
    const k = key.toLowerCase();

    const matchingConfig = $sectionsStore.find(
      (s) => s.shortcut && s.shortcut.toLowerCase() === k
    );

    const searchTerms: string[] = [];
    if (matchingConfig) {
      searchTerms.push(matchingConfig.name.toLowerCase(), matchingConfig.id.toLowerCase());
    }
    if (k === 'v') searchTerms.push('verso', 'verse', 'estrofe');
    if (k === 'r') searchTerms.push('refrão', 'refrao', 'chorus', 'coro');
    if (k === 'p') searchTerms.push('ponte', 'bridge');
    if (k === 'i') searchTerms.push('intro', 'início');
    if (k === 'f') searchTerms.push('final', 'outro', 'fim');

    if (searchTerms.length === 0) return false;

    const matchingIndices: number[] = [];
    selectedMedia.slides.forEach((slide, idx) => {
      const label = (slide.label || '').toLowerCase();
      if (searchTerms.some((term) => label.includes(term))) {
        matchingIndices.push(idx);
      }
    });

    if (matchingIndices.length === 0) return false;

    const nextIdx = matchingIndices.find((idx) => idx > focusedSlideIndex) ?? matchingIndices[0];
    focusedSlideIndex = nextIdx;
    if (isSelectedMediaLive) {
      selectSlide(selectedMedia.id, nextIdx);
    }
    return true;
  }

  // Buffer numérico para saltar para slide ordinal
  function appendNumberKey(digit: string) {
    if (numberBufferTimer) clearTimeout(numberBufferTimer);
    numberBuffer += digit;
    numberBufferTimer = setTimeout(() => {
      numberBuffer = '';
    }, 2500);
  }

  function submitNumberBuffer(): boolean {
    if (!numberBuffer) return false;
    const num = parseInt(numberBuffer, 10);
    numberBuffer = '';
    if (numberBufferTimer) {
      clearTimeout(numberBufferTimer);
      numberBufferTimer = null;
    }
    if (!isNaN(num) && selectedMedia && selectedMedia.slides.length > 0) {
      const targetIdx = Math.max(0, Math.min(num - 1, selectedMedia.slides.length - 1));
      presentSlide(targetIdx);
      return true;
    }
    return false;
  }

  // Atalhos de teclado globais
  function handleKeydown(e: KeyboardEvent) {
    // Atalho global do Command Palette (⌘K / Ctrl+K)
    if ((e.metaKey || e.ctrlKey) && (e.key === 'k' || e.key === 'K')) {
      e.preventDefault();
      isCommandPaletteOpen = !isCommandPaletteOpen;
      return;
    }

    // Atalho global para abrir a aba de Bíblia (⌘B / Ctrl+B ou F4)
    if (((e.metaKey || e.ctrlKey) && (e.key === 'b' || e.key === 'B')) || e.key === 'F4') {
      e.preventDefault();
      handleOpenBibleTab();
      return;
    }

    // Se estiver digitando em input ou textarea, ignorar
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

    // Se algum modal estiver aberto
    if (
      isCommandPaletteOpen ||
      isSongEditorOpen ||
      isSettingsOpen ||
      isSetlistModalOpen ||
      isQuickItemModalOpen ||
      isRenameModalOpen
    ) {
      if (e.key === 'Escape') {
        isCommandPaletteOpen = false;
        isSongEditorOpen = false;
        isSettingsOpen = false;
        isSetlistModalOpen = false;
        isQuickItemModalOpen = false;
        isRenameModalOpen = false;
        isSetlistSelectorOpen = false;
        isAddMenuOpen = false;
      }
      return;
    }

    // Tecla Escape: Para a apresentação
    if (e.key === 'Escape') {
      e.preventDefault();
      if (isSetlistSelectorOpen || isAddMenuOpen) {
        isSetlistSelectorOpen = false;
        isAddMenuOpen = false;
        return;
      }
      if (numberBuffer) {
        numberBuffer = '';
        if (numberBufferTimer) clearTimeout(numberBufferTimer);
        return;
      }
      stopPresentation();
      return;
    }

    // Tecla Enter
    if (e.key === 'Enter') {
      e.preventDefault();
      if (numberBuffer) {
        submitNumberBuffer();
      } else {
        presentSlide(focusedSlideIndex);
      }
      return;
    }

    // Teclas 0-9 para buffer de número
    if (/^[0-9]$/.test(e.key)) {
      e.preventDefault();
      appendNumberKey(e.key);
      return;
    }

    // Atalhos Dedicados de Saída (sem conflito com seções ou números)
    // Blackout: Ponto (.) OU Alt+B OU F1
    if (e.key === '.' || e.key === 'F1' || (e.altKey && (e.key === 'b' || e.key === 'B'))) {
      e.preventDefault();
      toggleBlackout();
      return;
    }

    // Limpar Texto: Vírgula (,) OU Alt+C OU F2
    if (e.key === ',' || e.key === 'F2' || (e.altKey && (e.key === 'c' || e.key === 'C'))) {
      e.preventDefault();
      toggleClearText();
      return;
    }

    // Papel de Parede (Tela de Repouso): Barra (/) OU Alt+W OU F3
    if (e.key === '/' || e.key === 'F3' || (e.altKey && (e.key === 'w' || e.key === 'W'))) {
      e.preventDefault();
      setDisplayMode('cleartext');
      return;
    }

    // Letras para salto de seção
    if (!e.metaKey && !e.ctrlKey && !e.altKey && /^[a-zA-Z]$/.test(e.key)) {
      if (jumpToSection(e.key)) {
        e.preventDefault();
        return;
      }
    }

    if (e.key === 'ArrowRight' || e.key === 'PageDown' || e.key === ' ') {
      e.preventDefault();
      nextSlide();
    } else if (e.key === 'ArrowLeft' || e.key === 'PageUp') {
      e.preventDefault();
      prevSlide();
    }
  }

  // Ações de Setlist
  async function handleSwitchSetlist(setlistId: string) {
    isSetlistSelectorOpen = false;
    await switchActiveSetlist(setlistId);
    await loadSetlists();
  }

  function handleOpenCreateSetlist() {
    isSetlistSelectorOpen = false;
    editingSetlist = null;
    isSetlistModalOpen = true;
  }

  async function handleOpenEditSetlist() {
    isSetlistSelectorOpen = false;
    const currentId = $presentationState.active_setlist_id;
    if (!currentId) return;
    try {
      const res = await fetch(`/api/setlists/${currentId}`);
      if (res.ok) {
        editingSetlist = await res.json();
        isSetlistModalOpen = true;
      }
    } catch (err) {
      console.error('Erro ao buscar setlist para edição:', err);
    }
  }

  async function handleDeleteSetlist() {
    isSetlistSelectorOpen = false;
    const currentId = $presentationState.active_setlist_id;
    if (!currentId) return;
    if (setlistsList.length <= 1) {
      alert('Não é possível excluir a única setlist existente.');
      return;
    }
    if (!confirm('Deseja realmente excluir esta setlist?')) return;

    try {
      await fetch(`/api/setlists/${currentId}`, { method: 'DELETE' });
      await loadSetlists();
      if (setlistsList.length > 0) {
        await switchActiveSetlist(setlistsList[0].id);
      }
    } catch (err) {
      console.error('Erro ao excluir setlist:', err);
    }
  }

  async function handleClearSetlist() {
    isSetlistSelectorOpen = false;
    if ($mediaItemsList.length === 0) return;
    if (!confirm('Deseja realmente remover todos os itens da setlist ativa?')) return;

    try {
      await clearSetlistItems();
      selectedMediaId = '';
    } catch (err) {
      console.error('Erro ao limpar itens da setlist:', err);
    }
  }

  // Menu de Contexto nos Itens da Setlist
  function handleItemContextMenu(e: MouseEvent, item: MediaItem) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'pin_top',
        label: 'Fixar no topo',
        icon: ArrowUpToLine,
        onclick: () => handlePinToTop(item),
      },
      {
        id: 'edit',
        label: item.kind === 'song' ? 'Editar Música' : 'Editar',
        icon: Pencil,
        onclick: () => handleEditItem(item),
      },
      {
        id: 'rename',
        label: 'Renomear',
        icon: Edit3,
        onclick: () => handleOpenRename(item),
      },
      {
        id: 'div_1',
        divider: true,
      },
      {
        id: 'remove',
        label: 'Remover da Setlist',
        icon: Trash2,
        destructive: true,
        onclick: () => handleRemoveItem(item),
      },
    ];

    contextMenuOpen = true;
  }

  // Menu de Contexto em Área Vazia da Setlist
  function handleEmptyContextMenu(e: MouseEvent) {
    e.preventDefault();
    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'add_song',
        label: 'Adicionar Música',
        icon: Music,
        onclick: () => (isCommandPaletteOpen = true),
      },
      {
        id: 'add_section',
        label: 'Adicionar Seção',
        icon: Bookmark,
        onclick: () => {
          quickItemMode = 'header';
          editingQuickItem = null;
          isQuickItemModalOpen = true;
        },
      },
      {
        id: 'add_text',
        label: 'Adicionar Texto Livre / Apresentação',
        icon: FileText,
        onclick: () => {
          songEditorMode = 'text';
          editingItem = null;
          editingSong = null;
          isSongEditorOpen = true;
        },
      },
      {
        id: 'add_image',
        label: 'Adicionar Imagens...',
        icon: ImageIcon,
        onclick: () => {
          imageSelectorMode = 'single';
          isImageSelectorOpen = true;
        },
      },
      {
        id: 'add_folder',
        label: 'Adicionar Pasta de Imagens...',
        icon: Folder,
        onclick: () => {
          imageSelectorMode = 'folder';
          isImageSelectorOpen = true;
        },
      },
      {
        id: 'div_empty_1',
        divider: true,
      },
      {
        id: 'add_scripture',
        label: 'Versículo Bíblico (Em breve)',
        icon: BookOpen,
        disabled: true,
      },
    ];

    contextMenuOpen = true;
  }

  function handlePinToTop(item: MediaItem) {
    const list = [...$mediaItemsList];
    const idx = list.findIndex((m) => m.id === item.id);
    if (idx > 0) {
      const [removed] = list.splice(idx, 1);
      list.unshift(removed);
      reorderSetlistItems(list);
    }
  }

  async function handleEditItem(item: MediaItem) {
    if (item.kind === 'song') {
      songEditorMode = 'song';
      try {
        const res = await fetch(`/api/songs/${item.id}`);
        if (res.ok) {
          editingSong = await res.json();
          editingItem = null;
          isSongEditorOpen = true;
          return;
        }
      } catch {}
      editingSong = {
        id: item.id,
        title: item.title,
        artist: item.artist_or_ref || '',
        sections: [],
        lyrics_raw: item.slides.map((s) => s.content).join('\n\n'),
      };
      editingItem = null;
      isSongEditorOpen = true;
    } else if (item.kind === 'header') {
      quickItemMode = 'header';
      editingQuickItem = item;
      isQuickItemModalOpen = true;
    } else {
      // Apresentação de Texto Livre / Aviso
      songEditorMode = 'text';
      editingItem = item;
      editingSong = null;
      isSongEditorOpen = true;
    }
  }

  async function handleDropReorder(fromIndex: number | null, toIndex: number) {
    if (fromIndex === null || fromIndex === toIndex) return;
    const items = [...$mediaItemsList];
    const [moved] = items.splice(fromIndex, 1);
    items.splice(toIndex, 0, moved);
    draggedItemIndex = null;
    await reorderSetlistItems(items);
  }

  async function handleAddSongToSetlist(songId: string) {
    try {
      await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ song_id: songId }),
      });
      selectedMediaId = songId;
    } catch (err) {
      console.error('Erro ao adicionar música:', err);
    }
  }

  async function handlePresentSong(songId: string) {
    await handleAddSongToSetlist(songId);
    selectSlide(songId, 0);
  }

  function handlePresentImage(image: ImageMediaItem) {
    const item: MediaItem = {
      id: `img-${Date.now()}`,
      title: image.name,
      artist_or_ref: image.folder_name,
      kind: 'image',
      slides: [
        {
          id: 'slide-1',
          label: 'Imagem',
          content: image.url,
        },
      ],
    };
    selectedMediaId = item.id;
    quickPresentMedia(item);
  }

  function handleOpenPreviewImage(image: ImageMediaItem) {
    previewImage = image;
    isPreviewImageOpen = true;
  }

  async function handleAddImageToSetlist(image: ImageMediaItem) {
    const item: MediaItem = {
      id: `img-${Date.now()}`,
      title: image.name,
      artist_or_ref: image.folder_name,
      kind: 'image',
      slides: [
        {
          id: 'slide-1',
          label: 'Imagem',
          content: image.url,
        },
      ],
    };
    try {
      await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item }),
      });
      selectedMediaId = item.id;
    } catch (err) {
      console.error('Erro ao adicionar imagem à setlist:', err);
    }
  }

  async function handleAddImageItems(items: MediaItem[]) {
    for (const item of items) {
      try {
        await fetch('/api/setlist/add', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ item }),
        });
        selectedMediaId = item.id;
      } catch (err) {
        console.error('Erro ao adicionar imagem à setlist:', err);
      }
    }
  }

  function handlePresentScripture(item: MediaItem, slideIndex: number) {
    selectedMediaId = item.id;
    quickPresentMedia(item, slideIndex);
  }

  async function handleAddScriptureToSetlist(item: MediaItem) {
    try {
      await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item }),
      });
      selectedMediaId = item.id;
    } catch (err) {
      console.error('Erro ao adicionar texto bíblico à setlist:', err);
    }
  }

  function handleOpenBibleTab() {
    targetScriptureRef = null;
    bottomPanelRequestedTab = 'bible';
    setTimeout(() => {
      bottomPanelRequestedTab = null;
    }, 150);
  }

  function handleOpenBibleFromPalette(ref: ParsedReference) {
    targetScriptureRef = ref;
    bottomPanelRequestedTab = 'bible';
    setTimeout(() => {
      bottomPanelRequestedTab = null;
    }, 150);
  }

  async function handlePresentScriptureFromPalette(ref: ParsedReference) {
    try {
      const biblesRes = await fetch('/api/bibles');
      if (!biblesRes.ok) return;
      const biblesList: BibleVersionSummary[] = await biblesRes.json();
      if (biblesList.length === 0) return;

      let targetBible = biblesList.find((b) => b.is_default) || biblesList[0];
      if (ref.version) {
        const found = biblesList.find(
          (b) =>
            b.abbreviation.toLowerCase() === ref.version!.toLowerCase() ||
            b.name.toLowerCase().includes(ref.version!.toLowerCase())
        );
        if (found) {
          targetBible = found;
        }
      }

      const versesRes = await fetch(
        `/api/bibles/${targetBible.id}/books/${ref.book_id}/chapters/${ref.chapter}/verses`
      );
      if (!versesRes.ok) return;
      const versesList: BibleVerse[] = await versesRes.json();
      if (versesList.length === 0) return;

      const slides: Slide[] = versesList.map((v) => ({
        id: `v-${v.verse}`,
        label: `${ref.book_abbreviation || ref.book_name} ${ref.chapter}:${v.verse}`,
        content: v.text,
        align_h: 'center',
        align_v: 'center',
      }));

      const scriptureItem: MediaItem = {
        id: `scripture-${ref.book_id}-${ref.chapter}-${Date.now()}`,
        title: `${ref.book_name} ${ref.chapter}`,
        artist_or_ref: targetBible.abbreviation,
        kind: 'scripture',
        slides,
        align_h: 'center',
        align_v: 'center',
      };

      const targetVerse = ref.verse_start || 1;
      const targetIndex = versesList.findIndex((v) => v.verse === targetVerse);
      const slideIdx = targetIndex >= 0 ? targetIndex : 0;

      selectedMediaId = scriptureItem.id;
      quickPresentMedia(scriptureItem, slideIdx);
    } catch (err) {
      console.error('Erro ao projetar passagem bíblica:', err);
    }
  }

  function handleItemSaved(item: MediaItem) {
    selectedMediaId = item.id;
  }

  function handleOpenRename(item: MediaItem) {
    renamingItemId = item.id;
    renamingItemTitle = item.title;
    isRenameModalOpen = true;
  }

  async function handleSaveRename() {
    if (renamingItemId && renamingItemTitle.trim()) {
      await renameSetlistItem(renamingItemId, renamingItemTitle.trim());
    }
    isRenameModalOpen = false;
  }

  async function handleRemoveItem(item: MediaItem) {
    await removeSetlistItem(item.id);
    if (selectedMediaId === item.id) {
      selectedMediaId = $mediaItemsList.find((m) => m.id !== item.id)?.id || '';
    }
  }

  async function handleSaveQuickItem(item: MediaItem) {
    try {
      await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ item }),
      });
      selectedMediaId = item.id;
    } catch (err) {
      console.error('Erro ao adicionar item:', err);
    }
  }

  function handleOpenCreate() {
    editingSong = null;
    isSongEditorOpen = true;
  }

  function handleImportSong(song: Song) {
    editingSong = song;
    isSongEditorOpen = true;
  }

  function handleSongSaved(song: Song) {
    selectedMediaId = song.id;
  }

  function handleSetlistSaved(setlist: Setlist) {
    loadSetlists();
  }

  // Abertura de janelas de saída
  function openPublicWindow() {
    window.open('/public', 'OpenPresenterPublic', 'width=1280,height=720,menubar=no,toolbar=no,location=no');
  }

  function openStageWindow() {
    window.open('/stage', 'OpenPresenterStage', 'width=1280,height=720,menubar=no,toolbar=no,location=no');
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="h-screen w-screen flex flex-col bg-background text-foreground overflow-hidden font-sans">
  <!-- Top Bar / Header Operacional -->
  <header class="h-14 border-b border-border bg-card/60 px-4 flex items-center justify-between shrink-0">
    <div class="flex items-center gap-2.5">
      <!-- Logo Oficial com Tamanho Nítido e Sem Duplicação -->
      <a href="/" class="flex items-center gap-2 group cursor-pointer" title="OpenPresenter Minimal">
        <img
          src="/branding/open-presenter-app-icon.svg"
          alt="OpenPresenter"
          class="w-7 h-7 rounded-lg shadow-sm group-hover:scale-105 transition-transform shrink-0"
        />
        <div class="flex items-center text-lg leading-none font-sans select-none tracking-tight">
          <span class="font-bold text-foreground">Open</span><span class="font-normal text-muted-foreground ml-0.5">Presenter</span>
        </div>
      </a>
    </div>

    <!-- Busca Global e Command Palette (⌘K) -->
    <button
      type="button"
      onclick={() => (isCommandPaletteOpen = true)}
      class="flex items-center gap-2 h-8 px-3 rounded-lg bg-muted/40 hover:bg-muted/70 border border-border text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer w-44 sm:w-64 md:w-80 justify-between mx-2"
    >
      <span class="flex items-center gap-2 truncate">
        <Search size={14} class="text-muted-foreground shrink-0" />
        <span class="truncate">Buscar música ou comando...</span>
      </span>
      <kbd class="px-1.5 py-0.5 rounded bg-background border border-border text-[10px] font-mono text-muted-foreground shrink-0">
        ⌘K
      </kbd>
    </button>

    <!-- Status e Ações Rápidas -->
    <div class="flex items-center gap-2 sm:gap-2.5">
      <!-- Widget de Cronômetro Global do Operador -->
      <div
        class={cn(
          'flex items-center gap-1.5 px-2.5 py-1 rounded-lg border transition-colors',
          isTimerOvertime
            ? 'bg-rose-950/40 border-rose-600/70 text-rose-300 animate-pulse'
            : isTimerWarning
              ? 'bg-amber-950/40 border-amber-600/70 text-amber-300'
              : isTimerRunning
                ? 'bg-primary/10 border-primary/40 text-primary'
                : 'bg-muted/40 border-border text-muted-foreground'
        )}
        title={`Cronômetro: ${timer.label || 'Geral'} (${isTimerRunning ? 'Em execução' : 'Pausado'})`}
      >
        <Timer
          size={13}
          class={isTimerRunning
            ? isTimerOvertime
              ? 'text-rose-400'
              : isTimerWarning
                ? 'text-amber-400'
                : 'text-primary'
            : 'text-muted-foreground'}
        />
        <span class="font-mono text-xs font-semibold tabular-nums tracking-tight">
          {formattedHeaderTimer.text}
        </span>
        <div class="flex items-center gap-0.5 ml-1 pl-1 border-l border-border/60">
          {#if isTimerRunning}
            <button
              type="button"
              onclick={pauseTimer}
              class="p-0.5 hover:bg-background/80 rounded text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
              title="Pausar Cronômetro"
            >
              <Pause size={12} />
            </button>
          {:else}
            <button
              type="button"
              onclick={startTimer}
              class="p-0.5 hover:bg-background/80 rounded text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
              title="Iniciar Cronômetro"
            >
              <Play size={12} />
            </button>
          {/if}
          <button
            type="button"
            onclick={resetTimer}
            class="p-0.5 hover:bg-background/80 rounded text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
            title="Reiniciar Cronômetro"
          >
            <RotateCcw size={12} />
          </button>
        </div>
      </div>

      <!-- Status WebSocket -->
      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-muted/40 border border-border text-xs">
        {#if $isWsConnected}
          <Wifi size={13} class="text-emerald-400" />
          <span class="text-emerald-400 font-medium">Online</span>
        {:else}
          <WifiOff size={13} class="text-destructive" />
          <span class="text-destructive font-medium">Reconectando</span>
        {/if}
      </div>

      <!-- Botão Configurações -->
      <Button
        variant="outline"
        size="sm"
        onclick={() => (isSettingsOpen = true)}
        class="gap-1.5 text-xs"
        title="Configurações (Aparência, Pastas, Atalhos, Atualizações)"
      >
        <Settings size={14} class="text-muted-foreground" />
        {#if $previewLabelsStore}
          <span class="hidden md:inline">Configurações</span>
        {/if}
      </Button>

      <!-- Badge / Botão de Atualização Disponível -->
      {#if updateInfo?.update_available}
        <button
          type="button"
          onclick={() => {
            settingsInitialTab = 'about';
            isSettingsOpen = true;
          }}
          class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-950/70 hover:bg-emerald-900 border border-emerald-500/50 text-emerald-300 text-xs font-semibold animate-pulse transition-colors cursor-pointer"
          title="Nova versão v{updateInfo.latest_version} disponível! Clique para ver e atualizar."
        >
          <Sparkles size={13} class="text-emerald-400" />
          {#if $previewLabelsStore}
            <span class="hidden sm:inline">Nova Versão</span>
          {/if}
          v{updateInfo.latest_version}
        </button>
      {/if}

      <!-- Botões de Abertura das Saídas -->
      <Button
        variant="outline"
        size="sm"
        onclick={openPublicWindow}
        class="gap-1.5 text-xs"
        title="Iniciar Janela Pública (Projetor)"
      >
        <MonitorPlay size={14} class="text-primary" />
        {#if $previewLabelsStore}
          <span class="hidden md:inline">Iniciar</span> Public
        {/if}
        <ExternalLink size={12} class="opacity-60" />
      </Button>

      <Button
        variant="outline"
        size="sm"
        onclick={openStageWindow}
        class="gap-1.5 text-xs"
        title="Iniciar Janela do Stage (Palco)"
      >
        <Tv size={14} class="text-primary" />
        {#if $previewLabelsStore}
          <span class="hidden md:inline">Iniciar</span> Stage
        {/if}
        <ExternalLink size={12} class="opacity-60" />
      </Button>

      <!-- Botão Compartilhar Saídas (QR Code) -->
      <Button
        variant="outline"
        size="sm"
        onclick={() => (isShareOutputsOpen = true)}
        class="gap-1.5 text-xs"
        title="Compartilhar Saídas com Celulares e Tablets via QR Code"
      >
        <QrCode size={14} class="text-primary" />
        {#if $previewLabelsStore}
          <span class="hidden lg:inline">QR Code</span>
        {/if}
      </Button>
    </div>
  </header>

  <!-- Workspace Principal de 3 Colunas -->
  <div class="flex-1 flex overflow-hidden">
    <!-- COLUNA 1: SIDEBAR DE MÍDIAS / SETLIST (Esquerda) -->
    {#if isSetlistSidebarOpen}
      <aside class="w-72 lg:w-80 border-r border-neutral-800/80 bg-neutral-900/30 flex flex-col shrink-0">
        <!-- Cabeçalho da Setlist com Seletor e Ações -->
        <div class="p-3 border-b border-neutral-800/80 space-y-2.5">
          <div class="flex items-center justify-between">
            <!-- Dropdown Seletor de Setlist -->
            <div class="relative min-w-0 flex-1 mr-2">
              <button
                type="button"
                onclick={() => (isSetlistSelectorOpen = !isSetlistSelectorOpen)}
                class="w-full flex items-center justify-between gap-1.5 px-2 py-1 rounded-md bg-neutral-900 hover:bg-neutral-800 border border-neutral-800 text-left transition-colors cursor-pointer group"
                title="Alternar ou gerenciar Setlists"
              >
                <div class="flex items-center gap-1.5 min-w-0">
                  <Layers size={14} class="text-emerald-400 shrink-0" />
                  <span class="text-xs font-bold text-neutral-100 truncate">
                    {currentSetlistTitle}
                  </span>
                </div>
                <ChevronDown size={13} class="text-neutral-500 group-hover:text-neutral-300 shrink-0" />
              </button>

              <!-- Dropdown Menu de Setlists -->
              {#if isSetlistSelectorOpen}
                <div
                  class="absolute left-0 top-full mt-1 w-64 rounded-xl border border-neutral-800 bg-neutral-900/95 backdrop-blur-md p-1.5 shadow-2xl z-40 text-xs text-neutral-200 select-none animate-in fade-in zoom-in-95 duration-100"
                >
                  <div class="px-2 py-1 text-[10px] font-semibold text-neutral-500 uppercase tracking-wider">
                    Setlists Cadastradas
                  </div>

                  <div class="max-h-48 overflow-y-auto space-y-0.5 py-1">
                    {#each setlistsList as s (s.id)}
                      <button
                        type="button"
                        onclick={() => handleSwitchSetlist(s.id)}
                        class={cn(
                          'w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors cursor-pointer',
                          s.is_active || s.id === $presentationState.active_setlist_id
                            ? 'bg-neutral-800 text-emerald-400 font-semibold'
                            : 'hover:bg-neutral-850 text-neutral-300 hover:text-white'
                        )}
                      >
                        <div class="min-w-0">
                          <div class="truncate text-xs">{s.title}</div>
                          {#if s.description}
                            <div class="text-[10px] text-neutral-500 truncate">{s.description}</div>
                          {/if}
                        </div>

                        {#if s.is_active || s.id === $presentationState.active_setlist_id}
                          <Check size={13} class="text-emerald-400 shrink-0 ml-1.5" />
                        {/if}
                      </button>
                    {/each}
                  </div>

                  <div class="h-[1px] bg-neutral-800 my-1"></div>

                  <button
                    type="button"
                    onclick={handleOpenCreateSetlist}
                    class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-left text-neutral-200 hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer"
                  >
                    <Plus size={13} class="text-emerald-400" />
                    <span>+ Nova Setlist</span>
                  </button>

                  <button
                    type="button"
                    onclick={handleOpenEditSetlist}
                    class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-left text-neutral-200 hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer"
                  >
                    <Settings size={13} class="text-neutral-400" />
                    <span>Editar Setlist Atual...</span>
                  </button>

                  <button
                    type="button"
                    disabled={$mediaItemsList.length === 0}
                    onclick={handleClearSetlist}
                    class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-left text-neutral-200 hover:bg-neutral-800 hover:text-amber-300 transition-colors cursor-pointer disabled:opacity-40 disabled:pointer-events-none"
                  >
                    <Eraser size={13} class="text-amber-400" />
                    <span>Limpar Itens da Setlist</span>
                  </button>

                  {#if setlistsList.length > 1}
                    <button
                      type="button"
                      onclick={handleDeleteSetlist}
                      class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-left text-red-400 hover:bg-red-500/20 hover:text-red-300 transition-colors cursor-pointer"
                    >
                      <Trash2 size={13} />
                      <span>Excluir Setlist</span>
                    </button>
                  {/if}
                </div>
              {/if}
            </div>

            <!-- Botão + com Menu Suspenso de Adição e Botão Fechar Sidebar -->
            <div class="relative shrink-0 flex items-center gap-1.5">
              <Badge variant="secondary" class="text-[11px] py-0 px-1.5">
                {$mediaItemsList.length} itens
              </Badge>

              <!-- Botão Rápido de Limpar Setlist -->
              {#if $mediaItemsList.length > 0}
                <button
                  type="button"
                  title="Limpar todos os itens da Setlist ativa"
                  onclick={handleClearSetlist}
                  class="w-7 h-7 rounded-md hover:bg-amber-500/20 text-neutral-400 hover:text-amber-300 border border-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
                >
                  <Eraser size={13} />
                </button>
              {/if}

              <button
                type="button"
                title="Adicionar à Setlist"
                onclick={() => (isAddMenuOpen = !isAddMenuOpen)}
                class="w-7 h-7 rounded-md bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 border border-emerald-500/30 flex items-center justify-center transition-colors cursor-pointer"
              >
                <Plus size={14} />
              </button>

              <button
                type="button"
                title="Recolher Setlist"
                onclick={() => (isSetlistSidebarOpen = false)}
                class="w-7 h-7 rounded-md hover:bg-neutral-800 text-neutral-400 hover:text-white border border-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
              >
                <PanelLeftClose size={14} />
              </button>

              <!-- Dropdown Menu do Botão + -->
              {#if isAddMenuOpen}
                <div
                  class="absolute right-0 top-full mt-1.5 w-48 rounded-xl border border-neutral-800 bg-neutral-900/95 backdrop-blur-md p-1.5 shadow-2xl z-40 text-xs text-neutral-200 select-none animate-in fade-in zoom-in-95 duration-100"
                >
                  <button
                    type="button"
                    onclick={() => {
                      isAddMenuOpen = false;
                      isCommandPaletteOpen = true;
                    }}
                    class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-left hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer"
                  >
                    <Music size={14} class="text-emerald-400" />
                    <span>Música (⌘K)</span>
                  </button>

                  <button
                    type="button"
                    onclick={() => {
                      isAddMenuOpen = false;
                      quickItemMode = 'header';
                      editingQuickItem = null;
                      isQuickItemModalOpen = true;
                    }}
                    class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-left hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer"
                  >
                    <Bookmark size={14} class="text-cyan-400" />
                    <span>Seção / Divisor</span>
                  </button>

                  <button
                    type="button"
                    onclick={() => {
                      isAddMenuOpen = false;
                      songEditorMode = 'text';
                      editingItem = null;
                      editingSong = null;
                      isSongEditorOpen = true;
                    }}
                    class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-left hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer"
                  >
                    <FileText size={14} class="text-amber-400" />
                    <span>Texto Livre / Apresentação</span>
                  </button>

                  <button
                    type="button"
                    onclick={() => {
                      isAddMenuOpen = false;
                      imageSelectorMode = 'single';
                      isImageSelectorOpen = true;
                    }}
                    class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-left hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer"
                  >
                    <ImageIcon size={14} class="text-primary" />
                    <span>Imagem...</span>
                  </button>

                  <button
                    type="button"
                    onclick={() => {
                      isAddMenuOpen = false;
                      imageSelectorMode = 'folder';
                      isImageSelectorOpen = true;
                    }}
                    class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-left hover:bg-neutral-800 hover:text-white transition-colors cursor-pointer"
                  >
                    <Folder size={14} class="text-primary" />
                    <span>Pasta de Imagens...</span>
                  </button>

                  <div class="h-[1px] bg-neutral-800 my-1"></div>

                  <div
                    class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left text-neutral-500 opacity-50 cursor-not-allowed"
                  >
                    <div class="flex items-center gap-2.5">
                      <BookOpen size={14} />
                      <span>Versículo Bíblico</span>
                    </div>
                    <span class="text-[9px] uppercase tracking-wider font-mono">Em breve</span>
                  </div>
                </div>
              {/if}
            </div>
          </div>

          <!-- Barra de Busca com Input Group e Botão de Filtros -->
          <div class="relative flex items-center">
            <Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-neutral-500 pointer-events-none" />
            <input
              type="text"
              bind:value={searchQuery}
              placeholder="Filtrar setlist (ou ⌘K p/ buscar)..."
              class="w-full h-8 pl-8 pr-8 bg-neutral-900 border border-neutral-800 rounded-md text-xs text-neutral-200 placeholder-neutral-500 focus:outline-none focus:border-emerald-500/60 focus:ring-1 focus:ring-emerald-500/40 transition-colors"
            />
            <!-- Botão Toggle de Filtro no Input Group -->
            <button
              type="button"
              onclick={() => (showCategoryFilters = !showCategoryFilters)}
              title={showCategoryFilters ? 'Ocultar filtros de categoria' : 'Filtrar por categoria (Louvores, Bíblia, Avisos)'}
              class={cn(
                'absolute right-1 top-1/2 -translate-y-1/2 w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer',
                showCategoryFilters || filterKind !== 'all'
                  ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/40'
                  : 'text-neutral-500 hover:text-neutral-300 hover:bg-neutral-800'
              )}
            >
              <Filter size={12} />
            </button>
          </div>

          <!-- Filtros por Categoria (Exibidos apenas quando o toggle estiver ativo) -->
          {#if showCategoryFilters}
            <div class="flex items-center gap-1 overflow-x-auto pb-0.5 animate-in fade-in slide-in-from-top-1 duration-150">
              {#each [
                { id: 'all', label: 'Todos' },
                { id: 'song', label: 'Louvores' },
                { id: 'scripture', label: 'Bíblia' },
                { id: 'announcement', label: 'Avisos' },
              ] as tab}
                <button
                  type="button"
                  onclick={() => (filterKind = tab.id as any)}
                  class={cn(
                    'px-2 py-1 rounded text-[11px] font-medium transition-colors shrink-0 cursor-pointer',
                    filterKind === tab.id
                      ? 'bg-neutral-800 text-emerald-400 font-semibold'
                      : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-850'
                  )}
                >
                  {tab.label}
                </button>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Lista de Itens de Mídia com Reorganização Drag and Drop e Context Menu -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="flex-1 overflow-y-auto p-2.5 space-y-1.5"
          oncontextmenu={handleEmptyContextMenu}
        >
          {#each filteredMedia as item, idx (item.id)}
            <MediaItemRow
              {item}
              index={idx}
              selected={selectedMediaId === item.id}
              isLive={$presentationState.active_media_id === item.id}
              onclick={() => (selectedMediaId = item.id)}
              oncontextmenu={(e) => handleItemContextMenu(e, item)}
              ondragstartitem={(e, i) => (draggedItemIndex = i)}
              ondropitem={(e, targetIdx) => handleDropReorder(draggedItemIndex, targetIdx)}
            />
          {:else}
            <div class="p-6 text-center text-xs text-neutral-500">
              Nenhuma mídia encontrada na Setlist. Clique com o botão direito para adicionar.
            </div>
          {/each}
        </div>
      </aside>
    {:else}
      <!-- Rail Colapsado da Setlist -->
      <aside class="w-10 border-r border-neutral-800/80 bg-neutral-900/40 flex flex-col items-center py-3 gap-4 shrink-0 select-none">
        <button
          type="button"
          onclick={() => (isSetlistSidebarOpen = true)}
          class="p-1.5 rounded-md hover:bg-neutral-800 text-neutral-400 hover:text-white transition-colors cursor-pointer"
          title="Expandir Setlist"
        >
          <PanelLeftOpen size={16} />
        </button>
        <span
          class="text-[10px] uppercase font-bold tracking-widest text-neutral-400 [writing-mode:vertical-lr] rotate-180 cursor-pointer hover:text-white transition-colors py-2"
          onclick={() => (isSetlistSidebarOpen = true)}
          role="button"
          tabindex="0"
          onkeydown={(e) => { if (e.key === 'Enter') isSetlistSidebarOpen = true; }}
        >
          Setlist ({$mediaItemsList.length})
        </span>
      </aside>
    {/if}

    <!-- COLUNA 2: PAINEL CENTRAL DE SLIDES / PREVIEW -->
    <main class="flex-1 flex flex-col min-w-0 bg-neutral-950/60 overflow-hidden">
      {#if selectedMedia}
        <!-- Cabeçalho da Mídia Selecionada com Ações de Apresentação -->
        <div class="h-14 px-5 border-b border-neutral-800/80 bg-neutral-900/40 flex items-center justify-between shrink-0">
          <div class="min-w-0">
            <div class="flex items-center gap-2">
              <h2 class="text-base font-bold text-white truncate leading-tight">
                {selectedMedia.title}
              </h2>
              <Badge variant="default" class="text-[10px] py-0 px-1.5">
                {selectedMedia.kind === 'song' ? 'Louvor' : selectedMedia.kind === 'scripture' ? 'Bíblia' : selectedMedia.kind === 'header' ? 'Seção' : 'Aviso'}
              </Badge>
              {#if isSelectedMediaLive}
                <Badge variant="live" class="text-[10px] py-0 px-1.5 animate-pulse">
                  NO AR
                </Badge>
              {/if}
            </div>
            {#if selectedMedia.artist_or_ref}
              <p class="text-xs text-neutral-400 truncate mt-0.5">
                {selectedMedia.artist_or_ref}
              </p>
            {/if}
          </div>

          <!-- Ações e Controles de Navegação -->
          <div class="flex items-center gap-1.5 shrink-0">
            {#if isSelectedMediaLive}
              <Button
                variant="destructive"
                size="sm"
                onclick={stopPresentation}
                title="Parar Projeção (Esc)"
                class="gap-1.5 text-xs font-semibold"
              >
                <Square size={13} class="fill-current" />
                {#if $previewLabelsStore}
                  <span>Parar (Esc)</span>
                {/if}
              </Button>
            {/if}

            <Button
              variant={isSelectedMediaLive ? 'live' : 'default'}
              size="sm"
              onclick={() => presentSlide(focusedSlideIndex)}
              title="Apresentar Slide Focado (Enter)"
              class="gap-1.5 text-xs font-semibold shadow-sm"
            >
              <Play size={13} class="fill-current" />
              {#if $previewLabelsStore}
                <span>Apresentar (Enter)</span>
              {/if}
            </Button>

            <div class="h-4 w-[1px] bg-neutral-700/60 mx-1"></div>

            <Button
              variant="secondary"
              size="sm"
              onclick={prevSlide}
              title="Slide Anterior (←)"
              class="gap-1 text-xs"
            >
              <ChevronLeft size={14} />
              {#if $previewLabelsStore}
                <span>Anterior</span>
              {/if}
            </Button>
            <Button
              variant="secondary"
              size="sm"
              onclick={nextSlide}
              title="Próximo Slide (→ ou Espaço)"
              class="gap-1 text-xs"
            >
              {#if $previewLabelsStore}
                <span>Próximo</span>
              {/if}
              <ChevronRight size={14} />
            </Button>
          </div>
        </div>

        <!-- Grade de Slides (Aspect Ratio 16:9) -->
        <div class="flex-1 overflow-y-auto p-5">
          <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
            {#each selectedMedia.slides as slide, idx (slide.id)}
              {@const isLive =
                $presentationState.active_media_id === selectedMedia.id &&
                $presentationState.active_slide_index === idx &&
                $presentationState.display_mode === 'live'}
              {@const isFocused = focusedSlideIndex === idx}
              {@const sectionStyle = getSectionStyle(slide.label, $sectionsStore)}

              <div
                class={cn(
                  'group flex flex-col rounded-lg border transition-all duration-150 overflow-hidden cursor-pointer select-none relative',
                  isLive
                    ? 'border-emerald-500 bg-neutral-900 shadow-lg ring-2 ring-emerald-500/50'
                    : isFocused
                      ? 'border-sky-500 bg-neutral-900 shadow-md ring-2 ring-sky-500/40'
                      : 'border-neutral-800 bg-neutral-900/50 hover:border-neutral-700 hover:bg-neutral-850'
                )}
                onclick={() => selectSlideForFocus(idx)}
                ondblclick={() => presentSlide(idx)}
                role="button"
                tabindex="0"
                onkeydown={(e) => {
                  if (e.key === 'Enter') presentSlide(idx);
                }}
              >
                <!-- Cabeçalho do Cartão do Slide com Cores Customizadas de Seção -->
                <div
                  class={cn(
                    'px-3 py-1.5 border-b flex items-center justify-between text-xs',
                    sectionStyle.bg,
                    sectionStyle.border
                  )}
                >
                  <span class={cn('font-semibold flex items-center gap-1.5', sectionStyle.text)}>
                    <span
                      class="w-4 h-4 rounded-full bg-black/40 text-white flex items-center justify-center text-[10px] font-mono"
                    >
                      {idx + 1}
                    </span>
                    {slide.label}
                  </span>

                  <div class="flex items-center gap-1">
                    {#if isLive}
                      <Badge variant="live" class="text-[10px] py-0 px-1.5 animate-pulse">
                        AO VIVO
                      </Badge>
                    {:else if isFocused}
                      <Badge variant="secondary" class="text-[10px] py-0 px-1.5 border-sky-500/40 text-sky-400 bg-sky-950/40">
                        FOCADO
                      </Badge>
                    {/if}
                  </div>
                </div>

                <!-- Conteúdo Visual do Slide (Proporção 16:9) -->
                {#if selectedMedia.kind === 'image'}
                  <div class="aspect-video w-full bg-black/80 flex items-center justify-center relative overflow-hidden">
                    <img src={slide.content} alt={selectedMedia.title} class="w-full h-full object-contain" />
                    <span class="absolute bottom-1.5 right-2 text-[10px] text-neutral-400 font-mono bg-black/60 px-1 rounded">
                      Imagem 16:9
                    </span>
                  </div>
                {:else}
                  <div class="aspect-video w-full p-4 flex flex-col bg-black/60 relative overflow-hidden">
                    <div
                      class={cn(
                        'w-full h-full flex flex-col leading-tight',
                        (slide.align_h || selectedMedia.align_h) === 'left'
                          ? 'text-left items-start'
                          : (slide.align_h || selectedMedia.align_h) === 'right'
                            ? 'text-right items-end'
                            : (slide.align_h || selectedMedia.align_h) === 'justify'
                              ? 'text-justify items-center'
                              : 'text-center items-center',
                        (slide.align_v || selectedMedia.align_v) === 'top'
                          ? 'justify-start'
                          : (slide.align_v || selectedMedia.align_v) === 'bottom'
                            ? 'justify-end'
                            : 'justify-center'
                      )}
                    >
                      <div
                        class={cn(
                          'text-xs sm:text-sm font-semibold select-none leading-relaxed',
                          isLive
                            ? 'text-emerald-200'
                            : isFocused
                              ? 'text-white'
                              : 'text-neutral-200 group-hover:text-white'
                        )}
                      >
                        {@html renderFormattedText(slide.content)}
                      </div>
                    </div>

                    <span class="absolute bottom-1.5 right-2 text-[10px] text-neutral-600 font-mono">
                      16:9
                    </span>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {:else}
        <div class="flex-1 flex items-center justify-center text-neutral-500 text-sm">
          Selecione uma mídia na setlist para visualizar os slides.
        </div>
      {/if}
    </main>

    <!-- COLUNA 3: PAINEL DE PREVIEWS (PUBLIC E STAGE - 16:9) -->
    {#if isPreviewSidebarOpen}
      <aside class="w-80 lg:w-96 border-l border-neutral-800/80 bg-neutral-900/30 flex flex-col shrink-0 overflow-y-auto">
        <!-- Botões de Controle Global da Projeção e Botão Fechar Sidebar -->
        <div class="p-3 border-b border-neutral-800/80 space-y-2 bg-neutral-900/60 shrink-0">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold text-neutral-400 uppercase tracking-wider block">
              Controle de Saídas
            </span>
            <button
              type="button"
              onclick={() => (isPreviewSidebarOpen = false)}
              class="w-6 h-6 rounded hover:bg-neutral-800 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
              title="Recolher painel de saídas"
            >
              <PanelRightClose size={14} />
            </button>
          </div>

          <div class="grid grid-cols-5 gap-1">
            <!-- Botão Ao Vivo -->
            <Button
              variant={$presentationState.display_mode === 'live' && $presentationState.active_media_id ? 'live' : 'secondary'}
              size="sm"
              onclick={() => setDisplayMode('live')}
              class="text-xs px-1 flex items-center justify-center gap-1"
              title="Colocar projeção ao vivo"
            >
              <Radio size={$previewLabelsStore ? 12 : 14} />
              {#if $previewLabelsStore}
                <span class="truncate text-[10px]">Ao Vivo</span>
              {/if}
            </Button>

            <!-- Botão Parar Apresentação -->
            <Button
              variant="secondary"
              size="sm"
              onclick={stopPresentation}
              class="text-xs px-1 hover:bg-red-950/40 hover:text-red-400 flex items-center justify-center gap-1"
              title="Parar apresentação (Esc)"
            >
              <Square size={$previewLabelsStore ? 12 : 14} class="text-red-400" />
              {#if $previewLabelsStore}
                <span class="truncate text-[10px]">Parar</span>
              {/if}
            </Button>

            <!-- Botão Blackout -->
            <Button
              variant={$presentationState.display_mode === 'blackout' ? 'destructive' : 'secondary'}
              size="sm"
              onclick={toggleBlackout}
              class="text-xs px-1 flex items-center justify-center gap-1"
              title="Blackout total (. ou Alt+B ou F1)"
            >
              <EyeOff size={$previewLabelsStore ? 12 : 14} />
              {#if $previewLabelsStore}
                <span class="truncate text-[10px]">Black</span>
              {/if}
            </Button>

            <!-- Botão Limpar Texto -->
            <Button
              variant={$presentationState.display_mode === 'cleartext' ? 'default' : 'secondary'}
              size="sm"
              onclick={toggleClearText}
              class="text-xs px-1 flex items-center justify-center gap-1"
              title="Ocultar texto (, ou Alt+C ou F2)"
            >
              <Slash size={$previewLabelsStore ? 12 : 14} />
              {#if $previewLabelsStore}
                <span class="truncate text-[10px]">Limpar</span>
              {/if}
            </Button>

            <!-- Botão Papel de Parede / Fundo -->
            <Button
              variant={$presentationState.display_mode === 'cleartext' && $presentationState.wallpaper_path ? 'default' : 'secondary'}
              size="sm"
              onclick={() => setDisplayMode('cleartext')}
              class="text-xs px-1 flex items-center justify-center gap-1"
              title="Tela de repouso / Papel de parede (/ ou Alt+W ou F3)"
            >
              <ImageIcon size={$previewLabelsStore ? 12 : 14} />
              {#if $previewLabelsStore}
                <span class="truncate text-[10px]">Fundo</span>
              {/if}
            </Button>
          </div>
        </div>

        <!-- PREVIEW PUBLIC (16:9) -->
        <div class="p-3 border-b border-neutral-800/80 space-y-2">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-1.5">
              <MonitorPlay size={14} class="text-emerald-400" />
              <span class="text-xs font-bold text-neutral-200">Public (Projetor)</span>
            </div>

            <div class="flex items-center gap-1">
              <button
                type="button"
                onclick={() => reloadOutputs('public')}
                class="w-5 h-5 rounded flex items-center justify-center text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
                title="Recarregar tela pública (projetor) remotamente"
              >
                <RefreshCw size={11} />
              </button>
              <Badge variant={$presentationState.public_connected ? 'success' : 'secondary'} class="text-[10px] py-0 px-1.5">
                {$presentationState.public_connected ? 'Conectado' : 'Aguardando'}
              </Badge>
            </div>
          </div>

          <!-- Preview Fiel do Public (16:9) Espelhando a Projeção Real -->
          <PublicPreviewBox
            publicSettings={$publicSettingsStore}
            churchBrand={$churchBrandStore}
            activeSlide={liveSlide}
            activeMedia={liveMedia}
            displayMode={$presentationState.display_mode}
            wallpaperPath={$presentationState.wallpaper_path}
            wallpaperMode={$presentationState.wallpaper_mode}
            resolvedWallpaper={resolvedWallpaper}
          />
        </div>

        <!-- PREVIEW STAGE (16:9) -->
        <div class="p-3 space-y-2">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-1.5">
              <Tv size={14} class="text-emerald-400" />
              <span class="text-xs font-bold text-neutral-200">Stage (Retorno Palco)</span>
            </div>

            <div class="flex items-center gap-1">
              <button
                type="button"
                onclick={() => reloadOutputs('stage')}
                class="w-5 h-5 rounded flex items-center justify-center text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
                title="Recarregar tela de retorno de palco remotamente"
              >
                <RefreshCw size={11} />
              </button>
              <Badge variant={$presentationState.stage_connected ? 'success' : 'secondary'} class="text-[10px] py-0 px-1.5">
                {$presentationState.stage_connected ? 'Conectado' : 'Aguardando'}
              </Badge>
            </div>
          </div>

          <!-- Preview Fiel do Stage com Preset, Cores e Proporções em Tempo Real -->
          <StagePreviewBox
            stageSettings={$stageSettingsStore}
            activeSlide={liveSlide}
            nextSlide={nextLiveSlide}
            activeMedia={liveMedia}
            timer={$presentationState.timer}
            stageClock={stageClock}
          />
        </div>

        <!-- Painel de Controle de Timer & Mensagens de Palco -->
        <TimerControlPanel />
      </aside>
    {:else}
      <!-- Rail Colapsado de Previews -->
      <aside class="w-10 border-l border-neutral-800/80 bg-neutral-900/40 flex flex-col items-center py-3 gap-4 shrink-0 select-none">
        <button
          type="button"
          onclick={() => (isPreviewSidebarOpen = true)}
          class="p-1.5 rounded-md hover:bg-neutral-800 text-neutral-400 hover:text-white transition-colors cursor-pointer"
          title="Expandir Previews de Saída"
        >
          <PanelRightOpen size={16} />
        </button>
        <span
          class="text-[10px] uppercase font-bold tracking-widest text-neutral-400 [writing-mode:vertical-lr] cursor-pointer hover:text-white transition-colors py-2"
          onclick={() => (isPreviewSidebarOpen = true)}
          role="button"
          tabindex="0"
          onkeydown={(e) => { if (e.key === 'Enter') isPreviewSidebarOpen = true; }}
        >
          Saídas (Public / Stage)
        </span>
      </aside>
    {/if}
  </div>

  <!-- Painel Inferior Colapsável e Redimensionável (Músicas, Imagens e Bíblias) -->
  <BottomPanel
    requestedTab={bottomPanelRequestedTab}
    targetScriptureRef={targetScriptureRef}
    onaddsongtosetlist={handleAddSongToSetlist}
    onpresentsong={handlePresentSong}
    onpresentimage={handlePresentImage}
    onpreviewimage={handleOpenPreviewImage}
    onaddimagetosetlist={handleAddImageToSetlist}
    onopensongeditor={handleOpenCreate}
    onopenfolderconfig={() => {
      settingsInitialTab = 'files';
      isSettingsOpen = true;
    }}
    onpresentscripture={handlePresentScripture}
    onaddscripturetosetlist={handleAddScriptureToSetlist}
    onopenbibleconfig={() => {
      settingsInitialTab = 'bibles';
      isSettingsOpen = true;
    }}
  />

  <!-- Floating Number Buffer HUD -->
  {#if numberBuffer}
    <div
      class="fixed bottom-12 left-1/2 -translate-x-1/2 z-50 px-4 py-2 bg-emerald-600 text-white font-mono font-bold text-base rounded-xl shadow-2xl flex items-center gap-2.5 border border-emerald-400/50"
    >
      <span class="text-xs uppercase tracking-wider text-emerald-200">Ir para slide:</span>
      <span class="text-lg underline decoration-2">{numberBuffer}</span>
      <span class="text-xs font-normal opacity-90">(Pressione Enter)</span>
    </div>
  {/if}

  <!-- Modal de Renomear Item -->
  {#if isRenameModalOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4 animate-in fade-in duration-100"
      onclick={() => (isRenameModalOpen = false)}
    >
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="w-full max-w-sm rounded-xl border border-neutral-800 bg-neutral-900 p-5 shadow-2xl space-y-4"
        onclick={(e) => e.stopPropagation()}
      >
        <h3 class="text-sm font-bold text-white">Renomear Item na Setlist</h3>
        <!-- svelte-ignore a11y_autofocus -->
        <input
          type="text"
          bind:value={renamingItemTitle}
          class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-xs text-white focus:outline-none focus:border-emerald-500"
          autofocus
          onkeydown={(e) => {
            if (e.key === 'Enter') handleSaveRename();
            if (e.key === 'Escape') isRenameModalOpen = false;
          }}
        />
        <div class="flex justify-end gap-2">
          <Button variant="secondary" size="sm" onclick={() => (isRenameModalOpen = false)}>
            Cancelar
          </Button>
          <Button variant="default" size="sm" onclick={handleSaveRename} class="bg-emerald-500 hover:bg-emerald-400 text-black font-semibold">
            Salvar
          </Button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Menu de Contexto Flutuante -->
  <ContextMenu
    open={contextMenuOpen}
    x={contextMenuX}
    y={contextMenuY}
    items={contextMenuItems}
    onclose={() => (contextMenuOpen = false)}
  />

  <!-- Modais Operacionais: Command Palette, Editor de Músicas, Setlist e Configurações -->
  <CommandPalette
    open={isCommandPaletteOpen}
    onclose={() => (isCommandPaletteOpen = false)}
    oncreatenew={handleOpenCreate}
    onimportsong={handleImportSong}
    onopenbible={handleOpenBibleFromPalette}
    onpresentscripture={handlePresentScriptureFromPalette}
    onopenbibletab={handleOpenBibleTab}
  />

  <SongEditorModal
    open={isSongEditorOpen}
    mode={songEditorMode}
    initialSong={editingSong}
    initialItem={editingItem}
    onclose={() => {
      isSongEditorOpen = false;
      editingSong = null;
      editingItem = null;
    }}
    onsave={handleSongSaved}
    onsaveItem={handleItemSaved}
  />

  <SetlistModal
    open={isSetlistModalOpen}
    initialSetlist={editingSetlist}
    onclose={() => {
      isSetlistModalOpen = false;
      editingSetlist = null;
    }}
    onsave={handleSetlistSaved}
  />

  <QuickItemModal
    open={isQuickItemModalOpen}
    mode={quickItemMode}
    initialItem={editingQuickItem}
    onclose={() => {
      isQuickItemModalOpen = false;
      editingQuickItem = null;
    }}
    onsave={handleSaveQuickItem}
  />

  <SettingsModal
    open={isSettingsOpen}
    initialTab={settingsInitialTab}
    onclose={() => {
      isSettingsOpen = false;
      refreshBibles();
    }}
    onbibleschanged={() => {
      refreshBibles();
    }}
  />

  <ImageSelectorModal
    open={isImageSelectorOpen}
    initialMode={imageSelectorMode}
    onclose={() => (isImageSelectorOpen = false)}
    onadditems={handleAddImageItems}
  />

  <ShareOutputsModal
    open={isShareOutputsOpen}
    onclose={() => (isShareOutputsOpen = false)}
  />

  <ImagePreviewModal
    open={isPreviewImageOpen}
    image={previewImage}
    onclose={() => {
      isPreviewImageOpen = false;
      previewImage = null;
    }}
    onpresent={handlePresentImage}
    onaddsetlist={handleAddImageToSetlist}
  />
</div>

<script lang="ts">
  import { onMount } from 'svelte';
  import {
    ChevronUp,
    ChevronDown,
    Maximize2,
    Minimize2,
    Music,
    ImageIcon,
    Plus,
    Search,
    Play,
    ListPlus,
    Upload,
    FolderPlus,
    Folder,
    RefreshCw,
    FileText,
    Eye,
    X,
    Palette,
    Sparkles,
    ChevronRight,
    Check,
    Layers,
    FolderOpen,
    BookOpen,
    Film,
    ShieldCheck,
    Tag,
    Boxes,
    CalendarDays,
    Bookmark,
    Pencil,
    Trash2,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import { cn } from '../utils';
  import BiblePanel from './BiblePanel.svelte';
  import { renderFormattedText } from '../markdown';
  import { notifySuccess, notifyError } from '../toast';
  import {
    customItemsStore,
    loadCustomItems,
    deleteCustomItem,
    importItemsFromSetlist,
  } from '../customItems';
  import {
    BACKGROUND_PRESETS,
    getAllBackgrounds,
    encodeBackground,
    resolveBackground,
    type BackgroundPreset,
  } from '../backgrounds';
  import { presentationState, setPresentationWallpaper } from '../ws';
  import {
    publicSettingsStore,
    savePublicDisplaySettings,
    backgroundStylesStore,
  } from '../displaySettings';
  import { refreshBibles } from '../bibles';
  import {
    ResizablePaneGroup,
    ResizablePane,
    ResizableHandle,
  } from './ui/resizable';
  import { parseRawLyrics, getSectionColor } from '../songParser';
  import type {
    SongSummary,
    Song,
    Slide,
    ImageMediaItem,
    MediaFolder,
    WallpaperItem,
    ImageFitMode,
    MediaItem,
    ParsedReference,
  } from '../types';

  interface Props {
    onaddsongtosetlist?: (songId: string) => void;
    onpresentsong?: (songId: string) => void;
    onpresentimage?: (image: ImageMediaItem) => void;
    onaddimagetosetlist?: (image: ImageMediaItem) => void;
    onpreviewimage?: (image: ImageMediaItem) => void;
    onopensongeditor?: () => void;
    onopenfolderconfig?: () => void;
    onpresentscripture?: (item: MediaItem, slideIndex: number) => void;
    onaddscripturetosetlist?: (item: MediaItem) => void;
    onopenbibleconfig?: () => void;
    onadditemtosetlist?: (item: MediaItem) => void;
    onpresentitem?: (item: MediaItem, slideIndex?: number) => void;
    onopenedititem?: (item: MediaItem) => void;
    onopenscheduleeditor?: () => void;
    onopentexteditor?: () => void;
    onopenquickitemeditor?: (mode: 'header' | 'announcement') => void;
    activeSetlistItems?: MediaItem[];
    requestedTab?: 'songs' | 'images' | 'backgrounds' | 'bible' | 'others' | null;
    targetScriptureRef?: ParsedReference | null;
  }

  let {
    onaddsongtosetlist,
    onpresentsong,
    onpresentimage,
    onaddimagetosetlist,
    onpreviewimage,
    onopensongeditor,
    onopenfolderconfig,
    onpresentscripture,
    onaddscripturetosetlist,
    onopenbibleconfig,
    onadditemtosetlist,
    onpresentitem,
    onopenedititem,
    onopenscheduleeditor,
    onopentexteditor,
    onopenquickitemeditor,
    activeSetlistItems = [],
    requestedTab = null,
    targetScriptureRef = null,
  }: Props = $props();

  let isCollapsed = $state(true);
  let isMaximized = $state(false);
  let activeTab = $state<'songs' | 'images' | 'backgrounds' | 'bible' | 'others'>('songs');
  let panelHeight = $state(290);
  let isDragging = $state(false);
  let startY = 0;
  let startHeight = 290;

  function selectTab(tab: 'songs' | 'images' | 'backgrounds' | 'bible' | 'others') {
    if (tab === 'bible') {
      if (isCollapsed || activeTab !== 'bible') {
        const h75 = typeof window !== 'undefined' ? Math.round(window.innerHeight * 0.75) : 550;
        const maxH = typeof window !== 'undefined' ? window.innerHeight - 80 : 700;
        const targetH = Math.min(Math.max(panelHeight, h75), maxH);
        panelHeight = targetH;
      }
    } else if (tab === 'others') {
      loadCustomItems();
    }
    activeTab = tab;
    if (isCollapsed) isCollapsed = false;
  }

  $effect(() => {
    if (requestedTab) {
      selectTab(requestedTab);
    }
  });

  $effect(() => {
    if (activeTab === 'bible') {
      refreshBibles();
    } else if (activeTab === 'songs') {
      loadSongs();
      loadSongTags();
    } else if (activeTab === 'others') {
      loadCustomItems();
    }
  });

  $effect(() => {
    const handleSongsChanged = () => {
      loadSongs();
      loadSongTags();
    };
    if (typeof window !== 'undefined') {
      window.addEventListener('openpresenter:songs-changed', handleSongsChanged);
      return () => {
        window.removeEventListener('openpresenter:songs-changed', handleSongsChanged);
      };
    }
  });

  // --- CATÁLOGO DE MÚSICAS ---
  let songSearch = $state('');
  let songs = $state<SongSummary[]>([]);
  let songTags = $state<string[]>([]);
  let selectedSongTag = $state<string | null>(null);
  let songLimit = $state<number>(
    typeof window !== 'undefined'
      ? Number(localStorage.getItem('open_presenter_song_limit') || '50')
      : 50
  );
  let isLoadingSongs = $state(false);
  let isImportingSongs = $state(false);
  let songFileInputEl: HTMLInputElement | null = $state(null);
  let viewingSong = $state<Song | null>(null);
  let isLoadingLyrics = $state(false);

  // Slides derivados com separação correta a partir de quebras de estrofes (\n\n) e seções (##)
  const viewingSongSlides = $derived.by<Slide[]>(() => {
    if (!viewingSong) return [];
    if (viewingSong.lyrics_raw && viewingSong.lyrics_raw.trim()) {
      const parsed = parseRawLyrics(viewingSong.lyrics_raw);
      if (parsed.slides && parsed.slides.length > 0) {
        return parsed.slides;
      }
    }
    if (viewingSong.sections && viewingSong.sections.length > 0) {
      const result: Slide[] = [];
      let slideNum = 1;
      for (const sec of viewingSong.sections) {
        const text = sec.lines.join('\n');
        const blocks = text.split(/\n\s*\n/).filter((b) => b.trim().length > 0);
        if (blocks.length > 1) {
          blocks.forEach((block, bIdx) => {
            result.push({
              id: `slide-${slideNum++}`,
              label: `${sec.label || 'Parte'} (${bIdx + 1}/${blocks.length})`,
              content: block,
            });
          });
        } else {
          result.push({
            id: `slide-${slideNum++}`,
            label: sec.label || `Parte ${slideNum}`,
            content: text,
          });
        }
      }
      return result;
    }
    const blocks = (viewingSong.lyrics_raw || '').split(/\n\s*\n/).filter((b) => b.trim().length > 0);
    return blocks.map((b, idx) => ({
      id: `slide-${idx + 1}`,
      label: `Slide ${idx + 1}`,
      content: b.trim(),
    }));
  });

  // --- GALERIA DE IMAGENS ---
  let images = $state<ImageMediaItem[]>([]);
  let folders = $state<MediaFolder[]>([]);
  let selectedFolderFilter = $state<string>('all');
  let isFoldersTreeExpanded = $state(true);
  let isLoadingImages = $state(false);
  let fileInputEl: HTMLInputElement | null = $state(null);

  // Por padrão exibe apenas pastas importadas por privacidade, com opção de habilitar imagens do sistema
  let showSystemImages = $state<boolean>(
    typeof window !== 'undefined'
      ? localStorage.getItem('open_presenter_show_system_images') === 'true'
      : false
  );

  function toggleShowSystemImages() {
    showSystemImages = !showSystemImages;
    if (typeof window !== 'undefined') {
      localStorage.setItem('open_presenter_show_system_images', String(showSystemImages));
    }
  }

  const visibleImagesCount = $derived(
    showSystemImages ? images.length : images.filter((img) => img.is_external).length
  );

  function handleSetImageFit(fit: ImageFitMode) {
    const updated = { ...$publicSettingsStore, image_fit: fit };
    savePublicDisplaySettings(updated);
  }

  function handleToggleMediaMargins() {
    const current = $publicSettingsStore.ignore_margins_for_media !== false;
    const updated = { ...$publicSettingsStore, ignore_margins_for_media: !current };
    savePublicDisplaySettings(updated);
  }

  // --- ABA DE FUNDOS & GRADIENTES ---
  let selectedBgCategory = $state<
    'all' | 'gradients' | 'mesh' | 'animated' | 'local' | 'custom'
  >('all');
  let bgSearchQuery = $state('');
  let localWallpapers = $state<WallpaperItem[]>([]);
  let isLoadingWallpapers = $state(false);

  onMount(() => {
    loadSongs();
    loadSongTags();
    loadImages();
    loadFolders();
    loadWallpapers();
    loadCustomItems();
  });

  async function loadSongTags() {
    try {
      const res = await fetch('/api/songs/tags');
      if (res.ok) {
        songTags = await res.json();
      }
    } catch (err) {
      console.warn('Erro ao carregar tags de músicas:', err);
    }
  }

  async function loadSongs() {
    isLoadingSongs = true;
    try {
      const params = new URLSearchParams();
      const q = songSearch.trim();
      if (q) params.set('q', q);
      if (selectedSongTag) params.set('tag', selectedSongTag);
      if (songLimit > 0) params.set('limit', String(songLimit));
      const url = params.toString() ? `/api/songs?${params.toString()}` : '/api/songs';
      const res = await fetch(url);
      if (res.ok) {
        songs = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar músicas:', err);
    } finally {
      isLoadingSongs = false;
    }
  }

  function handleSongLimitChange(newLimit: number) {
    songLimit = newLimit;
    if (typeof window !== 'undefined') {
      localStorage.setItem('open_presenter_song_limit', String(newLimit));
    }
    loadSongs();
  }

  function selectSongTag(tag: string | null) {
    selectedSongTag = tag;
    loadSongs();
  }

  function triggerSongImport() {
    songFileInputEl?.click();
  }

  async function handleSongFilesSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = input.files;
    if (!files || files.length === 0) return;

    isImportingSongs = true;
    let totalImported = 0;
    try {
      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        const content = await file.text();
        const res = await fetch('/api/songs/import', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content,
            tags: selectedSongTag ? [selectedSongTag] : [],
          }),
        });
        if (res.ok) {
          const data = await res.json();
          totalImported += data.imported_count || 1;
        }
      }
      notifySuccess(`${totalImported} música(s) importada(s) com sucesso!`);
      await loadSongs();
      await loadSongTags();
    } catch (err) {
      console.error('Erro ao importar arquivos de música:', err);
      notifyError('Erro ao importar músicas', String(err));
    } finally {
      isImportingSongs = false;
      input.value = '';
    }
  }

  async function handleViewSongLyrics(songId: string) {
    isLoadingLyrics = true;
    try {
      const res = await fetch(`/api/songs/${songId}`);
      if (res.ok) {
        viewingSong = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar letra:', err);
    } finally {
      isLoadingLyrics = false;
    }
  }

  async function loadImages() {
    isLoadingImages = true;
    try {
      const res = await fetch('/api/media/images');
      if (res.ok) {
        images = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar imagens:', err);
    } finally {
      isLoadingImages = false;
    }
  }

  async function loadFolders() {
    try {
      const res = await fetch('/api/media/folders');
      if (res.ok) {
        folders = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar pastas:', err);
    }
  }

  async function loadWallpapers() {
    isLoadingWallpapers = true;
    try {
      const res = await fetch('/api/media/wallpapers');
      if (res.ok) {
        localWallpapers = await res.json();
      }
    } catch (err) {
      console.error('Erro ao carregar papéis de parede:', err);
    } finally {
      isLoadingWallpapers = false;
    }
  }

  let isUploadingWallpaper = $state(false);
  let wallpaperFileInput = $state<HTMLInputElement>();

  function triggerImportWallpaper() {
    wallpaperFileInput?.click();
  }

  async function handleWallpaperUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];
    isUploadingWallpaper = true;
    try {
      const reader = new FileReader();
      reader.onload = async () => {
        const b64 = (reader.result as string).split(',')[1];
        const res = await fetch('/api/media/upload', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content_base64: b64,
          }),
        });
        if (res.ok) {
          await loadWallpapers();
        }
        isUploadingWallpaper = false;
        input.value = '';
      };
      reader.readAsDataURL(file);
    } catch (err) {
      console.error('Falha ao importar fundo:', err);
      isUploadingWallpaper = false;
    }
  }

  let selectedSubfolder = $state<string | null>(null);
  let expandedFolders = $state<Record<string, boolean>>({});

  function toggleFolderExpand(folderId: string, e?: MouseEvent) {
    e?.stopPropagation();
    expandedFolders = {
      ...expandedFolders,
      [folderId]: !expandedFolders[folderId],
    };
  }

  // Sub-pastas da pasta atualmente selecionada
  const currentSubfolders = $derived.by(() => {
    if (selectedFolderFilter === 'all') return [];
    if (selectedFolderFilter === 'app') {
      const set = new Map<string, number>();
      for (const img of images) {
        if (!img.is_external && img.subfolder) {
          set.set(img.subfolder, (set.get(img.subfolder) || 0) + 1);
        }
      }
      return Array.from(set.entries()).map(([sub, count]) => ({
        relative_path: sub,
        name: sub.split('/').pop() || sub,
        count,
      }));
    }
    const f = folders.find((item) => item.id === selectedFolderFilter);
    return f?.subfolders || [];
  });

  // Filtragem de imagens com suporte a sub-pastas e privacidade
  const filteredImages = $derived.by(() => {
    let list = images;
    // Por padrão (privacidade ativa), exibe apenas pastas importadas externas
    if (!showSystemImages && selectedFolderFilter !== 'app') {
      list = list.filter((img) => img.is_external);
    }
    if (selectedFolderFilter === 'app') {
      list = list.filter((img) => !img.is_external);
    } else if (selectedFolderFilter !== 'all') {
      list = list.filter((img) => img.folder_id === selectedFolderFilter);
    }
    if (selectedSubfolder !== null) {
      list = list.filter((img) => img.subfolder === selectedSubfolder);
    }
    return list;
  });

  // Nome da pasta selecionada para Breadcrumbs
  const currentFolderName = $derived.by(() => {
    if (selectedFolderFilter === 'all') return showSystemImages ? 'Todas as Imagens (+ Sistema)' : 'Pastas Importadas';
    if (selectedFolderFilter === 'app') return 'Pasta do App (Sistema)';
    const f = folders.find((item) => item.id === selectedFolderFilter);
    return f ? f.name : 'Pasta Selecionada';
  });

  const availableBackgrounds = $derived(getAllBackgrounds($backgroundStylesStore));

  // Filtragem de fundos
  const filteredBackgrounds = $derived.by(() => {
    return availableBackgrounds.filter((p) => {
      const matchesCat = selectedBgCategory === 'all' || p.category === selectedBgCategory;
      const matchesSearch =
        bgSearchQuery === '' ||
        p.name.toLowerCase().includes(bgSearchQuery.toLowerCase()) ||
        (p.description && p.description.toLowerCase().includes(bgSearchQuery.toLowerCase()));
      return matchesCat && matchesSearch;
    });
  });

  // Aplicação instantânea de fundo no projetor respeitando wallpaper_fit configurado
  function applyBackground(encodedOrUrl: string) {
    const fit = $publicSettingsStore.wallpaper_fit || 'contain';
    setPresentationWallpaper(encodedOrUrl, fit);
  }

  // Arraste fluido para redimensionamento de altura sem delay
  function startResize(e: MouseEvent) {
    if (isMaximized) return;
    e.preventDefault();
    isDragging = true;
    startY = e.clientY;
    startHeight = panelHeight;
    document.body.style.cursor = 'row-resize';
    document.body.style.userSelect = 'none';

    window.addEventListener('mousemove', handleResizeMove, { passive: false });
    window.addEventListener('mouseup', stopResize);
  }

  function handleResizeMove(e: MouseEvent) {
    if (!isDragging) return;
    const delta = startY - e.clientY;
    const minH = 140;
    const maxH = window.innerHeight - 80;
    const newHeight = Math.max(minH, Math.min(maxH, startHeight + delta));
    panelHeight = newHeight;
    if (isCollapsed) isCollapsed = false;
  }

  function stopResize() {
    isDragging = false;
    document.body.style.cursor = '';
    document.body.style.userSelect = '';
    window.removeEventListener('mousemove', handleResizeMove);
    window.removeEventListener('mouseup', stopResize);
  }

  function toggleCollapse() {
    if (isCollapsed && activeTab === 'bible') {
      const h75 = typeof window !== 'undefined' ? Math.round(window.innerHeight * 0.75) : 550;
      const maxH = typeof window !== 'undefined' ? window.innerHeight - 80 : 700;
      const targetH = Math.min(Math.max(panelHeight, h75), maxH);
      panelHeight = targetH;
    }
    isCollapsed = !isCollapsed;
  }

  function toggleMaximize() {
    isMaximized = !isMaximized;
    if (isMaximized) isCollapsed = false;
  }

  // Upload de Imagem para pasta do App
  async function handleFileUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];

    const reader = new FileReader();
    reader.onload = async () => {
      const base64 = reader.result as string;
      try {
        const resp = await fetch('/api/media/images/upload', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content_base64: base64,
          }),
        });
        if (resp.ok) {
          await loadImages();
        }
      } catch (err) {
        console.error('Erro no upload de imagem:', err);
      }
    };
    reader.readAsDataURL(file);
    input.value = '';
  }

  // --- ABA OUTROS (AGENDA, TEXTOS, AVISOS, SEÇÕES) ---
  let othersSearch = $state('');
  let othersKindFilter = $state<'all' | 'schedule' | 'text' | 'announcement' | 'header'>('all');
  let viewingCustomItem = $state<MediaItem | null>(null);
  let isImportingFromSetlist = $state(false);

  const filteredCustomItems = $derived.by(() => {
    return $customItemsStore.filter((item) => {
      const matchesKind =
        othersKindFilter === 'all' || item.kind === othersKindFilter;
      const term = othersSearch.trim().toLowerCase();
      if (!term) return matchesKind;

      const titleMatch = item.title.toLowerCase().includes(term);
      const artistMatch = item.artist_or_ref?.toLowerCase().includes(term);
      const slideMatch = item.slides.some(
        (s) =>
          s.label?.toLowerCase().includes(term) ||
          s.content?.toLowerCase().includes(term) ||
          (s.metadata?.schedule_day && s.metadata.schedule_day.toLowerCase().includes(term)) ||
          (s.metadata?.schedule_time && s.metadata.schedule_time.toLowerCase().includes(term)) ||
          (s.metadata?.schedule_details && s.metadata.schedule_details.toLowerCase().includes(term))
      );

      return matchesKind && (titleMatch || artistMatch || slideMatch);
    });
  });

  $effect(() => {
    if (activeTab === 'others') {
      if (!viewingCustomItem && filteredCustomItems.length > 0) {
        viewingCustomItem = filteredCustomItems[0];
      } else if (viewingCustomItem && !filteredCustomItems.some((i) => i.id === viewingCustomItem?.id)) {
        viewingCustomItem = filteredCustomItems[0] || null;
      }
    }
  });

  async function handleImportFromSetlistClick() {
    if (!activeSetlistItems || activeSetlistItems.length === 0) {
      notifyError('Setlist Vazia', 'Não há itens na setlist ativa para importar.');
      return;
    }
    isImportingFromSetlist = true;
    try {
      const count = await importItemsFromSetlist(activeSetlistItems);
      if (count > 0) {
        notifySuccess('Importação Concluída', `${count} novo(s) item(ns) importado(s) para o catálogo Outros.`);
      } else {
        notifySuccess('Tudo Atualizado', 'Todos os itens suportados da setlist já estão no catálogo.');
      }
    } catch (err) {
      notifyError('Erro na Importação', 'Não foi possível importar os itens da setlist.');
    } finally {
      isImportingFromSetlist = false;
    }
  }

  async function handleDeleteCustomItemClick(item: MediaItem, e: MouseEvent) {
    e.stopPropagation();
    if (confirm(`Deseja realmente excluir "${item.title}" do catálogo Outros?`)) {
      await deleteCustomItem(item.id);
      if (viewingCustomItem?.id === item.id) {
        viewingCustomItem = null;
      }
      notifySuccess('Item Excluído', `"${item.title}" foi removido do catálogo.`);
    }
  }

  function getItemKindBadge(kind: MediaItem['kind']) {
    switch (kind) {
      case 'schedule':
        return { label: 'Agenda', color: 'border-amber-500/40 bg-amber-500/10 text-amber-300' };
      case 'text':
        return { label: 'Texto', color: 'border-blue-500/40 bg-blue-500/10 text-blue-300' };
      case 'announcement':
        return { label: 'Aviso', color: 'border-emerald-500/40 bg-emerald-500/10 text-emerald-300' };
      case 'header':
        return { label: 'Seção', color: 'border-cyan-500/40 bg-cyan-500/10 text-cyan-300' };
      default:
        return { label: 'Outro', color: 'border-neutral-500/40 bg-neutral-500/10 text-neutral-300' };
    }
  }
</script>

<!-- Input Oculto de Arquivo de Imagem -->
<input
  type="file"
  accept="image/*"
  bind:this={fileInputEl}
  onchange={handleFileUpload}
  class="hidden"
/>

<div
  class={cn(
    'border-t border-border bg-card/95 backdrop-blur-md flex flex-col shrink-0 select-none relative z-30',
    !isDragging && 'transition-[height] duration-200 ease-out'
  )}
  style={isCollapsed
    ? 'height: 48px;'
    : isMaximized
      ? 'height: calc(100vh - 56px);'
      : `height: ${panelHeight}px;`}
>
  <!-- Handle de Redimensionamento Superior Acessível com Grip do Shadcn -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="separator"
    tabindex="0"
    aria-orientation="horizontal"
    aria-label="Ajustar altura do painel de mídias (setas para cima ou para baixo)"
    class={cn(
      'group h-2 w-full cursor-row-resize bg-card hover:bg-card focus-visible:bg-muted focus-visible:outline-none transition-colors flex items-center justify-center shrink-0 select-none mb-0.5',
      isDragging && 'bg-card'
    )}
    onmousedown={startResize}
    onkeydown={(e) => {
      if (e.key === 'ArrowUp') {
        panelHeight = Math.min(panelHeight + 20, window.innerHeight - 80);
        isCollapsed = false;
      } else if (e.key === 'ArrowDown') {
        panelHeight = Math.max(140, panelHeight - 20);
      }
    }}
    title="Arrastar ou usar setas para ajustar altura"
  >
    <div
      class={cn(
        'h-1 w-9 rounded-full bg-neutral-400 dark:bg-neutral-500 group-hover:bg-primary group-focus-visible:bg-primary transition-colors',
        isDragging && 'bg-primary'
      )}
    ></div>
  </div>

  <!-- Barra de Abas e Controles -->
  <div
    role="tablist"
    aria-label="Mídias e recursos do painel inferior"
    class="h-10 px-3 border-b border-border flex items-center justify-between shrink-0 bg-card"
  >
    <!-- Abas Disponíveis -->
    <div class="flex items-center gap-1">
      <!-- Aba Músicas -->
      <button
        type="button"
        role="tab"
        id="tab-songs"
        aria-controls="panel-songs"
        aria-selected={activeTab === 'songs' && !isCollapsed}
        onclick={() => selectTab('songs')}
        class={cn(
          'flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none',
          activeTab === 'songs' && !isCollapsed
            ? 'bg-muted text-foreground shadow-xs border border-border'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'
        )}
      >
        <Music size={13} class="text-primary" />
        <span>Músicas</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {songs.length}
        </Badge>
      </button>

      <!-- Aba Imagens -->
      <button
        type="button"
        role="tab"
        id="tab-images"
        aria-controls="panel-images"
        aria-selected={activeTab === 'images' && !isCollapsed}
        onclick={() => selectTab('images')}
        class={cn(
          'flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none',
          activeTab === 'images' && !isCollapsed
            ? 'bg-muted text-foreground shadow-xs border border-border'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'
        )}
      >
        <ImageIcon size={13} class="text-cyan-400" />
        <span>Imagens & Pastas</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {visibleImagesCount}
        </Badge>
      </button>

      <!-- Aba Fundos & Gradientes -->
      <button
        type="button"
        role="tab"
        id="tab-backgrounds"
        aria-controls="panel-backgrounds"
        aria-selected={activeTab === 'backgrounds' && !isCollapsed}
        onclick={() => selectTab('backgrounds')}
        class={cn(
          'flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none',
          activeTab === 'backgrounds' && !isCollapsed
            ? 'bg-muted text-foreground shadow-xs border border-border'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'
        )}
      >
        <Palette size={13} class="text-fuchsia-400" />
        <span>Fundos & Gradientes</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {BACKGROUND_PRESETS.length + localWallpapers.length}
        </Badge>
      </button>

      <!-- Aba Bíblia -->
      <button
        type="button"
        role="tab"
        id="tab-bible"
        aria-controls="panel-bible"
        aria-selected={activeTab === 'bible' && !isCollapsed}
        onclick={() => selectTab('bible')}
        class={cn(
          'flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none',
          activeTab === 'bible' && !isCollapsed
            ? 'bg-muted text-foreground shadow-xs border border-border'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'
        )}
      >
        <BookOpen size={13} class="text-amber-400" />
        <span>Bíblia</span>
      </button>

      <!-- Aba Outros (Agenda, Textos, Avisos, Seções) -->
      <button
        type="button"
        role="tab"
        id="tab-others"
        aria-controls="panel-others"
        aria-selected={activeTab === 'others' && !isCollapsed}
        onclick={() => selectTab('others')}
        class={cn(
          'flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none',
          activeTab === 'others' && !isCollapsed
            ? 'bg-muted text-foreground shadow-xs border border-border'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'
        )}
      >
        <Boxes size={13} class="text-emerald-400" />
        <span>Outros</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {$customItemsStore.length}
        </Badge>
      </button>
    </div>

    <!-- Controles de Janela do Painel Inferior -->
    <div class="flex items-center gap-1 text-muted-foreground">
      <button
        type="button"
        onclick={toggleMaximize}
        aria-label={isMaximized ? 'Restaurar altura do painel de mídias' : 'Maximizar painel de mídias'}
        title={isMaximized ? 'Restaurar Altura' : 'Expandir 100%'}
        class="w-8 h-8 rounded hover:bg-muted hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none"
      >
        {#if isMaximized}
          <Minimize2 size={13} />
        {:else}
          <Maximize2 size={13} />
        {/if}
      </button>

      <button
        type="button"
        onclick={toggleCollapse}
        aria-label={isCollapsed ? 'Expandir painel de mídias' : 'Recolher painel de mídias'}
        title={isCollapsed ? 'Abrir Painel' : 'Recolher Painel'}
        class="w-8 h-8 rounded hover:bg-muted hover:text-foreground flex items-center justify-center transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none"
      >
        {#if isCollapsed}
          <ChevronUp size={15} />
        {:else}
          <ChevronDown size={15} />
        {/if}
      </button>
    </div>
  </div>

  <!-- Conteúdo das Abas (Exibido quando não estiver colapsado) -->
  {#if !isCollapsed}
    <div class="flex-1 flex flex-col overflow-hidden bg-background/60 p-3">
      <!-- ================= ABA 1: MÚSICAS ================= -->
      {#if activeTab === 'songs'}
        <div role="tabpanel" id="panel-songs" aria-labelledby="tab-songs" class="flex-1 flex flex-col min-w-0 overflow-hidden">
          <!-- Barra de Ações da Aba Músicas -->
        <div class="flex items-center justify-between gap-3 mb-2 shrink-0">
          <div class="relative flex-1 max-w-md">
            <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-neutral-500" />
            <input
              type="text"
              bind:value={songSearch}
              oninput={loadSongs}
              placeholder="Pesquisar catálogo de músicas (clique no card para abrir letra)..."
              class="w-full h-7 pl-8 pr-3 bg-neutral-900 border border-neutral-800 rounded-md text-xs text-neutral-200 placeholder-neutral-500 focus:outline-none focus:border-primary/60"
            />
          </div>

          <div class="flex items-center gap-2">
            <!-- Seletor de limite de exibição: 50, 100, 150, 200 ou todas -->
            <div class="flex items-center gap-1.5 bg-neutral-900 border border-neutral-800 rounded-md px-2 h-7 text-xs text-neutral-300">
              <span class="text-[11px] text-neutral-400 font-medium hidden sm:inline">Exibir:</span>
              <select
                value={songLimit}
                onchange={(e) => handleSongLimitChange(Number((e.target as HTMLSelectElement).value))}
                aria-label="Filtrar quantidade de músicas exibidas"
                class="bg-transparent border-none text-xs text-foreground font-medium focus:outline-none cursor-pointer p-0"
              >
                <option value={50} class="bg-neutral-900 text-foreground">50</option>
                <option value={100} class="bg-neutral-900 text-foreground">100</option>
                <option value={150} class="bg-neutral-900 text-foreground">150</option>
                <option value={200} class="bg-neutral-900 text-foreground">200</option>
                <option value={0} class="bg-neutral-900 text-foreground">Todas</option>
              </select>
            </div>

            <Button
              variant="outline"
              size="sm"
              onclick={() => { loadSongs(); loadSongTags(); }}
              title="Recarregar catálogo"
              class="h-7 text-xs px-2"
            >
              <RefreshCw size={12} class={isLoadingSongs ? 'animate-spin' : ''} />
            </Button>
            <Button
              variant="outline"
              size="sm"
              onclick={triggerSongImport}
              disabled={isImportingSongs}
              title="Importar músicas (.json, .txt, .md)"
              class="h-7 text-xs gap-1 cursor-pointer"
            >
              <Upload size={12} class={isImportingSongs ? 'animate-spin text-primary' : 'text-primary'} />
              <span>{isImportingSongs ? 'Importando...' : 'Importar'}</span>
            </Button>
            <input
              type="file"
              bind:this={songFileInputEl}
              onchange={handleSongFilesSelected}
              accept=".json,.txt,.md,text/plain,application/json,text/markdown"
              multiple
              class="hidden"
            />
            <Button
              variant="default"
              size="sm"
              onclick={onopensongeditor}
              class="h-7 text-xs gap-1"
            >
              <Plus size={13} />
              <span>Nova Música</span>
            </Button>
          </div>
        </div>

        <!-- Barra de Filtros de Tags e Grupos -->
        {#if songTags.length > 0}
          <div class="flex items-center gap-1.5 mb-2.5 overflow-x-auto pb-1 text-xs shrink-0 select-none">
            <span class="text-[10px] text-muted-foreground font-semibold uppercase tracking-wider mr-1 flex items-center gap-1">
              <Tag size={11} /> Tags:
            </span>
            <button
              type="button"
              onclick={() => selectSongTag(null)}
              class="px-2 py-0.5 rounded-full text-[11px] font-medium transition-colors cursor-pointer {selectedSongTag === null ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'bg-muted hover:bg-muted/80 text-muted-foreground'}"
            >
              Todas
            </button>
            {#each songTags as tag}
              <button
                type="button"
                onclick={() => selectSongTag(tag)}
                class="px-2 py-0.5 rounded-full text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-0.5 {selectedSongTag === tag ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'bg-muted hover:bg-muted/80 text-muted-foreground'}"
              >
                <span>#{tag}</span>
              </button>
            {/each}
          </div>
        {/if}

        {#snippet songCard(song: SongSummary)}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            onclick={() => handleViewSongLyrics(song.id)}
            onkeydown={(e) => { if (e.key === 'Enter') handleViewSongLyrics(song.id); }}
            role="button"
            tabindex="0"
            class="p-2.5 rounded-lg border border-border bg-card/60 hover:bg-muted/40 transition-all flex flex-col justify-between group cursor-pointer {viewingSong?.id === song.id ? 'ring-1 ring-primary border-primary bg-primary/10' : ''}"
          >
            <div>
              <div class="flex items-center justify-between gap-1 mb-1">
                <h4 class="text-xs font-bold text-foreground truncate">{song.title}</h4>
                {#if song.song_key}
                  <Badge variant="outline" class="text-[9px] py-0 px-1 font-mono text-primary shrink-0">
                    {song.song_key}
                  </Badge>
                {/if}
              </div>
              {#if song.artist}
                <p class="text-[11px] text-muted-foreground truncate">{song.artist}</p>
              {/if}
              {#if song.preview_snippet}
                <p class="text-[10px] text-muted-foreground line-clamp-2 mt-1 italic">
                  "{song.preview_snippet}"
                </p>
              {/if}
              {#if song.tags && song.tags.length > 0}
                <div class="flex flex-wrap gap-1 mt-1.5">
                  {#each song.tags.slice(0, 3) as tag}
                    <span class="inline-flex items-center px-1.5 py-0.5 rounded text-[9px] font-medium bg-muted text-muted-foreground border border-border/70">
                      #{tag}
                    </span>
                  {/each}
                  {#if song.tags.length > 3}
                    <span class="text-[9px] text-muted-foreground self-center">+{song.tags.length - 3}</span>
                  {/if}
                </div>
              {/if}
            </div>

            <div class="flex items-center justify-between gap-1.5 mt-2 pt-2 border-t border-border">
              <span class="text-[10px] {viewingSong?.id === song.id ? 'text-primary font-bold' : 'text-primary font-semibold'} flex items-center gap-1 shrink-0 truncate">
                <FileText size={11} /> {viewingSong?.id === song.id ? 'Visualizando' : 'Ver Letra'}
              </span>

              <div class="flex items-center gap-1 shrink-0">
                <button
                  type="button"
                  onclick={(e) => {
                    e.stopPropagation();
                    onaddsongtosetlist?.(song.id);
                  }}
                  class="px-2 py-0.5 rounded bg-muted hover:bg-muted/80 text-[10px] text-foreground font-medium flex items-center gap-1 transition-colors cursor-pointer shrink-0"
                  title="Adicionar à Setlist"
                >
                  <ListPlus size={11} class="text-primary" />
                  + Setlist
                </button>

                <button
                  type="button"
                  onclick={(e) => {
                    e.stopPropagation();
                    onpresentsong?.(song.id);
                  }}
                  class="px-2 py-0.5 rounded bg-primary/20 hover:bg-primary/30 text-[10px] text-primary font-bold flex items-center gap-1 transition-colors cursor-pointer shrink-0"
                  title="Apresentar Imediatamente"
                >
                  <Play size={11} class="fill-current" />
                  Ao Vivo
                </button>
              </div>
            </div>
          </div>
        {/snippet}

        {#snippet viewingSongSheet()}
          {#if viewingSong}
            <div class="flex-1 min-w-0 border border-border bg-card/60 rounded-xl flex flex-col p-3.5 h-full overflow-hidden shadow-md animate-in slide-in-from-right duration-150">
              <!-- Cabeçalho do Sheet -->
              <div class="flex items-center justify-between pb-2.5 border-b border-border shrink-0">
                <div class="min-w-0 mr-3">
                  <div class="flex items-center gap-2">
                    <h4 class="text-sm font-bold text-foreground truncate">{viewingSong.title}</h4>
                    {#if viewingSongSlides.length > 0}
                      <Badge variant="secondary" class="text-[10px] font-mono">
                        {viewingSongSlides.length} slides
                      </Badge>
                    {/if}
                    {#if viewingSong.song_key}
                      <Badge variant="outline" class="text-[10px] py-0 px-1.5 font-mono text-primary">
                        Tom: {viewingSong.song_key}
                      </Badge>
                    {/if}
                    {#if viewingSong.bpm}
                      <span class="text-[10px] text-muted-foreground font-mono">
                        {viewingSong.bpm} BPM
                      </span>
                    {/if}
                  </div>
                  {#if viewingSong.artist || (viewingSong.tags && viewingSong.tags.length > 0)}
                    <div class="flex items-center gap-2 mt-0.5">
                      {#if viewingSong.artist}
                        <p class="text-xs text-muted-foreground truncate">{viewingSong.artist}</p>
                      {/if}
                      {#if viewingSong.tags && viewingSong.tags.length > 0}
                        <div class="flex items-center gap-1 overflow-x-auto">
                          {#each viewingSong.tags as tag}
                            <span class="inline-flex items-center px-1.5 py-0.2 rounded text-[9px] bg-secondary text-secondary-foreground font-mono">
                              #{tag}
                            </span>
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {/if}
                </div>

                <div class="flex items-center gap-1.5">
                  <Button
                    variant="secondary"
                    size="sm"
                    onclick={() => viewingSong && onaddsongtosetlist?.(viewingSong.id)}
                    class="h-7 text-xs gap-1"
                  >
                    <ListPlus size={12} class="text-primary" />
                    + Setlist
                  </Button>
                  <Button
                    variant="live"
                    size="sm"
                    onclick={() => viewingSong && onpresentsong?.(viewingSong.id)}
                    class="h-7 text-xs gap-1 font-bold"
                  >
                    <Play size={12} class="fill-current" />
                    Apresentar Música
                  </Button>
                  <button
                    type="button"
                    onclick={() => (viewingSong = null)}
                    class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer ml-1"
                    title="Fechar visualização de letra"
                  >
                    <X size={16} />
                  </button>
                </div>
              </div>

              <!-- Prévia dos Slides com Grid Responsivo em 1, 2 ou 3 colunas (Aspect Ratio 16:9) -->
              <div class="flex-1 overflow-y-auto py-3 pr-1 min-h-0">
                {#if viewingSongSlides.length > 0}
                  <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3">
                    {#each viewingSongSlides as slide, idx (slide.id || idx)}
                      {@const color = getSectionColor(slide.label)}
                      <div class="rounded-xl border border-border bg-background/60 overflow-hidden flex flex-col shadow-xs transition-all hover:border-primary/50">
                        <!-- Topo do Slide -->
                        <div class="px-3 py-1.5 bg-muted/40 border-b border-border flex items-center justify-between text-[10px]">
                          <span class="uppercase font-bold text-primary flex items-center gap-1.5 truncate">
                            <span class="w-4 h-4 rounded-full {color.accent || 'bg-primary/20'} text-primary flex items-center justify-center text-[9px] font-mono font-bold shrink-0">
                              {idx + 1}
                            </span>
                            <span class="truncate">{slide.label || `Slide ${idx + 1}`}</span>
                          </span>
                          <span class="text-[9px] text-muted-foreground font-mono shrink-0 ml-1">16:9</span>
                        </div>

                        <!-- Conteúdo do Slide em proporção 16:9 -->
                        <div class="aspect-video w-full p-3.5 flex flex-col items-center justify-center text-center bg-black/40 overflow-hidden">
                          <div class="text-xs sm:text-[13px] font-semibold text-foreground leading-relaxed line-clamp-6">
                            {@html renderFormattedText(slide.content)}
                          </div>
                        </div>
                      </div>
                    {/each}
                  </div>
                {:else if viewingSong.lyrics_raw}
                  <div class="p-4 rounded-xl border border-border bg-background/50 text-xs font-medium text-foreground leading-relaxed whitespace-pre-wrap">
                    {@html renderFormattedText(viewingSong.lyrics_raw)}
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        {/snippet}

        <!-- Grade/Lista de Músicas com Suporte a Sheet de Letra Lateral e Divisor Resizable -->
        <div class="flex-1 flex overflow-hidden min-h-0 w-full h-full">
          {#if viewingSong}
            <ResizablePaneGroup direction="horizontal" class="flex-1 w-full h-full min-h-0 overflow-hidden">
              <!-- Coluna do Catálogo de Músicas (Redimensionável) -->
              <ResizablePane defaultSize={35} minSize={20} maxSize={60} class="flex flex-col pr-3 overflow-y-auto min-h-0">
                <div class="grid grid-cols-1 gap-2">
                  {#each songs as song (song.id)}
                    {@render songCard(song)}
                  {/each}
                </div>
              </ResizablePane>

              <!-- Divisor Redimensionável Shadcn -->
              <ResizableHandle withHandle class="hover:bg-primary transition-colors cursor-col-resize" />

              <!-- Sheet Ampliado de Visualização de Letra (Redimensionável) -->
              <ResizablePane defaultSize={65} minSize={40} class="flex flex-col pl-3 min-w-0 min-h-0 overflow-hidden">
                {@render viewingSongSheet()}
              </ResizablePane>
            </ResizablePaneGroup>
          {:else}
            <!-- Catálogo em Tela Cheia (Quando nenhum preview de música estiver aberto) -->
            <div class="flex-1 overflow-y-auto flex flex-col pr-1 min-h-0">
              {#if songs.length === 0}
                <div class="h-32 flex flex-col items-center justify-center text-xs text-muted-foreground">
                  Nenhuma música encontrada no catálogo.
                </div>
              {:else}
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-2">
                  {#each songs as song (song.id)}
                    {@render songCard(song)}
                  {/each}
                </div>
              {/if}
            </div>
          {/if}
        </div>
      </div>

      <!-- ================= ABA 2: IMAGENS & PASTAS ================= -->
      {:else if activeTab === 'images'}
        <div role="tabpanel" id="panel-images" aria-labelledby="tab-images" class="flex-1 flex overflow-hidden gap-3">
          <!-- Tree Navigation Vertical para Pastas de Imagens -->
          <aside class="w-52 border-r border-border bg-card/40 flex flex-col shrink-0 p-2 overflow-y-auto space-y-1">
            <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-2 py-1 block">
              Navegação de Pastas
            </span>

            <!-- Nó: Todas as Imagens / Pastas Importadas -->
            <button
              type="button"
              onclick={() => {
                selectedFolderFilter = 'all';
                selectedSubfolder = null;
              }}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedFolderFilter === 'all' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2 truncate">
                <Layers size={13} />
                <span class="truncate">{showSystemImages ? 'Todas as Imagens' : 'Pastas Importadas'}</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {visibleImagesCount}
              </Badge>
            </button>

            <!-- Nó: Pasta do App (Sistema) -->
            <button
              type="button"
              onclick={() => {
                selectedFolderFilter = 'app';
                selectedSubfolder = null;
              }}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedFolderFilter === 'app' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2 truncate">
                <Folder size={13} class="text-primary" />
                <span class="truncate">Pasta do App (Sistema)</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {images.filter((i) => !i.is_external).length}
              </Badge>
            </button>

            <!-- Seção de Pastas Mapeadas -->
            <div class="pt-2">
              <button
                type="button"
                onclick={() => (isFoldersTreeExpanded = !isFoldersTreeExpanded)}
                class="w-full flex items-center justify-between px-2 py-1 text-[11px] font-bold text-muted-foreground hover:text-foreground cursor-pointer"
              >
                <span class="flex items-center gap-1">
                  {#if isFoldersTreeExpanded}
                    <ChevronDown size={12} />
                  {:else}
                    <ChevronRight size={12} />
                  {/if}
                  Pastas Mapeadas
                </span>
                <Badge variant="outline" class="text-[9px] py-0 px-1 font-mono">
                  {folders.length}
                </Badge>
              </button>

              {#if isFoldersTreeExpanded}
                <div class="space-y-0.5 mt-1 pl-1">
                  {#if folders.length === 0}
                    <p class="text-[10px] text-muted-foreground italic px-2 py-1">Nenhuma pasta mapeada</p>
                  {:else}
                    {#each folders as f (f.id)}
                      {@const count = images.filter((i) => i.folder_id === f.id).length}
                      {@const subs = f.subfolders || []}
                      {@const isExpanded = !!expandedFolders[f.id]}
                      {@const isFolderSelected = selectedFolderFilter === f.id}

                      <div class="space-y-0.5">
                        <div
                          role="button"
                          tabindex="0"
                          onclick={() => {
                            selectedFolderFilter = f.id;
                            selectedSubfolder = null;
                            if (subs.length > 0) expandedFolders[f.id] = true;
                          }}
                          onkeydown={(e) => {
                            if (e.key === 'Enter') {
                              selectedFolderFilter = f.id;
                              selectedSubfolder = null;
                            }
                          }}
                          class="w-full flex items-center justify-between px-2 py-1.5 rounded-md text-xs font-medium transition-colors cursor-pointer {isFolderSelected && selectedSubfolder === null ? 'bg-primary/15 text-primary font-bold' : isFolderSelected ? 'bg-muted/60 text-foreground font-semibold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
                          title={f.path}
                        >
                          <div class="flex items-center gap-1.5 min-w-0">
                            {#if subs.length > 0}
                              <button
                                type="button"
                                onclick={(e) => toggleFolderExpand(f.id, e)}
                                class="p-0.5 rounded hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer"
                                title={isExpanded ? 'Recolher sub-pastas' : 'Expandir sub-pastas'}
                              >
                                {#if isExpanded}
                                  <ChevronDown size={11} />
                                {:else}
                                  <ChevronRight size={11} />
                                {/if}
                              </button>
                            {:else}
                              <span class="w-3.5"></span>
                            {/if}
                            <FolderOpen size={12} class="text-amber-400 shrink-0" />
                            <span class="truncate text-[11px]">{f.name}</span>
                          </div>
                          <span class="text-[9px] font-mono text-muted-foreground ml-1 shrink-0">{count}</span>
                        </div>

                        {#if isExpanded && subs.length > 0}
                          <div class="pl-4 space-y-0.5 border-l border-border/50 ml-3">
                            {#each subs as sub (sub.relative_path)}
                              {@const isSubSelected = isFolderSelected && selectedSubfolder === sub.relative_path}
                              <button
                                type="button"
                                onclick={() => {
                                  selectedFolderFilter = f.id;
                                  selectedSubfolder = sub.relative_path;
                                }}
                                class="w-full flex items-center justify-between px-2 py-1 rounded text-[11px] transition-colors cursor-pointer {isSubSelected ? 'bg-primary/20 text-primary font-bold' : 'text-muted-foreground hover:bg-muted/60 hover:text-foreground'}"
                                title="{f.name} / {sub.relative_path}"
                              >
                                <div class="flex items-center gap-1.5 min-w-0">
                                  <Folder size={11} class="text-amber-400/80 shrink-0" />
                                  <span class="truncate">{sub.name}</span>
                                </div>
                                <span class="text-[9px] font-mono opacity-80 ml-1 shrink-0">{sub.count}</span>
                              </button>
                            {/each}
                          </div>
                        {/if}
                      </div>
                    {/each}
                  {/if}
                </div>
              {/if}
            </div>
          </aside>

          <!-- Área da Galeria de Imagens com Breadcrumbs no Topo -->
          <main class="flex-1 flex flex-col overflow-hidden">
            <!-- Barra de Breadcrumbs e Ações -->
            <div class="flex items-center justify-between gap-3 mb-2 shrink-0 pb-1.5 border-b border-border">
              <!-- Breadcrumbs -->
              <nav class="flex items-center gap-1.5 text-xs text-muted-foreground overflow-x-auto">
                <button
                  type="button"
                  onclick={() => {
                    selectedFolderFilter = 'all';
                    selectedSubfolder = null;
                  }}
                  class="hover:text-foreground transition-colors cursor-pointer"
                >
                  Mídias
                </button>
                <ChevronRight size={11} />
                <button
                  type="button"
                  onclick={() => {
                    selectedFolderFilter = 'all';
                    selectedSubfolder = null;
                  }}
                  class="hover:text-foreground transition-colors cursor-pointer"
                >
                  Imagens
                </button>
                {#if selectedFolderFilter !== 'all'}
                  <ChevronRight size={11} />
                  <button
                    type="button"
                    onclick={() => (selectedSubfolder = null)}
                    class="hover:text-foreground transition-colors cursor-pointer {selectedSubfolder ? 'text-muted-foreground hover:text-foreground' : 'text-foreground font-bold'}"
                  >
                    {currentFolderName}
                  </button>
                {/if}
                {#if selectedSubfolder}
                  <ChevronRight size={11} />
                  <span class="text-foreground font-bold truncate max-w-xs">
                    {selectedSubfolder}
                  </span>
                {/if}
                <Badge variant="secondary" class="text-[9px] py-0 px-1 ml-1 font-mono">
                  {filteredImages.length}
                </Badge>
              </nav>

              <!-- Ações de Upload e Gerenciamento -->
              <div class="flex items-center gap-2 shrink-0 flex-wrap">
                <!-- Filtro de Privacidade: Apenas Pastas Importadas vs Sistema -->
                <button
                  type="button"
                  onclick={toggleShowSystemImages}
                  class="px-2 py-1 rounded-lg text-[10px] font-semibold border transition-colors flex items-center gap-1.5 cursor-pointer {!showSystemImages ? 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30 hover:bg-emerald-500/25' : 'bg-muted/60 text-muted-foreground border-border hover:text-foreground'}"
                  title={!showSystemImages ? 'Exibindo apenas pastas importadas por segurança (fotos pessoais ocultas). Clique para incluir imagens do sistema.' : 'Exibindo todas as imagens incluindo do sistema. Clique para ocultar fotos do sistema.'}
                >
                  {#if !showSystemImages}
                    <ShieldCheck size={12} class="text-emerald-400" />
                    <span>Pastas Importadas</span>
                  {:else}
                    <Eye size={12} />
                    <span>Todas + Sistema</span>
                  {/if}
                </button>

                <!-- Controle Rápido de Margens e Encaixe de Imagens na Projeção -->
                <div class="flex items-center gap-1.5 px-2 py-0.5 bg-muted/40 border border-border rounded-lg text-xs">
                  <button
                    type="button"
                    onclick={handleToggleMediaMargins}
                    class="px-2 py-0.5 rounded text-[10px] font-semibold border transition-colors cursor-pointer {$publicSettingsStore.ignore_margins_for_media !== false ? 'bg-primary/20 text-primary border-primary/40' : 'bg-background text-muted-foreground border-border hover:text-foreground'}"
                    title="Alternar entre projeção sem margens (16:9 tela cheia) e com margens"
                  >
                    {$publicSettingsStore.ignore_margins_for_media !== false ? 'Sem Margens (16:9 Full)' : 'Com Margens'}
                  </button>
                  <div class="flex items-center gap-0.5 border-l border-border/60 pl-1">
                    {#each [
                      { id: 'contain', label: 'Ajustar' },
                      { id: 'cover', label: 'Preencher' },
                      { id: 'fill', label: 'Esticar' }
                    ] as f}
                      <button
                        type="button"
                        onclick={() => handleSetImageFit(f.id as ImageFitMode)}
                        class="px-1.5 py-0.5 rounded text-[10px] transition-colors cursor-pointer {($publicSettingsStore.image_fit || 'contain') === f.id ? 'bg-primary text-primary-foreground font-bold' : 'text-muted-foreground hover:text-foreground'}"
                      >
                        {f.label}
                      </button>
                    {/each}
                  </div>
                </div>

                <Button
                  variant="outline"
                  size="sm"
                  onclick={loadImages}
                  title="Recarregar imagens"
                  class="h-7 text-xs px-2"
                >
                  <RefreshCw size={12} class={isLoadingImages ? 'animate-spin' : ''} />
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={onopenfolderconfig}
                  class="h-7 text-xs gap-1"
                  title="Configurar pastas externas de mídia"
                >
                  <FolderPlus size={12} />
                  <span>Pastas</span>
                </Button>
                <Button
                  variant="default"
                  size="sm"
                  onclick={() => fileInputEl?.click()}
                  class="h-7 text-xs gap-1 font-semibold"
                >
                  <Upload size={12} />
                  <span>Importar Imagem</span>
                </Button>
              </div>
            </div>

            <!-- Grade de Miniaturas de Imagens com Ação de Apresentar Direta -->
            <div class="flex-1 overflow-y-auto pr-1">
              {#if selectedSubfolder === null && currentSubfolders.length > 0}
                <div class="mb-3.5 pb-3 border-b border-border/50">
                  <div class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground mb-1.5 flex items-center gap-1.5">
                    <Folder size={12} class="text-amber-400" />
                    <span>Sub-pastas ({currentSubfolders.length})</span>
                  </div>
                  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-2">
                    {#each currentSubfolders as sub (sub.relative_path)}
                      <button
                        type="button"
                        onclick={() => (selectedSubfolder = sub.relative_path)}
                        class="p-2 rounded-xl border border-border bg-card/60 hover:bg-neutral-850 hover:border-primary/50 text-left transition-all flex items-center gap-2.5 group cursor-pointer shadow-xs"
                      >
                        <div class="w-8 h-8 rounded-lg bg-amber-400/10 text-amber-400 flex items-center justify-center shrink-0 group-hover:scale-105 transition-transform">
                          <Folder size={16} />
                        </div>
                        <div class="min-w-0 flex-1">
                          <p class="text-xs font-semibold text-foreground truncate group-hover:text-primary transition-colors">
                            {sub.name}
                          </p>
                          <p class="text-[10px] text-muted-foreground font-mono">
                            {sub.count} {sub.count === 1 ? 'item' : 'itens'}
                          </p>
                        </div>
                      </button>
                    {/each}
                  </div>
                </div>
              {/if}

              {#if filteredImages.length === 0}
                <div class="h-32 flex flex-col items-center justify-center text-xs text-muted-foreground">
                  Nenhuma imagem encontrada nesta pasta.
                </div>
              {:else}
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-3">
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  {#each filteredImages as img (img.id)}
                    <div
                      ondblclick={() => onpresentimage?.(img)}
                      role="button"
                      tabindex="0"
                      onkeydown={(e) => { if (e.key === 'Enter') onpresentimage?.(img); }}
                      class="group relative rounded-lg border border-border bg-card overflow-hidden flex flex-col shadow-xs hover:border-primary/50 transition-all cursor-pointer"
                    >
                      <!-- Imagem Thumbnail -->
                      <div class="aspect-video w-full bg-black/60 relative overflow-hidden">
                        <img
                          src={img.url}
                          alt={img.name}
                          class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                          loading="lazy"
                        />

                        <!-- Overlay com nome e pasta -->
                        <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 via-black/40 to-transparent p-1 pt-3">
                          <p class="text-[10px] text-white font-medium truncate drop-shadow">{img.name}</p>
                          {#if img.subfolder && selectedSubfolder === null}
                            <p class="text-[9px] text-amber-300 font-mono truncate drop-shadow">{img.subfolder}</p>
                          {/if}
                        </div>
                      </div>

                      <!-- Barra de Ações Rápidas -->
                      <div class="p-1.5 bg-muted/30 border-t border-border flex items-center justify-between gap-1">
                        <button
                          type="button"
                          onclick={() => onpresentimage?.(img)}
                          class="flex-1 h-6 rounded bg-primary/20 hover:bg-primary/30 text-primary text-[10px] font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                          title="Apresentar Imagem Imediatamente (Sem adicionar à Setlist)"
                        >
                          <Play size={10} class="fill-current" />
                          <span>Apresentar</span>
                        </button>

                        <button
                          type="button"
                          onclick={() => onpreviewimage?.(img)}
                          class="w-6 h-6 rounded bg-muted hover:bg-muted/80 text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0"
                          title="Visualizar Prévia e Metadados"
                        >
                          <Eye size={11} />
                        </button>

                        <button
                          type="button"
                          onclick={() => onaddimagetosetlist?.(img)}
                          class="w-6 h-6 rounded bg-muted hover:bg-muted/80 text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0"
                          title="Adicionar à Setlist"
                        >
                          <ListPlus size={11} />
                        </button>
                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          </main>
        </div>

      <!-- ================= ABA 3: FUNDOS & GRADIENTES ================= -->
      {:else if activeTab === 'backgrounds'}
        <div role="tabpanel" id="panel-backgrounds" aria-labelledby="tab-backgrounds" class="flex-1 flex overflow-hidden gap-3">
          <!-- Tree Navigation Vertical para Categorias de Fundos -->
          <aside class="w-52 border-r border-border bg-card/40 flex flex-col shrink-0 p-2 overflow-y-auto space-y-1">
            <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-2 py-1 block">
              Tipos de Fundos
            </span>

            <button
              type="button"
              onclick={() => (selectedBgCategory = 'all')}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'all' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2">
                <Layers size={13} />
                <span>Todos</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {availableBackgrounds.length + localWallpapers.length}
              </Badge>
            </button>

            <button
              type="button"
              onclick={() => (selectedBgCategory = 'animated')}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'animated' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2">
                <Sparkles size={13} class="text-amber-400" />
                <span>Animados (CSS)</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {availableBackgrounds.filter((p) => p.category === 'animated').length}
              </Badge>
            </button>

            <button
              type="button"
              onclick={() => (selectedBgCategory = 'mesh')}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'mesh' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2">
                <Palette size={13} class="text-fuchsia-400" />
                <span>Mesh Gradients</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {availableBackgrounds.filter((p) => p.category === 'mesh').length}
              </Badge>
            </button>

            <button
              type="button"
              onclick={() => (selectedBgCategory = 'gradients')}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'gradients' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2">
                <Palette size={13} class="text-sky-400" />
                <span>Gradientes Lineares</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {availableBackgrounds.filter((p) => p.category === 'gradients').length}
              </Badge>
            </button>

            {#if availableBackgrounds.some((p) => p.category === 'custom')}
              <button
                type="button"
                onclick={() => (selectedBgCategory = 'custom' as any)}
                class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'custom' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
              >
                <div class="flex items-center gap-2">
                  <Sparkles size={13} class="text-emerald-400" />
                  <span>Customizados</span>
                </div>
                <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                  {availableBackgrounds.filter((p) => p.category === 'custom').length}
                </Badge>
              </button>
            {/if}

            <button
              type="button"
              onclick={() => (selectedBgCategory = 'local')}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'local' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2">
                <Folder size={13} class="text-emerald-400" />
                <span>Papéis Locais</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {localWallpapers.length}
              </Badge>
            </button>
          </aside>

          <!-- Área dos Cartões de Fundo com Breadcrumbs no Topo -->
          <main class="flex-1 flex flex-col overflow-hidden">
            <!-- Barra de Breadcrumbs e Filtros de Fundos -->
            <div class="flex items-center justify-between gap-3 mb-2 shrink-0 pb-1.5 border-b border-border">
              <!-- Breadcrumbs -->
              <nav class="flex items-center gap-1.5 text-xs text-muted-foreground">
                <span>Mídias</span>
                <ChevronRight size={11} />
                <span>Fundos</span>
                <ChevronRight size={11} />
                <span class="text-foreground font-bold capitalize">
                  {selectedBgCategory === 'all'
                    ? 'Todos os Fundos'
                    : selectedBgCategory === 'gradients'
                      ? 'Gradientes Lineares'
                      : selectedBgCategory === 'mesh'
                        ? 'Mesh Gradients'
                        : selectedBgCategory === 'animated'
                          ? 'Gradientes Animados'
                          : selectedBgCategory === 'custom'
                            ? 'Fundos Customizados'
                            : 'Papéis Locais'}
                </span>
              </nav>

              <!-- Ações da Barra: Busca e Importação de Fundos -->
              <div class="flex items-center gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onclick={triggerImportWallpaper}
                  disabled={isUploadingWallpaper}
                  class="h-6 text-xs gap-1.5 border-primary/40 text-primary hover:bg-primary/10 shrink-0 cursor-pointer"
                  title="Importar imagens ou vídeos de fundo"
                >
                  <Upload size={12} />
                  <span>{isUploadingWallpaper ? 'Importando...' : 'Importar Fundo'}</span>
                </Button>
                <input
                  type="file"
                  bind:this={wallpaperFileInput}
                  onchange={handleWallpaperUpload}
                  accept="image/png,image/jpeg,image/webp,image/svg+xml,video/mp4,video/webm,video/quicktime"
                  class="hidden"
                />

                <div class="relative w-56 sm:w-64">
                  <Search size={12} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
                  <input
                    type="text"
                    bind:value={bgSearchQuery}
                    placeholder="Filtrar fundos..."
                    class="w-full h-6 pl-7 pr-3 bg-neutral-900 border border-neutral-800 rounded-md text-[11px] text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
                  />
                </div>
              </div>
            </div>

            <!-- Grade de Cartões de Fundo 16:9 com Prévia Viva -->
            <div class="flex-1 overflow-y-auto pr-1">
              <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
                <!-- Presets em Código CSS -->
                {#if selectedBgCategory !== 'local'}
                  {#each filteredBackgrounds as preset (preset.id)}
                    {@const encoded = encodeBackground(preset)}
                    {@const isLiveBg = $presentationState.lyrics_background_path === encoded || ($presentationState.active_media_id && $presentationState.wallpaper_path === encoded)}

                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      onclick={() => applyBackground(encoded)}
                      role="button"
                      tabindex="0"
                      onkeydown={(e) => { if (e.key === 'Enter') applyBackground(encoded); }}
                      class="group relative rounded-xl border overflow-hidden flex flex-col transition-all cursor-pointer bg-card {isLiveBg ? 'border-primary ring-2 ring-primary shadow-lg' : 'border-border hover:border-primary/50'}"
                    >
                      <!-- Prévia 16:9 Viva com CSS e Animações Ativas -->
                      <div class="aspect-video w-full relative overflow-hidden flex items-center justify-center">
                        <div
                          class="absolute inset-0 w-full h-full {preset.animationClass || ''}"
                          style="background: {preset.style};"
                        ></div>

                        {#if isLiveBg}
                          <div class="absolute top-2 right-2 px-1.5 py-0.5 rounded-full bg-primary text-primary-foreground text-[9px] font-bold flex items-center gap-1 shadow-md">
                            <Check size={10} /> Fundo das Letras
                          </div>
                        {/if}

                        {#if preset.animationClass}
                          <span class="absolute bottom-1.5 left-1.5 px-1.5 py-0.5 rounded bg-black/60 text-amber-300 font-mono text-[9px] flex items-center gap-1">
                            <Sparkles size={9} /> Animado
                          </span>
                        {/if}
                      </div>

                      <!-- Barra de Ações Rápidas no Cartão -->
                      <div class="p-1.5 bg-muted/30 border-t border-border flex items-center justify-between gap-1">
                        <button
                          type="button"
                          onclick={(e) => {
                            e.stopPropagation();
                            applyBackground(encoded);
                          }}
                          class="flex-1 h-6 rounded bg-primary/20 hover:bg-primary/30 text-primary text-[10px] font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                          title="Definir como fundo de apresentação das letras"
                        >
                          <Play size={10} class="fill-current" />
                          <span>Usar nas Letras</span>
                        </button>
                      </div>
                    </div>
                  {/each}
                {/if}

                <!-- Papéis de Parede Locais da Pasta Wallpapers -->
                {#if selectedBgCategory === 'all' || selectedBgCategory === 'local'}
                  {#each localWallpapers as wall (wall.url)}
                    {@const isLiveBg = $presentationState.lyrics_background_path === wall.url || ($presentationState.active_media_id && $presentationState.wallpaper_path === wall.url)}
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      onclick={() => applyBackground(wall.url)}
                      role="button"
                      tabindex="0"
                      onkeydown={(e) => { if (e.key === 'Enter') applyBackground(wall.url); }}
                      class="group relative rounded-xl border overflow-hidden flex flex-col transition-all cursor-pointer bg-card {isLiveBg ? 'border-primary ring-2 ring-primary shadow-lg' : 'border-border hover:border-primary/50'}"
                    >
                      <div class="aspect-video w-full relative overflow-hidden bg-black/60 flex items-center justify-center">
                        {#if wall.is_video}
                          <video
                            src={wall.url}
                            class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                            autoplay
                            loop
                            muted
                            playsinline
                          ></video>
                          <div class="absolute top-2 left-2 px-1.5 py-0.5 rounded bg-black/75 border border-white/20 text-[9px] font-mono font-bold text-white flex items-center gap-1 shadow">
                            <Film size={10} class="text-primary" />
                            <span>Vídeo</span>
                          </div>
                        {:else}
                          <img
                            src={wall.url}
                            alt={wall.name}
                            class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                          />
                        {/if}

                        {#if isLiveBg}
                          <div class="absolute top-2 right-2 px-1.5 py-0.5 rounded-full bg-primary text-primary-foreground text-[9px] font-bold flex items-center gap-1 shadow-md">
                            <Check size={10} /> Fundo das Letras
                          </div>
                        {/if}

                        <span class="absolute bottom-1 inset-x-1 text-[9px] text-white truncate drop-shadow bg-black/60 px-1 rounded">
                          {wall.name}
                        </span>
                      </div>

                      <div class="p-1.5 bg-muted/30 border-t border-border flex items-center justify-between gap-1">
                        <button
                          type="button"
                          onclick={(e) => {
                            e.stopPropagation();
                            applyBackground(wall.url);
                          }}
                          class="flex-1 h-6 rounded bg-primary/20 hover:bg-primary/30 text-primary text-[10px] font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                          title="Definir como fundo de apresentação das letras"
                        >
                          <Play size={10} class="fill-current" />
                          <span>Usar nas Letras</span>
                        </button>
                      </div>
                    </div>
                  {/each}
                {/if}
              </div>
            </div>
          </main>
        </div>

      <!-- ================= ABA 4: BÍBLIA ================= -->
      {:else if activeTab === 'bible'}
        <div role="tabpanel" id="panel-bible" aria-labelledby="tab-bible" class="flex-1 flex flex-col min-w-0 overflow-hidden">
          <BiblePanel
            targetPassage={targetScriptureRef}
            {onpresentscripture}
            {onaddscripturetosetlist}
            {onopenbibleconfig}
          />
        </div>

      <!-- ================= ABA 5: OUTROS (AGENDA, TEXTOS, AVISOS, SEÇÕES) ================= -->
      {:else if activeTab === 'others'}
        <div role="tabpanel" id="panel-others" aria-labelledby="tab-others" class="flex-1 flex flex-col min-w-0 overflow-hidden">
          <!-- Barra de Ações Superior da Aba Outros -->
          <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-2.5 mb-2.5 shrink-0">
            <!-- Campo de Busca -->
            <div class="relative flex-1 max-w-md">
              <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-neutral-500" />
              <input
                type="text"
                bind:value={othersSearch}
                placeholder="Pesquisar em Outros (agenda, textos, avisos)..."
                class="w-full h-7 pl-8 pr-3 bg-neutral-900 border border-neutral-800 rounded-md text-xs text-neutral-200 placeholder-neutral-500 focus:outline-none focus:border-primary/60"
              />
            </div>

            <!-- Botões de Ação e Criação Rápida -->
            <div class="flex items-center gap-1.5 flex-wrap">
              <Button
                variant="outline"
                size="sm"
                onclick={loadCustomItems}
                title="Recarregar catálogo"
                class="h-7 text-xs px-2"
              >
                <RefreshCw size={12} />
              </Button>

              {#if activeSetlistItems && activeSetlistItems.length > 0}
                <Button
                  variant="outline"
                  size="sm"
                  onclick={handleImportFromSetlistClick}
                  disabled={isImportingFromSetlist}
                  title="Importar itens de agenda e texto da setlist ativa para o catálogo"
                  class="h-7 text-xs gap-1 cursor-pointer border-primary/40 text-primary hover:bg-primary/10"
                >
                  <Upload size={12} class={isImportingFromSetlist ? 'animate-spin' : ''} />
                  <span class="hidden md:inline">{isImportingFromSetlist ? 'Importando...' : 'Importar da Setlist'}</span>
                </Button>
              {/if}

              <!-- Botão Criar Agenda -->
              <Button
                variant="outline"
                size="sm"
                onclick={onopenscheduleeditor}
                title="Criar nova programação ou culto na Agenda"
                class="h-7 text-xs gap-1 border-amber-500/40 text-amber-300 hover:bg-amber-500/10 cursor-pointer"
              >
                <CalendarDays size={12} />
                <span>+ Agenda</span>
              </Button>

              <!-- Botão Criar Texto -->
              <Button
                variant="outline"
                size="sm"
                onclick={onopentexteditor}
                title="Criar nova apresentação de texto livre"
                class="h-7 text-xs gap-1 border-blue-500/40 text-blue-300 hover:bg-blue-500/10 cursor-pointer"
              >
                <FileText size={12} />
                <span>+ Texto</span>
              </Button>

              <!-- Botão Criar Aviso -->
              <Button
                variant="outline"
                size="sm"
                onclick={() => onopenquickitemeditor?.('announcement')}
                title="Criar novo aviso ou comunicado rápido"
                class="h-7 text-xs gap-1 border-emerald-500/40 text-emerald-300 hover:bg-emerald-500/10 cursor-pointer"
              >
                <Bookmark size={12} />
                <span>+ Aviso</span>
              </Button>
            </div>
          </div>

          <!-- Barra de Filtros de Categoria (Chips) -->
          <div class="flex items-center gap-1.5 mb-2.5 overflow-x-auto pb-1 text-xs shrink-0 select-none">
            {#each [
              { id: 'all', label: 'Todos', count: $customItemsStore.length },
              { id: 'schedule', label: '📅 Agenda', count: $customItemsStore.filter((i) => i.kind === 'schedule').length },
              { id: 'text', label: '📝 Textos', count: $customItemsStore.filter((i) => i.kind === 'text').length },
              { id: 'announcement', label: '📢 Avisos', count: $customItemsStore.filter((i) => i.kind === 'announcement').length },
              { id: 'header', label: '🔖 Seções', count: $customItemsStore.filter((i) => i.kind === 'header').length }
            ] as cat}
              <button
                type="button"
                onclick={() => (othersKindFilter = cat.id as any)}
                class="px-2.5 py-0.5 rounded-full text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 {othersKindFilter === cat.id ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'bg-muted hover:bg-muted/80 text-muted-foreground'}"
              >
                <span>{cat.label}</span>
                <span class="text-[9px] font-mono opacity-80">({cat.count})</span>
              </button>
            {/each}
          </div>

          <!-- Divisão Resizable: Lista de Cards (Esquerda) e Prévia dos Slides 16:9 (Direita) -->
          <div class="flex-1 min-h-0">
            <ResizablePaneGroup direction="horizontal" class="h-full rounded-xl border border-border overflow-hidden">
              <!-- Painel Esquerdo: Lista de Cards -->
              <ResizablePane defaultSize={35} minSize={25} maxSize={50} class="bg-card/40 flex flex-col min-w-0">
                <div class="flex-1 overflow-y-auto p-2 space-y-2">
                  {#if filteredCustomItems.length === 0}
                    <div class="h-full flex flex-col items-center justify-center text-center p-6 text-muted-foreground space-y-2">
                      <Boxes size={32} class="opacity-40" />
                      <p class="text-xs font-medium">Nenhum item encontrado nesta categoria.</p>
                      <div class="flex items-center gap-2 pt-2">
                        <Button variant="outline" size="sm" class="text-xs h-7" onclick={onopenscheduleeditor}>
                          + Criar Agenda
                        </Button>
                        <Button variant="outline" size="sm" class="text-xs h-7" onclick={onopentexteditor}>
                          + Criar Texto
                        </Button>
                      </div>
                    </div>
                  {:else}
                    {#each filteredCustomItems as item (item.id)}
                      {@const badge = getItemKindBadge(item.kind)}
                      {@const isSelected = viewingCustomItem?.id === item.id}
                      <div
                        role="button"
                        tabindex="0"
                        onclick={() => (viewingCustomItem = item)}
                        ondblclick={() => onpresentitem?.(item, 0)}
                        onkeydown={(e) => { if (e.key === 'Enter') viewingCustomItem = item; }}
                        class="p-2.5 rounded-lg border transition-all flex flex-col justify-between group cursor-pointer {isSelected ? 'ring-1 ring-primary border-primary bg-primary/10 shadow-xs' : 'border-border bg-card/60 hover:bg-muted/40'}"
                      >
                        <div>
                          <div class="flex items-center justify-between gap-1 mb-1">
                            <h4 class="text-xs font-bold text-foreground truncate">{item.title}</h4>
                            <span class="text-[9px] px-1.5 py-0.2 rounded font-semibold border shrink-0 {badge.color}">
                              {badge.label}
                            </span>
                          </div>

                          {#if item.kind === 'schedule'}
                            <p class="text-[10px] text-muted-foreground truncate">
                              {#if item.slides.length > 0}
                                {item.slides.map((s) => s.metadata?.schedule_day || s.label).filter(Boolean).slice(0, 3).join(' • ')}
                              {:else}
                                Programação de cultos
                              {/if}
                            </p>
                          {:else if item.slides[0]?.content}
                            <p class="text-[10px] text-muted-foreground line-clamp-2 mt-0.5 italic">
                              "{item.slides[0].content.slice(0, 90)}"
                            </p>
                          {/if}
                        </div>

                        <!-- Rodapé do Card com Ações Rápidas -->
                        <div class="flex items-center justify-between pt-2 mt-1.5 border-t border-border/50 text-[10px]">
                          <span class="font-mono text-muted-foreground">
                            {item.slides.length} {item.slides.length === 1 ? 'slide' : 'slides'}
                          </span>

                          <div class="flex items-center gap-1 opacity-90 group-hover:opacity-100 transition-opacity">
                            <button
                              type="button"
                              onclick={(e) => {
                                e.stopPropagation();
                                onadditemtosetlist?.(item);
                              }}
                              class="px-1.5 py-0.5 rounded hover:bg-primary/20 text-muted-foreground hover:text-primary flex items-center gap-1 transition-colors cursor-pointer"
                              title="Adicionar à Setlist"
                            >
                              <ListPlus size={11} class="text-primary" />
                              <span class="hidden sm:inline">+ Setlist</span>
                            </button>

                            <button
                              type="button"
                              onclick={(e) => {
                                e.stopPropagation();
                                onpresentitem?.(item, 0);
                              }}
                              class="px-1.5 py-0.5 rounded bg-primary/20 hover:bg-primary/30 text-primary font-bold flex items-center gap-1 transition-colors cursor-pointer"
                              title="Apresentar Imediatamente"
                            >
                              <Play size={10} class="fill-current" />
                              <span class="hidden sm:inline">Ao Vivo</span>
                            </button>

                            <button
                              type="button"
                              onclick={(e) => {
                                e.stopPropagation();
                                onopenedititem?.(item);
                              }}
                              class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                              title="Editar este item"
                            >
                              <Pencil size={11} />
                            </button>

                            <button
                              type="button"
                              onclick={(e) => handleDeleteCustomItemClick(item, e)}
                              class="p-1 rounded hover:bg-destructive/20 text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
                              title="Excluir do catálogo"
                            >
                              <Trash2 size={11} />
                            </button>
                          </div>
                        </div>
                      </div>
                    {/each}
                  {/if}
                </div>
              </ResizablePane>

              <!-- Divisor Resizable -->
              <ResizableHandle withHandle />

              <!-- Painel Direito: Prévia 16:9 dos Slides -->
              <ResizablePane defaultSize={65} minSize={40} class="bg-card/20 flex flex-col min-w-0">
                {#if viewingCustomItem}
                  {@const currentBadge = getItemKindBadge(viewingCustomItem.kind)}
                  <div class="flex-1 flex flex-col p-3 overflow-hidden">
                    <!-- Cabeçalho da Prévia -->
                    <div class="flex items-center justify-between pb-2.5 border-b border-border shrink-0">
                      <div class="min-w-0 mr-3">
                        <div class="flex items-center gap-2">
                          <h4 class="text-sm font-bold text-foreground truncate">{viewingCustomItem.title}</h4>
                          <span class="text-[9px] px-2 py-0.5 rounded font-semibold border shrink-0 {currentBadge.color}">
                            {currentBadge.label}
                          </span>
                          <Badge variant="secondary" class="text-[10px] font-mono">
                            {viewingCustomItem.slides.length} slides
                          </Badge>
                        </div>
                        {#if viewingCustomItem.artist_or_ref}
                          <p class="text-[11px] text-muted-foreground truncate mt-0.5">{viewingCustomItem.artist_or_ref}</p>
                        {/if}
                      </div>

                      <div class="flex items-center gap-1.5">
                        <Button
                          variant="secondary"
                          size="sm"
                          onclick={() => viewingCustomItem && onadditemtosetlist?.(viewingCustomItem)}
                          class="h-7 text-xs gap-1"
                        >
                          <ListPlus size={12} class="text-primary" />
                          <span>+ Setlist</span>
                        </Button>
                        <Button
                          variant="live"
                          size="sm"
                          onclick={() => viewingCustomItem && onpresentitem?.(viewingCustomItem, 0)}
                          class="h-7 text-xs gap-1 font-bold"
                        >
                          <Play size={12} class="fill-current" />
                          <span>Apresentar</span>
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onclick={() => viewingCustomItem && onopenedititem?.(viewingCustomItem)}
                          class="h-7 text-xs gap-1"
                        >
                          <Pencil size={12} />
                          <span>Editar</span>
                        </Button>
                      </div>
                    </div>

                    <!-- Grade Responsiva de Slides 16:9 -->
                    <div class="flex-1 overflow-y-auto py-3 pr-1 min-h-0">
                      {#if viewingCustomItem.slides.length > 0}
                        <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3">
                          {#each viewingCustomItem.slides as slide, idx (slide.id || idx)}
                            <div
                              role="button"
                              tabindex="0"
                              ondblclick={() => viewingCustomItem && onpresentitem?.(viewingCustomItem, idx)}
                              onkeydown={(e) => { if (e.key === 'Enter') viewingCustomItem && onpresentitem?.(viewingCustomItem, idx); }}
                              class="group rounded-xl border border-border bg-neutral-950 flex flex-col overflow-hidden shadow-xs hover:border-primary/60 transition-all cursor-pointer relative"
                            >
                              <!-- Barra superior do slide -->
                              <div class="h-6 px-2.5 bg-neutral-900/80 border-b border-border/50 flex items-center justify-between shrink-0">
                                <span class="text-[10px] font-bold text-neutral-300 font-mono">
                                  {slide.label || `Slide ${idx + 1}`}
                                </span>
                                <button
                                  type="button"
                                  onclick={(e) => {
                                    e.stopPropagation();
                                    viewingCustomItem && onpresentitem?.(viewingCustomItem, idx);
                                  }}
                                  class="text-[9px] text-primary hover:underline flex items-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity"
                                >
                                  <Play size={9} class="fill-current" />
                                  <span>Ao Vivo</span>
                                </button>
                              </div>

                              <!-- Área 16:9 de Conteúdo do Slide -->
                              <div class="aspect-video w-full p-3 flex flex-col justify-center items-center text-center select-none bg-neutral-950/90 relative overflow-hidden">
                                {#if viewingCustomItem.kind === 'schedule'}
                                  <!-- Card de Agenda em miniatura -->
                                  <div class="w-full flex flex-col items-center justify-center p-2 rounded-lg bg-white/5 border border-white/10">
                                    {#if slide.metadata?.schedule_day || slide.metadata?.schedule_time}
                                      <div class="flex items-center gap-1 mb-1">
                                        {#if slide.metadata?.schedule_day}
                                          <span class="px-1.5 py-0.2 rounded text-[8px] font-bold uppercase bg-amber-400/20 text-amber-300 border border-amber-400/30">
                                            {slide.metadata.schedule_day}
                                          </span>
                                        {/if}
                                        {#if slide.metadata?.schedule_time}
                                          <span class="px-1.5 py-0.2 rounded text-[8px] font-bold font-mono bg-cyan-400/20 text-cyan-200 border border-cyan-400/30">
                                            {slide.metadata.schedule_time}
                                          </span>
                                        {/if}
                                      </div>
                                    {/if}
                                    <h5 class="text-xs font-bold text-white line-clamp-2 leading-tight">
                                      {slide.label || slide.content}
                                    </h5>
                                    {#if slide.metadata?.schedule_details}
                                      <p class="text-[9px] text-neutral-400 line-clamp-1 mt-1">
                                        {slide.metadata.schedule_details}
                                      </p>
                                    {/if}
                                  </div>
                                {:else if viewingCustomItem.kind === 'header'}
                                  <div class="px-3 py-1 rounded bg-cyan-500/10 border border-cyan-500/30 text-cyan-200 font-bold text-xs uppercase tracking-wider">
                                    {slide.content || viewingCustomItem.title}
                                  </div>
                                {:else}
                                  <!-- Conteúdo de Texto ou Aviso -->
                                  <p class="text-xs text-white/90 font-medium leading-relaxed line-clamp-4 whitespace-pre-wrap">
                                    {slide.content}
                                  </p>
                                {/if}
                              </div>
                            </div>
                          {/each}
                        </div>
                      {:else}
                        <div class="h-full flex items-center justify-center text-xs text-muted-foreground">
                          Este item não possui slides cadastrados.
                        </div>
                      {/if}
                    </div>
                  </div>
                {:else}
                  <div class="flex-1 flex flex-col items-center justify-center text-center p-6 text-muted-foreground space-y-2">
                    <Boxes size={36} class="opacity-30" />
                    <p class="text-xs font-medium">Selecione um item à esquerda para visualizar seus slides em 16:9.</p>
                  </div>
                {/if}
              </ResizablePane>
            </ResizablePaneGroup>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

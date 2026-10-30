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
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import BiblePanel from './BiblePanel.svelte';
  import { renderFormattedText } from '../markdown';
  import {
    BACKGROUND_PRESETS,
    encodeBackground,
    resolveBackground,
    type BackgroundPreset,
  } from '../backgrounds';
  import { presentationState, setPresentationWallpaper } from '../ws';
  import { publicSettingsStore, savePublicDisplaySettings } from '../displaySettings';
  import { refreshBibles } from '../bibles';
  import type {
    SongSummary,
    Song,
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
    requestedTab?: 'songs' | 'images' | 'backgrounds' | 'bible' | null;
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
    requestedTab = null,
    targetScriptureRef = null,
  }: Props = $props();

  let isCollapsed = $state(true);
  let isMaximized = $state(false);
  let activeTab = $state<'songs' | 'images' | 'backgrounds' | 'bible'>('songs');
  let panelHeight = $state(290);
  let isDragging = $state(false);
  let startY = 0;
  let startHeight = 290;

  $effect(() => {
    if (requestedTab) {
      activeTab = requestedTab;
      isCollapsed = false;
    }
  });

  $effect(() => {
    if (activeTab === 'bible') {
      refreshBibles();
    }
  });

  // --- CATÁLOGO DE MÚSICAS ---
  let songSearch = $state('');
  let songs = $state<SongSummary[]>([]);
  let isLoadingSongs = $state(false);
  let viewingSong = $state<Song | null>(null);
  let isLoadingLyrics = $state(false);

  // --- GALERIA DE IMAGENS ---
  let images = $state<ImageMediaItem[]>([]);
  let folders = $state<MediaFolder[]>([]);
  let selectedFolderFilter = $state<string>('all');
  let isFoldersTreeExpanded = $state(true);
  let isLoadingImages = $state(false);
  let fileInputEl: HTMLInputElement | null = $state(null);

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
  let selectedBgCategory = $state<'all' | 'gradients' | 'mesh' | 'animated' | 'local'>('all');
  let bgSearchQuery = $state('');
  let localWallpapers = $state<WallpaperItem[]>([]);
  let isLoadingWallpapers = $state(false);

  onMount(() => {
    loadSongs();
    loadImages();
    loadFolders();
    loadWallpapers();
  });

  async function loadSongs() {
    isLoadingSongs = true;
    try {
      const q = songSearch.trim();
      const url = q ? `/api/songs?q=${encodeURIComponent(q)}` : '/api/songs';
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

  // Filtragem de imagens com suporte a sub-pastas
  const filteredImages = $derived.by(() => {
    let list = images;
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
    if (selectedFolderFilter === 'all') return 'Todas as Imagens';
    if (selectedFolderFilter === 'app') return 'Pasta do App';
    const f = folders.find((item) => item.id === selectedFolderFilter);
    return f ? f.name : 'Pasta Selecionada';
  });

  // Filtragem de fundos
  const filteredBackgrounds = $derived.by(() => {
    return BACKGROUND_PRESETS.filter((p) => {
      const matchesCat = selectedBgCategory === 'all' || p.category === selectedBgCategory;
      const matchesSearch =
        bgSearchQuery === '' ||
        p.name.toLowerCase().includes(bgSearchQuery.toLowerCase()) ||
        (p.description && p.description.toLowerCase().includes(bgSearchQuery.toLowerCase()));
      return matchesCat && matchesSearch;
    });
  });

  // Aplicação instantânea de fundo no projetor
  function applyBackground(encodedOrUrl: string) {
    setPresentationWallpaper(encodedOrUrl, 'cover');
  }

  // Arraste para redimensionamento de altura
  function startResize(e: MouseEvent) {
    if (isMaximized) return;
    isDragging = true;
    startY = e.clientY;
    startHeight = panelHeight;

    window.addEventListener('mousemove', handleResizeMove);
    window.addEventListener('mouseup', stopResize);
  }

  function handleResizeMove(e: MouseEvent) {
    if (!isDragging) return;
    const delta = startY - e.clientY;
    const newHeight = Math.min(Math.max(180, startHeight + delta), window.innerHeight - 80);
    panelHeight = newHeight;
    if (isCollapsed) isCollapsed = false;
  }

  function stopResize() {
    isDragging = false;
    window.removeEventListener('mousemove', handleResizeMove);
    window.removeEventListener('mouseup', stopResize);
  }

  function toggleCollapse() {
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
  class="border-t border-neutral-800 bg-neutral-900/95 backdrop-blur-md flex flex-col shrink-0 select-none transition-[height] duration-150 relative z-30"
  style={isCollapsed
    ? 'height: 40px;'
    : isMaximized
      ? 'height: calc(100vh - 56px);'
      : `height: ${panelHeight}px;`}
>
  <!-- Handle de Redimensionamento Superior -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="h-1.5 w-full cursor-row-resize bg-neutral-800/40 hover:bg-primary/50 transition-colors absolute top-0 left-0 right-0 z-40"
    onmousedown={startResize}
    title="Arrastar para ajustar altura"
  ></div>

  <!-- Barra de Abas e Controles -->
  <div
    class="h-10 px-3 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-950/60"
  >
    <!-- Abas Disponíveis -->
    <div class="flex items-center gap-1">
      <!-- Aba Músicas -->
      <button
        type="button"
        onclick={() => {
          activeTab = 'songs';
          if (isCollapsed) isCollapsed = false;
        }}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {activeTab === 'songs' && !isCollapsed ? 'bg-neutral-800 text-white shadow-xs' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-850'}"
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
        onclick={() => {
          activeTab = 'images';
          if (isCollapsed) isCollapsed = false;
        }}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {activeTab === 'images' && !isCollapsed ? 'bg-neutral-800 text-white shadow-xs' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-850'}"
      >
        <ImageIcon size={13} class="text-cyan-400" />
        <span>Imagens & Pastas</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {images.length}
        </Badge>
      </button>

      <!-- Aba Fundos & Gradientes -->
      <button
        type="button"
        onclick={() => {
          activeTab = 'backgrounds';
          if (isCollapsed) isCollapsed = false;
        }}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {activeTab === 'backgrounds' && !isCollapsed ? 'bg-neutral-800 text-white shadow-xs' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-850'}"
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
        onclick={() => {
          activeTab = 'bible';
          if (isCollapsed) isCollapsed = false;
        }}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {activeTab === 'bible' && !isCollapsed ? 'bg-neutral-800 text-white shadow-xs' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-850'}"
      >
        <BookOpen size={13} class="text-amber-400" />
        <span>Bíblia</span>
      </button>
    </div>

    <!-- Controles de Janela do Painel Inferior -->
    <div class="flex items-center gap-1 text-neutral-400">
      <button
        type="button"
        onclick={toggleMaximize}
        title={isMaximized ? 'Restaurar Altura' : 'Expandir 100%'}
        class="w-7 h-7 rounded hover:bg-neutral-800 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
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
        title={isCollapsed ? 'Abrir Painel' : 'Recolher Painel'}
        class="w-7 h-7 rounded hover:bg-neutral-800 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
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
    <div class="flex-1 flex flex-col overflow-hidden bg-neutral-950/40 p-3">
      <!-- ================= ABA 1: MÚSICAS ================= -->
      {#if activeTab === 'songs'}
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
            <Button
              variant="outline"
              size="sm"
              onclick={loadSongs}
              title="Recarregar catálogo"
              class="h-7 text-xs px-2"
            >
              <RefreshCw size={12} class={isLoadingSongs ? 'animate-spin' : ''} />
            </Button>
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

        <!-- Grade/Lista de Músicas com Suporte a Sheet de Letra Lateral -->
        <div class="flex-1 flex overflow-hidden gap-3">
          <!-- Coluna do Catálogo de Músicas: compacta quando o preview está aberto, ampla quando fechado -->
          <div class="{viewingSong ? 'w-80 sm:w-96 shrink-0 border-r border-border pr-2' : 'flex-1 pr-1'} overflow-y-auto flex flex-col">
            {#if songs.length === 0}
              <div class="h-32 flex flex-col items-center justify-center text-xs text-muted-foreground">
                Nenhuma música encontrada no catálogo.
              </div>
            {:else}
              <div class="grid {viewingSong ? 'grid-cols-1' : 'grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5'} gap-2">
                {#each songs as song (song.id)}
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
                {/each}
              </div>
            {/if}
          </div>

          <!-- Sheet Ampliado de Visualização de Letra (Preenche o espaço restante confortavelmente) -->
          {#if viewingSong}
            <div class="flex-1 min-w-0 border border-border bg-card/60 rounded-xl flex flex-col p-3.5 h-full overflow-hidden shadow-md animate-in slide-in-from-right duration-150">
              <!-- Cabeçalho do Sheet -->
              <div class="flex items-center justify-between pb-2.5 border-b border-border shrink-0">
                <div class="min-w-0 mr-3">
                  <div class="flex items-center gap-2">
                    <h4 class="text-sm font-bold text-foreground truncate">{viewingSong.title}</h4>
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
                  {#if viewingSong.artist}
                    <p class="text-xs text-muted-foreground truncate mt-0.5">{viewingSong.artist}</p>
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

              <!-- Prévia dos Slides em DUAS COLUNAS (Aspect Ratio 16:9) -->
              <div class="flex-1 overflow-y-auto py-3 pr-1">
                {#if viewingSong.sections && viewingSong.sections.length > 0}
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                    {#each viewingSong.sections as sec, idx}
                      <div class="rounded-xl border border-border bg-background/60 overflow-hidden flex flex-col shadow-xs">
                        <!-- Topo da Seção -->
                        <div class="px-3 py-1.5 bg-muted/40 border-b border-border flex items-center justify-between">
                          <span class="text-[10px] uppercase font-bold text-primary flex items-center gap-1.5">
                            <span class="w-4 h-4 rounded-full bg-primary/20 text-primary flex items-center justify-center text-[9px] font-mono font-bold">
                              {idx + 1}
                            </span>
                            {sec.label || `Parte ${idx + 1}`}
                          </span>
                          <span class="text-[9px] text-muted-foreground font-mono">16:9</span>
                        </div>

                        <!-- Conteúdo dos Slides em proporção 16:9 -->
                        <div class="aspect-video w-full p-3.5 flex flex-col items-center justify-center text-center bg-black/40">
                          <div class="text-xs sm:text-[13px] font-semibold text-foreground leading-relaxed">
                            {#each sec.lines as line}
                              <p>{@html renderFormattedText(line)}</p>
                            {/each}
                          </div>
                        </div>
                      </div>
                    {/each}
                  </div>
                {:else}
                  <div class="p-4 rounded-xl border border-border bg-background/50 text-xs font-medium text-foreground leading-relaxed whitespace-pre-wrap">
                    {@html renderFormattedText(viewingSong.lyrics_raw)}
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        </div>

      <!-- ================= ABA 2: IMAGENS & PASTAS ================= -->
      {:else if activeTab === 'images'}
        <div class="flex-1 flex overflow-hidden gap-3">
          <!-- Tree Navigation Vertical para Pastas de Imagens -->
          <aside class="w-52 border-r border-border bg-card/40 flex flex-col shrink-0 p-2 overflow-y-auto space-y-1">
            <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-2 py-1 block">
              Navegação de Pastas
            </span>

            <!-- Nó: Todas as Imagens -->
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
                <span class="truncate">Todas as Imagens</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {images.length}
              </Badge>
            </button>

            <!-- Nó: Pasta do App -->
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
                <span class="truncate">Pasta do App</span>
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
        <div class="flex-1 flex overflow-hidden gap-3">
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
                {BACKGROUND_PRESETS.length + localWallpapers.length}
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
                {BACKGROUND_PRESETS.filter((p) => p.category === 'animated').length}
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
                {BACKGROUND_PRESETS.filter((p) => p.category === 'mesh').length}
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
                {BACKGROUND_PRESETS.filter((p) => p.category === 'gradients').length}
              </Badge>
            </button>

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
                          : 'Papéis Locais'}
                </span>
              </nav>

              <!-- Busca Rápida de Fundos -->
              <div class="relative w-64">
                <Search size={12} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
                <input
                  type="text"
                  bind:value={bgSearchQuery}
                  placeholder="Filtrar fundos..."
                  class="w-full h-6 pl-7 pr-3 bg-neutral-900 border border-neutral-800 rounded-md text-[11px] text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
                />
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
                        <img
                          src={wall.url}
                          alt={wall.name}
                          class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                        />

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
        <BiblePanel
          targetPassage={targetScriptureRef}
          {onpresentscripture}
          {onaddscripturetosetlist}
          {onopenbibleconfig}
        />
      {/if}
    </div>
  {/if}
</div>

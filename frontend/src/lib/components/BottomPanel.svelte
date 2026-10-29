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
    ExternalLink,
    Folder,
    RefreshCw,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import type { SongSummary, ImageMediaItem, MediaFolder, MediaItem } from '../types';

  interface Props {
    onaddsongtosetlist?: (songId: string) => void;
    onpresentsong?: (songId: string) => void;
    onpresentimage?: (image: ImageMediaItem) => void;
    onaddimagetosetlist?: (image: ImageMediaItem) => void;
    onopensongeditor?: () => void;
    onopenfolderconfig?: () => void;
  }

  let {
    onaddsongtosetlist,
    onpresentsong,
    onpresentimage,
    onaddimagetosetlist,
    onopensongeditor,
    onopenfolderconfig,
  }: Props = $props();

  let isCollapsed = $state(true);
  let isMaximized = $state(false);
  let activeTab = $state<'songs' | 'images'>('songs');
  let panelHeight = $state(280);
  let isDragging = $state(false);
  let startY = 0;
  let startHeight = 280;

  // Catálogo de Músicas
  let songSearch = $state('');
  let songs = $state<SongSummary[]>([]);
  let isLoadingSongs = $state(false);

  // Galeria de Imagens
  let images = $state<ImageMediaItem[]>([]);
  let folders = $state<MediaFolder[]>([]);
  let selectedFolderFilter = $state<string>('all');
  let isLoadingImages = $state(false);
  let fileInputEl: HTMLInputElement | null = $state(null);

  onMount(() => {
    loadSongs();
    loadImages();
    loadFolders();
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

  // Filtragem de imagens
  const filteredImages = $derived.by(() => {
    if (selectedFolderFilter === 'all') return images;
    if (selectedFolderFilter === 'app') return images.filter((img) => !img.is_external);
    return images.filter((img) => img.folder_id === selectedFolderFilter);
  });

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
    const newHeight = Math.min(Math.max(160, startHeight + delta), window.innerHeight - 80);
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

<!-- Input Oculto de Arquivo -->
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
      <button
        type="button"
        onclick={() => {
          activeTab = 'songs';
          if (isCollapsed) isCollapsed = false;
        }}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {activeTab === 'songs' && !isCollapsed ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-850'}"
      >
        <Music size={13} class="text-primary" />
        <span>Músicas</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {songs.length}
        </Badge>
      </button>

      <button
        type="button"
        onclick={() => {
          activeTab = 'images';
          if (isCollapsed) isCollapsed = false;
        }}
        class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {activeTab === 'images' && !isCollapsed ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-850'}"
      >
        <ImageIcon size={13} class="text-cyan-400" />
        <span>Imagens & Mídias</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {images.length}
        </Badge>
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
      <!-- ABA: MÚSICAS -->
      {#if activeTab === 'songs'}
        <!-- Barra de Ações da Aba Músicas -->
        <div class="flex items-center justify-between gap-3 mb-2 shrink-0">
          <div class="relative flex-1 max-w-md">
            <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-neutral-500" />
            <input
              type="text"
              bind:value={songSearch}
              oninput={loadSongs}
              placeholder="Pesquisar catálogo de músicas..."
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

        <!-- Grade/Lista de Músicas -->
        <div class="flex-1 overflow-y-auto pr-1">
          {#if songs.length === 0}
            <div class="h-32 flex flex-col items-center justify-center text-xs text-neutral-500">
              Nenhuma música encontrada no catálogo.
            </div>
          {:else}
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
              {#each songs as song (song.id)}
                <div
                  class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/60 hover:bg-neutral-850 hover:border-neutral-700 transition-colors flex flex-col justify-between group"
                >
                  <div>
                    <div class="flex items-center justify-between gap-1 mb-1">
                      <h4 class="text-xs font-bold text-white truncate">{song.title}</h4>
                      {#if song.song_key}
                        <Badge variant="outline" class="text-[9px] py-0 px-1 font-mono text-emerald-400">
                          {song.song_key}
                        </Badge>
                      {/if}
                    </div>
                    {#if song.artist}
                      <p class="text-[11px] text-neutral-400 truncate">{song.artist}</p>
                    {/if}
                    {#if song.preview_snippet}
                      <p class="text-[10px] text-neutral-500 line-clamp-2 mt-1 italic">
                        "{song.preview_snippet}"
                      </p>
                    {/if}
                  </div>

                  <div class="flex items-center justify-end gap-1.5 mt-2 pt-2 border-t border-neutral-800/60">
                    <button
                      type="button"
                      onclick={() => onaddsongtosetlist?.(song.id)}
                      class="px-2 py-0.5 rounded bg-neutral-800 hover:bg-neutral-700 text-[10px] text-neutral-200 font-medium flex items-center gap-1 transition-colors cursor-pointer"
                      title="Adicionar à Setlist"
                    >
                      <ListPlus size={11} class="text-primary" />
                      + Setlist
                    </button>

                    <button
                      type="button"
                      onclick={() => onpresentsong?.(song.id)}
                      class="px-2 py-0.5 rounded bg-primary/20 hover:bg-primary/30 text-[10px] text-primary font-medium flex items-center gap-1 transition-colors cursor-pointer"
                      title="Apresentar Imediatamente"
                    >
                      <Play size={11} class="fill-current" />
                      Ao Vivo
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>

      <!-- ABA: IMAGENS -->
      {:else if activeTab === 'images'}
        <!-- Barra de Ações da Aba Imagens -->
        <div class="flex items-center justify-between gap-3 mb-2 shrink-0">
          <!-- Filtro de Pastas -->
          <div class="flex items-center gap-1.5 overflow-x-auto text-xs">
            <span class="text-[11px] text-neutral-500 font-medium flex items-center gap-1">
              <Folder size={12} /> Pasta:
            </span>
            <button
              type="button"
              onclick={() => (selectedFolderFilter = 'all')}
              class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {selectedFolderFilter === 'all' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-850'}"
            >
              Todas ({images.length})
            </button>
            <button
              type="button"
              onclick={() => (selectedFolderFilter = 'app')}
              class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {selectedFolderFilter === 'app' ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-850'}"
            >
              Pasta do App
            </button>
            {#each folders as f (f.id)}
              <button
                type="button"
                onclick={() => (selectedFolderFilter = f.id)}
                class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {selectedFolderFilter === f.id ? 'bg-neutral-800 text-white' : 'text-neutral-400 hover:bg-neutral-850'}"
              >
                {f.name}
              </button>
            {/each}
          </div>

          <!-- Ações de Upload e Gerenciamento -->
          <div class="flex items-center gap-2">
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
              class="h-7 text-xs gap-1"
            >
              <Upload size={12} />
              <span>Importar Imagem</span>
            </Button>
          </div>
        </div>

        <!-- Grade de Miniaturas de Imagens -->
        <div class="flex-1 overflow-y-auto pr-1">
          {#if filteredImages.length === 0}
            <div class="h-32 flex flex-col items-center justify-center text-xs text-neutral-500">
              Nenhuma imagem encontrada nesta pasta.
            </div>
          {:else}
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-3">
              {#each filteredImages as img (img.id)}
                <div
                  class="group relative aspect-video rounded-lg border border-neutral-800 bg-neutral-900 overflow-hidden flex flex-col shadow-sm hover:border-neutral-700 transition-all"
                >
                  <!-- Imagem Thumbnail -->
                  <img
                    src={img.url}
                    alt={img.name}
                    class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                    loading="lazy"
                  />

                  <!-- Overlay com Ações ao Hover -->
                  <div
                    class="absolute inset-0 bg-black/75 opacity-0 group-hover:opacity-100 flex flex-col justify-between p-2 transition-opacity"
                  >
                    <div class="text-[10px] text-white font-medium truncate drop-shadow">
                      {img.name}
                    </div>

                    <div class="flex items-center justify-center gap-1.5">
                      <button
                        type="button"
                        onclick={() => onpresentimage?.(img)}
                        class="p-1.5 rounded-full bg-primary text-black hover:scale-110 transition-transform cursor-pointer"
                        title="Apresentar Imagem ao Vivo"
                      >
                        <Play size={12} class="fill-current" />
                      </button>
                      <button
                        type="button"
                        onclick={() => onaddimagetosetlist?.(img)}
                        class="p-1.5 rounded-full bg-neutral-800 text-white hover:scale-110 transition-transform cursor-pointer"
                        title="Adicionar à Setlist"
                      >
                        <ListPlus size={12} />
                      </button>
                    </div>

                    <div class="text-[9px] text-neutral-400 truncate">
                      {img.folder_name}
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</div>

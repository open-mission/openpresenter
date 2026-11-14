<script lang="ts">
  import {
    Search,
    Play,
    ListPlus,
    Upload,
    FolderPlus,
    Folder,
    RefreshCw,
    X,
    ChevronUp,
    ChevronRight,
    Layers,
    FolderOpen,
    Film,
    AlertTriangle,
  } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import Badge from '../ui/Badge.svelte';
  import {
    fetchAppSubfolders,
    fetchVideos,
    formatBytes,
    readFileAsBase64,
    uploadVideo,
  } from '../../media';
  import { notifyError, notifySuccess } from '../../toast';
  import type { MediaFolder, MediaItem, MediaSubfolder, VideoMediaItem } from '../../types';
  import { formatFolderBreadcrumbs, getDirectSubfoldersForLevel } from './folderTree';

  interface OpenCreateFolderPayload {
    kind: 'images' | 'videos';
    folderId: string;
    parentSubfolder: string | null;
    locationLabel: string;
    oncreated: (folderId: string) => Promise<void> | void;
  }

  interface Props {
    active: boolean;
    folders: MediaFolder[];
    onpresentitem?: (item: MediaItem, slideIndex?: number) => void;
    onadditemtosetlist?: (item: MediaItem) => void;
    onopenfolderconfig?: () => void;
    onopencreatefolder: (payload: OpenCreateFolderPayload) => void;
    oncountchange?: (count: number) => void;
  }

  let {
    active,
    folders,
    onpresentitem,
    onadditemtosetlist,
    onopenfolderconfig,
    onopencreatefolder,
    oncountchange,
  }: Props = $props();

  let videos = $state<VideoMediaItem[]>([]);
  let appVideoSubfolders = $state<MediaSubfolder[]>([]);
  let videoSearch = $state('');
  let videoFolderFilter = $state<string>('all');
  let selectedVideoSubfolder = $state<string | null>(null);
  let isLoadingVideos = $state(false);
  let videosError = $state<string | null>(null);
  let isUploadingVideo = $state(false);
  let videoFileInputEl: HTMLInputElement | null = $state(null);
  let hoveredVideoId = $state<string | null>(null);
  let hasLoadedOnce = false;

  $effect(() => {
    oncountchange?.(videos.length);
  });

  $effect(() => {
    if (active && !hasLoadedOnce) {
      hasLoadedOnce = true;
      loadVideos();
    }
  });

  async function loadAppVideoSubfolders() {
    try {
      appVideoSubfolders = await fetchAppSubfolders('videos');
    } catch (err) {
      console.warn('Erro ao carregar subpastas de vídeos:', err);
    }
  }

  export async function loadVideos() {
    isLoadingVideos = true;
    videosError = null;
    try {
      videos = await fetchVideos();
      await loadAppVideoSubfolders();
    } catch (err: any) {
      console.error('Erro ao carregar vídeos:', err);
      videosError = err?.message || 'Não foi possível carregar os vídeos do servidor.';
    } finally {
      isLoadingVideos = false;
    }
  }

  const videoFoldersList = $derived.by(() => {
    const map = new Map<string, string>();
    for (const f of folders) {
      if (f.kind === 'videos' || f.kind === 'all' || !f.kind) {
        map.set(f.id, f.name);
      }
    }
    for (const v of videos) {
      if (v.folder_id && v.folder_id !== 'app' && v.is_external) {
        if (!map.has(v.folder_id)) {
          map.set(v.folder_id, v.folder_name || v.folder_id);
        }
      }
    }
    return Array.from(map.entries())
      .map(([id, name]) => ({ id, name }))
      .sort((a, b) => a.name.localeCompare(b.name));
  });

  const allAppVideoSubfolders = $derived.by(() => {
    const map = new Map<string, MediaSubfolder>();
    for (const s of appVideoSubfolders) {
      map.set(s.relative_path, { ...s });
    }
    for (const vid of videos) {
      if ((!vid.is_external || vid.folder_id === 'app') && vid.subfolder) {
        if (!map.has(vid.subfolder)) {
          map.set(vid.subfolder, {
            relative_path: vid.subfolder,
            name: vid.subfolder.split('/').pop() || vid.subfolder,
            count: 1,
          });
        }
      }
    }
    return Array.from(map.values());
  });

  const allCurrentVideoSubfolders = $derived.by(() => {
    if (videoFolderFilter === 'all') return [];
    if (videoFolderFilter === 'app') return allAppVideoSubfolders;
    const f = folders.find((item) => item.id === videoFolderFilter || item.name === videoFolderFilter);
    return f?.subfolders || [];
  });

  const currentVideoDirectSubfolders = $derived.by(() => {
    if (videoFolderFilter === 'all') return [];
    return getDirectSubfoldersForLevel(allCurrentVideoSubfolders, selectedVideoSubfolder);
  });

  const videoBreadcrumbs = $derived(formatFolderBreadcrumbs(selectedVideoSubfolder));

  const currentVideoFolderName = $derived.by(() => {
    if (videoFolderFilter === 'all') return 'Todos os Vídeos';
    if (videoFolderFilter === 'app') return 'Pasta do App';
    return videoFoldersList.find((f) => f.id === videoFolderFilter)?.name || videoFolderFilter;
  });

  function navigateUpVideoFolder() {
    if (!selectedVideoSubfolder) return;
    const parts = selectedVideoSubfolder.split('/');
    if (parts.length <= 1) {
      selectedVideoSubfolder = null;
    } else {
      parts.pop();
      selectedVideoSubfolder = parts.join('/');
    }
  }

  const filteredVideos = $derived.by(() => {
    let list = videos;
    if (videoFolderFilter === 'app') {
      list = list.filter((v) => !v.is_external || v.folder_id === 'app');
    } else if (videoFolderFilter !== 'all') {
      list = list.filter(
        (v) => v.folder_id === videoFolderFilter || v.folder_name === videoFolderFilter
      );
    }

    if (videoFolderFilter !== 'all') {
      if (selectedVideoSubfolder !== null) {
        list = list.filter((v) => v.subfolder === selectedVideoSubfolder);
      } else {
        list = list.filter((v) => !v.subfolder || v.subfolder === '');
      }
    }

    const q = videoSearch.trim().toLowerCase();
    if (!q) return list;
    return list.filter(
      (v) =>
        v.name.toLowerCase().includes(q) ||
        (v.folder_name && v.folder_name.toLowerCase().includes(q)) ||
        (v.subfolder && v.subfolder.toLowerCase().includes(q))
    );
  });

  function openNewVideoFolder() {
    const baseName =
      videoFolderFilter === 'all' || videoFolderFilter === 'app'
        ? 'Pasta do App (Vídeos)'
        : currentVideoFolderName;
    const locationLabel = selectedVideoSubfolder ? `${baseName} / ${selectedVideoSubfolder}` : baseName;
    onopencreatefolder({
      kind: 'videos',
      folderId: videoFolderFilter !== 'all' ? videoFolderFilter : 'app',
      parentSubfolder: selectedVideoSubfolder,
      locationLabel,
      oncreated: async (createdFolderId) => {
        if (videoFolderFilter === 'all') {
          videoFolderFilter = createdFolderId;
        }
        await loadVideos();
      },
    });
  }

  async function handleVideoFilesSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = Array.from(input.files || []);
    if (files.length === 0) return;

    isUploadingVideo = true;
    const folderId = videoFolderFilter !== 'all' ? videoFolderFilter : 'app';
    const subfolder = selectedVideoSubfolder || undefined;
    let uploaded = 0;

    try {
      for (const file of files) {
        const b64 = await readFileAsBase64(file);
        await uploadVideo(file.name, b64, folderId, subfolder);
        uploaded++;
      }
      notifySuccess(
        uploaded > 1 ? 'Vídeos Importados' : 'Vídeo Importado',
        uploaded > 1
          ? `${uploaded} vídeos salvos na biblioteca.`
          : `"${files[0].name}" foi salvo na biblioteca de vídeos.`
      );
      await loadVideos();
    } catch (err: any) {
      console.error('Erro no upload de vídeo:', err);
      notifyError('Erro no Upload', err?.message || 'Falha ao salvar arquivo de vídeo.');
    } finally {
      isUploadingVideo = false;
      input.value = '';
    }
  }

  function handlePresentVideo(video: VideoMediaItem) {
    const item: MediaItem = {
      id: `video-${Date.now()}`,
      title: video.name,
      artist_or_ref: video.folder_name || 'Vídeo',
      kind: 'video',
      slides: [{ id: 'slide-1', label: 'Vídeo', content: video.url }],
    };
    onpresentitem?.(item, 0);
  }

  function handleAddVideoToSetlist(video: VideoMediaItem) {
    const item: MediaItem = {
      id: `video-${Date.now()}`,
      title: video.name,
      artist_or_ref: video.folder_name || 'Vídeo',
      kind: 'video',
      slides: [{ id: 'slide-1', label: 'Vídeo', content: video.url }],
    };
    onadditemtosetlist?.(item);
    notifySuccess('Item Adicionado', `"${video.name}" adicionado à setlist.`);
  }
</script>

<div role="tabpanel" id="panel-videos" aria-labelledby="tab-videos" class="flex-1 flex flex-col overflow-hidden gap-2">
  <h2 class="sr-only">Biblioteca de Vídeos</h2>

  <!-- Barra de Ações e Filtro de Vídeos -->
  <div class="flex items-center justify-between gap-2 pb-2 border-b border-border/60 shrink-0 flex-wrap">
    <div class="flex items-center gap-2 flex-1 min-w-[200px] max-w-md">
      <div class="relative flex-1">
        <Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <input
          type="text"
          bind:value={videoSearch}
          aria-label="Buscar vídeos por nome"
          placeholder="Buscar vídeos por nome..."
          class="w-full h-8 pl-8 pr-8 rounded-lg bg-background border border-input text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
        />
        {#if videoSearch}
          <button
            type="button"
            onclick={() => (videoSearch = '')}
            aria-label="Limpar busca de vídeos"
            class="absolute right-1 top-1/2 -translate-y-1/2 w-6 h-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
          >
            <X size={12} />
          </button>
        {/if}
      </div>
      <Badge variant="secondary" class="text-[11px] font-mono py-0.5 px-2 shrink-0">
        {filteredVideos.length} {filteredVideos.length === 1 ? 'vídeo' : 'vídeos'}
      </Badge>
    </div>

    <div class="flex items-center gap-1.5 shrink-0 flex-wrap">
      <input
        type="file"
        bind:this={videoFileInputEl}
        onchange={handleVideoFilesSelected}
        accept="video/mp4,video/webm,video/ogg,video/quicktime,video/mkv,.mp4,.webm,.mov,.mkv"
        multiple
        class="hidden"
      />
      <Button
        variant="outline"
        size="sm"
        onclick={loadVideos}
        aria-label="Recarregar vídeos"
        title="Recarregar vídeos"
        class="h-8 w-8 p-0"
      >
        <RefreshCw size={13} class={isLoadingVideos ? 'animate-spin' : ''} />
      </Button>
      <Button
        variant="outline"
        size="sm"
        onclick={openNewVideoFolder}
        class="h-8 text-xs gap-1.5 font-medium"
        title="Criar nova pasta física de vídeos no diretório atual"
      >
        <FolderPlus size={13} class="text-purple-700 dark:text-purple-300" />
        <span>+ Pasta</span>
      </Button>
      <Button
        variant="outline"
        size="sm"
        onclick={onopenfolderconfig}
        class="h-8 text-xs gap-1.5"
        title="Mapear ou gerenciar pastas locais de vídeo"
      >
        <FolderOpen size={13} />
        <span>Pastas</span>
      </Button>
      <Button
        variant="default"
        size="sm"
        onclick={() => videoFileInputEl?.click()}
        disabled={isUploadingVideo}
        class="h-8 text-xs gap-1.5 font-semibold"
      >
        <Upload size={13} class={isUploadingVideo ? 'animate-spin' : ''} />
        <span>{isUploadingVideo ? 'Enviando...' : 'Importar Vídeo'}</span>
      </Button>
    </div>
  </div>

  {#if videosError}
    <div
      role="alert"
      class="px-3 py-1.5 rounded-lg border border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-between gap-2 text-xs shrink-0"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertTriangle size={14} class="shrink-0" />
        <span class="truncate font-medium">{videosError}</span>
      </div>
      <Button variant="outline" size="sm" onclick={loadVideos} class="h-7 px-2.5 text-[11px] shrink-0">
        Tentar novamente
      </Button>
    </div>
  {/if}

  <!-- Filtro de Pastas de Vídeos -->
  <div class="flex items-center gap-1.5 overflow-x-auto pb-1 text-xs shrink-0 select-none">
    <button
      type="button"
      onclick={() => {
        videoFolderFilter = 'all';
        selectedVideoSubfolder = null;
      }}
      class="min-h-[28px] px-2.5 py-1 rounded-lg border text-[11px] font-medium transition-colors cursor-pointer shrink-0 flex items-center gap-1 {videoFolderFilter === 'all' ? 'bg-purple-600 border-purple-500 text-white' : 'bg-muted/60 border-border text-muted-foreground hover:text-foreground'}"
    >
      <Layers size={11} /> Todos
    </button>
    <button
      type="button"
      onclick={() => {
        videoFolderFilter = 'app';
        selectedVideoSubfolder = null;
      }}
      class="min-h-[28px] px-2.5 py-1 rounded-lg border text-[11px] font-medium transition-colors cursor-pointer shrink-0 flex items-center gap-1 {videoFolderFilter === 'app' ? 'bg-purple-600 border-purple-500 text-white' : 'bg-muted/60 border-border text-muted-foreground hover:text-foreground'}"
    >
      <Folder size={11} /> Pasta do App
    </button>
    {#each videoFoldersList as vf (vf.id)}
      <button
        type="button"
        onclick={() => {
          videoFolderFilter = vf.id;
          selectedVideoSubfolder = null;
        }}
        class="min-h-[28px] px-2.5 py-1 rounded-lg border text-[11px] font-medium transition-colors cursor-pointer shrink-0 flex items-center gap-1 {videoFolderFilter === vf.id ? 'bg-purple-600 border-purple-500 text-white' : 'bg-muted/60 border-border text-muted-foreground hover:text-foreground'}"
      >
        <FolderOpen size={11} /> {vf.name}
      </button>
    {/each}
  </div>

  <!-- Breadcrumbs -->
  {#if videoFolderFilter !== 'all'}
    <div class="flex items-center justify-between gap-2 px-2.5 py-1 rounded-lg bg-muted/50 border border-border/60 text-xs text-muted-foreground shrink-0">
      <nav aria-label="Navegação de pastas de vídeo" class="flex items-center gap-1.5 overflow-x-auto min-w-0">
        <button
          type="button"
          onclick={() => {
            videoFolderFilter = 'all';
            selectedVideoSubfolder = null;
          }}
          class="min-h-[24px] px-1 hover:text-foreground transition-colors cursor-pointer shrink-0"
        >
          Vídeos
        </button>
        <ChevronRight size={12} class="shrink-0" />
        <button
          type="button"
          onclick={() => (selectedVideoSubfolder = null)}
          class="min-h-[24px] px-1 hover:text-foreground transition-colors cursor-pointer shrink-0 {!selectedVideoSubfolder ? 'text-foreground font-bold' : ''}"
        >
          {currentVideoFolderName}
        </button>
        {#each videoBreadcrumbs as crumb, i}
          <ChevronRight size={12} class="shrink-0" />
          {#if i === videoBreadcrumbs.length - 1}
            <span class="text-foreground font-bold truncate max-w-xs shrink-0">{crumb.name}</span>
          {:else}
            <button
              type="button"
              onclick={() => (selectedVideoSubfolder = crumb.path)}
              class="min-h-[24px] px-1 hover:text-foreground transition-colors cursor-pointer shrink-0"
            >
              {crumb.name}
            </button>
          {/if}
        {/each}
      </nav>
      {#if selectedVideoSubfolder}
        <button
          type="button"
          onclick={navigateUpVideoFolder}
          class="min-h-[24px] px-2 text-[11px] text-purple-700 dark:text-purple-300 hover:underline flex items-center gap-1 cursor-pointer font-medium shrink-0"
          title="Subir para a pasta anterior"
        >
          <ChevronUp size={12} />
          <span>Subir um nível</span>
        </button>
      {/if}
    </div>
  {/if}

  <!-- Grade de Vídeos e Subpastas -->
  <div class="flex-1 overflow-y-auto pr-1">
    {#if currentVideoDirectSubfolders.length > 0}
      <div class="mb-3.5 pb-3 border-b border-border/50">
        <div class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground mb-1.5 flex items-center justify-between">
          <span class="flex items-center gap-1.5">
            <Folder size={12} class="text-purple-700 dark:text-purple-300" />
            <span>Pastas ({currentVideoDirectSubfolders.length})</span>
          </span>
          {#if selectedVideoSubfolder}
            <button
              type="button"
              onclick={navigateUpVideoFolder}
              class="min-h-[24px] px-1.5 text-[11px] text-purple-700 dark:text-purple-300 hover:underline flex items-center gap-1 cursor-pointer font-medium normal-case tracking-normal"
              title="Subir para a pasta anterior"
            >
              <ChevronUp size={12} />
              <span>Subir um nível</span>
            </button>
          {/if}
        </div>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-2">
          {#each currentVideoDirectSubfolders as sub (sub.relative_path)}
            <button
              type="button"
              onclick={() => (selectedVideoSubfolder = sub.relative_path)}
              class="p-2.5 rounded-xl border border-border bg-card hover:bg-muted/60 hover:border-purple-500/50 text-left transition-all flex items-center gap-2.5 group cursor-pointer shadow-xs"
              title="Abrir pasta {sub.name}"
            >
              <div class="w-8 h-8 rounded-lg bg-purple-500/15 text-purple-700 dark:text-purple-300 flex items-center justify-center shrink-0 group-hover:scale-105 transition-transform">
                <Folder size={16} />
              </div>
              <div class="min-w-0 flex-1">
                <p class="text-xs font-semibold text-foreground truncate group-hover:text-purple-700 dark:group-hover:text-purple-300 transition-colors">
                  {sub.name}
                </p>
                <p class="text-[11px] text-muted-foreground font-mono">
                  {sub.count} {sub.count === 1 ? 'vídeo' : 'vídeos'}
                </p>
              </div>
            </button>
          {/each}
        </div>
      </div>
    {/if}

    {#if filteredVideos.length === 0}
      <div class="h-44 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2">
        <Film size={28} class="text-muted-foreground opacity-50" />
        {#if currentVideoDirectSubfolders.length > 0}
          <p>Nenhum vídeo na raiz desta pasta. Abra uma pasta acima ou importe vídeos.</p>
        {:else if isLoadingVideos}
          <p>Carregando vídeos...</p>
        {:else}
          <p>Nenhum vídeo encontrado nesta pasta.</p>
        {/if}
        <div class="flex items-center gap-2 mt-1">
          <Button variant="outline" size="sm" onclick={openNewVideoFolder} class="h-8 text-xs gap-1">
            <FolderPlus size={12} class="text-purple-700 dark:text-purple-300" />
            <span>Nova Pasta</span>
          </Button>
          <Button variant="outline" size="sm" onclick={() => videoFileInputEl?.click()} class="h-8 text-xs gap-1">
            <Upload size={12} />
            <span>Importar vídeo aqui</span>
          </Button>
        </div>
      </div>
    {:else}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(190px,1fr))] gap-3">
        {#each filteredVideos as vid (vid.id)}
          <article
            onpointerenter={() => (hoveredVideoId = vid.id)}
            onpointerleave={() => (hoveredVideoId = hoveredVideoId === vid.id ? null : hoveredVideoId)}
            class="group relative rounded-xl border border-border bg-card overflow-hidden flex flex-col shadow-xs hover:border-purple-500/60 transition-all"
          >
            <button
              type="button"
              onclick={() => handlePresentVideo(vid)}
              aria-label="Apresentar vídeo {vid.name}"
              class="aspect-video w-full bg-slate-950 relative overflow-hidden flex items-center justify-center cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
            >
              <!-- svelte-ignore a11y_media_has_caption -->
              <video
                src={vid.url}
                preload={hoveredVideoId === vid.id ? 'metadata' : 'none'}
                muted
                class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-200 pointer-events-none"
              ></video>
              <div class="absolute inset-0 bg-black/30 group-hover:bg-black/10 transition-colors flex items-center justify-center">
                <div class="w-9 h-9 rounded-full bg-purple-600/85 group-hover:bg-purple-600 text-white flex items-center justify-center shadow-lg transition-transform group-hover:scale-110">
                  <Play size={16} class="fill-current ml-0.5" />
                </div>
              </div>
              <div class="absolute bottom-1.5 right-1.5 px-1.5 py-0.5 rounded bg-black/80 text-[11px] text-purple-200 font-mono border border-purple-500/30">
                {formatBytes(vid.size_bytes)}
              </div>
            </button>

            <div class="p-2 bg-card border-t border-border flex flex-col gap-1.5 flex-1 justify-between">
              <div class="min-w-0">
                <h3 class="text-xs font-semibold text-foreground truncate" title={vid.name}>
                  {vid.name}
                </h3>
                {#if vid.folder_name}
                  <p class="text-[11px] text-muted-foreground truncate">{vid.folder_name}</p>
                {/if}
              </div>

              <div class="flex items-center gap-1.5 pt-1">
                <button
                  type="button"
                  onclick={() => handlePresentVideo(vid)}
                  class="flex-1 h-7 rounded-lg bg-purple-500/15 hover:bg-purple-600 text-purple-700 dark:text-purple-200 hover:text-white text-xs font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                  title="Apresentar Vídeo na Tela Pública"
                >
                  <Play size={11} class="fill-current" />
                  <span>Apresentar</span>
                </button>
                <button
                  type="button"
                  onclick={() => handleAddVideoToSetlist(vid)}
                  aria-label="Adicionar vídeo {vid.name} à Setlist"
                  class="h-7 w-7 rounded-lg bg-muted hover:bg-muted/80 text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0"
                  title="Adicionar à Setlist"
                >
                  <ListPlus size={13} />
                </button>
              </div>
            </div>
          </article>
        {/each}
      </div>
    {/if}
  </div>
</div>

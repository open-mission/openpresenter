<script lang="ts">
  import { onDestroy } from 'svelte';
  import {
    ChevronUp,
    ChevronDown,
    ChevronRight,
    ImageIcon,
    Play,
    ListPlus,
    Upload,
    FolderPlus,
    Folder,
    FolderOpen,
    RefreshCw,
    Eye,
    Layers,
    ShieldCheck,
    SlidersHorizontal,
    AlertTriangle,
  } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import Badge from '../ui/Badge.svelte';
  import {
    createMediaDirectory,
    fetchAppSubfolders,
    fetchImages,
    readFileAsBase64,
    uploadImage,
  } from '../../media';
  import { publicSettingsStore, savePublicDisplaySettings } from '../../displaySettings';
  import { notifyError, notifySuccess } from '../../toast';
  import type {
    ImageFitMode,
    ImageMediaItem,
    MediaFolder,
    MediaSubfolder,
  } from '../../types';
  import {
    buildFolderTree,
    formatFolderBreadcrumbs,
    getDirectSubfoldersForLevel,
    type FolderTreeNode,
  } from './folderTree';

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
    onfoldersrefresh?: () => Promise<void> | void;
    onpresentimage?: (image: ImageMediaItem) => void;
    onaddimagetosetlist?: (image: ImageMediaItem) => void;
    onpreviewimage?: (image: ImageMediaItem) => void;
    onopenfolderconfig?: () => void;
    onopencreatefolder: (payload: OpenCreateFolderPayload) => void;
    oncountchange?: (count: number) => void;
  }

  let {
    active,
    folders,
    onfoldersrefresh,
    onpresentimage,
    onaddimagetosetlist,
    onpreviewimage,
    onopenfolderconfig,
    onopencreatefolder,
    oncountchange,
  }: Props = $props();

  function getSavedShowSystemImages(): boolean {
    if (typeof window === 'undefined') return false;
    try {
      return localStorage.getItem('open_presenter_show_system_images') === 'true';
    } catch {
      return false;
    }
  }

  let images = $state<ImageMediaItem[]>([]);
  let appImageSubfolders = $state<MediaSubfolder[]>([]);
  let selectedFolderFilter = $state<string>('all');
  let selectedSubfolder = $state<string | null>(null);
  let isFoldersTreeExpanded = $state(true);
  let isAppFolderExpanded = $state(true);
  let expandedFolders = $state<Record<string, boolean>>({});
  let expandedSubfolders = $state<Record<string, boolean>>({});
  let isLoadingImages = $state(false);
  let isUploadingImage = $state(false);
  let imagesError = $state<string | null>(null);
  let showSystemImages = $state<boolean>(getSavedShowSystemImages());
  let isDisplayPopoverOpen = $state(false);
  let displayTriggerEl: HTMLSpanElement | null = $state(null);
  let displayPopoverEl: HTMLDivElement | null = $state(null);
  let fileInputEl: HTMLInputElement | null = $state(null);
  let hasLoadedOnce = false;

  const visibleImagesCount = $derived(
    showSystemImages ? images.length : images.filter((img) => img.is_external).length
  );

  $effect(() => {
    oncountchange?.(visibleImagesCount);
  });

  $effect(() => {
    if (active && !hasLoadedOnce) {
      hasLoadedOnce = true;
      loadImages();
    }
  });

  function toggleShowSystemImages() {
    showSystemImages = !showSystemImages;
    if (typeof window !== 'undefined') {
      try {
        localStorage.setItem('open_presenter_show_system_images', String(showSystemImages));
      } catch {
        // storage disabled
      }
    }
  }

  function handleSetImageFit(fit: ImageFitMode) {
    const updated = { ...$publicSettingsStore, image_fit: fit };
    savePublicDisplaySettings(updated).catch((err) => {
      console.warn('Falha ao salvar encaixe de imagem no servidor:', err);
    });
  }

  function handleToggleMediaMargins() {
    const current = $publicSettingsStore.ignore_margins_for_media !== false;
    const updated = { ...$publicSettingsStore, ignore_margins_for_media: !current };
    savePublicDisplaySettings(updated).catch((err) => {
      console.warn('Falha ao salvar margens de mídia no servidor:', err);
    });
  }

  async function loadAppImageSubfolders() {
    try {
      appImageSubfolders = await fetchAppSubfolders('images');
    } catch (err) {
      console.warn('Erro ao carregar subpastas de imagens:', err);
    }
  }

  export async function loadImages() {
    isLoadingImages = true;
    imagesError = null;
    try {
      images = await fetchImages();
      await loadAppImageSubfolders();
    } catch (err: any) {
      console.error('Erro ao carregar imagens:', err);
      imagesError = err?.message || 'Não foi possível carregar as imagens do servidor.';
    } finally {
      isLoadingImages = false;
    }
  }

  function toggleFolderExpand(folderId: string) {
    expandedFolders = {
      ...expandedFolders,
      [folderId]: !expandedFolders[folderId],
    };
  }

  function toggleSubfolderExpand(relPath: string) {
    const current = expandedSubfolders[relPath] ?? true;
    expandedSubfolders = {
      ...expandedSubfolders,
      [relPath]: !current,
    };
  }

  $effect(() => {
    if (selectedSubfolder) {
      const parts = selectedSubfolder.split('/');
      let acc = '';
      const updates: Record<string, boolean> = {};
      for (let i = 0; i < parts.length - 1; i++) {
        acc = acc ? `${acc}/${parts[i]}` : parts[i];
        if (expandedSubfolders[acc] === false) {
          updates[acc] = true;
        }
      }
      if (Object.keys(updates).length > 0) {
        expandedSubfolders = { ...expandedSubfolders, ...updates };
      }
    }
  });

  // Pré-computa contagens por pasta em O(N) em vez de filtrar dentro do template
  const imageCountsByFolder = $derived.by(() => {
    let appCount = 0;
    const map = new Map<string, number>();
    for (const img of images) {
      if (!img.is_external) {
        appCount++;
      } else if (img.folder_id) {
        map.set(img.folder_id, (map.get(img.folder_id) || 0) + 1);
      }
    }
    return { appCount, byFolderId: map };
  });

  const allAppImageSubfolders = $derived.by(() => {
    const map = new Map<string, MediaSubfolder>();
    for (const s of appImageSubfolders) {
      map.set(s.relative_path, { ...s });
    }
    for (const img of images) {
      if (!img.is_external && img.subfolder) {
        if (!map.has(img.subfolder)) {
          map.set(img.subfolder, {
            relative_path: img.subfolder,
            name: img.subfolder.split('/').pop() || img.subfolder,
            count: 1,
          });
        }
      }
    }
    return Array.from(map.values());
  });

  const appImageFolderTree = $derived(buildFolderTree(allAppImageSubfolders));

  const mappedFolderTrees = $derived.by(() => {
    const trees = new Map<string, FolderTreeNode[]>();
    for (const f of folders) {
      trees.set(f.id, f.subfolders ? buildFolderTree(f.subfolders) : []);
    }
    return trees;
  });

  const allCurrentFolderSubfolders = $derived.by(() => {
    if (selectedFolderFilter === 'all') return [];
    if (selectedFolderFilter === 'app') return allAppImageSubfolders;
    const f = folders.find((item) => item.id === selectedFolderFilter);
    return f?.subfolders || [];
  });

  const currentSubfolders = $derived.by(() => {
    if (selectedFolderFilter === 'all') return [];
    return getDirectSubfoldersForLevel(allCurrentFolderSubfolders, selectedSubfolder);
  });

  const imageBreadcrumbs = $derived(formatFolderBreadcrumbs(selectedSubfolder));

  const currentFolderName = $derived.by(() => {
    if (selectedFolderFilter === 'all') {
      return showSystemImages ? 'Todas as Imagens (+ Sistema)' : 'Pastas Importadas';
    }
    if (selectedFolderFilter === 'app') return 'Pasta do App (Sistema)';
    const f = folders.find((item) => item.id === selectedFolderFilter);
    return f ? f.name : 'Pasta Selecionada';
  });

  function navigateUpImageFolder() {
    if (!selectedSubfolder) return;
    const parts = selectedSubfolder.split('/');
    if (parts.length <= 1) {
      selectedSubfolder = null;
    } else {
      parts.pop();
      selectedSubfolder = parts.join('/');
    }
  }

  const filteredImages = $derived.by(() => {
    let list = images;
    if (!showSystemImages && selectedFolderFilter !== 'app') {
      list = list.filter((img) => img.is_external);
    }
    if (selectedFolderFilter === 'app') {
      list = list.filter((img) => !img.is_external);
    } else if (selectedFolderFilter !== 'all') {
      list = list.filter((img) => img.folder_id === selectedFolderFilter);
    }

    if (selectedFolderFilter === 'all') {
      if (selectedSubfolder !== null) {
        list = list.filter((img) => img.subfolder === selectedSubfolder);
      }
      return list;
    }

    if (selectedSubfolder !== null) {
      list = list.filter((img) => img.subfolder === selectedSubfolder);
    } else {
      list = list.filter((img) => !img.subfolder || img.subfolder === '');
    }
    return list;
  });

  function openNewImageFolder() {
    const baseName =
      selectedFolderFilter === 'all' || selectedFolderFilter === 'app'
        ? 'Pasta do App (Imagens)'
        : currentFolderName;
    const locationLabel = selectedSubfolder ? `${baseName} / ${selectedSubfolder}` : baseName;
    onopencreatefolder({
      kind: 'images',
      folderId: selectedFolderFilter !== 'all' ? selectedFolderFilter : 'app',
      parentSubfolder: selectedSubfolder,
      locationLabel,
      oncreated: async (createdFolderId) => {
        if (selectedFolderFilter === 'all') {
          selectedFolderFilter = createdFolderId;
        }
        await loadImages();
        await onfoldersrefresh?.();
      },
    });
  }

  async function handleFileUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = Array.from(input.files || []);
    if (files.length === 0) return;

    isUploadingImage = true;
    const folderId = selectedFolderFilter !== 'all' ? selectedFolderFilter : 'app';
    const subfolder = selectedSubfolder || undefined;
    let uploaded = 0;

    try {
      for (const file of files) {
        const b64 = await readFileAsBase64(file);
        await uploadImage(file.name, b64, folderId, subfolder);
        uploaded++;
      }
      notifySuccess(
        uploaded > 1 ? 'Imagens Importadas' : 'Imagem Importada',
        uploaded > 1
          ? `${uploaded} imagens importadas com sucesso.`
          : `"${files[0].name}" foi importada com sucesso.`
      );
      await loadImages();
    } catch (err: any) {
      console.error('Erro no upload de imagem:', err);
      notifyError('Erro no Upload', err?.message || 'Falha ao salvar arquivo de imagem.');
    } finally {
      isUploadingImage = false;
      input.value = '';
    }
  }

  $effect(() => {
    if (isDisplayPopoverOpen) {
      setTimeout(() => {
        displayPopoverEl?.querySelector<HTMLButtonElement>('button')?.focus();
      }, 10);
    }
  });

  function handlePopoverKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isDisplayPopoverOpen) {
      e.preventDefault();
      e.stopPropagation();
      isDisplayPopoverOpen = false;
      displayTriggerEl?.querySelector('button')?.focus();
    }
  }
</script>

<svelte:window onkeydown={handlePopoverKeyDown} />

<input
  type="file"
  accept="image/*"
  multiple
  bind:this={fileInputEl}
  onchange={handleFileUpload}
  class="hidden"
/>

{#snippet renderTreeNode(node: FolderTreeNode, folderId: string)}
  {@const isSelected = selectedFolderFilter === folderId && selectedSubfolder === node.relative_path}
  {@const hasChildren = node.children.length > 0}
  {@const isNodeExpanded = expandedSubfolders[node.relative_path] ?? true}
  {@const isCurrentOrParent =
    selectedFolderFilter === folderId &&
    selectedSubfolder !== null &&
    (selectedSubfolder === node.relative_path || selectedSubfolder.startsWith(node.relative_path + '/'))}

  <div class="space-y-0.5">
    <div
      class="w-full flex items-center gap-0.5 py-0.5 pr-1.5 rounded-md text-[11px] transition-colors {isSelected ? 'bg-primary/15 text-primary font-bold' : isCurrentOrParent ? 'bg-muted/70 text-foreground font-semibold' : 'text-muted-foreground hover:bg-muted/60 hover:text-foreground'}"
      style="padding-left: {node.depth * 10 + 4}px;"
    >
      {#if hasChildren}
        <button
          type="button"
          onclick={() => toggleSubfolderExpand(node.relative_path)}
          aria-label="{isNodeExpanded ? 'Recolher' : 'Expandir'} subpasta {node.name}"
          aria-expanded={isNodeExpanded}
          class="w-6 h-6 rounded hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center cursor-pointer shrink-0"
          title={isNodeExpanded ? 'Recolher subpasta' : 'Expandir subpasta'}
        >
          {#if isNodeExpanded}
            <ChevronDown size={12} />
          {:else}
            <ChevronRight size={12} />
          {/if}
        </button>
      {:else}
        <span class="w-6 shrink-0"></span>
      {/if}

      <button
        type="button"
        onclick={() => {
          selectedFolderFilter = folderId;
          selectedSubfolder = node.relative_path;
        }}
        class="flex-1 min-w-0 min-h-[26px] flex items-center justify-between gap-1 text-left cursor-pointer rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
        title="{node.name} ({node.relative_path})"
      >
        <span class="flex items-center gap-1.5 min-w-0">
          {#if isSelected || isCurrentOrParent}
            <FolderOpen size={12} class="text-amber-600 dark:text-amber-400 shrink-0" />
          {:else}
            <Folder size={12} class="text-amber-600/80 dark:text-amber-400/80 shrink-0" />
          {/if}
          <span class="truncate">{node.name}</span>
        </span>
        <span class="text-[11px] font-mono opacity-80 ml-1 shrink-0">{node.count}</span>
      </button>
    </div>

    {#if hasChildren && isNodeExpanded}
      <div class="relative space-y-0.5">
        <div
          class="absolute top-0 bottom-1 border-l border-border/60 pointer-events-none"
          style="left: {node.depth * 10 + 15}px;"
        ></div>
        {#each node.children as child (child.relative_path)}
          {@render renderTreeNode(child, folderId)}
        {/each}
      </div>
    {/if}
  </div>
{/snippet}

<div role="tabpanel" id="panel-images" aria-labelledby="tab-images" class="flex-1 flex overflow-hidden gap-3">
  <h2 class="sr-only">Galeria de Imagens e Pastas</h2>

  <!-- Tree Navigation Vertical para Pastas -->
  <aside
    aria-label="Árvore de pastas de imagens"
    class="w-52 lg:w-56 border-r border-border bg-card/50 flex flex-col shrink-0 p-2 overflow-y-auto space-y-1"
  >
    <span class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground px-2 py-1 block">
      Navegação de Pastas
    </span>

    <button
      type="button"
      onclick={() => {
        selectedFolderFilter = 'all';
        selectedSubfolder = null;
      }}
      class="w-full min-h-[30px] flex items-center justify-between px-2.5 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedFolderFilter === 'all' ? 'bg-primary/15 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      <div class="flex items-center gap-2 truncate">
        <Layers size={13} />
        <span class="truncate">{showSystemImages ? 'Todas as Imagens' : 'Pastas Importadas'}</span>
      </div>
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">
        {visibleImagesCount}
      </Badge>
    </button>

    <!-- Pasta do App -->
    <div class="space-y-0.5">
      <div
        class="w-full flex items-center gap-0.5 pr-1.5 py-0.5 rounded-lg text-xs font-medium transition-colors {selectedFolderFilter === 'app' && selectedSubfolder === null ? 'bg-primary/15 text-primary font-bold' : selectedFolderFilter === 'app' ? 'bg-muted/70 text-foreground font-semibold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
      >
        {#if appImageFolderTree.length > 0}
          <button
            type="button"
            onclick={() => (isAppFolderExpanded = !isAppFolderExpanded)}
            aria-label="{isAppFolderExpanded ? 'Recolher' : 'Expandir'} subpastas do App"
            aria-expanded={isAppFolderExpanded}
            class="w-6 h-6 rounded hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center cursor-pointer shrink-0"
            title={isAppFolderExpanded ? 'Recolher subpastas' : 'Expandir subpastas'}
          >
            {#if isAppFolderExpanded}
              <ChevronDown size={12} />
            {:else}
              <ChevronRight size={12} />
            {/if}
          </button>
        {:else}
          <span class="w-6 shrink-0"></span>
        {/if}

        <button
          type="button"
          onclick={() => {
            selectedFolderFilter = 'app';
            selectedSubfolder = null;
          }}
          class="flex-1 min-w-0 min-h-[28px] flex items-center justify-between gap-1.5 text-left cursor-pointer rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
        >
          <span class="flex items-center gap-1.5 truncate">
            <Folder size={13} class="text-primary shrink-0" />
            <span class="truncate">Pasta do App</span>
          </span>
          <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono shrink-0">
            {imageCountsByFolder.appCount}
          </Badge>
        </button>
      </div>

      {#if isAppFolderExpanded && appImageFolderTree.length > 0}
        <div class="space-y-0.5 relative pl-1">
          <div class="absolute top-0 bottom-1 border-l border-border/60 pointer-events-none left-3"></div>
          {#each appImageFolderTree as rootNode (rootNode.relative_path)}
            {@render renderTreeNode(rootNode, 'app')}
          {/each}
        </div>
      {/if}
    </div>

    <!-- Pastas Mapeadas -->
    <div class="pt-2">
      <button
        type="button"
        onclick={() => (isFoldersTreeExpanded = !isFoldersTreeExpanded)}
        aria-expanded={isFoldersTreeExpanded}
        class="w-full min-h-[28px] flex items-center justify-between px-2 py-1 text-[11px] font-bold text-muted-foreground hover:text-foreground cursor-pointer rounded"
      >
        <span class="flex items-center gap-1">
          {#if isFoldersTreeExpanded}
            <ChevronDown size={12} />
          {:else}
            <ChevronRight size={12} />
          {/if}
          Pastas Mapeadas
        </span>
        <Badge variant="outline" class="text-[11px] py-0 px-1.5 font-mono">
          {folders.length}
        </Badge>
      </button>

      {#if isFoldersTreeExpanded}
        <div class="space-y-0.5 mt-1 pl-1">
          {#if folders.length === 0}
            <p class="text-[11px] text-muted-foreground italic px-2 py-1">Nenhuma pasta mapeada</p>
          {:else}
            {#each folders as f (f.id)}
              {@const count = imageCountsByFolder.byFolderId.get(f.id) || 0}
              {@const subs = f.subfolders || []}
              {@const isExpanded = !!expandedFolders[f.id]}
              {@const isFolderSelected = selectedFolderFilter === f.id}
              {@const mappedTree = mappedFolderTrees.get(f.id) || []}

              <div class="space-y-0.5">
                <div
                  class="w-full flex items-center gap-0.5 pr-1.5 py-0.5 rounded-md text-xs font-medium transition-colors {isFolderSelected && selectedSubfolder === null ? 'bg-primary/15 text-primary font-bold' : isFolderSelected ? 'bg-muted/70 text-foreground font-semibold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
                  title={f.path}
                >
                  {#if subs.length > 0}
                    <button
                      type="button"
                      onclick={() => toggleFolderExpand(f.id)}
                      aria-label="{isExpanded ? 'Recolher' : 'Expandir'} subpastas de {f.name}"
                      aria-expanded={isExpanded}
                      class="w-6 h-6 rounded hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center cursor-pointer shrink-0"
                      title={isExpanded ? 'Recolher sub-pastas' : 'Expandir sub-pastas'}
                    >
                      {#if isExpanded}
                        <ChevronDown size={12} />
                      {:else}
                        <ChevronRight size={12} />
                      {/if}
                    </button>
                  {:else}
                    <span class="w-6 shrink-0"></span>
                  {/if}

                  <button
                    type="button"
                    onclick={() => {
                      selectedFolderFilter = f.id;
                      selectedSubfolder = null;
                      if (subs.length > 0) {
                        expandedFolders = { ...expandedFolders, [f.id]: true };
                      }
                    }}
                    class="flex-1 min-w-0 min-h-[28px] flex items-center justify-between gap-1 text-left cursor-pointer rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
                  >
                    <span class="flex items-center gap-1.5 min-w-0">
                      <FolderOpen size={12} class="text-amber-600 dark:text-amber-400 shrink-0" />
                      <span class="truncate text-[11px]">{f.name}</span>
                    </span>
                    <span class="text-[11px] font-mono text-muted-foreground ml-1 shrink-0">{count}</span>
                  </button>
                </div>

                {#if isExpanded && mappedTree.length > 0}
                  <div class="space-y-0.5 relative pl-1">
                    <div class="absolute top-0 bottom-1 border-l border-border/60 pointer-events-none left-3"></div>
                    {#each mappedTree as rootNode (rootNode.relative_path)}
                      {@render renderTreeNode(rootNode, f.id)}
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

  <!-- Área Principal da Galeria de Imagens -->
  <main class="flex-1 flex flex-col overflow-hidden min-w-0">
    <!-- Barra de Breadcrumbs e Ações Compacta (Caber em 1024px sem clipping) -->
    <div class="flex items-center justify-between gap-2 mb-2 shrink-0 pb-1.5 border-b border-border flex-wrap">
      <nav aria-label="Caminho da pasta de imagens" class="flex items-center gap-1.5 text-xs text-muted-foreground overflow-x-auto min-w-0">
        <button
          type="button"
          onclick={() => {
            selectedFolderFilter = 'all';
            selectedSubfolder = null;
          }}
          class="min-h-[24px] px-1 hover:text-foreground transition-colors cursor-pointer shrink-0"
        >
          Mídias
        </button>
        <ChevronRight size={12} class="shrink-0" />
        <button
          type="button"
          onclick={() => {
            selectedFolderFilter = 'all';
            selectedSubfolder = null;
          }}
          class="min-h-[24px] px-1 hover:text-foreground transition-colors cursor-pointer shrink-0 {selectedFolderFilter === 'all' ? 'text-foreground font-bold' : ''}"
        >
          Imagens
        </button>
        {#if selectedFolderFilter !== 'all'}
          <ChevronRight size={12} class="shrink-0" />
          <button
            type="button"
            onclick={() => (selectedSubfolder = null)}
            class="min-h-[24px] px-1 hover:text-foreground transition-colors cursor-pointer shrink-0 {!selectedSubfolder ? 'text-foreground font-bold' : ''}"
          >
            {currentFolderName}
          </button>
        {/if}
        {#each imageBreadcrumbs as crumb, i}
          <ChevronRight size={12} class="shrink-0" />
          {#if i === imageBreadcrumbs.length - 1}
            <span class="text-foreground font-bold truncate max-w-[160px] shrink-0">{crumb.name}</span>
          {:else}
            <button
              type="button"
              onclick={() => (selectedSubfolder = crumb.path)}
              class="min-h-[24px] px-1 hover:text-foreground transition-colors cursor-pointer shrink-0"
            >
              {crumb.name}
            </button>
          {/if}
        {/each}
        <Badge variant="secondary" class="text-[11px] py-0 px-1.5 ml-1 font-mono shrink-0">
          {filteredImages.length}
        </Badge>
      </nav>

      <div class="flex items-center gap-1.5 shrink-0 flex-wrap">
        <!-- Popover de Ajustes de Privacidade e Enquadramento (Progressive Disclosure) -->
        <div class="relative">
          <span class="inline-flex" bind:this={displayTriggerEl}>
            <Button
              variant="outline"
              size="sm"
              onclick={() => (isDisplayPopoverOpen = !isDisplayPopoverOpen)}
              aria-haspopup="true"
              aria-expanded={isDisplayPopoverOpen}
              class="h-8 text-xs gap-1.5"
              title="Configurar privacidade, margens e enquadramento"
            >
              <SlidersHorizontal size={13} />
              <span>Exibição</span>
            </Button>
          </span>

          {#if isDisplayPopoverOpen}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              class="fixed inset-0 z-40"
              onclick={() => (isDisplayPopoverOpen = false)}
              onkeydown={handlePopoverKeyDown}
            ></div>

            <div
              bind:this={displayPopoverEl}
              role="dialog"
              aria-label="Opções de exibição de imagens"
              tabindex="-1"
              onkeydown={handlePopoverKeyDown}
              class="absolute right-0 top-full mt-1.5 w-72 rounded-xl border border-border bg-popover text-popover-foreground shadow-2xl p-3 z-50 space-y-3"
            >
              <div class="space-y-1.5">
                <span class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground block">
                  Privacidade da Galeria
                </span>
                <button
                  type="button"
                  onclick={toggleShowSystemImages}
                  aria-pressed={!showSystemImages}
                  class="w-full h-8 px-2.5 rounded-lg text-xs font-semibold border transition-colors flex items-center justify-between cursor-pointer {!showSystemImages ? 'bg-emerald-500/15 text-emerald-700 dark:text-emerald-300 border-emerald-500/35' : 'bg-muted/60 text-foreground border-border hover:bg-muted'}"
                >
                  <span class="flex items-center gap-1.5">
                    {#if !showSystemImages}
                      <ShieldCheck size={13} />
                      <span>Somente Pastas Importadas</span>
                    {:else}
                      <Eye size={13} />
                      <span>Todas + Sistema</span>
                    {/if}
                  </span>
                  <span class="text-[11px] opacity-80">{!showSystemImages ? 'Ativo' : 'Aberto'}</span>
                </button>
              </div>

              <div class="space-y-1.5 pt-2 border-t border-border/60">
                <span class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground block">
                  Margens na Projeção
                </span>
                <button
                  type="button"
                  onclick={handleToggleMediaMargins}
                  aria-pressed={$publicSettingsStore.ignore_margins_for_media !== false}
                  class="w-full h-8 px-2.5 rounded-lg text-xs font-semibold border transition-colors flex items-center justify-between cursor-pointer {$publicSettingsStore.ignore_margins_for_media !== false ? 'bg-primary/15 text-primary border-primary/40' : 'bg-muted/60 text-foreground border-border hover:bg-muted'}"
                >
                  <span>{$publicSettingsStore.ignore_margins_for_media !== false ? 'Sem Margens (Tela Cheia 16:9)' : 'Com Margens Padrão'}</span>
                </button>
              </div>

              <div class="space-y-1.5 pt-2 border-t border-border/60">
                <span class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground block">
                  Enquadramento
                </span>
                <div class="grid grid-cols-3 gap-1">
                  {#each [
                    { id: 'contain', label: 'Ajustar' },
                    { id: 'cover', label: 'Preencher' },
                    { id: 'fill', label: 'Esticar' }
                  ] as f}
                    <button
                      type="button"
                      onclick={() => handleSetImageFit(f.id as ImageFitMode)}
                      aria-pressed={($publicSettingsStore.image_fit || 'contain') === f.id}
                      class="h-8 rounded-md text-[11px] transition-colors cursor-pointer border {($publicSettingsStore.image_fit || 'contain') === f.id ? 'bg-primary text-primary-foreground border-primary font-bold' : 'bg-background border-border text-muted-foreground hover:text-foreground'}"
                    >
                      {f.label}
                    </button>
                  {/each}
                </div>
              </div>
            </div>
          {/if}
        </div>

        <Button
          variant="outline"
          size="sm"
          onclick={loadImages}
          aria-label="Recarregar imagens"
          title="Recarregar imagens"
          class="h-8 w-8 p-0"
        >
          <RefreshCw size={13} class={isLoadingImages ? 'animate-spin' : ''} />
        </Button>

        <Button
          variant="outline"
          size="sm"
          onclick={openNewImageFolder}
          class="h-8 text-xs gap-1 font-medium"
          title="Criar nova pasta física no diretório atual"
        >
          <FolderPlus size={13} class="text-amber-600 dark:text-amber-400" />
          <span>+ Pasta</span>
        </Button>

        <Button
          variant="outline"
          size="sm"
          onclick={onopenfolderconfig}
          class="h-8 text-xs gap-1"
          title="Configurar pastas externas de mídia"
        >
          <FolderOpen size={13} />
          <span>Pastas</span>
        </Button>

        <Button
          variant="default"
          size="sm"
          onclick={() => fileInputEl?.click()}
          disabled={isUploadingImage}
          class="h-8 text-xs gap-1 font-semibold"
        >
          <Upload size={13} class={isUploadingImage ? 'animate-spin' : ''} />
          <span>{isUploadingImage ? 'Enviando...' : 'Importar Imagem'}</span>
        </Button>
      </div>
    </div>

    {#if imagesError}
      <div
        role="alert"
        class="mb-2 px-3 py-1.5 rounded-lg border border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-between gap-2 text-xs shrink-0"
      >
        <div class="flex items-center gap-2 min-w-0">
          <AlertTriangle size={14} class="shrink-0" />
          <span class="truncate font-medium">{imagesError}</span>
        </div>
        <Button variant="outline" size="sm" onclick={loadImages} class="h-7 px-2.5 text-[11px] shrink-0">
          Tentar novamente
        </Button>
      </div>
    {/if}

    <!-- Grade de Subpastas e Miniaturas -->
    <div class="flex-1 overflow-y-auto pr-1">
      {#if currentSubfolders.length > 0}
        <div class="mb-3.5 pb-3 border-b border-border/50">
          <div class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground mb-1.5 flex items-center justify-between">
            <span class="flex items-center gap-1.5">
              <Folder size={12} class="text-amber-600 dark:text-amber-400" />
              <span>Pastas ({currentSubfolders.length})</span>
            </span>
            {#if selectedSubfolder}
              <button
                type="button"
                onclick={navigateUpImageFolder}
                class="min-h-[24px] px-1.5 text-[11px] text-primary hover:underline flex items-center gap-1 cursor-pointer font-medium normal-case tracking-normal"
                title="Subir para a pasta anterior"
              >
                <ChevronUp size={12} />
                <span>Subir um nível</span>
              </button>
            {/if}
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-2">
            {#each currentSubfolders as sub (sub.relative_path)}
              <button
                type="button"
                onclick={() => (selectedSubfolder = sub.relative_path)}
                class="p-2.5 rounded-xl border border-border bg-card hover:bg-muted/60 hover:border-amber-500/50 text-left transition-all flex items-center gap-2.5 group cursor-pointer shadow-xs"
                title="Abrir pasta {sub.name}"
              >
                <div class="w-8 h-8 rounded-lg bg-amber-500/15 text-amber-700 dark:text-amber-300 flex items-center justify-center shrink-0 group-hover:scale-105 transition-transform">
                  <Folder size={16} />
                </div>
                <div class="min-w-0 flex-1">
                  <p class="text-xs font-semibold text-foreground truncate group-hover:text-amber-700 dark:group-hover:text-amber-300 transition-colors">
                    {sub.name}
                  </p>
                  <p class="text-[11px] text-muted-foreground font-mono">
                    {sub.count} {sub.count === 1 ? 'item' : 'itens'}
                  </p>
                </div>
              </button>
            {/each}
          </div>
        </div>
      {/if}

      {#if filteredImages.length === 0}
        <div class="h-32 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2">
          <ImageIcon size={28} class="opacity-40" />
          {#if currentSubfolders.length > 0}
            <span>Nenhuma imagem na raiz desta pasta. Abra uma pasta acima ou importe imagens.</span>
          {:else if isLoadingImages}
            <span>Carregando imagens...</span>
          {:else}
            <span>Nenhuma imagem encontrada nesta pasta.</span>
            <Button
              variant="outline"
              size="sm"
              onclick={() => fileInputEl?.click()}
              class="h-8 text-xs gap-1 mt-1"
            >
              <Upload size={12} />
              <span>Importar primeira imagem aqui</span>
            </Button>
          {/if}
        </div>
      {:else}
        <div class="grid grid-cols-[repeat(auto-fill,minmax(172px,1fr))] gap-3">
          {#each filteredImages as img (img.id)}
            <article class="group relative rounded-lg border border-border bg-card overflow-hidden flex flex-col shadow-xs hover:border-primary/50 transition-all">
              <button
                type="button"
                onclick={() => onpresentimage?.(img)}
                aria-label="Apresentar imagem {img.name}"
                class="aspect-video w-full bg-slate-950 relative overflow-hidden cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
              >
                <img
                  src={img.url}
                  alt={img.name}
                  class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                  loading="lazy"
                />

                <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/85 via-black/45 to-transparent p-1.5 pt-3 text-left">
                  <p class="text-[11px] text-white font-medium truncate drop-shadow">{img.name}</p>
                  {#if img.subfolder && selectedSubfolder === null}
                    <p class="text-[11px] text-amber-200 font-mono truncate drop-shadow">{img.subfolder}</p>
                  {/if}
                </div>
              </button>

              <div class="p-1.5 bg-muted/30 border-t border-border flex items-center justify-between gap-1">
                <button
                  type="button"
                  onclick={() => onpresentimage?.(img)}
                  class="flex-1 min-w-0 h-7 px-2 rounded-md bg-primary/15 hover:bg-primary/25 text-primary text-[11px] font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                  title="Apresentar Imagem Imediatamente (Sem adicionar à Setlist)"
                >
                  <Play size={11} class="fill-current shrink-0" />
                  <span class="truncate">Apresentar</span>
                </button>

                <button
                  type="button"
                  onclick={() => onpreviewimage?.(img)}
                  aria-label="Visualizar prévia de {img.name}"
                  class="w-7 h-7 rounded-md bg-muted hover:bg-muted/80 text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0"
                  title="Visualizar Prévia e Metadados"
                >
                  <Eye size={13} />
                </button>

                <button
                  type="button"
                  onclick={() => onaddimagetosetlist?.(img)}
                  aria-label="Adicionar imagem {img.name} à Setlist"
                  class="w-7 h-7 rounded-md bg-muted hover:bg-muted/80 text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0"
                  title="Adicionar à Setlist"
                >
                  <ListPlus size={13} />
                </button>
              </div>
            </article>
          {/each}
        </div>
      {/if}
    </div>
  </main>
</div>

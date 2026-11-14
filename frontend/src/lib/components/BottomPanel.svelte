<script lang="ts">
  import { onDestroy } from 'svelte';
  import {
    ChevronUp,
    ChevronDown,
    Maximize2,
    Minimize2,
    Music,
    ImageIcon,
    BookOpen,
    Film,
    Volume2,
    Palette,
    Boxes,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import { cn } from '../utils';
  import BiblePanel from './BiblePanel.svelte';
  import ContextMenu, { type MenuItem } from './ContextMenu.svelte';
  import SongsTab from './bottom-panel/SongsTab.svelte';
  import ImagesTab from './bottom-panel/ImagesTab.svelte';
  import VideosTab from './bottom-panel/VideosTab.svelte';
  import AudiosTab from './bottom-panel/AudiosTab.svelte';
  import BackgroundsTab from './bottom-panel/BackgroundsTab.svelte';
  import OthersTab from './bottom-panel/OthersTab.svelte';
  import CreateFolderModal from './bottom-panel/CreateFolderModal.svelte';
  import ConfirmModal from './bottom-panel/ConfirmModal.svelte';
  import { BACKGROUND_PRESETS } from '../backgrounds';
  import { customItemsStore } from '../customItems';
  import type {
    ImageMediaItem,
    MediaFolder,
    MediaItem,
    ParsedReference,
  } from '../types';

  export type BottomTab =
    | 'songs'
    | 'images'
    | 'videos'
    | 'audios'
    | 'backgrounds'
    | 'bible'
    | 'others';

  interface ConfirmRequest {
    title: string;
    message: string;
    confirmLabel?: string;
    onconfirm: () => Promise<void> | void;
  }

  interface CreateFolderRequest {
    kind: 'images' | 'videos';
    folderId: string;
    parentSubfolder: string | null;
    locationLabel: string;
    oncreated: (folderId: string) => Promise<void> | void;
  }

  interface TabMeta {
    id: BottomTab;
    label: string;
    icon: typeof Music;
    iconClass: string;
  }

  const TABS: TabMeta[] = [
    {
      id: 'songs',
      label: 'Músicas',
      icon: Music,
      iconClass: 'text-amber-600 dark:text-amber-400',
    },
    {
      id: 'bible',
      label: 'Bíblia',
      icon: BookOpen,
      iconClass: 'text-emerald-600 dark:text-emerald-400',
    },
    {
      id: 'images',
      label: 'Imagens & Pastas',
      icon: ImageIcon,
      iconClass: 'text-cyan-600 dark:text-cyan-400',
    },
    {
      id: 'videos',
      label: 'Vídeos',
      icon: Film,
      iconClass: 'text-purple-600 dark:text-purple-400',
    },
    {
      id: 'audios',
      label: 'Áudios',
      icon: Volume2,
      iconClass: 'text-indigo-600 dark:text-indigo-400',
    },
    {
      id: 'backgrounds',
      label: 'Fundos & Gradientes',
      icon: Palette,
      iconClass: 'text-fuchsia-600 dark:text-fuchsia-400',
    },
    {
      id: 'others',
      label: 'Outros',
      icon: Boxes,
      iconClass: 'text-amber-600 dark:text-amber-400',
    },
  ];

  interface Props {
    onaddsongtosetlist?: (songId: string) => void;
    onpresentsong?: (songId: string, slideIndex?: number) => void;
    onpresentimage?: (image: ImageMediaItem) => void;
    onaddimagetosetlist?: (image: ImageMediaItem) => void;
    onpreviewimage?: (image: ImageMediaItem) => void;
    onopensongeditor?: () => void;
    onopeneditsong?: (songId: string) => void;
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
    onopentimereditor?: (timerItem?: MediaItem) => void;
    activeSetlistItems?: MediaItem[];
    requestedTab?: BottomTab | null;
    targetScriptureRef?: ParsedReference | null;
  }

  let {
    onaddsongtosetlist,
    onpresentsong,
    onpresentimage,
    onaddimagetosetlist,
    onpreviewimage,
    onopensongeditor,
    onopeneditsong,
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
    onopentimereditor,
    activeSetlistItems = [],
    requestedTab = null,
    targetScriptureRef = null,
  }: Props = $props();

  // Panel open / height / tab state
  let activeTab = $state<BottomTab>('songs');
  let isCollapsed = $state(true);
  let isMaximized = $state(false);
  let mediaPanelHeight = $state(290);
  let biblePanelHeight = $state(
    typeof window !== 'undefined'
      ? Math.min(540, Math.max(360, Math.round(window.innerHeight * 0.62)))
      : 540
  );
  let viewportHeight = $state(typeof window !== 'undefined' ? window.innerHeight : 900);

  // Lazy mount flags so collapsed startup fires zero media/API scans and tab state persists across switches
  let visitedTabs = $state<Record<BottomTab, boolean>>({
    songs: false,
    bible: false,
    images: false,
    videos: false,
    audios: false,
    backgrounds: false,
    others: false,
  });

  // Tab counts (null until loaded so unvisited tabs show no bogus zeroes)
  let songsCount = $state<number | null>(null);
  let imagesCount = $state<number | null>(null);
  let videosCount = $state<number | null>(null);
  let audiosCount = $state<number | null>(null);
  let backgroundsCount = $state<number>(BACKGROUND_PRESETS.length);

  // Shared media folder list (fetched lazily once when Images or Videos tab opens)
  let folders = $state<MediaFolder[]>([]);
  let foldersLoaded = $state(false);
  let foldersAttempted = $state(false);
  let foldersLoading = $state(false);

  // Resize drag state
  let isResizing = $state(false);
  let startY = 0;
  let startHeight = 0;
  let activePointerId: number | null = null;
  let tabButtonEls = $state<(HTMLButtonElement | null)[]>([]);

  // Shared context menu state
  let contextMenu = $state<{
    open: boolean;
    x: number;
    y: number;
    items: MenuItem[];
  }>({ open: false, x: 0, y: 0, items: [] });

  // Shared create-folder modal state
  let createFolderModal = $state<{
    open: boolean;
    kind: 'images' | 'videos';
    folderId: string;
    parentSubfolder: string | null;
    locationLabel: string;
    oncreated: ((folderId: string) => Promise<void> | void) | null;
  }>({
    open: false,
    kind: 'images',
    folderId: 'app',
    parentSubfolder: null,
    locationLabel: '',
    oncreated: null,
  });

  // Shared accessible confirmation modal state (replaces native confirm())
  let confirmState = $state<{
    open: boolean;
    title: string;
    message: string;
    confirmLabel: string;
    isProcessing: boolean;
    onconfirm: (() => Promise<void> | void) | null;
  }>({
    open: false,
    title: '',
    message: '',
    confirmLabel: 'Excluir',
    isProcessing: false,
    onconfirm: null,
  });

  const maxPanelHeight = $derived(Math.max(260, viewportHeight - 80));
  const minTabHeight = $derived(activeTab === 'bible' ? 260 : 200);
  const currentTabHeight = $derived.by(() => {
    const raw = activeTab === 'bible' ? biblePanelHeight : mediaPanelHeight;
    return Math.max(minTabHeight, Math.min(maxPanelHeight, raw));
  });
  const effectivePanelHeight = $derived(isMaximized ? maxPanelHeight : currentTabHeight);

  // React to external tab requests from AdminView / Command Palette
  $effect(() => {
    if (requestedTab) {
      activeTab = requestedTab;
      isCollapsed = false;
      visitedTabs[requestedTab] = true;
    }
  });

  $effect(() => {
    if (targetScriptureRef) {
      activeTab = 'bible';
      isCollapsed = false;
      visitedTabs.bible = true;
    }
  });

  $effect(() => {
    if (!isCollapsed) {
      visitedTabs[activeTab] = true;
    }
  });

  $effect(() => {
    if (
      !isCollapsed &&
      (activeTab === 'images' || activeTab === 'videos') &&
      !foldersLoaded &&
      !foldersAttempted &&
      !foldersLoading
    ) {
      void loadFolders();
    }
  });

  async function loadFolders() {
    foldersAttempted = true;
    foldersLoading = true;
    try {
      const res = await fetch('/api/media/folders');
      if (res.ok) {
        folders = await res.json();
        foldersLoaded = true;
      }
    } catch (err) {
      console.error('Erro ao carregar pastas de mídia:', err);
    } finally {
      foldersLoading = false;
    }
  }

  function handleWindowResize() {
    viewportHeight = window.innerHeight;
  }

  if (typeof window !== 'undefined') {
    window.addEventListener('resize', handleWindowResize);
  }

  function clampTabHeight(height: number): number {
    return Math.max(minTabHeight, Math.min(maxPanelHeight, height));
  }

  function setCurrentTabHeight(nextHeight: number) {
    const clamped = clampTabHeight(nextHeight);
    if (activeTab === 'bible') {
      biblePanelHeight = clamped;
    } else {
      mediaPanelHeight = clamped;
    }
  }

  function selectTab(tab: BottomTab) {
    if (activeTab === tab && !isCollapsed) {
      isCollapsed = true;
      isMaximized = false;
    } else {
      activeTab = tab;
      visitedTabs[tab] = true;
      isCollapsed = false;
    }
  }

  function handleTabKeyDown(e: KeyboardEvent, index: number) {
    let nextIndex: number | null = null;
    if (e.key === 'ArrowRight') nextIndex = (index + 1) % TABS.length;
    else if (e.key === 'ArrowLeft') nextIndex = (index - 1 + TABS.length) % TABS.length;
    else if (e.key === 'Home') nextIndex = 0;
    else if (e.key === 'End') nextIndex = TABS.length - 1;
    else if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      selectTab(TABS[index].id);
      return;
    }

    if (nextIndex !== null) {
      e.preventDefault();
      const targetTab = TABS[nextIndex];
      activeTab = targetTab.id;
      if (!isCollapsed) {
        visitedTabs[targetTab.id] = true;
      }
      tabButtonEls[nextIndex]?.focus();
    }
  }

  function toggleMaximize() {
    if (isCollapsed) {
      isCollapsed = false;
      visitedTabs[activeTab] = true;
      isMaximized = true;
    } else {
      isMaximized = !isMaximized;
    }
  }

  function toggleCollapse() {
    isCollapsed = !isCollapsed;
    if (isCollapsed) {
      isMaximized = false;
    } else {
      visitedTabs[activeTab] = true;
    }
  }

  function startResize(e: PointerEvent) {
    if (isCollapsed || isMaximized || e.button !== 0) return;
    e.preventDefault();
    isResizing = true;
    startY = e.clientY;
    startHeight = currentTabHeight;
    activePointerId = e.pointerId;
    try {
      (e.currentTarget as HTMLElement | null)?.setPointerCapture?.(e.pointerId);
    } catch {
      // Ignore synthetic PointerEvents without active pointer capture
    }
    window.addEventListener('pointermove', onPointerMove);
    window.addEventListener('pointerup', stopResize);
    window.addEventListener('pointercancel', stopResize);
    window.addEventListener('blur', stopResize);
  }

  function onPointerMove(e: PointerEvent) {
    if (!isResizing) return;
    if (activePointerId !== null && e.pointerId !== activePointerId) return;
    const delta = startY - e.clientY;
    setCurrentTabHeight(startHeight + delta);
  }

  function stopResize() {
    if (!isResizing) return;
    isResizing = false;
    activePointerId = null;
    window.removeEventListener('pointermove', onPointerMove);
    window.removeEventListener('pointerup', stopResize);
    window.removeEventListener('pointercancel', stopResize);
    window.removeEventListener('blur', stopResize);
  }

  function handleSplitterKeyDown(e: KeyboardEvent) {
    if (isMaximized) return;
    const step = e.shiftKey ? 48 : 20;
    if (e.key === 'ArrowUp') {
      e.preventDefault();
      setCurrentTabHeight(currentTabHeight + step);
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      setCurrentTabHeight(currentTabHeight - step);
    } else if (e.key === 'Home') {
      e.preventDefault();
      setCurrentTabHeight(maxPanelHeight);
    } else if (e.key === 'End') {
      e.preventDefault();
      setCurrentTabHeight(minTabHeight);
    }
  }

  onDestroy(() => {
    stopResize();
    if (typeof window !== 'undefined') {
      window.removeEventListener('resize', handleWindowResize);
    }
  });

  function openContextMenu(x: number, y: number, items: MenuItem[]) {
    contextMenu = { open: true, x, y, items };
  }

  function openCreateFolderModal(req: CreateFolderRequest) {
    createFolderModal = {
      open: true,
      kind: req.kind,
      folderId: req.folderId,
      parentSubfolder: req.parentSubfolder,
      locationLabel: req.locationLabel,
      oncreated: req.oncreated,
    };
  }

  function requestConfirm(req: ConfirmRequest) {
    confirmState = {
      open: true,
      title: req.title,
      message: req.message,
      confirmLabel: req.confirmLabel ?? 'Excluir',
      isProcessing: false,
      onconfirm: req.onconfirm,
    };
  }

  async function handleConfirmAction() {
    if (!confirmState.onconfirm) {
      confirmState.open = false;
      return;
    }
    confirmState.isProcessing = true;
    try {
      await confirmState.onconfirm();
      confirmState.open = false;
      confirmState.onconfirm = null;
    } finally {
      confirmState.isProcessing = false;
    }
  }

  function getTabCount(id: BottomTab): number | null {
    switch (id) {
      case 'songs':
        return songsCount;
      case 'images':
        return imagesCount;
      case 'videos':
        return videosCount;
      case 'audios':
        return audiosCount;
      case 'backgrounds':
        return backgroundsCount;
      case 'others':
        return visitedTabs.others || $customItemsStore.length > 0
          ? $customItemsStore.length
          : null;
      default:
        return null;
    }
  }
</script>

<div
  class={cn(
    'flex flex-col border-t border-border bg-card transition-[height] duration-150 select-none shrink-0',
    isResizing && 'transition-none'
  )}
  style={isCollapsed ? '' : `height: ${effectivePanelHeight}px;`}
>
  <!-- Top Resizer Bar when expanded -->
  {#if !isCollapsed}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
      role="separator"
      aria-orientation="horizontal"
      aria-label="Redimensionar painel inferior"
      aria-valuemin={minTabHeight}
      aria-valuemax={maxPanelHeight}
      aria-valuenow={Math.round(currentTabHeight)}
      aria-disabled={isMaximized}
      tabindex={isMaximized ? -1 : 0}
      onpointerdown={startResize}
      onkeydown={handleSplitterKeyDown}
      class={cn(
        'h-3.5 py-1 touch-none flex items-center justify-center group bg-border/35 transition-colors shrink-0 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/50',
        isMaximized
          ? 'cursor-default opacity-45'
          : 'cursor-ns-resize hover:bg-primary/20'
      )}
      title={isMaximized
        ? 'Restaure o painel para redimensionar'
        : 'Arraste ou use setas cima/baixo para redimensionar'}
    >
      <div
        class="w-14 h-1 rounded-full bg-muted-foreground/45 group-hover:bg-primary transition-colors pointer-events-none"
      ></div>
    </div>
  {/if}

  <!-- Tab Header Bar -->
  <div
    class="flex items-center justify-between gap-2 px-3 py-1 bg-card/85 border-b border-border shrink-0"
  >
    <div
      role="tablist"
      aria-label="Abas do painel de mídia"
      class="flex items-center gap-1 overflow-x-auto flex-1 min-w-0 py-0.5"
    >
      {#each TABS as tab, idx (tab.id)}
        {@const TabIcon = tab.icon}
        {@const count = getTabCount(tab.id)}
        {@const isSelected = activeTab === tab.id && !isCollapsed}
        <button
          bind:this={tabButtonEls[idx]}
          id={`tab-${tab.id}`}
          type="button"
          role="tab"
          aria-selected={isSelected}
          aria-controls={`panel-${tab.id}`}
          tabindex={activeTab === tab.id ? 0 : -1}
          onclick={() => selectTab(tab.id)}
          onkeydown={(e) => handleTabKeyDown(e, idx)}
          class={cn(
            'flex items-center gap-1.5 min-h-[32px] px-2.5 py-1 rounded-md text-xs font-medium whitespace-nowrap shrink-0 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
            isSelected
              ? 'bg-accent text-accent-foreground shadow-xs'
              : 'text-muted-foreground hover:text-foreground hover:bg-muted/60'
          )}
        >
          <TabIcon class={cn('w-3.5 h-3.5 shrink-0', tab.iconClass)} />
          <span>{tab.label}</span>
          {#if count !== null}
            <Badge variant="secondary" class="text-[11px] leading-none px-1.5 py-0.5 ml-0.5">
              {count}
            </Badge>
          {/if}
        </button>
      {/each}
    </div>

    <!-- Window Controls (kept strictly outside role="tablist") -->
    <div class="flex items-center gap-1 shrink-0">
      {#if !isCollapsed}
        <Button
          size="sm"
          variant="ghost"
          aria-pressed={isMaximized}
          aria-label={isMaximized ? 'Restaurar tamanho do painel' : 'Maximizar painel'}
          class="min-h-[32px] px-2 text-xs gap-1 text-muted-foreground hover:text-foreground"
          onclick={toggleMaximize}
        >
          {#if isMaximized}
            <Minimize2 class="w-3.5 h-3.5" />
            <span class="hidden sm:inline">Restaurar</span>
          {:else}
            <Maximize2 class="w-3.5 h-3.5" />
            <span class="hidden sm:inline">Maximizar</span>
          {/if}
        </Button>
      {/if}

      <Button
        size="sm"
        variant="ghost"
        aria-expanded={!isCollapsed}
        aria-label={isCollapsed ? 'Expandir painel inferior' : 'Recolher painel inferior'}
        class="min-h-[32px] px-2 text-xs gap-1 text-muted-foreground hover:text-foreground"
        onclick={toggleCollapse}
      >
        {#if isCollapsed}
          <ChevronUp class="w-3.5 h-3.5" />
          <span>Expandir</span>
        {:else}
          <ChevronDown class="w-3.5 h-3.5" />
          <span>Recolher</span>
        {/if}
      </Button>
    </div>
  </div>

  <!-- Tab Panels (visited tabs stay mounted via hidden class to preserve state without idle startup loads) -->
  {#if visitedTabs.songs}
    <div
      id="panel-songs"
      role="tabpanel"
      aria-labelledby="tab-songs"
      aria-hidden={isCollapsed || activeTab !== 'songs'}
      class={cn(
        'flex-1 flex flex-col min-w-0 overflow-hidden',
        (isCollapsed || activeTab !== 'songs') && 'hidden'
      )}
    >
      <SongsTab
        active={!isCollapsed && activeTab === 'songs'}
        {onaddsongtosetlist}
        {onpresentsong}
        {onopensongeditor}
        {onopeneditsong}
        onopencontextmenu={openContextMenu}
        onrequestconfirm={requestConfirm}
        oncountchange={(c) => {
          songsCount = c;
        }}
      />
    </div>
  {/if}

  {#if visitedTabs.bible}
    <div
      id="panel-bible"
      role="tabpanel"
      aria-labelledby="tab-bible"
      aria-hidden={isCollapsed || activeTab !== 'bible'}
      class={cn(
        'flex-1 flex flex-col min-w-0 overflow-hidden',
        (isCollapsed || activeTab !== 'bible') && 'hidden'
      )}
    >
      <BiblePanel
        targetPassage={targetScriptureRef}
        {onpresentscripture}
        {onaddscripturetosetlist}
        {onopenbibleconfig}
      />
    </div>
  {/if}

  {#if visitedTabs.images}
    <div
      id="panel-images"
      role="tabpanel"
      aria-labelledby="tab-images"
      aria-hidden={isCollapsed || activeTab !== 'images'}
      class={cn(
        'flex-1 flex flex-col min-w-0 overflow-hidden',
        (isCollapsed || activeTab !== 'images') && 'hidden'
      )}
    >
      <ImagesTab
        active={!isCollapsed && activeTab === 'images'}
        {folders}
        onfoldersrefresh={loadFolders}
        {onpresentimage}
        {onaddimagetosetlist}
        {onpreviewimage}
        {onopenfolderconfig}
        onopencreatefolder={openCreateFolderModal}
        oncountchange={(c) => {
          imagesCount = c;
        }}
      />
    </div>
  {/if}

  {#if visitedTabs.videos}
    <div
      id="panel-videos"
      role="tabpanel"
      aria-labelledby="tab-videos"
      aria-hidden={isCollapsed || activeTab !== 'videos'}
      class={cn(
        'flex-1 flex flex-col min-w-0 overflow-hidden',
        (isCollapsed || activeTab !== 'videos') && 'hidden'
      )}
    >
      <VideosTab
        active={!isCollapsed && activeTab === 'videos'}
        {folders}
        {onpresentitem}
        {onadditemtosetlist}
        {onopenfolderconfig}
        onopencreatefolder={openCreateFolderModal}
        oncountchange={(c) => {
          videosCount = c;
        }}
      />
    </div>
  {/if}

  {#if visitedTabs.audios}
    <div
      id="panel-audios"
      role="tabpanel"
      aria-labelledby="tab-audios"
      aria-hidden={isCollapsed || activeTab !== 'audios'}
      class={cn(
        'flex-1 flex flex-col min-w-0 overflow-hidden',
        (isCollapsed || activeTab !== 'audios') && 'hidden'
      )}
    >
      <AudiosTab
        active={!isCollapsed && activeTab === 'audios'}
        {onpresentitem}
        {onadditemtosetlist}
        {onopenfolderconfig}
        oncountchange={(c) => {
          audiosCount = c;
        }}
      />
    </div>
  {/if}

  {#if visitedTabs.backgrounds}
    <div
      id="panel-backgrounds"
      role="tabpanel"
      aria-labelledby="tab-backgrounds"
      aria-hidden={isCollapsed || activeTab !== 'backgrounds'}
      class={cn(
        'flex-1 flex flex-col min-w-0 overflow-hidden',
        (isCollapsed || activeTab !== 'backgrounds') && 'hidden'
      )}
    >
      <BackgroundsTab
        active={!isCollapsed && activeTab === 'backgrounds'}
        onopencontextmenu={openContextMenu}
        onrequestconfirm={requestConfirm}
        oncountchange={(c) => {
          backgroundsCount = c;
        }}
      />
    </div>
  {/if}

  {#if visitedTabs.others}
    <div
      id="panel-others"
      role="tabpanel"
      aria-labelledby="tab-others"
      aria-hidden={isCollapsed || activeTab !== 'others'}
      class={cn(
        'flex-1 flex flex-col min-w-0 overflow-hidden',
        (isCollapsed || activeTab !== 'others') && 'hidden'
      )}
    >
      <OthersTab
        active={!isCollapsed && activeTab === 'others'}
        {activeSetlistItems}
        {onadditemtosetlist}
        {onpresentitem}
        {onopenedititem}
        {onopenscheduleeditor}
        {onopentexteditor}
        {onopenquickitemeditor}
        {onopentimereditor}
        onopencontextmenu={openContextMenu}
        onrequestconfirm={requestConfirm}
      />
    </div>
  {/if}
</div>

<CreateFolderModal
  open={createFolderModal.open}
  kind={createFolderModal.kind}
  folderId={createFolderModal.folderId}
  parentSubfolder={createFolderModal.parentSubfolder}
  locationLabel={createFolderModal.locationLabel}
  oncreated={async (createdFolderId) => {
    await loadFolders();
    if (createFolderModal.oncreated) {
      await createFolderModal.oncreated(createdFolderId);
    }
  }}
  onclose={() => {
    createFolderModal.open = false;
  }}
/>

<ConfirmModal
  open={confirmState.open}
  title={confirmState.title}
  message={confirmState.message}
  confirmLabel={confirmState.confirmLabel}
  isProcessing={confirmState.isProcessing}
  onconfirm={handleConfirmAction}
  onclose={() => {
    confirmState.open = false;
    confirmState.onconfirm = null;
  }}
/>

<ContextMenu
  open={contextMenu.open}
  x={contextMenu.x}
  y={contextMenu.y}
  items={contextMenu.items}
  onclose={() => {
    contextMenu.open = false;
  }}
/>

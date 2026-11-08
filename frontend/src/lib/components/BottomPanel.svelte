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
    Copy,
    Pause,
    Volume2,
    VolumeX,
    Repeat,
    Hourglass,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import { cn } from '../utils';
  import BiblePanel from './BiblePanel.svelte';
  import ContextMenu, { type MenuItem } from './ContextMenu.svelte';
  import { renderFormattedText } from '../markdown';
  import { notifySuccess, notifyError } from '../toast';
  import {
    fetchVideos,
    uploadVideo,
    fetchAudios,
    uploadAudio,
    formatBytes,
    formatDuration,
    fetchAppSubfolders,
    createMediaDirectory,
    uploadImage,
  } from '../media';
  import {
    customItemsStore,
    loadCustomItems,
    saveOrUpdateCustomItem,
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
  import { presentationState, setPresentationWallpaper, updateAudioPlayback, clearAudioPlayback } from '../ws';
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
    VideoMediaItem,
    AudioMediaItem,
    MediaFolder,
    MediaSubfolder,
    WallpaperItem,
    ImageFitMode,
    MediaItem,
    ParsedReference,
  } from '../types';

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
    requestedTab?: 'songs' | 'images' | 'videos' | 'audios' | 'backgrounds' | 'bible' | 'others' | null;
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

  let isCollapsed = $state(true);
  let isMaximized = $state(false);
  let activeTab = $state<'songs' | 'images' | 'videos' | 'audios' | 'backgrounds' | 'bible' | 'others'>('songs');
  let panelHeight = $state(290);
  let isDragging = $state(false);
  let startY = 0;
  let startHeight = 290;

  function selectTab(tab: 'songs' | 'images' | 'videos' | 'audios' | 'backgrounds' | 'bible' | 'others') {
    if (tab === 'bible') {
      if (isCollapsed || activeTab !== 'bible') {
        const h75 = typeof window !== 'undefined' ? Math.round(window.innerHeight * 0.75) : 550;
        const maxH = typeof window !== 'undefined' ? window.innerHeight - 80 : 700;
        const targetH = Math.min(Math.max(panelHeight, h75), maxH);
        panelHeight = targetH;
      }
    } else if (tab === 'videos') {
      loadVideos();
    } else if (tab === 'audios') {
      loadAudios();
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
    loadVideos();
    loadAudios();
    loadFolders();
    loadWallpapers();
    loadCustomItems();
    loadAppImageSubfolders();
    loadAppVideoSubfolders();
  });

  // --- GERENCIAMENTO DE PASTAS FÍSICAS E FILE EXPLORER ---
  let appImageSubfolders = $state<MediaSubfolder[]>([]);
  let appVideoSubfolders = $state<MediaSubfolder[]>([]);
  let isCreateFolderModalOpen = $state(false);
  let newFolderName = $state('');
  let isCreatingFolder = $state(false);
  let folderInputEl: HTMLInputElement | null = $state(null);
  let isAppFolderExpanded = $state(true);

  async function loadAppImageSubfolders() {
    try {
      appImageSubfolders = await fetchAppSubfolders('images');
    } catch (err) {
      console.warn('Erro ao carregar subpastas de imagens:', err);
    }
  }

  async function loadAppVideoSubfolders() {
    try {
      appVideoSubfolders = await fetchAppSubfolders('videos');
    } catch (err) {
      console.warn('Erro ao carregar subpastas de vídeos:', err);
    }
  }

  function getDirectSubfoldersForLevel(
    allSubfolders: MediaSubfolder[],
    currentLevel: string | null
  ): MediaSubfolder[] {
    const map = new Map<string, { relative_path: string; name: string; count: number }>();
    const prefix = currentLevel ? `${currentLevel}/` : '';

    for (const sub of allSubfolders) {
      if (currentLevel === null) {
        const topSegment = sub.relative_path.split('/')[0];
        const existing = map.get(topSegment);
        if (!existing) {
          map.set(topSegment, {
            relative_path: topSegment,
            name: topSegment,
            count: sub.relative_path === topSegment ? sub.count : 0,
          });
        } else if (sub.relative_path === topSegment) {
          existing.count = sub.count;
        }
      } else if (sub.relative_path.startsWith(prefix)) {
        const remainder = sub.relative_path.slice(prefix.length);
        const directName = remainder.split('/')[0];
        const directRelPath = `${prefix}${directName}`;
        const existing = map.get(directRelPath);
        if (!existing) {
          map.set(directRelPath, {
            relative_path: directRelPath,
            name: directName,
            count: sub.relative_path === directRelPath ? sub.count : 0,
          });
        } else if (sub.relative_path === directRelPath) {
          existing.count = sub.count;
        }
      }
    }

    return Array.from(map.values()).sort((a, b) => a.name.localeCompare(b.name));
  }

  function openCreateFolderModal() {
    newFolderName = '';
    isCreateFolderModalOpen = true;
    setTimeout(() => {
      folderInputEl?.focus();
    }, 50);
  }

  const createFolderLocationLabel = $derived.by(() => {
    const isVideo = activeTab === 'videos';
    if (isVideo) {
      const folderName = videoFolderFilter === 'all' || videoFolderFilter === 'app' ? 'Pasta do App (Vídeos)' : videoFolderFilter;
      return selectedVideoSubfolder ? `${folderName} / ${selectedVideoSubfolder}` : folderName;
    } else {
      const folderName = selectedFolderFilter === 'all' || selectedFolderFilter === 'app' ? 'Pasta do App (Imagens)' : currentFolderName;
      return selectedSubfolder ? `${folderName} / ${selectedSubfolder}` : folderName;
    }
  });

  async function handleCreateFolderSubmit() {
    const name = newFolderName.trim();
    if (!name) return;

    isCreatingFolder = true;
    try {
      const isVideo = activeTab === 'videos';
      const kind = isVideo ? 'videos' : 'images';
      const folderId = isVideo
        ? (videoFolderFilter !== 'all' ? videoFolderFilter : 'app')
        : (selectedFolderFilter !== 'all' ? selectedFolderFilter : 'app');
      const parentSubfolder = isVideo ? selectedVideoSubfolder : selectedSubfolder;

      await createMediaDirectory(name, kind, folderId, parentSubfolder);
      notifySuccess('Pasta Criada', `A pasta "${name}" foi criada com sucesso.`);

      if (!isVideo && selectedFolderFilter === 'all') {
        selectedFolderFilter = 'app';
      } else if (isVideo && videoFolderFilter === 'all') {
        videoFolderFilter = 'app';
      }

      newFolderName = '';
      isCreateFolderModalOpen = false;

      if (isVideo) {
        await loadVideos();
        await loadAppVideoSubfolders();
      } else {
        await loadImages();
        await loadFolders();
        await loadAppImageSubfolders();
      }
    } catch (err: any) {
      console.error('Erro ao criar pasta:', err);
      notifyError('Erro ao Criar Pasta', err?.message || 'Falha ao criar diretório.');
    } finally {
      isCreatingFolder = false;
    }
  }

  // --- GALERIA DE VÍDEOS ---
  let videos = $state<VideoMediaItem[]>([]);
  let videoSearch = $state('');
  let videoFolderFilter = $state<string>('all');
  let selectedVideoSubfolder = $state<string | null>(null);
  let isLoadingVideos = $state(false);
  let isUploadingVideo = $state(false);
  let videoFileInputEl: HTMLInputElement | null = $state(null);

  async function loadVideos() {
    isLoadingVideos = true;
    try {
      videos = await fetchVideos();
      await loadAppVideoSubfolders();
    } catch (err) {
      console.error('Erro ao carregar vídeos:', err);
    } finally {
      isLoadingVideos = false;
    }
  }

  const videoFoldersList = $derived.by(() => {
    const set = new Set<string>();
    for (const v of videos) {
      if (v.folder_name) set.add(v.folder_name);
    }
    return Array.from(set);
  });

  const allAppVideoSubfolders = $derived.by(() => {
    const map = new Map<string, MediaSubfolder>();
    for (const s of appVideoSubfolders) {
      map.set(s.relative_path, { ...s });
    }
    for (const vid of videos) {
      if ((!vid.folder_name || vid.folder_id === 'app') && vid.subfolder) {
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
    if (videoFolderFilter === 'app') {
      return allAppVideoSubfolders;
    }
    const f = folders.find((item) => item.name === videoFolderFilter || item.id === videoFolderFilter);
    return f?.subfolders || [];
  });

  const currentVideoDirectSubfolders = $derived.by(() => {
    if (videoFolderFilter === 'all') return [];
    return getDirectSubfoldersForLevel(allCurrentVideoSubfolders, selectedVideoSubfolder);
  });

  const videoBreadcrumbs = $derived.by(() => {
    if (!selectedVideoSubfolder) return [];
    const parts = selectedVideoSubfolder.split('/');
    return parts.map((part, index) => ({
      name: part,
      path: parts.slice(0, index + 1).join('/'),
    }));
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

  const currentVideoFolderName = $derived.by(() => {
    if (videoFolderFilter === 'all') return 'Todos os Vídeos';
    if (videoFolderFilter === 'app') return 'Pasta do App';
    return videoFolderFilter;
  });

  const filteredVideos = $derived.by(() => {
    let list = videos;
    if (videoFolderFilter === 'app') {
      list = list.filter((v) => !v.folder_name || v.folder_id === 'app');
    } else if (videoFolderFilter !== 'all') {
      list = list.filter((v) => v.folder_name === videoFolderFilter || v.folder_id === videoFolderFilter);
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

  async function handleVideoFilesSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    isUploadingVideo = true;
    try {
      const reader = new FileReader();
      reader.onload = async () => {
        try {
          const b64 = (reader.result as string).split(',')[1];
          const folderId = videoFolderFilter !== 'all' ? videoFolderFilter : 'app';
          const subfolder = selectedVideoSubfolder || undefined;
          await uploadVideo(file.name, b64, folderId, subfolder);
          notifySuccess('Vídeo Importado', `"${file.name}" foi salvo na biblioteca de vídeos.`);
          await loadVideos();
          await loadAppVideoSubfolders();
        } catch (err) {
          notifyError('Erro no Upload', 'Falha ao salvar arquivo de vídeo.');
        } finally {
          isUploadingVideo = false;
        }
      };
      reader.readAsDataURL(file);
    } catch (err) {
      isUploadingVideo = false;
      notifyError('Erro no Upload', 'Falha ao processar arquivo de vídeo.');
    }
    input.value = '';
  }

  function handlePresentVideo(video: VideoMediaItem) {
    const item: MediaItem = {
      id: `video-${Date.now()}`,
      title: video.name,
      artist_or_ref: video.folder_name || 'Vídeo',
      kind: 'video',
      slides: [
        {
          id: 'slide-1',
          label: 'Vídeo',
          content: video.url,
        },
      ],
    };
    onpresentitem?.(item, 0);
  }

  function handleAddVideoToSetlist(video: VideoMediaItem) {
    const item: MediaItem = {
      id: `video-${Date.now()}`,
      title: video.name,
      artist_or_ref: video.folder_name || 'Vídeo',
      kind: 'video',
      slides: [
        {
          id: 'slide-1',
          label: 'Vídeo',
          content: video.url,
        },
      ],
    };
    onadditemtosetlist?.(item);
    notifySuccess('Item Adicionado', `"${video.name}" adicionado à setlist.`);
  }

  // --- GALERIA DE ÁUDIOS ---
  let audios = $state<AudioMediaItem[]>([]);
  let audioSearch = $state('');
  let audioFolderFilter = $state<string>('all');
  let isLoadingAudios = $state(false);
  let isUploadingAudio = $state(false);
  let audioFileInputEl: HTMLInputElement | null = $state(null);
  let lastBroadcastAudioSec = -1;

  // Player de Áudio Embutido
  let playingAudio = $state<AudioMediaItem | null>(null);
  let isAudioPlaying = $state(false);
  let audioCurrentTime = $state(0);
  let audioDuration = $state(0);
  let audioVolume = $state(1.0);
  let isAudioMuted = $state(false);
  let isAudioLoop = $state(false);
  let audioElement: HTMLAudioElement | null = $state(null);

  async function loadAudios() {
    isLoadingAudios = true;
    try {
      audios = await fetchAudios();
    } catch (err) {
      console.error('Erro ao carregar áudios:', err);
    } finally {
      isLoadingAudios = false;
    }
  }

  const audioFoldersList = $derived.by(() => {
    const set = new Set<string>();
    for (const a of audios) {
      if (a.folder_name) set.add(a.folder_name);
    }
    return Array.from(set);
  });

  const filteredAudios = $derived.by(() => {
    let list = audios;
    if (audioFolderFilter === 'app') {
      list = list.filter((a) => !a.folder_name);
    } else if (audioFolderFilter !== 'all') {
      list = list.filter((a) => a.folder_name === audioFolderFilter);
    }
    const q = audioSearch.trim().toLowerCase();
    if (!q) return list;
    return list.filter(
      (a) =>
        a.name.toLowerCase().includes(q) ||
        (a.folder_name && a.folder_name.toLowerCase().includes(q))
    );
  });

  function syncAudioWs(playing: boolean, curTime: number, dur: number) {
    if (!playingAudio) return;
    const secFloor = Math.floor(curTime);
    if (playing !== isAudioPlaying || secFloor !== lastBroadcastAudioSec) {
      lastBroadcastAudioSec = secFloor;
      updateAudioPlayback(
        playing,
        curTime,
        dur || 0,
        playingAudio.name,
        playingAudio.folder_name
      );
    }
  }

  function handlePlayAudio(audio: AudioMediaItem) {
    if (playingAudio?.id === audio.id) {
      if (audioElement) {
        if (isAudioPlaying) {
          audioElement.pause();
          isAudioPlaying = false;
          syncAudioWs(false, audioElement.currentTime, audioElement.duration || 0);
        } else {
          audioElement.play().then(() => {
            isAudioPlaying = true;
            syncAudioWs(true, audioElement.currentTime, audioElement.duration || 0);
          }).catch(console.error);
        }
      }
      return;
    }
    playingAudio = audio;
    if (audioElement) {
      audioElement.src = audio.url;
      audioElement.currentTime = 0;
      audioElement.play().then(() => {
        isAudioPlaying = true;
        syncAudioWs(true, 0, audioElement?.duration || 0);
      }).catch(console.error);
    }
  }

  function handleStopAudio() {
    if (audioElement) {
      audioElement.pause();
      audioElement.currentTime = 0;
    }
    isAudioPlaying = false;
    audioCurrentTime = 0;
    lastBroadcastAudioSec = -1;
    clearAudioPlayback();
  }

  function handleAudioTimeUpdate() {
    if (audioElement) {
      audioCurrentTime = audioElement.currentTime;
      audioDuration = audioElement.duration || 0;
      if (isAudioPlaying && playingAudio) {
        const secFloor = Math.floor(audioCurrentTime);
        if (secFloor !== lastBroadcastAudioSec) {
          lastBroadcastAudioSec = secFloor;
          updateAudioPlayback(
            true,
            audioCurrentTime,
            audioDuration,
            playingAudio.name,
            playingAudio.folder_name
          );
        }
      }
    }
  }

  function handleAudioEnded() {
    if (!isAudioLoop) {
      isAudioPlaying = false;
      audioCurrentTime = 0;
      lastBroadcastAudioSec = -1;
      clearAudioPlayback();
    }
  }

  function handleSeekAudio(e: Event) {
    const val = Number((e.target as HTMLInputElement).value);
    if (audioElement) {
      audioElement.currentTime = val;
      audioCurrentTime = val;
      if (playingAudio) {
        syncAudioWs(isAudioPlaying, val, audioDuration);
      }
    }
  }

  function handlePresentAudio(audio: AudioMediaItem) {
    handlePlayAudio(audio);
    const item: MediaItem = {
      id: `audio-${Date.now()}`,
      title: audio.name,
      artist_or_ref: audio.folder_name || 'Áudio',
      kind: 'audio',
      slides: [
        {
          id: 'slide-1',
          label: 'Áudio',
          content: audio.url,
        },
      ],
    };
    onpresentitem?.(item, 0);
  }

  function handleVolumeChange(e: Event) {
    const val = Number((e.target as HTMLInputElement).value);
    audioVolume = val;
    if (audioElement) {
      audioElement.volume = val;
      isAudioMuted = val === 0;
    }
  }

  function toggleAudioMute() {
    if (audioElement) {
      if (isAudioMuted) {
        audioElement.muted = false;
        isAudioMuted = false;
        if (audioVolume === 0) {
          audioVolume = 0.5;
          audioElement.volume = 0.5;
        }
      } else {
        audioElement.muted = true;
        isAudioMuted = true;
      }
    }
  }

  function toggleAudioLoop() {
    isAudioLoop = !isAudioLoop;
    if (audioElement) {
      audioElement.loop = isAudioLoop;
    }
  }

  async function handleAudioFilesSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    isUploadingAudio = true;
    try {
      const reader = new FileReader();
      reader.onload = async () => {
        try {
          const b64 = (reader.result as string).split(',')[1];
          await uploadAudio(file.name, b64);
          notifySuccess('Áudio Importado', `"${file.name}" foi salvo na biblioteca de áudios.`);
          await loadAudios();
        } catch (err) {
          notifyError('Erro no Upload', 'Falha ao salvar arquivo de áudio.');
        } finally {
          isUploadingAudio = false;
        }
      };
      reader.readAsDataURL(file);
    } catch (err) {
      isUploadingAudio = false;
      notifyError('Erro no Upload', 'Falha ao processar arquivo de áudio.');
    }
    input.value = '';
  }

  function handleAddAudioToSetlist(audio: AudioMediaItem) {
    const item: MediaItem = {
      id: `audio-${Date.now()}`,
      title: audio.name,
      artist_or_ref: audio.folder_name || 'Áudio',
      kind: 'audio',
      slides: [
        {
          id: 'slide-1',
          label: 'Áudio',
          content: audio.url,
        },
      ],
    };
    onadditemtosetlist?.(item);
    notifySuccess('Item Adicionado', `"${audio.name}" adicionado à setlist.`);
  }

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
      await loadAppImageSubfolders();
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

  // Subpastas completas da pasta do app (incluindo diretórios físicos vazios)
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

  const allCurrentFolderSubfolders = $derived.by(() => {
    if (selectedFolderFilter === 'all') return [];
    if (selectedFolderFilter === 'app') {
      return allAppImageSubfolders;
    }
    const f = folders.find((item) => item.id === selectedFolderFilter);
    return f?.subfolders || [];
  });

  // Sub-pastas diretas do nível atual (File Explorer)
  const currentSubfolders = $derived.by(() => {
    if (selectedFolderFilter === 'all') return [];
    return getDirectSubfoldersForLevel(allCurrentFolderSubfolders, selectedSubfolder);
  });

  const imageBreadcrumbs = $derived.by(() => {
    if (!selectedSubfolder) return [];
    const parts = selectedSubfolder.split('/');
    return parts.map((part, index) => ({
      name: part,
      path: parts.slice(0, index + 1).join('/'),
    }));
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

  // Filtragem de imagens com suporte a sub-pastas e privacidade
  const filteredImages = $derived.by(() => {
    let list = images;
    // Por padrão (privacidade ativa), exibe apenas pastas importadas externas quando não na pasta do app
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

  // Upload de Imagem para pasta ativa e subpasta atual
  async function handleFileUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];

    const reader = new FileReader();
    reader.onload = async () => {
      const fullBase64 = reader.result as string;
      const b64 = fullBase64.includes(',') ? fullBase64.split(',')[1] : fullBase64;
      try {
        const folderId = selectedFolderFilter !== 'all' ? selectedFolderFilter : 'app';
        const subfolder = selectedSubfolder || undefined;
        await uploadImage(file.name, b64, folderId, subfolder);
        notifySuccess('Imagem Importada', `"${file.name}" foi importada com sucesso.`);
        await loadImages();
        await loadAppImageSubfolders();
      } catch (err) {
        console.error('Erro no upload de imagem:', err);
        notifyError('Erro no Upload', 'Falha ao salvar arquivo de imagem.');
      }
    };
    reader.readAsDataURL(file);
    input.value = '';
  }

  // --- ABA OUTROS (AGENDA, TEXTOS, AVISOS, SEÇÕES) ---
  let othersSearch = $state('');
  let othersKindFilter = $state<'all' | 'schedule' | 'text' | 'announcement' | 'header' | 'timer'>('all');
  let viewingCustomItem = $state<MediaItem | null>(null);
  let isImportingFromSetlist = $state(false);
  let isAddOtherMenuOpen = $state(false);

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
      if (viewingCustomItem && !filteredCustomItems.some((i) => i.id === viewingCustomItem?.id)) {
        viewingCustomItem = null;
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

  // Menu de Contexto para a Aba Outros
  let contextMenuOpen = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuItems = $state<MenuItem[]>([]);

  async function handleDuplicateCustomItem(item: MediaItem) {
    const newItem: MediaItem = {
      ...JSON.parse(JSON.stringify(item)),
      id: `${item.kind || 'item'}-${Date.now()}`,
      title: `${item.title} (Cópia)`,
      slides: item.slides.map((s, idx) => ({
        ...s,
        id: `${item.kind || 'slide'}-${Date.now()}-${idx + 1}`,
      })),
    };
    await saveOrUpdateCustomItem(newItem);
    viewingCustomItem = newItem;
    notifySuccess('Item Duplicado', `"${newItem.title}" criado com sucesso.`);
  }

  function handleOtherItemContextMenu(e: MouseEvent, item: MediaItem) {
    e.preventDefault();
    e.stopPropagation();

    viewingCustomItem = item;
    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'other-present-now',
        label: 'Apresentar Agora (Ao Vivo)',
        icon: Play,
        onclick: () => onpresentitem?.(item, 0),
      },
      {
        id: 'other-add-setlist',
        label: 'Adicionar à Setlist',
        icon: ListPlus,
        onclick: () => onadditemtosetlist?.(item),
      },
      { id: 'div-other-1', divider: true },
      {
        id: 'other-edit',
        label: 'Editar Item',
        icon: Pencil,
        onclick: () => onopenedititem?.(item),
      },
      {
        id: 'other-dup',
        label: 'Duplicar Item',
        icon: Copy,
        onclick: () => handleDuplicateCustomItem(item),
      },
      { id: 'div-other-2', divider: true },
      {
        id: 'other-del',
        label: 'Excluir do Catálogo',
        icon: Trash2,
        destructive: true,
        onclick: () => handleDeleteCustomItemClick(item, e),
      },
    ];

    contextMenuOpen = true;
  }

  function handleOtherSlideContextMenu(e: MouseEvent, item: MediaItem, slide: Slide, idx: number) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'slide-title',
        label: `${slide.label || `Slide ${idx + 1}`}`,
        disabled: true,
      },
      { id: 'div-slide-0', divider: true },
      {
        id: 'slide-present-live',
        label: 'Apresentar Este Slide Ao Vivo',
        icon: Play,
        onclick: () => onpresentitem?.(item, idx),
      },
      {
        id: 'slide-add-setlist',
        label: 'Adicionar Apresentação à Setlist',
        icon: ListPlus,
        onclick: () => onadditemtosetlist?.(item),
      },
      { id: 'div-slide-1', divider: true },
      {
        id: 'slide-edit-item',
        label: 'Editar Apresentação',
        icon: Pencil,
        onclick: () => onopenedititem?.(item),
      },
    ];

    contextMenuOpen = true;
  }

  function handleOtherEmptyContextMenu(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'empty-header',
        label: 'Aba Outros',
        disabled: true,
      },
      { id: 'div-empty-0', divider: true },
      {
        id: 'create-schedule',
        label: 'Nova Programação (Agenda)',
        icon: CalendarDays,
        onclick: () => onopenscheduleeditor?.(),
      },
      {
        id: 'create-text',
        label: 'Nova Apresentação de Texto',
        icon: FileText,
        onclick: () => onopentexteditor?.(),
      },
      {
        id: 'create-announcement',
        label: 'Novo Aviso / Comunicado',
        icon: Bookmark,
        onclick: () => onopenquickitemeditor?.('announcement'),
      },
      { id: 'div-empty-1', divider: true },
      {
        id: 'reload-catalog',
        label: 'Recarregar Catálogo',
        icon: RefreshCw,
        onclick: () => loadCustomItems(),
      },
    ];

    contextMenuOpen = true;
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
      case 'timer':
        return { label: 'Cronômetro', color: 'border-rose-500/40 bg-rose-500/10 text-rose-300' };
      case 'video':
        return { label: 'Vídeo', color: 'border-purple-500/40 bg-purple-500/10 text-purple-300' };
      case 'audio':
        return { label: 'Áudio', color: 'border-indigo-500/40 bg-indigo-500/10 text-indigo-300' };
      default:
        return { label: 'Outro', color: 'border-neutral-500/40 bg-neutral-500/10 text-neutral-300' };
    }
  }

  async function handleDeleteWallpaper(wall: WallpaperItem) {
    if (wall.is_default) {
      notifyError('Ação Bloqueada', 'Papéis de parede padrão do sistema não podem ser excluídos.');
      return;
    }

    const confirmed = confirm(
      `Deseja realmente excluir o papel de parede "${wall.name}"?\nEsta ação removerá o arquivo permanentemente.`
    );
    if (!confirmed) return;

    try {
      const res = await fetch(`/api/media/wallpapers/${encodeURIComponent(wall.name)}`, {
        method: 'DELETE',
      });
      if (!res.ok) {
        const text = await res.text();
        throw new Error(text || 'Falha ao excluir papel de parede');
      }
      notifySuccess('Papel de Parede Removido', `"${wall.name}" foi excluído com sucesso.`);
      await loadWallpapers();
    } catch (err: any) {
      notifyError('Erro ao Excluir', err.message || 'Não foi possível excluir o papel de parede.');
    }
  }

  function handleLocalWallpaperContextMenu(e: MouseEvent, wall: WallpaperItem) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'wall-name',
        label: wall.name,
        disabled: true,
      },
      { id: 'div-wall-0', divider: true },
      {
        id: 'wall-use-lyrics',
        label: 'Usar como Fundo das Letras (Ao Vivo)',
        icon: Play,
        onclick: () => applyBackground(wall.url),
      },
      {
        id: 'wall-copy-url',
        label: 'Copiar URL do Arquivo',
        icon: Copy,
        onclick: async () => {
          try {
            await navigator.clipboard.writeText(wall.url);
            notifySuccess('URL Copiada', 'Caminho do arquivo copiado para a área de transferência.');
          } catch {
            notifyError('Erro', 'Não foi possível copiar para a área de transferência.');
          }
        },
      },
      { id: 'div-wall-1', divider: true },
      ...(wall.is_default
        ? [
            {
              id: 'wall-protected',
              label: 'Papel de Parede Padrão (Protegido)',
              icon: ShieldCheck,
              disabled: true,
            },
          ]
        : [
            {
              id: 'wall-delete',
              label: 'Excluir Papel de Parede Local',
              icon: Trash2,
              destructive: true,
              onclick: () => handleDeleteWallpaper(wall),
            },
          ]),
    ];

    contextMenuOpen = true;
  }

  function handlePresetContextMenu(e: MouseEvent, preset: BackgroundPreset) {
    e.preventDefault();
    e.stopPropagation();

    const encoded = encodeBackground(preset);
    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'preset-name',
        label: preset.name,
        disabled: true,
      },
      { id: 'div-preset-0', divider: true },
      {
        id: 'preset-use-lyrics',
        label: 'Usar como Fundo das Letras (Ao Vivo)',
        icon: Play,
        onclick: () => applyBackground(encoded),
      },
      {
        id: 'preset-copy-css',
        label: 'Copiar Código CSS',
        icon: Copy,
        onclick: async () => {
          try {
            await navigator.clipboard.writeText(preset.style);
            notifySuccess('Código CSS Copiado', 'Estilo CSS copiado para a área de transferência.');
          } catch {
            notifyError('Erro', 'Não foi possível copiar o CSS.');
          }
        },
      },
    ];

    contextMenuOpen = true;
  }

  function handleBackgroundsEmptyContextMenu(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'bg-empty-header',
        label: 'Fundos & Gradientes',
        disabled: true,
      },
      { id: 'div-bg-empty-0', divider: true },
      {
        id: 'bg-import',
        label: 'Importar Fundo (Imagem / Vídeo)...',
        icon: Upload,
        onclick: () => triggerImportWallpaper(),
      },
      {
        id: 'bg-reload',
        label: 'Recarregar Fundos Locais',
        icon: RefreshCw,
        onclick: async () => {
          await loadWallpapers();
          notifySuccess('Fundos Atualizados', 'Lista de papéis de parede recarregada.');
        },
      },
      { id: 'div-bg-empty-1', divider: true },
      {
        id: 'bg-clear-lyrics',
        label: 'Remover Fundo Ativo das Letras',
        icon: X,
        destructive: true,
        onclick: () => {
          setPresentationWallpaper('');
          notifySuccess('Fundo Removido', 'Fundo personalizado de letras foi desativado.');
        },
      },
    ];

    contextMenuOpen = true;
  }

  async function handleCopySongLyrics(songId: string) {
    try {
      let lyrics = '';
      if (viewingSong && viewingSong.id === songId) {
        lyrics = viewingSong.lyrics_raw;
      } else {
        const res = await fetch(`/api/songs/${songId}`);
        if (res.ok) {
          const s: Song = await res.json();
          lyrics = s.lyrics_raw;
        }
      }
      if (lyrics) {
        await navigator.clipboard.writeText(lyrics);
        notifySuccess('Letra Copiada', 'Letra da música copiada para a área de transferência.');
      } else {
        notifyError('Erro', 'Letra da música não encontrada.');
      }
    } catch {
      notifyError('Erro', 'Falha ao copiar letra para a área de transferência.');
    }
  }

  async function handleCopySlideContent(content: string) {
    try {
      await navigator.clipboard.writeText(content);
      notifySuccess('Texto Copiado', 'Conteúdo do slide copiado para a área de transferência.');
    } catch {
      notifyError('Erro', 'Falha ao copiar texto do slide.');
    }
  }

  async function handleDeleteSongClick(song: SongSummary | Song, e?: MouseEvent) {
    if (e) e.stopPropagation();
    const confirmed = confirm(
      `Deseja realmente excluir a música "${song.title}" do catálogo de músicas?\nEsta ação não poderá ser desfeita.`
    );
    if (!confirmed) return;

    try {
      const res = await fetch(`/api/songs/${song.id}`, { method: 'DELETE' });
      if (res.ok) {
        notifySuccess('Música Excluída', `"${song.title}" foi removida do catálogo.`);
        if (viewingSong?.id === song.id) {
          viewingSong = null;
        }
        await loadSongs();
        await loadSongTags();
        if (typeof window !== 'undefined') {
          window.dispatchEvent(new CustomEvent('openpresenter:songs-changed'));
        }
      } else {
        const txt = await res.text();
        notifyError('Erro ao Excluir', txt || 'Não foi possível excluir a música.');
      }
    } catch {
      notifyError('Erro de Conexão', 'Falha ao se comunicar com o servidor.');
    }
  }

  function handleSongContextMenu(e: MouseEvent, song: SongSummary) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    const artistText = song.artist ? ` • ${song.artist}` : '';
    const keyText = song.song_key ? ` (${song.song_key})` : '';

    contextMenuItems = [
      {
        id: 'song-header',
        label: `${song.title}${artistText}${keyText}`,
        disabled: true,
      },
      { id: 'div-song-0', divider: true },
      {
        id: 'song-present-live',
        label: 'Apresentar Agora (Ao Vivo)',
        icon: Play,
        onclick: () => onpresentsong?.(song.id, 0),
      },
      {
        id: 'song-add-setlist',
        label: 'Adicionar à Setlist',
        icon: ListPlus,
        onclick: () => onaddsongtosetlist?.(song.id),
      },
      {
        id: 'song-view-lyrics',
        label: viewingSong?.id === song.id ? 'Visualizando Letra' : 'Ver Letra / Slides',
        icon: FileText,
        onclick: () => handleViewSongLyrics(song.id),
      },
      { id: 'div-song-1', divider: true },
      {
        id: 'song-edit',
        label: 'Editar Música',
        icon: Pencil,
        onclick: () => onopeneditsong?.(song.id),
      },
      {
        id: 'song-copy-lyrics',
        label: 'Copiar Letra Completa',
        icon: Copy,
        onclick: () => handleCopySongLyrics(song.id),
      },
      { id: 'div-song-2', divider: true },
      {
        id: 'song-delete',
        label: 'Excluir do Catálogo',
        icon: Trash2,
        destructive: true,
        onclick: () => handleDeleteSongClick(song, e),
      },
    ];

    contextMenuOpen = true;
  }

  function handleViewingSongSlideContextMenu(e: MouseEvent, song: Song, slide: Slide, idx: number) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'slide-header',
        label: `${song.title} — ${slide.label || `Slide ${idx + 1}`}`,
        disabled: true,
      },
      { id: 'div-slide-0', divider: true },
      {
        id: 'slide-present-live',
        label: 'Apresentar Este Slide Ao Vivo',
        icon: Play,
        onclick: () => onpresentsong?.(song.id, idx),
      },
      {
        id: 'slide-add-setlist',
        label: 'Adicionar Música à Setlist',
        icon: ListPlus,
        onclick: () => onaddsongtosetlist?.(song.id),
      },
      { id: 'div-slide-1', divider: true },
      {
        id: 'slide-copy-text',
        label: 'Copiar Texto Deste Slide',
        icon: Copy,
        onclick: () => handleCopySlideContent(slide.content),
      },
      {
        id: 'slide-edit-song',
        label: 'Editar Música',
        icon: Pencil,
        onclick: () => onopeneditsong?.(song.id),
      },
    ];

    contextMenuOpen = true;
  }

  function handleViewingSongSheetContextMenu(e: MouseEvent, song: Song) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    const artistText = song.artist ? ` • ${song.artist}` : '';
    const keyText = song.song_key ? ` (${song.song_key})` : '';

    contextMenuItems = [
      {
        id: 'sheet-header',
        label: `${song.title}${artistText}${keyText}`,
        disabled: true,
      },
      { id: 'div-sheet-0', divider: true },
      {
        id: 'sheet-present-start',
        label: 'Apresentar do Início (Ao Vivo)',
        icon: Play,
        onclick: () => onpresentsong?.(song.id, 0),
      },
      {
        id: 'sheet-add-setlist',
        label: 'Adicionar à Setlist',
        icon: ListPlus,
        onclick: () => onaddsongtosetlist?.(song.id),
      },
      { id: 'div-sheet-1', divider: true },
      {
        id: 'sheet-edit-song',
        label: 'Editar Música',
        icon: Pencil,
        onclick: () => onopeneditsong?.(song.id),
      },
      {
        id: 'sheet-copy-full-lyrics',
        label: 'Copiar Letra Completa',
        icon: Copy,
        onclick: () => handleCopySongLyrics(song.id),
      },
      { id: 'div-sheet-2', divider: true },
      {
        id: 'sheet-delete-song',
        label: 'Excluir do Catálogo',
        icon: Trash2,
        destructive: true,
        onclick: () => handleDeleteSongClick(song, e),
      },
      {
        id: 'sheet-close',
        label: 'Fechar Visualização',
        icon: X,
        onclick: () => (viewingSong = null),
      },
    ];

    contextMenuOpen = true;
  }

  function handleSongsEmptyContextMenu(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    contextMenuItems = [
      {
        id: 'songs-empty-header',
        label: 'Catálogo de Músicas',
        disabled: true,
      },
      { id: 'div-songs-empty-0', divider: true },
      {
        id: 'songs-new',
        label: 'Nova Música',
        icon: Plus,
        onclick: () => onopensongeditor?.(),
      },
      {
        id: 'songs-import',
        label: 'Importar Músicas (.json, .txt, .md)...',
        icon: Upload,
        onclick: () => triggerSongImport(),
      },
      { id: 'div-songs-empty-1', divider: true },
      {
        id: 'songs-reload',
        label: 'Recarregar Catálogo',
        icon: RefreshCw,
        onclick: () => {
          loadSongs();
          loadSongTags();
        },
      },
    ];

    contextMenuOpen = true;
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

      <!-- Aba Vídeos -->
      <button
        type="button"
        role="tab"
        id="tab-videos"
        aria-controls="panel-videos"
        aria-selected={activeTab === 'videos' && !isCollapsed}
        onclick={() => selectTab('videos')}
        class={cn(
          'flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none',
          activeTab === 'videos' && !isCollapsed
            ? 'bg-muted text-foreground shadow-xs border border-border'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'
        )}
      >
        <Film size={13} class="text-purple-400" />
        <span>Vídeos</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {videos.length}
        </Badge>
      </button>

      <!-- Aba Áudios -->
      <button
        type="button"
        role="tab"
        id="tab-audios"
        aria-controls="panel-audios"
        aria-selected={activeTab === 'audios' && !isCollapsed}
        onclick={() => selectTab('audios')}
        class={cn(
          'flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none',
          activeTab === 'audios' && !isCollapsed
            ? 'bg-muted text-foreground shadow-xs border border-border'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'
        )}
      >
        <Volume2 size={13} class="text-indigo-400" />
        <span>Áudios</span>
        <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
          {audios.length}
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
            oncontextmenu={(e) => handleSongContextMenu(e, song)}
            role="button"
            tabindex="0"
            class="p-2.5 rounded-lg border border-border bg-card/60 hover:bg-muted/40 transition-all flex flex-col justify-between group cursor-pointer {viewingSong?.id === song.id ? 'ring-1 ring-primary border-primary bg-primary/10' : ''}"
          >
            <div>
              <div class="flex items-center justify-between gap-1 mb-1">
                <h4 class="text-xs font-bold text-foreground truncate">{song.title}</h4>
                <div class="flex items-center gap-1 shrink-0">
                  {#if song.audio_url}
                    <Badge variant="outline" class="text-[9px] py-0 px-1 font-mono text-emerald-400 border-emerald-500/40 bg-emerald-950/40 flex items-center gap-0.5">
                      <Music size={9} />
                      Áudio
                    </Badge>
                  {/if}
                  {#if song.song_key}
                    <Badge variant="outline" class="text-[9px] py-0 px-1 font-mono text-primary">
                      {song.song_key}
                    </Badge>
                  {/if}
                </div>
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
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              oncontextmenu={(e) => viewingSong && handleViewingSongSheetContextMenu(e, viewingSong)}
              class="flex-1 min-w-0 border border-border bg-card/60 rounded-xl flex flex-col p-3.5 h-full overflow-hidden shadow-md animate-in slide-in-from-right duration-150"
            >
              <!-- Cabeçalho do Sheet -->
              <div class="flex items-center justify-between pb-2.5 border-b border-border shrink-0">
                <div class="min-w-0 mr-3">
                  <div class="flex items-center gap-2">
                    <h4 class="text-sm font-bold text-foreground truncate">{viewingSong.title}</h4>
                    {#if viewingSong.audio_url}
                      <Badge variant="outline" class="text-[10px] py-0 px-1.5 font-mono text-emerald-400 border-emerald-500/40 bg-emerald-950/40 flex items-center gap-1">
                        <Music size={10} />
                        Playback
                      </Badge>
                    {/if}
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
                      <!-- svelte-ignore a11y_no_static_element_interactions -->
                      <div
                        oncontextmenu={(e) => viewingSong && handleViewingSongSlideContextMenu(e, viewingSong, slide, idx)}
                        class="rounded-xl border border-border bg-background/60 overflow-hidden flex flex-col shadow-xs transition-all hover:border-primary/50 cursor-pointer"
                      >
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
              <ResizablePane defaultSize={35} minSize={20} maxSize={60} class="flex flex-col p-1.5 pr-2 overflow-y-auto min-h-0">
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div class="grid grid-cols-1 gap-2 min-h-full" oncontextmenu={handleSongsEmptyContextMenu}>
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
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="flex-1 overflow-y-auto flex flex-col p-1.5 min-h-0" oncontextmenu={handleSongsEmptyContextMenu}>
              {#if songs.length === 0}
                <div class="h-32 flex flex-col items-center justify-center text-xs text-muted-foreground">
                  Nenhuma música encontrada no catálogo.
                </div>
              {:else}
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-2.5">
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
            <div class="space-y-0.5">
              <div
                role="button"
                tabindex="0"
                onclick={() => {
                  selectedFolderFilter = 'app';
                  selectedSubfolder = null;
                }}
                onkeydown={(e) => {
                  if (e.key === 'Enter') {
                    selectedFolderFilter = 'app';
                    selectedSubfolder = null;
                  }
                }}
                class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedFolderFilter === 'app' && selectedSubfolder === null ? 'bg-primary/10 text-primary font-bold' : selectedFolderFilter === 'app' ? 'bg-muted/60 text-foreground font-semibold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
              >
                <div class="flex items-center gap-1.5 truncate">
                  {#if allAppImageSubfolders.length > 0}
                    <button
                      type="button"
                      onclick={(e) => {
                        e.stopPropagation();
                        isAppFolderExpanded = !isAppFolderExpanded;
                      }}
                      class="p-0.5 rounded hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer"
                      title={isAppFolderExpanded ? 'Recolher subpastas' : 'Expandir subpastas'}
                    >
                      {#if isAppFolderExpanded}
                        <ChevronDown size={11} />
                      {:else}
                        <ChevronRight size={11} />
                      {/if}
                    </button>
                  {:else}
                    <span class="w-3.5"></span>
                  {/if}
                  <Folder size={13} class="text-primary shrink-0" />
                  <span class="truncate">Pasta do App</span>
                </div>
                <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono shrink-0">
                  {images.filter((i) => !i.is_external).length}
                </Badge>
              </div>

              {#if isAppFolderExpanded && allAppImageSubfolders.length > 0}
                <div class="pl-4 space-y-0.5 border-l border-border/50 ml-3">
                  {#each allAppImageSubfolders as sub (sub.relative_path)}
                    {@const isSubSelected = selectedFolderFilter === 'app' && selectedSubfolder === sub.relative_path}
                    <button
                      type="button"
                      onclick={() => {
                        selectedFolderFilter = 'app';
                        selectedSubfolder = sub.relative_path;
                      }}
                      class="w-full flex items-center justify-between px-2 py-1 rounded text-[11px] transition-colors cursor-pointer {isSubSelected ? 'bg-primary/20 text-primary font-bold' : 'text-muted-foreground hover:bg-muted/60 hover:text-foreground'}"
                      title="Pasta do App / {sub.relative_path}"
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
              <nav class="flex items-center gap-1.5 text-xs text-muted-foreground overflow-x-auto min-w-0">
                <button
                  type="button"
                  onclick={() => {
                    selectedFolderFilter = 'all';
                    selectedSubfolder = null;
                  }}
                  class="hover:text-foreground transition-colors cursor-pointer shrink-0"
                >
                  Mídias
                </button>
                <ChevronRight size={11} class="shrink-0" />
                <button
                  type="button"
                  onclick={() => {
                    selectedFolderFilter = 'all';
                    selectedSubfolder = null;
                  }}
                  class="hover:text-foreground transition-colors cursor-pointer shrink-0 {selectedFolderFilter === 'all' ? 'text-foreground font-bold' : ''}"
                >
                  Imagens
                </button>
                {#if selectedFolderFilter !== 'all'}
                  <ChevronRight size={11} class="shrink-0" />
                  <button
                    type="button"
                    onclick={() => (selectedSubfolder = null)}
                    class="hover:text-foreground transition-colors cursor-pointer shrink-0 {!selectedSubfolder ? 'text-foreground font-bold' : ''}"
                  >
                    {currentFolderName}
                  </button>
                {/if}
                {#each imageBreadcrumbs as crumb, i}
                  <ChevronRight size={11} class="shrink-0" />
                  {#if i === imageBreadcrumbs.length - 1}
                    <span class="text-foreground font-bold truncate max-w-xs shrink-0">
                      {crumb.name}
                    </span>
                  {:else}
                    <button
                      type="button"
                      onclick={() => (selectedSubfolder = crumb.path)}
                      class="hover:text-foreground transition-colors cursor-pointer shrink-0"
                    >
                      {crumb.name}
                    </button>
                  {/if}
                {/each}
                <Badge variant="secondary" class="text-[9px] py-0 px-1 ml-1 font-mono shrink-0">
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
                  onclick={openCreateFolderModal}
                  class="h-7 text-xs gap-1 font-medium"
                  title="Criar nova pasta física no diretório atual"
                >
                  <FolderPlus size={12} class="text-amber-400" />
                  <span>+ Pasta</span>
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={onopenfolderconfig}
                  class="h-7 text-xs gap-1"
                  title="Configurar pastas externas de mídia"
                >
                  <FolderOpen size={12} />
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
              {#if currentSubfolders.length > 0}
                <div class="mb-3.5 pb-3 border-b border-border/50">
                  <div class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground mb-1.5 flex items-center justify-between">
                    <span class="flex items-center gap-1.5">
                      <Folder size={12} class="text-amber-400" />
                      <span>Pastas ({currentSubfolders.length})</span>
                    </span>
                    {#if selectedSubfolder}
                      <button
                        type="button"
                        onclick={navigateUpImageFolder}
                        class="text-[11px] text-primary hover:underline flex items-center gap-1 cursor-pointer font-normal"
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
                        class="p-2.5 rounded-xl border border-border bg-card/60 hover:bg-neutral-850 hover:border-amber-400/50 text-left transition-all flex items-center gap-2.5 group cursor-pointer shadow-xs"
                        title="Abrir pasta {sub.name}"
                      >
                        <div class="w-8 h-8 rounded-lg bg-amber-400/10 text-amber-400 flex items-center justify-center shrink-0 group-hover:scale-105 group-hover:bg-amber-400/20 transition-all">
                          <Folder size={16} />
                        </div>
                        <div class="min-w-0 flex-1">
                          <p class="text-xs font-semibold text-foreground truncate group-hover:text-amber-400 transition-colors">
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
                <div class="h-32 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2">
                  {#if currentSubfolders.length > 0}
                    <span>Nenhuma imagem na raiz desta pasta. Abra uma pasta acima ou importe imagens.</span>
                  {:else}
                    <span>Nenhuma imagem encontrada nesta pasta.</span>
                    <Button
                      variant="outline"
                      size="sm"
                      onclick={() => fileInputEl?.click()}
                      class="text-xs gap-1 mt-1"
                    >
                      <Upload size={12} />
                      <span>Importar primeira imagem aqui</span>
                    </Button>
                  {/if}
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

      <!-- ================= ABA: VÍDEOS ================= -->
      {:else if activeTab === 'videos'}
        <div role="tabpanel" id="panel-videos" aria-labelledby="tab-videos" class="flex-1 flex flex-col overflow-hidden gap-2">
          <!-- Barra de Ações e Filtro de Vídeos -->
          <div class="flex items-center justify-between gap-2 pb-2 border-b border-border/60 shrink-0">
            <div class="flex items-center gap-2 flex-1 max-w-md">
              <div class="relative flex-1">
                <Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
                <input
                  type="text"
                  bind:value={videoSearch}
                  placeholder="Buscar vídeos por nome..."
                  class="w-full h-8 pl-8 pr-3 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
                />
                {#if videoSearch}
                  <button
                    type="button"
                    onclick={() => (videoSearch = '')}
                    class="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer"
                  >
                    <X size={12} />
                  </button>
                {/if}
              </div>
              <Badge variant="secondary" class="text-[10px] font-mono py-0.5 px-2">
                {filteredVideos.length} {filteredVideos.length === 1 ? 'vídeo' : 'vídeos'}
              </Badge>
            </div>

            <div class="flex items-center gap-1.5 shrink-0">
              <input
                type="file"
                bind:this={videoFileInputEl}
                onchange={handleVideoFilesSelected}
                accept="video/mp4,video/webm,video/ogg,video/quicktime,video/mkv,.mp4,.webm,.mov,.mkv"
                class="hidden"
              />
              <Button
                variant="outline"
                size="sm"
                onclick={loadVideos}
                title="Recarregar vídeos"
                class="h-8 text-xs px-2.5"
              >
                <RefreshCw size={12} class={isLoadingVideos ? 'animate-spin' : ''} />
              </Button>
              <Button
                variant="outline"
                size="sm"
                onclick={openCreateFolderModal}
                class="h-8 text-xs gap-1.5 font-medium"
                title="Criar nova pasta física de vídeos no diretório atual"
              >
                <FolderPlus size={13} class="text-purple-400" />
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

          <!-- Filtro de Pastas de Vídeo -->
          {#if videoFoldersList.length > 0 || folders.length > 0 || allAppVideoSubfolders.length > 0}
            <div class="flex items-center gap-1.5 overflow-x-auto pb-1 shrink-0 text-xs">
              <span class="text-[11px] text-muted-foreground flex items-center gap-1 mr-1 shrink-0">
                <Folder size={12} class="text-purple-400" />
                <span>Pastas:</span>
              </span>
              <button
                type="button"
                onclick={() => {
                  videoFolderFilter = 'all';
                  selectedVideoSubfolder = null;
                }}
                class="px-2.5 py-1 rounded-lg text-[11px] font-medium transition-colors cursor-pointer shrink-0 {videoFolderFilter === 'all' ? 'bg-purple-600 text-white font-bold' : 'bg-neutral-900 border border-neutral-800 text-neutral-400 hover:text-white'}"
              >
                Todas ({videos.length})
              </button>
              <button
                type="button"
                onclick={() => {
                  videoFolderFilter = 'app';
                  selectedVideoSubfolder = null;
                }}
                class="px-2.5 py-1 rounded-lg text-[11px] font-medium transition-colors cursor-pointer shrink-0 {videoFolderFilter === 'app' ? 'bg-purple-600 text-white font-bold' : 'bg-neutral-900 border border-neutral-800 text-neutral-400 hover:text-white'}"
              >
                Pasta do App ({videos.filter((v) => !v.folder_name || v.folder_id === 'app').length})
              </button>
              {#each videoFoldersList as folderName}
                <button
                  type="button"
                  onclick={() => {
                    videoFolderFilter = folderName;
                    selectedVideoSubfolder = null;
                  }}
                  class="px-2.5 py-1 rounded-lg text-[11px] font-medium transition-colors cursor-pointer shrink-0 {videoFolderFilter === folderName ? 'bg-purple-600 text-white font-bold' : 'bg-neutral-900 border border-neutral-800 text-neutral-400 hover:text-white'}"
                >
                  {folderName} ({videos.filter((v) => v.folder_name === folderName).length})
                </button>
              {/each}
            </div>
          {/if}

          <!-- Breadcrumbs de Navegação estilo File Explorer para Vídeos -->
          {#if videoFolderFilter !== 'all' || selectedVideoSubfolder}
            <div class="flex items-center justify-between gap-2 px-2 py-1 bg-neutral-900/60 rounded-lg border border-border/40 shrink-0 text-xs text-muted-foreground">
              <nav class="flex items-center gap-1.5 overflow-x-auto min-w-0">
                <button
                  type="button"
                  onclick={() => {
                    videoFolderFilter = 'all';
                    selectedVideoSubfolder = null;
                  }}
                  class="hover:text-foreground transition-colors cursor-pointer shrink-0"
                >
                  Vídeos
                </button>
                <ChevronRight size={11} class="shrink-0" />
                <button
                  type="button"
                  onclick={() => (selectedVideoSubfolder = null)}
                  class="hover:text-foreground transition-colors cursor-pointer shrink-0 {!selectedVideoSubfolder ? 'text-foreground font-bold' : ''}"
                >
                  {currentVideoFolderName}
                </button>
                {#each videoBreadcrumbs as crumb, i}
                  <ChevronRight size={11} class="shrink-0" />
                  {#if i === videoBreadcrumbs.length - 1}
                    <span class="text-foreground font-bold truncate max-w-xs shrink-0">
                      {crumb.name}
                    </span>
                  {:else}
                    <button
                      type="button"
                      onclick={() => (selectedVideoSubfolder = crumb.path)}
                      class="hover:text-foreground transition-colors cursor-pointer shrink-0"
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
                  class="text-[11px] text-purple-400 hover:underline flex items-center gap-1 cursor-pointer font-medium shrink-0"
                  title="Subir para a pasta anterior"
                >
                  <ChevronUp size={12} />
                  <span>Subir um nível</span>
                </button>
              {/if}
            </div>
          {/if}

          <!-- Grade de Vídeos com File Explorer -->
          <div class="flex-1 overflow-y-auto pr-1">
            {#if currentVideoDirectSubfolders.length > 0}
              <div class="mb-3.5 pb-3 border-b border-border/50">
                <div class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground mb-1.5 flex items-center justify-between">
                  <span class="flex items-center gap-1.5">
                    <Folder size={12} class="text-purple-400" />
                    <span>Pastas ({currentVideoDirectSubfolders.length})</span>
                  </span>
                  {#if selectedVideoSubfolder}
                    <button
                      type="button"
                      onclick={navigateUpVideoFolder}
                      class="text-[11px] text-purple-400 hover:underline flex items-center gap-1 cursor-pointer font-normal"
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
                      class="p-2.5 rounded-xl border border-border bg-card/60 hover:bg-neutral-850 hover:border-purple-500/50 text-left transition-all flex items-center gap-2.5 group cursor-pointer shadow-xs"
                      title="Abrir pasta {sub.name}"
                    >
                      <div class="w-8 h-8 rounded-lg bg-purple-500/10 text-purple-400 flex items-center justify-center shrink-0 group-hover:scale-105 group-hover:bg-purple-500/20 transition-all">
                        <Folder size={16} />
                      </div>
                      <div class="min-w-0 flex-1">
                        <p class="text-xs font-semibold text-foreground truncate group-hover:text-purple-300 transition-colors">
                          {sub.name}
                        </p>
                        <p class="text-[10px] text-muted-foreground font-mono">
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
                <Film size={28} class="text-neutral-600 opacity-60" />
                {#if currentVideoDirectSubfolders.length > 0}
                  <p>Nenhum vídeo na raiz desta pasta. Abra uma pasta acima ou importe vídeos.</p>
                {:else}
                  <p>Nenhum vídeo encontrado nesta pasta.</p>
                {/if}
                <div class="flex items-center gap-2 mt-1">
                  <Button
                    variant="outline"
                    size="sm"
                    onclick={openCreateFolderModal}
                    class="text-xs gap-1"
                  >
                    <FolderPlus size={12} class="text-purple-400" />
                    <span>Nova Pasta</span>
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    onclick={() => videoFileInputEl?.click()}
                    class="text-xs gap-1"
                  >
                    <Upload size={12} />
                    <span>Importar vídeo aqui</span>
                  </Button>
                </div>
              </div>
            {:else}
              <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
                {#each filteredVideos as vid (vid.id)}
                  <div
                    ondblclick={() => handlePresentVideo(vid)}
                    role="button"
                    tabindex="0"
                    onkeydown={(e) => { if (e.key === 'Enter') handlePresentVideo(vid); }}
                    class="group relative rounded-xl border border-border bg-card overflow-hidden flex flex-col shadow-xs hover:border-purple-500/60 transition-all cursor-pointer"
                  >
                    <!-- Video Preview Container -->
                    <div class="aspect-video w-full bg-black/80 relative overflow-hidden flex items-center justify-center">
                      <!-- svelte-ignore a11y_media_has_caption -->
                      <video
                        src={vid.url}
                        preload="metadata"
                        muted
                        class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-200 pointer-events-none"
                      ></video>
                      <div class="absolute inset-0 bg-black/30 group-hover:bg-black/10 transition-colors flex items-center justify-center">
                        <div class="w-9 h-9 rounded-full bg-purple-600/80 group-hover:bg-purple-600 text-white flex items-center justify-center shadow-lg transition-transform group-hover:scale-110">
                          <Play size={16} class="fill-current ml-0.5" />
                        </div>
                      </div>
                      <div class="absolute bottom-1.5 right-1.5 px-1.5 py-0.5 rounded bg-black/80 text-[10px] text-purple-300 font-mono border border-purple-500/30">
                        {formatBytes(vid.size_bytes)}
                      </div>
                    </div>

                    <!-- Informações do Vídeo -->
                    <div class="p-2 bg-neutral-900/90 border-t border-border flex flex-col gap-1.5 flex-1 justify-between">
                      <div class="min-w-0">
                        <p class="text-xs font-semibold text-foreground truncate" title={vid.name}>
                          {vid.name}
                        </p>
                        {#if vid.folder_name}
                          <p class="text-[10px] text-muted-foreground truncate">{vid.folder_name}</p>
                        {/if}
                      </div>

                      <!-- Ações -->
                      <div class="flex items-center gap-1.5 pt-1">
                        <button
                          type="button"
                          onclick={() => handlePresentVideo(vid)}
                          class="flex-1 h-7 rounded-lg bg-purple-600/20 hover:bg-purple-600/30 text-purple-300 hover:text-purple-200 text-xs font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                          title="Apresentar Vídeo na Tela Pública"
                        >
                          <Play size={11} class="fill-current" />
                          <span>Apresentar</span>
                        </button>
                        <button
                          type="button"
                          onclick={() => handleAddVideoToSetlist(vid)}
                          class="h-7 px-2 rounded-lg bg-muted hover:bg-muted/80 text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0"
                          title="Adicionar à Setlist"
                        >
                          <ListPlus size={13} />
                        </button>
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>

      <!-- ================= ABA: ÁUDIOS ================= -->
      {:else if activeTab === 'audios'}
        <div role="tabpanel" id="panel-audios" aria-labelledby="tab-audios" class="flex-1 flex flex-col overflow-hidden gap-2">
          <!-- Audio Player Embutido (Persistent Control Bar) -->
          <audio
            bind:this={audioElement}
            ontimeupdate={handleAudioTimeUpdate}
            onended={handleAudioEnded}
            onpause={() => (isAudioPlaying = false)}
            onplay={() => (isAudioPlaying = true)}
            class="hidden"
          ></audio>

          <!-- Player Bar Card -->
          <div class="p-2.5 rounded-xl bg-neutral-950/80 border border-indigo-500/30 shadow-md flex items-center justify-between gap-4 shrink-0">
            <!-- Informações da Faixa em Reprodução -->
            <div class="flex items-center gap-3 min-w-0 max-w-xs md:max-w-sm">
              <div class="w-10 h-10 rounded-xl bg-indigo-500/20 border border-indigo-500/40 text-indigo-400 flex items-center justify-center shrink-0">
                <Volume2 size={20} class={isAudioPlaying ? 'animate-pulse text-indigo-300' : ''} />
              </div>
              <div class="min-w-0">
                <p class="text-xs font-bold text-foreground truncate">
                  {playingAudio ? playingAudio.name : 'Nenhum áudio selecionado'}
                </p>
                <p class="text-[10px] text-muted-foreground truncate">
                  {playingAudio ? (isAudioPlaying ? 'Reproduzindo' : 'Pausado') : 'Escolha uma faixa abaixo para tocar'}
                </p>
              </div>
            </div>

            <!-- Controles de Transporte e SeekBar -->
            <div class="flex-1 flex flex-col items-center gap-1 max-w-xl">
              <div class="flex items-center gap-3">
                <button
                  type="button"
                  onclick={toggleAudioLoop}
                  class="p-1.5 rounded-lg text-xs transition-colors cursor-pointer {isAudioLoop ? 'text-indigo-400 bg-indigo-500/20' : 'text-neutral-400 hover:text-white'}"
                  title={isAudioLoop ? 'Repetir ativado' : 'Repetir desativado'}
                >
                  <Repeat size={14} />
                </button>

                <button
                  type="button"
                  onclick={() => {
                    if (playingAudio) {
                      handlePlayAudio(playingAudio);
                    } else if (filteredAudios.length > 0) {
                      handlePlayAudio(filteredAudios[0]);
                    }
                  }}
                  disabled={!playingAudio && filteredAudios.length === 0}
                  class="w-8 h-8 rounded-full bg-indigo-600 hover:bg-indigo-500 text-white flex items-center justify-center shadow-lg transition-transform hover:scale-105 cursor-pointer disabled:opacity-40"
                  title={isAudioPlaying ? 'Pausar áudio' : 'Reproduzir áudio'}
                >
                  {#if isAudioPlaying}
                    <Pause size={15} class="fill-current" />
                  {:else}
                    <Play size={15} class="fill-current ml-0.5" />
                  {/if}
                </button>

                <button
                  type="button"
                  onclick={handleStopAudio}
                  disabled={!playingAudio}
                  class="p-1.5 rounded-lg text-neutral-400 hover:text-white transition-colors cursor-pointer disabled:opacity-40"
                  title="Parar reprodução"
                >
                  <div class="w-3.5 h-3.5 bg-current rounded-xs"></div>
                </button>
              </div>

              <!-- Seek Slider & Time -->
              <div class="w-full flex items-center gap-2 text-[10px] font-mono text-neutral-400">
                <span class="w-10 text-right">{formatDuration(audioCurrentTime)}</span>
                <input
                  type="range"
                  min="0"
                  max={audioDuration || 100}
                  step="0.5"
                  value={audioCurrentTime}
                  oninput={handleSeekAudio}
                  class="flex-1 h-1.5 bg-neutral-800 rounded-lg appearance-none cursor-pointer accent-indigo-500"
                />
                <span class="w-10">{formatDuration(audioDuration)}</span>
              </div>
            </div>

            <!-- Controle de Volume -->
            <div class="flex items-center gap-2 shrink-0">
              <button
                type="button"
                onclick={toggleAudioMute}
                class="text-neutral-400 hover:text-white transition-colors cursor-pointer"
                title={isAudioMuted ? 'Desmutar' : 'Mutar'}
              >
                {#if isAudioMuted || audioVolume === 0}
                  <VolumeX size={16} />
                {:else}
                  <Volume2 size={16} />
                {/if}
              </button>
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                value={isAudioMuted ? 0 : audioVolume}
                oninput={handleVolumeChange}
                class="w-16 h-1.5 bg-neutral-800 rounded-lg appearance-none cursor-pointer accent-indigo-500"
                title="Volume"
              />
            </div>
          </div>

          <!-- Barra de Busca e Upload de Áudios -->
          <div class="flex items-center justify-between gap-2 pb-2 border-b border-border/60 shrink-0">
            <div class="flex items-center gap-2 flex-1 max-w-md">
              <div class="relative flex-1">
                <Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
                <input
                  type="text"
                  bind:value={audioSearch}
                  placeholder="Buscar áudios por nome..."
                  class="w-full h-8 pl-8 pr-3 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
                />
                {#if audioSearch}
                  <button
                    type="button"
                    onclick={() => (audioSearch = '')}
                    class="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer"
                  >
                    <X size={12} />
                  </button>
                {/if}
              </div>
              <Badge variant="secondary" class="text-[10px] font-mono py-0.5 px-2">
                {filteredAudios.length} {filteredAudios.length === 1 ? 'faixa' : 'faixas'}
              </Badge>
            </div>

            <div class="flex items-center gap-1.5 shrink-0">
              <input
                type="file"
                bind:this={audioFileInputEl}
                onchange={handleAudioFilesSelected}
                accept="audio/mp3,audio/wav,audio/m4a,audio/aac,audio/ogg,audio/flac,.mp3,.wav,.m4a,.aac,.ogg,.flac"
                class="hidden"
              />
              <Button
                variant="outline"
                size="sm"
                onclick={loadAudios}
                title="Recarregar áudios"
                class="h-8 text-xs px-2.5"
              >
                <RefreshCw size={12} class={isLoadingAudios ? 'animate-spin' : ''} />
              </Button>
              <Button
                variant="outline"
                size="sm"
                onclick={onopenfolderconfig}
                class="h-8 text-xs gap-1.5"
                title="Mapear ou gerenciar pastas locais de áudio"
              >
                <FolderPlus size={13} class="text-indigo-400" />
                <span>Pastas</span>
              </Button>
              <Button
                variant="default"
                size="sm"
                onclick={() => audioFileInputEl?.click()}
                disabled={isUploadingAudio}
                class="h-8 text-xs gap-1.5 font-semibold"
              >
                <Upload size={13} class={isUploadingAudio ? 'animate-spin' : ''} />
                <span>{isUploadingAudio ? 'Enviando...' : 'Importar Áudio'}</span>
              </Button>
            </div>
          </div>

          <!-- Filtro de Pastas de Áudio -->
          {#if audioFoldersList.length > 0 || folders.length > 0}
            <div class="flex items-center gap-1.5 overflow-x-auto pb-1 shrink-0 text-xs">
              <span class="text-[11px] text-muted-foreground flex items-center gap-1 mr-1">
                <Folder size={12} class="text-indigo-400" />
                <span>Pastas:</span>
              </span>
              <button
                type="button"
                onclick={() => (audioFolderFilter = 'all')}
                class="px-2.5 py-1 rounded-lg text-[11px] font-medium transition-colors cursor-pointer {audioFolderFilter === 'all' ? 'bg-indigo-600 text-white font-bold' : 'bg-neutral-900 border border-neutral-800 text-neutral-400 hover:text-white'}"
              >
                Todas ({audios.length})
              </button>
              <button
                type="button"
                onclick={() => (audioFolderFilter = 'app')}
                class="px-2.5 py-1 rounded-lg text-[11px] font-medium transition-colors cursor-pointer {audioFolderFilter === 'app' ? 'bg-indigo-600 text-white font-bold' : 'bg-neutral-900 border border-neutral-800 text-neutral-400 hover:text-white'}"
              >
                Pasta do App ({audios.filter((a) => !a.folder_name).length})
              </button>
              {#each audioFoldersList as folderName}
                <button
                  type="button"
                  onclick={() => (audioFolderFilter = folderName)}
                  class="px-2.5 py-1 rounded-lg text-[11px] font-medium transition-colors cursor-pointer {audioFolderFilter === folderName ? 'bg-indigo-600 text-white font-bold' : 'bg-neutral-900 border border-neutral-800 text-neutral-400 hover:text-white'}"
                >
                  {folderName} ({audios.filter((a) => a.folder_name === folderName).length})
                </button>
              {/each}
            </div>
          {/if}

          <!-- Lista de Faixas de Áudio -->
          <div class="flex-1 overflow-y-auto pr-1">
            {#if filteredAudios.length === 0}
              <div class="h-44 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2">
                <Volume2 size={28} class="text-neutral-600 opacity-60" />
                <p>Nenhum áudio encontrado na biblioteca.</p>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={() => audioFileInputEl?.click()}
                  class="text-xs gap-1 mt-1"
                >
                  <Upload size={12} />
                  <span>Importar primeiro áudio (MP3, WAV, M4A)</span>
                </Button>
              </div>
            {:else}
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
                {#each filteredAudios as aud (aud.id)}
                  {@const isThisPlaying = playingAudio?.id === aud.id && isAudioPlaying}
                  {@const isThisSelected = playingAudio?.id === aud.id}
                  <div
                    role="button"
                    tabindex="0"
                    onclick={() => handlePlayAudio(aud)}
                    ondblclick={() => handlePresentAudio(aud)}
                    onkeydown={(e) => { if (e.key === 'Enter') handlePresentAudio(aud); }}
                    class="p-2.5 rounded-xl border transition-all flex items-center justify-between gap-3 group cursor-pointer {isThisSelected ? 'border-indigo-500 bg-indigo-950/30' : 'border-border bg-card/60 hover:bg-neutral-850'}"
                  >
                    <div class="flex items-center gap-3 min-w-0">
                      <button
                        type="button"
                        onclick={(e) => {
                          e.stopPropagation();
                          handlePlayAudio(aud);
                        }}
                        class="w-9 h-9 rounded-xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105 {isThisPlaying ? 'bg-indigo-600 text-white' : 'bg-indigo-500/20 text-indigo-400 group-hover:bg-indigo-600 group-hover:text-white'}"
                        title={isThisPlaying ? 'Pausar' : 'Reproduzir'}
                      >
                        {#if isThisPlaying}
                          <Pause size={15} class="fill-current" />
                        {:else}
                          <Play size={15} class="fill-current ml-0.5" />
                        {/if}
                      </button>

                      <div class="min-w-0">
                        <p class="text-xs font-semibold text-foreground truncate group-hover:text-indigo-300 transition-colors" title={aud.name}>
                          {aud.name}
                        </p>
                        <div class="flex items-center gap-1.5 text-[10px] text-muted-foreground font-mono">
                          <span>{formatBytes(aud.size_bytes)}</span>
                          {#if aud.folder_name}
                            <span>•</span>
                            <span class="text-indigo-300/80 truncate max-w-[120px]">{aud.folder_name}</span>
                          {/if}
                        </div>
                      </div>
                    </div>

                    <div class="flex items-center gap-1 shrink-0">
                      <button
                        type="button"
                        onclick={(e) => {
                          e.stopPropagation();
                          handlePresentAudio(aud);
                        }}
                        class="h-7 px-2 rounded-lg bg-indigo-600/20 hover:bg-indigo-600 text-indigo-300 hover:text-white flex items-center gap-1 text-xs font-medium transition-colors cursor-pointer"
                        title="Apresentar Áudio Imediatamente"
                      >
                        <Play size={11} class="fill-current" />
                        <span class="text-[11px] hidden sm:inline">Apresentar</span>
                      </button>

                      <button
                        type="button"
                        onclick={(e) => {
                          e.stopPropagation();
                          handleAddAudioToSetlist(aud);
                        }}
                        class="h-7 px-2 rounded-lg bg-muted hover:bg-muted/80 text-foreground flex items-center gap-1 text-xs font-medium transition-colors cursor-pointer"
                        title="Adicionar à Setlist"
                      >
                        <ListPlus size={13} />
                        <span class="text-[11px] hidden sm:inline">+ Setlist</span>
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
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
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
              class="flex-1 overflow-y-auto pr-1"
              oncontextmenu={handleBackgroundsEmptyContextMenu}
            >
              <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
                <!-- Presets em Código CSS -->
                {#if selectedBgCategory !== 'local'}
                  {#each filteredBackgrounds as preset (preset.id)}
                    {@const encoded = encodeBackground(preset)}
                    {@const isLiveBg = $presentationState.lyrics_background_path === encoded || ($presentationState.active_media_id && $presentationState.wallpaper_path === encoded)}

                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      onclick={() => applyBackground(encoded)}
                      oncontextmenu={(e) => handlePresetContextMenu(e, preset)}
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
                      oncontextmenu={(e) => handleLocalWallpaperContextMenu(e, wall)}
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

                        {#if !wall.is_default}
                          <button
                            type="button"
                            onclick={(e) => {
                              e.stopPropagation();
                              handleDeleteWallpaper(wall);
                            }}
                            class="h-6 w-6 rounded bg-transparent hover:bg-destructive/20 text-muted-foreground hover:text-destructive flex items-center justify-center transition-colors cursor-pointer shrink-0"
                            title="Excluir este papel de parede local"
                          >
                            <Trash2 size={11} />
                          </button>
                        {/if}
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

              <!-- Botão Dropdown Adicionar -->
              <div class="relative">
                <Button
                  variant="default"
                  size="sm"
                  onclick={() => (isAddOtherMenuOpen = !isAddOtherMenuOpen)}
                  title="Criar novo item ou importar para a aba Outros"
                  class="h-7 text-xs gap-1.5 cursor-pointer shadow-sm"
                  aria-expanded={isAddOtherMenuOpen}
                >
                  <Plus size={13} />
                  <span>Adicionar</span>
                  <ChevronDown size={11} class="opacity-70 transition-transform duration-200 {isAddOtherMenuOpen ? 'rotate-180' : ''}" />
                </Button>

                {#if isAddOtherMenuOpen}
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div
                    class="fixed inset-0 z-40"
                    onclick={() => (isAddOtherMenuOpen = false)}
                  ></div>

                  <div
                    class="absolute right-0 top-full mt-1.5 w-60 rounded-xl bg-neutral-900 border border-neutral-800 shadow-2xl p-1 z-50 animate-in fade-in zoom-in-95 duration-150 select-none text-left"
                    role="menu"
                  >
                    <!-- Opção Agenda -->
                    <button
                      type="button"
                      role="menuitem"
                      onclick={() => {
                        isAddOtherMenuOpen = false;
                        onopenscheduleeditor?.();
                      }}
                      class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-amber-300 hover:bg-neutral-800 transition-colors text-left cursor-pointer group"
                    >
                      <CalendarDays size={14} class="shrink-0 text-amber-400" />
                      <div class="flex flex-col min-w-0">
                        <span class="font-semibold text-neutral-200 group-hover:text-amber-300 transition-colors">Programação (Agenda)</span>
                        <span class="text-[10px] text-neutral-400 font-normal">Cultos semanais e eventos especiais</span>
                      </div>
                    </button>

                    <!-- Opção Texto Livre -->
                    <button
                      type="button"
                      role="menuitem"
                      onclick={() => {
                        isAddOtherMenuOpen = false;
                        onopentexteditor?.();
                      }}
                      class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-blue-300 hover:bg-neutral-800 transition-colors text-left cursor-pointer group"
                    >
                      <FileText size={14} class="shrink-0 text-blue-400" />
                      <div class="flex flex-col min-w-0">
                        <span class="font-semibold text-neutral-200 group-hover:text-blue-300 transition-colors">Texto Livre</span>
                        <span class="text-[10px] text-neutral-400 font-normal">Apresentação com versos ou leitura</span>
                      </div>
                    </button>

                    <!-- Opção Aviso -->
                    <button
                      type="button"
                      role="menuitem"
                      onclick={() => {
                        isAddOtherMenuOpen = false;
                        onopenquickitemeditor?.('announcement');
                      }}
                      class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-emerald-300 hover:bg-neutral-800 transition-colors text-left cursor-pointer group"
                    >
                      <Bookmark size={14} class="shrink-0 text-emerald-400" />
                      <div class="flex flex-col min-w-0">
                        <span class="font-semibold text-neutral-200 group-hover:text-emerald-300 transition-colors">Aviso / Comunicado</span>
                        <span class="text-[10px] text-neutral-400 font-normal">Avisos e comunicados da equipe</span>
                      </div>
                    </button>

                    <!-- Opção Divisor de Seção -->
                    <button
                      type="button"
                      role="menuitem"
                      onclick={() => {
                        isAddOtherMenuOpen = false;
                        onopenquickitemeditor?.('header');
                      }}
                      class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-purple-300 hover:bg-neutral-800 transition-colors text-left cursor-pointer group"
                    >
                      <Layers size={14} class="shrink-0 text-purple-400" />
                      <div class="flex flex-col min-w-0">
                        <span class="font-semibold text-neutral-200 group-hover:text-purple-300 transition-colors">Divisor de Seção</span>
                        <span class="text-[10px] text-neutral-400 font-normal">Separador visual de blocos na setlist</span>
                      </div>
                    </button>

                    <!-- Opção Cronômetro / Timer -->
                    <button
                      type="button"
                      role="menuitem"
                      onclick={() => {
                        isAddOtherMenuOpen = false;
                        onopentimereditor?.();
                      }}
                      class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-rose-300 hover:bg-neutral-800 transition-colors text-left cursor-pointer group"
                    >
                      <Hourglass size={14} class="shrink-0 text-rose-400" />
                      <div class="flex flex-col min-w-0">
                        <span class="font-semibold text-neutral-200 group-hover:text-rose-300 transition-colors">Cronômetro / Timer</span>
                        <span class="text-[10px] text-neutral-400 font-normal">Contagem regressiva sincronizada com o stage</span>
                      </div>
                    </button>

                    <div class="my-1 border-t border-neutral-800"></div>

                    <!-- Opção Importar da Setlist -->
                    <button
                      type="button"
                      role="menuitem"
                      onclick={() => {
                        isAddOtherMenuOpen = false;
                        handleImportFromSetlistClick();
                      }}
                      disabled={!activeSetlistItems || activeSetlistItems.length === 0 || isImportingFromSetlist}
                      class="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-primary hover:bg-neutral-800 disabled:opacity-40 disabled:hover:bg-transparent transition-colors text-left cursor-pointer group"
                      title="Salva itens criados avulsos na setlist ativa diretamente no catálogo para reuso"
                    >
                      <Upload size={14} class="shrink-0 text-primary {isImportingFromSetlist ? 'animate-spin' : ''}" />
                      <div class="flex flex-col min-w-0">
                        <span class="font-semibold text-neutral-200 group-hover:text-primary transition-colors">Importar da Setlist Ativa</span>
                        <span class="text-[10px] text-neutral-400 font-normal">Salva itens avulsos da setlist no catálogo</span>
                      </div>
                    </button>
                  </div>
                {/if}
              </div>
            </div>
          </div>

          <!-- Barra de Filtros de Categoria (Chips) -->
          <div class="flex items-center gap-1.5 mb-2.5 overflow-x-auto pb-1 text-xs shrink-0 select-none">
            {#each [
              { id: 'all', label: 'Todos', count: $customItemsStore.length },
              { id: 'schedule', label: 'Agenda', count: $customItemsStore.filter((i) => i.kind === 'schedule').length },
              { id: 'text', label: 'Textos', count: $customItemsStore.filter((i) => i.kind === 'text').length },
              { id: 'announcement', label: 'Avisos', count: $customItemsStore.filter((i) => i.kind === 'announcement').length },
              { id: 'header', label: 'Seções', count: $customItemsStore.filter((i) => i.kind === 'header').length },
              { id: 'timer', label: 'Cronômetros', count: $customItemsStore.filter((i) => i.kind === 'timer').length }
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

          <!-- Snippets de Card e Sheet de Outros -->
          {#snippet otherItemCard(item: MediaItem)}
            {@const badge = getItemKindBadge(item.kind)}
            {@const isSelected = viewingCustomItem?.id === item.id}
            <div
              role="button"
              tabindex="0"
              onclick={() => (viewingCustomItem = item)}
              ondblclick={() => onpresentitem?.(item, 0)}
              onkeydown={(e) => { if (e.key === 'Enter') viewingCustomItem = item; }}
              oncontextmenu={(e) => handleOtherItemContextMenu(e, item)}
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
                {:else if item.kind === 'timer'}
                  {@const dur = parseInt(item.slides[0]?.content || '0', 10)}
                  <p class="text-[10px] text-rose-300 font-mono mt-0.5 flex items-center gap-1">
                    <Hourglass size={10} />
                    <span>Duração: {formatDuration(isNaN(dur) ? 0 : dur)}</span>
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
          {/snippet}

          {#snippet viewingCustomItemSheet()}
            {#if viewingCustomItem}
              {@const currentBadge = getItemKindBadge(viewingCustomItem.kind)}
              <div class="flex-1 min-w-0 border border-border bg-card/60 rounded-xl flex flex-col p-3.5 h-full overflow-hidden shadow-md animate-in slide-in-from-right duration-150">
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
                    <button
                      type="button"
                      onclick={() => (viewingCustomItem = null)}
                      class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer ml-1"
                      title="Fechar visualização de slides"
                    >
                      <X size={16} />
                    </button>
                  </div>
                </div>

                <!-- Grade Responsiva de Slides 16:9 -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div class="flex-1 overflow-y-auto py-3 pr-1 min-h-0" oncontextmenu={handleOtherEmptyContextMenu}>
                  {#if viewingCustomItem.slides.length > 0}
                    <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3">
                      {#each viewingCustomItem.slides as slide, idx (slide.id || idx)}
                        <div
                          role="button"
                          tabindex="0"
                          ondblclick={() => viewingCustomItem && onpresentitem?.(viewingCustomItem, idx)}
                          onkeydown={(e) => { if (e.key === 'Enter') viewingCustomItem && onpresentitem?.(viewingCustomItem, idx); }}
                          oncontextmenu={(e) => viewingCustomItem && handleOtherSlideContextMenu(e, viewingCustomItem, slide, idx)}
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
                            {:else if viewingCustomItem.kind === 'timer'}
                              {@const secs = parseInt(slide.content || '0', 10)}
                              <div class="w-full flex flex-col items-center justify-center p-2 rounded-xl bg-rose-950/40 border border-rose-500/30 gap-1">
                                <div class="flex items-center gap-1.5 text-rose-400 font-semibold text-[10px] uppercase tracking-wider">
                                  <Hourglass size={12} />
                                  <span>{slide.label || 'Cronômetro'}</span>
                                </div>
                                <div class="text-3xl font-mono font-black text-rose-300 tracking-tight">
                                  {formatDuration(isNaN(secs) ? 0 : secs)}
                                </div>
                                {#if slide.lower_content}
                                  <p class="text-[10px] text-rose-200/80 line-clamp-2 mt-0.5 max-w-full italic">
                                    {slide.lower_content}
                                  </p>
                                {/if}
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
            {/if}
          {/snippet}

          <!-- Grade/Lista de Itens com Suporte a Sheet de Prévia Lateral e Divisor Resizable -->
          <div class="flex-1 flex overflow-hidden min-h-0 w-full h-full">
            {#if viewingCustomItem}
              <ResizablePaneGroup direction="horizontal" class="flex-1 w-full h-full min-h-0 overflow-hidden">
                <!-- Coluna do Catálogo de Outros (Redimensionável) -->
                <ResizablePane defaultSize={35} minSize={20} maxSize={60} class="flex flex-col p-1.5 pr-2 overflow-y-auto min-h-0">
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div class="grid grid-cols-1 gap-2" oncontextmenu={handleOtherEmptyContextMenu}>
                    {#each filteredCustomItems as item (item.id)}
                      {@render otherItemCard(item)}
                    {/each}
                  </div>
                </ResizablePane>

                <!-- Divisor Redimensionável Shadcn -->
                <ResizableHandle withHandle class="hover:bg-primary transition-colors cursor-col-resize" />

                <!-- Sheet Lateral de Prévia de Slides 16:9 (Redimensionável) -->
                <ResizablePane defaultSize={65} minSize={40} class="flex flex-col pl-3 min-w-0 min-h-0 overflow-hidden">
                  {@render viewingCustomItemSheet()}
                </ResizablePane>
              </ResizablePaneGroup>
            {:else}
              <!-- Catálogo em Tela Cheia (Quando nenhum item estiver aberto para visualização de slides) -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="flex-1 overflow-y-auto flex flex-col p-1.5 min-h-0" oncontextmenu={handleOtherEmptyContextMenu}>
                {#if filteredCustomItems.length === 0}
                  <div class="h-48 flex flex-col items-center justify-center text-center p-6 text-muted-foreground space-y-2">
                    <Boxes size={36} class="opacity-40" />
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
                  <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-2.5">
                    {#each filteredCustomItems as item (item.id)}
                      {@render otherItemCard(item)}
                    {/each}
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  {/if}

  <ContextMenu
    open={contextMenuOpen}
    x={contextMenuX}
    y={contextMenuY}
    items={contextMenuItems}
    onclose={() => (contextMenuOpen = false)}
  />

  {#if isCreateFolderModalOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4 animate-in fade-in duration-150"
      onclick={() => (isCreateFolderModalOpen = false)}
      role="presentation"
    >
      <div
        class="w-full max-w-sm rounded-xl border border-border bg-neutral-900 shadow-2xl flex flex-col overflow-hidden text-neutral-100"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-labelledby="create-folder-title"
        tabindex="-1"
      >
        <!-- Header -->
        <div class="h-12 px-4 border-b border-border flex items-center justify-between shrink-0 bg-neutral-900/80">
          <div class="flex items-center gap-2">
            <div class="w-7 h-7 rounded-lg {activeTab === 'videos' ? 'bg-purple-500/10 border-purple-500/20 text-purple-400' : 'bg-amber-500/10 border-amber-500/20 text-amber-400'} border flex items-center justify-center">
              <FolderPlus size={15} />
            </div>
            <div>
              <h3 id="create-folder-title" class="text-xs font-bold text-white">Nova Pasta</h3>
            </div>
          </div>
          <button
            type="button"
            onclick={() => (isCreateFolderModalOpen = false)}
            class="w-6 h-6 rounded-md hover:bg-neutral-800 flex items-center justify-center text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            <X size={14} />
          </button>
        </div>

        <!-- Form Body -->
        <form
          onsubmit={(e) => {
            e.preventDefault();
            handleCreateFolderSubmit();
          }}
          class="p-4 space-y-3.5"
        >
          <div class="space-y-1">
            <span class="text-[10px] text-muted-foreground uppercase font-bold tracking-wider">
              Localização:
            </span>
            <div class="p-2 rounded-lg bg-neutral-950 border border-neutral-800 text-[11px] text-neutral-300 font-mono truncate flex items-center gap-1.5">
              <FolderOpen size={12} class="{activeTab === 'videos' ? 'text-purple-400' : 'text-amber-400'} shrink-0" />
              <span class="truncate">{createFolderLocationLabel}</span>
            </div>
          </div>

          <div class="space-y-1.5">
            <label for="new-folder-input" class="text-xs font-semibold text-neutral-200">
              Nome da Pasta
            </label>
            <input
              id="new-folder-input"
              type="text"
              bind:this={folderInputEl}
              bind:value={newFolderName}
              placeholder="Ex: Culto da Noite, Anúncios, 2026..."
              maxlength={60}
              class="w-full h-8 px-3 rounded-lg bg-neutral-950 border border-neutral-800 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
            />
          </div>

          <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/50">
            <Button
              type="button"
              variant="ghost"
              size="sm"
              onclick={() => (isCreateFolderModalOpen = false)}
              disabled={isCreatingFolder}
              class="h-7 text-xs"
            >
              Cancelar
            </Button>
            <Button
              type="submit"
              variant="default"
              size="sm"
              disabled={!newFolderName.trim() || isCreatingFolder}
              class="h-7 text-xs font-semibold gap-1.5 {activeTab === 'videos' ? 'bg-purple-600 hover:bg-purple-700 text-white' : ''}"
            >
              {#if isCreatingFolder}
                <RefreshCw size={12} class="animate-spin" />
                <span>Criando...</span>
              {:else}
                <Plus size={13} />
                <span>Criar Pasta</span>
              {/if}
            </Button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>

<script module lang="ts">
  export type { SettingsTab } from './settings/types';
</script>

<script lang="ts">
  import { tick, onDestroy, untrack } from 'svelte';
  import {
    Palette,
    Keyboard,
    ListMusic,
    Monitor,
    Tv,
    Video,
    Sparkles,
    Sliders,
    Music,
    BookOpen,
    Church,
    HardDrive,
    ScrollText,
    Info,
    X,
    Search,
    Save,
    Check,
    RefreshCw,
    Settings,
  } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import ConfirmModal from './bottom-panel/ConfirmModal.svelte';
  import {
    SETTINGS_TABS,
    type SettingsTab,
    type SettingsConfirmRequest,
  } from './settings/types';
  import AppearanceTab from './settings/tabs/AppearanceTab.svelte';
  import ShortcutsTab from './settings/tabs/ShortcutsTab.svelte';
  import SectionsTab from './settings/tabs/SectionsTab.svelte';
  import PublicDisplayTab from './settings/tabs/PublicDisplayTab.svelte';
  import StageMonitorTab from './settings/tabs/StageMonitorTab.svelte';
  import LowerThirdTab from './settings/tabs/LowerThirdTab.svelte';
  import LyricsBackgroundsTab from './settings/tabs/LyricsBackgroundsTab.svelte';
  import OutputsTab from './settings/tabs/OutputsTab.svelte';
  import SongsTab from './settings/tabs/SongsTab.svelte';
  import BiblesTab from './settings/tabs/BiblesTab.svelte';
  import ChurchBrandTab from './settings/tabs/ChurchBrandTab.svelte';
  import FilesTab from './settings/tabs/FilesTab.svelte';
  import LogsTab from './settings/tabs/LogsTab.svelte';
  import AboutTab from './settings/tabs/AboutTab.svelte';
  import {
    sectionsStore,
    saveSections,
    resetSectionsToDefault,
    DEFAULT_SECTIONS,
    COLOR_PRESETS,
    setWallpaperFit,
    type CustomSectionConfig,
  } from '$lib/settings';
  import {
    publicSettingsStore,
    churchBrandStore,
    stageSettingsStore,
    backgroundStylesStore,
    lowerThirdSettingsStore,
    songSettingsStore,
    DEFAULT_PUBLIC_SETTINGS,
    DEFAULT_CHURCH_BRAND,
    DEFAULT_STAGE_SETTINGS,
    DEFAULT_BACKGROUND_STYLES,
    DEFAULT_LOWER_THIRD_SETTINGS,
    DEFAULT_SONG_SETTINGS,
    savePublicDisplaySettings,
    saveChurchBrandSettings,
    saveStageDisplaySettings,
    saveBackgroundStyles,
    saveLowerThirdSettings,
    saveSongSettings,
  } from '$lib/displaySettings';
  import type { BackgroundStylesMap } from '$lib/backgrounds';
  import {
    fetchUpdateCheck,
    fetchUpdaterStatus,
    triggerInstallUpdate,
    triggerSystemRestart,
    type UpdateCheckResponse,
    type UpdaterStatusPayload,
  } from '$lib/updater';
  import { readHttpError } from '$lib/media';
  import { notifySuccess, notifyError } from '$lib/toast';
  import type {
    ActionLog,
    BibleVersionSummary,
    ChurchBrandSettings,
    ImageFitMode,
    LowerThirdSettings,
    MediaFolder,
    PublicDisplaySettings,
    SongSettings,
    StageDisplaySettings,
    SystemPaths,
  } from '$lib/types';

  interface Props {
    open: boolean;
    initialTab?: SettingsTab;
    onclose: () => void;
    onbibleschanged?: () => void;
  }

  let { open, initialTab = 'appearance', onclose, onbibleschanged }: Props = $props();

  type IconComponent = typeof Palette;
  const TAB_ICONS: Record<SettingsTab, IconComponent> = {
    appearance: Palette,
    shortcuts: Keyboard,
    sections: ListMusic,
    public: Monitor,
    stage: Tv,
    lower_third: Video,
    lyrics_bg: Sparkles,
    outputs: Sliders,
    songs: Music,
    bibles: BookOpen,
    church: Church,
    files: HardDrive,
    logs: ScrollText,
    about: Info,
  };

  const GROUP_LABELS: Record<string, string> = {
    outputs: 'Projeção & Telas',
    general: 'Geral',
    interface: 'Interface',
    displays: 'Projeção & Telas',
    projection: 'Projeção & Telas',
    content: 'Acervo & Marca',
    library: 'Acervo & Marca',
    system: 'Sistema',
  };

  function cloneJson<T>(val: T): T {
    return JSON.parse(JSON.stringify(val));
  }

  // --- Navegação e Acessibilidade ---
  let activeTab = $state<SettingsTab>('appearance');
  let tabSearch = $state('');
  let openSelectId = $state<string | null>(null);
  let dialogEl = $state<HTMLDivElement | null>(null);
  let wasModalOpen = false;
  let previouslyFocusedEl: HTMLElement | null = null;

  const filteredTabs = $derived.by(() => {
    const q = tabSearch.trim().toLowerCase();
    if (!q) return SETTINGS_TABS;
    return SETTINGS_TABS.filter(
      (t) => t.label.toLowerCase().includes(q) || t.keywords.toLowerCase().includes(q)
    );
  });

  const activeTabMeta = $derived(
    SETTINGS_TABS.find((t) => t.id === activeTab) ?? SETTINGS_TABS[0]
  );

  // --- Estados Locais Editáveis ---
  let editableSections = $state<CustomSectionConfig[]>([]);
  let localPublic = $state<PublicDisplaySettings>({ ...DEFAULT_PUBLIC_SETTINGS });
  let localBrand = $state<ChurchBrandSettings>({ ...DEFAULT_CHURCH_BRAND });
  let localStage = $state<StageDisplaySettings>({ ...DEFAULT_STAGE_SETTINGS });
  let localBgStyles = $state<BackgroundStylesMap>({
    ...DEFAULT_BACKGROUND_STYLES,
    customPresets: [],
    overrides: {},
  });
  let localLowerThird = $state<LowerThirdSettings>({ ...DEFAULT_LOWER_THIRD_SETTINGS });
  let localSongSettings = $state<SongSettings>({ ...DEFAULT_SONG_SETTINGS });

  let savedSnap = $state({
    sections: '',
    public: '',
    brand: '',
    stage: '',
    bg: '',
    lt: '',
    songs: '',
  });

  const sectionsDirty = $derived(open && JSON.stringify(editableSections) !== savedSnap.sections);
  const publicDirty = $derived(open && JSON.stringify(localPublic) !== savedSnap.public);
  const brandDirty = $derived(open && JSON.stringify(localBrand) !== savedSnap.brand);
  const stageDirty = $derived(open && JSON.stringify(localStage) !== savedSnap.stage);
  const bgDirty = $derived(open && JSON.stringify(localBgStyles) !== savedSnap.bg);
  const ltDirty = $derived(open && JSON.stringify(localLowerThird) !== savedSnap.lt);
  const songsDirty = $derived(open && JSON.stringify(localSongSettings) !== savedSnap.songs);
  const hasUnsavedChanges = $derived(
    sectionsDirty ||
      publicDirty ||
      brandDirty ||
      stageDirty ||
      bgDirty ||
      ltDirty ||
      songsDirty
  );

  let isSaving = $state(false);
  let saveMessage = $state('');
  let saveError = $state('');

  // --- Confirmação Centralizada ---
  interface PendingConfirmState {
    title: string;
    message: string;
    confirmLabel: string;
    cancelLabel: string;
    destructive: boolean;
    onConfirm: () => Promise<void> | void;
  }

  let pendingConfirm = $state<PendingConfirmState | null>(null);
  let isConfirmProcessing = $state(false);

  function openConfirm(req: SettingsConfirmRequest) {
    pendingConfirm = {
      title: req.title,
      message: req.description,
      confirmLabel: req.confirmText ?? 'Confirmar',
      cancelLabel: 'Cancelar',
      destructive: req.variant === 'danger',
      onConfirm: req.onConfirm,
    };
  }

  async function executePendingConfirm() {
    if (!pendingConfirm || isConfirmProcessing) return;
    const action = pendingConfirm.onConfirm;
    isConfirmProcessing = true;
    try {
      await action();
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      saveError = msg;
      notifyError('Falha ao executar ação', msg);
    } finally {
      isConfirmProcessing = false;
      pendingConfirm = null;
    }
  }

  // --- Estados Assíncronos por Aba ---
  let systemPaths = $state<SystemPaths | null>(null);
  let isLoadingPaths = $state(false);
  let pathsError = $state('');

  let mediaFolders = $state<MediaFolder[]>([]);
  let isLoadingFolders = $state(false);
  let foldersError = $state('');

  let biblesList = $state<BibleVersionSummary[]>([]);
  let isLoadingBibles = $state(false);
  let bibleError = $state('');

  let actionLogs = $state<ActionLog[]>([]);
  let selectedLogCategory = $state('all');
  let isLoadingLogs = $state(false);
  let logsError = $state('');

  let updateInfo = $state<UpdateCheckResponse | null>(null);
  let updaterStatus = $state<UpdaterStatusPayload | null>(null);
  let isCheckingUpdate = $state(false);
  let updaterError = $state('');
  let isRestarting = $state(false);

  let updaterPollInterval: ReturnType<typeof setInterval> | null = null;
  let restartPollInterval: ReturnType<typeof setInterval> | null = null;

  // Guardas não reativas para evitar loops de repetição em abas sem backend
  const attemptedTabs = {
    bibles: false,
    files: false,
    logs: false,
    about: false,
  };

  function stopUpdaterPolling() {
    if (updaterPollInterval) {
      clearInterval(updaterPollInterval);
      updaterPollInterval = null;
    }
  }

  function stopRestartPolling() {
    if (restartPollInterval) {
      clearInterval(restartPollInterval);
      restartPollInterval = null;
    }
  }

  onDestroy(() => {
    stopUpdaterPolling();
    stopRestartPolling();
  });

  function hydrateDraftsFromStores() {
    editableSections = cloneJson($sectionsStore);
    localPublic = cloneJson({ ...DEFAULT_PUBLIC_SETTINGS, ...$publicSettingsStore });
    localBrand = cloneJson({ ...DEFAULT_CHURCH_BRAND, ...$churchBrandStore });
    localStage = cloneJson({ ...DEFAULT_STAGE_SETTINGS, ...$stageSettingsStore });
    localBgStyles = cloneJson({
      ...DEFAULT_BACKGROUND_STYLES,
      ...$backgroundStylesStore,
      customPresets: $backgroundStylesStore.customPresets || [],
      overrides: $backgroundStylesStore.overrides || {},
    });
    localLowerThird = cloneJson({
      ...DEFAULT_LOWER_THIRD_SETTINGS,
      ...$lowerThirdSettingsStore,
    });
    localSongSettings = cloneJson({ ...DEFAULT_SONG_SETTINGS, ...$songSettingsStore });

    savedSnap = {
      sections: JSON.stringify(editableSections),
      public: JSON.stringify(localPublic),
      brand: JSON.stringify(localBrand),
      stage: JSON.stringify(localStage),
      bg: JSON.stringify(localBgStyles),
      lt: JSON.stringify(localLowerThird),
      songs: JSON.stringify(localSongSettings),
    };
  }

  $effect(() => {
    if (open && !wasModalOpen) {
      wasModalOpen = true;
      previouslyFocusedEl =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      untrack(() => {
        activeTab = initialTab || 'appearance';
        tabSearch = '';
        openSelectId = null;
        saveMessage = '';
        saveError = '';
        pendingConfirm = null;
        pathsError = '';
        foldersError = '';
        bibleError = '';
        logsError = '';
        updaterError = '';
        attemptedTabs.bibles = false;
        attemptedTabs.files = false;
        attemptedTabs.logs = false;
        attemptedTabs.about = false;
        hydrateDraftsFromStores();
      });

      tick().then(() => {
        const activeBtn = Array.from(
          dialogEl?.querySelectorAll<HTMLElement>('[role="tab"][aria-selected="true"]') ?? []
        ).find((el) => el.offsetParent !== null);
        (activeBtn || dialogEl)?.focus({ preventScroll: true });
      });
    } else if (!open && wasModalOpen) {
      wasModalOpen = false;
      untrack(() => {
        stopUpdaterPolling();
        stopRestartPolling();
        pendingConfirm = null;
        openSelectId = null;
      });
      tick().then(() => {
        previouslyFocusedEl?.focus?.();
        previouslyFocusedEl = null;
      });
    }
  });

  // Carregamento sob demanda ao trocar para abas com dados de API
  $effect(() => {
    if (!open) return;
    const current = activeTab;
    untrack(() => {
      if (current === 'bibles' && !attemptedTabs.bibles) {
        attemptedTabs.bibles = true;
        void loadBibles();
      } else if (current === 'files' && !attemptedTabs.files) {
        attemptedTabs.files = true;
        void loadSystemPaths();
        void loadMediaFolders();
      } else if (current === 'logs' && !attemptedTabs.logs) {
        attemptedTabs.logs = true;
        void fetchLogs();
      } else if (current === 'about' && !attemptedTabs.about) {
        attemptedTabs.about = true;
        void refreshUpdaterStatus();
      }
    });
  });

  // --- Loaders Assíncronos ---
  async function loadSystemPaths() {
    isLoadingPaths = true;
    pathsError = '';
    try {
      const res = await fetch('/api/system/paths');
      if (!res.ok) {
        pathsError = await readHttpError(res, 'Não foi possível carregar os caminhos do sistema.');
        return;
      }
      systemPaths = await res.json();
    } catch (err) {
      pathsError = err instanceof Error ? err.message : 'Falha de conexão ao consultar caminhos.';
    } finally {
      isLoadingPaths = false;
    }
  }

  async function loadMediaFolders() {
    isLoadingFolders = true;
    foldersError = '';
    try {
      const res = await fetch('/api/media-folders');
      if (!res.ok) {
        foldersError = await readHttpError(res, 'Não foi possível carregar pastas vinculadas.');
        return;
      }
      mediaFolders = await res.json();
    } catch (err) {
      foldersError = err instanceof Error ? err.message : 'Falha ao consultar pastas.';
    } finally {
      isLoadingFolders = false;
    }
  }

  async function loadBibles() {
    isLoadingBibles = true;
    bibleError = '';
    try {
      const res = await fetch('/api/bibles');
      if (!res.ok) {
        bibleError = await readHttpError(res, 'Falha ao listar Bíblias instaladas.');
        return;
      }
      biblesList = await res.json();
      onbibleschanged?.();
    } catch (err) {
      bibleError = err instanceof Error ? err.message : 'Erro de conexão ao buscar Bíblias.';
    } finally {
      isLoadingBibles = false;
    }
  }

  async function fetchLogs() {
    isLoadingLogs = true;
    logsError = '';
    try {
      const url =
        selectedLogCategory === 'all'
          ? '/api/logs?limit=100'
          : `/api/logs?limit=100&category=${encodeURIComponent(selectedLogCategory)}`;
      const res = await fetch(url);
      if (!res.ok) {
        logsError = await readHttpError(res, 'Falha ao carregar histórico de auditoria.');
        return;
      }
      actionLogs = await res.json();
    } catch (err) {
      logsError = err instanceof Error ? err.message : 'Erro de conexão ao consultar logs.';
    } finally {
      isLoadingLogs = false;
    }
  }

  function handleSelectLogCategory(category: string) {
    selectedLogCategory = category;
    void fetchLogs();
  }

  function requestClearLogs() {
    openConfirm({
      title: 'Limpar Histórico de Auditoria?',
      description: 'Deseja remover permanentemente todos os registros de logs do sistema?',
      confirmText: 'Limpar Logs',
      variant: 'danger',
      onConfirm: async () => {
        try {
          const res = await fetch('/api/logs', { method: 'DELETE' });
          if (!res.ok) {
            const msg = await readHttpError(res, 'Falha ao limpar logs.');
            logsError = msg;
            notifyError('Erro ao limpar logs', msg);
            return;
          }
          actionLogs = [];
          notifySuccess('Histórico limpo com sucesso');
          await fetchLogs();
        } catch (err) {
          const msg = err instanceof Error ? err.message : String(err);
          logsError = msg;
          notifyError('Erro de conexão', msg);
        }
      },
    });
  }

  async function refreshUpdaterStatus() {
    try {
      const status = await fetchUpdaterStatus();
      updaterStatus = status;
      if (status.last_check) {
        updateInfo = status.last_check;
      }
      if (status.stage.type === 'Downloading' || status.stage.type === 'Extracting') {
        startUpdaterPolling();
      }
    } catch (err) {
      // Silencioso apenas no bootstrap quando offline/preview sem API, mas guarda se houver mensagem explícita
      updaterError = err instanceof Error ? err.message : '';
    }
  }

  function startUpdaterPolling() {
    if (updaterPollInterval) return;
    updaterPollInterval = setInterval(async () => {
      try {
        const status = await fetchUpdaterStatus();
        updaterStatus = status;
        if (status.stage.type === 'ReadyToRestart' || status.stage.type === 'Error') {
          stopUpdaterPolling();
          if (status.stage.type === 'Error') {
            updaterError = status.stage.message;
          }
        }
      } catch {
        stopUpdaterPolling();
      }
    }, 1500);
  }

  async function handleCheckUpdates() {
    isCheckingUpdate = true;
    updaterError = '';
    try {
      const info = await fetchUpdateCheck();
      updateInfo = info;
      await refreshUpdaterStatus();
    } catch (err) {
      updaterError = err instanceof Error ? err.message : 'Falha ao verificar atualizações.';
    } finally {
      isCheckingUpdate = false;
    }
  }

  async function handleStartUpdate() {
    if (!updateInfo) return;
    updaterError = '';
    try {
      await triggerInstallUpdate({
        download_url: updateInfo.download_url || undefined,
        asset_name: updateInfo.asset_name || undefined,
        target_version: updateInfo.latest_version,
      });
      startUpdaterPolling();
      await refreshUpdaterStatus();
    } catch (err) {
      updaterError = err instanceof Error ? err.message : 'Falha ao iniciar download da atualização.';
    }
  }

  function requestRestartApplication() {
    openConfirm({
      title: 'Reiniciar Sistema?',
      description:
        'O servidor do OpenPresenter será reiniciado para aplicar a nova versão ou restaurar o estado. Telas conectadas ficarão brevemente indisponíveis.',
      confirmText: 'Reiniciar Agora',
      variant: 'danger',
      onConfirm: executeRestartApplication,
    });
  }

  async function executeRestartApplication() {
    if (isRestarting) return;
    isRestarting = true;
    updaterError = '';
    stopRestartPolling();
    try {
      await triggerSystemRestart();
      let attempts = 0;
      restartPollInterval = setInterval(async () => {
        attempts++;
        try {
          const res = await fetch('/api/status');
          if (res.ok) {
            stopRestartPolling();
            window.location.reload();
            return;
          }
        } catch {
          // Aguarda enquanto o processo reinicia
        }
        if (attempts > 40) {
          stopRestartPolling();
          isRestarting = false;
          updaterError =
            'O servidor demorou para responder após o reinício. Tente recarregar a página.';
        }
      }, 1000);
    } catch (err) {
      isRestarting = false;
      updaterError =
        err instanceof Error ? err.message : 'Falha ao solicitar reinicialização do sistema.';
    }
  }

  // --- Manipulação de Rascunho e Salvamento ---
  function selectTab(tabId: SettingsTab) {
    activeTab = tabId;
    openSelectId = null;
  }

  function handleSectionNameChange(idx: number, val: string) {
    editableSections = editableSections.map((sec, i) =>
      i === idx ? { ...sec, name: val } : sec
    );
  }

  function handleSectionShortcutChange(idx: number, val: string) {
    const clean = val.trim().slice(0, 1).toLowerCase();
    editableSections = editableSections.map((sec, i) =>
      i === idx ? { ...sec, shortcut: clean } : sec
    );
  }

  function handleSectionColorChange(idx: number, colorId: string) {
    const preset = COLOR_PRESETS.find((p) => p.id === colorId);
    if (!preset) return;
    editableSections = editableSections.map((sec, i) =>
      i === idx
        ? {
            ...sec,
            colorName: preset.name,
            bg: preset.bg,
            text: preset.text,
            border: preset.border,
            accent: preset.accent,
          }
        : sec
    );
  }

  function handleResetSections() {
    resetSectionsToDefault();
    editableSections = cloneJson(DEFAULT_SECTIONS);
    savedSnap = { ...savedSnap, sections: JSON.stringify(editableSections) };
    notifySuccess('Seções restauradas para o padrão');
  }

  function handleUpdateBrandField<K extends keyof ChurchBrandSettings>(
    key: K,
    val: ChurchBrandSettings[K]
  ) {
    localBrand = { ...localBrand, [key]: val };
  }

  async function handleSelectWallpaperFit(fit: ImageFitMode | 'repeat') {
    const next = { ...localPublic, wallpaper_fit: fit };
    localPublic = next;
    setWallpaperFit(fit);
    saveError = '';
    try {
      await savePublicDisplaySettings(next);
      savedSnap = { ...savedSnap, public: JSON.stringify(next) };
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Falha ao salvar ajuste de papel de parede.';
      saveError = msg;
      notifyError('Falha ao salvar ajuste', msg);
    }
  }

  async function handleSelectLinesPerSlide(lines: number) {
    const next = { ...localSongSettings, lines_per_slide: lines };
    localSongSettings = next;
    saveError = '';
    try {
      await saveSongSettings(next);
      savedSnap = { ...savedSnap, songs: JSON.stringify(next) };
      notifySuccess(
        'Divisão de letras atualizada',
        lines === 0 ? 'Sem divisão automática' : `${lines} linha(s) por slide`
      );
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Falha ao salvar preferência de músicas.';
      saveError = msg;
      notifyError('Erro ao salvar', msg);
    }
  }

  async function handleSaveBrandNow(next: ChurchBrandSettings) {
    localBrand = next;
    saveError = '';
    try {
      await saveChurchBrandSettings(next);
      savedSnap = { ...savedSnap, brand: JSON.stringify(next) };
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Falha ao salvar identidade da igreja.';
      saveError = msg;
      notifyError('Erro ao salvar identidade', msg);
    }
  }

  async function handleSaveLowerThirdNow(next: LowerThirdSettings) {
    localLowerThird = next;
    saveError = '';
    try {
      await saveLowerThirdSettings(next);
      savedSnap = { ...savedSnap, lt: JSON.stringify(next) };
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Falha ao salvar Lower Third.';
      saveError = msg;
      notifyError('Erro ao salvar Lower Third', msg);
    }
  }

  async function saveAllChanges() {
    if (isSaving || !hasUnsavedChanges) return;
    isSaving = true;
    saveMessage = '';
    saveError = '';

    const errors: string[] = [];

    if (sectionsDirty) {
      saveSections(editableSections);
      savedSnap = { ...savedSnap, sections: JSON.stringify(editableSections) };
    }

    const tasks: Array<Promise<void>> = [];

    if (publicDirty) {
      const snapshotTarget = cloneJson(localPublic);
      tasks.push(
        savePublicDisplaySettings(snapshotTarget).then(() => {
          savedSnap = { ...savedSnap, public: JSON.stringify(snapshotTarget) };
        })
      );
    }
    if (brandDirty) {
      const snapshotTarget = cloneJson(localBrand);
      tasks.push(
        saveChurchBrandSettings(snapshotTarget).then(() => {
          savedSnap = { ...savedSnap, brand: JSON.stringify(snapshotTarget) };
        })
      );
    }
    if (stageDirty) {
      const snapshotTarget = cloneJson(localStage);
      tasks.push(
        saveStageDisplaySettings(snapshotTarget).then(() => {
          savedSnap = { ...savedSnap, stage: JSON.stringify(snapshotTarget) };
        })
      );
    }
    if (bgDirty) {
      const snapshotTarget = cloneJson(localBgStyles);
      tasks.push(
        saveBackgroundStyles(snapshotTarget).then(() => {
          savedSnap = { ...savedSnap, bg: JSON.stringify(snapshotTarget) };
        })
      );
    }
    if (ltDirty) {
      const snapshotTarget = cloneJson(localLowerThird);
      tasks.push(
        saveLowerThirdSettings(snapshotTarget).then(() => {
          savedSnap = { ...savedSnap, lt: JSON.stringify(snapshotTarget) };
        })
      );
    }
    if (songsDirty) {
      const snapshotTarget = cloneJson(localSongSettings);
      tasks.push(
        saveSongSettings(snapshotTarget).then(() => {
          savedSnap = { ...savedSnap, songs: JSON.stringify(snapshotTarget) };
        })
      );
    }

    const settled = await Promise.allSettled(tasks);
    for (const res of settled) {
      if (res.status === 'rejected') {
        errors.push(
          res.reason instanceof Error ? res.reason.message : 'Erro desconhecido ao salvar.'
        );
      }
    }

    isSaving = false;

    if (errors.length > 0) {
      saveError = errors[0];
      notifyError('Falha ao salvar configurações', errors[0]);
      return;
    }

    saveMessage = 'Alterações salvas e sincronizadas.';
    notifySuccess('Configurações salvas');
    setTimeout(() => {
      saveMessage = '';
    }, 3000);
  }

  function requestClose() {
    if (isRestarting) return;
    if (hasUnsavedChanges) {
      openConfirm({
        title: 'Descartar alterações não salvas?',
        description:
          'Há ajustes modificados nesta sessão que ainda não foram gravados no servidor. Deseja descartá-los e fechar?',
        confirmText: 'Descartar e Fechar',
        variant: 'danger',
        onConfirm: () => {
          pendingConfirm = null;
          onclose();
        },
      });
      return;
    }
    onclose();
  }

  // --- Teclado: Esc em Camadas, Tab Focus Trap, Navegação Roving em Tablist ---
  function getFocusableElements(): HTMLElement[] {
    if (!dialogEl) return [];
    const selector =
      'a[href], button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex]:not([tabindex="-1"])';
    return Array.from(dialogEl.querySelectorAll<HTMLElement>(selector)).filter((el) => {
      if (el.getAttribute('aria-hidden') === 'true') return false;
      return el.offsetParent !== null || el === document.activeElement;
    });
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (!open) return;

    const hasNestedDialog =
      typeof document !== 'undefined' &&
      Boolean(
        document.querySelector('[role="dialog"][aria-modal="true"]:not([data-settings-shell="true"])')
      );

    if (e.key === 'Escape') {
      if (e.defaultPrevented || pendingConfirm || hasNestedDialog) {
        return;
      }
      e.preventDefault();
      if (openSelectId !== null) {
        openSelectId = null;
        return;
      }
      requestClose();
      return;
    }

    if (e.key === 'Tab' && !pendingConfirm && !hasNestedDialog) {
      const focusables = getFocusableElements();
      if (focusables.length === 0) return;
      const first = focusables[0];
      const last = focusables[focusables.length - 1];
      const active = document.activeElement as HTMLElement | null;

      if (e.shiftKey) {
        if (!active || !dialogEl?.contains(active) || active === first) {
          e.preventDefault();
          last.focus();
        }
      } else {
        if (!active || !dialogEl?.contains(active) || active === last) {
          e.preventDefault();
          first.focus();
        }
      }
    }
  }

  function handleTablistKeydown(e: KeyboardEvent) {
    if (filteredTabs.length === 0) return;
    const currentIdx = filteredTabs.findIndex((t) => t.id === activeTab);
    let nextIdx = -1;

    if (e.key === 'ArrowDown' || e.key === 'ArrowRight') {
      e.preventDefault();
      nextIdx = currentIdx < 0 ? 0 : (currentIdx + 1) % filteredTabs.length;
    } else if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') {
      e.preventDefault();
      nextIdx =
        currentIdx <= 0 ? filteredTabs.length - 1 : currentIdx - 1;
    } else if (e.key === 'Home') {
      e.preventDefault();
      nextIdx = 0;
    } else if (e.key === 'End') {
      e.preventDefault();
      nextIdx = filteredTabs.length - 1;
    }

    if (nextIdx >= 0) {
      const targetTab = filteredTabs[nextIdx];
      const container = e.currentTarget as HTMLElement | null;
      selectTab(targetTab.id);
      tick().then(() => {
        container
          ?.querySelector<HTMLElement>(`[data-tab-id="${targetTab.id}"]`)
          ?.focus();
      });
    }
  }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

{#if open}
  <!-- Backdrop e Diálogo Principal -->
  <div
    data-settings-shell="true"
    class="fixed inset-0 z-50 bg-black/65 backdrop-blur-sm flex items-center justify-center p-3 sm:p-5 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
    aria-labelledby="settings-modal-title"
    tabindex="-1"
    onclick={(e) => {
      if (e.target === e.currentTarget) requestClose();
    }}
    onkeydown={(e) => {
      if (e.key === 'Escape' && e.target === e.currentTarget) {
        e.preventDefault();
        requestClose();
      }
    }}
  >
    <div
      bind:this={dialogEl}
      class="relative w-full max-w-6xl h-[90vh] max-h-[820px] rounded-2xl border border-border bg-card text-foreground shadow-2xl flex flex-col overflow-hidden"
    >
      <!-- Cabeçalho -->
      <header class="px-5 py-3.5 border-b border-border bg-card flex items-center justify-between gap-3 shrink-0">
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="w-8 h-8 rounded-lg bg-primary/15 border border-primary/25 text-primary flex items-center justify-center shrink-0">
            <Settings size={16} />
          </div>
          <div class="min-w-0">
            <h2 id="settings-modal-title" class="text-sm font-bold text-foreground truncate">
              Configurações — {activeTabMeta.label}
            </h2>
            <p class="text-[11px] text-muted-foreground truncate">
              Preferências do aplicativo, saídas gráficas, bíblias, acervo e sistema
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          {#if hasUnsavedChanges}
            <span class="hidden sm:inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[11px] font-medium bg-amber-500/15 text-amber-800 dark:text-amber-300 border border-amber-500/35">
              Alterações não salvas
            </span>
          {/if}

          <button
            type="button"
            onclick={requestClose}
            aria-label="Fechar configurações"
            class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          >
            <X size={16} />
          </button>
        </div>
      </header>

      <!-- Barra Compacta de Abas para Telas Menores (< md) -->
      <div class="md:hidden border-b border-border bg-muted/30 px-3 py-2 space-y-2 shrink-0">
        <div class="relative">
          <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <input
            type="search"
            bind:value={tabSearch}
            placeholder="Filtrar configurações..."
            aria-label="Filtrar abas de configurações"
            class="w-full h-8 pl-8 pr-3 rounded-lg bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>
        <div
          role="tablist"
          aria-label="Seções de configurações"
          aria-orientation="horizontal"
          tabindex="-1"
          onkeydown={handleTablistKeydown}
          class="flex items-center gap-1 overflow-x-auto pb-0.5"
        >
          {#each filteredTabs as tab (tab.id)}
            {@const IconComp = TAB_ICONS[tab.id]}
            {@const isActive = activeTab === tab.id}
            <button
              type="button"
              role="tab"
              data-tab-id={tab.id}
              aria-selected={isActive}
              tabindex={isActive ? 0 : -1}
              onclick={() => selectTab(tab.id)}
              class="min-h-8 px-2.5 py-1.5 rounded-lg text-xs font-medium inline-flex items-center gap-1.5 whitespace-nowrap shrink-0 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isActive ? 'bg-primary text-primary-foreground shadow-sm font-semibold' : 'text-muted-foreground hover:text-foreground hover:bg-muted/70'}"
            >
              <IconComp size={13} class="shrink-0" />
              <span>{tab.label}</span>
            </button>
          {/each}
        </div>
      </div>

      <!-- Corpo Principal: Sidebar Desktop + Conteúdo -->
      <div class="flex-1 min-h-0 flex overflow-hidden">
        <!-- Sidebar Desktop -->
        <aside class="hidden md:flex md:w-56 lg:w-60 shrink-0 flex-col border-r border-border bg-muted/25">
          <div class="p-3 border-b border-border/70">
            <div class="relative">
              <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
              <input
                type="search"
                bind:value={tabSearch}
                placeholder="Buscar ajuste..."
                aria-label="Buscar aba de configurações"
                class="w-full h-8 pl-8 pr-2.5 rounded-lg bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              />
            </div>
          </div>

          <nav
            aria-label="Categorias de configurações"
            class="flex-1 overflow-y-auto p-2.5"
          >
            <div
              role="tablist"
              aria-label="Categorias de configurações"
              aria-orientation="vertical"
              tabindex="-1"
              onkeydown={handleTablistKeydown}
              class="space-y-0.5"
            >
              {#if filteredTabs.length === 0}
                <p class="px-2 py-4 text-xs text-muted-foreground text-center">
                  Nenhuma aba encontrada para "{tabSearch}".
                </p>
              {:else}
                {#each filteredTabs as tab, idx (tab.id)}
                  {@const prevGroup = idx > 0 ? filteredTabs[idx - 1].group : null}
                  {@const showGroupLabel = tab.group !== prevGroup}
                  {@const IconComp = TAB_ICONS[tab.id]}
                  {@const isActive = activeTab === tab.id}

                  {#if showGroupLabel}
                    <div class="{idx > 0 ? 'pt-3 mt-2 border-t border-border/60' : 'pt-0.5'} px-2.5 pb-1 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">
                      {GROUP_LABELS[tab.group] ?? tab.group}
                    </div>
                  {/if}

                  <button
                    type="button"
                    role="tab"
                    id="settings-tab-{tab.id}"
                    data-tab-id={tab.id}
                    aria-controls="settings-tabpanel"
                    aria-selected={isActive}
                    tabindex={isActive ? 0 : -1}
                    onclick={() => selectTab(tab.id)}
                    class="w-full min-h-8 px-2.5 py-1.5 rounded-lg text-xs text-left flex items-center gap-2.5 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isActive ? 'bg-primary/15 text-primary font-semibold border border-primary/30' : 'text-muted-foreground hover:text-foreground hover:bg-muted/70 border border-transparent'}"
                  >
                    <IconComp size={14} class="shrink-0" />
                    <span class="truncate">{tab.label}</span>
                  </button>
                {/each}
              {/if}
            </div>
          </nav>
        </aside>

        <!-- Área da Aba Ativa -->
        <main class="flex-1 min-w-0 overflow-y-auto p-4 sm:p-6 bg-background/40">
          <div
            id="settings-tabpanel"
            role="tabpanel"
            aria-labelledby="settings-tab-{activeTab}"
            tabindex="-1"
          >
          {#if activeTab === 'appearance'}
            <AppearanceTab
              {openSelectId}
              onSetOpenSelect={(id) => (openSelectId = id)}
            />
          {:else if activeTab === 'shortcuts'}
            <ShortcutsTab />
          {:else if activeTab === 'sections'}
            <SectionsTab
              sections={editableSections}
              onNameChange={handleSectionNameChange}
              onShortcutChange={handleSectionShortcutChange}
              onColorChange={handleSectionColorChange}
              onReset={handleResetSections}
            />
          {:else if activeTab === 'public'}
            <PublicDisplayTab
              {localPublic}
              onUpdatePublic={(next) => (localPublic = next)}
              onSelectWallpaperFit={handleSelectWallpaperFit}
            />
          {:else if activeTab === 'stage'}
            <StageMonitorTab
              {localStage}
              onUpdateStage={(next) => (localStage = next)}
            />
          {:else if activeTab === 'lower_third'}
            <LowerThirdTab
              {localLowerThird}
              onUpdateLowerThird={(next) => (localLowerThird = next)}
              onSaveLowerThirdNow={handleSaveLowerThirdNow}
              onRequestConfirm={openConfirm}
            />
          {:else if activeTab === 'lyrics_bg'}
            <LyricsBackgroundsTab
              {localBgStyles}
              {localPublic}
              onUpdateStyles={(next) => (localBgStyles = next)}
              onRequestConfirm={openConfirm}
            />
          {:else if activeTab === 'outputs'}
            <OutputsTab
              {localPublic}
              onSelectWallpaperFit={handleSelectWallpaperFit}
              onNavigateToPublicTab={() => selectTab('public')}
            />
          {:else if activeTab === 'songs'}
            <SongsTab
              songSettings={localSongSettings}
              onSelectLinesPerSlide={handleSelectLinesPerSlide}
            />
          {:else if activeTab === 'bibles'}
            <BiblesTab
              {biblesList}
              {isLoadingBibles}
              {bibleError}
              onReloadBibles={loadBibles}
              onDismissError={() => (bibleError = '')}
              onRequestConfirm={openConfirm}
            />
          {:else if activeTab === 'church'}
            <ChurchBrandTab
              {localBrand}
              onUpdateField={handleUpdateBrandField}
              onSaveBrandNow={handleSaveBrandNow}
              onRequestConfirm={openConfirm}
            />
          {:else if activeTab === 'files'}
            <FilesTab
              {systemPaths}
              {isLoadingPaths}
              {pathsError}
              {mediaFolders}
              {isLoadingFolders}
              {foldersError}
              onRetryPaths={() => void loadSystemPaths()}
              onReloadFolders={loadMediaFolders}
              onRequestConfirm={openConfirm}
            />
          {:else if activeTab === 'logs'}
            <LogsTab
              {actionLogs}
              selectedCategory={selectedLogCategory}
              isLoading={isLoadingLogs}
              error={logsError}
              onSelectCategory={handleSelectLogCategory}
              onRefresh={() => void fetchLogs()}
              onRequestClear={requestClearLogs}
            />
          {:else if activeTab === 'about'}
            <AboutTab
              {updateInfo}
              {updaterStatus}
              {isCheckingUpdate}
              {isRestarting}
              {updaterError}
              onCheckUpdates={handleCheckUpdates}
              onStartUpdate={handleStartUpdate}
              onRequestRestart={requestRestartApplication}
            />
          {/if}
          </div>
        </main>
      </div>

      <!-- Rodapé com Status Acessível e Ações -->
      <footer class="px-5 py-3 border-t border-border bg-card flex flex-wrap items-center justify-between gap-3 shrink-0">
        <div class="min-w-0 flex-1">
          {#if saveError}
            <p role="alert" class="text-xs text-red-700 dark:text-red-300 font-medium truncate">
              {saveError}
            </p>
          {:else if saveMessage}
            <p role="status" class="text-xs text-emerald-700 dark:text-emerald-300 font-medium flex items-center gap-1.5">
              <Check size={14} class="shrink-0" />
              <span>{saveMessage}</span>
            </p>
          {:else if hasUnsavedChanges}
            <p role="status" class="text-xs text-amber-800 dark:text-amber-300">
              Você possui alterações não salvas no servidor.
            </p>
          {:else}
            <p class="text-[11px] text-muted-foreground">
              Todas as configurações estão sincronizadas.
            </p>
          {/if}
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <Button variant="ghost" size="sm" onclick={requestClose} class="text-xs">
            Fechar
          </Button>
          <Button
            variant="default"
            size="sm"
            disabled={!hasUnsavedChanges || isSaving}
            onclick={saveAllChanges}
            class="gap-1.5 text-xs"
          >
            {#if isSaving}
              <RefreshCw size={13} class="animate-spin" />
              <span>Salvando...</span>
            {:else}
              <Save size={13} />
              <span>Salvar Alterações</span>
            {/if}
          </Button>
        </div>
      </footer>

      <!-- Sobreposição durante Reinicialização -->
      {#if isRestarting}
        <div
          role="status"
          aria-live="polite"
          class="absolute inset-0 z-20 bg-background/95 backdrop-blur-sm flex flex-col items-center justify-center p-6 text-center gap-3"
        >
          <RefreshCw size={28} class="text-primary animate-spin" />
          <div class="text-sm font-bold text-foreground">Reiniciando servidor do OpenPresenter...</div>
          <p class="text-xs text-muted-foreground max-w-sm">
            A página será recarregada automaticamente assim que o serviço voltar a responder.
          </p>
        </div>
      {/if}
    </div>
  </div>

  <!-- Modal Unificado de Confirmação -->
  <ConfirmModal
    open={pendingConfirm !== null}
    title={pendingConfirm?.title ?? ''}
    message={pendingConfirm?.message ?? ''}
    confirmLabel={pendingConfirm?.confirmLabel ?? 'Confirmar'}
    cancelLabel={pendingConfirm?.cancelLabel ?? 'Cancelar'}
    destructive={pendingConfirm?.destructive ?? false}
    isProcessing={isConfirmProcessing}
    onconfirm={executePendingConfirm}
    onclose={() => {
      if (!isConfirmProcessing) pendingConfirm = null;
    }}
  />
{/if}

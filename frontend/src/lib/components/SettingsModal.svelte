<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import {
    X,
    Sun,
    Moon,
    Laptop,
    Palette,
    Monitor,
    Keyboard,
    Info,
    RotateCcw,
    Check,
    Tv,
    ExternalLink,
    HardDrive,
    FolderPlus,
    Folder,
    Trash2,
    AlertCircle,
    ChevronDown,
    Copy,
    FolderOpen,
    Sliders,
    Type,
    Layers,
    Upload,
    Church,
    Maximize2,
    Save,
    Image as ImageIcon,
    BookOpen,
    Edit3,
    Plus,
    CheckCircle2,
    Download,
    RefreshCw,
    Sparkles,
    AlertTriangle,
    Music,
    Clock,
    Video,
  } from 'lucide-svelte';
  import {
    fetchUpdateCheck,
    fetchUpdaterStatus,
    triggerInstallUpdate,
    type UpdateCheckResponse,
    type UpdaterStatusPayload,
  } from '../updater';
  import {
    themeStore,
    baseColorStore,
    accentThemeStore,
    sectionsStore,
    wallpaperFitStore,
    previewLabelsStore,
    setTheme,
    setBaseColor,
    setAccentTheme,
    setWallpaperFit,
    setPreviewLabels,
    saveSections,
    resetSectionsToDefault,
    COLOR_PRESETS,
    BASE_COLOR_PRESETS,
    ACCENT_THEME_PRESETS,
    type ThemeMode,
    type BaseColor,
    type AccentTheme,
    type WallpaperFitMode,
    type CustomSectionConfig,
  } from '../settings';
  import {
    publicSettingsStore,
    churchBrandStore,
    stageSettingsStore,
    backgroundStylesStore,
    lowerThirdSettingsStore,
    savePublicDisplaySettings,
    saveChurchBrandSettings,
    saveStageDisplaySettings,
    saveBackgroundStyles,
    saveLowerThirdSettings,
    DEFAULT_PUBLIC_SETTINGS,
    DEFAULT_CHURCH_BRAND,
    DEFAULT_STAGE_SETTINGS,
    DEFAULT_BACKGROUND_STYLES,
    DEFAULT_LOWER_THIRD_SETTINGS,
  } from '../displaySettings';
  import {
    getAllBackgrounds,
    BACKGROUND_PRESETS,
    type BackgroundPreset,
    type BackgroundStylesMap,
  } from '../backgrounds';
  import { biblesStore, refreshBibles } from '../bibles';
  import Badge from './ui/Badge.svelte';
  import Button from './ui/Button.svelte';
  import StagePreviewBox from './StagePreviewBox.svelte';
  import type {
    MediaFolder,
    SystemPaths,
    PublicDisplaySettings,
    ChurchBrandSettings,
    StageDisplaySettings,
    LowerThirdSettings,
    StagePreset,
    BibleVersionSummary,
  } from '../types';

  export type SettingsTab =
    | 'appearance'
    | 'public'
    | 'lyrics_bg'
    | 'stage'
    | 'lower_third'
    | 'church'
    | 'bibles'
    | 'files'
    | 'sections'
    | 'outputs'
    | 'shortcuts'
    | 'about';

  interface Props {
    open: boolean;
    initialTab?: SettingsTab;
    onclose: () => void;
    onbibleschanged?: () => void;
  }

  let { open = false, initialTab = 'appearance', onclose, onbibleschanged }: Props = $props();

  let activeTab = $state<SettingsTab>('appearance');
  let editableSections = $state<CustomSectionConfig[]>([]);

  // Configurações de Telas e Marca da Igreja
  let localPublic = $state<PublicDisplaySettings>({ ...DEFAULT_PUBLIC_SETTINGS });
  let localBrand = $state<ChurchBrandSettings>({ ...DEFAULT_CHURCH_BRAND });
  let localStage = $state<StageDisplaySettings>({ ...DEFAULT_STAGE_SETTINGS });
  let localLowerThird = $state<LowerThirdSettings>({ ...DEFAULT_LOWER_THIRD_SETTINGS });
  let uploadStatus = $state<{ [key: string]: string }>({});

  // Sub-seções internas
  let publicSection = $state<'lyrics' | 'scripture' | 'general'>('lyrics');
  let stageSection = $state<'lyrics' | 'scripture' | 'general'>('lyrics');
  let bgPreviewMode = $state<'lyrics' | 'scripture'>('lyrics');
  let ltPreviewMode = $state<'lyrics' | 'scripture'>('lyrics');

  // Configurações de Fundos de Letras e CSS
  let localBgStyles = $state<BackgroundStylesMap>({
    defaultBackgroundId: undefined,
    customPresets: [],
    overrides: {},
  });
  let selectedPresetId = $state<string>('grad-obsidian');
  let bgCategoryFilter = $state<'all' | 'gradients' | 'mesh' | 'animated' | 'custom'>('all');
  let bgSearchTerm = $state('');

  const QUICK_TEXT_COLORS = [
    { label: 'Branco Puro', hex: '#FFFFFF' },
    { label: 'Off-White', hex: '#F8FAFC' },
    { label: 'Ouro / Dourado', hex: '#FACC15' },
    { label: 'Âmbar Quente', hex: '#F59E0B' },
    { label: 'Amarelo Suave', hex: '#FEF08A' },
    { label: 'Ciano Gelo', hex: '#67E8F9' },
    { label: 'Menta Verde', hex: '#86EFAC' },
    { label: 'Rosa Claro', hex: '#F472B6' },
  ];

  const QUICK_GRADIENTS = [
    { name: 'Índigo Noturno', css: 'linear-gradient(135deg, #090d16 0%, #1e1b4b 60%, #020617 100%)' },
    { name: 'Púrpura Profundo', css: 'linear-gradient(135deg, #18052e 0%, #3b0764 50%, #090114 100%)' },
    { name: 'Vinho Nobre', css: 'linear-gradient(135deg, #1a050d 0%, #4c0519 50%, #0a0105 100%)' },
    { name: 'Floresta Escura', css: 'linear-gradient(135deg, #022c22 0%, #064e3b 50%, #021a14 100%)' },
    { name: 'Azul Meia-Noite', css: 'linear-gradient(135deg, #030712 0%, #0c4a6e 50%, #020617 100%)' },
    { name: 'Radial Sombrio', css: 'radial-gradient(ellipse at center, #2e1a05 0%, #0d0a06 60%, #000000 100%)' },
    { name: 'Mesh Aurora', css: 'radial-gradient(at 0% 0%, #312e81 0px, transparent 50%), radial-gradient(at 100% 0%, #701a75 0px, transparent 50%), radial-gradient(at 100% 100%, #1e1b4b 0px, transparent 50%), #020617' },
  ];

  const allAvailableBackgrounds = $derived(getAllBackgrounds(localBgStyles));

  const filteredBgList = $derived.by(() => {
    return allAvailableBackgrounds.filter((p) => {
      const matchesCat = bgCategoryFilter === 'all' || p.category === bgCategoryFilter;
      const matchesSearch =
        bgSearchTerm === '' ||
        p.name.toLowerCase().includes(bgSearchTerm.toLowerCase()) ||
        p.id.toLowerCase().includes(bgSearchTerm.toLowerCase());
      return matchesCat && matchesSearch;
    });
  });

  const currentSelectedPreset = $derived.by<BackgroundPreset>(() => {
    return (
      allAvailableBackgrounds.find((p) => p.id === selectedPresetId) ||
      allAvailableBackgrounds[0] || {
        id: 'grad-obsidian',
        name: 'Obsidiana Escuro',
        category: 'gradients',
        style: 'linear-gradient(135deg, #09090b 0%, #18181b 50%, #09090b 100%)',
        textColor: '#ffffff',
      }
    );
  });

  function updateCurrentTextColor(color: string) {
    if (!currentSelectedPreset) return;
    if (currentSelectedPreset.isCustom) {
      const idx = localBgStyles.customPresets.findIndex((p) => p.id === currentSelectedPreset.id);
      if (idx !== -1) {
        localBgStyles.customPresets[idx] = { ...localBgStyles.customPresets[idx], textColor: color };
      }
    } else {
      const current = localBgStyles.overrides[currentSelectedPreset.id] || {};
      localBgStyles.overrides[currentSelectedPreset.id] = { ...current, textColor: color };
    }
  }

  function updateCurrentStyle(styleCss: string) {
    if (!currentSelectedPreset) return;
    if (currentSelectedPreset.isCustom) {
      const idx = localBgStyles.customPresets.findIndex((p) => p.id === currentSelectedPreset.id);
      if (idx !== -1) {
        localBgStyles.customPresets[idx] = { ...localBgStyles.customPresets[idx], style: styleCss };
      }
    } else {
      const current = localBgStyles.overrides[currentSelectedPreset.id] || {};
      localBgStyles.overrides[currentSelectedPreset.id] = { ...current, style: styleCss };
    }
  }

  function updateCurrentAnimation(animClass: string) {
    if (!currentSelectedPreset) return;
    const finalAnim = animClass === 'none' ? undefined : animClass;
    if (currentSelectedPreset.isCustom) {
      const idx = localBgStyles.customPresets.findIndex((p) => p.id === currentSelectedPreset.id);
      if (idx !== -1) {
        localBgStyles.customPresets[idx] = { ...localBgStyles.customPresets[idx], animationClass: finalAnim };
      }
    } else {
      const current = localBgStyles.overrides[currentSelectedPreset.id] || {};
      localBgStyles.overrides[currentSelectedPreset.id] = { ...current, animationClass: finalAnim };
    }
  }

  function updateCurrentName(name: string) {
    if (!currentSelectedPreset || !currentSelectedPreset.isCustom) return;
    const idx = localBgStyles.customPresets.findIndex((p) => p.id === currentSelectedPreset.id);
    if (idx !== -1) {
      localBgStyles.customPresets[idx] = { ...localBgStyles.customPresets[idx], name };
    }
  }

  function createNewCustomPreset() {
    const newId = `custom-${Date.now()}`;
    const newPreset: BackgroundPreset = {
      id: newId,
      name: 'Novo Fundo Customizado',
      category: 'custom',
      style: 'linear-gradient(135deg, #0f172a 0%, #1e1b4b 50%, #312e81 100%)',
      textColor: '#ffffff',
      animationClass: undefined,
      isCustom: true,
    };
    localBgStyles.customPresets = [...localBgStyles.customPresets, newPreset];
    selectedPresetId = newId;
    bgCategoryFilter = 'custom';
  }

  function deleteCustomPreset(id: string) {
    localBgStyles.customPresets = localBgStyles.customPresets.filter((p) => p.id !== id);
    if (localBgStyles.defaultBackgroundId === id) {
      localBgStyles.defaultBackgroundId = undefined;
    }
    selectedPresetId = 'grad-obsidian';
  }

  function restorePresetDefault(id: string) {
    if (localBgStyles.overrides[id]) {
      const copy = { ...localBgStyles.overrides };
      delete copy[id];
      localBgStyles.overrides = copy;
    }
  }

  function setDefaultBackground(id: string) {
    localBgStyles.defaultBackgroundId = localBgStyles.defaultBackgroundId === id ? undefined : id;
  }

  // Feedback de salvamento
  let wasModalOpen = false;
  let isSaving = $state(false);
  let saveStatusMessage = $state('');

  // Estado da Aba Arquivos e Pastas
  let systemPaths = $state<SystemPaths | null>(null);
  let mediaFolders = $state<MediaFolder[]>([]);
  let newFolderName = $state('');
  let newFolderPath = $state('');
  let folderError = $state('');
  let isAddingFolder = $state(false);

  // Estado da Aba Bíblias
  let biblesList = $state<BibleVersionSummary[]>([]);
  let isLoadingBibles = $state(false);
  let bibleError = $state('');
  let isImportingBible = $state(false);
  let showPathImport = $state(false);
  let importPathInput = $state('');
  let bibleFileInput: HTMLInputElement | null = $state(null);

  // Edição de Versão da Bíblia
  let isEditingBible = $state(false);
  let editingBibleId = $state<string | null>(null);
  let editBibleName = $state('');
  let editBibleAbbr = $state('');
  let editBibleIsDefault = $state(false);

  // Estado do Auto-Updater
  let updateInfo = $state<UpdateCheckResponse | null>(null);
  let updaterStatus = $state<UpdaterStatusPayload | null>(null);
  let isCheckingUpdate = $state(false);
  let updaterError = $state('');
  let updatePollInterval: ReturnType<typeof setInterval> | null = null;

  async function handleCheckForUpdates() {
    isCheckingUpdate = true;
    updaterError = '';
    try {
      const res = await fetchUpdateCheck();
      updateInfo = res;
      await refreshUpdaterStatus();
    } catch (err: any) {
      updaterError = err.message || 'Não foi possível verificar atualizações.';
    } finally {
      isCheckingUpdate = false;
    }
  }

  async function refreshUpdaterStatus() {
    try {
      const st = await fetchUpdaterStatus();
      updaterStatus = st;
      if (st.last_check && !updateInfo) {
        updateInfo = st.last_check;
      }
      if (st.stage.type === 'Downloading' || st.stage.type === 'Extracting') {
        if (!updatePollInterval) {
          updatePollInterval = setInterval(refreshUpdaterStatus, 1000);
        }
      } else if (updatePollInterval) {
        clearInterval(updatePollInterval);
        updatePollInterval = null;
      }
    } catch {
      // silencioso
    }
  }

  async function handleStartUpdate() {
    if (!updateInfo?.download_url) return;
    updaterError = '';
    try {
      await triggerInstallUpdate({
        download_url: updateInfo.download_url,
        asset_name: updateInfo.asset_name || undefined,
        target_version: updateInfo.latest_version,
      });
      await refreshUpdaterStatus();
      if (!updatePollInterval) {
        updatePollInterval = setInterval(refreshUpdaterStatus, 1000);
      }
    } catch (err: any) {
      updaterError = err.message || 'Falha ao iniciar atualização.';
    }
  }

  // Dropdown open states for Shadcn style selects
  let isBaseColorMenuOpen = $state(false);
  let isAccentThemeMenuOpen = $state(false);

  // Carrega configurações estritamente ao abrir o modal, sem criar dependências reativas nas stores
  $effect(() => {
    if (open && !wasModalOpen) {
      wasModalOpen = true;
      activeTab = initialTab;
      editableSections = JSON.parse(JSON.stringify(get(sectionsStore)));
      localPublic = JSON.parse(JSON.stringify(get(publicSettingsStore)));
      localBrand = JSON.parse(JSON.stringify(get(churchBrandStore)));
      localStage = JSON.parse(JSON.stringify(get(stageSettingsStore)));
      localBgStyles = JSON.parse(JSON.stringify(get(backgroundStylesStore)));
      localLowerThird = JSON.parse(JSON.stringify(get(lowerThirdSettingsStore)));
      loadSystemPaths();
      loadMediaFolders();
      loadBibles();
      refreshUpdaterStatus();
    } else if (!open) {
      if (updatePollInterval) {
        clearInterval(updatePollInterval);
        updatePollInterval = null;
      }
      if (wasModalOpen) {
        refreshBibles();
        onbibleschanged?.();
      }
      wasModalOpen = false;
      saveStatusMessage = '';
    }
  });

  async function handleSaveAllSettings() {
    isSaving = true;
    try {
      await savePublicDisplaySettings(localPublic);
      await saveChurchBrandSettings(localBrand);
      await saveStageDisplaySettings(localStage);
      await saveBackgroundStyles(localBgStyles);
      await saveLowerThirdSettings(localLowerThird);
      saveSections(editableSections);
      saveStatusMessage = 'Configurações salvas com sucesso!';
      setTimeout(() => {
        saveStatusMessage = '';
      }, 3500);
    } catch (err) {
      saveStatusMessage = 'Erro ao salvar configurações';
    } finally {
      isSaving = false;
    }
  }

  async function handleLogoUpload(
    variant: 'primary' | 'dark' | 'light' | 'full',
    e: Event
  ) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];
    uploadStatus[variant] = 'Enviando...';

    const reader = new FileReader();
    reader.onload = async () => {
      try {
        const result = reader.result as string;
        const commaIdx = result.indexOf(',');
        const base64 = commaIdx >= 0 ? result.slice(commaIdx + 1) : result;

        const res = await fetch('/api/church/upload-logo', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content_base64: base64,
          }),
        });

        if (!res.ok) {
          throw new Error(await res.text());
        }

        const data = await res.json();
        const key = `logo_${variant}` as keyof ChurchBrandSettings;
        (localBrand as any)[key] = data.url;
        await saveChurchBrandSettings(localBrand);
        uploadStatus[variant] = 'Salvo!';
        setTimeout(() => {
          uploadStatus[variant] = '';
        }, 3000);
      } catch (err: any) {
        uploadStatus[variant] = `Erro: ${err.message || 'Falha no upload'}`;
      }
    };
    reader.readAsDataURL(file);
  }

  function updatePublicField<K extends keyof PublicDisplaySettings>(key: K, val: PublicDisplaySettings[K]) {
    localPublic = { ...localPublic, [key]: val };
    savePublicDisplaySettings(localPublic);
  }

  function updateBrandField<K extends keyof ChurchBrandSettings>(key: K, val: ChurchBrandSettings[K]) {
    localBrand = { ...localBrand, [key]: val };
    saveChurchBrandSettings(localBrand);
  }

  function updateStageField<K extends keyof StageDisplaySettings>(key: K, val: StageDisplaySettings[K]) {
    localStage = { ...localStage, [key]: val };
    saveStageDisplaySettings(localStage);
  }

  function handleResetStageColors() {
    localStage = {
      ...localStage,
      current_text_color: DEFAULT_STAGE_SETTINGS.current_text_color,
      next_text_color: DEFAULT_STAGE_SETTINGS.next_text_color,
      current_border_color: DEFAULT_STAGE_SETTINGS.current_border_color,
      next_border_color: DEFAULT_STAGE_SETTINGS.next_border_color,
      scripture_text_color: DEFAULT_STAGE_SETTINGS.scripture_text_color,
      scripture_border_color: DEFAULT_STAGE_SETTINGS.scripture_border_color,
      timer_color: DEFAULT_STAGE_SETTINGS.timer_color,
      clock_color: DEFAULT_STAGE_SETTINGS.clock_color,
    };
    saveStageDisplaySettings(localStage);
  }

  async function loadSystemPaths() {
    try {
      const res = await fetch('/api/system/paths');
      if (res.ok) {
        systemPaths = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar caminhos do sistema:', err);
    }
  }

  async function loadMediaFolders() {
    try {
      const res = await fetch('/api/media/folders');
      if (res.ok) {
        mediaFolders = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar pastas mapeadas:', err);
    }
  }

  async function handleAddFolder() {
    const p = newFolderPath.trim();
    if (!p) {
      folderError = 'Informe o caminho absoluto da pasta no seu computador.';
      return;
    }

    isAddingFolder = true;
    folderError = '';

    try {
      const res = await fetch('/api/media/folders', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: newFolderName.trim() || undefined,
          path: p,
          kind: 'images',
        }),
      });

      if (!res.ok) {
        const errorText = await res.text();
        throw new Error(errorText || 'Falha ao mapear pasta');
      }

      newFolderName = '';
      newFolderPath = '';
      await loadMediaFolders();
    } catch (err: any) {
      folderError = err.message || 'Erro ao validar e adicionar pasta';
    } finally {
      isAddingFolder = false;
    }
  }

  async function handleDeleteFolder(id: string) {
    try {
      const res = await fetch(`/api/media/folders/${id}`, { method: 'DELETE' });
      if (res.ok) {
        await loadMediaFolders();
      }
    } catch (err) {
      console.error('Erro ao desvincular pasta:', err);
    }
  }

  let isCopiedDataDir = $state(false);
  let folderPickerInput: HTMLInputElement | null = $state(null);

  async function handleCopyText(text: string) {
    try {
      await navigator.clipboard.writeText(text);
      isCopiedDataDir = true;
      setTimeout(() => (isCopiedDataDir = false), 2000);
    } catch (err) {
      console.error('Erro ao copiar texto:', err);
    }
  }

  async function handleOpenInFinder(path: string) {
    try {
      await fetch('/api/system/open-path', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ path }),
      });
    } catch (err) {
      console.error('Erro ao abrir pasta:', err);
    }
  }

  async function handleNativePickFolder() {
    folderError = '';
    try {
      const res = await fetch('/api/system/pick-folder', { method: 'POST' });
      if (res.ok) {
        const data = await res.json();
        if (data.path) {
          newFolderPath = data.path;
          if (!newFolderName && data.name) {
            newFolderName = data.name;
          }
          return;
        }
      }
    } catch (err) {
      console.error('Erro ao abrir seletor nativo:', err);
    }
    // Fallback para input HTML se o nativo não responder
    folderPickerInput?.click();
  }

  function handleHtmlFolderSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];
      const relative = file.webkitRelativePath || '';
      const folderName = relative.split('/')[0] || file.name;
      if (!newFolderName) newFolderName = folderName;
      if (!newFolderPath) newFolderPath = folderName;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.stopPropagation();
      onclose();
    }
  }

  function handleColorChange(secIdx: number, presetId: string) {
    const preset = COLOR_PRESETS.find((p) => p.id === presetId);
    if (!preset) return;

    editableSections[secIdx].colorName = preset.name;
    editableSections[secIdx].bg = preset.bg;
    editableSections[secIdx].text = preset.text;
    editableSections[secIdx].border = preset.border;
    editableSections[secIdx].accent = preset.accent;
    saveSections(editableSections);
  }

  function handleShortcutChange(secIdx: number, newKey: string) {
    editableSections[secIdx].shortcut = newKey.toLowerCase().slice(-1);
    saveSections(editableSections);
  }

  function handleNameChange(secIdx: number, newName: string) {
    editableSections[secIdx].name = newName;
    saveSections(editableSections);
  }

  function handleResetSections() {
    resetSectionsToDefault();
    editableSections = JSON.parse(JSON.stringify($sectionsStore));
  }

  async function loadBibles() {
    isLoadingBibles = true;
    bibleError = '';
    try {
      const list = await refreshBibles();
      biblesList = list;
      onbibleschanged?.();
    } catch (err: any) {
      bibleError = err.message || 'Erro de conexão ao buscar Bíblias';
    } finally {
      isLoadingBibles = false;
    }
  }

  async function handleBibleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];
    isImportingBible = true;
    bibleError = '';

    const reader = new FileReader();
    reader.onload = async () => {
      try {
        const result = reader.result as string;
        const commaIdx = result.indexOf(',');
        const base64 = commaIdx >= 0 ? result.slice(commaIdx + 1) : result;

        const res = await fetch('/api/bibles/import', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            filename: file.name,
            content_base64: base64,
          }),
        });

        if (!res.ok) {
          const msg = await res.text();
          if (msg.includes('length limit exceeded') || res.status === 413) {
            throw new Error('O arquivo excede o tamanho máximo suportado.');
          }
          throw new Error(msg || 'Falha ao importar Bíblia SQLite');
        }

        await loadBibles();
        if (bibleFileInput) bibleFileInput.value = '';
      } catch (err: any) {
        bibleError = err.message || 'Erro ao importar arquivo de Bíblia';
      } finally {
        isImportingBible = false;
      }
    };
    reader.onerror = () => {
      bibleError = 'Erro ao ler arquivo local.';
      isImportingBible = false;
    };
    reader.readAsDataURL(file);
  }

  async function handleImportBibleByPath() {
    const p = importPathInput.trim();
    if (!p) {
      bibleError = 'Informe o caminho absoluto do arquivo SQLite da Bíblia.';
      return;
    }
    isImportingBible = true;
    bibleError = '';
    try {
      const res = await fetch('/api/bibles/import', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          file_path: p,
        }),
      });
      if (!res.ok) {
        const msg = await res.text();
        throw new Error(msg || 'Falha ao importar Bíblia a partir do caminho');
      }
      importPathInput = '';
      showPathImport = false;
      await loadBibles();
    } catch (err: any) {
      bibleError = err.message || 'Erro ao importar arquivo do caminho informado';
    } finally {
      isImportingBible = false;
    }
  }

  function handleStartEditBible(b: BibleVersionSummary) {
    editingBibleId = b.id;
    editBibleName = b.name;
    editBibleAbbr = b.abbreviation;
    editBibleIsDefault = b.is_default;
    isEditingBible = true;
  }

  async function handleSaveEditBible() {
    if (!editingBibleId) return;
    try {
      const res = await fetch(`/api/bibles/${editingBibleId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: editBibleName.trim() || undefined,
          abbreviation: editBibleAbbr.trim() || undefined,
          is_default: editBibleIsDefault,
        }),
      });
      if (!res.ok) {
        throw new Error(await res.text());
      }
      isEditingBible = false;
      editingBibleId = null;
      await loadBibles();
    } catch (err: any) {
      bibleError = err.message || 'Erro ao salvar alterações da Bíblia';
    }
  }

  async function handleSetDefaultBible(id: string) {
    try {
      const res = await fetch(`/api/bibles/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ is_default: true }),
      });
      if (res.ok) {
        await loadBibles();
      }
    } catch (err) {
      console.error('Erro ao definir Bíblia padrão:', err);
    }
  }

  async function handleDeleteBible(id: string, name: string) {
    if (!confirm(`Tem certeza que deseja excluir a Bíblia "${name}"?`)) return;
    try {
      const res = await fetch(`/api/bibles/${id}`, { method: 'DELETE' });
      if (res.ok) {
        await loadBibles();
      } else {
        const msg = await res.text();
        bibleError = msg || 'Falha ao excluir Bíblia';
      }
    } catch (err: any) {
      bibleError = err.message || 'Erro ao excluir Bíblia';
    }
  }

  const currentBaseColorPreset = $derived(
    BASE_COLOR_PRESETS.find((p) => p.id === $baseColorStore) || BASE_COLOR_PRESETS[0]
  );

  const currentAccentPreset = $derived(
    ACCENT_THEME_PRESETS.find((p) => p.id === $accentThemeStore) || ACCENT_THEME_PRESETS[0]
  );
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-3 sm:p-6 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
    aria-label="Configurações do OpenPresenter"
    tabindex="-1"
    onclick={(e) => {
      if (e.target === e.currentTarget) onclose();
    }}
  >
    <!-- Modal Container -->
    <div
      class="w-full max-w-4xl xl:max-w-5xl h-[88vh] max-h-[760px] bg-card border border-border rounded-xl shadow-2xl flex flex-col overflow-hidden text-foreground"
    >
      <!-- Header do Modal -->
      <div class="h-14 px-6 border-b border-neutral-800 bg-neutral-900/60 flex items-center justify-between shrink-0">
        <div class="flex items-center gap-3">
          <img src="/branding/open-presenter-app-icon.svg" alt="OpenPresenter" class="w-7 h-7 rounded-md" />
          <div>
            <h2 class="text-sm font-semibold tracking-tight text-white">Configurações</h2>
            <p class="text-[11px] text-neutral-400">Preferências, temas, pastas do sistema e atalhos de projeção</p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="w-8 h-8 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800/80 flex items-center justify-center transition-colors cursor-pointer"
          aria-label="Fechar"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Corpo com Abas Verticais à Esquerda -->
      <div class="flex-1 flex overflow-hidden">
        <!-- Sidebar de Abas Verticais (Estilo macOS / ProPresenter) -->
        <aside class="w-52 sm:w-60 border-r border-neutral-800 bg-neutral-900/30 p-2.5 flex flex-col gap-1 shrink-0 overflow-y-auto">
          <button
            type="button"
            onclick={() => (activeTab = 'appearance')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'appearance' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Palette size={15} class={activeTab === 'appearance' ? 'text-primary' : 'text-neutral-500'} />
            <span>Aparência & Cores</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'public')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'public' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Monitor size={15} class={activeTab === 'public' ? 'text-primary' : 'text-neutral-500'} />
            <span>Tela Pública (Public)</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'lyrics_bg')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'lyrics_bg' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Sparkles size={15} class={activeTab === 'lyrics_bg' ? 'text-primary' : 'text-neutral-500'} />
            <span>Fundos de Letras & CSS</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'stage')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'stage' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Tv size={15} class={activeTab === 'stage' ? 'text-primary' : 'text-neutral-500'} />
            <span>Monitor de Palco (Stage)</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'lower_third')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'lower_third' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Video size={15} class={activeTab === 'lower_third' ? 'text-primary' : 'text-neutral-500'} />
            <span>Lower Third (OBS)</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'church')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'church' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Church size={15} class={activeTab === 'church' ? 'text-primary' : 'text-neutral-500'} />
            <span>Marca da Igreja</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'bibles')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'bibles' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <BookOpen size={15} class={activeTab === 'bibles' ? 'text-primary' : 'text-neutral-500'} />
            <span>Bíblias Sagradas</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'files')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'files' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <HardDrive size={15} class={activeTab === 'files' ? 'text-primary' : 'text-neutral-500'} />
            <span>Arquivos e Pastas</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'sections')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'sections' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <span class="w-3.5 h-3.5 rounded-full bg-primary/20 border border-primary/40 flex items-center justify-center text-[9px] font-bold text-primary">#</span>
            <span>Partes e Cores</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'outputs')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'outputs' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Sliders size={15} class={activeTab === 'outputs' ? 'text-primary' : 'text-neutral-500'} />
            <span>Telas e Saídas</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'shortcuts')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'shortcuts' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Keyboard size={15} class={activeTab === 'shortcuts' ? 'text-primary' : 'text-neutral-500'} />
            <span>Atalhos Rápidos</span>
          </button>

          <div class="my-2 border-t border-neutral-800/80"></div>

          <button
            type="button"
            onclick={() => (activeTab = 'about')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center justify-between transition-colors cursor-pointer {activeTab === 'about' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <div class="flex items-center gap-2.5">
              <Info size={15} class={activeTab === 'about' ? 'text-primary' : 'text-neutral-500'} />
              <span>Sobre & Updates</span>
            </div>
            {#if updateInfo?.update_available}
              <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            {/if}
          </button>
        </aside>

        <!-- Conteúdo da Aba Selecionada -->
        <main class="flex-1 p-6 overflow-y-auto">
          <!-- ABA 1: APARÊNCIA & CORES (Shadcn UI Base Color e Theme) -->
          {#if activeTab === 'appearance'}
            <div class="space-y-6 max-w-xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Modo de Exibição</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Alterne entre modo escuro, claro ou automático conforme o sistema operacional.
                </p>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
                <button
                  type="button"
                  onclick={() => setTheme('system')}
                  class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {$themeStore === 'system' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900'}"
                >
                  <div class="flex items-center justify-between mb-3">
                    <Laptop size={20} class={$themeStore === 'system' ? 'text-primary' : 'text-neutral-400'} />
                    {#if $themeStore === 'system'}
                      <Check size={16} class="text-primary" />
                    {/if}
                  </div>
                  <div>
                    <div class="text-xs font-semibold text-white">Sistema</div>
                    <div class="text-[10px] text-neutral-400 mt-0.5">Automático (OS)</div>
                  </div>
                </button>

                <button
                  type="button"
                  onclick={() => setTheme('dark')}
                  class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {$themeStore === 'dark' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900'}"
                >
                  <div class="flex items-center justify-between mb-3">
                    <Moon size={20} class={$themeStore === 'dark' ? 'text-primary' : 'text-neutral-400'} />
                    {#if $themeStore === 'dark'}
                      <Check size={16} class="text-primary" />
                    {/if}
                  </div>
                  <div>
                    <div class="text-xs font-semibold text-white">Escuro</div>
                    <div class="text-[10px] text-neutral-400 mt-0.5">Otimizado p/ cabines</div>
                  </div>
                </button>

                <button
                  type="button"
                  onclick={() => setTheme('light')}
                  class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {$themeStore === 'light' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900'}"
                >
                  <div class="flex items-center justify-between mb-3">
                    <Sun size={20} class={$themeStore === 'light' ? 'text-primary' : 'text-neutral-400'} />
                    {#if $themeStore === 'light'}
                      <Check size={16} class="text-primary" />
                    {/if}
                  </div>
                  <div>
                    <div class="text-xs font-semibold text-white">Claro</div>
                    <div class="text-[10px] text-neutral-400 mt-0.5">Ambientes iluminados</div>
                  </div>
                </button>
              </div>

              <!-- CONTROLES ESTILO SHADCN UI: Base Color e Theme Accent -->
              <div class="pt-4 border-t border-neutral-800 space-y-4">
                <div>
                  <h3 class="text-sm font-semibold text-white">Cores e Temas (Shadcn UI)</h3>
                  <p class="text-xs text-neutral-400 mt-0.5">
                    Personalize a Base Color (tons de superfície) e o Theme (cor de destaque primária).
                  </p>
                </div>

                <!-- 1. BASE COLOR SELECTOR (Conforme Screenshot 2) -->
                <div class="relative">
                  <span class="block text-[11px] font-medium text-neutral-400 mb-1.5">Base Color</span>
                  <button
                    type="button"
                    onclick={() => {
                      isBaseColorMenuOpen = !isBaseColorMenuOpen;
                      isAccentThemeMenuOpen = false;
                    }}
                    class="w-full h-11 px-3 rounded-lg bg-neutral-900 border border-neutral-800 flex items-center justify-between transition-colors hover:border-neutral-700 cursor-pointer"
                  >
                    <div class="flex items-center gap-2.5">
                      <span
                        class="w-4 h-4 rounded-full border border-white/20 shrink-0 shadow-inner"
                        style="background-color: {currentBaseColorPreset.previewColor};"
                      ></span>
                      <span class="text-xs font-semibold text-white">{currentBaseColorPreset.label}</span>
                      <span class="text-[11px] text-neutral-400 hidden sm:inline">- {currentBaseColorPreset.description}</span>
                    </div>
                    <ChevronDown size={14} class="text-neutral-400" />
                  </button>

                  {#if isBaseColorMenuOpen}
                    <div
                      class="absolute left-0 right-0 top-full mt-1.5 rounded-xl border border-neutral-800 bg-neutral-900/95 backdrop-blur-md p-1.5 shadow-2xl z-40 max-h-56 overflow-y-auto space-y-1 animate-in fade-in zoom-in-95 duration-100"
                    >
                      {#each BASE_COLOR_PRESETS as preset}
                        <button
                          type="button"
                          onclick={() => {
                            setBaseColor(preset.id);
                            isBaseColorMenuOpen = false;
                          }}
                          class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {$baseColorStore === preset.id ? 'bg-neutral-800 text-white font-semibold' : 'text-neutral-300 hover:bg-neutral-850 hover:text-white'}"
                        >
                          <div class="flex items-center gap-2.5">
                            <span
                              class="w-3.5 h-3.5 rounded-full border border-white/20 shrink-0"
                              style="background-color: {preset.previewColor};"
                            ></span>
                            <span class="text-xs">{preset.label}</span>
                          </div>
                          {#if $baseColorStore === preset.id}
                            <Check size={14} class="text-primary" />
                          {/if}
                        </button>
                      {/each}
                    </div>
                  {/if}
                </div>

                <!-- 2. THEME ACCENT COLOR (Conforme Screenshot 1) -->
                <div class="relative">
                  <span class="block text-[11px] font-medium text-neutral-400 mb-1.5">Theme (Cor de Destaque)</span>
                  <button
                    type="button"
                    onclick={() => {
                      isAccentThemeMenuOpen = !isAccentThemeMenuOpen;
                      isBaseColorMenuOpen = false;
                    }}
                    class="w-full h-11 px-3 rounded-lg bg-neutral-900 border border-neutral-800 flex items-center justify-between transition-colors hover:border-neutral-700 cursor-pointer"
                  >
                    <div class="flex items-center gap-2.5">
                      <span
                        class="w-4 h-4 rounded-full border border-white/20 shrink-0 shadow-inner"
                        style="background-color: {currentAccentPreset.previewColor};"
                      ></span>
                      <span class="text-xs font-semibold text-white">{currentAccentPreset.label}</span>
                    </div>
                    <ChevronDown size={14} class="text-neutral-400" />
                  </button>

                  {#if isAccentThemeMenuOpen}
                    <div
                      class="absolute left-0 right-0 top-full mt-1.5 rounded-xl border border-neutral-800 bg-neutral-900/95 backdrop-blur-md p-1.5 shadow-2xl z-40 max-h-56 overflow-y-auto space-y-1 animate-in fade-in zoom-in-95 duration-100"
                    >
                      {#each ACCENT_THEME_PRESETS as preset}
                        <button
                          type="button"
                          onclick={() => {
                            setAccentTheme(preset.id);
                            isAccentThemeMenuOpen = false;
                          }}
                          class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {$accentThemeStore === preset.id ? 'bg-neutral-800 text-white font-semibold' : 'text-neutral-300 hover:bg-neutral-850 hover:text-white'}"
                        >
                          <div class="flex items-center gap-2.5">
                            <span
                              class="w-3.5 h-3.5 rounded-full border border-white/20 shrink-0"
                              style="background-color: {preset.previewColor};"
                            ></span>
                            <span class="text-xs">{preset.label}</span>
                          </div>
                          {#if $accentThemeStore === preset.id}
                            <Check size={14} class="text-primary" />
                          {/if}
                        </button>
                      {/each}
                    </div>
                  {/if}
                </div>
              </div>

              <!-- CONTROLE DO PAINEL DE CONTROLE DE SAÍDAS / PREVIEWS -->
              <div class="pt-4 border-t border-neutral-800 space-y-4">
                <div>
                  <h3 class="text-sm font-semibold text-white">Botões da Interface do Operador</h3>
                  <p class="text-xs text-neutral-400 mt-0.5">
                    Personalize a exibição dos botões na barra de navegação de slides, controles de saídas e cabeçalho.
                  </p>
                </div>

                <div class="flex items-center justify-between p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40">
                  <div class="space-y-0.5 pr-4">
                    <div class="text-xs font-semibold text-white">Exibir textos (labels) nos botões</div>
                    <div class="text-[11px] text-neutral-400">
                      Quando desativado, exibe apenas os ícones compactos para ganhar espaço de tela e manter o visual minimalista.
                    </div>
                  </div>
                  <button
                    type="button"
                    role="switch"
                    aria-label="Alternar exibição de textos nos botões"
                    aria-checked={$previewLabelsStore}
                    onclick={() => setPreviewLabels(!$previewLabelsStore)}
                    class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {$previewLabelsStore ? 'bg-emerald-600' : 'bg-neutral-800'}"
                  >
                    <span
                      aria-hidden="true"
                      class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {$previewLabelsStore ? 'translate-x-5' : 'translate-x-0'}"
                    ></span>
                  </button>
                </div>
              </div>
            </div>

          <!-- ABA: TELA PÚBLICA (PUBLIC) -->
          {:else if activeTab === 'public'}
            <div class="space-y-6 max-w-2xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Configuração da Projeção Pública</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Personalize de forma independente o comportamento de letras de louvor, textos bíblicos e margens seguras.
                </p>
              </div>

              <!-- Seletor de Seções da Projeção Pública -->
              <div class="p-1 bg-neutral-950 rounded-xl border border-neutral-800 grid grid-cols-3 gap-1">
                <button
                  type="button"
                  onclick={() => (publicSection = 'lyrics')}
                  class="flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer {publicSection === 'lyrics' ? 'bg-primary text-primary-foreground shadow' : 'text-neutral-400 hover:text-white'}"
                >
                  <Music size={13} />
                  <span>Letras de Louvor</span>
                </button>

                <button
                  type="button"
                  onclick={() => (publicSection = 'scripture')}
                  class="flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer {publicSection === 'scripture' ? 'bg-primary text-primary-foreground shadow' : 'text-neutral-400 hover:text-white'}"
                >
                  <BookOpen size={13} />
                  <span>Textos Bíblicos</span>
                </button>

                <button
                  type="button"
                  onclick={() => (publicSection = 'general')}
                  class="flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer {publicSection === 'general' ? 'bg-primary text-primary-foreground shadow' : 'text-neutral-400 hover:text-white'}"
                >
                  <Sliders size={13} />
                  <span>Margens & Geral</span>
                </button>
              </div>

              {#if publicSection === 'lyrics'}
                <!-- SEÇÃO 1: LETRAS DE LOUVOR -->
                <div class="space-y-4">
                  <!-- Alinhamento das Letras -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <div class="text-xs font-semibold text-white flex items-center gap-2">
                      <Sliders size={14} class="text-primary" />
                      Alinhamento Padrão das Letras
                    </div>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
                      <!-- Alinhamento Horizontal -->
                      <div class="space-y-1.5">
                        <span class="text-[11px] text-neutral-400">Horizontal</span>
                        <div class="grid grid-cols-4 gap-1">
                          {#each [
                            { id: 'left', label: 'Esq' },
                            { id: 'center', label: 'Centro' },
                            { id: 'right', label: 'Dir' },
                            { id: 'justify', label: 'Just' }
                          ] as opt}
                            <button
                              type="button"
                              onclick={() => updatePublicField('lyrics_align_h', opt.id as any)}
                              class="py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors {(localPublic.lyrics_align_h || 'center') === opt.id ? 'bg-primary text-primary-foreground font-bold border-primary' : 'bg-neutral-950 border-neutral-800 text-neutral-400 hover:text-white'}"
                            >
                              {opt.label}
                            </button>
                          {/each}
                        </div>
                      </div>

                      <!-- Alinhamento Vertical -->
                      <div class="space-y-1.5">
                        <span class="text-[11px] text-neutral-400">Vertical</span>
                        <div class="grid grid-cols-3 gap-1">
                          {#each [
                            { id: 'top', label: 'Topo' },
                            { id: 'center', label: 'Centro' },
                            { id: 'bottom', label: 'Base' }
                          ] as opt}
                            <button
                              type="button"
                              onclick={() => updatePublicField('lyrics_align_v', opt.id as any)}
                              class="py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors {(localPublic.lyrics_align_v || 'center') === opt.id ? 'bg-primary text-primary-foreground font-bold border-primary' : 'bg-neutral-950 border-neutral-800 text-neutral-400 hover:text-white'}"
                            >
                              {opt.label}
                            </button>
                          {/each}
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- Auto-scale e Tamanhos de Fonte das Letras -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Type size={14} class="text-primary" />
                        Auto-scale das Letras na Projeção
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        O tamanho da fonte se adapta automaticamente ao volume do texto de cada estrofe.
                      </p>
                    </div>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2">
                        <div class="flex justify-between items-center text-xs">
                          <span class="text-neutral-400">Tamanho Máximo (Alvo)</span>
                          <span class="font-mono text-primary font-bold">{(localPublic.lyrics_max_font_size || localPublic.max_font_size) ?? 128}px</span>
                        </div>
                        <input
                          type="range"
                          min="64"
                          max="192"
                          step="8"
                          value={(localPublic.lyrics_max_font_size || localPublic.max_font_size) ?? 128}
                          oninput={(e) => {
                            const val = Number((e.target as HTMLInputElement).value);
                            updatePublicField('lyrics_max_font_size', val);
                            updatePublicField('max_font_size', val);
                          }}
                          class="w-full accent-primary"
                        />
                      </div>

                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2">
                        <div class="flex justify-between items-center text-xs">
                          <span class="text-neutral-400">Tamanho Mínimo</span>
                          <span class="font-mono text-white font-bold">{(localPublic.lyrics_min_font_size || localPublic.min_font_size) ?? 24}px</span>
                        </div>
                        <input
                          type="range"
                          min="16"
                          max="64"
                          step="4"
                          value={(localPublic.lyrics_min_font_size || localPublic.min_font_size) ?? 24}
                          oninput={(e) => {
                            const val = Number((e.target as HTMLInputElement).value);
                            updatePublicField('lyrics_min_font_size', val);
                            updatePublicField('min_font_size', val);
                          }}
                          class="w-full accent-primary"
                        />
                      </div>
                    </div>
                  </div>

                  <!-- Preenchimento Máximo -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Maximize2 size={14} class="text-primary" />
                        Preencher o Máximo de Espaço nas Letras
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Amplia as letras para preencher a tela, garantindo alta legibilidade no fundo do templo.
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar preenchimento máximo de espaço"
                      onclick={() => updatePublicField('fill_space', !localPublic.fill_space)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.fill_space ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={localPublic.fill_space}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.fill_space ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>

                  <!-- Quebra de Linha -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Type size={14} class="text-primary" />
                        Permitir Quebra de Linhas Longas (Word Wrap)
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Se desativado, o texto preserva os versos originais em linha única e o auto-scale reduz a fonte para caber sem reticências (...).
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar quebra de linhas longas"
                      onclick={() => updatePublicField('word_wrap', !localPublic.word_wrap)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.word_wrap ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={localPublic.word_wrap}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.word_wrap ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>

                  <!-- Exibir Metadados -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Info size={14} class="text-primary" />
                        Exibir Metadados do Louvor
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Exibe o título da música e artista/compositor em uma legenda discreta no rodapé da projeção pública.
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar exibição de metadados"
                      onclick={() => updatePublicField('show_metadata', !localPublic.show_metadata)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.show_metadata ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={localPublic.show_metadata}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.show_metadata ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>
                </div>

              {:else if publicSection === 'scripture'}
                <!-- SEÇÃO 2: TEXTOS BÍBLICOS (ESCRITURAS) -->
                <div class="space-y-4">
                  <!-- Alinhamento dos Textos Bíblicos -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <div class="text-xs font-semibold text-white flex items-center gap-2">
                      <BookOpen size={14} class="text-primary" />
                      Alinhamento das Passagens Bíblicas
                    </div>
                    <p class="text-[11px] text-neutral-400">
                      Configure o posicionamento do texto dos versículos na tela pública. O padrão recomendado é Centralizado tanto na horizontal quanto na vertical.
                    </p>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
                      <!-- Alinhamento Horizontal Bíblico -->
                      <div class="space-y-1.5">
                        <span class="text-[11px] text-neutral-400">Horizontal</span>
                        <div class="grid grid-cols-4 gap-1">
                          {#each [
                            { id: 'left', label: 'Esq' },
                            { id: 'center', label: 'Centro' },
                            { id: 'right', label: 'Dir' },
                            { id: 'justify', label: 'Just' }
                          ] as opt}
                            <button
                              type="button"
                              onclick={() => updatePublicField('scripture_align_h', opt.id as any)}
                              class="py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors {(localPublic.scripture_align_h || 'center') === opt.id ? 'bg-primary text-primary-foreground font-bold border-primary' : 'bg-neutral-950 border-neutral-800 text-neutral-400 hover:text-white'}"
                            >
                              {opt.label}
                            </button>
                          {/each}
                        </div>
                      </div>

                      <!-- Alinhamento Vertical Bíblico -->
                      <div class="space-y-1.5">
                        <span class="text-[11px] text-neutral-400">Vertical</span>
                        <div class="grid grid-cols-3 gap-1">
                          {#each [
                            { id: 'top', label: 'Topo' },
                            { id: 'center', label: 'Centro' },
                            { id: 'bottom', label: 'Base' }
                          ] as opt}
                            <button
                              type="button"
                              onclick={() => updatePublicField('scripture_align_v', opt.id as any)}
                              class="py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors {(localPublic.scripture_align_v || 'center') === opt.id ? 'bg-primary text-primary-foreground font-bold border-primary' : 'bg-neutral-950 border-neutral-800 text-neutral-400 hover:text-white'}"
                            >
                              {opt.label}
                            </button>
                          {/each}
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- Auto-scale e Tamanhos de Fonte Bíblicos -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Type size={14} class="text-primary" />
                        Auto-scale Especializado para Versículos
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Como passagens bíblicas contêm mais texto corrido que estrofes de louvor, você pode definir um teto máximo e mínimo próprio para a Bíblia.
                      </p>
                    </div>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2">
                        <div class="flex justify-between items-center text-xs">
                          <span class="text-neutral-400">Tamanho Máximo Bíblico</span>
                          <span class="font-mono text-primary font-bold">{localPublic.scripture_max_font_size ?? 140}px</span>
                        </div>
                        <input
                          type="range"
                          min="80"
                          max="200"
                          step="8"
                          value={localPublic.scripture_max_font_size ?? 140}
                          oninput={(e) => updatePublicField('scripture_max_font_size', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                        <div class="flex justify-between text-[9px] text-neutral-500 font-mono">
                          <span>80px</span>
                          <span>140px (Padrão)</span>
                          <span>200px</span>
                        </div>
                      </div>

                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2">
                        <div class="flex justify-between items-center text-xs">
                          <span class="text-neutral-400">Tamanho Mínimo Bíblico</span>
                          <span class="font-mono text-white font-bold">{localPublic.scripture_min_font_size ?? 28}px</span>
                        </div>
                        <input
                          type="range"
                          min="18"
                          max="64"
                          step="2"
                          value={localPublic.scripture_min_font_size ?? 28}
                          oninput={(e) => updatePublicField('scripture_min_font_size', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                        <div class="flex justify-between text-[9px] text-neutral-500 font-mono">
                          <span>18px</span>
                          <span>28px (Padrão)</span>
                          <span>64px</span>
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- Preenchimento Total da Tela para Versículos -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Maximize2 size={14} class="text-primary" />
                        Aproveitar 100% da Largura da Tela na Bíblia
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Permite que os versículos usem toda a largura útil da projeção, acomodando mais palavras por linha e permitindo que a fonte fique maior.
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar preenchimento bíblico"
                      onclick={() => updatePublicField('scripture_fill_space', localPublic.scripture_fill_space === false ? true : false)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.scripture_fill_space !== false ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={localPublic.scripture_fill_space !== false}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.scripture_fill_space !== false ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>

                  <!-- Exibir Referência e Tradução -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <div class="flex items-center justify-between gap-4">
                      <div>
                        <div class="text-xs font-semibold text-white flex items-center gap-2">
                          <BookOpen size={14} class="text-primary" />
                          Exibir Referência Bíblica e Versão (Ex: João 3:16 • ARA)
                        </div>
                        <p class="text-[11px] text-neutral-400 mt-0.5">
                          Mostra o livro, capítulo, versículo e a sigla da tradução utilizada na projeção pública.
                        </p>
                      </div>
                      <button
                        type="button"
                        aria-label="Alternar referência bíblica"
                        onclick={() => updatePublicField('scripture_show_reference', localPublic.scripture_show_reference === false ? true : false)}
                        class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.scripture_show_reference !== false ? 'bg-emerald-600' : 'bg-neutral-800'}"
                        role="switch"
                        aria-checked={localPublic.scripture_show_reference !== false}
                      >
                        <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.scripture_show_reference !== false ? 'translate-x-5' : 'translate-x-0'}"></span>
                      </button>
                    </div>

                    {#if localPublic.scripture_show_reference !== false}
                      <!-- Posição da Referência: No Rodapé vs Abaixo do Texto -->
                      <div class="pt-2 border-t border-neutral-800/80 space-y-2.5">
                        <span class="text-xs font-medium text-neutral-300">Posição da Referência na Tela</span>
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                          <button
                            type="button"
                            onclick={() => updatePublicField('scripture_reference_position', 'footer')}
                            class="p-2.5 rounded-lg border text-left transition-all cursor-pointer {(localPublic.scripture_reference_position ?? 'footer') === 'footer' ? 'border-primary bg-primary/10 text-white' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                          >
                            <div class="text-xs font-semibold">{(localPublic.scripture_reference_position ?? 'footer') === 'footer' ? '✓ ' : ''}No Rodapé (Inferior da Tela)</div>
                            <div class="text-[10px] opacity-70 mt-0.5">Fixado na parte inferior com linha divisória elegante</div>
                          </button>

                          <button
                            type="button"
                            onclick={() => updatePublicField('scripture_reference_position', 'below_text')}
                            class="p-2.5 rounded-lg border text-left transition-all cursor-pointer {localPublic.scripture_reference_position === 'below_text' ? 'border-primary bg-primary/10 text-white' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                          >
                            <div class="text-xs font-semibold">{localPublic.scripture_reference_position === 'below_text' ? '✓ ' : ''}Abaixo do Texto (Junto ao Versículo)</div>
                            <div class="text-[10px] opacity-70 mt-0.5">Acompanha o versículo centralizado verticalmente</div>
                          </button>
                        </div>
                      </div>

                      {#if (localPublic.scripture_reference_position ?? 'footer') === 'footer'}
                        <!-- Tamanho da Fonte da Referência no Rodapé -->
                        <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2 max-w-md">
                          <div class="flex justify-between items-center text-xs">
                            <span class="text-neutral-300 font-medium">Tamanho da Fonte da Referência no Rodapé</span>
                            <span class="font-mono text-primary font-bold bg-primary/10 px-2 py-0.5 rounded border border-primary/30">
                              {localPublic.scripture_reference_font_size ?? 28}px
                            </span>
                          </div>
                          <input
                            type="range"
                            min="16"
                            max="64"
                            step="2"
                            value={localPublic.scripture_reference_font_size ?? 28}
                            oninput={(e) => updatePublicField('scripture_reference_font_size', Number((e.target as HTMLInputElement).value))}
                            class="w-full h-1.5 bg-neutral-800 rounded-lg appearance-none cursor-pointer accent-primary"
                          />
                          <div class="flex justify-between text-[10px] text-neutral-500 font-mono">
                            <span>16px (Discreto)</span>
                            <span>28px (Padrão)</span>
                            <span>64px (Grande)</span>
                          </div>
                        </div>
                      {/if}
                    {/if}
                  </div>
                </div>

              {:else}
                <!-- SEÇÃO 3: MARGENS & GERAL -->
                <div class="space-y-4">
                  <!-- Escurecer Plano de Fundo Geral (Backdrop da Tela Toda) -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Sun size={14} class="text-primary" />
                        Escurecer Plano de Fundo Geral (Backdrop Completo)
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Aplica uma película escura translúcida sobre todo o papel de parede quando houver um slide ativo na tela para diminuir o brilho do fundo.
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar escurecimento do plano de fundo"
                      onclick={() => updatePublicField('darken_background', !localPublic.darken_background)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.darken_background ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={Boolean(localPublic.darken_background)}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.darken_background ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>

                  <!-- Fundo nas Caixas de Texto (Placa de Leitura / Card) -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Layers size={14} class="text-primary" />
                        Fundo nas Caixas de Texto (Card de Leitura)
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Adiciona uma placa translúcida com cantos arredondados diretamente atrás dos textos e versículos bíblicos.
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar fundo nas caixas de texto"
                      onclick={() => updatePublicField('text_box_background', !localPublic.text_box_background)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.text_box_background ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={Boolean(localPublic.text_box_background)}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.text_box_background ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>

                  <!-- Ignorar Margens para Imagens (Edge-to-Edge) -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <ImageIcon size={14} class="text-primary" />
                        Projetar Imagens em Tela Cheia (Sem Margens de Texto)
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Quando ativado, imagens 16:9 ocupam 100% da tela sem aplicar as margens seguras de texto, eliminando bordas pretas extras.
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar margens para imagens"
                      onclick={() => updatePublicField('ignore_margins_for_media', !localPublic.ignore_margins_for_media)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.ignore_margins_for_media !== false ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={localPublic.ignore_margins_for_media !== false}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.ignore_margins_for_media !== false ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>

                  <!-- Encaixe do Papel de Parede na Projeção (Wallpaper Fit) -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-2.5">
                    <div class="text-xs font-semibold text-white flex items-center gap-2">
                      <Maximize2 size={14} class="text-primary" />
                      Encaixe do Papel de Parede na Projeção (TV / Chromecast)
                    </div>
                    <p class="text-[11px] text-neutral-400">
                      Escolha como a imagem de fundo deve se ajustar à tela. Para TVs ou Chromecast com cortes de borda (overscan), use "Ajustar (Conter)".
                    </p>
                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-2 pt-1">
                      {#each [
                        { id: 'contain', label: 'Ajustar (Conter)', desc: '100% visível, sem cortes (TV)' },
                        { id: 'cover', label: 'Preencher (Cobrir)', desc: 'Preenche cortando bordas' },
                        { id: 'fill', label: 'Esticar', desc: 'Preenche 100% da tela' }
                      ] as wallFitOpt}
                        <button
                          type="button"
                          onclick={() => updatePublicField('wallpaper_fit', wallFitOpt.id as any)}
                          class="p-2 rounded-lg border text-left transition-all cursor-pointer {(localPublic.wallpaper_fit || 'contain') === wallFitOpt.id ? 'border-primary bg-primary/10 text-white' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                        >
                          <div class="text-xs font-medium">{(localPublic.wallpaper_fit || 'contain') === wallFitOpt.id ? '✓ ' : ''}{wallFitOpt.label}</div>
                          <div class="text-[10px] opacity-70 mt-0.5">{wallFitOpt.desc}</div>
                        </button>
                      {/each}
                    </div>
                  </div>

                  <!-- Margens Seguras no Papel de Parede -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        <Sliders size={14} class="text-primary" />
                        Margens Seguras no Papel de Parede (Anti-Overscan TV)
                      </div>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Aplica as margens seguras também ao plano de fundo, evitando que artes com bordas sejam cortadas por televisores ou Chromecast.
                      </p>
                    </div>
                    <button
                      type="button"
                      aria-label="Alternar margens seguras no papel de parede"
                      onclick={() => updatePublicField('wallpaper_safe_margins', !localPublic.wallpaper_safe_margins)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localPublic.wallpaper_safe_margins ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      role="switch"
                      aria-checked={Boolean(localPublic.wallpaper_safe_margins)}
                    >
                      <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localPublic.wallpaper_safe_margins ? 'translate-x-5' : 'translate-x-0'}"></span>
                    </button>
                  </div>
                  <!-- Encaixe Padrão de Imagens de Mídia -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-2.5">
                    <div class="text-xs font-semibold text-white flex items-center gap-2">
                      <ImageIcon size={14} class="text-primary" />
                      Encaixe Padrão de Imagens de Mídia (Slides/Fotos)
                    </div>
                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 pt-1">
                      {#each [
                        { id: 'contain', label: 'Ajustar (Conter)', desc: 'Sem cortar' },
                        { id: 'cover', label: 'Preencher', desc: 'Corta excedente' },
                        { id: 'fill', label: 'Esticar', desc: 'Preenche 100%' },
                        { id: 'center', label: 'Original', desc: 'Sem zoom' }
                      ] as fitOption}
                        <button
                          type="button"
                          onclick={() => updatePublicField('image_fit', fitOption.id as any)}
                          class="p-2 rounded-lg border text-left transition-all cursor-pointer {(localPublic.image_fit || 'contain') === fitOption.id ? 'border-primary bg-primary/10 text-white' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                        >
                          <div class="text-xs font-medium">{(localPublic.image_fit || 'contain') === fitOption.id ? '✓ ' : ''}{fitOption.label}</div>
                          <div class="text-[10px] opacity-70 mt-0.5">{fitOption.desc}</div>
                        </button>
                      {/each}
                    </div>
                  </div>

                  <!-- Margens da Projeção -->
                  <div class="pt-4 border-t border-neutral-800 space-y-4">
                    <div class="flex items-center justify-between">
                      <div>
                        <h4 class="text-xs font-semibold text-white">Margens da Projeção Pública (Safe Area)</h4>
                        <p class="text-[11px] text-neutral-400 mt-0.5">
                          Evite que as letras encostem nas bordas do telão ou sofram corte por overscan do projetor/TV.
                        </p>
                      </div>
                      <Button
                        variant="outline"
                        size="sm"
                        class="text-[11px] h-7"
                        onclick={() => {
                          localPublic = {
                            ...localPublic,
                            margin_top: DEFAULT_PUBLIC_SETTINGS.margin_top,
                            margin_bottom: DEFAULT_PUBLIC_SETTINGS.margin_bottom,
                            margin_horizontal: DEFAULT_PUBLIC_SETTINGS.margin_horizontal,
                          };
                          savePublicDisplaySettings(localPublic);
                        }}
                      >
                        Restaurar Margens
                      </Button>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/30 space-y-2">
                        <div class="flex justify-between text-xs">
                          <span class="text-neutral-400">Margem Topo</span>
                          <span class="font-mono text-white font-semibold">{localPublic.margin_top}px</span>
                        </div>
                        <input
                          type="range"
                          min="0"
                          max="160"
                          step="4"
                          value={localPublic.margin_top}
                          oninput={(e) => updatePublicField('margin_top', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                      </div>

                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/30 space-y-2">
                        <div class="flex justify-between text-xs">
                          <span class="text-neutral-400">Margem Base</span>
                          <span class="font-mono text-white font-semibold">{localPublic.margin_bottom}px</span>
                        </div>
                        <input
                          type="range"
                          min="0"
                          max="160"
                          step="4"
                          value={localPublic.margin_bottom}
                          oninput={(e) => updatePublicField('margin_bottom', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                      </div>

                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/30 space-y-2">
                        <div class="flex justify-between text-xs">
                          <span class="text-neutral-400">Laterais</span>
                          <span class="font-mono text-white font-semibold">{localPublic.margin_horizontal}px</span>
                        </div>
                        <input
                          type="range"
                          min="0"
                          max="240"
                          step="4"
                          value={localPublic.margin_horizontal}
                          oninput={(e) => updatePublicField('margin_horizontal', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                      </div>
                    </div>

                    <!-- Prévia gráfica das margens -->
                    <div class="p-4 rounded-xl border border-neutral-800 bg-black flex flex-col items-center justify-center">
                      <div class="text-[10px] text-neutral-500 mb-2 font-mono uppercase">Simulação de Área Útil (16:9)</div>
                      <div class="w-64 h-36 rounded border border-neutral-700 bg-neutral-950 flex items-center justify-center p-2 relative overflow-hidden">
                        <div
                          class="w-full h-full border border-dashed border-primary/60 bg-primary/5 rounded flex items-center justify-center text-center p-1"
                          style="margin-top: {localPublic.margin_top / 6}px; margin-bottom: {localPublic.margin_bottom / 6}px; margin-left: {localPublic.margin_horizontal / 6}px; margin-right: {localPublic.margin_horizontal / 6}px;"
                        >
                          <span class="text-[9px] text-primary/80 font-medium">Área de Letra e Mídia</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              {/if}
            </div>

          <!-- ABA: FUNDOS DE LETRAS & CSS -->
          {:else if activeTab === 'lyrics_bg'}
            <div class="space-y-6">
              <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                <div>
                  <h3 class="text-sm font-semibold text-white">Estilos, Cores de Texto e Fundos de Letras</h3>
                  <p class="text-xs text-neutral-400 mt-0.5">
                    Personalize o CSS dos planos de fundo, defina cores de texto para máxima legibilidade e crie novos fundos exclusivos.
                  </p>
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={createNewCustomPreset}
                  class="gap-1.5 shrink-0 text-xs border-primary/40 text-primary hover:bg-primary/10"
                >
                  <Plus size={14} />
                  <span>Novo Fundo Customizado</span>
                </Button>
              </div>

              <!-- Simulação 16:9 em Tempo Real -->
              <div class="rounded-2xl border border-neutral-800 bg-neutral-950/70 p-4 sm:p-5 flex flex-col items-center">
                <div class="w-full flex flex-col sm:flex-row items-center justify-between gap-2 mb-2 px-1">
                  <span class="text-xs font-semibold text-neutral-300 flex items-center gap-1.5">
                    <Sparkles size={13} class="text-primary" />
                    <span>Prévia em Tempo Real (16:9)</span>
                  </span>

                  <!-- Alternador entre Letra de Louvor e Texto Bíblico -->
                  <div class="flex items-center rounded-lg border border-neutral-800 bg-neutral-900 p-0.5 text-xs">
                    <button
                      type="button"
                      onclick={() => (bgPreviewMode = 'lyrics')}
                      class="px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 {bgPreviewMode === 'lyrics' ? 'bg-primary text-primary-foreground font-semibold shadow' : 'text-neutral-400 hover:text-white'}"
                    >
                      <Music size={12} />
                      <span>Letra de Louvor</span>
                    </button>
                    <button
                      type="button"
                      onclick={() => (bgPreviewMode = 'scripture')}
                      class="px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 {bgPreviewMode === 'scripture' ? 'bg-primary text-primary-foreground font-semibold shadow' : 'text-neutral-400 hover:text-white'}"
                    >
                      <BookOpen size={12} />
                      <span>Texto Bíblico</span>
                    </button>
                  </div>

                  <div class="flex items-center gap-2">
                    {#if localBgStyles.defaultBackgroundId === currentSelectedPreset.id}
                      <span class="text-[10px] bg-primary/20 text-primary border border-primary/30 px-2 py-0.5 rounded-full font-semibold">
                        Fundo Padrão
                      </span>
                    {/if}
                    <span class="text-[11px] font-mono text-neutral-500">{currentSelectedPreset.name}</span>
                  </div>
                </div>

                <div class="w-full max-w-xl aspect-video rounded-xl border border-neutral-800 relative overflow-hidden shadow-2xl flex flex-col justify-between p-6 sm:p-8 select-none">
                  <!-- Background animado ou estático -->
                  <div
                    class="absolute inset-0 w-full h-full {currentSelectedPreset.animationClass === 'none' ? '' : (currentSelectedPreset.animationClass || '')}"
                    style="background: {currentSelectedPreset.style};"
                  ></div>

                  {#if bgPreviewMode === 'lyrics'}
                    <!-- Letra de Louvor centralizada -->
                    <div class="relative z-10 my-auto text-center flex flex-col items-center justify-center">
                      <p
                        class="text-base sm:text-2xl font-bold leading-relaxed tracking-tight drop-shadow-[0_4px_18px_rgba(0,0,0,0.95)]"
                        style="color: {currentSelectedPreset.textColor || '#ffffff'};"
                      >
                        Grandioso és Tu, Senhor,<br />
                        ao contemplar os céus e a terra!
                      </p>
                      <span class="text-[10px] sm:text-xs mt-2 opacity-75 font-medium tracking-wide" style="color: {currentSelectedPreset.textColor || '#ffffff'};">
                        Harpa Cristã • 526
                      </span>
                    </div>
                  {:else}
                    <!-- Versículo Bíblico centralizado com rodapé ou abaixo do texto -->
                    <div class="relative z-10 my-auto text-center flex flex-col items-center justify-center">
                      <p
                        class="text-sm sm:text-lg font-bold leading-relaxed tracking-tight drop-shadow-[0_4px_18px_rgba(0,0,0,0.95)]"
                        style="color: {currentSelectedPreset.textColor || '#ffffff'};"
                      >
                        Disse também Deus: Haja luzeiros no firmamento dos céus, para fazerem separação entre o dia e a noite; e sejam eles para sinais, para estações, para dias e anos.
                      </p>

                      {#if localPublic.scripture_show_reference !== false && localPublic.scripture_reference_position === 'below_text'}
                        <div class="mt-3 flex items-center justify-center gap-2 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
                          <span class="text-xs sm:text-sm font-bold tracking-tight opacity-90" style="color: {currentSelectedPreset.textColor || '#ffffff'};">
                            Gênesis 1:14
                          </span>
                          <span class="text-[8px] px-1.5 py-0.5 rounded bg-white/15 border border-white/20 uppercase font-mono font-bold text-neutral-100">
                            ARA
                          </span>
                        </div>
                      {/if}
                    </div>

                    {#if localPublic.scripture_show_reference !== false && (localPublic.scripture_reference_position ?? 'footer') === 'footer'}
                      <!-- Rodapé com referência bíblica -->
                      <div class="relative z-10 flex items-center justify-between border-t border-white/10 pt-2 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
                        <span
                          class="font-bold tracking-tight"
                          style="color: {currentSelectedPreset.textColor || '#ffffff'}; font-size: {Math.max(12, Math.round((localPublic.scripture_reference_font_size ?? 28) * 0.45))}px;"
                        >
                          Gênesis 1:14
                        </span>
                        <span class="text-[9px] px-2 py-0.5 rounded bg-white/15 border border-white/20 uppercase font-mono font-bold text-neutral-100 shadow">
                          ARA
                        </span>
                      </div>
                    {/if}
                  {/if}
                </div>
              </div>

              <!-- Editor do Fundo Selecionado -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <!-- Coluna 1: Cor do Texto & Ações de Preset -->
                <div class="p-4 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-4">
                  <div class="flex items-center justify-between">
                    <h4 class="text-xs font-semibold text-neutral-200 uppercase tracking-wider flex items-center gap-1.5">
                      <Type size={14} class="text-primary" />
                      <span>Cor do Texto de Projeção</span>
                    </h4>
                    {#if localBgStyles.overrides[currentSelectedPreset.id] && !currentSelectedPreset.isCustom}
                      <button
                        type="button"
                        onclick={() => restorePresetDefault(currentSelectedPreset.id)}
                        class="text-[11px] text-amber-400 hover:text-amber-300 flex items-center gap-1 cursor-pointer"
                        title="Restaurar valores padrão deste preset"
                      >
                        <RotateCcw size={12} />
                        <span>Restaurar Padrão</span>
                      </button>
                    {/if}
                  </div>

                  <div>
                    <span class="text-[11px] text-neutral-400 mb-1.5 block">Seletor de Cor Hexadecimal</span>
                    <div class="flex items-center gap-2">
                      <input
                        type="color"
                        value={currentSelectedPreset.textColor || '#ffffff'}
                        oninput={(e) => updateCurrentTextColor((e.target as HTMLInputElement).value)}
                        class="w-9 h-9 rounded-lg border border-neutral-700 bg-neutral-900 p-0.5 cursor-pointer shrink-0"
                      />
                      <input
                        type="text"
                        value={currentSelectedPreset.textColor || '#ffffff'}
                        oninput={(e) => updateCurrentTextColor((e.target as HTMLInputElement).value)}
                        placeholder="#ffffff"
                        class="flex-1 h-9 px-3 rounded-lg border border-neutral-800 bg-neutral-900 font-mono text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary uppercase"
                      />
                    </div>
                  </div>

                  <div>
                    <span class="text-[11px] text-neutral-400 mb-1.5 block">Atalhos de Contraste Recomendados</span>
                    <div class="grid grid-cols-4 gap-1.5">
                      {#each QUICK_TEXT_COLORS as color}
                        <button
                          type="button"
                          onclick={() => updateCurrentTextColor(color.hex)}
                          class="flex items-center gap-1.5 px-2 py-1.5 rounded-lg border text-[11px] font-medium transition-all cursor-pointer {(currentSelectedPreset.textColor || '#ffffff').toLowerCase() === color.hex.toLowerCase() ? 'border-primary bg-primary/10 text-white' : 'border-neutral-800 bg-neutral-900/60 text-neutral-300 hover:border-neutral-700'}"
                        >
                          <span class="w-3 h-3 rounded-full border border-black/40 shrink-0" style="background-color: {color.hex};"></span>
                          <span class="truncate">{color.label}</span>
                        </button>
                      {/each}
                    </div>
                  </div>

                  <!-- Botões de Ação do Preset -->
                  <div class="pt-2 border-t border-neutral-800/80 flex flex-wrap items-center justify-between gap-2">
                    <button
                      type="button"
                      onclick={() => setDefaultBackground(currentSelectedPreset.id)}
                      class="px-3 py-1.5 rounded-lg text-xs font-medium border flex items-center gap-1.5 transition-colors cursor-pointer {localBgStyles.defaultBackgroundId === currentSelectedPreset.id ? 'bg-primary text-primary-foreground border-primary' : 'border-neutral-700 bg-neutral-800 text-neutral-300 hover:text-white'}"
                    >
                      <Check size={13} />
                      <span>{localBgStyles.defaultBackgroundId === currentSelectedPreset.id ? 'Fundo Padrão Ativo' : 'Definir como Padrão de Letras'}</span>
                    </button>

                    {#if currentSelectedPreset.isCustom}
                      <button
                        type="button"
                        onclick={() => deleteCustomPreset(currentSelectedPreset.id)}
                        class="px-2.5 py-1.5 rounded-lg text-xs text-red-400 hover:text-red-300 hover:bg-red-500/10 border border-red-500/20 flex items-center gap-1 transition-colors cursor-pointer"
                      >
                        <Trash2 size={13} />
                        <span>Excluir</span>
                      </button>
                    {/if}
                  </div>
                </div>

                <!-- Coluna 2: Nome, CSS do Fundo & Animação -->
                <div class="p-4 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-4">
                  <h4 class="text-xs font-semibold text-neutral-200 uppercase tracking-wider flex items-center gap-1.5">
                    <Sliders size={14} class="text-primary" />
                    <span>Propriedades do Fundo CSS</span>
                  </h4>

                  {#if currentSelectedPreset.isCustom}
                    <div>
                      <label for="bg-custom-name" class="text-[11px] text-neutral-400 mb-1.5 block">Nome do Fundo</label>
                      <input
                        id="bg-custom-name"
                        type="text"
                        value={currentSelectedPreset.name}
                        oninput={(e) => updateCurrentName((e.target as HTMLInputElement).value)}
                        placeholder="Nome do preset..."
                        class="w-full h-8 px-3 rounded-lg border border-neutral-800 bg-neutral-900 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
                      />
                    </div>
                  {/if}

                  <div>
                    <label for="bg-anim-select" class="text-[11px] text-neutral-400 mb-1.5 block">Efeito de Animação CSS</label>
                    <select
                      id="bg-anim-select"
                      value={currentSelectedPreset.animationClass || 'none'}
                      onchange={(e) => updateCurrentAnimation((e.target as HTMLSelectElement).value)}
                      class="w-full h-8 px-2.5 rounded-lg border border-neutral-800 bg-neutral-900 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
                    >
                      <option value="none">Nenhuma animação (Estático)</option>
                      <option value="animate-bg-pulse">Pulsar Brilho Sutil (animate-bg-pulse)</option>
                      <option value="animate-bg-flow">Fluxo Gradiente Dinâmico (animate-bg-flow)</option>
                      <option value="animate-bg-aurora">Aurora Boreal Cósmica (animate-bg-aurora)</option>
                      <option value="animate-bg-nebula">Nebulosa Cósmica (animate-bg-nebula)</option>
                    </select>
                  </div>

                  <div>
                    <div class="flex items-center justify-between mb-1.5">
                      <label for="bg-css-style" class="text-[11px] text-neutral-400">CSS da Propriedade `background`</label>
                      <span class="text-[10px] text-neutral-500 font-mono">linear-gradient / radial-gradient / solid</span>
                    </div>
                    <textarea
                      id="bg-css-style"
                      value={currentSelectedPreset.style}
                      oninput={(e) => updateCurrentStyle((e.target as HTMLTextAreaElement).value)}
                      rows="3"
                      class="w-full p-2.5 rounded-lg border border-neutral-800 bg-neutral-900 font-mono text-[11px] text-foreground focus:outline-none focus:ring-1 focus:ring-primary resize-y"
                    ></textarea>
                  </div>

                  <!-- Templates rápidos de gradiente -->
                  <div>
                    <span class="text-[10px] text-neutral-400 mb-1 block">Modelos de Gradiente Rápidos</span>
                    <div class="flex flex-wrap gap-1">
                      {#each QUICK_GRADIENTS as tpl}
                        <button
                          type="button"
                          onclick={() => updateCurrentStyle(tpl.css)}
                          class="text-[10px] px-2 py-0.5 rounded border border-neutral-800 bg-neutral-900/80 text-neutral-400 hover:text-white hover:border-neutral-700 transition-colors cursor-pointer"
                        >
                          {tpl.name}
                        </button>
                      {/each}
                    </div>
                  </div>
                </div>
              </div>

              <!-- Seletor de Fundos Disponíveis (Grade de Navegação) -->
              <div class="rounded-xl border border-neutral-800 bg-neutral-900/20 p-4 space-y-3">
                <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                  <h4 class="text-xs font-semibold text-neutral-300">Escolha um Fundo para Personalizar</h4>
                  
                  <div class="flex items-center gap-2">
                    <input
                      type="text"
                      bind:value={bgSearchTerm}
                      placeholder="Pesquisar fundo..."
                      class="h-7 px-2.5 rounded-md border border-neutral-800 bg-neutral-900 text-xs text-foreground placeholder:text-neutral-500 focus:outline-none focus:ring-1 focus:ring-primary w-36 sm:w-48"
                    />
                    <div class="flex items-center rounded-lg border border-neutral-800 bg-neutral-900 p-0.5 text-xs">
                      {#each [
                        { id: 'all', label: 'Todos' },
                        { id: 'gradients', label: 'Gradientes' },
                        { id: 'mesh', label: 'Mesh' },
                        { id: 'animated', label: 'Animados' },
                        { id: 'custom', label: 'Custom' }
                      ] as cat}
                        <button
                          type="button"
                          onclick={() => (bgCategoryFilter = cat.id as any)}
                          class="px-2 py-0.5 rounded-md text-[11px] font-medium transition-colors cursor-pointer {bgCategoryFilter === cat.id ? 'bg-primary text-primary-foreground font-semibold' : 'text-neutral-400 hover:text-neutral-200'}"
                        >
                          {cat.label}
                        </button>
                      {/each}
                    </div>
                  </div>
                </div>

                <!-- Grade de Cartões com Prévia -->
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2.5 max-h-56 overflow-y-auto pr-1">
                  {#each filteredBgList as preset (preset.id)}
                    {@const isSelected = preset.id === currentSelectedPreset.id}
                    {@const isOverridden = !!localBgStyles.overrides[preset.id]}
                    {@const isDefault = localBgStyles.defaultBackgroundId === preset.id}
                    <button
                      type="button"
                      onclick={() => (selectedPresetId = preset.id)}
                      class="group relative rounded-lg border overflow-hidden flex flex-col text-left transition-all cursor-pointer bg-neutral-900/80 {isSelected ? 'border-primary ring-2 ring-primary/80 shadow-lg' : 'border-neutral-800 hover:border-neutral-700'}"
                    >
                      <div class="aspect-video w-full relative overflow-hidden flex items-center justify-center">
                        <div
                          class="absolute inset-0 w-full h-full {preset.animationClass === 'none' ? '' : (preset.animationClass || '')}"
                          style="background: {preset.style};"
                        ></div>
                        <span
                          class="relative z-10 text-[10px] font-bold drop-shadow-[0_1px_4px_rgba(0,0,0,0.9)]"
                          style="color: {preset.textColor || '#ffffff'};"
                        >
                          Aa
                        </span>

                        {#if isDefault}
                          <span class="absolute top-1 left-1 bg-primary text-primary-foreground text-[8px] font-bold px-1 rounded shadow">
                            Padrão
                          </span>
                        {/if}
                        {#if isOverridden}
                          <span class="absolute top-1 right-1 bg-amber-500/90 text-black text-[8px] font-bold px-1 rounded shadow">
                            Modificado
                          </span>
                        {:else if preset.isCustom}
                          <span class="absolute top-1 right-1 bg-emerald-500/90 text-black text-[8px] font-bold px-1 rounded shadow">
                            Custom
                          </span>
                        {/if}
                      </div>

                      <div class="p-1.5 flex items-center justify-between">
                        <span class="text-[11px] font-medium text-neutral-200 truncate">{preset.name}</span>
                      </div>
                    </button>
                  {/each}
                </div>
              </div>
            </div>

          <!-- ABA: MONITOR DE PALCO (STAGE) -->
          {:else if activeTab === 'stage'}
            <div class="space-y-6 max-w-2xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Layout e Cores do Monitor de Retorno</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Configure de forma independente o retorno de letras de louvor, versículos bíblicos e o layout/timer do palco.
                </p>
              </div>

              <!-- Seletor de Seções do Monitor de Retorno (Stage) -->
              <div class="p-1 bg-neutral-950 rounded-xl border border-neutral-800 grid grid-cols-3 gap-1">
                <button
                  type="button"
                  onclick={() => (stageSection = 'lyrics')}
                  class="flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer {stageSection === 'lyrics' ? 'bg-primary text-primary-foreground shadow' : 'text-neutral-400 hover:text-white'}"
                >
                  <Music size={13} />
                  <span>Letras de Louvor</span>
                </button>

                <button
                  type="button"
                  onclick={() => (stageSection = 'scripture')}
                  class="flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer {stageSection === 'scripture' ? 'bg-primary text-primary-foreground shadow' : 'text-neutral-400 hover:text-white'}"
                >
                  <BookOpen size={13} />
                  <span>Textos Bíblicos</span>
                </button>

                <button
                  type="button"
                  onclick={() => (stageSection = 'general')}
                  class="flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer {stageSection === 'general' ? 'bg-primary text-primary-foreground shadow' : 'text-neutral-400 hover:text-white'}"
                >
                  <Sliders size={13} />
                  <span>Layout & Tempo</span>
                </button>
              </div>

              {#if stageSection === 'lyrics'}
                <!-- SEÇÃO 1: LETRAS DE LOUVOR NO PALCO -->
                <div class="space-y-4">
                  <!-- Auto-scale das Letras no Palco -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <div>
                      <h4 class="text-xs font-semibold text-white flex items-center gap-2">
                        <Type size={14} class="text-primary" />
                        Tamanho Mínimo da Fonte para Louvores
                      </h4>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Define o menor tamanho aceitável para letras de músicas no monitor de retorno, evitando que estrofes longas fiquem ilegíveis para a equipe de louvor.
                      </p>
                    </div>
                    <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2 max-w-md">
                      <div class="flex justify-between items-center text-xs">
                        <span class="text-neutral-300 font-medium">Tamanho Mínimo das Letras</span>
                        <span class="font-mono text-primary font-bold bg-primary/10 px-2 py-0.5 rounded border border-primary/30">
                          {localStage.lyrics_min_font_size ?? localStage.min_font_size ?? 24}px
                        </span>
                      </div>
                      <input
                        type="range"
                        min="14"
                        max="64"
                        step="2"
                        value={localStage.lyrics_min_font_size ?? localStage.min_font_size ?? 24}
                        oninput={(e) => {
                          const val = Number((e.target as HTMLInputElement).value);
                          localStage = { ...localStage, lyrics_min_font_size: val, min_font_size: val };
                          saveStageDisplaySettings(localStage);
                        }}
                        class="w-full accent-primary cursor-pointer"
                      />
                      <div class="flex justify-between text-[10px] text-neutral-500 font-mono">
                        <span>14px (Mais linhas)</span>
                        <span>24px (Padrão)</span>
                        <span>64px (Super legível)</span>
                      </div>
                    </div>
                  </div>

                  <!-- Cores das Letras de Louvor no Palco -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-4">
                    <div class="flex items-center justify-between">
                      <div>
                        <h4 class="text-xs font-semibold text-white flex items-center gap-2">
                          <Sliders size={14} class="text-primary" />
                          Cores das Letras e Molduras do Palco
                        </h4>
                        <p class="text-[11px] text-neutral-400 mt-0.5">
                          Personalize as cores do slide ao vivo e do próximo slide da música para contraste com a iluminação do palco.
                        </p>
                      </div>
                      <Button
                        variant="outline"
                        size="sm"
                        class="text-[11px] h-7"
                        onclick={() => {
                          localStage = {
                            ...localStage,
                            current_text_color: DEFAULT_STAGE_SETTINGS.current_text_color,
                            next_text_color: DEFAULT_STAGE_SETTINGS.next_text_color,
                            current_border_color: DEFAULT_STAGE_SETTINGS.current_border_color,
                            next_border_color: DEFAULT_STAGE_SETTINGS.next_border_color,
                          };
                          saveStageDisplaySettings(localStage);
                        }}
                      >
                        Restaurar Cores
                      </Button>
                    </div>

                    <div class="grid grid-cols-2 sm:grid-cols-2 gap-3">
                      <!-- Texto Slide Atual -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Texto do Slide Atual</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.current_text_color}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.current_text_color}
                          oninput={(e) => updateStageField('current_text_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>

                      <!-- Borda Slide Atual -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Moldura do Slide Atual</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.current_border_color}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.current_border_color}
                          oninput={(e) => updateStageField('current_border_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>

                      <!-- Texto Próximo Slide -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Texto do Próximo Slide</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.next_text_color}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.next_text_color}
                          oninput={(e) => updateStageField('next_text_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>

                      <!-- Borda Próximo Slide -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Moldura do Próximo Slide</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.next_border_color}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.next_border_color}
                          oninput={(e) => updateStageField('next_border_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>
                    </div>
                  </div>
                </div>

              {:else if stageSection === 'scripture'}
                <!-- SEÇÃO 2: TEXTOS BÍBLICOS NO PALCO -->
                <div class="space-y-4">
                  <!-- Auto-scale dos Versículos Bíblicos no Palco -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <div>
                      <h4 class="text-xs font-semibold text-white flex items-center gap-2">
                        <Type size={14} class="text-primary" />
                        Tamanho Mínimo da Fonte para Passagens Bíblicas
                      </h4>
                      <p class="text-[11px] text-neutral-400 mt-0.5">
                        Versículos contêm blocos narrativos mais extensos. Ajuste o tamanho mínimo para garantir que o pregador no púlpito consiga ler confortavelmente no monitor de retorno.
                      </p>
                    </div>
                    <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2 max-w-md">
                      <div class="flex justify-between items-center text-xs">
                        <span class="text-neutral-300 font-medium">Tamanho Mínimo Bíblico</span>
                        <span class="font-mono text-primary font-bold bg-primary/10 px-2 py-0.5 rounded border border-primary/30">
                          {localStage.scripture_min_font_size ?? 24}px
                        </span>
                      </div>
                      <input
                        type="range"
                        min="14"
                        max="64"
                        step="2"
                        value={localStage.scripture_min_font_size ?? 24}
                        oninput={(e) => updateStageField('scripture_min_font_size', Number((e.target as HTMLInputElement).value))}
                        class="w-full accent-primary cursor-pointer"
                      />
                      <div class="flex justify-between text-[10px] text-neutral-500 font-mono">
                        <span>14px (Mais compacto)</span>
                        <span>24px (Padrão)</span>
                        <span>64px (Máxima visibilidade)</span>
                      </div>
                    </div>
                  </div>

                  <!-- Cores de Exibição Bíblica no Palco -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-4">
                    <div class="flex items-center justify-between">
                      <div>
                        <h4 class="text-xs font-semibold text-white flex items-center gap-2">
                          <Sliders size={14} class="text-primary" />
                          Cores do Versículo no Monitor de Palco
                        </h4>
                        <p class="text-[11px] text-neutral-400 mt-0.5">
                          Defina cores personalizadas para as passagens bíblicas no retorno do pregador.
                        </p>
                      </div>
                      <Button
                        variant="outline"
                        size="sm"
                        class="text-[11px] h-7"
                        onclick={() => {
                          localStage = {
                            ...localStage,
                            scripture_text_color: DEFAULT_STAGE_SETTINGS.scripture_text_color,
                            scripture_border_color: DEFAULT_STAGE_SETTINGS.scripture_border_color,
                          };
                          saveStageDisplaySettings(localStage);
                        }}
                      >
                        Restaurar Padrão
                      </Button>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                      <!-- Cor do Texto Bíblico -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Cor do Texto do Versículo</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.scripture_text_color || '#FFFFFF'}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.scripture_text_color || '#FFFFFF'}
                          oninput={(e) => updateStageField('scripture_text_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>

                      <!-- Cor da Moldura Bíblica -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Moldura do Bloco Bíblico</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.scripture_border_color || '#3B82F6'}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.scripture_border_color || '#3B82F6'}
                          oninput={(e) => updateStageField('scripture_border_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>
                    </div>
                  </div>
                </div>

              {:else}
                <!-- SEÇÃO 3: LAYOUT & TEMPO -->
                <div class="space-y-4">
                  <!-- Preset do Palco (Grid Moderno Pro Padronizado) -->
                  <div class="p-4 rounded-xl border border-primary bg-primary/10 ring-1 ring-primary/40 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                    <div class="space-y-1">
                      <div class="flex items-center gap-2">
                        <div class="text-sm font-bold text-white">Grid Moderno Pro</div>
                        <span class="px-2 py-0.5 rounded text-[10px] font-semibold bg-emerald-500/20 text-emerald-400 border border-emerald-500/40">
                          Padrão Ativo
                        </span>
                      </div>
                      <p class="text-xs text-neutral-300">
                        Slide atual em destaque com auto-scale, próximo slide abaixo e coluna lateral dedicada com aviso sempre visível, relógio e cronômetro. Sem gaps nem bordas arredondadas para aproveitamento máximo da tela.
                      </p>
                    </div>
                    <div class="h-16 w-36 bg-neutral-950 rounded border border-neutral-800 p-1 flex gap-1 shrink-0">
                      <div class="flex-[2] flex flex-col gap-0.5">
                        <div class="flex-1 border border-blue-500/40 bg-blue-500/10 text-[7px] text-blue-300 font-bold flex items-center justify-center">Slide Atual</div>
                        <div class="h-4 border border-yellow-500/40 bg-yellow-500/10 text-[6px] text-yellow-300 font-bold flex items-center justify-center">A Seguir</div>
                      </div>
                      <div class="flex-1 border border-neutral-700 bg-neutral-900 text-[6px] text-neutral-300 font-bold flex flex-col items-center justify-center gap-0.5">
                        <span class="text-emerald-400">Aviso</span>
                        <span class="text-neutral-400">12:00</span>
                        <span class="text-red-400 font-mono">10:00</span>
                      </div>
                    </div>
                  </div>

                  <!-- Cores do Timer e Relógio -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-3">
                    <h4 class="text-xs font-semibold text-white flex items-center gap-2">
                      <Clock size={14} class="text-primary" />
                      Cores do Relógio e Cronômetro
                    </h4>
                    <div class="grid grid-cols-2 gap-3">
                      <!-- Cor do Timer -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Cor do Timer / Cronômetro</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.timer_color}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.timer_color}
                          oninput={(e) => updateStageField('timer_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>

                      <!-- Cor do Relógio -->
                      <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-950 flex items-center justify-between">
                        <div>
                          <div class="text-[11px] font-semibold text-white">Cor do Relógio</div>
                          <div class="text-[9px] text-neutral-400 font-mono">{localStage.clock_color}</div>
                        </div>
                        <input
                          type="color"
                          value={localStage.clock_color}
                          oninput={(e) => updateStageField('clock_color', (e.target as HTMLInputElement).value)}
                          class="w-8 h-8 rounded border-0 cursor-pointer bg-transparent"
                        />
                      </div>
                    </div>
                  </div>

                  <!-- Proporções de Preenchimento dos Blocos na Tela -->
                  <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-4">
                    <div class="flex items-center justify-between">
                      <div>
                        <h4 class="text-xs font-semibold text-white">Distribuição de Espaço na Tela (Proporções dos Blocos)</h4>
                        <p class="text-[11px] text-neutral-400 mt-0.5">
                          Defina a porcentagem de área dedicada ao slide atual, próximo slide e barra de tempo no retorno.
                        </p>
                      </div>
                      <Button
                        variant="outline"
                        size="sm"
                        class="text-[11px] h-7"
                        onclick={() => {
                          localStage = {
                            ...localStage,
                            current_slide_percent: DEFAULT_STAGE_SETTINGS.current_slide_percent,
                            next_slide_percent: DEFAULT_STAGE_SETTINGS.next_slide_percent,
                            footer_time_percent: DEFAULT_STAGE_SETTINGS.footer_time_percent,
                          };
                          saveStageDisplaySettings(localStage);
                        }}
                      >
                        Restaurar Proporções
                      </Button>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2">
                        <div class="flex justify-between text-xs">
                          <span class="text-neutral-400">Slide Atual</span>
                          <span class="font-mono text-white font-semibold">{localStage.current_slide_percent ?? 60}%</span>
                        </div>
                        <input
                          type="range"
                          min="30"
                          max="80"
                          step="5"
                          value={localStage.current_slide_percent ?? 60}
                          oninput={(e) => updateStageField('current_slide_percent', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                        <div class="text-[9px] text-neutral-500">Área principal do texto ao vivo</div>
                      </div>

                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2">
                        <div class="flex justify-between text-xs">
                          <span class="text-neutral-400">Próximo Slide</span>
                          <span class="font-mono text-white font-semibold">{localStage.next_slide_percent ?? 30}%</span>
                        </div>
                        <input
                          type="range"
                          min="10"
                          max="50"
                          step="5"
                          value={localStage.next_slide_percent ?? 30}
                          oninput={(e) => updateStageField('next_slide_percent', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                        <div class="text-[9px] text-neutral-500">Prévia do próximo trecho</div>
                      </div>

                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-950 space-y-2">
                        <div class="flex justify-between text-xs">
                          <span class="text-neutral-400">Relógio & Timer</span>
                          <span class="font-mono text-white font-semibold">{localStage.footer_time_percent ?? 10}%</span>
                        </div>
                        <input
                          type="range"
                          min="8"
                          max="25"
                          step="1"
                          value={localStage.footer_time_percent ?? 10}
                          oninput={(e) => updateStageField('footer_time_percent', Number((e.target as HTMLInputElement).value))}
                          class="w-full accent-primary"
                        />
                        <div class="text-[9px] text-neutral-500">Altura da barra lateral</div>
                      </div>
                    </div>

                    <!-- Amostra Gráfica Interativa da Divisão Proporcional -->
                    <div class="p-4 rounded-xl border border-neutral-800 bg-black flex flex-col items-center justify-center">
                      <div class="flex items-center justify-between w-full max-w-sm mb-2">
                        <span class="text-[10px] text-neutral-500 font-mono uppercase">Simulação do Retorno</span>
                        <span class="text-[10px] px-2 py-0.5 rounded bg-primary/20 text-primary font-mono font-semibold uppercase">
                          Preset: {localStage.preset}
                        </span>
                      </div>

                      <div class="w-80 max-w-full">
                        <StagePreviewBox
                          stageSettings={localStage}
                          activeSlide={{
                            id: 'demo_cur',
                            label: 'João 3:16',
                            content: 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito',
                          }}
                          nextSlide={{
                            id: 'demo_next',
                            label: 'João 3:17',
                            content: 'Para que todo aquele que nele crê não pereça, mas tenha a vida eterna',
                          }}
                          activeMedia={{
                            id: 'demo_med',
                            title: 'Evangelho de João',
                            kind: 'scripture',
                            artist_or_ref: 'ARA',
                            slides: [],
                          }}
                          timer={{
                            duration_seconds: 600,
                            remaining_seconds: 450,
                            running: true,
                            is_overtime: false,
                            allow_overtime: false,
                            warning_threshold_seconds: 60,
                            stage_message: 'Aviso Palco',
                          }}
                          stageClock="19:45:00"
                        />
                      </div>
                    </div>
                  </div>
                </div>
              {/if}
            </div>

          <!-- ABA: LOWER THIRD (OBS / TRANSMISSÃO) -->
          {:else if activeTab === 'lower_third'}
            <div class="space-y-6">
              <div class="flex items-center justify-between pb-3 border-b border-neutral-800">
                <div>
                  <h3 class="text-sm font-semibold text-white flex items-center gap-2">
                    <Video size={16} class="text-primary" />
                    <span>Saída Lower Third Transparente para OBS Studio / vMix</span>
                  </h3>
                  <p class="text-xs text-neutral-400 mt-0.5">
                    Projeta letras e passagens bíblicas no terço inferior da tela com fundo transparente (canal alfa nativo).
                  </p>
                </div>

                <!-- Botão Testar / Abrir -->
                <a
                  href="/lower-third"
                  target="_blank"
                  rel="noreferrer"
                  class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-xs font-semibold text-neutral-200 transition-colors"
                >
                  <ExternalLink size={13} />
                  <span>Abrir /lower-third</span>
                </a>
              </div>

              <!-- URL e Instruções Rápidas para OBS -->
              <div class="p-3.5 rounded-xl bg-neutral-900 border border-neutral-800 space-y-2">
                <span class="text-xs font-bold text-neutral-200 block">URL da Fonte de Navegador (Browser Source):</span>
                <div class="flex items-center gap-2">
                  <input
                    type="text"
                    readonly
                    value="{typeof window !== 'undefined' ? window.location.origin : 'http://localhost:18080'}/lower-third"
                    class="flex-1 px-3 py-1.5 rounded-lg bg-black border border-neutral-800 text-xs font-mono text-emerald-400 select-all"
                  />
                  <Button
                    variant="outline"
                    size="sm"
                    onclick={() => {
                      if (typeof window !== 'undefined') {
                        navigator.clipboard.writeText(`${window.location.origin}/lower-third`);
                      }
                    }}
                    class="shrink-0 gap-1 text-xs"
                  >
                    <Copy size={13} />
                    <span>Copiar</span>
                  </Button>
                </div>
                <p class="text-[11px] text-neutral-400">
                  No OBS Studio, adicione uma fonte <strong>"Navegador" (Browser Source)</strong> com resolução <strong>1920x1080</strong> e marque <em>"Desligar fonte quando invisível"</em>.
                </p>
              </div>

              <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
                <!-- Coluna de Controles (Esquerda) -->
                <div class="lg:col-span-7 space-y-5">
                  <!-- Ativação Geral -->
                  <div class="p-4 rounded-xl bg-neutral-900 border border-neutral-800 flex items-center justify-between">
                    <div>
                      <span class="text-xs font-bold text-white block">Ativar Saída Lower Third</span>
                      <p class="text-[11px] text-neutral-400">
                        Quando desativado, o texto é ocultado automaticamente da transmissão.
                      </p>
                    </div>
                    <button
                      type="button"
                      role="switch"
                      aria-label="Ativar Saída Lower Third"
                      aria-checked={localLowerThird.enabled}
                      onclick={() => (localLowerThird.enabled = !localLowerThird.enabled)}
                      class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out {localLowerThird.enabled ? 'bg-emerald-600' : 'bg-neutral-800'}"
                    >
                      <span
                        class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localLowerThird.enabled ? 'translate-x-5' : 'translate-x-0'}"
                      ></span>
                    </button>
                  </div>

                  <!-- Seletor de Tema / Estilo Visual -->
                  <div class="space-y-2">
                    <span class="text-xs font-bold text-neutral-200 block">Estilo Visual do Lower Third:</span>
                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
                      <button
                        type="button"
                        onclick={() => (localLowerThird.theme = 'gradient')}
                        class="p-2.5 rounded-xl border text-left transition-all cursor-pointer {localLowerThird.theme === 'gradient' ? 'border-primary bg-primary/10 text-white font-bold' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                      >
                        <span class="text-xs block">Gradiente Central</span>
                        <span class="text-[10px] opacity-70 font-normal">Suave & Dinâmico</span>
                      </button>

                      <button
                        type="button"
                        onclick={() => (localLowerThird.theme = 'bar')}
                        class="p-2.5 rounded-xl border text-left transition-all cursor-pointer {localLowerThird.theme === 'bar' ? 'border-primary bg-primary/10 text-white font-bold' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                      >
                        <span class="text-xs block">Barra Inferior</span>
                        <span class="text-[10px] opacity-70 font-normal">Estilo TV / Notícias</span>
                      </button>

                      <button
                        type="button"
                        onclick={() => (localLowerThird.theme = 'floating')}
                        class="p-2.5 rounded-xl border text-left transition-all cursor-pointer {localLowerThird.theme === 'floating' ? 'border-primary bg-primary/10 text-white font-bold' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                      >
                        <span class="text-xs block">Caixa Flutuante</span>
                        <span class="text-[10px] opacity-70 font-normal">Glassmorphism</span>
                      </button>

                      <button
                        type="button"
                        onclick={() => (localLowerThird.theme = 'minimal')}
                        class="p-2.5 rounded-xl border text-left transition-all cursor-pointer {localLowerThird.theme === 'minimal' ? 'border-primary bg-primary/10 text-white font-bold' : 'border-neutral-800 bg-neutral-900/60 text-neutral-400 hover:text-white'}"
                      >
                        <span class="text-xs block">Minimalista</span>
                        <span class="text-[10px] opacity-70 font-normal">Apenas Tipografia</span>
                      </button>
                    </div>
                  </div>

                  <!-- Cores e Gradiente -->
                  <div class="p-4 rounded-xl bg-neutral-900 border border-neutral-800 space-y-4">
                    <h4 class="text-xs font-bold text-neutral-200">Cores & Composição de Fundo</h4>

                    <div class="flex items-center justify-between">
                      <div>
                        <span class="text-xs font-semibold text-neutral-300 block">Vincular Cor ao Fundo Público Ativo</span>
                        <span class="text-[11px] text-neutral-500">Adapta automaticamente o gradiente à cor do tema de projeção.</span>
                      </div>
                      <button
                        type="button"
                        role="switch"
                        aria-label="Vincular Cor ao Fundo Público Ativo"
                        aria-checked={localLowerThird.use_public_bg}
                        onclick={() => (localLowerThird.use_public_bg = !localLowerThird.use_public_bg)}
                        class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out {localLowerThird.use_public_bg ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      >
                        <span
                          class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localLowerThird.use_public_bg ? 'translate-x-5' : 'translate-x-0'}"
                        ></span>
                      </button>
                    </div>

                    {#if !localLowerThird.use_public_bg}
                      <div class="flex items-center justify-between pt-2 border-t border-neutral-800">
                        <span class="text-xs text-neutral-300">Cor Base do Gradiente / Barra:</span>
                        <div class="flex items-center gap-2">
                          <input
                            type="color"
                            bind:value={localLowerThird.bg_color}
                            class="w-8 h-8 rounded border border-neutral-700 bg-transparent cursor-pointer"
                          />
                          <input
                            type="text"
                            bind:value={localLowerThird.bg_color}
                            class="w-20 px-2 py-1 rounded bg-black border border-neutral-800 text-xs font-mono uppercase text-center"
                          />
                        </div>
                      </div>
                    {/if}

                    <!-- Opacidade do Fundo -->
                    <div class="space-y-1 pt-1">
                      <div class="flex justify-between text-xs text-neutral-300">
                        <span>Opacidade do Fundo / Gradiente:</span>
                        <span class="font-mono text-primary font-bold">{localLowerThird.bg_opacity}%</span>
                      </div>
                      <input
                        type="range"
                        min="10"
                        max="100"
                        step="5"
                        bind:value={localLowerThird.bg_opacity}
                        class="w-full accent-primary cursor-pointer"
                      />
                    </div>

                    <!-- Margem Inferior -->
                    <div class="space-y-1 pt-1">
                      <div class="flex justify-between text-xs text-neutral-300">
                        <span>Distância da Margem Inferior:</span>
                        <span class="font-mono text-primary font-bold">{localLowerThird.margin_bottom}px</span>
                      </div>
                      <input
                        type="range"
                        min="16"
                        max="80"
                        step="4"
                        bind:value={localLowerThird.margin_bottom}
                        class="w-full accent-primary cursor-pointer"
                      />
                    </div>

                    <!-- Largura Máxima -->
                    <div class="space-y-1 pt-1">
                      <div class="flex justify-between text-xs text-neutral-300">
                        <span>Aproveitamento Máximo de Largura:</span>
                        <span class="font-mono text-primary font-bold">{localLowerThird.max_width_percent}%</span>
                      </div>
                      <input
                        type="range"
                        min="50"
                        max="95"
                        step="5"
                        bind:value={localLowerThird.max_width_percent}
                        class="w-full accent-primary cursor-pointer"
                      />
                    </div>
                  </div>

                  <!-- Tipografia e Sombra -->
                  <div class="p-4 rounded-xl bg-neutral-900 border border-neutral-800 space-y-4">
                    <h4 class="text-xs font-bold text-neutral-200">Tipografia & Contraste para Transmissão</h4>

                    <!-- Tamanho da Fonte -->
                    <div class="space-y-1">
                      <div class="flex justify-between text-xs text-neutral-300">
                        <span>Tamanho da Fonte:</span>
                        <span class="font-mono text-primary font-bold">{localLowerThird.font_size}px</span>
                      </div>
                      <input
                        type="range"
                        min="20"
                        max="56"
                        step="2"
                        bind:value={localLowerThird.font_size}
                        class="w-full accent-primary cursor-pointer"
                      />
                    </div>

                    <div class="grid grid-cols-2 gap-3 pt-1">
                      <!-- Alinhamento -->
                      <div>
                        <span class="text-xs text-neutral-300 block mb-1">Alinhamento:</span>
                        <select
                          bind:value={localLowerThird.text_align}
                          class="w-full px-2.5 py-1.5 rounded-lg bg-black border border-neutral-800 text-xs text-neutral-200 font-semibold"
                        >
                          <option value="left">Esquerda</option>
                          <option value="center">Centralizado</option>
                          <option value="right">Direita</option>
                        </select>
                      </div>

                      <!-- Peso da Fonte -->
                      <div>
                        <span class="text-xs text-neutral-300 block mb-1">Peso da Fonte:</span>
                        <select
                          bind:value={localLowerThird.font_weight}
                          class="w-full px-2.5 py-1.5 rounded-lg bg-black border border-neutral-800 text-xs text-neutral-200 font-semibold"
                        >
                          <option value="normal">Normal (400)</option>
                          <option value="semibold">Semibold (600)</option>
                          <option value="bold">Negrito (700)</option>
                          <option value="extrabold">Extra Negrito (800)</option>
                        </select>
                      </div>
                    </div>

                    <div class="grid grid-cols-2 gap-3 pt-1">
                      <!-- Força da Sombra -->
                      <div>
                        <span class="text-xs text-neutral-300 block mb-1">Força da Sombra do Texto:</span>
                        <select
                          bind:value={localLowerThird.shadow_strength}
                          class="w-full px-2.5 py-1.5 rounded-lg bg-black border border-neutral-800 text-xs text-neutral-200 font-semibold"
                        >
                          <option value="none">Nenhuma</option>
                          <option value="subtle">Suave</option>
                          <option value="medium">Média</option>
                          <option value="strong">Forte (Alto Contraste)</option>
                        </select>
                      </div>

                      <!-- Animação -->
                      <div>
                        <span class="text-xs text-neutral-300 block mb-1">Animação de Transição:</span>
                        <select
                          bind:value={localLowerThird.animation}
                          class="w-full px-2.5 py-1.5 rounded-lg bg-black border border-neutral-800 text-xs text-neutral-200 font-semibold"
                        >
                          <option value="fade">Fade Suave</option>
                          <option value="slide">Deslize Vertical (Slide)</option>
                          <option value="zoom">Zoom Leve</option>
                          <option value="none">Instantânea</option>
                        </select>
                      </div>
                    </div>

                    <!-- Exibir Referência -->
                    <div class="flex items-center justify-between pt-2 border-t border-neutral-800">
                      <div>
                        <span class="text-xs font-semibold text-neutral-300 block">Exibir Referência Bíblica / Título</span>
                        <span class="text-[11px] text-neutral-500">Mostra etiqueta identificadora no topo do Lower Third.</span>
                      </div>
                      <button
                        type="button"
                        role="switch"
                        aria-label="Exibir Referência Bíblica ou Título"
                        aria-checked={localLowerThird.show_reference}
                        onclick={() => (localLowerThird.show_reference = !localLowerThird.show_reference)}
                        class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out {localLowerThird.show_reference ? 'bg-emerald-600' : 'bg-neutral-800'}"
                      >
                        <span
                          class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localLowerThird.show_reference ? 'translate-x-5' : 'translate-x-0'}"
                        ></span>
                      </button>
                    </div>
                  </div>
                </div>

                <!-- Coluna de Prévia Interativa 16:9 (Direita) -->
                <div class="lg:col-span-5 space-y-3">
                  <div class="flex items-center justify-between">
                    <span class="text-xs font-bold text-neutral-200">Prévia 16:9 (Simulação de Vídeo OBS):</span>
                    <!-- Alternador de Conteúdo da Prévia -->
                    <div class="inline-flex rounded-lg bg-neutral-900 border border-neutral-800 p-0.5">
                      <button
                        type="button"
                        onclick={() => (ltPreviewMode = 'lyrics')}
                        class="px-2 py-0.5 text-[10px] font-bold rounded {ltPreviewMode === 'lyrics' ? 'bg-primary text-primary-foreground' : 'text-neutral-400'}"
                      >
                        🎵 Louvor
                      </button>
                      <button
                        type="button"
                        onclick={() => (ltPreviewMode = 'scripture')}
                        class="px-2 py-0.5 text-[10px] font-bold rounded {ltPreviewMode === 'scripture' ? 'bg-primary text-primary-foreground' : 'text-neutral-400'}"
                      >
                        📖 Bíblia
                      </button>
                    </div>
                  </div>

                  <!-- Container 16:9 com Fundo de Simulação de Câmera/Vídeo -->
                  <div class="relative w-full aspect-video rounded-xl overflow-hidden border border-neutral-800 shadow-2xl flex flex-col justify-end bg-gradient-to-tr from-slate-900 via-indigo-950 to-neutral-900">
                    <!-- Overlay de iluminação simulando câmera de palco -->
                    <div class="absolute inset-0 bg-radial from-transparent via-black/20 to-black/60 pointer-events-none"></div>

                    <!-- Indicador simulador de Live OBS -->
                    <div class="absolute top-2 left-2 px-1.5 py-0.5 rounded bg-red-600/80 text-white font-mono text-[9px] font-bold uppercase tracking-wider flex items-center gap-1 z-10">
                      <span class="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></span>
                      <span>LIVE OBS</span>
                    </div>

                    <!-- Lower Third na Prévia -->
                    <div
                      class="w-full flex flex-col items-center justify-end z-10 p-3 transition-all"
                      style="padding-bottom: {Math.max(8, Math.round((localLowerThird.margin_bottom ?? 36) * 0.35))}px;"
                    >
                      <div
                        class="w-full transition-all"
                        style="max-width: {localLowerThird.max_width_percent ?? 85}%;"
                      >
                        {#if localLowerThird.enabled}
                          <!-- Prévia Gradiente -->
                          {#if localLowerThird.theme === 'gradient'}
                            <div
                              class="px-4 py-2 rounded-xl text-center flex flex-col items-center justify-center overflow-hidden"
                              style="background: radial-gradient(ellipse at center, rgba(0,0,0,{(localLowerThird.bg_opacity ?? 75) / 100}) 0%, rgba(0,0,0,{(localLowerThird.bg_opacity ?? 75) / 100}) 60%, transparent 100%);"
                            >
                              {#if localLowerThird.show_reference}
                                <span class="px-2 py-0.5 rounded-full text-[8px] font-bold uppercase tracking-wider mb-1 bg-black/60 border border-white/20 text-white shadow">
                                  {ltPreviewMode === 'scripture' ? '📖 João 3:16 (ARA)' : '🎵 Porque Ele Vive'}
                                </span>
                              {/if}
                              <p
                                class="leading-tight {localLowerThird.text_align === 'left' ? 'text-left' : localLowerThird.text_align === 'right' ? 'text-right' : 'text-center'}"
                                style="font-size: {Math.max(9, Math.round((localLowerThird.font_size ?? 32) * 0.35))}px; font-weight: {localLowerThird.font_weight === 'extrabold' ? '800' : localLowerThird.font_weight === 'bold' ? '700' : localLowerThird.font_weight === 'semibold' ? '600' : '400'}; color: {localLowerThird.text_color || '#ffffff'}; text-shadow: 0 1px 3px rgba(0,0,0,0.9);"
                              >
                                {ltPreviewMode === 'scripture'
                                  ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                                  : 'Porque Ele vive, eu posso crer no amanhã'}
                              </p>
                            </div>

                          <!-- Prévia Barra -->
                          {:else if localLowerThird.theme === 'bar'}
                            <div
                              class="w-full px-4 py-1.5 rounded-lg border-l-4 border-primary text-left bg-black/75 shadow-lg"
                            >
                              {#if localLowerThird.show_reference}
                                <span class="text-[8px] font-bold uppercase text-emerald-400 block">
                                  {ltPreviewMode === 'scripture' ? '📖 João 3:16 (ARA)' : '🎵 Porque Ele Vive'}
                                </span>
                              {/if}
                              <p
                                class="leading-tight {localLowerThird.text_align === 'left' ? 'text-left' : localLowerThird.text_align === 'right' ? 'text-right' : 'text-center'}"
                                style="font-size: {Math.max(9, Math.round((localLowerThird.font_size ?? 32) * 0.35))}px; font-weight: {localLowerThird.font_weight === 'extrabold' ? '800' : localLowerThird.font_weight === 'bold' ? '700' : '600'}; color: {localLowerThird.text_color || '#ffffff'};"
                              >
                                {ltPreviewMode === 'scripture'
                                  ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                                  : 'Porque Ele vive, eu posso crer no amanhã'}
                              </p>
                            </div>

                          <!-- Prévia Floating -->
                          {:else if localLowerThird.theme === 'floating'}
                            <div class="px-4 py-1.5 rounded-2xl bg-black/70 border border-white/20 text-center mx-auto shadow-lg backdrop-blur-sm">
                              {#if localLowerThird.show_reference}
                                <span class="text-[8px] font-bold uppercase text-white/80 block mb-0.5">
                                  {ltPreviewMode === 'scripture' ? '📖 João 3:16' : '🎵 Porque Ele Vive'}
                                </span>
                              {/if}
                              <p
                                class="leading-tight"
                                style="font-size: {Math.max(9, Math.round((localLowerThird.font_size ?? 32) * 0.35))}px; font-weight: {localLowerThird.font_weight === 'extrabold' ? '800' : localLowerThird.font_weight === 'bold' ? '700' : '600'}; color: {localLowerThird.text_color || '#ffffff'};"
                              >
                                {ltPreviewMode === 'scripture'
                                  ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                                  : 'Porque Ele vive, eu posso crer no amanhã'}
                              </p>
                            </div>

                          <!-- Prévia Minimal -->
                          {:else}
                            <div class="text-center w-full">
                              {#if localLowerThird.show_reference}
                                <span class="text-[8px] font-bold uppercase text-amber-300 block mb-0.5 drop-shadow">
                                  {ltPreviewMode === 'scripture' ? '📖 João 3:16' : '🎵 Porque Ele Vive'}
                                </span>
                              {/if}
                              <p
                                class="leading-tight drop-shadow-lg"
                                style="font-size: {Math.max(9, Math.round((localLowerThird.font_size ?? 32) * 0.35))}px; font-weight: {localLowerThird.font_weight === 'extrabold' ? '800' : localLowerThird.font_weight === 'bold' ? '700' : '600'}; color: {localLowerThird.text_color || '#ffffff'}; text-shadow: 0 2px 6px rgba(0,0,0,1);"
                              >
                                {ltPreviewMode === 'scripture'
                                  ? 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito'
                                  : 'Porque Ele vive, eu posso crer no amanhã'}
                              </p>
                            </div>
                          {/if}
                        {:else}
                          <div class="p-3 text-center text-[10px] text-neutral-400 italic">
                            Lower Third desativado.
                          </div>
                        {/if}
                      </div>
                    </div>
                  </div>

                  <p class="text-[11px] text-neutral-400">
                    A visualização acima simula como o texto aparecerá no OBS Studio sobreposto ao feed da câmera de transmissão.
                  </p>
                </div>
              </div>
            </div>

          <!-- ABA: MARCA DA IGREJA (CHURCH BRAND) -->
          {:else if activeTab === 'church'}
            <div class="space-y-6 max-w-2xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Identidade Visual da Igreja</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Faça o upload dos logotipos oficiais da sua igreja para sobreposição nas projeções públicas e transmissões.
                </p>
              </div>

              <!-- Nome da Igreja -->
              <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 space-y-2">
                <label for="church_name" class="block text-xs font-semibold text-white">Nome da Congregação / Ministério</label>
                <input
                  id="church_name"
                  type="text"
                  value={localBrand.name}
                  oninput={(e) => updateBrandField('name', (e.target as HTMLInputElement).value)}
                  placeholder="Ex: Igreja Batista Central"
                  class="w-full h-8 px-2.5 rounded bg-neutral-900 border border-neutral-700 text-xs text-white focus:outline-none focus:border-primary"
                />
              </div>

              <!-- Cards de Upload dos 4 formatos de Logotipo -->
              <div class="space-y-2">
                <span class="block text-xs font-semibold text-white">Arquivos de Logotipo</span>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  {#each [
                    { key: 'primary', label: 'Logo Principal', desc: 'Símbolo ou ícone padrão' },
                    { key: 'dark', label: 'Versão Escura', desc: 'Para fundos muito claros' },
                    { key: 'light', label: 'Versão Clara / Branca', desc: 'Para fundos escuros e projeção' },
                    { key: 'full', label: 'Logo Completo (Horizontal)', desc: 'Símbolo com nome por extenso' },
                  ] as variant}
                    <div class="p-3 rounded-xl border border-neutral-800 bg-neutral-900/40 flex flex-col justify-between gap-3">
                      <div class="flex items-center justify-between">
                        <div>
                          <div class="text-xs font-semibold text-white">{variant.label}</div>
                          <div class="text-[10px] text-neutral-400">{variant.desc}</div>
                        </div>
                        {#if localBrand[`logo_${variant.key}` as keyof ChurchBrandSettings]}
                          <Badge variant="outline" class="text-[9px] text-emerald-400 border-emerald-500/30">Ativo</Badge>
                        {/if}
                      </div>

                      <!-- Preview do Logo -->
                      <div class="h-20 w-full rounded-lg bg-neutral-950 border border-neutral-800 flex items-center justify-center p-2 relative overflow-hidden">
                        {#if localBrand[`logo_${variant.key}` as keyof ChurchBrandSettings]}
                          <img
                            src={String(localBrand[`logo_${variant.key}` as keyof ChurchBrandSettings])}
                            alt={variant.label}
                            class="max-h-full max-w-full object-contain"
                          />
                        {:else}
                          <div class="text-[10px] text-neutral-600 flex flex-col items-center gap-1">
                            <Church size={18} class="opacity-40" />
                            <span>Sem arquivo</span>
                          </div>
                        {/if}
                      </div>

                      <div class="flex items-center justify-between pt-1">
                        <label class="cursor-pointer">
                          <input
                            type="file"
                            accept="image/*"
                            class="hidden"
                            onchange={(e) => handleLogoUpload(variant.key as any, e)}
                          />
                          <span class="px-2.5 py-1 text-xs rounded bg-neutral-800 hover:bg-neutral-700 text-white font-medium flex items-center gap-1.5 transition-colors">
                            <Upload size={12} />
                            <span>{localBrand[`logo_${variant.key}` as keyof ChurchBrandSettings] ? 'Substituir...' : 'Enviar...'}</span>
                          </span>
                        </label>

                        {#if localBrand[`logo_${variant.key}` as keyof ChurchBrandSettings]}
                          <button
                            type="button"
                            onclick={() => updateBrandField(`logo_${variant.key}` as any, '')}
                            class="text-xs text-neutral-500 hover:text-red-400 transition-colors cursor-pointer"
                          >
                            Remover
                          </button>
                        {/if}
                      </div>

                      {#if uploadStatus[variant.key]}
                        <div class="text-[10px] text-primary">{uploadStatus[variant.key]}</div>
                      {/if}
                    </div>
                  {/each}
                </div>
              </div>

              <!-- Configuração de Exibição no Público -->
              <div class="pt-4 border-t border-neutral-800 space-y-4">
                <div>
                  <h4 class="text-xs font-semibold text-white">Sobreposição no Telão Público (Public Screen)</h4>
                  <p class="text-[11px] text-neutral-400 mt-0.5">
                    Defina se e onde o logotipo oficial da igreja ficará fixado durante a projeção.
                  </p>
                </div>

                <!-- Toggle de Exibir no Telão -->
                <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                  <div>
                    <div class="text-xs font-semibold text-white">Exibir Logotipo na Tela Pública</div>
                    <p class="text-[11px] text-neutral-400 mt-0.5">
                      Mantém o logo visível sobre os slides e vídeos da projeção.
                    </p>
                  </div>
                  <button
                    type="button"
                    aria-label="Alternar exibição de logotipo na tela pública"
                    onclick={() => updateBrandField('show_logo_on_public', !localBrand.show_logo_on_public)}
                    class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none {localBrand.show_logo_on_public ? 'bg-emerald-600' : 'bg-neutral-800'}"
                    role="switch"
                    aria-checked={localBrand.show_logo_on_public}
                  >
                    <span aria-hidden="true" class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {localBrand.show_logo_on_public ? 'translate-x-5' : 'translate-x-0'}"></span>
                  </button>
                </div>

                {#if localBrand.show_logo_on_public}
                  <!-- Variante Selecionada -->
                  <div class="space-y-1.5">
                    <span class="text-xs font-medium text-neutral-300">Variante a Exibir</span>
                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
                      {#each [
                        { key: 'primary', label: 'Principal' },
                        { key: 'light', label: 'Clara / Branca' },
                        { key: 'dark', label: 'Escura' },
                        { key: 'full', label: 'Completa' },
                      ] as opt}
                        <button
                          type="button"
                          onclick={() => updateBrandField('logo_variant', opt.key as any)}
                          class="p-2 rounded-lg border text-xs font-medium text-center transition-all cursor-pointer {localBrand.logo_variant === opt.key ? 'border-primary bg-primary/10 text-primary font-bold' : 'border-neutral-800 bg-neutral-900/30 text-neutral-400 hover:text-white'}"
                        >
                          {opt.label}
                        </button>
                      {/each}
                    </div>
                  </div>

                  <!-- Posição no Telão (4 Cantos) -->
                  <div class="space-y-1.5">
                    <span class="text-xs font-medium text-neutral-300">Posição no Slide</span>
                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
                      {#each [
                        { key: 'top-left', label: 'Superior Esquerdo' },
                        { key: 'top-right', label: 'Superior Direito' },
                        { key: 'bottom-left', label: 'Inferior Esquerdo' },
                        { key: 'bottom-right', label: 'Inferior Direito' },
                      ] as pos}
                        <button
                          type="button"
                          onclick={() => updateBrandField('logo_position', pos.key as any)}
                          class="p-2.5 rounded-lg border text-xs font-medium text-center transition-all cursor-pointer {localBrand.logo_position === pos.key ? 'border-primary bg-primary/10 text-primary font-bold' : 'border-neutral-800 bg-neutral-900/30 text-neutral-400 hover:text-white'}"
                        >
                          {pos.label}
                        </button>
                      {/each}
                    </div>
                  </div>

                  <!-- Tamanho e Opacidade -->
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
                    <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/30 space-y-2">
                      <div class="flex justify-between text-xs">
                        <span class="text-neutral-400">Tamanho Máximo</span>
                        <span class="font-mono text-white font-semibold">{localBrand.logo_size}px</span>
                      </div>
                      <input
                        type="range"
                        min="32"
                        max="240"
                        step="4"
                        value={localBrand.logo_size}
                        oninput={(e) => updateBrandField('logo_size', Number((e.target as HTMLInputElement).value))}
                        class="w-full accent-primary"
                      />
                    </div>

                    <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/30 space-y-2">
                      <div class="flex justify-between text-xs">
                        <span class="text-neutral-400">Opacidade</span>
                        <span class="font-mono text-white font-semibold">{localBrand.logo_opacity}%</span>
                      </div>
                      <input
                        type="range"
                        min="10"
                        max="100"
                        step="5"
                        value={localBrand.logo_opacity}
                        oninput={(e) => updateBrandField('logo_opacity', Number((e.target as HTMLInputElement).value))}
                        class="w-full accent-primary"
                      />
                    </div>
                  </div>

                  <!-- Margens em Relação às Bordas -->
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
                    <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/30 space-y-2">
                      <div class="flex justify-between text-xs">
                        <span class="text-neutral-400">Margem Horizontal</span>
                        <span class="font-mono text-white font-semibold">{localBrand.logo_margin_horizontal ?? 24}px</span>
                      </div>
                      <input
                        type="range"
                        min="0"
                        max="160"
                        step="4"
                        value={localBrand.logo_margin_horizontal ?? 24}
                        oninput={(e) => updateBrandField('logo_margin_horizontal', Number((e.target as HTMLInputElement).value))}
                        class="w-full accent-primary"
                      />
                    </div>

                    <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/30 space-y-2">
                      <div class="flex justify-between text-xs">
                        <span class="text-neutral-400">Margem Vertical</span>
                        <span class="font-mono text-white font-semibold">{localBrand.logo_margin_vertical ?? 24}px</span>
                      </div>
                      <input
                        type="range"
                        min="0"
                        max="160"
                        step="4"
                        value={localBrand.logo_margin_vertical ?? 24}
                        oninput={(e) => updateBrandField('logo_margin_vertical', Number((e.target as HTMLInputElement).value))}
                        class="w-full accent-primary"
                      />
                    </div>
                  </div>
                {/if}
              </div>
            </div>

          <!-- ABA: BÍBLIAS (VERSÕES SQLITE) -->
          {:else if activeTab === 'bibles'}
            <div class="space-y-6 w-full max-w-3xl">
              <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
                <div>
                  <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <BookOpen size={16} class="text-primary" />
                    Versões da Bíblia Sagrada
                  </h3>
                  <p class="text-xs text-muted-foreground mt-0.5">
                    Gerencie e importe versões da Bíblia em formato SQLite (.sqlite). Os arquivos são copiados automaticamente para a pasta local de dados.
                  </p>
                </div>

                <div class="flex items-center gap-2 shrink-0">
                  <input
                    type="file"
                    accept=".sqlite,.db"
                    bind:this={bibleFileInput}
                    onchange={handleBibleFileSelect}
                    class="hidden"
                  />
                  <Button
                    variant="outline"
                    size="sm"
                    class="text-xs gap-1.5"
                    onclick={() => (showPathImport = !showPathImport)}
                  >
                    Por Caminho...
                  </Button>
                  <Button
                    variant="default"
                    size="sm"
                    class="text-xs gap-1.5"
                    disabled={isImportingBible}
                    onclick={() => bibleFileInput?.click()}
                  >
                    <Upload size={13} />
                    {isImportingBible ? 'Importando...' : 'Importar Bíblia (.sqlite)'}
                  </Button>
                </div>
              </div>

              {#if bibleError}
                <div class="p-3 rounded-lg border border-red-500/40 bg-red-500/10 text-red-300 text-xs flex items-center justify-between gap-2">
                  <div class="flex items-center gap-2">
                    <AlertCircle size={15} class="shrink-0" />
                    <span>{bibleError}</span>
                  </div>
                  <button
                    type="button"
                    class="text-red-400 hover:text-white text-xs underline cursor-pointer"
                    onclick={() => (bibleError = '')}
                  >
                    Fechar
                  </button>
                </div>
              {/if}

              <!-- Formulário Opcional para Importação Direta por Caminho Local -->
              {#if showPathImport}
                <div class="p-4 rounded-xl border border-border bg-card/60 space-y-3">
                  <div class="flex items-center justify-between">
                    <h4 class="text-xs font-semibold text-foreground">Importar por Caminho de Arquivo Local</h4>
                    <button
                      type="button"
                      class="text-xs text-muted-foreground hover:text-foreground cursor-pointer"
                      onclick={() => (showPathImport = false)}
                    >
                      Cancelar
                    </button>
                  </div>
                  <div class="flex gap-2">
                    <input
                      type="text"
                      bind:value={importPathInput}
                      placeholder="/caminho/completo/para/biblia.sqlite"
                      class="flex-1 h-8 px-3 rounded-md bg-neutral-900 border border-neutral-700 text-xs text-foreground placeholder:text-neutral-500 focus:outline-none focus:border-primary"
                    />
                    <Button
                      variant="default"
                      size="sm"
                      class="text-xs"
                      disabled={isImportingBible}
                      onclick={handleImportBibleByPath}
                    >
                      {isImportingBible ? 'Copiando...' : 'Importar'}
                    </Button>
                  </div>
                  <p class="text-[11px] text-muted-foreground">
                    O arquivo SQLite será validado e copiado diretamente para a pasta de dados do OpenPresenter (<code class="text-neutral-300">bibles/</code>).
                  </p>
                </div>
              {/if}

              <!-- Lista de Bíblias Importadas -->
              <div class="space-y-3">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-semibold text-neutral-300 uppercase tracking-wider">
                    Versões Instaladas ({biblesList.length})
                  </span>
                  {#if isLoadingBibles}
                    <span class="text-[11px] text-muted-foreground animate-pulse">Carregando...</span>
                  {/if}
                </div>

                {#if biblesList.length === 0 && !isLoadingBibles}
                  <div class="p-8 rounded-xl border border-dashed border-border bg-neutral-900/20 text-center space-y-3">
                    <div class="w-10 h-10 rounded-full bg-primary/10 text-primary flex items-center justify-center mx-auto">
                      <BookOpen size={20} />
                    </div>
                    <div class="max-w-sm mx-auto">
                      <h4 class="text-xs font-semibold text-foreground">Nenhuma versão bíblica instalada</h4>
                      <p class="text-[11px] text-muted-foreground mt-1">
                        Importe um arquivo de Bíblia SQLite (.sqlite) para habilitar a pesquisa de passagens, projeção de versículos e controle de slides.
                      </p>
                    </div>
                    <Button
                      variant="outline"
                      size="sm"
                      class="text-xs gap-1.5"
                      onclick={() => bibleFileInput?.click()}
                    >
                      <Upload size={13} />
                      Selecionar Arquivo SQLite
                    </Button>
                  </div>
                {:else}
                  <div class="grid grid-cols-1 gap-2.5">
                    {#each biblesList as bible (bible.id)}
                      <div class="p-3.5 rounded-xl border border-border bg-neutral-900/40 hover:bg-neutral-900/70 transition-all flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
                        <div class="flex items-center gap-3 min-w-0">
                          <div class="w-9 h-9 rounded-lg bg-primary/15 border border-primary/25 text-primary flex items-center justify-center font-bold text-xs shrink-0 uppercase tracking-wider">
                            {bible.abbreviation || 'BIB'}
                          </div>

                          <div class="min-w-0">
                            <div class="flex items-center gap-2">
                              <span class="text-xs font-bold text-foreground truncate">{bible.name}</span>
                              <Badge variant="outline" class="text-[10px] uppercase font-mono py-0 px-1.5">
                                {bible.abbreviation}
                              </Badge>
                              {#if bible.is_default}
                                <span class="text-[10px] px-1.5 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 font-semibold border border-emerald-500/30 flex items-center gap-1">
                                  <Check size={10} />
                                  Padrão
                                </span>
                              {/if}
                            </div>
                            <div class="text-[11px] text-muted-foreground mt-0.5 flex items-center gap-2 font-mono">
                              <span>{bible.book_count} livros</span>
                              <span>•</span>
                              <span>{bible.verse_count.toLocaleString()} versículos</span>
                              <span>•</span>
                              <span class="truncate text-neutral-500 max-w-[200px]" title={bible.filename}>{bible.filename}</span>
                            </div>
                          </div>
                        </div>

                        <!-- Ações da Versão -->
                        <div class="flex items-center gap-1.5 shrink-0 self-end sm:self-auto">
                          {#if !bible.is_default}
                            <button
                              type="button"
                              onclick={() => handleSetDefaultBible(bible.id)}
                              class="px-2.5 py-1 rounded text-xs text-neutral-400 hover:text-emerald-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                              title="Definir como versão padrão para projeção rápida"
                            >
                              Tornar Padrão
                            </button>
                          {/if}

                          <button
                            type="button"
                            onclick={() => handleStartEditBible(bible)}
                            class="p-1.5 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
                            title="Renomear versão e abreviação"
                          >
                            <Edit3 size={14} />
                          </button>

                          <button
                            type="button"
                            onclick={() => handleDeleteBible(bible.id, bible.name)}
                            class="p-1.5 rounded text-neutral-400 hover:text-red-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                            title="Remover Bíblia do app"
                          >
                            <Trash2 size={14} />
                          </button>
                        </div>
                      </div>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>

            <!-- Modal Inline de Edição da Versão da Bíblia (Nome / Sigla / Padrão) -->
            {#if isEditingBible}
              <div
                class="fixed inset-0 z-[60] bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in"
                role="dialog"
                aria-modal="true"
                aria-label="Editar Versão da Bíblia"
              >
                <div class="w-full max-w-sm rounded-xl border border-neutral-700 bg-neutral-900 p-5 shadow-2xl space-y-4 text-foreground">
                  <div>
                    <h4 class="text-sm font-bold text-white">Editar Versão da Bíblia</h4>
                    <p class="text-xs text-neutral-400 mt-0.5">
                      Ajuste o nome de exibição e a sigla canônica da versão (ex: NVI, ARA, ARC, NVT).
                    </p>
                  </div>

                  <div class="space-y-3">
                    <div>
                      <label for="edit_bible_name" class="block text-xs font-semibold text-neutral-300 mb-1">
                        Nome da Versão
                      </label>
                      <input
                        id="edit_bible_name"
                        type="text"
                        bind:value={editBibleName}
                        placeholder="Ex: Almeida Revista e Atualizada"
                        class="w-full h-8 px-2.5 rounded bg-neutral-950 border border-neutral-700 text-xs text-white focus:outline-none focus:border-primary"
                      />
                    </div>

                    <div>
                      <label for="edit_bible_abbr" class="block text-xs font-semibold text-neutral-300 mb-1">
                        Sigla / Abreviação
                      </label>
                      <input
                        id="edit_bible_abbr"
                        type="text"
                        bind:value={editBibleAbbr}
                        placeholder="Ex: ARA"
                        maxlength="10"
                        class="w-full h-8 px-2.5 rounded bg-neutral-950 border border-neutral-700 text-xs text-white uppercase font-mono focus:outline-none focus:border-primary"
                      />
                    </div>

                    <label class="flex items-center gap-2 cursor-pointer pt-1">
                      <input
                        type="checkbox"
                        bind:checked={editBibleIsDefault}
                        class="rounded border-neutral-700 text-primary focus:ring-0 accent-primary"
                      />
                      <span class="text-xs text-neutral-300">Definir como versão padrão</span>
                    </label>
                  </div>

                  <div class="flex items-center justify-end gap-2 pt-2 border-t border-neutral-800">
                    <Button
                      variant="ghost"
                      size="sm"
                      class="text-xs"
                      onclick={() => {
                        isEditingBible = false;
                        editingBibleId = null;
                      }}
                    >
                      Cancelar
                    </Button>
                    <Button
                      variant="default"
                      size="sm"
                      class="text-xs"
                      onclick={handleSaveEditBible}
                    >
                      Salvar Alterações
                    </Button>
                  </div>
                </div>
              </div>
            {/if}

          <!-- ABA 2: ARQUIVOS E PASTAS (Mapeamento Zero-Copy e Pastas do Sistema) -->
          {:else if activeTab === 'files'}
            <div class="space-y-6 w-full max-w-3xl">
              <div>
                <h3 class="text-sm font-semibold text-foreground">Arquivos e Pastas do Sistema</h3>
                <p class="text-xs text-muted-foreground mt-0.5">
                  Estrutura de dados local do OpenPresenter e integração com diretórios externos de fotos e vídeos.
                </p>
              </div>

              <!-- Input invisível de fallback para picker de pasta no navegador -->
              <input
                type="file"
                bind:this={folderPickerInput}
                onchange={handleHtmlFolderSelected}
                webkitdirectory
                class="hidden"
              />

              <!-- Pasta Raiz de Dados do Sistema em Destaque -->
              {#if systemPaths}
                <div class="p-4 rounded-xl border border-primary/30 bg-primary/5 space-y-3">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2.5">
                      <div class="w-8 h-8 rounded-lg bg-primary/20 text-primary flex items-center justify-center">
                        <HardDrive size={18} />
                      </div>
                      <div>
                        <h4 class="text-xs font-bold text-foreground">Pasta de Dados do Sistema (Data Dir)</h4>
                        <p class="text-[11px] text-muted-foreground">Onde ficam salvos o banco de dados SQLite, arquivos e mídias do usuário</p>
                      </div>
                    </div>
                    <Badge variant="outline" class="text-[10px] text-primary border-primary/40 font-mono">
                      data_dir
                    </Badge>
                  </div>

                  <div class="p-2.5 rounded-lg bg-background border border-border flex items-center justify-between gap-3">
                    <code class="text-xs font-mono text-foreground break-all select-all flex-1">
                      {systemPaths.data_dir}
                    </code>

                    <div class="flex items-center gap-1.5 shrink-0">
                      <Button
                        variant="outline"
                        size="sm"
                        onclick={() => handleCopyText(systemPaths?.data_dir || '')}
                        class="h-7 text-xs gap-1"
                        title="Copiar caminho da pasta de dados"
                      >
                        {#if isCopiedDataDir}
                          <Check size={12} class="text-emerald-400" />
                          <span class="text-emerald-400">Copiado</span>
                        {:else}
                          <Copy size={12} />
                          <span>Copiar</span>
                        {/if}
                      </Button>

                      <Button
                        variant="outline"
                        size="sm"
                        onclick={() => handleOpenInFinder(systemPaths?.data_dir || '')}
                        class="h-7 text-xs gap-1"
                        title="Abrir pasta no Finder / Explorador de Arquivos"
                      >
                        <FolderOpen size={12} />
                        <span>Abrir</span>
                      </Button>
                    </div>
                  </div>
                </div>

                <!-- Subdiretórios Padrão do Sistema -->
                <div class="space-y-2.5">
                  <span class="text-xs font-semibold text-foreground uppercase tracking-wider block">
                    Subdiretórios da Aplicação
                  </span>
                  <div class="grid grid-cols-1 gap-2 text-xs">
                    <div class="p-2.5 rounded-lg border border-border bg-card/60 flex items-center justify-between gap-3">
                      <div class="min-w-0 flex-1">
                        <div class="font-semibold text-foreground">Banco de Dados SQLite</div>
                        <div class="text-[11px] text-muted-foreground font-mono mt-0.5 truncate" title={systemPaths.db_path}>
                          {systemPaths.db_path}
                        </div>
                      </div>
                      <div class="flex items-center gap-2 shrink-0">
                        <Badge variant="outline" class="text-[10px]">openpresenter.db</Badge>
                        <button
                          type="button"
                          onclick={() => handleOpenInFinder(systemPaths?.data_dir || '')}
                          class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                          title="Abrir no Finder"
                        >
                          <FolderOpen size={13} />
                        </button>
                      </div>
                    </div>

                    <div class="p-2.5 rounded-lg border border-border bg-card/60 flex items-center justify-between gap-3">
                      <div class="min-w-0 flex-1">
                        <div class="font-semibold text-foreground">Imagens do Aplicativo</div>
                        <div class="text-[11px] text-muted-foreground font-mono mt-0.5 truncate" title={systemPaths.images_path}>
                          {systemPaths.images_path}
                        </div>
                      </div>
                      <div class="flex items-center gap-2 shrink-0">
                        <Badge variant="outline" class="text-[10px]">media/images</Badge>
                        <button
                          type="button"
                          onclick={() => handleOpenInFinder(systemPaths?.images_path || '')}
                          class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                          title="Abrir no Finder"
                        >
                          <FolderOpen size={13} />
                        </button>
                      </div>
                    </div>

                    <div class="p-2.5 rounded-lg border border-border bg-card/60 flex items-center justify-between gap-3">
                      <div class="min-w-0 flex-1">
                        <div class="font-semibold text-foreground">Papéis de Parede</div>
                        <div class="text-[11px] text-muted-foreground font-mono mt-0.5 truncate" title={systemPaths.wallpapers_path}>
                          {systemPaths.wallpapers_path}
                        </div>
                      </div>
                      <div class="flex items-center gap-2 shrink-0">
                        <Badge variant="outline" class="text-[10px]">media/wallpapers</Badge>
                        <button
                          type="button"
                          onclick={() => handleOpenInFinder(systemPaths?.wallpapers_path || '')}
                          class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
                          title="Abrir no Finder"
                        >
                          <FolderOpen size={13} />
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              {/if}

              <!-- Pastas Externas Mapeadas (Zero-Copy) -->
              <div class="pt-4 border-t border-border space-y-3">
                <div>
                  <h3 class="text-sm font-semibold text-foreground">Pastas de Mídia Importadas</h3>
                  <p class="text-xs text-muted-foreground mt-0.5">
                    Adicione pastas do seu computador (ex.: fotos do culto, avisos). O OpenPresenter lê diretamente sem duplicar espaço em disco.
                  </p>
                </div>

                <!-- Formulário para Mapear Nova Pasta -->
                <div class="p-3.5 rounded-xl border border-border bg-card/60 space-y-3">
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
                    <div>
                      <label for="folder-name-input" class="block text-[11px] text-muted-foreground mb-1">Rótulo da Pasta</label>
                      <input
                        id="folder-name-input"
                        type="text"
                        bind:value={newFolderName}
                        placeholder="Ex: Fotos do Culto de Domingo"
                        class="w-full h-8 px-2.5 rounded bg-background border border-border text-xs text-foreground placeholder-muted-foreground focus:outline-none focus:border-primary"
                      />
                    </div>
                    <div>
                      <label for="folder-path-input" class="block text-[11px] text-muted-foreground mb-1">Caminho no Computador *</label>
                      <div class="flex items-center gap-1.5">
                        <input
                          id="folder-path-input"
                          type="text"
                          bind:value={newFolderPath}
                          placeholder="Ex: /Users/usuario/Imagens/Culto"
                          class="flex-1 h-8 px-2.5 rounded bg-background border border-border text-xs text-foreground placeholder-muted-foreground focus:outline-none focus:border-primary font-mono text-[11px]"
                        />
                        <Button
                          variant="outline"
                          size="sm"
                          onclick={handleNativePickFolder}
                          class="h-8 text-xs gap-1.5 shrink-0"
                          title="Selecionar Pasta graficamente no computador"
                        >
                          <Folder size={13} class="text-primary" />
                          <span>Selecionar...</span>
                        </Button>
                      </div>
                    </div>
                  </div>

                  {#if folderError}
                    <div class="p-2 rounded bg-destructive/10 border border-destructive/30 text-destructive text-xs flex items-center gap-1.5">
                      <AlertCircle size={13} class="shrink-0" />
                      <span>{folderError}</span>
                    </div>
                  {/if}

                  <div class="flex justify-end">
                    <Button
                      variant="default"
                      size="sm"
                      onclick={handleAddFolder}
                      disabled={isAddingFolder}
                      class="gap-1.5 text-xs font-semibold"
                    >
                      <FolderPlus size={13} />
                      <span>{isAddingFolder ? 'Validando...' : 'Mapear Pasta'}</span>
                    </Button>
                  </div>
                </div>

                <!-- Lista de Pastas Mapeadas -->
                <div class="space-y-2">
                  {#if mediaFolders.length === 0}
                    <div class="p-4 rounded-lg border border-dashed border-border text-center text-xs text-muted-foreground">
                      Nenhuma pasta externa mapeada ainda. Clique em "Selecionar..." para importar sua primeira pasta.
                    </div>
                  {:else}
                    {#each mediaFolders as folder (folder.id)}
                      <div class="p-3 rounded-lg border border-border bg-card/60 flex items-center justify-between gap-3">
                        <div class="flex items-center gap-2.5 min-w-0 flex-1">
                          <Folder size={16} class="text-primary shrink-0" />
                          <div class="min-w-0 flex-1">
                            <div class="text-xs font-semibold text-foreground truncate">{folder.name}</div>
                            <div class="text-[10px] text-muted-foreground font-mono truncate" title={folder.path}>{folder.path}</div>
                          </div>
                        </div>

                        <div class="flex items-center gap-1.5 shrink-0">
                          <Button
                            variant="outline"
                            size="sm"
                            onclick={() => handleOpenInFinder(folder.path)}
                            class="h-7 text-[11px] gap-1 px-2"
                            title="Abrir no Finder"
                          >
                            <FolderOpen size={12} />
                            <span>Abrir</span>
                          </Button>

                          <button
                            type="button"
                            onclick={() => handleDeleteFolder(folder.id)}
                            title="Remover pasta mapeada"
                            class="w-7 h-7 rounded text-muted-foreground hover:text-destructive hover:bg-destructive/10 flex items-center justify-center transition-colors cursor-pointer"
                          >
                            <Trash2 size={13} />
                          </button>
                        </div>
                      </div>
                    {/each}
                  {/if}
                </div>
              </div>
            </div>

          <!-- ABA 3: PARTES DA MÚSICA E CORES -->
          {:else if activeTab === 'sections'}
            <div class="space-y-5 max-w-2xl">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-sm font-semibold text-white">Partes da Música e Atalhos</h3>
                  <p class="text-xs text-neutral-400 mt-0.5">
                    Personalize o rótulo, a cor de destaque e a tecla de atalho de cada parte da música.
                  </p>
                </div>

                <Button variant="outline" size="sm" onclick={handleResetSections} class="text-xs gap-1.5">
                  <RotateCcw size={13} />
                  Restaurar Padrões
                </Button>
              </div>

              <!-- Lista de Partes Configuráveis -->
              <div class="space-y-2.5">
                {#each editableSections as section, idx (section.id)}
                  <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/40 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                    <div class="flex items-center gap-3">
                      <!-- Amostra Visual da Seção -->
                      <div class="px-2.5 py-1 rounded text-xs font-semibold border flex items-center gap-1.5 {section.bg} {section.border} {section.text} min-w-[90px] justify-center">
                        <span class="w-2 h-2 rounded-full {section.accent}"></span>
                        <span>{section.name}</span>
                      </div>

                      <!-- Nome Editável -->
                      <input
                        type="text"
                        value={section.name}
                        oninput={(e) => handleNameChange(idx, (e.target as HTMLInputElement).value)}
                        class="h-7 w-28 px-2 rounded bg-neutral-900 border border-neutral-700 text-xs text-white focus:outline-none focus:border-primary"
                        title="Nome da parte"
                      />
                    </div>

                    <div class="flex items-center gap-3">
                      <!-- Atalho de Tecla -->
                      <div class="flex items-center gap-1.5">
                        <span class="text-[11px] text-neutral-400">Atalho:</span>
                        <input
                          type="text"
                          maxlength="1"
                          value={section.shortcut.toUpperCase()}
                          oninput={(e) => handleShortcutChange(idx, (e.target as HTMLInputElement).value)}
                          class="h-7 w-9 text-center font-mono font-bold uppercase rounded bg-neutral-900 border border-neutral-700 text-xs text-primary focus:outline-none focus:border-primary"
                          title="Pressione esta letra para ir diretamente a esta parte durante a música"
                        />
                      </div>

                      <!-- Seletor de Paleta de Cores -->
                      <div class="flex items-center gap-1">
                        {#each COLOR_PRESETS as preset}
                          <button
                            type="button"
                            title={preset.name}
                            onclick={() => handleColorChange(idx, preset.id)}
                            class="w-5 h-5 rounded-full {preset.accent} transition-transform hover:scale-110 flex items-center justify-center cursor-pointer {section.colorName === preset.name ? 'ring-2 ring-white ring-offset-1 ring-offset-neutral-950 scale-105' : 'opacity-70'}"
                          >
                            {#if section.colorName === preset.name}
                              <Check size={11} class="text-black font-bold" />
                            {/if}
                          </button>
                        {/each}
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            </div>

          <!-- ABA 4: TELAS E SAÍDAS -->
          {:else if activeTab === 'outputs'}
            <div class="space-y-6 max-w-xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Saídas Gráficas</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Gerencie as telas de saída do projetor e do retorno de palco conforme a especificação multi-telas.
                </p>
              </div>

              <div class="space-y-3.5">
                <!-- Saída Pública -->
                <div class="p-4 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 flex items-center justify-center">
                      <Monitor size={18} />
                    </div>
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        Tela Pública (Public)
                        <Badge variant="outline" class="text-[9px] text-emerald-400 border-emerald-500/30">16:9 Nativo</Badge>
                      </div>
                      <div class="text-[11px] text-neutral-400 mt-0.5">
                        Projetada para a congregação e captura via OBS em <code>/public</code>
                      </div>
                    </div>
                  </div>

                  <Button
                    variant="outline"
                    size="sm"
                    onclick={() => window.open('/public', 'OpenPresenterPublic', 'width=1280,height=720')}
                    class="text-xs gap-1.5"
                  >
                    <span>Abrir</span>
                    <ExternalLink size={12} />
                  </Button>
                </div>

                <!-- Saída do Palco (Stage) -->
                <div class="p-4 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 flex items-center justify-center">
                      <Tv size={18} />
                    </div>
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        Retorno do Palco (Stage)
                        <Badge variant="outline" class="text-[9px] text-emerald-400 border-emerald-500/30">Relógio + Prévia</Badge>
                      </div>
                      <div class="text-[11px] text-neutral-400 mt-0.5">
                        Exibe o slide atual, próximo slide e relógio digital em <code>/stage</code>
                      </div>
                    </div>
                  </div>

                  <Button
                    variant="outline"
                    size="sm"
                    onclick={() => window.open('/stage', 'OpenPresenterStage', 'width=1280,height=720')}
                    class="text-xs gap-1.5"
                  >
                    <span>Abrir</span>
                    <ExternalLink size={12} />
                  </Button>
                </div>
              </div>

              <!-- Ajuste de Papel de Parede Padrão -->
              <div class="pt-4 border-t border-neutral-800 space-y-3">
                <div>
                  <h3 class="text-sm font-semibold text-white">Ajuste de Papel de Parede</h3>
                  <p class="text-xs text-neutral-400 mt-0.5">
                    Modo padrão de renderização do papel de parede quando nenhuma música estiver sendo projetada.
                  </p>
                </div>

                <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
                  {#each [
                    { id: 'cover', label: 'Preencher', desc: 'Preenche toda a tela 16:9' },
                    { id: 'contain', label: 'Ajustar', desc: 'Preserva proporção sem cortes' },
                    { id: 'center', label: 'Centralizar', desc: 'Exibe no centro sem zoom' },
                    { id: 'repeat', label: 'Repetir', desc: 'Lado a lado em mosaico' },
                  ] as mode}
                    <button
                      type="button"
                      onclick={() => setWallpaperFit(mode.id as any)}
                      class="p-2.5 rounded-lg border text-left transition-all cursor-pointer {$wallpaperFitStore === mode.id ? 'border-primary bg-primary/10 ring-1 ring-primary/40 text-primary font-semibold' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900 text-neutral-300'}"
                    >
                      <div class="text-xs">{mode.label}</div>
                      <div class="text-[10px] text-neutral-400 font-normal mt-0.5">{mode.desc}</div>
                    </button>
                  {/each}
                </div>
              </div>
            </div>

          <!-- ABA 5: ATALHOS RÁPIDOS -->
          {:else if activeTab === 'shortcuts'}
            <div class="space-y-4 max-w-xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Atalhos Operacionais de Teclado</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Agilidade durante o culto e eventos ao vivo sem tirar as mãos do teclado.
                </p>
              </div>

              <div class="rounded-xl border border-neutral-800 bg-neutral-900/30 divide-y divide-neutral-800/80 text-xs">
                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Apresentar slide selecionado</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-emerald-400">Enter</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Parar apresentação ao vivo</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-red-400">Esc</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Avançar para o próximo slide</span>
                  <div class="flex gap-1">
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">→</kbd>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">Espaço</kbd>
                  </div>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Retornar ao slide anterior</span>
                  <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">←</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Pular direto para slide ordinal</span>
                  <span class="text-neutral-400"><kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">[Número]</kbd> + <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">Enter</kbd></span>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Pular para o Refrão</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-emerald-400">R</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Pular para o Verso</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-blue-400">V</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Pular para a Ponte</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-amber-400">P</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Pular para a Introdução</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-cyan-400">I</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Pular para o Final / Encerramento</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-purple-400">F</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Abrir Paleta de Comandos</span>
                  <div class="flex gap-1">
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">⌘K</kbd>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">Ctrl+K</kbd>
                  </div>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Blackout (tela preta total)</span>
                  <div class="flex gap-1.5 items-center">
                    <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-primary font-bold">.</kbd>
                    <span class="text-[10px] text-neutral-500">ou</span>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">Alt+B</kbd>
                    <span class="text-[10px] text-neutral-500">ou</span>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">F1</kbd>
                  </div>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Limpar Texto (mantém o fundo)</span>
                  <div class="flex gap-1.5 items-center">
                    <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-primary font-bold">,</kbd>
                    <span class="text-[10px] text-neutral-500">ou</span>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">Alt+C</kbd>
                    <span class="text-[10px] text-neutral-500">ou</span>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">F2</kbd>
                  </div>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Papel de Parede (Tela de Repouso)</span>
                  <div class="flex gap-1.5 items-center">
                    <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px] text-primary font-bold">/</kbd>
                    <span class="text-[10px] text-neutral-500">ou</span>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">Alt+W</kbd>
                    <span class="text-[10px] text-neutral-500">ou</span>
                    <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[10px]">F3</kbd>
                  </div>
                </div>
              </div>
            </div>

          <!-- ABA 6: SOBRE & UPDATES -->
          {:else if activeTab === 'about'}
            <div class="space-y-6 max-w-xl">
              <!-- Banner da Marca -->
              <div class="p-6 rounded-xl border border-neutral-800 bg-neutral-900/40 text-center space-y-3">
                <img
                  src="/branding/open-presenter-wordmark-dark.svg"
                  alt="OpenPresenter"
                  class="h-10 mx-auto"
                />
                <p class="text-xs text-neutral-400">
                  Sistema moderno e aberto de projeção para igrejas e eventos.
                </p>
                <div class="flex items-center justify-center gap-2 pt-1 flex-wrap">
                  <Badge variant="secondary" class="text-[10px] font-mono">
                    v{updateInfo?.current_version || '0.2.0'}
                  </Badge>
                  <Badge variant="outline" class="text-[10px] text-primary border-primary/40">GPL-3.0</Badge>
                  <Badge variant="outline" class="text-[10px] text-neutral-400">Rust · Axum · Svelte · SQLite</Badge>
                </div>
              </div>

              <!-- Seção do Auto-Updater -->
              <div class="p-5 rounded-xl border border-neutral-800 bg-neutral-900/30 space-y-4">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2">
                    <Sparkles size={16} class="text-primary" />
                    <h4 class="text-xs font-semibold text-white">Atualizações do Sistema</h4>
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    class="h-7 text-xs gap-1.5"
                    onclick={handleCheckForUpdates}
                    disabled={isCheckingUpdate}
                  >
                    <RefreshCw size={12} class={isCheckingUpdate ? 'animate-spin' : ''} />
                    <span>{isCheckingUpdate ? 'Verificando...' : 'Verificar Atualizações'}</span>
                  </Button>
                </div>

                {#if updaterError}
                  <div class="p-3 rounded-lg bg-rose-950/40 border border-rose-600/40 text-rose-300 text-xs flex items-start gap-2">
                    <AlertTriangle size={15} class="shrink-0 mt-0.5 text-rose-400" />
                    <span>{updaterError}</span>
                  </div>
                {/if}

                {#if updaterStatus?.stage?.type === 'ReadyToRestart'}
                  <div class="p-4 rounded-xl bg-emerald-950/40 border border-emerald-600/60 text-emerald-200 text-xs space-y-2">
                    <div class="flex items-center gap-2 font-semibold text-emerald-300">
                      <CheckCircle2 size={16} class="text-emerald-400" />
                      <span>Atualização Instalada com Sucesso!</span>
                    </div>
                    <p class="text-[11px] text-emerald-300/80">
                      A nova versão foi gravada no executável. Reinicie o aplicativo OpenPresenter para aplicar todas as novidades.
                    </p>
                  </div>
                {:else if updaterStatus?.stage?.type === 'Downloading'}
                  <div class="p-4 rounded-xl border border-neutral-800 bg-neutral-950/60 space-y-3">
                    <div class="flex items-center justify-between text-xs">
                      <span class="text-neutral-300 font-medium flex items-center gap-1.5">
                        <Download size={14} class="animate-pulse text-primary transition-transform duration-300" />
                        Baixando nova versão...
                      </span>
                      <span class="font-mono text-primary font-bold">
                        {updaterStatus.stage.percent}%
                      </span>
                    </div>
                    <div class="w-full h-2 rounded-full bg-neutral-800 overflow-hidden">
                      <div
                        class="h-full bg-primary transition-all duration-200 rounded-full"
                        style="width: {updaterStatus.stage.percent}%"
                      ></div>
                    </div>
                    <div class="text-[10px] text-neutral-500 font-mono text-right">
                      {(updaterStatus.stage.downloaded_bytes / (1024 * 1024)).toFixed(1)} MB / {(updaterStatus.stage.total_bytes / (1024 * 1024)).toFixed(1)} MB
                    </div>
                  </div>
                {:else if updaterStatus?.stage?.type === 'Extracting'}
                  <div class="p-4 rounded-xl border border-neutral-800 bg-neutral-950/60 flex items-center gap-3 text-xs text-neutral-300">
                    <RefreshCw size={16} class="animate-spin text-primary shrink-0" />
                    <span>Extraindo arquivos e aplicando a substituição atômica no executável...</span>
                  </div>
                {:else if updaterStatus?.stage?.type === 'Error'}
                  <div class="p-4 rounded-xl bg-rose-950/30 border border-rose-600/40 text-xs text-rose-300 space-y-2">
                    <div class="flex items-center gap-2 font-semibold">
                      <AlertTriangle size={15} class="text-rose-400" />
                      <span>Erro ao aplicar atualização</span>
                    </div>
                    <p class="text-[11px] text-rose-200/80">{updaterStatus.stage.message}</p>
                    <Button variant="outline" size="sm" class="h-7 text-xs" onclick={handleStartUpdate}>
                      Tentar Novamente
                    </Button>
                  </div>
                {:else if updateInfo?.update_available}
                  <!-- Banner de Atualização Disponível -->
                  <div class="p-4 rounded-xl border border-emerald-600/40 bg-emerald-950/20 space-y-3">
                    <div class="flex items-center justify-between">
                      <div class="flex items-center gap-2">
                        <Badge variant="default" class="bg-emerald-600 text-white hover:bg-emerald-600 text-[10px] font-bold">
                          NOVO
                        </Badge>
                        <span class="text-xs font-bold text-white">Versão {updateInfo.latest_version} disponível</span>
                      </div>
                      {#if updateInfo.published_at}
                        <span class="text-[10px] text-neutral-400">
                          {new Date(updateInfo.published_at).toLocaleDateString('pt-BR')}
                        </span>
                      {/if}
                    </div>

                    <div class="text-xs text-neutral-200 font-medium">
                      {updateInfo.release_name}
                    </div>

                    {#if updateInfo.release_notes}
                      <div class="p-2.5 rounded-lg bg-neutral-950/70 border border-neutral-800/80 max-h-36 overflow-y-auto text-[11px] text-neutral-300 font-mono whitespace-pre-wrap leading-relaxed">
                        {updateInfo.release_notes}
                      </div>
                    {/if}

                    <div class="flex items-center justify-between pt-1 text-[11px] text-neutral-400">
                      <span>Arquivo: {updateInfo.asset_name || 'Pacote multiplataforma'}</span>
                      {#if updateInfo.asset_size}
                        <span>{(updateInfo.asset_size / (1024 * 1024)).toFixed(1)} MB</span>
                      {/if}
                    </div>

                    <div class="pt-2 flex items-center justify-end gap-2">
                      <Button
                        variant="default"
                        size="sm"
                        class="gap-1.5 font-semibold bg-emerald-600 hover:bg-emerald-500 text-white"
                        onclick={handleStartUpdate}
                        disabled={!updateInfo.download_url}
                      >
                        <Download size={14} />
                        <span>Baixar e Atualizar Agora</span>
                      </Button>
                    </div>
                  </div>
                {:else if updateInfo && !updateInfo.update_available}
                  <div class="p-3.5 rounded-lg border border-neutral-800 bg-neutral-950/40 flex items-center gap-2.5 text-xs text-neutral-300">
                    <CheckCircle2 size={16} class="text-emerald-400 shrink-0" />
                    <span>O OpenPresenter está na versão mais recente (v{updateInfo.current_version}).</span>
                  </div>
                {:else}
                  <div class="p-3.5 rounded-lg border border-neutral-800 bg-neutral-950/40 text-xs text-neutral-400 flex items-center justify-between">
                    <span>Nenhuma verificação de atualização realizada nesta sessão.</span>
                    <Button variant="ghost" size="sm" class="h-6 text-xs" onclick={handleCheckForUpdates}>
                      Checar agora
                    </Button>
                  </div>
                {/if}
              </div>

              <!-- Informações adicionais -->
              <div class="text-[11px] text-neutral-500 space-y-1.5 text-center">
                <p>Repositório de código aberto: <a href="https://github.com/open-mission/openpresenter" target="_blank" rel="noreferrer" class="text-primary hover:underline">github.com/open-mission/openpresenter</a></p>
                <p>Todos os seus dados (banco SQLite, mídias e bíblias) permanecem preservados em qualquer atualização.</p>
              </div>
            </div>
          {/if}
        </main>
      </div>

      <!-- Rodapé do Modal -->
      <div class="h-14 px-6 border-t border-neutral-800 bg-neutral-900/60 flex items-center justify-between shrink-0 text-xs text-neutral-500">
        <div class="flex items-center gap-3">
          <span>Pressione <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 font-mono text-[10px]">Esc</kbd> para fechar</span>
          {#if saveStatusMessage}
            <span class="text-xs text-emerald-400 font-medium flex items-center gap-1.5 animate-in fade-in">
              <Check size={14} class="text-emerald-400" />
              {saveStatusMessage}
            </span>
          {/if}
        </div>

        <div class="flex items-center gap-2">
          <Button variant="outline" size="sm" onclick={onclose}>
            Fechar
          </Button>
          <Button variant="default" size="sm" onclick={handleSaveAllSettings} disabled={isSaving} class="gap-1.5 font-semibold">
            {#if isSaving}
              <span>Salvando...</span>
            {:else}
              <Save size={14} />
              <span>Salvar Alterações</span>
            {/if}
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}

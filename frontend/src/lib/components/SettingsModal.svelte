<script lang="ts">
  import { onMount } from 'svelte';
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
  } from 'lucide-svelte';
  import {
    themeStore,
    baseColorStore,
    accentThemeStore,
    sectionsStore,
    wallpaperFitStore,
    setTheme,
    setBaseColor,
    setAccentTheme,
    setWallpaperFit,
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
    savePublicDisplaySettings,
    saveChurchBrandSettings,
    saveStageDisplaySettings,
    DEFAULT_PUBLIC_SETTINGS,
    DEFAULT_CHURCH_BRAND,
    DEFAULT_STAGE_SETTINGS,
  } from '../displaySettings';
  import Badge from './ui/Badge.svelte';
  import Button from './ui/Button.svelte';
  import type {
    MediaFolder,
    SystemPaths,
    PublicDisplaySettings,
    ChurchBrandSettings,
    StageDisplaySettings,
    StagePreset,
  } from '../types';

  export type SettingsTab =
    | 'appearance'
    | 'public'
    | 'stage'
    | 'church'
    | 'files'
    | 'sections'
    | 'outputs'
    | 'shortcuts'
    | 'about';

  interface Props {
    open: boolean;
    initialTab?: SettingsTab;
    onclose: () => void;
  }

  let { open = false, initialTab = 'appearance', onclose }: Props = $props();

  let activeTab = $state<SettingsTab>('appearance');
  let editableSections = $state<CustomSectionConfig[]>([]);

  // Configurações de Telas e Marca da Igreja
  let localPublic = $state<PublicDisplaySettings>({ ...DEFAULT_PUBLIC_SETTINGS });
  let localBrand = $state<ChurchBrandSettings>({ ...DEFAULT_CHURCH_BRAND });
  let localStage = $state<StageDisplaySettings>({ ...DEFAULT_STAGE_SETTINGS });
  let uploadStatus = $state<{ [key: string]: string }>({});

  // Estado da Aba Arquivos e Pastas
  let systemPaths = $state<SystemPaths | null>(null);
  let mediaFolders = $state<MediaFolder[]>([]);
  let newFolderName = $state('');
  let newFolderPath = $state('');
  let folderError = $state('');
  let isAddingFolder = $state(false);

  // Dropdown open states for Shadcn style selects
  let isBaseColorMenuOpen = $state(false);
  let isAccentThemeMenuOpen = $state(false);

  // Carrega configurações ao abrir
  $effect(() => {
    if (open) {
      activeTab = initialTab;
      editableSections = JSON.parse(JSON.stringify($sectionsStore));
      localPublic = JSON.parse(JSON.stringify($publicSettingsStore));
      localBrand = JSON.parse(JSON.stringify($churchBrandStore));
      localStage = JSON.parse(JSON.stringify($stageSettingsStore));
      loadSystemPaths();
      loadMediaFolders();
    }
  });

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
    localPublic[key] = val;
    savePublicDisplaySettings(localPublic);
  }

  function updateBrandField<K extends keyof ChurchBrandSettings>(key: K, val: ChurchBrandSettings[K]) {
    localBrand[key] = val;
    saveChurchBrandSettings(localBrand);
  }

  function updateStageField<K extends keyof StageDisplaySettings>(key: K, val: StageDisplaySettings[K]) {
    localStage[key] = val;
    saveStageDisplaySettings(localStage);
  }

  function handleResetStageColors() {
    localStage = {
      ...localStage,
      current_text_color: DEFAULT_STAGE_SETTINGS.current_text_color,
      next_text_color: DEFAULT_STAGE_SETTINGS.next_text_color,
      current_border_color: DEFAULT_STAGE_SETTINGS.current_border_color,
      next_border_color: DEFAULT_STAGE_SETTINGS.next_border_color,
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
            onclick={() => (activeTab = 'stage')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'stage' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Tv size={15} class={activeTab === 'stage' ? 'text-primary' : 'text-neutral-500'} />
            <span>Monitor de Palco (Stage)</span>
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
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'about' ? 'bg-neutral-800 text-primary font-semibold' : 'text-neutral-400 hover:text-neutral-200 hover:bg-neutral-900'}"
          >
            <Info size={15} class={activeTab === 'about' ? 'text-primary' : 'text-neutral-500'} />
            <span>Sobre o App</span>
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
            </div>

          <!-- ABA: TELA PÚBLICA (PUBLIC) -->
          {:else if activeTab === 'public'}
            <div class="space-y-6 max-w-2xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Configuração da Projeção Pública</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Ajuste o comportamento do texto, aproveitamento de espaço e margens da projeção para a congregação.
                </p>
              </div>

              <!-- Switches de Opções -->
              <div class="space-y-3">
                <!-- Preenchimento Máximo -->
                <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-4">
                  <div>
                    <div class="text-xs font-semibold text-white flex items-center gap-2">
                      <Maximize2 size={14} class="text-primary" />
                      Preencher o Máximo de Espaço
                    </div>
                    <p class="text-[11px] text-neutral-400 mt-0.5">
                      Amplia os textos para preencher a tela, garantindo alta legibilidade até mesmo para fundos de auditórios.
                    </p>
                  </div>
                  <button
                    type="button"
                    aria-label="Alternar preenchimento máximo de espaço"
                    onclick={() => updatePublicField('fill_space', !localPublic.fill_space)}
                    class="w-11 h-6 rounded-full transition-colors relative cursor-pointer {localPublic.fill_space ? 'bg-primary' : 'bg-neutral-800'}"
                    role="switch"
                    aria-checked={localPublic.fill_space}
                  >
                    <span class="w-4 h-4 rounded-full bg-white absolute top-1 transition-transform {localPublic.fill_space ? 'left-6' : 'left-1'}"></span>
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
                      Se desativado, o texto não quebra automaticamente e preserva cada verso da estrofe em sua linha original.
                    </p>
                  </div>
                  <button
                    type="button"
                    aria-label="Alternar quebra de linhas longas"
                    onclick={() => updatePublicField('word_wrap', !localPublic.word_wrap)}
                    class="w-11 h-6 rounded-full transition-colors relative cursor-pointer {localPublic.word_wrap ? 'bg-primary' : 'bg-neutral-800'}"
                    role="switch"
                    aria-checked={localPublic.word_wrap}
                  >
                    <span class="w-4 h-4 rounded-full bg-white absolute top-1 transition-transform {localPublic.word_wrap ? 'left-6' : 'left-1'}"></span>
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
                    class="w-11 h-6 rounded-full transition-colors relative cursor-pointer {localPublic.show_metadata ? 'bg-primary' : 'bg-neutral-800'}"
                    role="switch"
                    aria-checked={localPublic.show_metadata}
                  >
                    <span class="w-4 h-4 rounded-full bg-white absolute top-1 transition-transform {localPublic.show_metadata ? 'left-6' : 'left-1'}"></span>
                  </button>
                </div>
              </div>

              <!-- Margens da Projeção -->
              <div class="pt-4 border-t border-neutral-800 space-y-4">
                <div class="flex items-center justify-between">
                  <div>
                    <h4 class="text-xs font-semibold text-white">Margens da Projeção Pública (Safe Area)</h4>
                    <p class="text-[11px] text-neutral-400 mt-0.5">
                      Evite que as letras encostem nas bordas do telão ou sofram corte por overscan do projetor.
                    </p>
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    class="text-[11px] h-7"
                    onclick={() => {
                      localPublic.margin_top = DEFAULT_PUBLIC_SETTINGS.margin_top;
                      localPublic.margin_bottom = DEFAULT_PUBLIC_SETTINGS.margin_bottom;
                      localPublic.margin_horizontal = DEFAULT_PUBLIC_SETTINGS.margin_horizontal;
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

          <!-- ABA: MONITOR DE PALCO (STAGE) -->
          {:else if activeTab === 'stage'}
            <div class="space-y-6 max-w-2xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Layout e Cores do Monitor de Retorno</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Configure o preset visual do palco para os músicos e pregador e personalize as cores de alto contraste.
                </p>
              </div>

              <!-- Presets do Palco (Cards com diagramas) -->
              <div class="space-y-3">
                <span class="block text-xs font-semibold text-white">Preset de Exibição</span>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <!-- Preset 1: Split 50-50 -->
                  <button
                    type="button"
                    onclick={() => updateStageField('preset', 'split-50-50')}
                    class="p-3.5 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {localStage.preset === 'split-50-50' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900'}"
                  >
                    <div class="flex items-center justify-between mb-2">
                      <div class="text-xs font-bold text-white">Divisão Equilibrada (50/50)</div>
                      {#if localStage.preset === 'split-50-50'}
                        <Check size={15} class="text-primary" />
                      {/if}
                    </div>
                    <div class="h-14 w-full bg-neutral-950 rounded border border-neutral-800 p-1 flex gap-1 mb-2">
                      <div class="flex-1 rounded border border-blue-500/40 bg-blue-500/10 flex items-center justify-center text-[8px] text-blue-300 font-bold">Atual</div>
                      <div class="flex-1 rounded border border-yellow-500/40 bg-yellow-500/10 flex items-center justify-center text-[8px] text-yellow-300 font-bold">A Seguir</div>
                    </div>
                    <div class="text-[10px] text-neutral-400">Dois cartões lado a lado com etiquetas e rodapé de tempo.</div>
                  </button>

                  <!-- Preset 2: Modern Grid -->
                  <button
                    type="button"
                    onclick={() => updateStageField('preset', 'modern-grid')}
                    class="p-3.5 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {localStage.preset === 'modern-grid' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900'}"
                  >
                    <div class="flex items-center justify-between mb-2">
                      <div class="text-xs font-bold text-white">Grid Moderno Pro</div>
                      {#if localStage.preset === 'modern-grid'}
                        <Check size={15} class="text-primary" />
                      {/if}
                    </div>
                    <div class="h-14 w-full bg-neutral-950 rounded border border-neutral-800 p-1 flex gap-1 mb-2">
                      <div class="flex-[2] flex flex-col gap-0.5">
                        <div class="flex-1 rounded border border-blue-500/40 bg-blue-500/10 text-[7px] text-blue-300 font-bold flex items-center justify-center">Slide Atual</div>
                        <div class="h-4 rounded border border-yellow-500/40 bg-yellow-500/10 text-[6px] text-yellow-300 font-bold flex items-center justify-center">A Seguir</div>
                      </div>
                      <div class="flex-1 rounded border border-neutral-700 bg-neutral-900 text-[6px] text-neutral-300 font-bold flex flex-col items-center justify-center gap-0.5">
                        <span>Aviso</span>
                        <span class="text-red-400 font-mono">10:00</span>
                      </div>
                    </div>
                    <div class="text-[10px] text-neutral-400">Slide atual em destaque, próximo abaixo e coluna lateral para timer e aviso.</div>
                  </button>

                  <!-- Preset 3: Video Focus -->
                  <button
                    type="button"
                    onclick={() => updateStageField('preset', 'video-focus')}
                    class="p-3.5 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {localStage.preset === 'video-focus' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900'}"
                  >
                    <div class="flex items-center justify-between mb-2">
                      <div class="text-xs font-bold text-white">Foco em Vídeo / Letra</div>
                      {#if localStage.preset === 'video-focus'}
                        <Check size={15} class="text-primary" />
                      {/if}
                    </div>
                    <div class="h-14 w-full bg-neutral-950 rounded border border-neutral-800 p-1 flex flex-col gap-1 mb-2">
                      <div class="flex-1 rounded border border-blue-500/40 bg-blue-500/10 flex items-center justify-center text-[8px] text-blue-300 font-bold">Projeção Central Expandida</div>
                      <div class="h-3.5 rounded bg-neutral-900 border border-neutral-700 flex items-center justify-between px-1 text-[6px] text-neutral-400">
                        <span class="text-yellow-400 truncate">Próximo...</span>
                        <span class="text-red-400 font-mono">08:30</span>
                      </div>
                    </div>
                    <div class="text-[10px] text-neutral-400">Área ampla central para leitura e barra inferior minimalista com contadores.</div>
                  </button>

                  <!-- Preset 4: Horizontal Stack -->
                  <button
                    type="button"
                    onclick={() => updateStageField('preset', 'horizontal-stack')}
                    class="p-3.5 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {localStage.preset === 'horizontal-stack' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-neutral-800 bg-neutral-900/40 hover:bg-neutral-900'}"
                  >
                    <div class="flex items-center justify-between mb-2">
                      <div class="text-xs font-bold text-white">Faixas Horizontais</div>
                      {#if localStage.preset === 'horizontal-stack'}
                        <Check size={15} class="text-primary" />
                      {/if}
                    </div>
                    <div class="h-14 w-full bg-neutral-950 rounded border border-neutral-800 p-1 flex flex-col gap-0.5 mb-2">
                      <div class="h-3 rounded bg-neutral-900 border border-neutral-700 flex items-center justify-between px-1 text-[6px] text-neutral-400">
                        <span>Horário</span>
                        <span class="text-red-400 font-mono">05:00</span>
                      </div>
                      <div class="flex-1 rounded border border-blue-500/40 bg-blue-500/10 flex items-center justify-center text-[8px] text-blue-300 font-bold">Slide Atual Gigante</div>
                      <div class="h-3 rounded border border-yellow-500/40 bg-yellow-500/10 flex items-center px-1 text-[6px] text-yellow-300 font-bold">A Seguir...</div>
                    </div>
                    <div class="text-[10px] text-neutral-400">Header de status, texto central em tipografia máxima e próximo slide na base.</div>
                  </button>
                </div>
              </div>

              <!-- Cores Customizáveis do Palco -->
              <div class="pt-4 border-t border-neutral-800 space-y-4">
                <div class="flex items-center justify-between">
                  <div>
                    <h4 class="text-xs font-semibold text-white">Cores de Texto e Molduras</h4>
                    <p class="text-[11px] text-neutral-400 mt-0.5">
                      Personalize o contraste visual para melhor visualização em monitores de palco de diferentes distâncias.
                    </p>
                  </div>
                  <Button variant="outline" size="sm" class="text-[11px] h-7" onclick={handleResetStageColors}>
                    Restaurar Cores Padrão
                  </Button>
                </div>

                <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
                  <!-- Texto Slide Atual -->
                  <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/30 flex items-center justify-between">
                    <div>
                      <div class="text-[11px] font-semibold text-white">Texto Atual</div>
                      <div class="text-[9px] text-neutral-400">{localStage.current_text_color}</div>
                    </div>
                    <input
                      type="color"
                      value={localStage.current_text_color}
                      oninput={(e) => updateStageField('current_text_color', (e.target as HTMLInputElement).value)}
                      class="w-7 h-7 rounded border-0 cursor-pointer bg-transparent"
                    />
                  </div>

                  <!-- Borda Slide Atual -->
                  <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/30 flex items-center justify-between">
                    <div>
                      <div class="text-[11px] font-semibold text-white">Borda Atual</div>
                      <div class="text-[9px] text-neutral-400">{localStage.current_border_color}</div>
                    </div>
                    <input
                      type="color"
                      value={localStage.current_border_color}
                      oninput={(e) => updateStageField('current_border_color', (e.target as HTMLInputElement).value)}
                      class="w-7 h-7 rounded border-0 cursor-pointer bg-transparent"
                    />
                  </div>

                  <!-- Texto Próximo Slide -->
                  <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/30 flex items-center justify-between">
                    <div>
                      <div class="text-[11px] font-semibold text-white">Texto Próximo</div>
                      <div class="text-[9px] text-neutral-400">{localStage.next_text_color}</div>
                    </div>
                    <input
                      type="color"
                      value={localStage.next_text_color}
                      oninput={(e) => updateStageField('next_text_color', (e.target as HTMLInputElement).value)}
                      class="w-7 h-7 rounded border-0 cursor-pointer bg-transparent"
                    />
                  </div>

                  <!-- Borda Próximo Slide -->
                  <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/30 flex items-center justify-between">
                    <div>
                      <div class="text-[11px] font-semibold text-white">Borda Próximo</div>
                      <div class="text-[9px] text-neutral-400">{localStage.next_border_color}</div>
                    </div>
                    <input
                      type="color"
                      value={localStage.next_border_color}
                      oninput={(e) => updateStageField('next_border_color', (e.target as HTMLInputElement).value)}
                      class="w-7 h-7 rounded border-0 cursor-pointer bg-transparent"
                    />
                  </div>

                  <!-- Cor do Timer -->
                  <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/30 flex items-center justify-between">
                    <div>
                      <div class="text-[11px] font-semibold text-white">Cor do Timer</div>
                      <div class="text-[9px] text-neutral-400">{localStage.timer_color}</div>
                    </div>
                    <input
                      type="color"
                      value={localStage.timer_color}
                      oninput={(e) => updateStageField('timer_color', (e.target as HTMLInputElement).value)}
                      class="w-7 h-7 rounded border-0 cursor-pointer bg-transparent"
                    />
                  </div>

                  <!-- Cor do Relógio -->
                  <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/30 flex items-center justify-between">
                    <div>
                      <div class="text-[11px] font-semibold text-white">Cor do Relógio</div>
                      <div class="text-[9px] text-neutral-400">{localStage.clock_color}</div>
                    </div>
                    <input
                      type="color"
                      value={localStage.clock_color}
                      oninput={(e) => updateStageField('clock_color', (e.target as HTMLInputElement).value)}
                      class="w-7 h-7 rounded border-0 cursor-pointer bg-transparent"
                    />
                  </div>
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
                    class="w-11 h-6 rounded-full transition-colors relative cursor-pointer {localBrand.show_logo_on_public ? 'bg-primary' : 'bg-neutral-800'}"
                    role="switch"
                    aria-checked={localBrand.show_logo_on_public}
                  >
                    <span class="w-4 h-4 rounded-full bg-white absolute top-1 transition-transform {localBrand.show_logo_on_public ? 'left-6' : 'left-1'}"></span>
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
                {/if}
              </div>
            </div>

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

          <!-- ABA 6: SOBRE -->
          {:else if activeTab === 'about'}
            <div class="space-y-6 max-w-md">
              <div class="p-6 rounded-xl border border-neutral-800 bg-neutral-900/40 text-center space-y-3">
                <img
                  src="/branding/open-presenter-wordmark-dark.svg"
                  alt="OpenPresenter"
                  class="h-10 mx-auto"
                />
                <p class="text-xs text-neutral-400">
                  Sistema moderno e aberto de projeção para igrejas e eventos.
                </p>
                <div class="flex items-center justify-center gap-2 pt-1">
                  <Badge variant="secondary" class="text-[10px]">v0.7.0</Badge>
                  <Badge variant="outline" class="text-[10px] text-primary border-primary/40">GPL-3.0</Badge>
                  <Badge variant="outline" class="text-[10px] text-neutral-400">Rust + Axum + Svelte</Badge>
                </div>
              </div>

              <div class="text-[11px] text-neutral-500 space-y-1.5 text-center">
                <p>Inspirado na facilidade do Holyrics e no poder operacional do ProPresenter.</p>
                <p>Desenvolvido com foco em estabilidade, velocidade instantânea e simplicidade.</p>
              </div>
            </div>
          {/if}
        </main>
      </div>

      <!-- Rodapé do Modal -->
      <div class="h-12 px-6 border-t border-neutral-800 bg-neutral-900/60 flex items-center justify-between shrink-0 text-xs text-neutral-500">
        <span>Pressione <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 font-mono text-[10px]">Esc</kbd> para fechar</span>
        <Button variant="default" size="sm" onclick={onclose}>
          Concluir
        </Button>
      </div>
    </div>
  </div>
{/if}

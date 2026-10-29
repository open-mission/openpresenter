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
  import Badge from './ui/Badge.svelte';
  import Button from './ui/Button.svelte';
  import type { MediaFolder, SystemPaths } from '../types';

  interface Props {
    open: boolean;
    initialTab?: 'appearance' | 'sections' | 'outputs' | 'files' | 'shortcuts' | 'about';
    onclose: () => void;
  }

  let { open = false, initialTab = 'appearance', onclose }: Props = $props();

  let activeTab = $state<'appearance' | 'sections' | 'outputs' | 'files' | 'shortcuts' | 'about'>('appearance');
  let editableSections = $state<CustomSectionConfig[]>([]);

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
      loadSystemPaths();
      loadMediaFolders();
    }
  });

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
      class="w-full max-w-4xl h-[85vh] max-h-[720px] bg-neutral-950 border border-neutral-800 rounded-xl shadow-2xl flex flex-col overflow-hidden text-neutral-100"
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
            <Monitor size={15} class={activeTab === 'outputs' ? 'text-primary' : 'text-neutral-500'} />
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

          <!-- ABA 2: ARQUIVOS E PASTAS (Mapeamento Zero-Copy e Pastas do Sistema) -->
          {:else if activeTab === 'files'}
            <div class="space-y-6 max-w-2xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Arquivos e Pastas do Sistema</h3>
                <p class="text-xs text-neutral-400 mt-0.5">
                  Estrutura de dados local do OpenPresenter e integração com diretórios externos de fotos e vídeos.
                </p>
              </div>

              <!-- Pastas Padrão do Sistema -->
              {#if systemPaths}
                <div class="space-y-2.5">
                  <span class="text-xs font-semibold text-neutral-300 uppercase tracking-wider block">
                    Diretórios Padrão da Aplicação
                  </span>
                  <div class="grid grid-cols-1 gap-2 text-xs">
                    <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/40 flex items-center justify-between">
                      <div>
                        <div class="font-semibold text-white">Banco de Dados SQLite</div>
                        <div class="text-[11px] text-neutral-400 font-mono mt-0.5 truncate">{systemPaths.db_path}</div>
                      </div>
                      <Badge variant="outline" class="text-[10px]">openpresenter.db</Badge>
                    </div>

                    <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/40 flex items-center justify-between">
                      <div>
                        <div class="font-semibold text-white">Imagens do Aplicativo</div>
                        <div class="text-[11px] text-neutral-400 font-mono mt-0.5 truncate">{systemPaths.images_path}</div>
                      </div>
                      <Badge variant="outline" class="text-[10px]">media/images</Badge>
                    </div>

                    <div class="p-2.5 rounded-lg border border-neutral-800 bg-neutral-900/40 flex items-center justify-between">
                      <div>
                        <div class="font-semibold text-white">Papéis de Parede</div>
                        <div class="text-[11px] text-neutral-400 font-mono mt-0.5 truncate">{systemPaths.wallpapers_path}</div>
                      </div>
                      <Badge variant="outline" class="text-[10px]">media/wallpapers</Badge>
                    </div>
                  </div>
                </div>
              {/if}

              <!-- Pastas Externas Mapeadas (Zero-Copy) -->
              <div class="pt-4 border-t border-neutral-800 space-y-3">
                <div>
                  <h3 class="text-sm font-semibold text-white">Pastas de Mídia Importadas</h3>
                  <p class="text-xs text-neutral-400 mt-0.5">
                    Adicione pastas do seu computador (ex.: fotos do culto, avisos). O OpenPresenter lê diretamente sem duplicar espaço em disco.
                  </p>
                </div>

                <!-- Formulário para Mapear Nova Pasta -->
                <div class="p-3.5 rounded-xl border border-neutral-800 bg-neutral-900/50 space-y-3">
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
                    <div>
                      <label for="folder-name-input" class="block text-[11px] text-neutral-400 mb-1">Rótulo da Pasta</label>
                      <input
                        id="folder-name-input"
                        type="text"
                        bind:value={newFolderName}
                        placeholder="Ex: Fotos do Culto de Domingo"
                        class="w-full h-8 px-2.5 rounded bg-neutral-900 border border-neutral-700 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-primary"
                      />
                    </div>
                    <div>
                      <label for="folder-path-input" class="block text-[11px] text-neutral-400 mb-1">Caminho no Computador *</label>
                      <input
                        id="folder-path-input"
                        type="text"
                        bind:value={newFolderPath}
                        placeholder="Ex: /Users/usuario/Imagens/Culto"
                        class="w-full h-8 px-2.5 rounded bg-neutral-900 border border-neutral-700 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-primary font-mono text-[11px]"
                      />
                    </div>
                  </div>

                  {#if folderError}
                    <div class="p-2 rounded bg-red-950/40 border border-red-800/40 text-red-300 text-xs flex items-center gap-1.5">
                      <AlertCircle size={13} class="text-red-400 shrink-0" />
                      <span>{folderError}</span>
                    </div>
                  {/if}

                  <div class="flex justify-end">
                    <Button
                      variant="default"
                      size="sm"
                      onclick={handleAddFolder}
                      disabled={isAddingFolder}
                      class="gap-1.5 text-xs"
                    >
                      <FolderPlus size={13} />
                      <span>{isAddingFolder ? 'Validando...' : 'Mapear Pasta'}</span>
                    </Button>
                  </div>
                </div>

                <!-- Lista de Pastas Mapeadas -->
                <div class="space-y-2">
                  {#if mediaFolders.length === 0}
                    <div class="p-4 rounded-lg border border-dashed border-neutral-800 text-center text-xs text-neutral-500">
                      Nenhuma pasta externa mapeada ainda.
                    </div>
                  {:else}
                    {#each mediaFolders as folder (folder.id)}
                      <div class="p-3 rounded-lg border border-neutral-800 bg-neutral-900/40 flex items-center justify-between gap-3">
                        <div class="flex items-center gap-2.5 min-w-0">
                          <Folder size={16} class="text-primary shrink-0" />
                          <div class="min-w-0">
                            <div class="text-xs font-semibold text-white truncate">{folder.name}</div>
                            <div class="text-[10px] text-neutral-400 font-mono truncate">{folder.path}</div>
                          </div>
                        </div>

                        <button
                          type="button"
                          onclick={() => handleDeleteFolder(folder.id)}
                          title="Remover pasta mapeada"
                          class="w-7 h-7 rounded text-neutral-400 hover:text-red-400 hover:bg-red-500/10 flex items-center justify-center transition-colors cursor-pointer"
                        >
                          <Trash2 size={13} />
                        </button>
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
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px]">F1</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-neutral-300">Limpar Texto (mantém o fundo)</span>
                  <kbd class="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-[11px]">F2</kbd>
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

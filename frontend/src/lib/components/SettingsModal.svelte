<script lang="ts">
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
  } from 'lucide-svelte';
  import {
    themeStore,
    sectionsStore,
    neutralFamilyStore,
    wallpaperFitStore,
    setTheme,
    setNeutralFamily,
    setWallpaperFit,
    saveSections,
    resetSectionsToDefault,
    COLOR_PRESETS,
    NEUTRAL_PRESETS,
    type ThemeMode,
    type NeutralFamily,
    type WallpaperFitMode,
    type CustomSectionConfig,
  } from '../settings';
  import Badge from './ui/Badge.svelte';
  import Button from './ui/Button.svelte';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open = false, onclose }: Props = $props();

  let activeTab = $state<'appearance' | 'sections' | 'outputs' | 'shortcuts' | 'about'>('appearance');
  let editableSections = $state<CustomSectionConfig[]>([]);

  // Carrega seções editáveis quando abrir
  $effect(() => {
    if (open) {
      editableSections = JSON.parse(JSON.stringify($sectionsStore));
    }
  });

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
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-3 sm:p-6 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
    aria-label="Configurações do OpenPresenter"
    tabindex="-1"
    onclick={onclose}
    onkeydown={(e) => {
      if (e.key === 'Escape') onclose();
    }}
  >
    <!-- Modal Container -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="w-full max-w-4xl h-[85vh] max-h-[720px] bg-zinc-950 border border-zinc-800 rounded-xl shadow-2xl flex flex-col overflow-hidden text-zinc-100"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header do Modal -->
      <div class="h-14 px-6 border-b border-zinc-800 bg-zinc-900/60 flex items-center justify-between shrink-0">
        <div class="flex items-center gap-3">
          <img src="/branding/open-presenter-app-icon.svg" alt="OpenPresenter" class="w-7 h-7 rounded-md" />
          <div>
            <h2 class="text-sm font-semibold tracking-tight text-white">Configurações</h2>
            <p class="text-[11px] text-zinc-400">Preferências, aparência, saídas e atalhos de projeção</p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="w-8 h-8 rounded-lg text-zinc-400 hover:text-white hover:bg-zinc-800/80 flex items-center justify-center transition-colors cursor-pointer"
          aria-label="Fechar"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Corpo com Abas Verticais à Esquerda -->
      <div class="flex-1 flex overflow-hidden">
        <!-- Sidebar de Abas Verticais (Estilo macOS / ProPresenter) -->
        <aside class="w-48 sm:w-56 border-r border-zinc-800 bg-zinc-900/30 p-2.5 flex flex-col gap-1 shrink-0 overflow-y-auto">
          <button
            type="button"
            onclick={() => (activeTab = 'appearance')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'appearance' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900'}"
          >
            <Palette size={15} class={activeTab === 'appearance' ? 'text-emerald-400' : 'text-zinc-500'} />
            <span>Aparência</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'sections')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'sections' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900'}"
          >
            <span class="w-3.5 h-3.5 rounded-full bg-emerald-500/20 border border-emerald-500/40 flex items-center justify-center text-[9px] font-bold text-emerald-400">#</span>
            <span>Partes e Cores</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'outputs')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'outputs' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900'}"
          >
            <Monitor size={15} class={activeTab === 'outputs' ? 'text-emerald-400' : 'text-zinc-500'} />
            <span>Telas e Saídas</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'shortcuts')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'shortcuts' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900'}"
          >
            <Keyboard size={15} class={activeTab === 'shortcuts' ? 'text-emerald-400' : 'text-zinc-500'} />
            <span>Atalhos Rápidos</span>
          </button>

          <div class="my-2 border-t border-zinc-800/80"></div>

          <button
            type="button"
            onclick={() => (activeTab = 'about')}
            class="w-full px-3 py-2 rounded-lg text-left text-xs font-medium flex items-center gap-2.5 transition-colors cursor-pointer {activeTab === 'about' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900'}"
          >
            <Info size={15} class={activeTab === 'about' ? 'text-emerald-400' : 'text-zinc-500'} />
            <span>Sobre o App</span>
          </button>
        </aside>

        <!-- Conteúdo da Aba Selecionada -->
        <main class="flex-1 p-6 overflow-y-auto">
          <!-- ABA 1: APARÊNCIA (Dark / Light / System) -->
          {#if activeTab === 'appearance'}
            <div class="space-y-6 max-w-xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Tema da Interface</h3>
                <p class="text-xs text-zinc-400 mt-0.5">
                  Escolha como o painel do operador é renderizado. O tema do sistema responde automaticamente às configurações do seu sistema operacional.
                </p>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
                <!-- Tema: Sistema -->
                <button
                  type="button"
                  onclick={() => setTheme('system')}
                  class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {$themeStore === 'system' ? 'border-emerald-500 bg-emerald-950/20 ring-1 ring-emerald-500/50' : 'border-zinc-800 bg-zinc-900/40 hover:bg-zinc-900'}"
                >
                  <div class="flex items-center justify-between mb-3">
                    <Laptop size={20} class={$themeStore === 'system' ? 'text-emerald-400' : 'text-zinc-400'} />
                    {#if $themeStore === 'system'}
                      <Check size={16} class="text-emerald-400" />
                    {/if}
                  </div>
                  <div>
                    <div class="text-xs font-semibold text-white">Sistema</div>
                    <div class="text-[10px] text-zinc-400 mt-0.5">Automático conforme o OS</div>
                  </div>
                </button>

                <!-- Tema: Escuro (Dark) -->
                <button
                  type="button"
                  onclick={() => setTheme('dark')}
                  class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {$themeStore === 'dark' ? 'border-emerald-500 bg-emerald-950/20 ring-1 ring-emerald-500/50' : 'border-zinc-800 bg-zinc-900/40 hover:bg-zinc-900'}"
                >
                  <div class="flex items-center justify-between mb-3">
                    <Moon size={20} class={$themeStore === 'dark' ? 'text-emerald-400' : 'text-zinc-400'} />
                    {#if $themeStore === 'dark'}
                      <Check size={16} class="text-emerald-400" />
                    {/if}
                  </div>
                  <div>
                    <div class="text-xs font-semibold text-white">Escuro</div>
                    <div class="text-[10px] text-zinc-400 mt-0.5">Otimizado para projeção</div>
                  </div>
                </button>

                <!-- Tema: Claro (Light) -->
                <button
                  type="button"
                  onclick={() => setTheme('light')}
                  class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer {$themeStore === 'light' ? 'border-emerald-500 bg-emerald-950/20 ring-1 ring-emerald-500/50' : 'border-zinc-800 bg-zinc-900/40 hover:bg-zinc-900'}"
                >
                  <div class="flex items-center justify-between mb-3">
                    <Sun size={20} class={$themeStore === 'light' ? 'text-emerald-400' : 'text-zinc-400'} />
                    {#if $themeStore === 'light'}
                      <Check size={16} class="text-emerald-400" />
                    {/if}
                  </div>
                  <div>
                    <div class="text-xs font-semibold text-white">Claro</div>
                    <div class="text-[10px] text-zinc-400 mt-0.5">Ambientes iluminados</div>
                  </div>
                </button>
              </div>

              <!-- Cores Principais do Shadcn-UI (Paletas Neutras) -->
              <div class="pt-4 border-t border-zinc-800/80 space-y-3">
                <div>
                  <h3 class="text-sm font-semibold text-white">Paleta Neutra (Design System)</h3>
                  <p class="text-xs text-zinc-400 mt-0.5">
                    Defina a família de tons neutros usada nos painéis, bordas e botões do sistema.
                  </p>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
                  {#each NEUTRAL_PRESETS as preset}
                    <button
                      type="button"
                      onclick={() => setNeutralFamily(preset.id)}
                      class="p-3 rounded-xl border text-left flex items-center justify-between transition-all cursor-pointer {$neutralFamilyStore === preset.id ? 'border-emerald-500 bg-emerald-950/20 ring-1 ring-emerald-500/40' : 'border-zinc-800 bg-zinc-900/40 hover:bg-zinc-900'}"
                    >
                      <div class="flex items-center gap-2.5 min-w-0">
                        <div
                          class="w-5 h-5 rounded-md border border-white/20 shrink-0 shadow-inner"
                          style="background-color: {preset.previewColor};"
                        ></div>
                        <div class="min-w-0">
                          <div class="text-xs font-semibold text-white truncate">{preset.label}</div>
                          <div class="text-[10px] text-zinc-400 truncate">{preset.description}</div>
                        </div>
                      </div>

                      {#if $neutralFamilyStore === preset.id}
                        <Check size={14} class="text-emerald-400 shrink-0 ml-2" />
                      {/if}
                    </button>
                  {/each}
                </div>
              </div>
            </div>

          <!-- ABA 2: PARTES DA MÚSICA E CORES -->
          {:else if activeTab === 'sections'}
            <div class="space-y-5 max-w-2xl">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-sm font-semibold text-white">Partes da Música e Atalhos</h3>
                  <p class="text-xs text-zinc-400 mt-0.5">
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
                  <div class="p-3 rounded-lg border border-zinc-800 bg-zinc-900/40 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
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
                        class="h-7 w-28 px-2 rounded bg-zinc-900 border border-zinc-700 text-xs text-white focus:outline-none focus:border-emerald-500"
                        title="Nome da parte"
                      />
                    </div>

                    <div class="flex items-center gap-3">
                      <!-- Atalho de Tecla -->
                      <div class="flex items-center gap-1.5">
                        <span class="text-[11px] text-zinc-400">Atalho:</span>
                        <input
                          type="text"
                          maxlength="1"
                          value={section.shortcut.toUpperCase()}
                          oninput={(e) => handleShortcutChange(idx, (e.target as HTMLInputElement).value)}
                          class="h-7 w-9 text-center font-mono font-bold uppercase rounded bg-zinc-900 border border-zinc-700 text-xs text-emerald-400 focus:outline-none focus:border-emerald-500"
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
                            class="w-5 h-5 rounded-full {preset.accent} transition-transform hover:scale-110 flex items-center justify-center cursor-pointer {section.colorName === preset.name ? 'ring-2 ring-white ring-offset-1 ring-offset-zinc-950 scale-105' : 'opacity-70'}"
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

          <!-- ABA 3: TELAS E SAÍDAS -->
          {:else if activeTab === 'outputs'}
            <div class="space-y-6 max-w-xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Saídas Gráficas</h3>
                <p class="text-xs text-zinc-400 mt-0.5">
                  Gerencie as telas de saída do projetor e do retorno de palco conforme a especificação multi-telas.
                </p>
              </div>

              <div class="space-y-3.5">
                <!-- Saída Pública -->
                <div class="p-4 rounded-xl border border-zinc-800 bg-zinc-900/40 flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 flex items-center justify-center">
                      <Monitor size={18} />
                    </div>
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        Tela Pública (Public)
                        <Badge variant="outline" class="text-[9px] text-emerald-400 border-emerald-500/30">16:9 Nativo</Badge>
                      </div>
                      <div class="text-[11px] text-zinc-400 mt-0.5">
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
                <div class="p-4 rounded-xl border border-zinc-800 bg-zinc-900/40 flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-lg bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 flex items-center justify-center">
                      <Tv size={18} />
                    </div>
                    <div>
                      <div class="text-xs font-semibold text-white flex items-center gap-2">
                        Retorno do Palco (Stage)
                        <Badge variant="outline" class="text-[9px] text-emerald-400 border-emerald-500/30">Relógio + Prévia</Badge>
                      </div>
                      <div class="text-[11px] text-zinc-400 mt-0.5">
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
              <div class="pt-4 border-t border-zinc-800 space-y-3">
                <div>
                  <h3 class="text-sm font-semibold text-white">Ajuste de Papel de Parede</h3>
                  <p class="text-xs text-zinc-400 mt-0.5">
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
                      class="p-2.5 rounded-lg border text-left transition-all cursor-pointer {$wallpaperFitStore === mode.id ? 'border-emerald-500 bg-emerald-950/20 ring-1 ring-emerald-500/40 text-emerald-400 font-semibold' : 'border-zinc-800 bg-zinc-900/40 hover:bg-zinc-900 text-zinc-300'}"
                    >
                      <div class="text-xs">{mode.label}</div>
                      <div class="text-[10px] text-zinc-400 font-normal mt-0.5">{mode.desc}</div>
                    </button>
                  {/each}
                </div>
              </div>

              <!-- Pasta de Mídias no Sistema -->
              <div class="p-3.5 rounded-xl border border-zinc-800 bg-zinc-900/30 space-y-1.5">
                <div class="flex items-center gap-2 text-xs font-semibold text-white">
                  <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
                  Pasta de Mídias no Sistema
                </div>
                <p class="text-[11px] text-zinc-400 leading-relaxed">
                  Os papéis de parede e arquivos de mídia ficam salvos no diretório de dados do sistema operacional. Você pode facilmente arrastar novas imagens para a subpasta <code>media/wallpapers</code> ou migrar todos os dados entre computadores.
                </p>
              </div>
            </div>

          <!-- ABA 4: ATALHOS RÁPIDOS -->
          {:else if activeTab === 'shortcuts'}
            <div class="space-y-4 max-w-xl">
              <div>
                <h3 class="text-sm font-semibold text-white">Atalhos Operacionais de Teclado</h3>
                <p class="text-xs text-zinc-400 mt-0.5">
                  Agilidade durante o culto e eventos ao vivo sem tirar as mãos do teclado.
                </p>
              </div>

              <div class="rounded-xl border border-zinc-800 bg-zinc-900/30 divide-y divide-zinc-800/80 text-xs">
                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Apresentar slide selecionado</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px] text-emerald-400">Enter</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Parar apresentação ao vivo</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px] text-red-400">Esc</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Avançar para o próximo slide</span>
                  <div class="flex gap-1">
                    <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[10px]">→</kbd>
                    <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[10px]">Espaço</kbd>
                  </div>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Retornar ao slide anterior</span>
                  <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[10px]">←</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Pular direto para slide ordinal</span>
                  <span class="text-zinc-400"><kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[10px]">[Número]</kbd> + <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[10px]">Enter</kbd></span>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Pular para o Refrão</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px] text-emerald-400">R</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Pular para o Verso</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px] text-blue-400">V</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Pular para a Ponte</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px] text-amber-400">P</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Pular para a Introdução</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px] text-cyan-400">I</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Pular para o Final / Encerramento</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px] text-purple-400">F</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Abrir Paleta de Comandos</span>
                  <div class="flex gap-1">
                    <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[10px]">⌘K</kbd>
                    <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[10px]">Ctrl+K</kbd>
                  </div>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Blackout (tela preta total)</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px]">F1</kbd>
                </div>

                <div class="p-2.5 flex items-center justify-between">
                  <span class="text-zinc-300">Limpar Texto (mantém o fundo)</span>
                  <kbd class="px-2 py-0.5 rounded bg-zinc-800 border border-zinc-700 font-mono text-[11px]">F2</kbd>
                </div>
              </div>
            </div>

          <!-- ABA 5: SOBRE -->
          {:else if activeTab === 'about'}
            <div class="space-y-6 max-w-md">
              <div class="p-6 rounded-xl border border-zinc-800 bg-zinc-900/40 text-center space-y-3">
                <img
                  src="/branding/open-presenter-wordmark-dark.svg"
                  alt="OpenPresenter"
                  class="h-10 mx-auto"
                />
                <p class="text-xs text-zinc-400">
                  Sistema moderno e aberto de projeção para igrejas e eventos.
                </p>
                <div class="flex items-center justify-center gap-2 pt-1">
                  <Badge variant="secondary" class="text-[10px]">v0.5.0</Badge>
                  <Badge variant="outline" class="text-[10px] text-emerald-400 border-emerald-500/40">GPL-3.0</Badge>
                  <Badge variant="outline" class="text-[10px] text-zinc-400">Rust + Axum + Svelte</Badge>
                </div>
              </div>

              <div class="text-[11px] text-zinc-500 space-y-1.5 text-center">
                <p>Inspirado na facilidade do Holyrics e no poder operacional do ProPresenter.</p>
                <p>Desenvolvido com foco em estabilidade, velocidade instantânea e simplicidade.</p>
              </div>
            </div>
          {/if}
        </main>
      </div>

      <!-- Rodapé do Modal -->
      <div class="h-12 px-6 border-t border-zinc-800 bg-zinc-900/60 flex items-center justify-between shrink-0 text-xs text-zinc-500">
        <span>Pressione <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 font-mono text-[10px]">Esc</kbd> para fechar</span>
        <Button variant="default" size="sm" onclick={onclose}>
          Concluir
        </Button>
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import { onMount } from 'svelte';
  import {
    X,
    Palette,
    Sparkles,
    Layers,
    Folder,
    Check,
    Search,
    ChevronRight,
    Play,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import {
    BACKGROUND_PRESETS,
    getAllBackgrounds,
    encodeBackground,
    resolveBackground,
    type BackgroundPreset,
  } from '../backgrounds';
  import { backgroundStylesStore } from '../displaySettings';
  import type { WallpaperItem } from '../types';

  interface Props {
    open: boolean;
    currentValue?: string;
    onselect: (encodedValue: string) => void;
    onclose: () => void;
  }

  let { open = false, currentValue = '', onselect, onclose }: Props = $props();

  let selectedCategory = $state<'all' | 'gradients' | 'mesh' | 'animated' | 'custom' | 'local'>('all');
  let searchQuery = $state('');
  let localWallpapers = $state<WallpaperItem[]>([]);
  let isLoadingLocal = $state(false);

  onMount(() => {
    loadLocalWallpapers();
  });

  $effect(() => {
    if (open) {
      loadLocalWallpapers();
    }
  });

  async function loadLocalWallpapers() {
    isLoadingLocal = true;
    try {
      const res = await fetch('/api/media/wallpapers');
      if (res.ok) {
        localWallpapers = await res.json();
      }
    } catch (err) {
      console.error('Erro ao buscar papéis de parede locais:', err);
    } finally {
      isLoadingLocal = false;
    }
  }

  const availablePresets = $derived(getAllBackgrounds($backgroundStylesStore));

  const filteredPresets = $derived.by(() => {
    return availablePresets.filter((p) => {
      const matchesCat = selectedCategory === 'all' || p.category === selectedCategory;
      const matchesSearch =
        searchQuery === '' ||
        p.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (p.description && p.description.toLowerCase().includes(searchQuery.toLowerCase()));
      return matchesCat && matchesSearch;
    });
  });

  function handleSelectPreset(preset: BackgroundPreset) {
    const encoded = encodeBackground(preset);
    onselect(encoded);
    onclose();
  }

  function handleSelectLocal(wall: WallpaperItem) {
    onselect(wall.url);
    onclose();
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-150">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="bg-card border border-border w-full max-w-4xl h-[85vh] max-h-[720px] rounded-2xl shadow-2xl flex flex-col overflow-hidden text-foreground animate-in zoom-in-95 duration-150"
      onclick={(e) => e.stopPropagation()}
      onkeydown={() => {}}
    >
      <!-- Cabeçalho -->
      <div class="px-5 py-3.5 border-b border-border flex items-center justify-between bg-card shrink-0">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 text-primary flex items-center justify-center">
            <Palette size={16} />
          </div>
          <div>
            <h2 class="text-sm font-bold text-foreground">Biblioteca de Fundos para Projeção</h2>
            <p class="text-[11px] text-muted-foreground">Escolha gradientes elegantes, mesh gradients ou animações CSS</p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="p-1 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Barra de Busca e Filtros -->
      <div class="px-5 py-2.5 border-b border-border bg-muted/20 flex items-center justify-between gap-3 shrink-0">
        <div class="relative flex-1 max-w-md">
          <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Pesquisar fundos e gradientes..."
            class="w-full h-7 pl-8 pr-3 bg-background border border-border rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-primary"
          />
        </div>

        <!-- Breadcrumb / Categoria Ativa -->
        <div class="flex items-center gap-1 text-xs text-muted-foreground">
          <span>Fundos</span>
          <ChevronRight size={12} />
          <span class="text-foreground font-semibold capitalize">
            {selectedCategory === 'all'
              ? 'Todos os Fundos'
              : selectedCategory === 'gradients'
                ? 'Gradientes Lineares'
                : selectedCategory === 'mesh'
                  ? 'Mesh Gradients'
                  : selectedCategory === 'animated'
                    ? 'Gradientes Animados'
                    : selectedCategory === 'custom'
                      ? 'Fundos Customizados'
                      : 'Papéis de Parede Locais'}
          </span>
        </div>
      </div>

      <!-- Corpo Principal com Tree Navigation à Esquerda e Grid à Direita -->
      <div class="flex-1 flex overflow-hidden">
        <!-- Tree Navigation Vertical -->
        <aside class="w-56 border-r border-border bg-card/60 flex flex-col shrink-0 p-3 overflow-y-auto space-y-1">
          <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-2 py-1 block">
            Categorias
          </span>

          <button
            type="button"
            onclick={() => (selectedCategory = 'all')}
            class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedCategory === 'all' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
          >
            <div class="flex items-center gap-2">
              <Layers size={13} />
              <span>Todos</span>
            </div>
            <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
              {availablePresets.length + localWallpapers.length}
            </Badge>
          </button>

          <button
            type="button"
            onclick={() => (selectedCategory = 'animated')}
            class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedCategory === 'animated' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
          >
            <div class="flex items-center gap-2">
              <Sparkles size={13} class="text-amber-400" />
              <span>Animados (CSS)</span>
            </div>
            <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
              {availablePresets.filter((p) => p.category === 'animated').length}
            </Badge>
          </button>

          <button
            type="button"
            onclick={() => (selectedCategory = 'mesh')}
            class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedCategory === 'mesh' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
          >
            <div class="flex items-center gap-2">
              <Palette size={13} class="text-fuchsia-400" />
              <span>Mesh Gradients</span>
            </div>
            <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
              {availablePresets.filter((p) => p.category === 'mesh').length}
            </Badge>
          </button>

          <button
            type="button"
            onclick={() => (selectedCategory = 'gradients')}
            class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedCategory === 'gradients' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
          >
            <div class="flex items-center gap-2">
              <Palette size={13} class="text-sky-400" />
              <span>Gradientes Lineares</span>
            </div>
            <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
              {availablePresets.filter((p) => p.category === 'gradients').length}
            </Badge>
          </button>

          {#if availablePresets.some((p) => p.category === 'custom')}
            <button
              type="button"
              onclick={() => (selectedCategory = 'custom')}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedCategory === 'custom' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <div class="flex items-center gap-2">
                <Sparkles size={13} class="text-emerald-400" />
                <span>Customizados</span>
              </div>
              <Badge variant="secondary" class="text-[9px] py-0 px-1 font-mono">
                {availablePresets.filter((p) => p.category === 'custom').length}
              </Badge>
            </button>
          {/if}

          <button
            type="button"
            onclick={() => (selectedCategory = 'local')}
            class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedCategory === 'local' ? 'bg-primary/10 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
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

        <!-- Grade de Cartões em 16:9 -->
        <main class="flex-1 p-4 overflow-y-auto bg-background/50">
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
            <!-- Presets em Código CSS -->
            {#if selectedCategory !== 'local'}
              {#each filteredPresets as preset (preset.id)}
                {@const encoded = encodeBackground(preset)}
                {@const isSelected = currentValue === encoded}
                <div
                  onclick={() => handleSelectPreset(preset)}
                  role="button"
                  tabindex="0"
                  onkeydown={(e) => { if (e.key === 'Enter') handleSelectPreset(preset); }}
                  class="group rounded-xl border overflow-hidden flex flex-col transition-all cursor-pointer bg-card {isSelected ? 'border-primary ring-2 ring-primary/40 shadow-lg' : 'border-border hover:border-primary/50'}"
                >
                  <!-- Visual 16:9 Prévia com CSS Ativo -->
                  <div class="aspect-video w-full relative overflow-hidden flex items-center justify-center">
                    <div
                      class="absolute inset-0 w-full h-full {preset.animationClass || ''}"
                      style="background: {preset.style};"
                    ></div>

                    <!-- Overlay de Texto Sutil para Demonstração de Legibilidade -->
                    <div class="relative z-10 px-2 py-1 rounded bg-black/40 backdrop-blur-[1px] text-[10px] text-white/90 font-medium">
                      Exemplo de Letra
                    </div>

                    {#if isSelected}
                      <div class="absolute top-2 right-2 w-5 h-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center shadow">
                        <Check size={12} />
                      </div>
                    {/if}

                    {#if preset.animationClass}
                      <span class="absolute bottom-1.5 right-1.5 px-1.5 py-0.5 rounded bg-black/60 text-amber-300 font-mono text-[9px] flex items-center gap-1">
                        <Sparkles size={9} /> Animado
                      </span>
                    {/if}
                  </div>

                  <!-- Detalhes do Fundo -->
                  <div class="p-2.5 flex items-center justify-between gap-1 border-t border-border bg-card">
                    <div class="min-w-0">
                      <h4 class="text-xs font-bold text-foreground truncate">{preset.name}</h4>
                      {#if preset.description}
                        <p class="text-[10px] text-muted-foreground truncate">{preset.description}</p>
                      {/if}
                    </div>
                    <Button
                      variant={isSelected ? 'default' : 'secondary'}
                      size="sm"
                      class="h-6 text-[10px] px-2 shrink-0"
                    >
                      {isSelected ? 'Selecionado' : 'Escolher'}
                    </Button>
                  </div>
                </div>
              {/each}
            {/if}

            <!-- Papéis de Parede Locais / Imagens do Sistema -->
            {#if selectedCategory === 'all' || selectedCategory === 'local'}
              {#each localWallpapers as wall (wall.url)}
                {@const isSelected = currentValue === wall.url}
                <div
                  onclick={() => handleSelectLocal(wall)}
                  role="button"
                  tabindex="0"
                  onkeydown={(e) => { if (e.key === 'Enter') handleSelectLocal(wall); }}
                  class="group rounded-xl border overflow-hidden flex flex-col transition-all cursor-pointer bg-card {isSelected ? 'border-primary ring-2 ring-primary/40 shadow-lg' : 'border-border hover:border-primary/50'}"
                >
                  <div class="aspect-video w-full relative overflow-hidden bg-black/60 flex items-center justify-center">
                    <img
                      src={wall.url}
                      alt={wall.name}
                      class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                    />

                    {#if isSelected}
                      <div class="absolute top-2 right-2 w-5 h-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center shadow">
                        <Check size={12} />
                      </div>
                    {/if}
                  </div>

                  <div class="p-2.5 flex items-center justify-between gap-1 border-t border-border bg-card">
                    <div class="min-w-0">
                      <h4 class="text-xs font-bold text-foreground truncate">{wall.name}</h4>
                      <p class="text-[10px] text-muted-foreground truncate">Papel de Parede Local</p>
                    </div>
                    <Button
                      variant={isSelected ? 'default' : 'secondary'}
                      size="sm"
                      class="h-6 text-[10px] px-2 shrink-0"
                    >
                      {isSelected ? 'Selecionado' : 'Escolher'}
                    </Button>
                  </div>
                </div>
              {/each}
            {/if}
          </div>
        </main>
      </div>

      <!-- Rodapé -->
      <div class="px-5 py-3 border-t border-border bg-card flex items-center justify-between shrink-0">
        <span class="text-xs text-muted-foreground">
          Ao selecionar, o fundo é configurado automaticamente.
        </span>
        <Button variant="outline" size="sm" onclick={onclose}>
          Fechar
        </Button>
      </div>
    </div>
  </div>
{/if}

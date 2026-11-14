<script lang="ts">
  import {
    Search,
    Play,
    Upload,
    Folder,
    RefreshCw,
    X,
    Palette,
    Sparkles,
    ChevronRight,
    Check,
    Layers,
    Film,
    Trash2,
    Copy,
    ShieldCheck,
    AlertTriangle,
  } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import Badge from '../ui/Badge.svelte';
  import type { MenuItem } from '../ContextMenu.svelte';
  import {
    BACKGROUND_PRESETS,
    encodeBackground,
    getAllBackgrounds,
    type BackgroundPreset,
  } from '../../backgrounds';
  import { backgroundStylesStore, publicSettingsStore } from '../../displaySettings';
  import { readFileAsBase64, readHttpError, uploadWallpaper } from '../../media';
  import { notifyError, notifySuccess } from '../../toast';
  import { presentationState, setPresentationWallpaper } from '../../ws';
  import type { WallpaperItem } from '../../types';

  interface ConfirmRequest {
    title: string;
    message: string;
    confirmLabel?: string;
    onconfirm: () => Promise<void> | void;
  }

  interface Props {
    active: boolean;
    onopencontextmenu: (x: number, y: number, items: MenuItem[]) => void;
    onrequestconfirm: (req: ConfirmRequest) => void;
    oncountchange?: (count: number) => void;
  }

  let {
    active,
    onopencontextmenu,
    onrequestconfirm,
    oncountchange,
  }: Props = $props();

  let selectedBgCategory = $state<'all' | 'gradients' | 'mesh' | 'animated' | 'local' | 'custom'>('all');
  let bgSearchQuery = $state('');
  let localWallpapers = $state<WallpaperItem[]>([]);
  let isLoadingWallpapers = $state(false);
  let wallpapersError = $state<string | null>(null);
  let isUploadingWallpaper = $state(false);
  let wallpaperFileInput: HTMLInputElement | undefined = $state();
  let selectedCardKey = $state<string | null>(null);
  let hoveredPresetId = $state<string | null>(null);
  let hasLoadedOnce = false;

  const availableBackgrounds = $derived(getAllBackgrounds($backgroundStylesStore));

  $effect(() => {
    oncountchange?.(availableBackgrounds.length + localWallpapers.length);
  });

  $effect(() => {
    if (active && !hasLoadedOnce) {
      hasLoadedOnce = true;
      loadWallpapers();
    }
  });

  // Contagens pré-computadas por categoria em O(N)
  const categoryCounts = $derived.by(() => {
    let animated = 0;
    let mesh = 0;
    let gradients = 0;
    let custom = 0;
    for (const p of availableBackgrounds) {
      if (p.category === 'animated') animated++;
      else if (p.category === 'mesh') mesh++;
      else if (p.category === 'gradients') gradients++;
      else if (p.category === 'custom') custom++;
    }
    return {
      all: availableBackgrounds.length + localWallpapers.length,
      animated,
      mesh,
      gradients,
      custom,
      local: localWallpapers.length,
    };
  });

  const filteredBackgrounds = $derived.by(() => {
    const q = bgSearchQuery.trim().toLowerCase();
    return availableBackgrounds.filter((p) => {
      const matchesCat = selectedBgCategory === 'all' || p.category === selectedBgCategory;
      if (!matchesCat) return false;
      if (!q) return true;
      return (
        p.name.toLowerCase().includes(q) ||
        Boolean(p.description && p.description.toLowerCase().includes(q))
      );
    });
  });

  const filteredLocalWallpapers = $derived.by(() => {
    const q = bgSearchQuery.trim().toLowerCase();
    if (!q) return localWallpapers;
    return localWallpapers.filter((w) => w.name.toLowerCase().includes(q));
  });

  export async function loadWallpapers() {
    isLoadingWallpapers = true;
    wallpapersError = null;
    try {
      const res = await fetch('/api/media/wallpapers');
      if (res.ok) {
        localWallpapers = await res.json();
      } else {
        wallpapersError = `Não foi possível carregar papéis locais (HTTP ${res.status}).`;
      }
    } catch (err) {
      console.error('Erro ao carregar papéis de parede:', err);
      wallpapersError = 'Não foi possível conectar ao servidor para carregar papéis locais.';
    } finally {
      isLoadingWallpapers = false;
    }
  }

  function applyBackground(encodedOrUrl: string) {
    const fit = $publicSettingsStore.wallpaper_fit || 'contain';
    setPresentationWallpaper(encodedOrUrl, fit);
  }

  async function handleWallpaperUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = Array.from(input.files || []);
    if (files.length === 0) return;

    isUploadingWallpaper = true;
    let uploaded = 0;
    try {
      for (const file of files) {
        const b64 = await readFileAsBase64(file);
        await uploadWallpaper(file.name, b64);
        uploaded++;
      }
      notifySuccess(
        uploaded > 1 ? 'Fundos Importados' : 'Fundo Importado',
        uploaded > 1
          ? `${uploaded} fundos importados com sucesso.`
          : `"${files[0].name}" salvo nos papéis locais.`
      );
      await loadWallpapers();
    } catch (err: any) {
      console.error('Falha ao importar fundo:', err);
      notifyError('Erro ao Importar Fundo', err?.message || 'Não foi possível importar o arquivo.');
    } finally {
      isUploadingWallpaper = false;
      input.value = '';
    }
  }

  function requestDeleteWallpaper(wall: WallpaperItem) {
    if (wall.is_default) {
      notifyError('Ação Bloqueada', 'Papéis de parede padrão do sistema não podem ser excluídos.');
      return;
    }
    onrequestconfirm({
      title: 'Excluir Papel de Parede',
      message: `Deseja realmente excluir "${wall.name}"?\nEsta ação removerá o arquivo permanentemente.`,
      confirmLabel: 'Excluir Fundo',
      onconfirm: async () => {
        try {
          const res = await fetch(`/api/media/wallpapers/${encodeURIComponent(wall.name)}`, {
            method: 'DELETE',
          });
          if (!res.ok) {
            throw new Error(await readHttpError(res, 'Falha ao excluir papel de parede'));
          }
          notifySuccess('Papel de Parede Removido', `"${wall.name}" foi excluído com sucesso.`);
          await loadWallpapers();
        } catch (err: any) {
          notifyError('Erro ao Excluir', err.message || 'Não foi possível excluir o papel de parede.');
        }
      },
    });
  }

  function handlePresetContextMenu(e: MouseEvent, preset: BackgroundPreset) {
    e.preventDefault();
    e.stopPropagation();
    const encoded = encodeBackground(preset);
    selectedCardKey = preset.id;
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'preset-name', label: preset.name, disabled: true },
      { id: 'div-preset-0', divider: true },
      {
        id: 'preset-use-lyrics',
        label: 'Usar como Fundo das Letras (Ao Vivo)',
        icon: Play,
        onclick: () => applyBackground(encoded),
      },
      {
        id: 'preset-copy-css',
        label: 'Copiar Estilo Visual',
        icon: Copy,
        onclick: async () => {
          try {
            await navigator.clipboard.writeText(preset.style);
            notifySuccess('Estilo Copiado', 'Estilo visual copiado para a área de transferência.');
          } catch {
            notifyError('Erro', 'Não foi possível copiar o estilo.');
          }
        },
      },
    ]);
  }

  function handleLocalWallpaperContextMenu(e: MouseEvent, wall: WallpaperItem) {
    e.preventDefault();
    e.stopPropagation();
    selectedCardKey = wall.url;
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'wall-name', label: wall.name, disabled: true },
      { id: 'div-wall-0', divider: true },
      {
        id: 'wall-use-lyrics',
        label: 'Usar como Fundo das Letras (Ao Vivo)',
        icon: Play,
        onclick: () => applyBackground(wall.url),
      },
      {
        id: 'wall-copy-url',
        label: 'Copiar Caminho do Arquivo',
        icon: Copy,
        onclick: async () => {
          try {
            await navigator.clipboard.writeText(wall.url);
            notifySuccess('Caminho Copiado', 'Endereço do arquivo copiado.');
          } catch {
            notifyError('Erro', 'Não foi possível copiar o endereço.');
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
              onclick: () => requestDeleteWallpaper(wall),
            },
          ]),
    ]);
  }

  function handleBackgroundsEmptyContextMenu(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'bg-empty-header', label: 'Fundos & Gradientes', disabled: true },
      { id: 'div-bg-empty-0', divider: true },
      {
        id: 'bg-import',
        label: 'Importar Fundo (Imagem / Vídeo)...',
        icon: Upload,
        onclick: () => wallpaperFileInput?.click(),
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
    ]);
  }
</script>

<div role="tabpanel" id="panel-backgrounds" aria-labelledby="tab-backgrounds" class="flex-1 flex overflow-hidden gap-3">
  <h2 class="sr-only">Biblioteca de Fundos e Gradientes</h2>

  <aside
    aria-label="Categorias de fundos"
    class="w-48 lg:w-52 border-r border-border bg-card/50 flex flex-col shrink-0 p-2 overflow-y-auto space-y-1"
  >
    <span class="text-[11px] font-bold uppercase tracking-wider text-muted-foreground px-2 py-1 block">
      Tipos de Fundos
    </span>

    <button
      type="button"
      onclick={() => (selectedBgCategory = 'all')}
      class="w-full min-h-[30px] flex items-center justify-between px-2.5 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'all' ? 'bg-primary/15 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      <div class="flex items-center gap-2">
        <Layers size={13} />
        <span>Todos</span>
      </div>
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">{categoryCounts.all}</Badge>
    </button>

    <button
      type="button"
      onclick={() => (selectedBgCategory = 'animated')}
      class="w-full min-h-[30px] flex items-center justify-between px-2.5 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'animated' ? 'bg-primary/15 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      <div class="flex items-center gap-2">
        <Sparkles size={13} class="text-amber-600 dark:text-amber-400" />
        <span>Animados</span>
      </div>
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">{categoryCounts.animated}</Badge>
    </button>

    <button
      type="button"
      onclick={() => (selectedBgCategory = 'mesh')}
      class="w-full min-h-[30px] flex items-center justify-between px-2.5 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'mesh' ? 'bg-primary/15 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      <div class="flex items-center gap-2">
        <Palette size={13} class="text-fuchsia-600 dark:text-fuchsia-400" />
        <span>Gradientes Suaves</span>
      </div>
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">{categoryCounts.mesh}</Badge>
    </button>

    <button
      type="button"
      onclick={() => (selectedBgCategory = 'gradients')}
      class="w-full min-h-[30px] flex items-center justify-between px-2.5 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'gradients' ? 'bg-primary/15 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      <div class="flex items-center gap-2">
        <Palette size={13} class="text-sky-600 dark:text-sky-400" />
        <span>Gradientes Lineares</span>
      </div>
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">{categoryCounts.gradients}</Badge>
    </button>

    {#if categoryCounts.custom > 0}
      <button
        type="button"
        onclick={() => (selectedBgCategory = 'custom')}
        class="w-full min-h-[30px] flex items-center justify-between px-2.5 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'custom' ? 'bg-primary/15 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
      >
        <div class="flex items-center gap-2">
          <Sparkles size={13} class="text-emerald-600 dark:text-emerald-400" />
          <span>Customizados</span>
        </div>
        <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">{categoryCounts.custom}</Badge>
      </button>
    {/if}

    <button
      type="button"
      onclick={() => (selectedBgCategory = 'local')}
      class="w-full min-h-[30px] flex items-center justify-between px-2.5 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer {selectedBgCategory === 'local' ? 'bg-primary/15 text-primary font-bold' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
    >
      <div class="flex items-center gap-2">
        <Folder size={13} class="text-emerald-600 dark:text-emerald-400" />
        <span>Papéis Locais</span>
      </div>
      <Badge variant="secondary" class="text-[11px] py-0 px-1.5 font-mono">{categoryCounts.local}</Badge>
    </button>
  </aside>

  <main class="flex-1 flex flex-col overflow-hidden min-w-0">
    <div class="flex items-center justify-between gap-2 mb-2 shrink-0 pb-1.5 border-b border-border flex-wrap">
      <nav aria-label="Caminho da categoria de fundos" class="flex items-center gap-1.5 text-xs text-muted-foreground">
        <span>Mídias</span>
        <ChevronRight size={12} />
        <span>Fundos</span>
        <ChevronRight size={12} />
        <span class="text-foreground font-bold">
          {selectedBgCategory === 'all'
            ? 'Todos os Fundos'
            : selectedBgCategory === 'gradients'
              ? 'Gradientes Lineares'
              : selectedBgCategory === 'mesh'
                ? 'Gradientes Suaves'
                : selectedBgCategory === 'animated'
                  ? 'Gradientes Animados'
                  : selectedBgCategory === 'custom'
                    ? 'Fundos Customizados'
                    : 'Papéis Locais'}
        </span>
      </nav>

      <div class="flex items-center gap-1.5 flex-wrap">
        <Button
          variant="outline"
          size="sm"
          onclick={loadWallpapers}
          aria-label="Recarregar fundos locais"
          title="Recarregar fundos locais"
          class="h-8 w-8 p-0"
        >
          <RefreshCw size={13} class={isLoadingWallpapers ? 'animate-spin' : ''} />
        </Button>

        <Button
          variant="outline"
          size="sm"
          onclick={() => wallpaperFileInput?.click()}
          disabled={isUploadingWallpaper}
          class="h-8 text-xs gap-1.5 border-primary/40 text-primary hover:bg-primary/10 shrink-0 cursor-pointer"
          title="Importar imagens ou vídeos de fundo"
        >
          <Upload size={13} />
          <span>{isUploadingWallpaper ? 'Importando...' : 'Importar Fundo'}</span>
        </Button>
        <input
          type="file"
          bind:this={wallpaperFileInput}
          onchange={handleWallpaperUpload}
          accept="image/png,image/jpeg,image/webp,image/svg+xml,video/mp4,video/webm,video/quicktime"
          multiple
          class="hidden"
        />

        <div class="relative w-48 sm:w-60">
          <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <input
            type="text"
            bind:value={bgSearchQuery}
            aria-label="Filtrar fundos por nome"
            placeholder="Filtrar fundos..."
            class="w-full h-8 pl-8 pr-8 bg-background border border-input rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
          />
          {#if bgSearchQuery}
            <button
              type="button"
              onclick={() => (bgSearchQuery = '')}
              aria-label="Limpar busca de fundos"
              class="absolute right-1 top-1/2 -translate-y-1/2 w-6 h-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
            >
              <X size={12} />
            </button>
          {/if}
        </div>
      </div>
    </div>

    {#if wallpapersError}
      <div
        role="alert"
        class="mb-2 px-3 py-1.5 rounded-lg border border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-between gap-2 text-xs shrink-0"
      >
        <div class="flex items-center gap-2 min-w-0">
          <AlertTriangle size={14} class="shrink-0" />
          <span class="truncate font-medium">{wallpapersError}</span>
        </div>
        <Button variant="outline" size="sm" onclick={loadWallpapers} class="h-7 px-2.5 text-[11px] shrink-0">
          Tentar novamente
        </Button>
      </div>
    {/if}

    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="flex-1 overflow-y-auto pr-1" oncontextmenu={handleBackgroundsEmptyContextMenu}>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
        {#if selectedBgCategory !== 'local'}
          {#each filteredBackgrounds as preset (preset.id)}
            {@const encoded = encodeBackground(preset)}
            {@const isLiveBg =
              $presentationState.lyrics_background_path === encoded ||
              (Boolean($presentationState.active_media_id) && $presentationState.wallpaper_path === encoded)}
            {@const isSelectedCard = selectedCardKey === preset.id}
            {@const shouldAnimate = Boolean(preset.animationClass) && (hoveredPresetId === preset.id || isLiveBg)}

            <article
              onpointerenter={() => (hoveredPresetId = preset.id)}
              onpointerleave={() => (hoveredPresetId = hoveredPresetId === preset.id ? null : hoveredPresetId)}
              oncontextmenu={(e) => handlePresetContextMenu(e, preset)}
              class="group relative rounded-xl border overflow-hidden flex flex-col transition-all bg-card {isLiveBg ? 'border-primary ring-2 ring-primary shadow-lg' : isSelectedCard ? 'border-primary/70 ring-1 ring-primary/50' : 'border-border hover:border-primary/50'}"
            >
              <button
                type="button"
                onclick={() => (selectedCardKey = preset.id)}
                ondblclick={() => applyBackground(encoded)}
                onfocusin={() => (hoveredPresetId = preset.id)}
                onfocusout={() => (hoveredPresetId = hoveredPresetId === preset.id ? null : hoveredPresetId)}
                aria-label="Selecionar fundo {preset.name} (duplo clique aplica ao vivo)"
                aria-pressed={isSelectedCard || isLiveBg}
                class="aspect-video w-full relative overflow-hidden flex items-center justify-center cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
              >
                <div
                  class="absolute inset-0 w-full h-full {shouldAnimate ? preset.animationClass : ''}"
                  style="background: {preset.style};"
                ></div>

                {#if isLiveBg}
                  <div class="absolute top-1.5 right-1.5 px-2 py-0.5 rounded-full bg-primary text-primary-foreground text-[11px] font-bold flex items-center gap-1 shadow-md">
                    <Check size={11} /> Em Uso
                  </div>
                {/if}

                {#if preset.animationClass}
                  <span class="absolute top-1.5 left-1.5 px-1.5 py-0.5 rounded bg-slate-950/85 text-amber-200 font-mono text-[11px] flex items-center gap-1">
                    <Sparkles size={10} /> Animado
                  </span>
                {/if}

                <span class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-black/85 via-black/45 to-transparent px-2 pt-3 pb-1 text-left text-[11px] text-white font-medium truncate">
                  {preset.name}
                </span>
              </button>

              <div class="p-1.5 bg-muted/30 border-t border-border flex items-center justify-between gap-1">
                <button
                  type="button"
                  onclick={() => applyBackground(encoded)}
                  class="flex-1 h-7 rounded-md bg-primary/15 hover:bg-primary/25 text-primary text-[11px] font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                  title="Definir como fundo das letras ao vivo"
                >
                  <Play size={11} class="fill-current" />
                  <span>Usar nas Letras</span>
                </button>
              </div>
            </article>
          {/each}
        {/if}

        {#if selectedBgCategory === 'all' || selectedBgCategory === 'local'}
          {#each filteredLocalWallpapers as wall (wall.url)}
            {@const isLiveBg =
              $presentationState.lyrics_background_path === wall.url ||
              (Boolean($presentationState.active_media_id) && $presentationState.wallpaper_path === wall.url)}
            {@const isSelectedCard = selectedCardKey === wall.url}

            <article
              oncontextmenu={(e) => handleLocalWallpaperContextMenu(e, wall)}
              class="group relative rounded-xl border overflow-hidden flex flex-col transition-all bg-card {isLiveBg ? 'border-primary ring-2 ring-primary shadow-lg' : isSelectedCard ? 'border-primary/70 ring-1 ring-primary/50' : 'border-border hover:border-primary/50'}"
            >
              <button
                type="button"
                onclick={() => (selectedCardKey = wall.url)}
                ondblclick={() => applyBackground(wall.url)}
                onpointerenter={(e) => {
                  const v = e.currentTarget.querySelector('video');
                  v?.play().catch(() => {});
                }}
                onpointerleave={(e) => {
                  const v = e.currentTarget.querySelector('video');
                  if (v) {
                    v.pause();
                    v.currentTime = 0;
                  }
                }}
                aria-label="Selecionar fundo local {wall.name} (duplo clique aplica ao vivo)"
                aria-pressed={isSelectedCard || isLiveBg}
                class="aspect-video w-full relative overflow-hidden bg-slate-950 flex items-center justify-center cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
              >
                {#if wall.is_video}
                  <video
                    src={wall.url}
                    class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200 pointer-events-none"
                    preload="none"
                    loop
                    muted
                    playsinline
                  ></video>
                  <div class="absolute top-1.5 left-1.5 px-1.5 py-0.5 rounded bg-slate-950/85 border border-white/20 text-[11px] font-mono font-bold text-white flex items-center gap-1 shadow">
                    <Film size={10} class="text-primary" />
                    <span>Vídeo</span>
                  </div>
                {:else}
                  <img
                    src={wall.url}
                    alt={wall.name}
                    loading="lazy"
                    class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
                  />
                {/if}

                {#if isLiveBg}
                  <div class="absolute top-1.5 right-1.5 px-2 py-0.5 rounded-full bg-primary text-primary-foreground text-[11px] font-bold flex items-center gap-1 shadow-md">
                    <Check size={11} /> Em Uso
                  </div>
                {/if}

                <span class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-black/85 via-black/45 to-transparent px-2 pt-3 pb-1 text-left text-[11px] text-white font-medium truncate">
                  {wall.name}
                </span>
              </button>

              <div class="p-1.5 bg-muted/30 border-t border-border flex items-center justify-between gap-1">
                <button
                  type="button"
                  onclick={() => applyBackground(wall.url)}
                  class="flex-1 h-7 rounded-md bg-primary/15 hover:bg-primary/25 text-primary text-[11px] font-bold flex items-center justify-center gap-1 transition-colors cursor-pointer"
                  title="Definir como fundo das letras ao vivo"
                >
                  <Play size={11} class="fill-current" />
                  <span>Usar nas Letras</span>
                </button>

                {#if !wall.is_default}
                  <button
                    type="button"
                    onclick={() => requestDeleteWallpaper(wall)}
                    aria-label="Excluir fundo local {wall.name}"
                    class="h-7 w-7 rounded-md bg-transparent hover:bg-destructive/15 text-muted-foreground hover:text-destructive flex items-center justify-center transition-colors cursor-pointer shrink-0"
                    title="Excluir este papel de parede local"
                  >
                    <Trash2 size={13} />
                  </button>
                {/if}
              </div>
            </article>
          {/each}
        {/if}
      </div>
    </div>
  </main>
</div>

<script lang="ts">
  import { onMount } from 'svelte';
  import {
    X,
    ImageIcon,
    Folder,
    Check,
    Plus,
    Search,
    Layers,
    FolderPlus,
    RefreshCw,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import type { ImageMediaItem, MediaFolder, MediaItem } from '$lib/types';

  interface Props {
    open: boolean;
    initialMode?: 'single' | 'folder';
    title?: string;
    subtitle?: string;
    allowFolders?: boolean;
    onclose: () => void;
    onadditems?: (items: MediaItem[]) => void;
    onselectimage?: (url: string) => void;
  }

  let {
    open = false,
    initialMode = 'single',
    title,
    subtitle,
    allowFolders = true,
    onclose,
    onadditems,
    onselectimage,
  }: Props = $props();

  let mode = $state<'single' | 'folder'>('single');
  let images = $state<ImageMediaItem[]>([]);
  let folders = $state<MediaFolder[]>([]);
  let selectedFolderFilter = $state<string>('all');
  let searchQuery = $state<string>('');
  let selectedImageIds = $state<Set<string>>(new Set());
  let selectedFolderId = $state<string>('');
  let isLoading = $state(false);

  $effect(() => {
    if (open) {
      mode = onselectimage || !allowFolders ? 'single' : initialMode;
      selectedImageIds = new Set();
      loadData();
    }
  });

  async function loadData() {
    isLoading = true;
    try {
      const [imgRes, fldRes] = await Promise.all([
        fetch('/api/media/images'),
        fetch('/api/media/folders'),
      ]);
      if (imgRes.ok) images = await imgRes.json();
      if (fldRes.ok) folders = await fldRes.json();
      if (folders.length > 0 && !selectedFolderId) {
        selectedFolderId = folders[0].id;
      }
    } catch (err) {
      console.error('Erro ao carregar imagens:', err);
    } finally {
      isLoading = false;
    }
  }

  const filteredImages = $derived.by(() => {
    return images.filter((img) => {
      const matchesFolder =
        selectedFolderFilter === 'all' ||
        (selectedFolderFilter === 'app' && !img.is_external) ||
        (selectedFolderFilter !== 'all' &&
          selectedFolderFilter !== 'app' &&
          img.folder_id === selectedFolderFilter);

      const matchesSearch =
        searchQuery === '' ||
        img.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (img.subfolder && img.subfolder.toLowerCase().includes(searchQuery.toLowerCase()));

      return matchesFolder && matchesSearch;
    });
  });

  const appImagesCount = $derived.by(() => images.filter((i) => !i.is_external).length);

  function toggleImageSelection(id: string) {
    if (onselectimage) {
      selectedImageIds = selectedImageIds.has(id) ? new Set() : new Set([id]);
      return;
    }
    const next = new Set(selectedImageIds);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    selectedImageIds = next;
  }

  function handleSelectAll() {
    if (onselectimage) return;
    if (selectedImageIds.size === filteredImages.length) {
      selectedImageIds = new Set();
    } else {
      selectedImageIds = new Set(filteredImages.map((img) => img.id));
    }
  }

  function handleConfirmSingle() {
    if (selectedImageIds.size === 0) return;
    const selectedList = images.filter((img) => selectedImageIds.has(img.id));
    const items: MediaItem[] = selectedList.map((img, idx) => ({
      id: `img-${Date.now()}-${idx}`,
      title: img.name,
      artist_or_ref: img.folder_name,
      kind: 'image',
      slides: [
        {
          id: `slide-img-${Date.now()}-${idx}`,
          label: 'Imagem',
          content: img.url,
        },
      ],
    }));

    onadditems?.(items);
    onclose();
  }

  function handleConfirmFolder() {
    let folderImages: ImageMediaItem[] = [];
    let title = 'Pasta de Imagens';
    let artist = 'Imagens';

    if (selectedFolderId === 'app') {
      folderImages = images.filter((i) => !i.is_external);
      title = 'Imagens do Aplicativo';
      artist = 'Pasta do Sistema';
    } else {
      const f = folders.find((fld) => fld.id === selectedFolderId);
      if (f) {
        folderImages = images.filter((i) => i.folder_id === f.id);
        title = f.name;
        artist = f.path;
      }
    }

    if (folderImages.length === 0) {
      alert('Esta pasta não possui imagens.');
      return;
    }

    // Cria um item de mídia na Setlist contendo todas as imagens como slides em ordem
    const item: MediaItem = {
      id: `folder-img-${Date.now()}`,
      title,
      artist_or_ref: artist,
      kind: 'image',
      slides: folderImages.map((img, idx) => ({
        id: `slide-${Date.now()}-${idx}`,
        label: `Slide ${idx + 1}`,
        content: img.url,
      })),
    };

    onadditems?.([item]);
    onclose();
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4 animate-in fade-in duration-100"
    onclick={onclose}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="w-full max-w-3xl h-[80vh] max-h-[640px] rounded-2xl border border-border bg-card shadow-2xl flex flex-col overflow-hidden text-foreground select-none"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Cabeçalho do Modal -->
      <div class="h-14 px-5 border-b border-border bg-card/60 flex items-center justify-between shrink-0">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/30 text-primary flex items-center justify-center">
            <ImageIcon size={16} />
          </div>
          <div>
            <h2 class="text-sm font-bold text-foreground">
              {title || (onselectimage ? 'Selecionar Imagem para Lower Third' : 'Adicionar Imagens à Setlist')}
            </h2>
            <p class="text-[11px] text-muted-foreground">
              {subtitle || (onselectimage ? 'Escolha uma imagem dedicada para ser exibida na saída do OBS' : 'Selecione imagens avulsas ou importe uma pasta inteira')}
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="w-8 h-8 rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer"
          title="Fechar"
        >
          <X size={16} />
        </button>
      </div>

      <!-- Abas de Modo: Imagens Avulsas vs Pasta Completa -->
      <div class="px-5 pt-3 pb-2 border-b border-border bg-muted/20 flex items-center justify-between gap-4 shrink-0">
        {#if allowFolders && !onselectimage}
          <div class="flex items-center gap-1.5 p-1 rounded-lg bg-muted/40 border border-border">
            <button
              type="button"
              onclick={() => (mode = 'single')}
              class="px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {mode === 'single' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            >
              Imagens Individuais
            </button>
            <button
              type="button"
              onclick={() => (mode = 'folder')}
              class="px-3 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer {mode === 'folder' ? 'bg-primary text-primary-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
            >
              Pasta Completa de Imagens
            </button>
          </div>
        {:else}
          <div class="text-xs font-medium text-muted-foreground">
            {onselectimage ? 'Clique para selecionar a imagem alternativa' : 'Imagens'}
          </div>
        {/if}

        {#if mode === 'single'}
          <!-- Filtro de Busca -->
          <div class="relative w-52 sm:w-64">
            <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
            <input
              type="text"
              bind:value={searchQuery}
              placeholder="Buscar imagem..."
              class="w-full h-8 pl-8 pr-3 rounded-lg bg-background border border-border text-xs text-foreground placeholder-muted-foreground focus:outline-none focus:border-primary"
            />
          </div>
        {/if}
      </div>

      <!-- Conteúdo Conforme o Modo -->
      <div class="flex-1 overflow-y-auto p-5">
        {#if mode === 'single'}
          <!-- Filtro por Pasta -->
          <div class="flex items-center gap-1.5 overflow-x-auto pb-3 mb-3 border-b border-border text-xs">
            <span class="text-[11px] text-muted-foreground font-medium flex items-center gap-1">
              <Folder size={12} /> Pasta:
            </span>
            <button
              type="button"
              onclick={() => (selectedFolderFilter = 'all')}
              class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {selectedFolderFilter === 'all' ? 'bg-primary/20 text-primary font-bold' : 'text-muted-foreground hover:bg-muted'}"
            >
              Todas ({images.length})
            </button>
            <button
              type="button"
              onclick={() => (selectedFolderFilter = 'app')}
              class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {selectedFolderFilter === 'app' ? 'bg-primary/20 text-primary font-bold' : 'text-muted-foreground hover:bg-muted'}"
            >
              Pasta do App
            </button>
            {#each folders as f (f.id)}
              <button
                type="button"
                onclick={() => (selectedFolderFilter = f.id)}
                class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {selectedFolderFilter === f.id ? 'bg-primary/20 text-primary font-bold' : 'text-muted-foreground hover:bg-muted'}"
              >
                {f.name}
              </button>
            {/each}

            {#if !onselectimage}
              <div class="ml-auto">
                <button
                  type="button"
                  onclick={handleSelectAll}
                  class="text-[11px] text-primary hover:underline font-medium cursor-pointer"
                >
                  {selectedImageIds.size === filteredImages.length && filteredImages.length > 0 ? 'Desmarcar Todas' : 'Selecionar Todas'}
                </button>
              </div>
            {/if}
          </div>

          <!-- Grade de Imagens Selecionáveis -->
          {#if filteredImages.length === 0}
            <div class="h-44 flex flex-col items-center justify-center text-xs text-muted-foreground">
              Nenhuma imagem encontrada.
            </div>
          {:else}
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
              {#each filteredImages as img (img.id)}
                {@const isSelected = selectedImageIds.has(img.id)}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                  onclick={() => toggleImageSelection(img.id)}
                  ondblclick={() => {
                    if (onselectimage) {
                      onselectimage(img.url);
                      onclose();
                    }
                  }}
                  class="group relative aspect-video rounded-xl border transition-all cursor-pointer overflow-hidden shadow-sm {isSelected ? 'border-primary ring-2 ring-primary/50' : 'border-border bg-card/60 hover:border-primary/40'}"
                >
                  <img src={img.url} alt={img.name} class="w-full h-full object-cover" loading="lazy" />

                  <!-- Checkbox de Seleção -->
                  <div class="absolute top-2 right-2 w-5 h-5 rounded-md flex items-center justify-center transition-colors {isSelected ? 'bg-primary text-primary-foreground shadow' : 'bg-black/60 border border-white/40 text-transparent group-hover:border-white'}">
                    {#if isSelected}
                      <Check size={12} strokeWidth={3} />
                    {/if}
                  </div>

                  <!-- Legenda -->
                  <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/50 to-transparent p-2 pt-4">
                    <p class="text-[11px] font-semibold text-white truncate drop-shadow">{img.name}</p>
                    <p class="text-[9px] text-neutral-400 truncate">{img.folder_name}{img.subfolder ? ` • ${img.subfolder}` : ''}</p>
                  </div>
                </div>
              {/each}
            </div>
          {/if}

        {:else}
          <!-- Modo Pasta Completa -->
          <div class="space-y-4 max-w-xl mx-auto py-2">
            <div>
              <h3 class="text-xs font-bold text-foreground uppercase tracking-wider mb-1">Escolha a Pasta a Inserir na Setlist</h3>
              <p class="text-xs text-muted-foreground">Todas as imagens da pasta selecionada serão organizadas em uma apresentação sequencial de slides.</p>
            </div>

            <div class="space-y-2">
              <!-- Pasta Interna do App -->
              <div
                onclick={() => (selectedFolderId = 'app')}
                class="p-3.5 rounded-xl border transition-all cursor-pointer flex items-center justify-between {selectedFolderId === 'app' ? 'border-primary bg-primary/5 ring-1 ring-primary/40' : 'border-border bg-card/40 hover:bg-muted/40'}"
              >
                <div class="flex items-center gap-3">
                  <div class="w-9 h-9 rounded-lg bg-primary/10 border border-primary/30 text-primary flex items-center justify-center">
                    <Folder size={18} />
                  </div>
                  <div>
                    <h4 class="text-xs font-bold text-foreground">Imagens do Aplicativo</h4>
                    <p class="text-[11px] text-muted-foreground">Pasta interna de mídia do OpenPresenter</p>
                  </div>
                </div>
                <Badge variant="outline" class="text-xs">{appImagesCount} imagens</Badge>
              </div>

              <!-- Pastas Mapeadas do Usuário -->
              {#each folders as f (f.id)}
                {@const count = images.filter((i) => i.folder_id === f.id).length}
                <div
                  onclick={() => (selectedFolderId = f.id)}
                  class="p-3.5 rounded-xl border transition-all cursor-pointer flex items-center justify-between {selectedFolderId === f.id ? 'border-primary bg-primary/5 ring-1 ring-primary/40' : 'border-border bg-card/40 hover:bg-muted/40'}"
                >
                  <div class="flex items-center gap-3 min-w-0">
                    <div class="w-9 h-9 rounded-lg bg-primary/10 border border-primary/30 text-primary flex items-center justify-center shrink-0">
                      <Folder size={18} />
                    </div>
                    <div class="min-w-0">
                      <h4 class="text-xs font-bold text-foreground truncate">{f.name}</h4>
                      <p class="text-[10px] text-muted-foreground font-mono truncate">{f.path}</p>
                    </div>
                  </div>
                  <Badge variant="outline" class="text-xs shrink-0">{count} imagens</Badge>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Rodapé do Modal -->
      <div class="h-14 px-5 border-t border-border bg-card/60 flex items-center justify-between shrink-0">
        <div class="text-xs text-muted-foreground">
          {#if onselectimage}
            <span>{selectedImageIds.size > 0 ? '1 imagem selecionada (ou clique duplo)' : 'Selecione uma imagem'}</span>
          {:else if mode === 'single'}
            <span>{selectedImageIds.size} imagem(ns) selecionada(s)</span>
          {:else}
            <span>Inserir pasta como apresentação de slides</span>
          {/if}
        </div>

        <div class="flex items-center gap-2">
          <Button variant="secondary" size="sm" onclick={onclose}>Cancelar</Button>
          {#if onselectimage}
            <Button
              variant="default"
              size="sm"
              onclick={() => {
                const id = Array.from(selectedImageIds)[0];
                const img = images.find((i) => i.id === id);
                if (img) {
                  onselectimage(img.url);
                  onclose();
                }
              }}
              disabled={selectedImageIds.size === 0}
              class="gap-1.5"
            >
              <Check size={14} />
              <span>Selecionar Imagem</span>
            </Button>
          {:else if mode === 'single'}
            <Button
              variant="default"
              size="sm"
              onclick={handleConfirmSingle}
              disabled={selectedImageIds.size === 0}
              class="gap-1.5"
            >
              <Plus size={14} />
              <span>Adicionar ({selectedImageIds.size}) à Setlist</span>
            </Button>
          {:else}
            <Button
              variant="default"
              size="sm"
              onclick={handleConfirmFolder}
              disabled={!selectedFolderId}
              class="gap-1.5"
            >
              <Plus size={14} />
              <span>Adicionar Pasta à Setlist</span>
            </Button>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}

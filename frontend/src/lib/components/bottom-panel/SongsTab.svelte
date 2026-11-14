<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import {
    Music,
    Plus,
    Search,
    Play,
    ListPlus,
    Upload,
    RefreshCw,
    FileText,
    X,
    Tag,
    Pencil,
    Trash2,
    Copy,
    AlertTriangle,
  } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import Badge from '../ui/Badge.svelte';
  import { ResizablePaneGroup, ResizablePane, ResizableHandle } from '../ui/resizable';
  import type { MenuItem } from '../ContextMenu.svelte';
  import { renderFormattedText } from '../../markdown';
  import { readHttpError } from '../../media';
  import { parseRawLyrics, getSectionColor } from '../../songParser';
  import { notifyError, notifySuccess } from '../../toast';
  import type { Slide, Song, SongSummary } from '../../types';

  interface ConfirmRequest {
    title: string;
    message: string;
    confirmLabel?: string;
    onconfirm: () => Promise<void> | void;
  }

  interface Props {
    active: boolean;
    onaddsongtosetlist?: (songId: string) => void;
    onpresentsong?: (songId: string, slideIndex?: number) => void;
    onopensongeditor?: () => void;
    onopeneditsong?: (songId: string) => void;
    onopencontextmenu: (x: number, y: number, items: MenuItem[]) => void;
    onrequestconfirm: (req: ConfirmRequest) => void;
    oncountchange?: (count: number) => void;
  }

  let {
    active,
    onaddsongtosetlist,
    onpresentsong,
    onopensongeditor,
    onopeneditsong,
    onopencontextmenu,
    onrequestconfirm,
    oncountchange,
  }: Props = $props();

  function getSavedSongLimit(): number {
    if (typeof window === 'undefined') return 50;
    try {
      const raw = Number(localStorage.getItem('open_presenter_song_limit') || '50');
      return Number.isFinite(raw) ? raw : 50;
    } catch {
      return 50;
    }
  }

  let songSearch = $state('');
  let songs = $state<SongSummary[]>([]);
  let songTags = $state<string[]>([]);
  let selectedSongTag = $state<string | null>(null);
  let songLimit = $state<number>(getSavedSongLimit());
  let isLoadingSongs = $state(false);
  let songsError = $state<string | null>(null);
  let isImportingSongs = $state(false);
  let songFileInputEl: HTMLInputElement | null = $state(null);
  let viewingSong = $state<Song | null>(null);
  let isLoadingLyrics = $state(false);
  let hasLoadedOnce = false;

  let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let songsAbortController: AbortController | null = null;
  let songsRequestId = 0;

  $effect(() => {
    oncountchange?.(songs.length);
  });

  $effect(() => {
    if (active && !hasLoadedOnce) {
      hasLoadedOnce = true;
      loadSongs();
      loadSongTags();
    }
  });

  onMount(() => {
    const handleSongsChanged = () => {
      if (hasLoadedOnce || active) {
        loadSongs();
        loadSongTags();
      }
    };
    if (typeof window !== 'undefined') {
      window.addEventListener('openpresenter:songs-changed', handleSongsChanged);
      return () => {
        window.removeEventListener('openpresenter:songs-changed', handleSongsChanged);
      };
    }
  });

  onDestroy(() => {
    if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
    songsAbortController?.abort();
  });

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
            const curNum = slideNum++;
            result.push({
              id: `slide-${curNum}`,
              label: `${sec.label || 'Parte'} (${bIdx + 1}/${blocks.length})`,
              content: block,
            });
          });
        } else {
          const curNum = slideNum++;
          result.push({
            id: `slide-${curNum}`,
            label: sec.label || `Parte ${curNum}`,
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
    songsAbortController?.abort();
    songsAbortController = new AbortController();
    const reqId = ++songsRequestId;

    isLoadingSongs = true;
    songsError = null;
    try {
      const params = new URLSearchParams();
      const q = songSearch.trim();
      if (q) params.set('q', q);
      if (selectedSongTag) params.set('tag', selectedSongTag);
      if (songLimit > 0) params.set('limit', String(songLimit));
      const url = params.toString() ? `/api/songs?${params.toString()}` : '/api/songs';
      const res = await fetch(url, { signal: songsAbortController.signal });
      if (reqId !== songsRequestId) return;
      if (res.ok) {
        songs = await res.json();
      } else {
        songsError = `Não foi possível carregar as músicas (HTTP ${res.status}).`;
      }
    } catch (err: any) {
      if (err?.name === 'AbortError') return;
      console.error('Erro ao buscar músicas:', err);
      songsError = 'Não foi possível conectar ao servidor para carregar as músicas.';
    } finally {
      if (reqId === songsRequestId) {
        isLoadingSongs = false;
      }
    }
  }

  function handleSearchInput() {
    if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
    searchDebounceTimer = setTimeout(() => {
      loadSongs();
    }, 220);
  }

  function handleSongLimitChange(newLimit: number) {
    songLimit = newLimit;
    if (typeof window !== 'undefined') {
      try {
        localStorage.setItem('open_presenter_song_limit', String(newLimit));
      } catch {
        // storage disabled
      }
    }
    loadSongs();
  }

  function selectSongTag(tag: string | null) {
    selectedSongTag = tag;
    loadSongs();
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
        if (!res.ok) {
          throw new Error(`Falha ao importar "${file.name}" (HTTP ${res.status}).`);
        }
        const data = await res.json();
        totalImported += data.imported_count || 1;
      }
      notifySuccess('Importação Concluída', `${totalImported} música(s) importada(s) com sucesso!`);
      await loadSongs();
      await loadSongTags();
    } catch (err: any) {
      console.error('Erro ao importar arquivos de música:', err);
      notifyError('Erro ao Importar Músicas', err?.message || String(err));
    } finally {
      isImportingSongs = false;
      input.value = '';
    }
  }

  async function handleViewSongLyrics(songId: string) {
    isLoadingLyrics = true;
    try {
      const res = await fetch(`/api/songs/${songId}`);
      if (!res.ok) {
        notifyError('Erro', `Não foi possível abrir a letra (HTTP ${res.status}).`);
        return;
      }
      viewingSong = await res.json();
    } catch (err) {
      console.error('Erro ao buscar letra:', err);
      notifyError('Erro', 'Não foi possível carregar a letra da música.');
    } finally {
      isLoadingLyrics = false;
    }
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

  function requestDeleteSong(song: SongSummary | Song) {
    onrequestconfirm({
      title: 'Excluir Música',
      message: `Deseja realmente excluir "${song.title}" do catálogo de músicas?\nEsta ação não poderá ser desfeita.`,
      confirmLabel: 'Excluir Música',
      onconfirm: async () => {
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
            notifyError('Erro ao Excluir', await readHttpError(res, 'Não foi possível excluir a música'));
          }
        } catch {
          notifyError('Erro de Conexão', 'Falha ao se comunicar com o servidor.');
        }
      },
    });
  }

  function handleSongContextMenu(e: MouseEvent, song: SongSummary) {
    e.preventDefault();
    e.stopPropagation();
    const artistText = song.artist ? ` • ${song.artist}` : '';
    const keyText = song.song_key ? ` (${song.song_key})` : '';
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'song-header', label: `${song.title}${artistText}${keyText}`, disabled: true },
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
        onclick: () => requestDeleteSong(song),
      },
    ]);
  }

  function handleViewingSongSlideContextMenu(e: MouseEvent, song: Song, slide: Slide, idx: number) {
    e.preventDefault();
    e.stopPropagation();
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'slide-header', label: `${song.title} — ${slide.label || `Slide ${idx + 1}`}`, disabled: true },
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
    ]);
  }

  function handleViewingSongSheetContextMenu(e: MouseEvent, song: Song) {
    e.preventDefault();
    e.stopPropagation();
    const artistText = song.artist ? ` • ${song.artist}` : '';
    const keyText = song.song_key ? ` (${song.song_key})` : '';
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'sheet-header', label: `${song.title}${artistText}${keyText}`, disabled: true },
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
        onclick: () => requestDeleteSong(song),
      },
      {
        id: 'sheet-close',
        label: 'Fechar Visualização',
        icon: X,
        onclick: () => (viewingSong = null),
      },
    ]);
  }

  function handleSongsEmptyContextMenu(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    onopencontextmenu(e.clientX, e.clientY, [
      { id: 'songs-empty-header', label: 'Catálogo de Músicas', disabled: true },
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
        onclick: () => songFileInputEl?.click(),
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
    ]);
  }
</script>

<div role="tabpanel" id="panel-songs" aria-labelledby="tab-songs" class="flex-1 flex flex-col min-w-0 overflow-hidden">
  <h2 class="sr-only">Catálogo de Músicas</h2>

  <!-- Barra de Ações da Aba Músicas -->
  <div class="flex items-center justify-between gap-2 mb-2 shrink-0 flex-wrap">
    <div class="relative flex-1 min-w-[200px] max-w-md">
      <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
      <input
        type="text"
        bind:value={songSearch}
        oninput={handleSearchInput}
        aria-label="Pesquisar músicas no catálogo"
        placeholder="Pesquisar músicas (clique para ver letra)..."
        class="w-full h-8 pl-8 pr-8 bg-background border border-input rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
      />
      {#if songSearch}
        <button
          type="button"
          onclick={() => {
            songSearch = '';
            loadSongs();
          }}
          aria-label="Limpar busca de músicas"
          class="absolute right-1 top-1/2 -translate-y-1/2 w-6 h-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
        >
          <X size={12} />
        </button>
      {/if}
    </div>

    <div class="flex items-center gap-1.5 flex-wrap">
      <div class="flex items-center gap-1.5 bg-background border border-input rounded-md px-2 h-8 text-xs text-foreground">
        <label for="songs-limit-select" class="text-[11px] text-muted-foreground font-medium hidden sm:inline">
          Exibir:
        </label>
        <select
          id="songs-limit-select"
          value={songLimit}
          onchange={(e) => handleSongLimitChange(Number((e.target as HTMLSelectElement).value))}
          aria-label="Quantidade de músicas exibidas"
          class="bg-transparent border-none text-xs text-foreground font-medium focus:outline-none cursor-pointer p-0"
        >
          <option value={50} class="bg-popover text-popover-foreground">50</option>
          <option value={100} class="bg-popover text-popover-foreground">100</option>
          <option value={150} class="bg-popover text-popover-foreground">150</option>
          <option value={200} class="bg-popover text-popover-foreground">200</option>
          <option value={0} class="bg-popover text-popover-foreground">Todas</option>
        </select>
      </div>

      <Button
        variant="outline"
        size="sm"
        onclick={() => {
          loadSongs();
          loadSongTags();
        }}
        aria-label="Recarregar catálogo de músicas"
        title="Recarregar catálogo"
        class="h-8 w-8 p-0"
      >
        <RefreshCw size={13} class={isLoadingSongs ? 'animate-spin' : ''} />
      </Button>

      <Button
        variant="outline"
        size="sm"
        onclick={() => songFileInputEl?.click()}
        disabled={isImportingSongs}
        title="Importar músicas (.json, .txt, .md)"
        class="h-8 text-xs gap-1 cursor-pointer"
      >
        <Upload size={13} class={isImportingSongs ? 'animate-spin text-primary' : 'text-primary'} />
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

      <Button variant="default" size="sm" onclick={onopensongeditor} class="h-8 text-xs gap-1">
        <Plus size={13} />
        <span>Nova Música</span>
      </Button>
    </div>
  </div>

  {#if songsError}
    <div
      role="alert"
      class="mb-2 px-3 py-1.5 rounded-lg border border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-between gap-2 text-xs shrink-0"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertTriangle size={14} class="shrink-0" />
        <span class="truncate font-medium">{songsError}</span>
      </div>
      <Button variant="outline" size="sm" onclick={loadSongs} class="h-7 px-2.5 text-[11px] shrink-0">
        Tentar novamente
      </Button>
    </div>
  {/if}

  {#if songTags.length > 0}
    <div class="flex items-center gap-1.5 mb-2 overflow-x-auto pb-1 text-xs shrink-0 select-none">
      <span class="text-[11px] text-muted-foreground font-semibold uppercase tracking-wider mr-1 flex items-center gap-1 shrink-0">
        <Tag size={11} /> Tags:
      </span>
      <button
        type="button"
        onclick={() => selectSongTag(null)}
        class="min-h-[26px] px-2.5 py-0.5 rounded-full text-[11px] font-medium transition-colors cursor-pointer shrink-0 {selectedSongTag === null ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'bg-muted hover:bg-muted/80 text-muted-foreground hover:text-foreground'}"
      >
        Todas
      </button>
      {#each songTags as tag (tag)}
        <button
          type="button"
          onclick={() => selectSongTag(tag)}
          class="min-h-[26px] px-2.5 py-0.5 rounded-full text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-0.5 shrink-0 {selectedSongTag === tag ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'bg-muted hover:bg-muted/80 text-muted-foreground hover:text-foreground'}"
        >
          <span>#{tag}</span>
        </button>
      {/each}
    </div>
  {/if}

  {#snippet songCard(song: SongSummary)}
    <article
      class="rounded-lg border border-border bg-card hover:bg-muted/35 transition-all flex flex-col justify-between group {viewingSong?.id === song.id ? 'ring-1 ring-primary border-primary bg-primary/10' : ''}"
    >
      <button
        type="button"
        onclick={() => handleViewSongLyrics(song.id)}
        oncontextmenu={(e) => handleSongContextMenu(e, song)}
        class="p-2.5 pb-1.5 text-left flex-1 cursor-pointer rounded-t-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
      >
        <div class="flex items-center justify-between gap-1 mb-1">
          <h3 class="text-xs font-bold text-foreground truncate">{song.title}</h3>
          <div class="flex items-center gap-1 shrink-0">
            {#if song.audio_url}
              <Badge
                variant="outline"
                class="text-[11px] py-0 px-1.5 font-mono text-emerald-700 dark:text-emerald-300 border-emerald-500/40 bg-emerald-500/10 flex items-center gap-0.5"
              >
                <Music size={10} />
                Áudio
              </Badge>
            {/if}
            {#if song.song_key}
              <Badge variant="outline" class="text-[11px] py-0 px-1.5 font-mono text-primary">
                {song.song_key}
              </Badge>
            {/if}
          </div>
        </div>

        {#if song.artist}
          <p class="text-[11px] text-muted-foreground truncate">{song.artist}</p>
        {/if}
        {#if song.preview_snippet}
          <p class="text-[11px] text-muted-foreground line-clamp-2 mt-1 italic">
            "{song.preview_snippet}"
          </p>
        {/if}
        {#if song.tags && song.tags.length > 0}
          <div class="flex flex-wrap gap-1 mt-1.5">
            {#each song.tags.slice(0, 3) as tag}
              <span class="inline-flex items-center px-1.5 py-0.5 rounded text-[11px] font-medium bg-muted text-muted-foreground border border-border/70">
                #{tag}
              </span>
            {/each}
            {#if song.tags.length > 3}
              <span class="text-[11px] text-muted-foreground self-center">+{song.tags.length - 3}</span>
            {/if}
          </div>
        {/if}
      </button>

      <div class="flex items-center justify-between gap-1.5 px-2.5 py-1.5 border-t border-border/70">
        <span class="text-[11px] text-primary font-semibold flex items-center gap-1 truncate">
          <FileText size={11} />
          {viewingSong?.id === song.id ? 'Visualizando' : 'Ver Letra'}
        </span>

        <div class="flex items-center gap-1 shrink-0">
          <button
            type="button"
            onclick={() => onaddsongtosetlist?.(song.id)}
            class="h-7 px-2 rounded-md bg-muted hover:bg-muted/80 text-[11px] text-foreground font-medium flex items-center gap-1 transition-colors cursor-pointer shrink-0"
            title="Adicionar à Setlist"
          >
            <ListPlus size={12} class="text-primary" />
            + Setlist
          </button>

          <button
            type="button"
            onclick={() => onpresentsong?.(song.id)}
            class="h-7 px-2 rounded-md bg-primary/15 hover:bg-primary/25 text-[11px] text-primary font-bold flex items-center gap-1 transition-colors cursor-pointer shrink-0"
            title="Apresentar Imediatamente"
          >
            <Play size={11} class="fill-current" />
            Ao Vivo
          </button>
        </div>
      </div>
    </article>
  {/snippet}

  {#snippet viewingSongSheet()}
    {#if viewingSong}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        onkeydown={(e) => {
          if (e.key === 'Escape') {
            e.stopPropagation();
            viewingSong = null;
          }
        }}
        oncontextmenu={(e) => viewingSong && handleViewingSongSheetContextMenu(e, viewingSong)}
        class="flex-1 min-w-0 border border-border bg-card rounded-xl flex flex-col p-3.5 h-full overflow-hidden shadow-md animate-in slide-in-from-right duration-150"
      >
        <div class="flex items-center justify-between pb-2.5 border-b border-border shrink-0 gap-2 flex-wrap">
          <div class="min-w-0 mr-2">
            <div class="flex items-center gap-2 flex-wrap">
              <h3 class="text-sm font-bold text-foreground truncate">{viewingSong.title}</h3>
              {#if viewingSong.audio_url}
                <Badge
                  variant="outline"
                  class="text-[11px] py-0 px-1.5 font-mono text-emerald-700 dark:text-emerald-300 border-emerald-500/40 bg-emerald-500/10 flex items-center gap-1"
                >
                  <Music size={10} />
                  Playback
                </Badge>
              {/if}
              {#if viewingSongSlides.length > 0}
                <Badge variant="secondary" class="text-[11px] font-mono">
                  {viewingSongSlides.length} slides
                </Badge>
              {/if}
              {#if viewingSong.song_key}
                <Badge variant="outline" class="text-[11px] py-0 px-1.5 font-mono text-primary">
                  Tom: {viewingSong.song_key}
                </Badge>
              {/if}
              {#if viewingSong.bpm}
                <span class="text-[11px] text-muted-foreground font-mono">
                  {viewingSong.bpm} BPM
                </span>
              {/if}
            </div>
            {#if viewingSong.artist || (viewingSong.tags && viewingSong.tags.length > 0)}
              <div class="flex items-center gap-2 mt-0.5 flex-wrap">
                {#if viewingSong.artist}
                  <p class="text-xs text-muted-foreground truncate">{viewingSong.artist}</p>
                {/if}
                {#if viewingSong.tags && viewingSong.tags.length > 0}
                  <div class="flex items-center gap-1 overflow-x-auto">
                    {#each viewingSong.tags as tag}
                      <span class="inline-flex items-center px-1.5 py-0.5 rounded text-[11px] bg-secondary text-secondary-foreground font-mono">
                        #{tag}
                      </span>
                    {/each}
                  </div>
                {/if}
              </div>
            {/if}
          </div>

          <div class="flex items-center gap-1.5 shrink-0">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => viewingSong && onaddsongtosetlist?.(viewingSong.id)}
              class="h-8 text-xs gap-1"
            >
              <ListPlus size={12} class="text-primary" />
              + Setlist
            </Button>
            <Button
              variant="live"
              size="sm"
              onclick={() => viewingSong && onpresentsong?.(viewingSong.id)}
              class="h-8 text-xs gap-1 font-bold"
            >
              <Play size={12} class="fill-current" />
              Apresentar Música
            </Button>
            <button
              type="button"
              onclick={() => (viewingSong = null)}
              aria-label="Fechar visualização da letra"
              class="w-8 h-8 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center cursor-pointer"
              title="Fechar visualização de letra (Esc)"
            >
              <X size={16} />
            </button>
          </div>
        </div>

        <div class="flex-1 overflow-y-auto py-3 pr-1 min-h-0">
          {#if isLoadingLyrics}
            <div class="h-32 flex items-center justify-center text-xs text-muted-foreground">
              Carregando letra...
            </div>
          {:else if viewingSongSlides.length > 0}
            <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3">
              {#each viewingSongSlides as slide, idx (slide.id || idx)}
                {@const color = getSectionColor(slide.label)}
                <article
                  oncontextmenu={(e) => viewingSong && handleViewingSongSlideContextMenu(e, viewingSong, slide, idx)}
                  class="rounded-xl border border-border bg-card overflow-hidden flex flex-col shadow-xs transition-all hover:border-primary/50"
                >
                  <div class="px-2.5 py-1.5 bg-muted/50 border-b border-border flex items-center justify-between text-[11px] gap-1">
                    <span class="uppercase font-bold text-primary flex items-center gap-1.5 truncate">
                      <span class="w-1.5 h-3.5 rounded-full {color.accent || 'bg-primary'} shrink-0" aria-hidden="true"></span>
                      <span class="w-5 h-5 rounded-full bg-muted text-foreground border border-border flex items-center justify-center text-[11px] font-mono font-bold shrink-0">
                        {idx + 1}
                      </span>
                      <span class="truncate">{slide.label || `Slide ${idx + 1}`}</span>
                    </span>
                    <button
                      type="button"
                      onclick={() => viewingSong && onpresentsong?.(viewingSong.id, idx)}
                      aria-label="Apresentar slide {slide.label || idx + 1} ao vivo"
                      class="h-6 px-2 rounded bg-primary/15 hover:bg-primary/25 text-primary text-[11px] font-bold flex items-center gap-1 shrink-0 cursor-pointer"
                      title="Apresentar este slide ao vivo"
                    >
                      <Play size={10} class="fill-current" />
                      <span>Ao Vivo</span>
                    </button>
                  </div>

                  <button
                    type="button"
                    onclick={() => viewingSong && onpresentsong?.(viewingSong.id, idx)}
                    class="aspect-video w-full p-3.5 flex flex-col items-center justify-center text-center bg-slate-950 text-slate-50 overflow-hidden cursor-pointer hover:bg-slate-900 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-inset"
                    title="Clique para projetar este slide ao vivo"
                  >
                    <div class="text-xs sm:text-[13px] font-semibold text-slate-50 leading-relaxed line-clamp-6">
                      {@html renderFormattedText(slide.content)}
                    </div>
                  </button>
                </article>
              {/each}
            </div>
          {:else if viewingSong.lyrics_raw}
            <div class="p-4 rounded-xl border border-border bg-muted/40 text-xs font-medium text-foreground leading-relaxed whitespace-pre-wrap">
              {@html renderFormattedText(viewingSong.lyrics_raw)}
            </div>
          {/if}
        </div>
      </div>
    {/if}
  {/snippet}

  <div class="flex-1 flex overflow-hidden min-h-0 w-full h-full">
    {#if viewingSong}
      <ResizablePaneGroup direction="horizontal" class="flex-1 w-full h-full min-h-0 overflow-hidden">
        <ResizablePane defaultSize={35} minSize={20} maxSize={60} class="flex flex-col p-1.5 pr-2 overflow-y-auto min-h-0">
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="grid grid-cols-1 gap-2 min-h-full" oncontextmenu={handleSongsEmptyContextMenu}>
            {#each songs as song (song.id)}
              {@render songCard(song)}
            {/each}
          </div>
        </ResizablePane>

        <ResizableHandle withHandle class="hover:bg-primary transition-colors cursor-col-resize" />

        <ResizablePane defaultSize={65} minSize={40} class="flex flex-col pl-3 min-w-0 min-h-0 overflow-hidden">
          {@render viewingSongSheet()}
        </ResizablePane>
      </ResizablePaneGroup>
    {:else}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="flex-1 overflow-y-auto flex flex-col p-1.5 min-h-0" oncontextmenu={handleSongsEmptyContextMenu}>
        {#if songs.length === 0}
          <div class="h-32 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2">
            <span>{isLoadingSongs ? 'Carregando catálogo...' : 'Nenhuma música encontrada no catálogo.'}</span>
            {#if !isLoadingSongs && songsError}
              <Button variant="outline" size="sm" onclick={loadSongs} class="h-7 text-xs">
                Recarregar
              </Button>
            {/if}
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

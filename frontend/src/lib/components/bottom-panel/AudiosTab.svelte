<script lang="ts">
  import {
    Search,
    Play,
    Pause,
    ListPlus,
    Upload,
    FolderPlus,
    Folder,
    FolderOpen,
    RefreshCw,
    X,
    Layers,
    Volume2,
    VolumeX,
    Repeat,
    AlertTriangle,
  } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import Badge from '../ui/Badge.svelte';
  import {
    fetchAudios,
    formatBytes,
    formatDuration,
    readFileAsBase64,
    uploadAudio,
  } from '../../media';
  import {
    audioCurrentTimeStore,
    audioDurationStore,
    audioErrorStore,
    audioVolumeStore,
    currentTrackStore,
    isAudioLoopingStore,
    isAudioMutedStore,
    isAudioPlayingStore,
    playTrack,
    seekTrack,
    setTrackVolume,
    stopTrack,
    togglePlayTrack,
    toggleTrackLoop,
    toggleTrackMute,
  } from '../../audioPlayer';
  import { notifyError, notifySuccess } from '../../toast';
  import type { AudioMediaItem, MediaItem } from '../../types';

  interface Props {
    active: boolean;
    onpresentitem?: (item: MediaItem, slideIndex?: number) => void;
    onadditemtosetlist?: (item: MediaItem) => void;
    onopenfolderconfig?: () => void;
    oncountchange?: (count: number) => void;
  }

  let {
    active,
    onpresentitem,
    onadditemtosetlist,
    onopenfolderconfig,
    oncountchange,
  }: Props = $props();

  let audios = $state<AudioMediaItem[]>([]);
  let audioSearch = $state('');
  let audioFolderFilter = $state<string>('all');
  let isLoadingAudios = $state(false);
  let audiosError = $state<string | null>(null);
  let isUploadingAudio = $state(false);
  let audioFileInputEl: HTMLInputElement | null = $state(null);
  let hasLoadedOnce = false;

  $effect(() => {
    oncountchange?.(audios.length);
  });

  $effect(() => {
    if (active && !hasLoadedOnce) {
      hasLoadedOnce = true;
      loadAudios();
    }
  });

  export async function loadAudios() {
    isLoadingAudios = true;
    audiosError = null;
    try {
      audios = await fetchAudios();
    } catch (err: any) {
      console.error('Erro ao carregar áudios:', err);
      audiosError = err?.message || 'Não foi possível carregar os áudios do servidor.';
    } finally {
      isLoadingAudios = false;
    }
  }

  const audioFoldersList = $derived.by(() => {
    const map = new Map<string, string>();
    for (const a of audios) {
      if (a.is_external && a.folder_id && a.folder_id !== 'app') {
        if (!map.has(a.folder_id)) {
          map.set(a.folder_id, a.folder_name || a.folder_id);
        }
      }
    }
    return Array.from(map.entries())
      .map(([id, name]) => ({ id, name }))
      .sort((a, b) => a.name.localeCompare(b.name));
  });

  const filteredAudios = $derived.by(() => {
    let list = audios;
    if (audioFolderFilter === 'app') {
      list = list.filter((a) => !a.is_external || a.folder_id === 'app');
    } else if (audioFolderFilter !== 'all') {
      list = list.filter((a) => a.folder_id === audioFolderFilter || a.folder_name === audioFolderFilter);
    }
    const q = audioSearch.trim().toLowerCase();
    if (!q) return list;
    return list.filter(
      (a) =>
        a.name.toLowerCase().includes(q) ||
        (a.folder_name && a.folder_name.toLowerCase().includes(q))
    );
  });

  function isTrackActive(audio: AudioMediaItem): boolean {
    const current = $currentTrackStore;
    if (!current) return false;
    return current.id === audio.id || current.url === audio.url;
  }

  function handlePlayAudio(audio: AudioMediaItem) {
    togglePlayTrack({
      id: audio.id,
      url: audio.url,
      title: audio.name,
      artist_or_ref: audio.folder_name || 'Áudio',
    });
  }

  function handlePresentAudio(audio: AudioMediaItem) {
    playTrack({
      id: audio.id,
      url: audio.url,
      title: audio.name,
      artist_or_ref: audio.folder_name || 'Áudio',
    });
    const item: MediaItem = {
      id: `audio-${Date.now()}`,
      title: audio.name,
      artist_or_ref: audio.folder_name || 'Áudio',
      kind: 'audio',
      slides: [{ id: 'slide-1', label: 'Áudio', content: audio.url }],
    };
    onpresentitem?.(item, 0);
  }

  function handleAddAudioToSetlist(audio: AudioMediaItem) {
    const item: MediaItem = {
      id: `audio-${Date.now()}`,
      title: audio.name,
      artist_or_ref: audio.folder_name || 'Áudio',
      kind: 'audio',
      slides: [{ id: 'slide-1', label: 'Áudio', content: audio.url }],
    };
    onadditemtosetlist?.(item);
    notifySuccess('Item Adicionado', `"${audio.name}" adicionado à setlist.`);
  }

  async function handleAudioFilesSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = Array.from(input.files || []);
    if (files.length === 0) return;

    isUploadingAudio = true;
    let uploaded = 0;
    try {
      for (const file of files) {
        const b64 = await readFileAsBase64(file);
        await uploadAudio(file.name, b64);
        uploaded++;
      }
      notifySuccess(
        uploaded > 1 ? 'Áudios Importados' : 'Áudio Importado',
        uploaded > 1
          ? `${uploaded} faixas salvas na biblioteca de áudios.`
          : `"${files[0].name}" foi salvo na biblioteca de áudios.`
      );
      await loadAudios();
    } catch (err: any) {
      console.error('Erro no upload de áudio:', err);
      notifyError('Erro no Upload', err?.message || 'Falha ao salvar arquivo de áudio.');
    } finally {
      isUploadingAudio = false;
      input.value = '';
    }
  }
</script>

<div role="tabpanel" id="panel-audios" aria-labelledby="tab-audios" class="flex-1 flex flex-col overflow-hidden gap-2">
  <h2 class="sr-only">Biblioteca de Áudios e Reprodutor</h2>

  <!-- Player Bar Global (Sincronizado com audioPlayer.ts) -->
  <div class="p-2.5 rounded-xl bg-card border border-indigo-500/35 shadow-sm flex items-center justify-between gap-3 shrink-0 flex-wrap">
    <div class="flex items-center gap-3 min-w-0 flex-1 basis-[180px] max-w-xs">
      <div class="w-10 h-10 rounded-xl bg-indigo-500/15 border border-indigo-500/35 text-indigo-700 dark:text-indigo-300 flex items-center justify-center shrink-0">
        <Volume2 size={20} class={$isAudioPlayingStore ? 'animate-pulse' : ''} />
      </div>
      <div class="min-w-0">
        <p class="text-xs font-bold text-foreground truncate">
          {$currentTrackStore ? $currentTrackStore.title : 'Nenhum áudio selecionado'}
        </p>
        <p class="text-[11px] text-muted-foreground truncate">
          {$currentTrackStore ? ($isAudioPlayingStore ? 'Reproduzindo' : 'Pausado') : 'Escolha uma faixa abaixo para tocar'}
        </p>
      </div>
    </div>

    <!-- Controles de Transporte e SeekBar -->
    <div class="flex-1 flex flex-col items-center gap-1 min-w-[240px] max-w-xl">
      <div class="flex items-center gap-2">
        <button
          type="button"
          onclick={() => toggleTrackLoop()}
          aria-pressed={$isAudioLoopingStore}
          aria-label={$isAudioLoopingStore ? 'Desativar repetição contínua' : 'Ativar repetição contínua'}
          class="w-8 h-8 rounded-lg flex items-center justify-center transition-colors cursor-pointer {$isAudioLoopingStore ? 'text-indigo-700 dark:text-indigo-300 bg-indigo-500/20' : 'text-muted-foreground hover:text-foreground hover:bg-muted'}"
          title={$isAudioLoopingStore ? 'Repetir ativado' : 'Repetir desativado'}
        >
          <Repeat size={15} />
        </button>

        <button
          type="button"
          onclick={() => {
            if ($currentTrackStore) {
              togglePlayTrack();
            } else if (filteredAudios.length > 0) {
              handlePlayAudio(filteredAudios[0]);
            }
          }}
          disabled={!$currentTrackStore && filteredAudios.length === 0}
          aria-label={$isAudioPlayingStore ? 'Pausar áudio' : 'Reproduzir áudio'}
          class="w-9 h-9 rounded-full bg-indigo-600 hover:bg-indigo-500 text-white flex items-center justify-center shadow-md transition-transform hover:scale-105 cursor-pointer disabled:opacity-40"
          title={$isAudioPlayingStore ? 'Pausar áudio' : 'Reproduzir áudio'}
        >
          {#if $isAudioPlayingStore}
            <Pause size={16} class="fill-current" />
          {:else}
            <Play size={16} class="fill-current ml-0.5" />
          {/if}
        </button>

        <button
          type="button"
          onclick={() => stopTrack()}
          disabled={!$currentTrackStore}
          aria-label="Parar reprodução de áudio"
          class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer disabled:opacity-40"
          title="Parar reprodução"
        >
          <span class="w-3.5 h-3.5 bg-current rounded-xs block"></span>
        </button>
      </div>

      <div class="w-full flex items-center gap-2 text-[11px] font-mono text-muted-foreground">
        <span class="w-11 text-right">{formatDuration($audioCurrentTimeStore)}</span>
        <input
          type="range"
          min="0"
          max={$audioDurationStore || 100}
          step="0.5"
          value={$audioCurrentTimeStore}
          aria-label="Posição da reprodução de áudio"
          oninput={(e) => seekTrack(Number((e.target as HTMLInputElement).value))}
          class="flex-1 h-2 bg-muted rounded-lg cursor-pointer accent-indigo-600"
        />
        <span class="w-11">{formatDuration($audioDurationStore)}</span>
      </div>
    </div>

    <!-- Controle de Volume -->
    <div class="flex items-center gap-1.5 shrink-0">
      <button
        type="button"
        onclick={() => toggleTrackMute()}
        aria-pressed={$isAudioMutedStore}
        aria-label={$isAudioMutedStore ? 'Ativar som' : 'Silenciar áudio'}
        class="w-8 h-8 rounded-lg flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
        title={$isAudioMutedStore ? 'Ativar som' : 'Silenciar'}
      >
        {#if $isAudioMutedStore || $audioVolumeStore === 0}
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
        value={$isAudioMutedStore ? 0 : $audioVolumeStore}
        aria-label="Volume do reprodutor de áudio"
        oninput={(e) => setTrackVolume(Number((e.target as HTMLInputElement).value))}
        class="w-20 h-2 bg-muted rounded-lg cursor-pointer accent-indigo-600"
        title="Volume"
      />
    </div>
  </div>

  {#if $audioErrorStore}
    <div
      role="alert"
      class="px-3 py-1.5 rounded-lg border border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-between gap-2 text-xs shrink-0"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertTriangle size={14} class="shrink-0" />
        <span class="truncate font-medium">{$audioErrorStore}</span>
      </div>
      <button
        type="button"
        onclick={() => audioErrorStore.set(null)}
        aria-label="Dispensar aviso de áudio"
        class="w-7 h-7 rounded flex items-center justify-center hover:bg-destructive/15 cursor-pointer"
      >
        <X size={13} />
      </button>
    </div>
  {/if}

  <!-- Barra de Busca e Upload de Áudios -->
  <div class="flex items-center justify-between gap-2 pb-2 border-b border-border/60 shrink-0 flex-wrap">
    <div class="flex items-center gap-2 flex-1 min-w-[200px] max-w-md">
      <div class="relative flex-1">
        <Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <input
          type="text"
          bind:value={audioSearch}
          aria-label="Buscar áudios por nome"
          placeholder="Buscar áudios por nome..."
          class="w-full h-8 pl-8 pr-8 rounded-lg bg-background border border-input text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
        />
        {#if audioSearch}
          <button
            type="button"
            onclick={() => (audioSearch = '')}
            aria-label="Limpar busca de áudios"
            class="absolute right-1 top-1/2 -translate-y-1/2 w-6 h-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"
          >
            <X size={12} />
          </button>
        {/if}
      </div>
      <Badge variant="secondary" class="text-[11px] font-mono py-0.5 px-2 shrink-0">
        {filteredAudios.length} {filteredAudios.length === 1 ? 'faixa' : 'faixas'}
      </Badge>
    </div>

    <div class="flex items-center gap-1.5 shrink-0 flex-wrap">
      <input
        type="file"
        bind:this={audioFileInputEl}
        onchange={handleAudioFilesSelected}
        accept="audio/mp3,audio/wav,audio/m4a,audio/aac,audio/ogg,audio/flac,.mp3,.wav,.m4a,.aac,.ogg,.flac"
        multiple
        class="hidden"
      />
      <Button
        variant="outline"
        size="sm"
        onclick={loadAudios}
        aria-label="Recarregar áudios"
        title="Recarregar áudios"
        class="h-8 w-8 p-0"
      >
        <RefreshCw size={13} class={isLoadingAudios ? 'animate-spin' : ''} />
      </Button>
      <Button
        variant="outline"
        size="sm"
        onclick={onopenfolderconfig}
        class="h-8 text-xs gap-1.5"
        title="Mapear ou gerenciar pastas locais de áudio"
      >
        <FolderPlus size={13} class="text-indigo-700 dark:text-indigo-300" />
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

  {#if audiosError}
    <div
      role="alert"
      class="px-3 py-1.5 rounded-lg border border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-between gap-2 text-xs shrink-0"
    >
      <div class="flex items-center gap-2 min-w-0">
        <AlertTriangle size={14} class="shrink-0" />
        <span class="truncate font-medium">{audiosError}</span>
      </div>
      <Button variant="outline" size="sm" onclick={loadAudios} class="h-7 px-2.5 text-[11px] shrink-0">
        Tentar novamente
      </Button>
    </div>
  {/if}

  <!-- Filtro de Pastas de Áudio -->
  <div class="flex items-center gap-1.5 overflow-x-auto pb-1 text-xs shrink-0 select-none">
    <button
      type="button"
      onclick={() => (audioFolderFilter = 'all')}
      class="min-h-[28px] px-2.5 py-1 rounded-lg border text-[11px] font-medium transition-colors cursor-pointer shrink-0 flex items-center gap-1 {audioFolderFilter === 'all' ? 'bg-indigo-600 border-indigo-500 text-white' : 'bg-muted/60 border-border text-muted-foreground hover:text-foreground'}"
    >
      <Layers size={11} /> Todos
    </button>
    <button
      type="button"
      onclick={() => (audioFolderFilter = 'app')}
      class="min-h-[28px] px-2.5 py-1 rounded-lg border text-[11px] font-medium transition-colors cursor-pointer shrink-0 flex items-center gap-1 {audioFolderFilter === 'app' ? 'bg-indigo-600 border-indigo-500 text-white' : 'bg-muted/60 border-border text-muted-foreground hover:text-foreground'}"
    >
      <Folder size={11} /> Pasta do App
    </button>
    {#each audioFoldersList as af (af.id)}
      <button
        type="button"
        onclick={() => (audioFolderFilter = af.id)}
        class="min-h-[28px] px-2.5 py-1 rounded-lg border text-[11px] font-medium transition-colors cursor-pointer shrink-0 flex items-center gap-1 {audioFolderFilter === af.id ? 'bg-indigo-600 border-indigo-500 text-white' : 'bg-muted/60 border-border text-muted-foreground hover:text-foreground'}"
      >
        <FolderOpen size={11} /> {af.name}
      </button>
    {/each}
  </div>

  <!-- Lista de Áudios -->
  <div class="flex-1 overflow-y-auto pr-1">
    {#if filteredAudios.length === 0}
      <div class="h-44 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2">
        <Volume2 size={28} class="opacity-45" />
        <p>{isLoadingAudios ? 'Carregando áudios...' : 'Nenhum áudio encontrado na biblioteca.'}</p>
        {#if !isLoadingAudios}
          <Button
            variant="outline"
            size="sm"
            onclick={() => audioFileInputEl?.click()}
            class="h-8 text-xs gap-1 mt-1"
          >
            <Upload size={12} />
            <span>Importar primeiro áudio (MP3, WAV, M4A)</span>
          </Button>
        {/if}
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2">
        {#each filteredAudios as aud (aud.id)}
          {@const isThisSelected = isTrackActive(aud)}
          {@const isThisPlaying = isThisSelected && $isAudioPlayingStore}
          <article
            class="p-2.5 rounded-xl border transition-all flex items-center justify-between gap-2.5 group {isThisSelected ? 'border-indigo-500 bg-indigo-500/10' : 'border-border bg-card hover:bg-muted/50'}"
          >
            <div class="flex items-center gap-2.5 min-w-0 flex-1">
              <button
                type="button"
                onclick={() => handlePlayAudio(aud)}
                aria-label="{isThisPlaying ? 'Pausar' : 'Reproduzir'} {aud.name}"
                class="w-9 h-9 rounded-xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105 cursor-pointer {isThisPlaying ? 'bg-indigo-600 text-white' : 'bg-indigo-500/15 text-indigo-700 dark:text-indigo-300 hover:bg-indigo-600 hover:text-white'}"
                title={isThisPlaying ? 'Pausar' : 'Reproduzir'}
              >
                {#if isThisPlaying}
                  <Pause size={15} class="fill-current" />
                {:else}
                  <Play size={15} class="fill-current ml-0.5" />
                {/if}
              </button>

              <div class="min-w-0">
                <h3 class="text-xs font-semibold text-foreground truncate" title={aud.name}>
                  {aud.name}
                </h3>
                <div class="flex items-center gap-1.5 text-[11px] text-muted-foreground font-mono">
                  <span>{formatBytes(aud.size_bytes)}</span>
                  {#if aud.folder_name}
                    <span>•</span>
                    <span class="text-indigo-700 dark:text-indigo-300 truncate max-w-[120px]">{aud.folder_name}</span>
                  {/if}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-1 shrink-0">
              <button
                type="button"
                onclick={() => handlePresentAudio(aud)}
                class="h-7 px-2.5 rounded-lg bg-indigo-500/15 hover:bg-indigo-600 text-indigo-700 dark:text-indigo-200 hover:text-white flex items-center gap-1 text-[11px] font-semibold transition-colors cursor-pointer"
                title="Apresentar Áudio Imediatamente"
              >
                <Play size={11} class="fill-current" />
                <span class="hidden sm:inline">Apresentar</span>
              </button>

              <button
                type="button"
                onclick={() => handleAddAudioToSetlist(aud)}
                aria-label="Adicionar áudio {aud.name} à Setlist"
                class="h-7 px-2 rounded-lg bg-muted hover:bg-muted/80 text-foreground flex items-center gap-1 text-[11px] font-medium transition-colors cursor-pointer"
                title="Adicionar à Setlist"
              >
                <ListPlus size={13} />
                <span class="hidden sm:inline">+ Setlist</span>
              </button>
            </div>
          </article>
        {/each}
      </div>
    {/if}
  </div>
</div>

<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { Check, ChevronDown, Music, Play, Square, Trash2, Upload, X } from 'lucide-svelte';
  import { fetchAudios, uploadAudio } from '$lib/media';
  import { toast } from '$lib/toast';
  import type { AudioMediaItem } from '$lib/types';
  import type { SongEditorMode } from './types';

  interface Props {
    mode: SongEditorMode;
    title?: string;
    artist?: string;
    composer?: string;
    songKey?: string;
    bpm?: number | undefined;
    tags?: string[];
    audioUrl?: string;
    titleError?: string;
    ontitleinput?: () => void;
  }

  let {
    mode,
    title = $bindable(''),
    artist = $bindable(''),
    composer = $bindable(''),
    songKey = $bindable(''),
    bpm = $bindable(undefined),
    tags = $bindable([]),
    audioUrl = $bindable(''),
    titleError = '',
    ontitleinput,
  }: Props = $props();

  const MAX_AUDIO_BYTES = 50 * 1024 * 1024;

  const fieldClass =
    'w-full h-8 px-2.5 bg-background border border-border rounded-md text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/40 transition-colors';

  let availableAudios = $state<AudioMediaItem[]>([]);
  let isUploadingAudio = $state(false);
  let isPreviewPlaying = $state(false);
  let audioLoadError = $state('');
  let tagInput = $state('');
  let previewAudioEl: HTMLAudioElement | null = null;

  function stopPreview() {
    if (previewAudioEl) {
      previewAudioEl.pause();
      previewAudioEl.currentTime = 0;
    }
    isPreviewPlaying = false;
  }

  function toggleAudioPreview() {
    if (!audioUrl) return;

    if (isPreviewPlaying && previewAudioEl) {
      previewAudioEl.pause();
      isPreviewPlaying = false;
      return;
    }

    if (!previewAudioEl) {
      previewAudioEl = new Audio(audioUrl);
      previewAudioEl.onended = () => {
        isPreviewPlaying = false;
      };
      previewAudioEl.onerror = () => {
        isPreviewPlaying = false;
        toast.error('Não foi possível reproduzir o áudio selecionado.');
      };
    } else {
      previewAudioEl.src = audioUrl;
    }

    void previewAudioEl
      .play()
      .then(() => {
        isPreviewPlaying = true;
      })
      .catch(() => {
        isPreviewPlaying = false;
        toast.error('Erro ao reproduzir preview de áudio.');
      });
  }

  async function loadAudios() {
    try {
      availableAudios = await fetchAudios();
      audioLoadError = '';
    } catch (err) {
      console.error('Falha ao listar áudios:', err);
      audioLoadError = 'Não foi possível listar os áudios disponíveis.';
    }
  }

  function readFileAsDataUrl(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result as string);
      reader.onerror = () => reject(reader.error ?? new Error('Falha ao ler o arquivo.'));
      reader.readAsDataURL(file);
    });
  }

  async function handleAudioFileUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    input.value = '';
    if (!file) return;

    if (file.size > MAX_AUDIO_BYTES) {
      toast.error('Áudio maior que 50 MB. Escolha um arquivo menor.');
      return;
    }

    isUploadingAudio = true;
    try {
      const dataUrl = await readFileAsDataUrl(file);
      const base64 = dataUrl.split(',')[1] ?? '';
      const result = await uploadAudio(file.name, base64);
      stopPreview();
      audioUrl = result.url;
      await loadAudios();
      toast.success(`Áudio "${file.name}" carregado e vinculado!`);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Erro desconhecido';
      toast.error('Falha ao enviar áudio: ' + message);
    } finally {
      isUploadingAudio = false;
    }
  }

  function addTag(tagText: string) {
    const clean = tagText.trim();
    if (clean && !tags.some((tag) => tag.toLowerCase() === clean.toLowerCase())) {
      tags = [...tags, clean];
    }
    tagInput = '';
  }

  function removeTag(tagToRemove: string) {
    tags = tags.filter((tag) => tag !== tagToRemove);
  }

  function handleTagKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ',') {
      e.preventDefault();
      addTag(tagInput);
    } else if (e.key === 'Backspace' && !tagInput && tags.length > 0) {
      tags = tags.slice(0, -1);
    }
  }

  function removeAudio() {
    stopPreview();
    audioUrl = '';
  }

  onMount(() => {
    void loadAudios();
  });

  onDestroy(stopPreview);
</script>

<div class="p-4 border-b border-border bg-muted/20 space-y-3 shrink-0">
  {#if mode === 'song'}
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <div>
        <label for="song-title-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
          Título da Música <span class="text-emerald-700 dark:text-emerald-300">*</span>
        </label>
        <input
          id="song-title-input"
          type="text"
          bind:value={title}
          oninput={ontitleinput}
          placeholder="Ex: Porque Ele Vive"
          class={fieldClass}
          aria-invalid={Boolean(titleError)}
          aria-describedby={titleError ? 'song-title-error' : undefined}
        />
        {#if titleError}
          <p id="song-title-error" role="alert" class="mt-1 text-[11px] text-red-700 dark:text-red-300">
            {titleError}
          </p>
        {/if}
      </div>

      <div>
        <label for="song-artist-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
          Artista / Banda / Autor
        </label>
        <input
          id="song-artist-input"
          type="text"
          bind:value={artist}
          placeholder="Ex: Harpa Cristã / Matt Maher"
          class={fieldClass}
        />
      </div>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
      <div>
        <label for="song-key-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
          Tom (Key)
        </label>
        <input
          id="song-key-input"
          type="text"
          bind:value={songKey}
          placeholder="Ex: G, Dm, C#"
          class={fieldClass}
        />
      </div>

      <div>
        <label for="song-bpm-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
          BPM (Andamento)
        </label>
        <input
          id="song-bpm-input"
          type="number"
          min="20"
          max="400"
          bind:value={bpm}
          placeholder="Ex: 72"
          class={fieldClass}
        />
      </div>

      <div>
        <label for="song-composer-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
          Compositor
        </label>
        <input
          id="song-composer-input"
          type="text"
          bind:value={composer}
          placeholder="Ex: Bill Gaither"
          class={fieldClass}
        />
      </div>
    </div>

    <div>
      <label for="song-tags-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
        Grupos e Tags (Ex: Louvor, Harpa Cristã, Comunhão, Celebração)
      </label>
      <div
        class="flex flex-wrap items-center gap-1.5 p-1.5 bg-background border border-border rounded-md min-h-[34px]"
      >
        {#each tags as tag (tag)}
          <span
            class="inline-flex items-center gap-1 pl-2 pr-1 py-0.5 rounded-full bg-emerald-500/15 border border-emerald-600/35 text-emerald-800 dark:text-emerald-300 text-[11px] font-medium"
          >
            <span>{tag}</span>
            <button
              type="button"
              onclick={() => removeTag(tag)}
              class="w-6 h-6 rounded-full hover:bg-red-500/15 hover:text-red-700 dark:hover:text-red-300 flex items-center justify-center transition-colors cursor-pointer"
              aria-label={`Remover tag ${tag}`}
              title="Remover tag"
            >
              <X size={12} />
            </button>
          </span>
        {/each}
        <input
          id="song-tags-input"
          type="text"
          bind:value={tagInput}
          onkeydown={handleTagKeydown}
          onblur={() => tagInput.trim() && addTag(tagInput)}
          placeholder={tags.length === 0
            ? 'Digite tags separadas por Enter ou vírgula...'
            : 'Adicionar tag...'}
          class="flex-1 min-w-[120px] min-h-[24px] bg-transparent border-0 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-0 px-0.5 py-1"
        />
      </div>
    </div>

    <div class="p-3 bg-muted/40 border border-border rounded-lg space-y-2">
      <div class="flex items-center justify-between gap-2">
        <label for="song-audio-select" class="text-[11px] font-semibold text-foreground flex items-center gap-1.5">
          <Music size={13} class="text-emerald-700 dark:text-emerald-300" aria-hidden="true" />
          Áudio / Playback Vinculado (Opcional)
        </label>
        {#if audioUrl}
          <span class="text-[11px] text-emerald-800 dark:text-emerald-300 font-mono flex items-center gap-1">
            <Check size={11} aria-hidden="true" /> Áudio configurado
          </span>
        {/if}
      </div>

      <div class="flex flex-wrap items-center gap-2">
        <div class="relative flex-1 min-w-[160px]">
          <select
            id="song-audio-select"
            bind:value={audioUrl}
            onchange={stopPreview}
            class="w-full h-8 pl-2.5 pr-8 bg-background border border-border rounded-md text-xs text-foreground focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/40 transition-colors appearance-none cursor-pointer"
          >
            <option value="">Nenhum playback vinculado</option>
            {#if audioUrl && !availableAudios.some((audio) => audio.url === audioUrl)}
              <option value={audioUrl}>{audioUrl.split('/').pop() || audioUrl}</option>
            {/if}
            {#each availableAudios as audio (audio.id)}
              <option value={audio.url}>{audio.name}</option>
            {/each}
          </select>
          <ChevronDown
            size={14}
            class="absolute right-2.5 top-2 text-muted-foreground pointer-events-none"
            aria-hidden="true"
          />
        </div>

        {#if audioUrl}
          <button
            type="button"
            onclick={toggleAudioPreview}
            class="h-8 px-2.5 bg-secondary hover:bg-muted border border-border rounded-md text-xs font-medium text-foreground flex items-center gap-1.5 transition-colors cursor-pointer shrink-0"
            title={isPreviewPlaying ? 'Pausar preview' : 'Ouvir preview'}
            aria-label={isPreviewPlaying ? 'Pausar preview do áudio' : 'Ouvir preview do áudio'}
          >
            {#if isPreviewPlaying}
              <Square size={12} class="text-amber-700 dark:text-amber-300 fill-current" aria-hidden="true" />
              <span>Pausar</span>
            {:else}
              <Play size={12} class="text-emerald-700 dark:text-emerald-300 fill-current" aria-hidden="true" />
              <span>Ouvir</span>
            {/if}
          </button>

          <button
            type="button"
            onclick={removeAudio}
            class="h-8 w-8 bg-secondary hover:bg-red-500/15 border border-border rounded-md text-muted-foreground hover:text-red-700 dark:hover:text-red-300 flex items-center justify-center transition-colors cursor-pointer shrink-0"
            title="Remover áudio vinculado"
            aria-label="Remover áudio vinculado"
          >
            <Trash2 size={13} />
          </button>
        {/if}

        <label
          class="h-8 px-3 bg-secondary hover:bg-muted border border-border rounded-md text-xs font-medium text-foreground flex items-center gap-1.5 transition-colors cursor-pointer shrink-0 focus-within:ring-2 focus-within:ring-ring {isUploadingAudio
            ? 'opacity-50 pointer-events-none'
            : ''}"
          title="Enviar arquivo de áudio (MP3, WAV, etc.)"
        >
          <Upload size={12} aria-hidden="true" />
          <span>{isUploadingAudio ? 'Enviando...' : 'Enviar Áudio'}</span>
          <input
            type="file"
            accept="audio/*,.mp3,.wav,.m4a,.aac,.ogg,.flac"
            class="sr-only"
            aria-label="Enviar arquivo de áudio para vincular"
            onchange={handleAudioFileUpload}
            disabled={isUploadingAudio}
          />
        </label>
      </div>

      {#if audioLoadError}
        <div
          role="alert"
          class="text-[11px] text-red-700 dark:text-red-300 flex flex-wrap items-center gap-2"
        >
          <span>{audioLoadError}</span>
          <button
            type="button"
            onclick={() => void loadAudios()}
            class="h-7 px-2 rounded-md border border-border bg-background hover:bg-muted text-[11px] font-medium text-foreground transition-colors cursor-pointer"
          >
            Tentar novamente
          </button>
        </div>
      {/if}

      {#if audioUrl}
        <div class="text-[11px] text-muted-foreground truncate flex items-center gap-1.5">
          <span class="font-mono">Arquivo:</span>
          <span class="font-mono text-foreground truncate">{audioUrl}</span>
        </div>
      {/if}
    </div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <div>
        <label for="text-title-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
          Título da Apresentação <span class="text-amber-700 dark:text-amber-300">*</span>
        </label>
        <input
          id="text-title-input"
          type="text"
          bind:value={title}
          oninput={ontitleinput}
          placeholder="Ex: Avisos e Boas-Vindas"
          class={fieldClass}
          aria-invalid={Boolean(titleError)}
          aria-describedby={titleError ? 'text-title-error' : undefined}
        />
        {#if titleError}
          <p id="text-title-error" role="alert" class="mt-1 text-[11px] text-red-700 dark:text-red-300">
            {titleError}
          </p>
        {/if}
      </div>

      <div>
        <label for="text-subtitle-input" class="block text-[11px] font-medium text-muted-foreground mb-1">
          Subtítulo ou Categoria (Opcional)
        </label>
        <input
          id="text-subtitle-input"
          type="text"
          bind:value={artist}
          placeholder="Ex: Comunicação da Semana"
          class={fieldClass}
        />
      </div>
    </div>
  {/if}
</div>

<script lang="ts">
  import { onMount } from 'svelte';
  import { X, Save, Music, AlertCircle, Sparkles, Check, Hash } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import { parseRawLyrics, getSectionColor } from '../songParser';
  import type { Song, Slide } from '../types';

  interface Props {
    open: boolean;
    initialSong?: Partial<Song> | null;
    onclose: () => void;
    onsave?: (savedSong: Song) => void;
  }

  let { open = false, initialSong = null, onclose, onsave }: Props = $props();

  let id = $state('');
  let title = $state('');
  let artist = $state('');
  let composer = $state('');
  let songKey = $state('');
  let bpm = $state<number | undefined>(undefined);
  let lyricsRaw = $state('');
  let addToSetlist = $state(true);
  let isSaving = $state(false);
  let errorMessage = $state('');
  let prevOpen = false;

  // Sincroniza estado apenas na abertura do modal
  $effect(() => {
    if (open && !prevOpen) {
      if (initialSong) {
        id = initialSong.id || `song-${Date.now()}`;
        title = initialSong.title || '';
        artist = initialSong.artist || '';
        composer = initialSong.composer || '';
        songKey = initialSong.song_key || '';
        bpm = initialSong.bpm;
        lyricsRaw = initialSong.lyrics_raw || '';
      } else {
        id = `song-${Date.now()}`;
        title = '';
        artist = '';
        composer = '';
        songKey = '';
        bpm = undefined;
        lyricsRaw = `## Verso 1
Primeira estrofe da canção
Segunda linha aqui

Outro slide deste verso
Linha vazia separou

## Refrão
Porque Ele vive, posso crer no amanhã
Porque Ele vive, temor não há`;
      }
      errorMessage = '';
    }
    prevOpen = open;
  });

  // Geração reativa dos slides e seções
  const parsed = $derived.by(() => {
    return parseRawLyrics(lyricsRaw);
  });

  function handleKeydown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.stopPropagation();
      onclose();
    }
    if ((e.metaKey || e.ctrlKey) && e.key === 's') {
      e.preventDefault();
      saveSong();
    }
  }

  async function saveSong() {
    if (!title.trim()) {
      errorMessage = 'O título da música é obrigatório.';
      return;
    }

    isSaving = true;
    errorMessage = '';

    const songPayload: Song = {
      id: id || `song-${Date.now()}`,
      title: title.trim(),
      artist: artist.trim(),
      album: undefined,
      composer: composer.trim() || undefined,
      song_key: songKey.trim() || undefined,
      bpm: bpm ? Number(bpm) : undefined,
      sections: parsed.sections,
      lyrics_raw: lyricsRaw,
    };

    try {
      const resp = await fetch('/api/songs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          song: songPayload,
          add_to_setlist: addToSetlist,
        }),
      });

      if (!resp.ok) {
        const errorText = await resp.text();
        throw new Error(errorText || 'Erro ao salvar música');
      }

      const saved: Song = await resp.json();
      onsave?.(saved);
      onclose();
    } catch (err: any) {
      errorMessage = err.message || 'Falha na conexão ao salvar';
    } finally {
      isSaving = false;
    }
  }

  function insertSectionTag(tag: string) {
    const textToInsert = `\n\n## ${tag}\n`;
    lyricsRaw += textToInsert;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-3 sm:p-6 animate-in fade-in duration-200"
    role="dialog"
    aria-modal="true"
    aria-label="Editor de Música"
    tabindex="-1"
    onkeydown={(e) => {
      if (e.key === 'Escape') onclose();
    }}
  >
    <!-- Modal Container (95% da tela) -->
    <div
      class="w-[95vw] h-[95vh] max-w-7xl bg-zinc-950 border border-zinc-800 rounded-xl shadow-2xl flex flex-col overflow-hidden text-zinc-100"
    >
      <!-- Cabeçalho do Modal -->
      <div
        class="h-14 px-5 border-b border-zinc-800 bg-zinc-900/60 flex items-center justify-between shrink-0"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-8 h-8 rounded-lg bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400"
          >
            <Music size={18} />
          </div>
          <div>
            <h2 class="text-sm sm:text-base font-semibold tracking-tight text-white flex items-center gap-2">
              {title ? title : 'Nova Música'}
              <Badge variant="outline" class="text-[10px] text-zinc-400 font-mono">
                {parsed.slides.length} slides
              </Badge>
            </h2>
            <p class="text-[11px] text-zinc-400">
              Editor de letras e partes no padrão Holyrics e ProPresenter
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2">
          <span class="text-[11px] text-zinc-500 hidden md:inline">
            Salvar: <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-300 font-mono text-[10px]">⌘S</kbd> / <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-300 font-mono text-[10px]">Ctrl+S</kbd>
          </span>
          <button
            type="button"
            onclick={onclose}
            class="w-8 h-8 rounded-lg text-zinc-400 hover:text-white hover:bg-zinc-800/80 flex items-center justify-center transition-colors cursor-pointer"
            aria-label="Fechar"
          >
            <X size={18} />
          </button>
        </div>
      </div>

      {#if errorMessage}
        <div
          class="px-5 py-2.5 bg-red-950/40 border-b border-red-800/40 text-red-300 text-xs flex items-center gap-2 shrink-0"
        >
          <AlertCircle size={15} class="shrink-0 text-red-400" />
          <span>{errorMessage}</span>
        </div>
      {/if}

      <!-- Conteúdo Dividido em 2 Painéis -->
      <div class="flex-1 flex flex-col lg:flex-row overflow-hidden divide-y lg:divide-y-0 lg:divide-x divide-zinc-800/80">
        <!-- PAINEL ESQUERDO: Metadados + Editor de Texto -->
        <div class="w-full lg:w-1/2 flex flex-col overflow-hidden bg-zinc-950">
          <!-- Metadados -->
          <div class="p-4 border-b border-zinc-800/80 bg-zinc-900/20 space-y-3 shrink-0">
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <div>
                <label for="song-title-input" class="block text-[11px] font-medium text-zinc-400 mb-1">
                  Título da Música <span class="text-emerald-400">*</span>
                </label>
                <input
                  id="song-title-input"
                  type="text"
                  bind:value={title}
                  placeholder="Ex: Porque Ele Vive"
                  class="w-full h-8 px-2.5 bg-zinc-900 border border-zinc-700/80 rounded-md text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                />
              </div>

              <div>
                <label for="song-artist-input" class="block text-[11px] font-medium text-zinc-400 mb-1">
                  Artista / Banda / Autor
                </label>
                <input
                  id="song-artist-input"
                  type="text"
                  bind:value={artist}
                  placeholder="Ex: Harpa Cristã / Matt Maher"
                  class="w-full h-8 px-2.5 bg-zinc-900 border border-zinc-700/80 rounded-md text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                />
              </div>
            </div>

            <div class="grid grid-cols-3 gap-3">
              <div>
                <label for="song-key-input" class="block text-[11px] font-medium text-zinc-400 mb-1">
                  Tom (Key)
                </label>
                <input
                  id="song-key-input"
                  type="text"
                  bind:value={songKey}
                  placeholder="Ex: G, Dm, C#"
                  class="w-full h-8 px-2.5 bg-zinc-900 border border-zinc-700/80 rounded-md text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                />
              </div>

              <div>
                <label for="song-bpm-input" class="block text-[11px] font-medium text-zinc-400 mb-1">
                  BPM (Andamento)
                </label>
                <input
                  id="song-bpm-input"
                  type="number"
                  bind:value={bpm}
                  placeholder="Ex: 72"
                  class="w-full h-8 px-2.5 bg-zinc-900 border border-zinc-700/80 rounded-md text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                />
              </div>

              <div>
                <label for="song-composer-input" class="block text-[11px] font-medium text-zinc-400 mb-1">
                  Compositor
                </label>
                <input
                  id="song-composer-input"
                  type="text"
                  bind:value={composer}
                  placeholder="Ex: Bill Gaither"
                  class="w-full h-8 px-2.5 bg-zinc-900 border border-zinc-700/80 rounded-md text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                />
              </div>
            </div>
          </div>

          <!-- Barra de Atalhos de Tags de Seções -->
          <div class="px-4 py-2 border-b border-zinc-800/60 bg-zinc-900/30 flex items-center gap-1.5 overflow-x-auto text-xs shrink-0">
            <span class="text-[11px] text-zinc-500 font-medium shrink-0 flex items-center gap-1 mr-1">
              <Hash size={13} class="text-zinc-500" /> Inserir Parte:
            </span>
            {#each ['Verso 1', 'Verso 2', 'Verso 3', 'Pré-Refrão', 'Refrão', 'Ponte', 'Intro', 'Final'] as tag}
              <button
                type="button"
                onclick={() => insertSectionTag(tag)}
                class="px-2 py-0.5 rounded bg-zinc-800/80 hover:bg-zinc-700 border border-zinc-700/50 text-[11px] text-zinc-300 font-medium transition-colors shrink-0 cursor-pointer"
              >
                + {tag}
              </button>
            {/each}
          </div>

          <!-- Dica de Sintaxe (Holyrics style) -->
          <div class="px-4 py-1.5 bg-zinc-900/50 text-[11px] text-zinc-400 flex items-center justify-between shrink-0 border-b border-zinc-800/40">
            <span>Use <code>## Verso 1</code> ou <code>## Refrão</code> para nomear partes. Linha em branco separa slides.</span>
            <span class="text-zinc-500 font-mono text-[10px]">{lyricsRaw.length} chars</span>
          </div>

          <!-- Textarea do Editor de Letra -->
          <div class="flex-1 p-4 overflow-hidden flex flex-col">
            <textarea
              bind:value={lyricsRaw}
              placeholder="Digite ou cole a letra da música aqui..."
              class="w-full flex-1 p-3 bg-zinc-900/70 border border-zinc-800 rounded-lg text-sm text-zinc-100 font-mono leading-relaxed resize-none focus:outline-none focus:border-emerald-500/80 focus:ring-1 focus:ring-emerald-500/50"
            ></textarea>
          </div>
        </div>

        <!-- PAINEL DIREITO: Prévia ao Vivo dos Slides (ProPresenter Style) -->
        <div class="w-full lg:w-1/2 flex flex-col overflow-hidden bg-zinc-950/60">
          <div
            class="h-10 px-4 border-b border-zinc-800/80 bg-zinc-900/30 flex items-center justify-between shrink-0"
          >
            <span class="text-xs font-semibold text-zinc-300 flex items-center gap-2">
              <Sparkles size={14} class="text-emerald-400" />
              Prévia dos Slides ({parsed.slides.length})
            </span>
            <span class="text-[11px] text-zinc-500 font-medium">Projeção 16:9 em tempo real</span>
          </div>

          <!-- Grid de Slides (Scrollable) -->
          <div class="flex-1 p-4 overflow-y-auto space-y-4">
            {#if parsed.slides.length === 0}
              <div
                class="h-64 flex flex-col items-center justify-center text-zinc-500 text-xs border border-dashed border-zinc-800 rounded-xl"
              >
                <Music size={28} class="mb-2 opacity-40 text-emerald-400" />
                <p>Nenhum slide gerado ainda.</p>
                <p class="text-[11px] text-zinc-600 mt-1">
                  Digite a letra no painel esquerdo para visualizar os slides aqui.
                </p>
              </div>
            {:else}
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
                {#each parsed.slides as slide, idx (slide.id)}
                  {@const color = getSectionColor(slide.label)}
                  <div
                    class="group relative aspect-video bg-zinc-900/90 rounded-lg border border-zinc-800 overflow-hidden flex flex-col shadow-md transition-all hover:border-zinc-700"
                  >
                    <!-- Header do Card com Tag da Seção (ProPresenter style) -->
                    <div
                      class="h-6 px-2.5 flex items-center justify-between border-b border-zinc-800/80 text-[10px] font-medium {color.bg}"
                    >
                      <div class="flex items-center gap-1.5">
                        <span class="w-2 h-2 rounded-full {color.accent}"></span>
                        <span class="font-semibold {color.text}">{slide.label}</span>
                      </div>
                      <span class="text-zinc-500 font-mono">#{idx + 1}</span>
                    </div>

                    <!-- Conteúdo Projetado (Simulação 16:9) -->
                    <div class="flex-1 p-3 flex items-center justify-center text-center">
                      <p
                        class="text-xs sm:text-sm font-semibold text-white tracking-wide leading-snug drop-shadow whitespace-pre-line line-clamp-4"
                      >
                        {slide.content}
                      </p>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Rodapé do Modal com Ações -->
      <div
        class="h-14 px-5 border-t border-zinc-800 bg-zinc-900/80 flex items-center justify-between shrink-0"
      >
        <label class="flex items-center gap-2 cursor-pointer select-none">
          <input
            type="checkbox"
            bind:checked={addToSetlist}
            class="w-4 h-4 rounded bg-zinc-800 border-zinc-700 text-emerald-500 focus:ring-emerald-500/40 accent-emerald-500"
          />
          <span class="text-xs text-zinc-300 font-medium">Adicionar à Setlist ao salvar</span>
        </label>

        <div class="flex items-center gap-2.5">
          <Button variant="ghost" size="sm" onclick={onclose} disabled={isSaving}>
            Cancelar
          </Button>
          <Button
            variant="default"
            size="sm"
            onclick={saveSong}
            disabled={isSaving}
            class="gap-1.5 min-w-[120px]"
          >
            {#if isSaving}
              <span class="inline-block w-3.5 h-3.5 border-2 border-white/20 border-t-white rounded-full animate-spin"></span>
              Salvando...
            {:else}
              <Save size={14} />
              Salvar Música
            {/if}
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}

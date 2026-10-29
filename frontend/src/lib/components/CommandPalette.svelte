<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Search,
    Plus,
    Globe,
    Music,
    ArrowRight,
    CornerDownLeft,
    Check,
    Loader2,
    X,
    ChevronRight,
    FileText,
    BookOpen,
  } from 'lucide-svelte';
  import type { SongSummary, OnlineLyricHit, Song, ParsedReference } from '../types';
  import { parseBibleReference } from '../bibleParser';
  import Badge from './ui/Badge.svelte';

  interface Props {
    open: boolean;
    onclose: () => void;
    oncreatenew: () => void;
    onimportsong: (song: Song) => void;
    onopenbible?: (ref: ParsedReference) => void;
    onpresentscripture?: (ref: ParsedReference) => void;
    onopenbibletab?: () => void;
  }

  let {
    open = false,
    onclose,
    oncreatenew,
    onimportsong,
    onopenbible,
    onpresentscripture,
    onopenbibletab,
  }: Props = $props();

  let query = $state('');
  let activeTab = $state<'local' | 'online'>('local');
  let localResults = $state<SongSummary[]>([]);
  let onlineResults = $state<OnlineLyricHit[]>([]);
  let isLoading = $state(false);
  let selectedIndex = $state(0);
  let addedIds = $state<Record<string, boolean>>({});
  let inputElement = $state<HTMLInputElement | null>(null);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let prevOpen = false;

  const parsedBibleRef = $derived.by<ParsedReference | null>(() => {
    const q = query.trim();
    if (!q || q.length < 2) return null;
    return parseBibleReference(q);
  });

  // Gerencia abertura/fechamento do Command Palette
  $effect(() => {
    if (open && !prevOpen) {
      query = '';
      activeTab = 'local';
      localResults = [];
      onlineResults = [];
      selectedIndex = 0;
      searchLocal('');
      setTimeout(() => {
        inputElement?.focus();
      }, 50);
    } else if (!open && prevOpen) {
      if (debounceTimer) clearTimeout(debounceTimer);
    }
    prevOpen = open;
  });

  // Busca reativa com debounce ao digitar
  $effect(() => {
    if (!open) return;
    const currentQuery = query;
    const currentTab = activeTab;

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      if (currentTab === 'local') {
        searchLocal(currentQuery);
      } else {
        searchOnline(currentQuery);
      }
    }, 150);

    return () => {
      if (debounceTimer) clearTimeout(debounceTimer);
    };
  });

  async function searchLocal(q: string) {
    isLoading = true;
    try {
      const resp = await fetch(`/api/songs?q=${encodeURIComponent(q.trim())}`);
      if (resp.ok) {
        localResults = await resp.json();
        selectedIndex = 0;
      }
    } catch {
      localResults = [];
    } finally {
      isLoading = false;
    }
  }

  async function searchOnline(q: string) {
    const clean = q.trim();
    if (!clean) {
      onlineResults = [];
      return;
    }
    isLoading = true;
    try {
      const resp = await fetch(`/api/songs/online/search?q=${encodeURIComponent(clean)}`);
      if (resp.ok) {
        onlineResults = await resp.json();
        selectedIndex = 0;
      }
    } catch {
      onlineResults = [];
    } finally {
      isLoading = false;
    }
  }

  async function addToSetlist(songId: string, e?: MouseEvent) {
    if (e) e.stopPropagation();
    try {
      const resp = await fetch('/api/setlist/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ song_id: songId }),
      });
      if (resp.ok) {
        addedIds = { ...addedIds, [songId]: true };
        setTimeout(() => {
          addedIds = { ...addedIds, [songId]: false };
        }, 2000);
      }
    } catch (err) {
      console.error('Erro ao adicionar na setlist', err);
    }
  }

  async function importOnlineHit(hit: OnlineLyricHit) {
    if (!hit.provider || !hit.provider_id) return;
    isLoading = true;
    try {
      const resp = await fetch(
        `/api/songs/online/fetch?provider=${encodeURIComponent(hit.provider)}&id=${encodeURIComponent(hit.provider_id)}`
      );
      if (resp.ok) {
        const song: Song = await resp.json();
        onclose();
        onimportsong(song);
      }
    } catch (err) {
      console.error('Erro ao baixar letra online', err);
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!open) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      onclose();
      return;
    }

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      const max =
        activeTab === 'local'
          ? localResults.length + 3 // +3 opções rápidas (Bíblia, Nova Música, Web)
          : onlineResults.length;
      if (max > 0) {
        selectedIndex = (selectedIndex + 1) % max;
      }
      return;
    }

    if (e.key === 'ArrowUp') {
      e.preventDefault();
      const max =
        activeTab === 'local'
          ? localResults.length + 3
          : onlineResults.length;
      if (max > 0) {
        selectedIndex = (selectedIndex - 1 + max) % max;
      }
      return;
    }

    // Atalhos quando uma referência bíblica for digitada (ex: "joao 3.16" ou "Salmos 91")
    if (parsedBibleRef) {
      if (e.key === 'Enter' && e.shiftKey) {
        e.preventDefault();
        onpresentscripture?.(parsedBibleRef);
        onclose();
        return;
      }
      if (e.key === 'Enter') {
        e.preventDefault();
        onopenbible?.(parsedBibleRef);
        onclose();
        return;
      }
    }

    // Shift + Enter: Adiciona à setlist imediatamente
    if (e.key === 'Enter' && e.shiftKey) {
      e.preventDefault();
      if (activeTab === 'local' && localResults.length > 0) {
        const targetIdx = selectedIndex >= 3 ? selectedIndex - 3 : 0;
        const song = localResults[targetIdx];
        if (song) {
          addToSetlist(song.id);
        }
      }
      return;
    }

    // Enter simples
    if (e.key === 'Enter') {
      e.preventDefault();
      if (activeTab === 'local') {
        if (selectedIndex === 0) {
          // Abrir Aba Bíblia Sagrada
          onclose();
          onopenbibletab?.();
        } else if (selectedIndex === 1) {
          // Criar nova música manualmente
          onclose();
          oncreatenew();
        } else if (selectedIndex === 2) {
          // Buscar na internet
          activeTab = 'online';
          searchOnline(query);
        } else {
          const songIdx = selectedIndex - 3;
          const song = localResults[songIdx];
          if (song) {
            addToSetlist(song.id);
          }
        }
      } else {
        const hit = onlineResults[selectedIndex];
        if (hit) {
          importOnlineHit(hit);
        }
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-start justify-center pt-[10vh] px-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={onclose}
    onkeydown={(e) => {
      if (e.key === 'Escape') onclose();
    }}
  >
    <!-- Card do Command Palette -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="w-full max-w-2xl bg-zinc-950 border border-zinc-800 rounded-xl shadow-2xl overflow-hidden flex flex-col text-zinc-100 animate-in zoom-in-95 duration-150"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Barra de Input Superior -->
      <div class="h-14 px-4 border-b border-zinc-800 flex items-center gap-3 bg-zinc-900/60 shrink-0">
        <Search size={18} class="text-zinc-400 shrink-0" />
        <input
          bind:this={inputElement}
          type="text"
          bind:value={query}
          placeholder={activeTab === 'local'
            ? 'Buscar música por título ou letra, ou selecione uma ação...'
            : 'Buscar na internet (Holyrics, Vagalume)...'}
          class="flex-1 bg-transparent border-0 text-sm text-white placeholder-zinc-500 focus:outline-none focus:ring-0"
        />

        {#if isLoading}
          <Loader2 size={16} class="text-emerald-400 animate-spin shrink-0" />
        {/if}

        <div class="flex items-center gap-1.5 shrink-0">
          <button
            type="button"
            onclick={() => {
              activeTab = activeTab === 'local' ? 'online' : 'local';
              if (activeTab === 'online') searchOnline(query);
              else searchLocal(query);
            }}
            class="px-2 py-1 rounded text-[11px] font-medium border border-zinc-700/80 bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 transition-colors cursor-pointer"
          >
            {activeTab === 'local' ? 'Web 🌐' : 'Catálogo 📚'}
          </button>
          <button
            type="button"
            onclick={onclose}
            class="w-7 h-7 rounded text-zinc-400 hover:text-white hover:bg-zinc-800 flex items-center justify-center transition-colors cursor-pointer"
            aria-label="Fechar"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      <!-- Abas de Navegação -->
      <div class="px-4 py-1.5 bg-zinc-900/40 border-b border-zinc-800/60 flex items-center justify-between text-xs">
        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={() => {
              activeTab = 'local';
              searchLocal(query);
            }}
            class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {activeTab === 'local' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
          >
            Catálogo Local ({localResults.length})
          </button>
          <button
            type="button"
            onclick={() => {
              activeTab = 'online';
              searchOnline(query);
            }}
            class="px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer {activeTab === 'online' ? 'bg-zinc-800 text-emerald-400 font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
          >
            Buscar na Internet
          </button>
        </div>

        <span class="text-[10px] text-zinc-500 hidden sm:inline">
          <kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Shift+Enter</kbd> Add na Setlist
        </span>
      </div>

      <!-- Lista de Resultados e Ações -->
      <div class="max-h-[60vh] overflow-y-auto p-2 space-y-1">
        {#if parsedBibleRef}
          <!-- Card de Referência Bíblica Detectada pelo Parser -->
          <div class="p-3 mb-2 rounded-xl border border-amber-500/40 bg-amber-500/10 text-white flex items-center justify-between gap-3 animate-in fade-in">
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-8 h-8 rounded-lg bg-amber-500/20 text-amber-400 flex items-center justify-center shrink-0">
                <BookOpen size={16} />
              </div>
              <div class="min-w-0">
                <div class="text-[11px] font-bold text-amber-300 flex items-center gap-2">
                  <span>Passagem Bíblica</span>
                  <Badge variant="outline" class="text-[9px] border-amber-500/50 text-amber-300 font-mono py-0 px-1 uppercase">
                    {parsedBibleRef.book_abbreviation}
                  </Badge>
                </div>
                <div class="text-sm font-extrabold text-white truncate">
                  {parsedBibleRef.book_name} {parsedBibleRef.chapter}:{parsedBibleRef.verse_start || 1}{parsedBibleRef.verse_end && parsedBibleRef.verse_end !== parsedBibleRef.verse_start ? `-${parsedBibleRef.verse_end}` : ''}
                </div>
              </div>
            </div>

            <div class="flex items-center gap-1.5 shrink-0">
              <button
                type="button"
                onclick={() => {
                  onopenbible?.(parsedBibleRef!);
                  onclose();
                }}
                class="px-2.5 py-1.5 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-xs font-semibold text-zinc-200 flex items-center gap-1.5 transition-colors cursor-pointer"
                title="Abrir o texto bíblico (Enter)"
              >
                <span>Abrir Texto</span>
                <kbd class="px-1 py-0.2 rounded bg-black/50 text-[10px] font-mono text-zinc-400">↵</kbd>
              </button>

              <button
                type="button"
                onclick={() => {
                  onpresentscripture?.(parsedBibleRef!);
                  onclose();
                }}
                class="px-2.5 py-1.5 rounded-lg bg-amber-500 hover:bg-amber-600 text-xs font-bold text-black flex items-center gap-1.5 transition-colors cursor-pointer shadow-md"
                title="Projetar na tela imediatamente (Shift+Enter)"
              >
                <span>Projetar Agora</span>
                <kbd class="px-1 py-0.2 rounded bg-black/20 text-[10px] font-mono">⇧↵</kbd>
              </button>
            </div>
          </div>
        {/if}

        {#if activeTab === 'local'}
          <!-- Ações Rápidas (Sempre no Topo da aba Local) -->
          <div class="space-y-0.5 mb-2 pb-2 border-b border-zinc-800/60">
            <!-- Ação 0: Abrir Bíblia Sagrada -->
            <button
              type="button"
              onclick={() => {
                onclose();
                onopenbibletab?.();
              }}
              class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {selectedIndex === 0 ? 'bg-zinc-800/90 text-white' : 'hover:bg-zinc-900/80 text-zinc-300'}"
            >
              <div class="flex items-center gap-2.5">
                <div class="w-6 h-6 rounded bg-amber-500/20 text-amber-400 flex items-center justify-center shrink-0">
                  <BookOpen size={14} />
                </div>
                <div>
                  <div class="text-xs font-semibold text-white">Abrir Bíblia Sagrada</div>
                  <div class="text-[10px] text-zinc-400">Navegar por livros, capítulos e versículos bíblicos</div>
                </div>
              </div>
              <div class="flex items-center gap-1">
                <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700/80 text-[10px] font-mono text-zinc-400">⌘B</kbd>
                <kbd class="px-1.5 py-0.5 rounded bg-zinc-800 border border-zinc-700/80 text-[10px] font-mono text-zinc-400">F4</kbd>
              </div>
            </button>

            <!-- Ação 1: Criar Música Manualmente -->
            <button
              type="button"
              onclick={() => {
                onclose();
                oncreatenew();
              }}
              class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {selectedIndex === 1 ? 'bg-zinc-800/90 text-white' : 'hover:bg-zinc-900/80 text-zinc-300'}"
            >
              <div class="flex items-center gap-2.5">
                <div class="w-6 h-6 rounded bg-emerald-500/20 text-emerald-400 flex items-center justify-center shrink-0">
                  <Plus size={14} />
                </div>
                <div>
                  <div class="text-xs font-semibold text-white">Criar nova música manualmente</div>
                  <div class="text-[10px] text-zinc-400">Abre o editor completo em tela com prévia de slides</div>
                </div>
              </div>
              <Badge variant="outline" class="text-[10px] font-mono text-zinc-400">Novo</Badge>
            </button>

            <!-- Ação 2: Buscar na Internet -->
            <button
              type="button"
              onclick={() => {
                activeTab = 'online';
                searchOnline(query);
              }}
              class="w-full px-3 py-2 rounded-lg text-left flex items-center justify-between transition-colors cursor-pointer {selectedIndex === 2 ? 'bg-zinc-800/90 text-white' : 'hover:bg-zinc-900/80 text-zinc-300'}"
            >
              <div class="flex items-center gap-2.5">
                <div class="w-6 h-6 rounded bg-sky-500/20 text-sky-400 flex items-center justify-center shrink-0">
                  <Globe size={14} />
                </div>
                <div>
                  <div class="text-xs font-semibold text-white">
                    Buscar {query ? `"${query}"` : 'música'} na internet
                  </div>
                  <div class="text-[10px] text-zinc-400">Consulta Holyrics e Vagalume com download automático</div>
                </div>
              </div>
              <ChevronRight size={14} class="text-zinc-500" />
            </button>
          </div>

          <!-- Resultados do Banco SQLite (FTS5) -->
          {#if localResults.length > 0}
            <div class="px-2 py-1 text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
              Músicas no Banco ({localResults.length})
            </div>

            {#each localResults as song, idx (song.id)}
              {@const isSelected = selectedIndex === idx + 2}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                role="button"
                tabindex="0"
                onclick={() => addToSetlist(song.id)}
                class="w-full px-3 py-2.5 rounded-lg flex items-center justify-between transition-colors cursor-pointer text-left {isSelected ? 'bg-zinc-800/90 ring-1 ring-emerald-500/50' : 'hover:bg-zinc-900/80'}"
              >
                <div class="flex items-start gap-2.5 min-w-0 pr-2">
                  <div class="w-6 h-6 rounded bg-zinc-800 text-zinc-400 flex items-center justify-center shrink-0 mt-0.5">
                    <Music size={13} class="text-emerald-400" />
                  </div>
                  <div class="min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="text-xs font-medium text-white truncate">{song.title}</span>
                      {#if song.song_key}
                        <Badge variant="outline" class="text-[9px] py-0 px-1 border-zinc-700 text-zinc-300 font-mono">
                          {song.song_key}
                        </Badge>
                      {/if}
                      {#if song.bpm}
                        <Badge variant="outline" class="text-[9px] py-0 px-1 border-zinc-700 text-zinc-400 font-mono">
                          {song.bpm} BPM
                        </Badge>
                      {/if}
                    </div>

                    <div class="text-[11px] text-zinc-400 truncate">
                      {song.artist || 'Sem artista'}
                    </div>

                    {#if song.preview_snippet}
                      <div class="text-[10px] text-zinc-500 line-clamp-1 mt-0.5 font-mono italic">
                        {@html song.preview_snippet}
                      </div>
                    {/if}
                  </div>
                </div>

                <!-- Botão Adicionar à Setlist -->
                <button
                  type="button"
                  onclick={(e) => addToSetlist(song.id, e)}
                  class="px-2.5 py-1 rounded bg-zinc-800 hover:bg-emerald-600 hover:text-white border border-zinc-700/80 text-xs font-medium text-zinc-200 transition-colors shrink-0 flex items-center gap-1.5 cursor-pointer"
                >
                  {#if addedIds[song.id]}
                    <Check size={12} class="text-emerald-400" />
                    <span>Adicionado</span>
                  {:else}
                    <Plus size={12} />
                    <span>Add na Setlist</span>
                  {/if}
                </button>
              </div>
            {/each}
          {:else if query}
            <div class="p-6 text-center space-y-2">
              <p class="text-xs text-zinc-400">Nenhuma música encontrada no catálogo local para "{query}".</p>
              <button
                type="button"
                onclick={() => {
                  activeTab = 'online';
                  searchOnline(query);
                }}
                class="px-3 py-1.5 rounded-lg bg-emerald-600/20 border border-emerald-500/40 text-emerald-300 hover:bg-emerald-600/30 text-xs font-medium inline-flex items-center gap-1.5 transition-colors cursor-pointer"
              >
                <Globe size={13} />
                Buscar "{query}" na internet
              </button>
            </div>
          {/if}
        {:else}
          <!-- ABA ONLINE: Provedores (Holyrics / Vagalume) -->
          {#if onlineResults.length > 0}
            <div class="px-2 py-1 text-[10px] font-semibold uppercase tracking-wider text-zinc-500">
              Resultados da Internet ({onlineResults.length})
            </div>

            {#each onlineResults as hit, idx}
              {@const isSelected = selectedIndex === idx}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div
                role="button"
                tabindex="0"
                onclick={() => importOnlineHit(hit)}
                class="w-full px-3 py-2.5 rounded-lg flex items-center justify-between transition-colors cursor-pointer text-left {isSelected ? 'bg-zinc-800/90 ring-1 ring-sky-500/50' : 'hover:bg-zinc-900/80'}"
              >
                <div class="flex items-start gap-2.5 min-w-0 pr-2">
                  <div class="w-6 h-6 rounded bg-sky-500/20 text-sky-400 flex items-center justify-center shrink-0 mt-0.5">
                    <Globe size={13} />
                  </div>
                  <div class="min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="text-xs font-medium text-white truncate">{hit.track_name}</span>
                      <Badge
                        variant="secondary"
                        class="text-[9px] py-0 px-1 {hit.provider === 'holyrics' ? 'bg-emerald-950 text-emerald-300 border border-emerald-800/50' : 'bg-blue-950 text-blue-300 border border-blue-800/50'}"
                      >
                        {hit.provider === 'holyrics' ? 'Holyrics' : 'Vagalume'}
                      </Badge>
                    </div>
                    <div class="text-[11px] text-zinc-400 truncate">
                      {hit.artist_name || 'Desconhecido'}
                    </div>
                  </div>
                </div>

                <button
                  type="button"
                  onclick={() => importOnlineHit(hit)}
                  class="px-2.5 py-1 rounded bg-zinc-800 hover:bg-sky-600 hover:text-white border border-zinc-700/80 text-xs font-medium text-zinc-200 transition-colors shrink-0 flex items-center gap-1.5 cursor-pointer"
                >
                  <FileText size={12} />
                  <span>Importar e Editar</span>
                </button>
              </div>
            {/each}
          {:else if query && !isLoading}
            <div class="p-8 text-center text-zinc-500 text-xs">
              Nenhum resultado online para "{query}". Tente buscar por outros termos ou crie manualmente.
            </div>
          {:else if !query}
            <div class="p-8 text-center text-zinc-500 text-xs">
              Digite o nome de um louvor, artista ou trecho para pesquisar online.
            </div>
          {/if}
        {/if}
      </div>

      <!-- Rodapé de Dicas de Teclado -->
      <div class="h-9 px-4 border-t border-zinc-800/80 bg-zinc-900/40 flex items-center justify-between text-[11px] text-zinc-500 shrink-0">
        <div class="flex items-center gap-3">
          <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">↑↓</kbd> Navegar</span>
          <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Enter</kbd> Selecionar</span>
          <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Shift+Enter</kbd> Add Setlist</span>
        </div>
        <span><kbd class="px-1 py-0.5 rounded bg-zinc-800 font-mono text-[9px]">Esc</kbd> Fechar</span>
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import { onMount } from 'svelte';
  import {
    X,
    Save,
    Music,
    AlertCircle,
    Sparkles,
    Check,
    Hash,
    Bold,
    Italic,
    Underline as UnderlineIcon,
    Highlighter,
    AlignLeft,
    AlignCenter,
    AlignRight,
    AlignJustify,
    AlignVerticalJustifyStart,
    AlignVerticalJustifyCenter,
    AlignVerticalJustifyEnd,
    FileText,
    Rows2,
    ChevronDown,
    Copy,
    Trash2,
    Tag,
    Plus,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import ContextMenu, { type MenuItem } from './ContextMenu.svelte';
  import { parseRawLyrics, getSectionColor, formatLyricsLinesPerSlide } from '../songParser';
  import { renderFormattedText } from '../markdown';
  import type { Song, Slide, MediaItem } from '../types';
  import { toast } from '$lib/toast';

  interface Props {
    open: boolean;
    mode?: 'song' | 'text';
    initialSong?: Partial<Song> | null;
    initialItem?: Partial<MediaItem> | null;
    onclose: () => void;
    onsave?: (savedSong: Song) => void;
    onsaveItem?: (savedItem: MediaItem) => void;
  }

  let {
    open = false,
    mode = 'song',
    initialSong = null,
    initialItem = null,
    onclose,
    onsave,
    onsaveItem,
  }: Props = $props();

  let id = $state('');
  let title = $state('');
  let artist = $state('');
  let composer = $state('');
  let songKey = $state('');
  let bpm = $state<number | undefined>(undefined);
  let lyricsRaw = $state('');
  let tags = $state<string[]>([]);
  let tagInput = $state('');
  let addToSetlist = $state(true);
  let isSaving = $state(false);
  let errorMessage = $state('');
  let prevOpen = false;

  // Alinhamento
  let alignH = $state<'left' | 'center' | 'right' | 'justify'>('center');
  let alignV = $state<'top' | 'center' | 'bottom'>('center');

  // Referência ao textarea para inserção de formatação
  let textareaEl: HTMLTextAreaElement | null = $state(null);

  // Estado do menu de contexto
  let contextMenuOpen = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuItems = $state<MenuItem[]>([]);

  function addTag(tagText: string) {
    const clean = tagText.trim();
    if (clean && !tags.some((t) => t.toLowerCase() === clean.toLowerCase())) {
      tags = [...tags, clean];
    }
    tagInput = '';
  }

  function removeTag(tagToRemove: string) {
    tags = tags.filter((t) => t !== tagToRemove);
  }

  function handleTagKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ',') {
      e.preventDefault();
      addTag(tagInput);
    } else if (e.key === 'Backspace' && !tagInput && tags.length > 0) {
      tags = tags.slice(0, -1);
    }
  }

  function applyLinesPerSlide() {
    lyricsRaw = formatLyricsLinesPerSlide(lyricsRaw, 2);
    toast.success('Letra reorganizada no padrão de 2 linhas por slide!');
  }

  // Sincroniza estado apenas na abertura do modal
  $effect(() => {
    if (open && !prevOpen) {
      if (mode === 'song') {
        if (initialSong) {
          id = initialSong.id || `song-${Date.now()}`;
          title = initialSong.title || '';
          artist = initialSong.artist || '';
          composer = initialSong.composer || '';
          songKey = initialSong.song_key || '';
          bpm = initialSong.bpm;
          lyricsRaw = initialSong.lyrics_raw || '';
          tags = initialSong.tags ? [...initialSong.tags] : [];
        } else {
          id = `song-${Date.now()}`;
          title = '';
          artist = '';
          composer = '';
          songKey = '';
          bpm = undefined;
          tags = [];
          lyricsRaw = `## Verso 1
Primeira estrofe da canção
Segunda linha aqui

Outro slide deste verso
Linha vazia separou

## Refrão
Porque Ele vive, posso crer no amanhã
Porque Ele vive, temor não há`;
        }
      } else {
        // Modo Texto / Apresentação Rápida
        if (initialItem) {
          id = initialItem.id || `text-${Date.now()}`;
          title = initialItem.title || '';
          artist = initialItem.artist_or_ref || '';
          alignH = initialItem.align_h || 'center';
          alignV = initialItem.align_v || 'center';
          lyricsRaw = initialItem.slides?.map((s) => s.content).join('\n\n') || '';
        } else {
          id = `text-${Date.now()}`;
          title = '';
          artist = 'Avisos da Semana';
          alignH = 'center';
          alignV = 'center';
          lyricsRaw = `## Boas-vindas
Sejam bem-vindos à nossa reunião!
Estamos felizes com sua presença.

## Próximo Evento
Domingo às 19h00: Culto de Celebração.
Traga sua família e convidados!`;
        }
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
      saveContent();
    }
  }

  // Ferramentas Typeset: envolve texto selecionado no textarea
  function wrapSelection(prefix: string, suffix: string, defaultText = 'texto') {
    if (!textareaEl) {
      lyricsRaw += `${prefix}${defaultText}${suffix}`;
      return;
    }

    const start = textareaEl.selectionStart;
    const end = textareaEl.selectionEnd;
    const current = lyricsRaw;
    const selected = current.slice(start, end);
    const replacement = selected ? `${prefix}${selected}${suffix}` : `${prefix}${defaultText}${suffix}`;

    lyricsRaw = current.slice(0, start) + replacement + current.slice(end);

    setTimeout(() => {
      if (textareaEl) {
        textareaEl.focus();
        textareaEl.setSelectionRange(start + prefix.length, start + prefix.length + (selected ? selected.length : defaultText.length));
      }
    }, 10);
  }

  function applyBold() {
    wrapSelection('**', '**', 'negrito');
  }

  function applyItalic() {
    wrapSelection('*', '*', 'itálico');
  }

  function applyUnderline() {
    wrapSelection('<u>', '</u>', 'sublinhado');
  }

  function applyHighlight() {
    wrapSelection('==', '==', 'destaque');
  }

  async function saveContent() {
    if (!title.trim()) {
      errorMessage = mode === 'song' ? 'O título da música é obrigatório.' : 'O título da apresentação é obrigatório.';
      return;
    }

    isSaving = true;
    errorMessage = '';

    if (mode === 'song') {
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
        tags,
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
        toast.success(id ? 'Música atualizada com sucesso!' : 'Música salva com sucesso no catálogo!');
        if (typeof window !== 'undefined') {
          window.dispatchEvent(new CustomEvent('openpresenter:songs-changed'));
        }
        onsave?.(saved);
        onclose();
      } catch (err: any) {
        errorMessage = err.message || 'Falha na conexão ao salvar';
        toast.error(errorMessage);
      } finally {
        isSaving = false;
      }
    } else {
      // Modo Texto / Apresentação Rápida
      const slidesPayload: Slide[] = parsed.slides.map((s) => ({
        ...s,
        align_h: alignH,
        align_v: alignV,
      }));

      const mediaItemPayload: MediaItem = {
        id: id || `text-${Date.now()}`,
        title: title.trim(),
        artist_or_ref: artist.trim() || undefined,
        kind: 'announcement',
        slides: slidesPayload,
        align_h: alignH,
        align_v: alignV,
      };

      try {
        const resp = await fetch('/api/setlist/add', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            item: mediaItemPayload,
          }),
        });

        if (!resp.ok) {
          const errorText = await resp.text();
          throw new Error(errorText || 'Erro ao salvar texto na setlist');
        }

        toast.success('Apresentação rápida adicionada à setlist!');
        onsaveItem?.(mediaItemPayload);
        onclose();
      } catch (err: any) {
        errorMessage = err.message || 'Falha ao salvar apresentação';
        toast.error(errorMessage);
      } finally {
        isSaving = false;
      }
    }
  }

  function insertSectionTag(tag: string) {
    if (!textareaEl) {
      lyricsRaw += `\n\n## ${tag}\n`;
      return;
    }

    const start = textareaEl.selectionStart;
    const end = textareaEl.selectionEnd;
    const current = lyricsRaw;

    let prefix = '';
    if (start > 0) {
      if (current.slice(0, start).endsWith('\n\n')) {
        prefix = '## ';
      } else if (current.slice(0, start).endsWith('\n')) {
        prefix = '\n## ';
      } else {
        prefix = '\n\n## ';
      }
    } else {
      prefix = '## ';
    }

    const suffix = '\n';
    const insertion = `${prefix}${tag}${suffix}`;
    lyricsRaw = current.slice(0, start) + insertion + current.slice(end);
    const newCursor = start + insertion.length;

    setTimeout(() => {
      if (textareaEl) {
        textareaEl.focus();
        textareaEl.setSelectionRange(newCursor, newCursor);
      }
    }, 10);
  }

  interface SlideLocation {
    slideIndex: number;
    sectionLabel: string;
    sectionHeaderStart: number;
    sectionHeaderEnd: number;
    isFirstInSec: boolean;
    blockStart: number;
    blockEnd: number;
  }

  function getSlideLocations(raw: string): SlideLocation[] {
    const locations: SlideLocation[] = [];
    const lineRegex = /([^\r\n]*)(\r?\n|$)/g;
    let match: RegExpExecArray | null;

    let currentSectionLabel = 'Verso 1';
    let currentSecHeaderStart = -1;
    let currentSecHeaderEnd = -1;
    let isFirstInSec = true;

    let currentBlockStart = -1;
    let currentBlockEnd = -1;
    let inBlock = false;

    while ((match = lineRegex.exec(raw)) !== null) {
      if (match.index === raw.length && match[0] === '') break;

      const line = match[1];
      const lineStart = match.index;
      const lineEnd = lineStart + line.length;
      const trimmed = line.trim();

      if (trimmed.startsWith('##') || trimmed.startsWith('# ')) {
        if (inBlock) {
          locations.push({
            slideIndex: locations.length,
            sectionLabel: currentSectionLabel,
            sectionHeaderStart: currentSecHeaderStart,
            sectionHeaderEnd: currentSecHeaderEnd,
            isFirstInSec,
            blockStart: currentBlockStart,
            blockEnd: currentBlockEnd,
          });
          inBlock = false;
          isFirstInSec = false;
        }
        currentSectionLabel = trimmed.replace(/^#+\s*/, '').trim() || 'Seção';
        currentSecHeaderStart = lineStart;
        currentSecHeaderEnd = lineEnd;
        isFirstInSec = true;
      } else if (trimmed === '') {
        if (inBlock) {
          locations.push({
            slideIndex: locations.length,
            sectionLabel: currentSectionLabel,
            sectionHeaderStart: currentSecHeaderStart,
            sectionHeaderEnd: currentSecHeaderEnd,
            isFirstInSec,
            blockStart: currentBlockStart,
            blockEnd: currentBlockEnd,
          });
          inBlock = false;
          isFirstInSec = false;
        }
      } else {
        if (!inBlock) {
          inBlock = true;
          currentBlockStart = lineStart;
        }
        currentBlockEnd = lineEnd;
      }

      if (match[2] === '') break;
    }

    if (inBlock) {
      locations.push({
        slideIndex: locations.length,
        sectionLabel: currentSectionLabel,
        sectionHeaderStart: currentSecHeaderStart,
        sectionHeaderEnd: currentSecHeaderEnd,
        isFirstInSec,
        blockStart: currentBlockStart,
        blockEnd: currentBlockEnd,
      });
    }

    return locations;
  }

  function setSlideSection(slideIdx: number, newLabel: string) {
    const locs = getSlideLocations(lyricsRaw);
    const loc = locs[slideIdx];
    if (!loc) return;

    if (loc.isFirstInSec && loc.sectionHeaderStart >= 0) {
      lyricsRaw =
        lyricsRaw.slice(0, loc.sectionHeaderStart) +
        `## ${newLabel}` +
        lyricsRaw.slice(loc.sectionHeaderEnd);
    } else {
      const before = lyricsRaw.slice(0, loc.blockStart).trimEnd();
      const after = lyricsRaw.slice(loc.blockStart);
      lyricsRaw = `${before}\n\n## ${newLabel}\n${after}`;
    }
    toast.success(`Seção alterada para "${newLabel}"!`);
  }

  function duplicateSlide(slideIdx: number) {
    const locs = getSlideLocations(lyricsRaw);
    const loc = locs[slideIdx];
    if (!loc) return;
    const slideContent = lyricsRaw.slice(loc.blockStart, loc.blockEnd);
    const before = lyricsRaw.slice(0, loc.blockEnd);
    const after = lyricsRaw.slice(loc.blockEnd);
    lyricsRaw = `${before}\n\n${slideContent}${after}`;
    toast.success('Slide duplicado!');
  }

  function deleteSlide(slideIdx: number) {
    const locs = getSlideLocations(lyricsRaw);
    const loc = locs[slideIdx];
    if (!loc) return;

    const hasSiblingInSec = locs.some(
      (l, idx) => idx !== slideIdx && l.sectionHeaderStart === loc.sectionHeaderStart
    );

    let deleteStart = loc.blockStart;
    if (!hasSiblingInSec && loc.sectionHeaderStart >= 0) {
      deleteStart = loc.sectionHeaderStart;
    }

    let deleteEnd = loc.blockEnd;
    while (
      deleteEnd < lyricsRaw.length &&
      (lyricsRaw[deleteEnd] === '\n' || lyricsRaw[deleteEnd] === '\r')
    ) {
      deleteEnd++;
    }

    const before = lyricsRaw.slice(0, deleteStart).trimEnd();
    const after = lyricsRaw.slice(deleteEnd).trimStart();

    if (before && after) {
      const joiner = before.split('\n').pop()?.startsWith('#') ? '\n' : '\n\n';
      lyricsRaw = `${before}${joiner}${after}`;
    } else {
      lyricsRaw = before || after || '';
    }
    toast.success('Slide removido!');
  }

  function promptInsertNewSection() {
    const customName = prompt(
      'Digite o nome da nova seção (Ex: Ministração, Espontâneo, Refrão 2, Solo):'
    );
    if (customName && customName.trim()) {
      insertSectionTag(customName.trim());
    }
  }

  function promptNewSection(slideIdx: number) {
    const customName = prompt(
      'Digite o nome da nova seção (Ex: Ministração, Espontâneo, Refrão 2, Solo):'
    );
    if (customName && customName.trim()) {
      setSlideSection(slideIdx, customName.trim());
    }
  }

  function handleOpenSectionMenu(e: MouseEvent) {
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    contextMenuX = rect.left;
    contextMenuY = rect.bottom + 4;

    const sectionsList =
      mode === 'song'
        ? [
            'Verso 1',
            'Verso 2',
            'Verso 3',
            'Pré-Refrão',
            'Refrão',
            'Ponte',
            'Intro',
            'Final',
            'Instrumental',
          ]
        : ['Boas-vindas', 'Aviso', 'Campanha', 'Informações', 'Encerramento'];

    contextMenuItems = [
      {
        id: 'tb-sec-header',
        label: 'Inserir Seção no Cursor:',
        disabled: true,
      },
      { id: 'tb-div-0', divider: true },
      ...sectionsList.map((sec) => ({
        id: `tb-sec-${sec}`,
        label: sec,
        icon: Tag,
        onclick: () => insertSectionTag(sec),
      })),
      { id: 'tb-div-1', divider: true },
      {
        id: 'tb-sec-new',
        label: '+ Nova Seção...',
        icon: Plus,
        onclick: () => promptInsertNewSection(),
      },
    ];

    contextMenuOpen = true;
  }

  function handleSlideContextMenu(e: MouseEvent, slide: Slide, idx: number) {
    e.preventDefault();
    e.stopPropagation();

    contextMenuX = e.clientX;
    contextMenuY = e.clientY;

    const sectionsList =
      mode === 'song'
        ? [
            'Verso 1',
            'Verso 2',
            'Verso 3',
            'Pré-Refrão',
            'Refrão',
            'Ponte',
            'Intro',
            'Final',
            'Instrumental',
          ]
        : ['Boas-vindas', 'Aviso', 'Campanha', 'Informações', 'Encerramento'];

    const sectionItems: MenuItem[] = sectionsList.map((sec) => ({
      id: `sec-${sec}`,
      label: slide.label.startsWith(sec) ? `✓ ${sec}` : sec,
      icon: Tag,
      onclick: () => setSlideSection(idx, sec),
    }));

    contextMenuItems = [
      {
        id: 'slide-header',
        label: `Slide #${idx + 1} (${slide.label})`,
        disabled: true,
      },
      { id: 'div-sections', divider: true },
      {
        id: 'sections-title',
        label: 'Mudar Seção:',
        disabled: true,
      },
      ...sectionItems,
      {
        id: 'sec-new-custom',
        label: '+ Nova Seção...',
        icon: Plus,
        onclick: () => promptNewSection(idx),
      },
      { id: 'div-actions', divider: true },
      {
        id: 'dup-slide',
        label: 'Duplicar Slide',
        icon: Copy,
        onclick: () => duplicateSlide(idx),
      },
      {
        id: 'del-slide',
        label: 'Excluir Slide',
        icon: Trash2,
        destructive: true,
        onclick: () => deleteSlide(idx),
      },
    ];

    contextMenuOpen = true;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-3 sm:p-6 animate-in fade-in duration-200"
    role="dialog"
    aria-modal="true"
    aria-label={mode === 'song' ? 'Editor de Música' : 'Editor de Apresentação e Texto'}
    tabindex="-1"
    onclick={(e) => {
      if (e.target === e.currentTarget) onclose();
    }}
  >
    <!-- Modal Container (95% da tela) -->
    <div
      class="w-[95vw] h-[95vh] max-w-7xl bg-neutral-950 border border-neutral-800 rounded-xl shadow-2xl flex flex-col overflow-hidden text-neutral-100"
    >
      <!-- Cabeçalho do Modal -->
      <div
        class="h-14 px-5 border-b border-neutral-800 bg-neutral-900/60 flex items-center justify-between shrink-0"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-8 h-8 rounded-lg bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400"
          >
            {#if mode === 'song'}
              <Music size={18} />
            {:else}
              <FileText size={18} class="text-amber-400" />
            {/if}
          </div>
          <div>
            <h2 class="text-sm sm:text-base font-semibold tracking-tight text-white flex items-center gap-2">
              {title ? title : mode === 'song' ? 'Nova Música' : 'Nova Apresentação de Texto'}
              <Badge variant="outline" class="text-[10px] text-neutral-400 font-mono">
                {parsed.slides.length} slides
              </Badge>
            </h2>
            <p class="text-[11px] text-neutral-400">
              {mode === 'song'
                ? 'Editor de letras e partes no padrão Holyrics e ProPresenter'
                : 'Editor de slides rápidos, avisos e comunicados com formatação rica'}
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2">
          <span class="text-[11px] text-neutral-500 hidden md:inline">
            Salvar: <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 font-mono text-[10px]">⌘S</kbd> / <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 font-mono text-[10px]">Ctrl+S</kbd>
          </span>
          <button
            type="button"
            onclick={onclose}
            class="w-8 h-8 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800/80 flex items-center justify-center transition-colors cursor-pointer"
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
      <div class="flex-1 flex flex-col lg:flex-row overflow-hidden divide-y lg:divide-y-0 lg:divide-x divide-neutral-800/80">
        <!-- PAINEL ESQUERDO: Metadados + Toolbar Typeset + Editor de Texto -->
        <div class="w-full lg:w-1/2 flex flex-col overflow-hidden bg-neutral-950">
          <!-- Metadados -->
          <div class="p-4 border-b border-neutral-800/80 bg-neutral-900/20 space-y-3 shrink-0">
            {#if mode === 'song'}
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div>
                  <label for="song-title-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                    Título da Música <span class="text-emerald-400">*</span>
                  </label>
                  <input
                    id="song-title-input"
                    type="text"
                    bind:value={title}
                    placeholder="Ex: Porque Ele Vive"
                    class="w-full h-8 px-2.5 bg-neutral-900 border border-neutral-700/80 rounded-md text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                  />
                </div>

                <div>
                  <label for="song-artist-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                    Artista / Banda / Autor
                  </label>
                  <input
                    id="song-artist-input"
                    type="text"
                    bind:value={artist}
                    placeholder="Ex: Harpa Cristã / Matt Maher"
                    class="w-full h-8 px-2.5 bg-neutral-900 border border-neutral-700/80 rounded-md text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                  />
                </div>
              </div>

              <div class="grid grid-cols-3 gap-3">
                <div>
                  <label for="song-key-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                    Tom (Key)
                  </label>
                  <input
                    id="song-key-input"
                    type="text"
                    bind:value={songKey}
                    placeholder="Ex: G, Dm, C#"
                    class="w-full h-8 px-2.5 bg-neutral-900 border border-neutral-700/80 rounded-md text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                  />
                </div>

                <div>
                  <label for="song-bpm-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                    BPM (Andamento)
                  </label>
                  <input
                    id="song-bpm-input"
                    type="number"
                    bind:value={bpm}
                    placeholder="Ex: 72"
                    class="w-full h-8 px-2.5 bg-neutral-900 border border-neutral-700/80 rounded-md text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                  />
                </div>

                <div>
                  <label for="song-composer-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                    Compositor
                  </label>
                  <input
                    id="song-composer-input"
                    type="text"
                    bind:value={composer}
                    placeholder="Ex: Bill Gaither"
                    class="w-full h-8 px-2.5 bg-neutral-900 border border-neutral-700/80 rounded-md text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 transition-colors"
                  />
                </div>
              </div>

              <!-- Tags / Grupos de Classificação -->
              <div>
                <label for="song-tags-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                  Grupos e Tags (Ex: Louvor, Harpa Cristã, Comunhão, Celebração)
                </label>
                <div class="flex flex-wrap items-center gap-1.5 p-1.5 bg-neutral-900 border border-neutral-700/80 rounded-md min-h-[34px]">
                  {#each tags as tag}
                    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-emerald-500/20 border border-emerald-500/40 text-emerald-300 text-xs font-medium">
                      <span>{tag}</span>
                      <button
                        type="button"
                        onclick={() => removeTag(tag)}
                        class="hover:text-red-400 text-emerald-400/80 transition-colors cursor-pointer"
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
                    placeholder={tags.length === 0 ? "Digite tags separadas por Enter ou vírgula..." : "Adicionar tag..."}
                    class="flex-1 min-w-[120px] bg-transparent border-0 text-xs text-white placeholder-neutral-500 focus:outline-none focus:ring-0 p-0.5"
                  />
                </div>
              </div>
            {:else}
              <!-- Metadados do Modo Texto -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div>
                  <label for="text-title-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                    Título da Apresentação <span class="text-amber-400">*</span>
                  </label>
                  <input
                    id="text-title-input"
                    type="text"
                    bind:value={title}
                    placeholder="Ex: Avisos e Boas-Vindas"
                    class="w-full h-8 px-2.5 bg-neutral-900 border border-neutral-700/80 rounded-md text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-amber-500 focus:ring-1 focus:ring-amber-500 transition-colors"
                  />
                </div>

                <div>
                  <label for="text-subtitle-input" class="block text-[11px] font-medium text-neutral-400 mb-1">
                    Subtítulo ou Categoria (Opcional)
                  </label>
                  <input
                    id="text-subtitle-input"
                    type="text"
                    bind:value={artist}
                    placeholder="Ex: Comunicação da Semana"
                    class="w-full h-8 px-2.5 bg-neutral-900 border border-neutral-700/80 rounded-md text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-amber-500 focus:ring-1 focus:ring-amber-500 transition-colors"
                  />
                </div>
              </div>
            {/if}
          </div>

          <!-- BARRA DE FERRAMENTAS TYPESET E ALINHAMENTO -->
          <div class="px-3 py-2 border-b border-neutral-800 bg-neutral-900/60 flex items-center justify-between gap-2 text-xs shrink-0 select-none">
            <!-- Formatação de Texto (Markdown Typeset) -->
            <div class="flex items-center gap-1 shrink-0">
              <span class="text-[10px] font-semibold text-neutral-400 uppercase tracking-wider mr-1">Typeset:</span>
              <button
                type="button"
                onclick={applyBold}
                title="Negrito (**texto**)"
                class="w-7 h-7 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-200 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
              >
                <Bold size={13} />
              </button>
              <button
                type="button"
                onclick={applyItalic}
                title="Itálico (*texto*)"
                class="w-7 h-7 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-200 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
              >
                <Italic size={13} />
              </button>
              <button
                type="button"
                onclick={applyUnderline}
                title="Sublinhado (<u>texto</u>)"
                class="w-7 h-7 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-200 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
              >
                <UnderlineIcon size={13} />
              </button>
              <button
                type="button"
                onclick={applyHighlight}
                title="Marca-texto / Highlight (==texto==)"
                class="w-7 h-7 rounded bg-neutral-800 hover:bg-neutral-700 text-yellow-400 hover:text-yellow-300 flex items-center justify-center transition-colors cursor-pointer"
              >
                <Highlighter size={13} />
              </button>
            </div>

            <div class="h-4 w-[1px] bg-neutral-800 shrink-0 mx-1"></div>

            <!-- Alinhamento Horizontal e Vertical -->
            <div class="flex items-center gap-2 shrink-0">
              <!-- Horizontal -->
              <div class="flex items-center rounded-md bg-neutral-900 border border-neutral-800 p-0.5">
                <button
                  type="button"
                  onclick={() => (alignH = 'left')}
                  title="Alinhar à Esquerda"
                  class="w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer {alignH === 'left' ? 'bg-neutral-700 text-white' : 'text-neutral-400 hover:text-neutral-200'}"
                >
                  <AlignLeft size={12} />
                </button>
                <button
                  type="button"
                  onclick={() => (alignH = 'center')}
                  title="Centralizar Horizontalmente"
                  class="w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer {alignH === 'center' ? 'bg-neutral-700 text-white' : 'text-neutral-400 hover:text-neutral-200'}"
                >
                  <AlignCenter size={12} />
                </button>
                <button
                  type="button"
                  onclick={() => (alignH = 'right')}
                  title="Alinhar à Direita"
                  class="w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer {alignH === 'right' ? 'bg-neutral-700 text-white' : 'text-neutral-400 hover:text-neutral-200'}"
                >
                  <AlignRight size={12} />
                </button>
                <button
                  type="button"
                  onclick={() => (alignH = 'justify')}
                  title="Justificado"
                  class="w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer {alignH === 'justify' ? 'bg-neutral-700 text-white' : 'text-neutral-400 hover:text-neutral-200'}"
                >
                  <AlignJustify size={12} />
                </button>
              </div>

              <!-- Vertical -->
              <div class="flex items-center rounded-md bg-neutral-900 border border-neutral-800 p-0.5">
                <button
                  type="button"
                  onclick={() => (alignV = 'top')}
                  title="Alinhar ao Topo"
                  class="w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer {alignV === 'top' ? 'bg-neutral-700 text-white' : 'text-neutral-400 hover:text-neutral-200'}"
                >
                  <AlignVerticalJustifyStart size={12} />
                </button>
                <button
                  type="button"
                  onclick={() => (alignV = 'center')}
                  title="Centralizar Verticalmente"
                  class="w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer {alignV === 'center' ? 'bg-neutral-700 text-white' : 'text-neutral-400 hover:text-neutral-200'}"
                >
                  <AlignVerticalJustifyCenter size={12} />
                </button>
                <button
                  type="button"
                  onclick={() => (alignV = 'bottom')}
                  title="Alinhar à Base"
                  class="w-6 h-6 rounded flex items-center justify-center transition-colors cursor-pointer {alignV === 'bottom' ? 'bg-neutral-700 text-white' : 'text-neutral-400 hover:text-neutral-200'}"
                >
                  <AlignVerticalJustifyEnd size={12} />
                </button>
              </div>

              {#if mode === 'song'}
                <div class="h-4 w-[1px] bg-neutral-800 shrink-0 mx-1"></div>
                <button
                  type="button"
                  onclick={applyLinesPerSlide}
                  title="Formatar em 2 linhas por slide preservando seções"
                  class="px-2 py-1 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white text-[11px] font-medium flex items-center gap-1.5 transition-colors cursor-pointer shrink-0"
                >
                  <Rows2 size={13} class="text-emerald-400" />
                  <span>2 Linhas/Slide</span>
                </button>
              {/if}

              <div class="h-4 w-[1px] bg-neutral-800 shrink-0 mx-1"></div>

              <!-- Botão Compacto para Inserir Seção no Cursor -->
              <button
                type="button"
                onclick={handleOpenSectionMenu}
                title="Inserir tag de seção no cursor"
                class="px-2 py-1 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white text-[11px] font-medium flex items-center gap-1.5 transition-colors cursor-pointer shrink-0"
              >
                <Hash size={13} class="text-primary" />
                <span>+ Seção</span>
                <ChevronDown size={11} class="text-neutral-400" />
              </button>
            </div>
          </div>

          <!-- Dica de Sintaxe -->
          <div class="px-4 py-1.5 bg-neutral-900/50 text-[11px] text-neutral-400 flex items-center justify-between shrink-0 border-b border-neutral-800/40">
            <span>Use <code>## Rótulo</code> para nomear slides. Linha em branco separa slides. Suporta markdown: <code>**negrito**</code>, <code>==destaque==</code>.</span>
            <span class="text-neutral-500 font-mono text-[10px]">{lyricsRaw.length} chars</span>
          </div>

          <!-- Textarea do Editor de Letra/Texto -->
          <div class="flex-1 p-4 overflow-hidden flex flex-col">
            <textarea
              bind:this={textareaEl}
              bind:value={lyricsRaw}
              placeholder={mode === 'song' ? 'Digite ou cole a letra da música aqui...' : 'Digite os slides da sua apresentação aqui...'}
              class="w-full flex-1 p-3 bg-neutral-900/70 border border-neutral-800 rounded-lg text-sm text-neutral-100 font-mono leading-relaxed resize-none focus:outline-none focus:border-primary/80 focus:ring-1 focus:ring-primary/50"
            ></textarea>
          </div>
        </div>

        <!-- PAINEL DIREITO: Prévia ao Vivo dos Slides com Formatação e Alinhamento -->
        <div class="w-full lg:w-1/2 flex flex-col overflow-hidden bg-neutral-950/60">
          <div
            class="h-10 px-4 border-b border-neutral-800/80 bg-neutral-900/30 flex items-center justify-between shrink-0"
          >
            <span class="text-xs font-semibold text-neutral-300 flex items-center gap-2">
              <Sparkles size={14} class="text-primary" />
              Prévia dos Slides ({parsed.slides.length})
            </span>
            <div class="flex items-center gap-2 text-[11px] text-neutral-500 font-mono">
              <span>Alinhamento: {alignH} / {alignV}</span>
              <span>•</span>
              <span>16:9 Real-time</span>
            </div>
          </div>

          <!-- Grid de Slides (Scrollable) -->
          <div class="flex-1 p-4 overflow-y-auto space-y-4">
            {#if parsed.slides.length === 0}
              <div
                class="h-64 flex flex-col items-center justify-center text-neutral-500 text-xs border border-dashed border-neutral-800 rounded-xl"
              >
                {#if mode === 'song'}
                  <Music size={28} class="mb-2 opacity-40 text-primary" />
                {:else}
                  <FileText size={28} class="mb-2 opacity-40 text-amber-400" />
                {/if}
                <p>Nenhum slide gerado ainda.</p>
                <p class="text-[11px] text-neutral-600 mt-1">
                  Digite o conteúdo no painel esquerdo para visualizar os slides aqui.
                </p>
              </div>
            {:else}
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
                {#each parsed.slides as slide, idx (slide.id)}
                  {@const color = getSectionColor(slide.label)}
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div
                    class="group relative aspect-video bg-neutral-900/90 rounded-lg border border-neutral-800 overflow-hidden flex flex-col shadow-md transition-all hover:border-neutral-700 cursor-context-menu"
                    oncontextmenu={(e) => handleSlideContextMenu(e, slide, idx)}
                    title="Botão direito para mudar seção, duplicar ou excluir slide"
                  >
                    <!-- Header do Card com Tag da Seção -->
                    <div
                      class="h-6 px-2.5 flex items-center justify-between border-b border-neutral-800/80 text-[10px] font-medium {color.bg}"
                    >
                      <div class="flex items-center gap-1.5">
                        <span class="w-2 h-2 rounded-full {color.accent}"></span>
                        <span class="font-semibold {color.text}">{slide.label}</span>
                      </div>
                      <span class="text-neutral-500 font-mono">#{idx + 1}</span>
                    </div>

                    <!-- Conteúdo Projetado (Simulação 16:9 com Markdown & Alinhamento) -->
                    <div
                      class="flex-1 p-3 flex flex-col {alignV === 'top' ? 'justify-start' : alignV === 'bottom' ? 'justify-end' : 'justify-center'} {alignH === 'left' ? 'text-left items-start' : alignH === 'right' ? 'text-right items-end' : alignH === 'justify' ? 'text-justify items-stretch' : 'text-center items-center'}"
                    >
                      <div
                        class="text-xs sm:text-sm font-semibold text-white tracking-wide leading-snug drop-shadow line-clamp-5 max-w-full"
                      >
                        {@html renderFormattedText(slide.content)}
                      </div>
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
        class="h-14 px-5 border-t border-neutral-800 bg-neutral-900/80 flex items-center justify-between shrink-0"
      >
        {#if mode === 'song'}
          <label class="flex items-center gap-2 cursor-pointer select-none">
            <input
              type="checkbox"
              bind:checked={addToSetlist}
              class="w-4 h-4 rounded bg-neutral-800 border-neutral-700 text-emerald-500 focus:ring-emerald-500/40 accent-emerald-500"
            />
            <span class="text-xs text-neutral-300 font-medium">Adicionar à Setlist ao salvar</span>
          </label>
        {:else}
          <div class="text-xs text-neutral-400">
            Será salvo diretamente na Setlist ativa.
          </div>
        {/if}

        <div class="flex items-center gap-2.5">
          <Button variant="ghost" size="sm" onclick={onclose} disabled={isSaving}>
            Cancelar
          </Button>
          <Button
            variant="default"
            size="sm"
            onclick={saveContent}
            disabled={isSaving}
            class="gap-1.5 min-w-[120px]"
          >
            {#if isSaving}
              <span class="inline-block w-3.5 h-3.5 border-2 border-white/20 border-t-white rounded-full animate-spin"></span>
              Salvando...
            {:else}
              <Save size={14} />
              {mode === 'song' ? 'Salvar Música' : 'Salvar Apresentação'}
            {/if}
          </Button>
        </div>
      </div>
    </div>
  </div>

  <ContextMenu
    open={contextMenuOpen}
    x={contextMenuX}
    y={contextMenuY}
    items={contextMenuItems}
    onclose={() => (contextMenuOpen = false)}
  />
{/if}

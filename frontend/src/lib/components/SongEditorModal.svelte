<script lang="ts">
  import { AlertCircle, Save } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import ContextMenu from './ContextMenu.svelte';
  import type { Song, MediaItem } from '../types';
  import SongEditorHeader from './song-editor/SongEditorHeader.svelte';
  import SongEditorMetadata from './song-editor/SongEditorMetadata.svelte';
  import SongEditorToolbar from './song-editor/SongEditorToolbar.svelte';
  import SongEditorLyrics from './song-editor/SongEditorLyrics.svelte';
  import SongEditorPreview from './song-editor/SongEditorPreview.svelte';
  import SongEditorConfirmDiscard from './song-editor/SongEditorConfirmDiscard.svelte';
  import SongEditorSectionName from './song-editor/SongEditorSectionName.svelte';
  import { SongEditorController } from './song-editor/controller.svelte';
  import type { SongEditorMode } from './song-editor/types';

  interface Props {
    open: boolean;
    mode?: SongEditorMode;
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

  const editor = new SongEditorController({
    close: () => onclose(),
    save: (song) => onsave?.(song),
    saveItem: (item) => onsaveItem?.(item),
  });

  $effect(() => {
    editor.syncOpen(open, mode, initialSong, initialItem);
  });

  const metadataSummary = $derived.by(() => {
    const parts: string[] = [];
    if (editor.artist.trim()) parts.push(editor.artist.trim());
    if (mode === 'song') {
      if (editor.songKey.trim()) parts.push(editor.songKey.trim());
      if (editor.bpm) parts.push(`${editor.bpm} BPM`);
    }
    return parts;
  });
</script>

<svelte:window onkeydown={(event) => editor.handleKeydown(event)} />

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm flex items-center justify-center animate-in fade-in duration-200 {editor.isMaximized
      ? 'p-0'
      : 'p-3 sm:p-6'}"
    role="presentation"
    onclick={() => {
      if (!editor.showDiscardConfirm && !editor.showSectionName) editor.requestClose();
    }}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      bind:this={editor.dialogEl}
      class="{editor.isMaximized
        ? 'w-full h-full max-w-none max-h-none rounded-none border-0'
        : 'w-[95vw] h-[95vh] max-w-7xl rounded-xl border border-border'} bg-card text-card-foreground shadow-2xl flex flex-col overflow-hidden transition-all duration-150"
      onclick={(event) => event.stopPropagation()}
      onkeydown={(event) => editor.handleDialogKeydown(event)}
      role="dialog"
      aria-modal="true"
      aria-label={mode === 'song' ? 'Editor de Música' : 'Editor de Apresentação e Texto'}
      inert={editor.showDiscardConfirm || editor.showSectionName}
      tabindex="-1"
    >
      <SongEditorHeader
        {mode}
        title={editor.title}
        slideCount={editor.parsed.slides.length}
        isDirty={editor.isDirty}
        isMaximized={editor.isMaximized}
        ontogglemaximize={() => (editor.isMaximized = !editor.isMaximized)}
        onclose={() => editor.requestClose()}
      />

      {#if editor.errorMessage}
        <div
          role="alert"
          class="px-5 py-2.5 bg-red-500/10 border-b border-red-500/35 text-red-700 dark:text-red-300 text-xs flex items-center gap-2 shrink-0"
        >
          <AlertCircle size={15} class="shrink-0" aria-hidden="true" />
          <span>{editor.errorMessage}</span>
        </div>
      {/if}

      <div
        class="flex-1 min-h-0 flex flex-col lg:flex-row overflow-hidden divide-y lg:divide-y-0 lg:divide-x divide-border"
      >
        <div
          class="{editor.isLyricsMaximized
            ? 'overflow-hidden'
            : 'overflow-y-auto overflow-x-hidden'} w-full lg:w-1/2 flex flex-col bg-card min-h-0"
        >
          {#if editor.isLyricsMaximized}
            <div
              class="px-4 py-2 border-b border-border bg-muted/40 flex items-center justify-between gap-3 text-xs shrink-0 select-none"
            >
              <div class="flex items-center gap-1.5 min-w-0">
                <span class="font-medium text-foreground truncate">
                  {editor.title.trim() || 'Sem título'}
                </span>
                {#each metadataSummary as part}
                  <span class="text-muted-foreground shrink-0" aria-hidden="true">·</span>
                  <span class="text-muted-foreground truncate">{part}</span>
                {/each}
              </div>
              <button
                type="button"
                onclick={() => editor.toggleLyricsMaximized()}
                class="h-7 px-2 rounded-md bg-secondary hover:bg-muted border border-border text-foreground text-[11px] font-medium flex items-center shrink-0 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                title={mode === 'song'
                  ? 'Abrir formulário de metadados da música'
                  : 'Abrir formulário de metadados da apresentação'}
              >
                Editar metadados
              </button>
            </div>
          {:else}
            <SongEditorMetadata
              {mode}
              bind:title={editor.title}
              bind:artist={editor.artist}
              bind:composer={editor.composer}
              bind:songKey={editor.songKey}
              bind:bpm={editor.bpm}
              bind:tags={editor.tags}
              bind:audioUrl={editor.audioUrl}
              titleError={editor.titleError}
              ontitleinput={() => editor.clearTitleError()}
              oncollapsemadata={() => editor.toggleLyricsMaximized()}
            />
          {/if}
          <SongEditorToolbar
            {mode}
            bind:alignH={editor.alignH}
            bind:alignV={editor.alignV}
            onbold={() => editor.applyBold()}
            onitalic={() => editor.applyItalic()}
            onunderline={() => editor.applyUnderline()}
            onhighlight={() => editor.applyHighlight()}
            onformatlines={() => editor.applyLinesPerSlide()}
            oninsertsection={(label) => editor.insertSectionTag(label)}
            onopensectionmenu={(event) => editor.handleOpenSectionMenu(event)}
            isLyricsMaximized={editor.isLyricsMaximized}
            ontogglelyricsmaximize={() => editor.toggleLyricsMaximized()}
          />
          <SongEditorLyrics
            {mode}
            bind:lyrics={editor.lyricsRaw}
            bind:textareaEl={editor.textareaEl}
          />
        </div>

        <SongEditorPreview
          {mode}
          slides={editor.parsed.slides}
          alignH={editor.alignH}
          alignV={editor.alignV}
          onfocusblock={(idx) => editor.focusSlideBlock(idx)}
          onslidecontextmenu={(event, slide, idx) =>
            editor.handleSlideContextMenu(event, slide, idx)}
          onslideactions={(event, slide, idx) => editor.handleSlideActionsMenu(event, slide, idx)}
        />
      </div>

      <footer
        class="min-h-14 px-4 sm:px-5 py-2 border-t border-border bg-muted/40 flex flex-wrap items-center justify-between gap-3 shrink-0"
      >
        {#if mode === 'song'}
          <label class="flex items-center gap-2 cursor-pointer select-none min-h-[28px]">
            <input
              type="checkbox"
              bind:checked={editor.addToSetlist}
              class="w-4 h-4 rounded border-border bg-background text-primary focus:ring-primary/40 accent-primary"
            />
            <span class="text-xs text-foreground font-medium">Adicionar à Setlist ao salvar</span>
          </label>
        {:else}
          <div class="text-xs text-muted-foreground">Será salvo diretamente na Setlist ativa.</div>
        {/if}

        <div class="flex items-center gap-2.5 shrink-0">
          <Button
            variant="ghost"
            size="sm"
            onclick={() => editor.requestClose()}
            disabled={editor.isSaving}
            class="min-h-[36px]"
          >
            Cancelar
          </Button>
          <Button
            variant="default"
            size="sm"
            onclick={() => editor.saveContent()}
            disabled={editor.isSaving}
            aria-busy={editor.isSaving}
            class="gap-1.5 min-w-[120px] min-h-[36px]"
          >
            {#if editor.isSaving}
              <span
                class="inline-block w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"
                aria-hidden="true"
              ></span>
              Salvando...
            {:else}
              <Save size={14} aria-hidden="true" />
              {mode === 'song' ? 'Salvar Música' : 'Salvar Apresentação'}
            {/if}
          </Button>
        </div>
      </footer>
    </div>

    <SongEditorSectionName
      open={editor.showSectionName}
      bind:value={editor.sectionName}
      error={editor.sectionNameError}
      isInsert={editor.sectionNameIsInsert}
      oninput={() => (editor.sectionNameError = '')}
      onconfirm={() => editor.confirmSectionName()}
      oncancel={() => editor.cancelSectionName()}
    />

    <SongEditorConfirmDiscard
      open={editor.showDiscardConfirm}
      onconfirm={() => editor.confirmDiscard()}
      oncancel={() => editor.cancelDiscard()}
    />

    <ContextMenu
      open={editor.contextMenuOpen}
      x={editor.contextMenuX}
      y={editor.contextMenuY}
      items={editor.contextMenuItems}
      onclose={() => editor.closeContextMenu()}
    />
  </div>
{/if}

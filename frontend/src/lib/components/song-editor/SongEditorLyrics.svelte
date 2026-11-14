<script lang="ts">
  import type { SongEditorMode } from './types';

  interface Props {
    mode: SongEditorMode;
    lyrics?: string;
    textareaEl?: HTMLTextAreaElement | null;
    onkeydown?: (event: KeyboardEvent) => void;
  }

  let {
    mode,
    lyrics = $bindable(''),
    textareaEl = $bindable(null),
    onkeydown,
  }: Props = $props();

  const lineCount = $derived(lyrics ? lyrics.split(/\r?\n/).length : 0);
  const sectionCount = $derived((lyrics.match(/^#{1,2}\s+\S/gm) ?? []).length);
  const hintId = 'song-editor-lyrics-hint';
</script>

<div class="flex-1 min-h-[260px] flex flex-col overflow-hidden">
  <div
    id={hintId}
    class="px-4 py-1.5 bg-muted/30 text-[11px] text-muted-foreground flex items-center justify-between gap-3 shrink-0 border-b border-border/60"
  >
    <span class="min-w-0 leading-snug">
      Use <code class="font-mono text-foreground">## Rótulo</code> para nomear slides. Linha em branco
      separa slides. Suporta <code class="font-mono text-foreground">**negrito**</code> e
      <code class="font-mono text-foreground">==destaque==</code>.
    </span>
    <span class="font-mono shrink-0">
      {lineCount} {lineCount === 1 ? 'linha' : 'linhas'} · {sectionCount}
      {sectionCount === 1 ? 'seção' : 'seções'} · {lyrics.length}
      {lyrics.length === 1 ? 'caractere' : 'caracteres'}
    </span>
  </div>

  <div class="flex-1 min-h-0 p-4 flex flex-col">
    <textarea
      bind:this={textareaEl}
      bind:value={lyrics}
      onkeydown={onkeydown}
      placeholder={mode === 'song'
        ? 'Digite ou cole a letra da música aqui...'
        : 'Digite os slides da sua apresentação aqui...'}
      aria-label={mode === 'song' ? 'Letra da música' : 'Conteúdo da apresentação'}
      aria-describedby={hintId}
      spellcheck="true"
      class="w-full flex-1 min-h-0 p-3 bg-background border border-border rounded-lg text-sm text-foreground placeholder:text-muted-foreground font-mono leading-relaxed resize-none focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/40"
    ></textarea>
  </div>
</div>

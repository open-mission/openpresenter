<script lang="ts">
  import { QUICK_ITEM_CONFIG } from './defaults';
  import type { QuickItemMode } from './types';

  interface Props {
    mode: QuickItemMode;
    isMaximized: boolean;
    title: string;
    content: string;
    slideCount: number;
    onupdatetitle: (val: string) => void;
    onupdatecontent: (val: string) => void;
  }

  let {
    mode,
    isMaximized,
    title,
    content,
    slideCount,
    onupdatetitle,
    onupdatecontent,
  }: Props = $props();

  const config = $derived(QUICK_ITEM_CONFIG[mode]);

  function handleTitleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onupdatetitle(target.value);
  }

  function handleContentInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    onupdatecontent(target.value);
  }
</script>

<div class="p-5 space-y-4 flex-1 flex flex-col min-h-0 overflow-y-auto">
  <div class="space-y-1.5 shrink-0">
    <label for="item-title" class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
      {config.titleLabel}
    </label>
    <input
      id="item-title"
      type="text"
      value={title}
      oninput={handleTitleInput}
      placeholder={config.titlePlaceholder}
      class="w-full h-10 px-3.5 bg-background border border-border rounded-xl text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:border-emerald-500/60 transition-colors"
    />
  </div>

  {#if mode === 'announcement'}
    <div class="space-y-1.5 flex-1 flex flex-col min-h-0">
      <div class="flex items-center justify-between">
        <label for="item-content" class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
          Conteúdo dos Slides
        </label>
        <span class="text-xs text-muted-foreground font-mono bg-muted px-2 py-0.5 rounded border border-border">
          {slideCount} {slideCount === 1 ? 'slide' : 'slides'}
        </span>
      </div>
      <textarea
        id="item-content"
        value={content}
        oninput={handleContentInput}
        rows={isMaximized ? undefined : 6}
        placeholder="Digite o texto do slide. Uma linha em branco separa múltiplos slides..."
        class="w-full p-3.5 bg-background border border-border rounded-xl text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:border-emerald-500/60 font-mono {isMaximized
          ? 'flex-1 h-full resize-none'
          : 'resize-none'}"
      ></textarea>
      <span class="text-xs text-muted-foreground shrink-0">
        Dica: Linhas em branco criam novos slides. Pressione ⌘↵ para salvar.
      </span>
    </div>
  {/if}
</div>

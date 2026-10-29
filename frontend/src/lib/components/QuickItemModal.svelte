<script lang="ts">
  import { cn } from '$lib/utils';
  import type { MediaItem, MediaKind, Slide } from '$lib/types';
  import Button from './ui/Button.svelte';
  import { X, FolderPlus, FileText, Bookmark } from 'lucide-svelte';

  interface Props {
    open: boolean;
    mode: 'header' | 'announcement';
    initialItem?: MediaItem | null;
    onclose: () => void;
    onsave: (item: MediaItem) => void;
  }

  let { open = false, mode = 'header', initialItem = null, onclose, onsave }: Props = $props();

  let title = $state('');
  let content = $state('');

  $effect(() => {
    if (open) {
      if (initialItem) {
        title = initialItem.title;
        content = initialItem.slides.map((s) => s.content).join('\n\n');
      } else {
        title = '';
        content = '';
      }
    }
  });

  function handleSave() {
    if (!title.trim()) return;

    let slides: Slide[] = [];

    if (mode === 'header') {
      slides = [];
    } else {
      const blocks = content.trim() ? content.split(/\n\s*\n/) : [title.trim()];
      slides = blocks.map((block, idx) => ({
        id: `slide-${Date.now()}-${idx + 1}`,
        label: `Slide ${idx + 1}`,
        content: block.trim(),
      }));
    }

    const item: MediaItem = {
      id: initialItem?.id || `item-${Date.now()}`,
      title: title.trim(),
      artist_or_ref: mode === 'header' ? 'Divisor de Seção' : 'Texto / Aviso',
      kind: mode,
      slides,
    };

    onsave(item);
    onclose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.stopPropagation();
      onclose();
    } else if (e.key === 'Enter' && (e.metaKey || e.ctrlKey || mode === 'header')) {
      e.preventDefault();
      handleSave();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4 animate-in fade-in duration-150"
    onclick={onclose}
    role="presentation"
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="w-full max-w-md rounded-xl border border-neutral-800 bg-neutral-900 shadow-2xl flex flex-col overflow-hidden text-neutral-100"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="-1"
    >
      <!-- Cabeçalho -->
      <div class="h-14 px-5 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-900/60">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 flex items-center justify-center">
            {#if mode === 'header'}
              <Bookmark size={16} />
            {:else}
              <FileText size={16} />
            {/if}
          </div>
          <div>
            <h2 class="text-sm font-bold text-white">
              {initialItem
                ? 'Editar Item'
                : mode === 'header'
                  ? 'Nova Seção na Setlist'
                  : 'Novo Texto Livre / Aviso'}
            </h2>
            <p class="text-[11px] text-neutral-400">
              {mode === 'header'
                ? 'Cria um divisor visual para organizar sua programação'
                : 'Cria slides de texto livre ou anúncios'}
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="w-8 h-8 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
        >
          <X size={16} />
        </button>
      </div>

      <!-- Conteúdo -->
      <div class="p-5 space-y-4">
        <div class="space-y-1.5">
          <label for="item-title" class="text-xs font-semibold text-neutral-300">
            {mode === 'header' ? 'Nome da Seção' : 'Título do Aviso / Texto'}
          </label>
          <!-- svelte-ignore a11y_autofocus -->
          <input
            id="item-title"
            type="text"
            bind:value={title}
            placeholder={mode === 'header'
              ? 'Ex.: Momento de Louvor, Dízimos e Ofertas, Palavra Pastoral...'
              : 'Ex.: Boas-Vindas aos Visitantes, Avisos da Semana...'}
            class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500/60 focus:ring-1 focus:ring-emerald-500/40"
            autofocus
          />
        </div>

        {#if mode === 'announcement'}
          <div class="space-y-1.5">
            <label for="item-content" class="text-xs font-semibold text-neutral-300">
              Conteúdo dos Slides
            </label>
            <textarea
              id="item-content"
              bind:value={content}
              rows="5"
              placeholder="Digite o texto do slide. Uma linha em branco separa múltiplos slides..."
              class="w-full p-3 bg-neutral-950 border border-neutral-800 rounded-lg text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500/60 focus:ring-1 focus:ring-emerald-500/40 resize-none font-mono"
            ></textarea>
            <span class="text-[10px] text-neutral-500">
              Dica: Linhas em branco criam novos slides.
            </span>
          </div>
        {/if}
      </div>

      <!-- Rodapé -->
      <div class="h-14 px-5 border-t border-neutral-800 bg-neutral-900/60 flex items-center justify-end gap-2 shrink-0">
        <Button variant="secondary" size="sm" onclick={onclose}>
          Cancelar
        </Button>
        <Button
          variant="default"
          size="sm"
          onclick={handleSave}
          disabled={!title.trim()}
          class="bg-emerald-500 hover:bg-emerald-400 text-black font-semibold"
        >
          Salvar
        </Button>
      </div>
    </div>
  </div>
{/if}

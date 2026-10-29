<script lang="ts">
  import { X, Play, ListPlus, Copy, Check, ExternalLink, Image as ImageIcon, Info, Maximize2 } from 'lucide-svelte';
  import type { ImageMediaItem } from '$lib/types';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';

  let {
    open = false,
    image = null,
    onclose,
    onpresent,
    onaddsetlist,
  }: {
    open: boolean;
    image: ImageMediaItem | null;
    onclose: () => void;
    onpresent?: (image: ImageMediaItem) => void;
    onaddsetlist?: (image: ImageMediaItem) => void;
  } = $props();

  let naturalWidth = $state<number>(0);
  let naturalHeight = $state<number>(0);
  let isCopied = $state(false);

  // Calcula a proporção mais comum simplificada
  const aspectRatioText = $derived.by(() => {
    if (!naturalWidth || !naturalHeight) return 'Desconhecida';
    const ratio = naturalWidth / naturalHeight;
    if (Math.abs(ratio - 16 / 9) < 0.05) return '16:9 (Widescreen)';
    if (Math.abs(ratio - 4 / 3) < 0.05) return '4:3 (Tradicional)';
    if (Math.abs(ratio - 1) < 0.05) return '1:1 (Quadrado)';
    if (Math.abs(ratio - 9 / 16) < 0.05) return '9:16 (Vertical)';
    if (Math.abs(ratio - 21 / 9) < 0.05) return '21:9 (Ultrawide)';
    return `${(ratio).toFixed(2)}:1`;
  });

  const fileExtension = $derived.by(() => {
    if (!image) return 'IMG';
    const parts = image.name.split('.');
    return parts.length > 1 ? parts.pop()?.toUpperCase() || 'IMG' : 'IMG';
  });

  $effect(() => {
    if (open && image) {
      naturalWidth = 0;
      naturalHeight = 0;
      isCopied = false;
      const img = new Image();
      img.src = image.url;
      img.onload = () => {
        naturalWidth = img.naturalWidth;
        naturalHeight = img.naturalHeight;
      };
    }
  });

  async function handleCopyUrl() {
    if (!image) return;
    try {
      const fullUrl = window.location.origin + image.url;
      await navigator.clipboard.writeText(fullUrl);
      isCopied = true;
      setTimeout(() => (isCopied = false), 2000);
    } catch (err) {
      console.error('Falha ao copiar URL:', err);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) {
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open && image}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/85 backdrop-blur-md animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
    aria-labelledby="preview-title"
  >
    <div
      class="bg-neutral-900 border border-neutral-800 rounded-2xl w-full max-w-4xl max-h-[90vh] flex flex-col shadow-2xl overflow-hidden"
    >
      <!-- Cabeçalho do Modal -->
      <div class="px-5 py-3.5 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-950/60">
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="p-1.5 rounded-lg bg-primary/10 text-primary">
            <ImageIcon size={18} />
          </div>
          <div class="min-w-0">
            <h3 id="preview-title" class="text-sm font-bold text-white truncate max-w-md">
              {image.name}
            </h3>
            <div class="flex items-center gap-2 text-[11px] text-neutral-400 mt-0.5">
              <span>{image.folder_name}</span>
              <span>•</span>
              <Badge variant="outline" class="text-[10px] py-0 px-1 font-mono uppercase">
                {fileExtension}
              </Badge>
              {#if naturalWidth > 0}
                <span>•</span>
                <span class="font-mono text-neutral-300">{naturalWidth} × {naturalHeight} px</span>
              {/if}
            </div>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="p-1.5 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
          aria-label="Fechar prévia"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Área de Prévia em Alta Resolução -->
      <div class="flex-1 bg-black flex items-center justify-center p-4 min-h-[300px] max-h-[60vh] overflow-hidden relative select-none">
        <img
          src={image.url}
          alt={image.name}
          class="max-w-full max-h-[56vh] object-contain rounded shadow-2xl"
        />

        {#if aspectRatioText}
          <div class="absolute bottom-3 left-3 bg-black/70 backdrop-blur-md px-2.5 py-1 rounded-md border border-neutral-800 text-[11px] font-mono text-neutral-300 flex items-center gap-1.5">
            <Maximize2 size={12} class="text-primary" />
            <span>{aspectRatioText}</span>
          </div>
        {/if}
      </div>

      <!-- Rodapé com Metadados e Ações -->
      <div class="p-4 bg-neutral-950/80 border-t border-neutral-800 flex flex-wrap items-center justify-between gap-3 shrink-0">
        <!-- Metadados da Imagem -->
        <div class="flex items-center gap-4 text-xs text-neutral-400">
          <div class="flex items-center gap-1.5">
            <Info size={13} class="text-neutral-500" />
            <span>Origem: <strong class="text-neutral-200">{image.is_external ? 'Pasta Mapeada' : 'Interna do Sistema'}</strong></span>
          </div>
          {#if naturalWidth > 0}
            <div class="hidden sm:inline">
              Resolução: <strong class="text-neutral-200 font-mono">{naturalWidth} × {naturalHeight}</strong>
            </div>
          {/if}
        </div>

        <!-- Botões de Ação -->
        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onclick={handleCopyUrl}
            class="text-xs gap-1.5"
            title="Copiar URL direta da imagem"
          >
            {#if isCopied}
              <Check size={13} class="text-emerald-400" />
              <span class="text-emerald-400 font-semibold">Copiado!</span>
            {:else}
              <Copy size={13} />
              <span>Copiar URL</span>
            {/if}
          </Button>

          {#if onaddsetlist}
            <Button
              variant="outline"
              size="sm"
              onclick={() => {
                onaddsetlist(image!);
                onclose();
              }}
              class="text-xs gap-1.5"
              title="Adicionar à Setlist do Culto"
            >
              <ListPlus size={14} />
              <span>Adicionar à Setlist</span>
            </Button>
          {/if}

          {#if onpresent}
            <Button
              variant="default"
              size="sm"
              onclick={() => {
                onpresent(image!);
                onclose();
              }}
              class="text-xs gap-1.5 font-bold"
              title="Apresentar Imediatamente no Projetor e Palco"
            >
              <Play size={13} class="fill-current" />
              <span>Apresentar Agora</span>
            </Button>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}

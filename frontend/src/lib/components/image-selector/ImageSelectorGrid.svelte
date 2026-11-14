<script lang="ts">
  import { Check, ImageOff } from 'lucide-svelte';
  import type { ImageMediaItem } from '$lib/types';

  interface Props {
    images: ImageMediaItem[];
    selectedImageIds: Set<string>;
    isLowerThird: boolean;
    ontoggle: (id: string) => void;
    ondoubleclick: (url: string) => void;
  }

  let { images, selectedImageIds, isLowerThird, ontoggle, ondoubleclick }: Props = $props();

  function handleKeydown(e: KeyboardEvent, img: ImageMediaItem) {
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      ontoggle(img.id);
    }
  }
</script>

{#if images.length === 0}
  <div class="h-48 flex flex-col items-center justify-center text-center p-6 text-muted-foreground animate-in fade-in duration-150">
    <div
      class="w-12 h-12 rounded-2xl bg-muted/60 border border-border flex items-center justify-center mb-3"
      aria-hidden="true"
    >
      <ImageOff size={24} class="text-muted-foreground" />
    </div>
    <h3 class="text-sm font-semibold text-foreground">Nenhuma imagem encontrada</h3>
    <p class="text-xs text-muted-foreground mt-1 max-w-sm">
      Não encontramos imagens correspondentes ao filtro atual. Tente alterar o termo de busca ou a pasta selecionada.
    </p>
  </div>
{:else}
  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3 select-none">
    {#each images as img (img.id)}
      {@const isSelected = selectedImageIds.has(img.id)}
      <div
        role="checkbox"
        aria-checked={isSelected}
        aria-label={`${img.name} - ${isSelected ? 'Selecionada' : 'Não selecionada'}`}
        tabindex="0"
        onclick={() => ontoggle(img.id)}
        onkeydown={(e) => handleKeydown(e, img)}
        ondblclick={() => ondoubleclick(img.url)}
        class="group relative aspect-video rounded-xl border transition-all cursor-pointer overflow-hidden shadow-xs focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isSelected
          ? 'border-primary ring-2 ring-primary/50'
          : 'border-border bg-card/60 hover:border-primary/40'}"
      >
        <img
          src={img.url}
          alt=""
          aria-hidden="true"
          class="w-full h-full object-cover"
          loading="lazy"
        />

        <!-- Checkbox de Seleção -->
        <div
          class="absolute top-2 right-2 w-5 h-5 rounded-md flex items-center justify-center transition-colors {isSelected
            ? 'bg-primary text-primary-foreground shadow-xs'
            : 'bg-black/60 border border-white/40 text-transparent group-hover:border-white'}"
          aria-hidden="true"
        >
          {#if isSelected}
            <Check size={12} strokeWidth={3} />
          {/if}
        </div>

        <!-- Legenda -->
        <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 via-black/55 to-transparent p-2.5 pt-4">
          <p class="text-xs font-semibold text-white truncate drop-shadow-xs">{img.name}</p>
          <p class="text-[11px] text-neutral-300 truncate mt-0.5">
            {img.folder_name}{img.subfolder ? ` • ${img.subfolder}` : ''}
          </p>
        </div>
      </div>
    {/each}
  </div>
{/if}

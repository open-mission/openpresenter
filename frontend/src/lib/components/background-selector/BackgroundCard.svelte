<script lang="ts">
  import { Check, Sparkles, Film } from 'lucide-svelte';
  import type { BackgroundPreset } from '../../backgrounds';
  import type { WallpaperItem } from '$lib/types';

  interface Props {
    preset?: BackgroundPreset;
    localWallpaper?: WallpaperItem;
    isSelected: boolean;
    onselect: () => void;
  }

  let { preset, localWallpaper, isSelected, onselect }: Props = $props();

  const title = $derived(preset?.name || localWallpaper?.name || 'Plano de Fundo');
  const description = $derived(
    preset?.description || (localWallpaper ? 'Papel de Parede Local' : '')
  );
</script>

<button
  type="button"
  onclick={onselect}
  aria-pressed={isSelected}
  aria-label={`${title} - ${isSelected ? 'Selecionado' : 'Clique para selecionar'}`}
  class="group rounded-xl border overflow-hidden flex flex-col text-left transition-all cursor-pointer bg-card focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring select-none {isSelected
    ? 'border-primary ring-2 ring-primary/40 shadow-md'
    : 'border-border hover:border-primary/50'}"
>
  <!-- Visual 16:9 Prévia -->
  <div class="aspect-video w-full relative overflow-hidden flex items-center justify-center bg-black/60">
    {#if preset}
      <div
        class="absolute inset-0 w-full h-full {preset.animationClass || ''}"
        style="background: {preset.style};"
        aria-hidden="true"
      ></div>

      <!-- Overlay de Texto Sutil para Demonstração de Legibilidade -->
      <span class="relative z-10 px-2.5 py-1 rounded bg-black/50 backdrop-blur-[1px] text-[11px] text-white font-medium">
        Exemplo de Letra
      </span>

      {#if preset.animationClass}
        <span
          class="absolute bottom-2 right-2 px-1.5 py-0.5 rounded bg-black/75 text-amber-300 font-mono text-[11px] flex items-center gap-1 shadow-xs"
        >
          <Sparkles size={11} aria-hidden="true" />
          <span>Animado</span>
        </span>
      {/if}
    {:else if localWallpaper}
      {#if localWallpaper.is_video}
        <video
          src={localWallpaper.url}
          class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
          autoplay
          loop
          muted
          playsinline
          aria-hidden="true"
        ></video>
        <span
          class="absolute top-2 left-2 px-1.5 py-0.5 rounded bg-black/75 border border-white/20 text-[11px] font-mono font-bold text-white flex items-center gap-1 shadow-xs"
        >
          <Film size={12} class="text-primary" aria-hidden="true" />
          <span>Vídeo</span>
        </span>
      {:else}
        <img
          src={localWallpaper.url}
          alt=""
          aria-hidden="true"
          class="w-full h-full object-cover transition-transform group-hover:scale-105 duration-200"
        />
      {/if}
    {/if}

    {#if isSelected}
      <div
        class="absolute top-2 right-2 w-5 h-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center shadow-xs"
        aria-hidden="true"
      >
        <Check size={12} strokeWidth={3} />
      </div>
    {/if}
  </div>

  <!-- Detalhes do Fundo -->
  <div class="p-3 flex items-center justify-between gap-2 border-t border-border bg-card">
    <div class="min-w-0 flex-1">
      <h4 class="text-xs font-bold text-foreground truncate">{title}</h4>
      {#if description}
        <p class="text-[11px] text-muted-foreground truncate mt-0.5">{description}</p>
      {/if}
    </div>
    <span
      class="h-7 text-xs px-2.5 rounded-md font-medium shrink-0 flex items-center justify-center transition-colors {isSelected
        ? 'bg-primary text-primary-foreground font-semibold shadow-xs'
        : 'bg-secondary text-secondary-foreground group-hover:bg-primary/10 group-hover:text-primary'}"
    >
      {isSelected ? 'Selecionado' : 'Escolher'}
    </span>
  </div>
</button>

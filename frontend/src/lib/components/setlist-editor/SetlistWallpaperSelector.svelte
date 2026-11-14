<script lang="ts">
  import { cn } from '$lib/utils';
  import type { WallpaperItem } from '$lib/types';
  import type { ResolvedBackground } from '../../backgrounds';
  import type { WallpaperMode } from './types';
  import { WALLPAPER_MODES } from './defaults';
  import Button from '../ui/Button.svelte';
  import { Check, Palette, Sparkles, Upload } from 'lucide-svelte';

  interface Props {
    selectedWallpaper: string;
    wallpaperMode: WallpaperMode;
    resolvedWallpaper: ResolvedBackground;
    availableWallpapers: WallpaperItem[];
    isUploading: boolean;
    onopenbackgroundmodal: () => void;
    onuploadfile: (e: Event) => void;
    onselectwallpaper: (url: string) => void;
    onselectmode: (mode: WallpaperMode) => void;
  }

  let {
    selectedWallpaper,
    wallpaperMode,
    resolvedWallpaper,
    availableWallpapers,
    isUploading,
    onopenbackgroundmodal,
    onuploadfile,
    onselectwallpaper,
    onselectmode,
  }: Props = $props();

  let fileInputRef = $state<HTMLInputElement | null>(null);
</script>

<div class="space-y-4 pt-3 border-t border-border">
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
    <div>
      <span class="text-xs font-semibold text-foreground block">
        Papel de Parede da Projeção (Público)
      </span>
      <p class="text-xs text-muted-foreground mt-0.5">
        Exibido na tela cheia quando nenhum slide estiver ao vivo ou em modo repouso
      </p>
    </div>

    <!-- Ações de Escolha de Fundo -->
    <div class="flex items-center gap-2 shrink-0">
      <Button
        variant="default"
        size="sm"
        onclick={onopenbackgroundmodal}
        class="gap-1.5 text-xs font-semibold bg-emerald-600 hover:bg-emerald-500 text-white dark:bg-emerald-500 dark:hover:bg-emerald-400 dark:text-black min-h-[32px]"
      >
        <Palette size={14} aria-hidden="true" />
        Explorar Fundos & Gradientes
      </Button>

      <input
        bind:this={fileInputRef}
        id="setlist-wallpaper-upload-input"
        type="file"
        accept="image/*"
        onchange={onuploadfile}
        class="hidden"
        aria-hidden="true"
        tabindex="-1"
      />
      <Button
        variant="outline"
        size="sm"
        onclick={() => fileInputRef?.click()}
        disabled={isUploading}
        class="gap-1.5 text-xs min-h-[32px]"
      >
        <Upload size={14} aria-hidden="true" />
        {isUploading ? 'Enviando...' : 'Carregar Imagem'}
      </Button>
    </div>
  </div>

  <!-- Destaque do Fundo Selecionado (se for CSS/Gradiente) -->
  {#if resolvedWallpaper.isCss}
    <div
      class="p-3.5 rounded-xl border border-primary/40 bg-primary/5 flex items-center justify-between gap-3 animate-in fade-in duration-150"
    >
      <div class="flex items-center gap-3 min-w-0">
        <div class="w-16 aspect-video rounded-lg overflow-hidden border border-border relative shrink-0 shadow-xs">
          <div
            class="w-full h-full {resolvedWallpaper.className}"
            style={resolvedWallpaper.style}
            aria-hidden="true"
          ></div>
        </div>
        <div class="min-w-0">
          <h4 class="text-xs font-bold text-foreground flex items-center gap-1.5 truncate">
            <Sparkles size={14} class="text-primary shrink-0" aria-hidden="true" />
            Fundo CSS / Gradiente Ativo
          </h4>
          <p class="text-xs text-muted-foreground truncate">
            Fundo procedural vetorial de alta definição para a projeção
          </p>
        </div>
      </div>
      <Button
        variant="outline"
        size="sm"
        onclick={onopenbackgroundmodal}
        class="text-xs shrink-0 min-h-[30px]"
      >
        Alterar...
      </Button>
    </div>
  {/if}

  <!-- Grade de Wallpapers Disponíveis -->
  <fieldset class="space-y-2">
    <legend class="sr-only">Opções de papéis de parede disponíveis</legend>
    <div
      role="radiogroup"
      aria-label="Galeria de papéis de parede"
      class="grid grid-cols-2 sm:grid-cols-4 gap-2.5"
    >
      <!-- Opção Sem Papel de Parede (Preto Absoluto) -->
      <button
        type="button"
        role="radio"
        aria-checked={selectedWallpaper === ''}
        aria-label="Preto Puro (Sem papel de parede)"
        onclick={() => onselectwallpaper('')}
        class={cn(
          'aspect-video rounded-lg border flex flex-col items-center justify-center text-center p-2 transition-all cursor-pointer relative bg-black focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
          selectedWallpaper === ''
            ? 'border-emerald-500 ring-2 ring-emerald-500/40 shadow-xs'
            : 'border-border hover:border-muted-foreground/40'
        )}
      >
        <span class="text-xs font-medium text-neutral-300">Preto Puro</span>
        {#if selectedWallpaper === ''}
          <div
            class="absolute top-1.5 right-1.5 w-4 h-4 rounded-full bg-emerald-500 text-black flex items-center justify-center shadow-xs"
            aria-hidden="true"
          >
            <Check size={11} strokeWidth={3} />
          </div>
        {/if}
      </button>

      <!-- Wallpapers Cadastrados na pasta do sistema -->
      {#each availableWallpapers as wall (wall.url)}
        <button
          type="button"
          role="radio"
          aria-checked={selectedWallpaper === wall.url}
          aria-label={`Papel de parede: ${wall.name}`}
          onclick={() => onselectwallpaper(wall.url)}
          class={cn(
            'aspect-video rounded-lg border overflow-hidden relative transition-all cursor-pointer group bg-muted/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
            selectedWallpaper === wall.url
              ? 'border-emerald-500 ring-2 ring-emerald-500/40 shadow-xs'
              : 'border-border hover:border-muted-foreground/40'
          )}
        >
          <img
            src={wall.url}
            alt=""
            aria-hidden="true"
            class="w-full h-full object-cover"
          />
          <span
            class="absolute bottom-0 inset-x-0 bg-background/85 dark:bg-black/80 backdrop-blur-xs text-[11px] font-medium text-foreground px-1.5 py-0.5 truncate text-left"
          >
            {wall.name}
          </span>

          {#if selectedWallpaper === wall.url}
            <div
              class="absolute top-1.5 right-1.5 w-4 h-4 rounded-full bg-emerald-500 text-black flex items-center justify-center shadow-xs"
              aria-hidden="true"
            >
              <Check size={11} strokeWidth={3} />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </fieldset>

  <!-- Modo de Ajuste do Papel de Parede -->
  {#if selectedWallpaper}
    <div class="pt-2 flex flex-col sm:flex-row sm:items-center justify-between gap-2.5">
      <span id="wallpaper-mode-label" class="text-xs font-medium text-foreground">
        Modo de Ajuste do Papel de Parede:
      </span>
      <div
        role="radiogroup"
        aria-labelledby="wallpaper-mode-label"
        class="inline-flex items-center gap-1 bg-muted p-1 rounded-lg border border-border"
      >
        {#each WALLPAPER_MODES as mode (mode.id)}
          <button
            type="button"
            role="radio"
            aria-checked={wallpaperMode === mode.id}
            onclick={() => onselectmode(mode.id)}
            class={cn(
              'px-3 py-1.5 rounded-md text-xs font-medium transition-all cursor-pointer min-h-[30px] flex items-center justify-center focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
              wallpaperMode === mode.id
                ? 'bg-card text-emerald-700 dark:text-emerald-400 font-semibold shadow-xs border border-border'
                : 'text-muted-foreground hover:text-foreground'
            )}
          >
            {mode.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

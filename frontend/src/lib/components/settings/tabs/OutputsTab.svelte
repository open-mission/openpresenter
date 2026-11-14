<script lang="ts">
  import { Monitor, Tv, Radio, ExternalLink } from 'lucide-svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import type { ImageFitMode, PublicDisplaySettings } from '$lib/types';

  interface Props {
    localPublic: PublicDisplaySettings;
    onSelectWallpaperFit: (fit: ImageFitMode | 'repeat') => void;
    onNavigateToPublicTab: () => void;
  }

  let { localPublic, onSelectWallpaperFit, onNavigateToPublicTab }: Props = $props();

  const currentFit = $derived(localPublic.wallpaper_fit || 'contain');

  const fitOptions: Array<{
    id: ImageFitMode | 'repeat';
    label: string;
    desc: string;
  }> = [
    { id: 'contain', label: 'Ajustar (Conter)', desc: '100% visível, sem cortes (ideal para TV)' },
    { id: 'cover', label: 'Preencher (Cobrir)', desc: 'Preenche toda a tela 16:9' },
    { id: 'fill', label: 'Esticar', desc: 'Ajusta largura e altura à força' },
    { id: 'center', label: 'Centralizar', desc: 'Exibe em tamanho original sem zoom' },
    { id: 'repeat', label: 'Repetir', desc: 'Lado a lado em mosaico' },
  ];
</script>

<div class="space-y-6 w-full max-w-4xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Saídas Gráficas</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Abra e gerencie as janelas de projeção pública, retorno de palco e legenda OBS.
    </p>
  </div>

  <div class="space-y-3.5">
    <div class="p-4 rounded-xl border border-border bg-card/60 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div class="flex items-center gap-3 min-w-0">
        <div class="w-9 h-9 rounded-lg bg-emerald-500/15 border border-emerald-500/35 text-emerald-700 dark:text-emerald-300 flex items-center justify-center shrink-0">
          <Monitor size={18} />
        </div>
        <div class="min-w-0">
          <div class="text-xs font-semibold text-foreground flex flex-wrap items-center gap-2">
            <span>Tela Pública (Public)</span>
            <Badge variant="outline" class="text-[11px] text-emerald-700 dark:text-emerald-300 border-emerald-500/35">
              16:9 Nativo
            </Badge>
          </div>
          <div class="text-[11px] text-muted-foreground mt-0.5">
            Projetada para a congregação e captura em <code>/public</code>
          </div>
        </div>
      </div>

      <Button
        variant="outline"
        size="sm"
        onclick={() => window.open('/public', 'OpenPresenterPublic', 'width=1280,height=720')}
        class="text-xs gap-1.5 shrink-0 self-end sm:self-center"
      >
        <span>Abrir Janela</span>
        <ExternalLink size={12} />
      </Button>
    </div>

    <div class="p-4 rounded-xl border border-border bg-card/60 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div class="flex items-center gap-3 min-w-0">
        <div class="w-9 h-9 rounded-lg bg-emerald-500/15 border border-emerald-500/35 text-emerald-700 dark:text-emerald-300 flex items-center justify-center shrink-0">
          <Tv size={18} />
        </div>
        <div class="min-w-0">
          <div class="text-xs font-semibold text-foreground flex flex-wrap items-center gap-2">
            <span>Retorno do Palco (Stage)</span>
            <Badge variant="outline" class="text-[11px] text-emerald-700 dark:text-emerald-300 border-emerald-500/35">
              Relógio + Prévia
            </Badge>
          </div>
          <div class="text-[11px] text-muted-foreground mt-0.5">
            Exibe o slide atual, próximo slide e cronômetro em <code>/stage</code>
          </div>
        </div>
      </div>

      <Button
        variant="outline"
        size="sm"
        onclick={() => window.open('/stage', 'OpenPresenterStage', 'width=1280,height=720')}
        class="text-xs gap-1.5 shrink-0 self-end sm:self-center"
      >
        <span>Abrir Janela</span>
        <ExternalLink size={12} />
      </Button>
    </div>

    <div class="p-4 rounded-xl border border-border bg-card/60 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div class="flex items-center gap-3 min-w-0">
        <div class="w-9 h-9 rounded-lg bg-sky-500/15 border border-sky-500/35 text-sky-700 dark:text-sky-300 flex items-center justify-center shrink-0">
          <Radio size={18} />
        </div>
        <div class="min-w-0">
          <div class="text-xs font-semibold text-foreground flex flex-wrap items-center gap-2">
            <span>Transmissão OBS (Lower Third)</span>
            <Badge variant="outline" class="text-[11px] text-sky-700 dark:text-sky-300 border-sky-500/35">
              Fundo Transparente
            </Badge>
          </div>
          <div class="text-[11px] text-muted-foreground mt-0.5">
            Browser Source para livestreaming em <code>/lower-third</code>
          </div>
        </div>
      </div>

      <Button
        variant="outline"
        size="sm"
        onclick={() => window.open('/lower-third', 'OpenPresenterLowerThird', 'width=1280,height=720')}
        class="text-xs gap-1.5 shrink-0 self-end sm:self-center"
      >
        <span>Abrir Prévia</span>
        <ExternalLink size={12} />
      </Button>
    </div>
  </div>

  <!-- Fonte Única de Ajuste de Papel de Parede -->
  <div class="pt-4 border-t border-border space-y-3">
    <div class="flex flex-wrap items-start justify-between gap-2">
      <div>
        <h3 class="text-sm font-semibold text-foreground">Ajuste do Papel de Parede na Tela Pública</h3>
        <p class="text-xs text-muted-foreground mt-0.5">
          Modo padrão de renderização aplicado em tempo real à saída <code>/public</code> e prévias ao vivo.
        </p>
      </div>
      <Button variant="ghost" size="sm" onclick={onNavigateToPublicTab} class="text-xs text-primary">
        Margens na Tela Pública →
      </Button>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2.5" role="radiogroup" aria-label="Modo de encaixe do papel de parede">
      {#each fitOptions as mode (mode.id)}
        {@const active = currentFit === mode.id}
        <button
          type="button"
          role="radio"
          aria-checked={active}
          onclick={() => onSelectWallpaperFit(mode.id)}
          class="p-3 rounded-lg border text-left transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active ? 'border-primary bg-primary/10 ring-1 ring-primary/40 text-foreground font-semibold' : 'border-border bg-card/60 hover:bg-muted/60 text-muted-foreground hover:text-foreground'}"
        >
          <div class="text-xs font-semibold {active ? 'text-primary' : 'text-foreground'}">
            {active ? '✓ ' : ''}{mode.label}
          </div>
          <div class="text-[11px] text-muted-foreground font-normal mt-0.5">{mode.desc}</div>
        </button>
      {/each}
    </div>
  </div>
</div>

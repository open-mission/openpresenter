<script lang="ts">
  import { Check, RotateCcw } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { COLOR_PRESETS, type CustomSectionConfig } from '$lib/settings';

  interface Props {
    sections: CustomSectionConfig[];
    onNameChange: (idx: number, name: string) => void;
    onShortcutChange: (idx: number, key: string) => void;
    onColorChange: (idx: number, presetId: string) => void;
    onReset: () => void;
  }

  let {
    sections,
    onNameChange,
    onShortcutChange,
    onColorChange,
    onReset,
  }: Props = $props();
</script>

<div class="space-y-5 w-full max-w-4xl">
  <div class="flex flex-wrap items-center justify-between gap-3">
    <div>
      <h3 class="text-sm font-semibold text-foreground">Partes da Música e Atalhos</h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Personalize o rótulo, a cor de destaque e a tecla de atalho de cada parte da música.
      </p>
    </div>

    <Button variant="outline" size="sm" onclick={onReset} class="text-xs gap-1.5">
      <RotateCcw size={13} />
      Restaurar Padrões
    </Button>
  </div>

  <div class="space-y-2.5">
    {#each sections as section, idx (section.id)}
      <div class="p-3 rounded-xl border border-border bg-card/60 flex flex-col md:flex-row md:items-center justify-between gap-3">
        <div class="flex items-center gap-3 min-w-0">
          <div class="px-2.5 py-1 rounded text-xs font-semibold border flex items-center gap-1.5 {section.bg} {section.border} {section.text} min-w-[96px] justify-center shrink-0">
            <span class="w-2 h-2 rounded-full {section.accent}"></span>
            <span>{section.name}</span>
          </div>

          <input
            type="text"
            aria-label="Nome da parte {section.name}"
            value={section.name}
            oninput={(e) => onNameChange(idx, (e.target as HTMLInputElement).value)}
            class="h-8 w-36 px-2.5 rounded bg-background border border-border text-xs text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            title="Nome da parte"
          />
        </div>

        <div class="flex flex-wrap items-center gap-3">
          <div class="flex items-center gap-1.5">
            <label for="section-shortcut-{section.id}" class="text-[11px] text-muted-foreground">Atalho:</label>
            <input
              id="section-shortcut-{section.id}"
              type="text"
              maxlength="1"
              value={section.shortcut.toUpperCase()}
              oninput={(e) => onShortcutChange(idx, (e.target as HTMLInputElement).value)}
              class="h-8 w-9 text-center font-mono font-bold uppercase rounded bg-background border border-border text-xs text-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              title="Pressione esta letra para pular diretamente para esta parte durante a música"
            />
          </div>

          <div class="flex items-center gap-1.5" role="radiogroup" aria-label="Cor da seção {section.name}">
            {#each COLOR_PRESETS as preset (preset.id)}
              {@const selected = section.colorName === preset.name}
              <button
                type="button"
                role="radio"
                aria-checked={selected}
                aria-label="Cor {preset.name} para {section.name}"
                title={preset.name}
                onclick={() => onColorChange(idx, preset.id)}
                class="w-6 h-6 rounded-full {preset.accent} transition-transform hover:scale-110 flex items-center justify-center cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-1 focus-visible:ring-offset-background {selected ? 'ring-2 ring-foreground scale-105' : 'opacity-80'}"
              >
                {#if selected}
                  <Check size={12} class="text-white drop-shadow" />
                {/if}
              </button>
            {/each}
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>

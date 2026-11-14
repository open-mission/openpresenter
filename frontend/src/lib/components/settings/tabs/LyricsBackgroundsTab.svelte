<script lang="ts">
  import {
    Plus,
    Sparkles,
    Music,
    BookOpen,
    CalendarDays,
    Type,
    RotateCcw,
    Check,
    Trash2,
    Sliders,
  } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { getAllBackgrounds, type BackgroundPreset, type BackgroundStylesMap } from '$lib/backgrounds';
  import type { PublicDisplaySettings } from '$lib/types';
  import type { SettingsConfirmRequest } from '../types';

  interface Props {
    localBgStyles: BackgroundStylesMap;
    localPublic: PublicDisplaySettings;
    onUpdateStyles: (next: BackgroundStylesMap) => void;
    onRequestConfirm: (req: SettingsConfirmRequest) => void;
  }

  let { localBgStyles, localPublic, onUpdateStyles, onRequestConfirm }: Props = $props();

  let selectedPresetId = $state('grad-obsidian');
  let bgCategoryFilter = $state<'all' | 'gradients' | 'mesh' | 'animated' | 'custom'>('all');
  let bgSearchTerm = $state('');
  let bgPreviewMode = $state<'lyrics' | 'scripture' | 'schedule'>('lyrics');

  const QUICK_TEXT_COLORS = [
    { label: 'Branco Puro', hex: '#FFFFFF' },
    { label: 'Off-White', hex: '#F8FAFC' },
    { label: 'Ouro / Dourado', hex: '#FACC15' },
    { label: 'Âmbar Quente', hex: '#F59E0B' },
    { label: 'Amarelo Suave', hex: '#FEF08A' },
    { label: 'Ciano Gelo', hex: '#67E8F9' },
    { label: 'Menta Verde', hex: '#86EFAC' },
    { label: 'Rosa Claro', hex: '#F472B6' },
  ];

  const QUICK_GRADIENTS = [
    {
      name: 'Índigo Noturno',
      css: 'linear-gradient(135deg, #090d16 0%, #1e1b4b 60%, #020617 100%)',
    },
    {
      name: 'Púrpura Profundo',
      css: 'linear-gradient(135deg, #18052e 0%, #3b0764 50%, #090114 100%)',
    },
    {
      name: 'Vinho Nobre',
      css: 'linear-gradient(135deg, #1a050d 0%, #4c0519 50%, #0a0105 100%)',
    },
    {
      name: 'Floresta Escura',
      css: 'linear-gradient(135deg, #022c22 0%, #064e3b 50%, #021a14 100%)',
    },
    {
      name: 'Azul Meia-Noite',
      css: 'linear-gradient(135deg, #030712 0%, #0c4a6e 50%, #020617 100%)',
    },
    {
      name: 'Radial Sombrio',
      css: 'radial-gradient(ellipse at center, #2e1a05 0%, #0d0a06 60%, #000000 100%)',
    },
    {
      name: 'Mesh Aurora',
      css: 'radial-gradient(at 0% 0%, #312e81 0px, transparent 50%), radial-gradient(at 100% 0%, #701a75 0px, transparent 50%), radial-gradient(at 100% 100%, #1e1b4b 0px, transparent 50%), #020617',
    },
  ];

  const allAvailableBackgrounds = $derived(getAllBackgrounds(localBgStyles));

  const filteredBgList = $derived.by(() => {
    const query = bgSearchTerm.trim().toLowerCase();
    return allAvailableBackgrounds.filter((p) => {
      const matchesCat = bgCategoryFilter === 'all' || p.category === bgCategoryFilter;
      const matchesSearch =
        query === '' ||
        p.name.toLowerCase().includes(query) ||
        p.id.toLowerCase().includes(query);
      return matchesCat && matchesSearch;
    });
  });

  const currentSelectedPreset = $derived.by<BackgroundPreset>(() => {
    return (
      allAvailableBackgrounds.find((p) => p.id === selectedPresetId) ||
      allAvailableBackgrounds[0] || {
        id: 'grad-obsidian',
        name: 'Obsidiana Escuro',
        category: 'gradients',
        style: 'linear-gradient(135deg, #09090b 0%, #18181b 50%, #09090b 100%)',
        textColor: '#ffffff',
      }
    );
  });

  function patchSelectedPreset(patch: Partial<BackgroundPreset>) {
    if (!currentSelectedPreset) return;
    if (currentSelectedPreset.isCustom) {
      const nextCustom = localBgStyles.customPresets.map((p) =>
        p.id === currentSelectedPreset.id ? { ...p, ...patch } : p
      );
      onUpdateStyles({ ...localBgStyles, customPresets: nextCustom });
    } else {
      const prev = localBgStyles.overrides[currentSelectedPreset.id] || {};
      onUpdateStyles({
        ...localBgStyles,
        overrides: {
          ...localBgStyles.overrides,
          [currentSelectedPreset.id]: { ...prev, ...patch },
        },
      });
    }
  }

  function createNewCustomPreset() {
    const newId = `custom-${Date.now()}`;
    const newPreset: BackgroundPreset = {
      id: newId,
      name: 'Novo Fundo Customizado',
      category: 'custom',
      style: 'linear-gradient(135deg, #0f172a 0%, #1e1b4b 50%, #312e81 100%)',
      textColor: '#ffffff',
      animationClass: undefined,
      isCustom: true,
    };
    onUpdateStyles({
      ...localBgStyles,
      customPresets: [...localBgStyles.customPresets, newPreset],
    });
    selectedPresetId = newId;
    bgCategoryFilter = 'custom';
  }

  function requestDeleteCustomPreset(preset: BackgroundPreset) {
    onRequestConfirm({
      title: 'Excluir Fundo Customizado',
      description: `Deseja excluir permanentemente o fundo "${preset.name}"?`,
      confirmText: 'Excluir Fundo',
      variant: 'danger',
      onConfirm: () => {
        onUpdateStyles({
          ...localBgStyles,
          customPresets: localBgStyles.customPresets.filter((p) => p.id !== preset.id),
          defaultBackgroundId:
            localBgStyles.defaultBackgroundId === preset.id
              ? undefined
              : localBgStyles.defaultBackgroundId,
        });
        selectedPresetId = 'grad-obsidian';
      },
    });
  }

  function restorePresetDefault(id: string) {
    if (!localBgStyles.overrides[id]) return;
    const copy = { ...localBgStyles.overrides };
    delete copy[id];
    onUpdateStyles({ ...localBgStyles, overrides: copy });
  }

  function setDefaultBackground(id: string) {
    onUpdateStyles({
      ...localBgStyles,
      defaultBackgroundId: localBgStyles.defaultBackgroundId === id ? undefined : id,
    });
  }
</script>

<div class="space-y-6">
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
    <div>
      <h3 class="text-sm font-semibold text-foreground">Estilos, Cores de Texto e Fundos de Letras</h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Personalize o CSS dos planos de fundo, defina cores de texto para máxima legibilidade e crie novos fundos exclusivos.
      </p>
    </div>
    <Button
      variant="outline"
      size="sm"
      onclick={createNewCustomPreset}
      class="gap-1.5 shrink-0 text-xs border-primary/40 text-primary hover:bg-primary/10"
    >
      <Plus size={14} />
      <span>Novo Fundo Customizado</span>
    </Button>
  </div>

  <!-- Simulação 16:9 em Tempo Real -->
  <div class="rounded-2xl border border-border bg-card p-4 sm:p-5 flex flex-col items-center">
    <div class="w-full flex flex-col sm:flex-row items-center justify-between gap-2 mb-3 px-1">
      <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
        <Sparkles size={13} class="text-primary shrink-0" />
        <span>Prévia em Tempo Real (16:9)</span>
      </span>

      <div
        class="flex items-center rounded-lg border border-border bg-muted/50 p-0.5 text-xs"
        role="group"
        aria-label="Tipo de conteúdo da prévia"
      >
        <button
          type="button"
          aria-pressed={bgPreviewMode === 'lyrics'}
          onclick={() => (bgPreviewMode = 'lyrics')}
          class="min-h-7 px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {bgPreviewMode === 'lyrics'
            ? 'bg-primary text-primary-foreground font-semibold shadow-xs'
            : 'text-muted-foreground hover:text-foreground'}"
        >
          <Music size={12} />
          <span>Letra de Louvor</span>
        </button>
        <button
          type="button"
          aria-pressed={bgPreviewMode === 'scripture'}
          onclick={() => (bgPreviewMode = 'scripture')}
          class="min-h-7 px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {bgPreviewMode === 'scripture'
            ? 'bg-primary text-primary-foreground font-semibold shadow-xs'
            : 'text-muted-foreground hover:text-foreground'}"
        >
          <BookOpen size={12} />
          <span>Texto Bíblico</span>
        </button>
        <button
          type="button"
          aria-pressed={bgPreviewMode === 'schedule'}
          onclick={() => (bgPreviewMode = 'schedule')}
          class="min-h-7 px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer flex items-center gap-1.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {bgPreviewMode === 'schedule'
            ? 'bg-primary text-primary-foreground font-semibold shadow-xs'
            : 'text-muted-foreground hover:text-foreground'}"
        >
          <CalendarDays size={12} />
          <span>Agenda</span>
        </button>
      </div>

      <div class="flex items-center gap-2">
        {#if localBgStyles.defaultBackgroundId === currentSelectedPreset.id}
          <span class="text-[11px] bg-primary/15 text-primary border border-primary/30 px-2 py-0.5 rounded-full font-semibold">
            Fundo Padrão
          </span>
        {/if}
        <span class="text-[11px] font-mono text-muted-foreground">{currentSelectedPreset.name}</span>
      </div>
    </div>

    <!-- Tela de projeção 16:9 -->
    <div class="w-full max-w-xl aspect-video rounded-xl border border-border relative overflow-hidden shadow-2xl flex flex-col justify-between p-6 sm:p-8 select-none bg-zinc-950">
      <div
        class="absolute inset-0 w-full h-full {currentSelectedPreset.animationClass === 'none'
          ? ''
          : currentSelectedPreset.animationClass || ''}"
        style="background: {currentSelectedPreset.style};"
      ></div>

      {#if bgPreviewMode === 'lyrics'}
        <div class="relative z-10 my-auto text-center flex flex-col items-center justify-center">
          <p
            class="text-base sm:text-2xl font-bold leading-relaxed tracking-tight drop-shadow-[0_4px_18px_rgba(0,0,0,0.95)]"
            style="color: {currentSelectedPreset.textColor || '#ffffff'};"
          >
            Grandioso és Tu, Senhor,<br />
            ao contemplar os céus e a terra!
          </p>
          <span
            class="text-[11px] sm:text-xs mt-2 opacity-80 font-medium tracking-wide"
            style="color: {currentSelectedPreset.textColor || '#ffffff'};"
          >
            Harpa Cristã • 526
          </span>
        </div>
      {:else if bgPreviewMode === 'scripture'}
        <div class="relative z-10 my-auto text-center flex flex-col items-center justify-center">
          <p
            class="text-sm sm:text-lg font-bold leading-relaxed tracking-tight drop-shadow-[0_4px_18px_rgba(0,0,0,0.95)]"
            style="color: {currentSelectedPreset.textColor || '#ffffff'};"
          >
            Disse também Deus: Haja luzeiros no firmamento dos céus, para fazerem separação entre o dia e a noite; e sejam eles para sinais, para estações, para dias e anos.
          </p>

          {#if localPublic.scripture_show_reference !== false && localPublic.scripture_reference_position === 'below_text'}
            <div class="mt-3 flex items-center justify-center gap-2 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
              <span
                class="text-xs sm:text-sm font-bold tracking-tight opacity-90"
                style="color: {currentSelectedPreset.textColor || '#ffffff'};"
              >
                Gênesis 1:14
              </span>
              <span class="text-[11px] px-1.5 py-0.5 rounded bg-white/15 border border-white/25 uppercase font-mono font-bold text-white">
                ARA
              </span>
            </div>
          {/if}
        </div>

        {#if localPublic.scripture_show_reference !== false && (localPublic.scripture_reference_position ?? 'footer') === 'footer'}
          <div class="relative z-10 flex items-center justify-between border-t border-white/15 pt-2 drop-shadow-[0_2px_8px_rgba(0,0,0,0.9)]">
            <span
              class="font-bold tracking-tight"
              style="color: {currentSelectedPreset.textColor || '#ffffff'}; font-size: {Math.max(12, Math.round((localPublic.scripture_reference_font_size ?? 28) * 0.45))}px;"
            >
              Gênesis 1:14
            </span>
            <span class="text-[11px] px-2 py-0.5 rounded bg-white/15 border border-white/25 uppercase font-mono font-bold text-white shadow">
              ARA
            </span>
          </div>
        {/if}
      {:else}
        <div
          class="relative z-10 my-auto w-full flex flex-col {localPublic.schedule_align_h === 'left'
            ? 'items-start text-left'
            : localPublic.schedule_align_h === 'right'
              ? 'items-end text-right'
              : 'items-center text-center'}"
        >
          <div
            class="rounded-xl transition-all duration-300 w-full {localPublic.schedule_box_background !== false
              ? 'border border-white/20 shadow-2xl backdrop-blur-md'
              : ''} {localPublic.schedule_padding === 'compact'
              ? 'p-3'
              : localPublic.schedule_padding === 'spacious'
                ? 'p-6'
                : 'p-4'}"
            style="max-width: {localPublic.schedule_max_width_percent ?? 90}%; background-color: {localPublic.schedule_box_background !== false ? `rgba(0, 0, 0, ${(localPublic.schedule_bg_opacity ?? 80) / 100})` : 'transparent'};"
          >
            {#if localPublic.schedule_show_badges !== false}
              <div
                class="flex flex-wrap items-center gap-1.5 mb-2 {localPublic.schedule_align_h === 'left'
                  ? 'justify-start'
                  : localPublic.schedule_align_h === 'right'
                    ? 'justify-end'
                    : 'justify-center'}"
              >
                <span
                  class="px-2 py-0.5 rounded font-black tracking-wider uppercase border border-amber-400/40 bg-amber-400/20 text-amber-200 shadow-sm"
                  style="font-size: {Math.max(11, Math.round((localPublic.schedule_day_font_size ?? 18) * 0.55))}px;"
                >
                  Quarta-Feira
                </span>
                <span
                  class="px-2 py-0.5 rounded font-bold font-mono border border-cyan-400/40 bg-cyan-400/20 text-cyan-200 shadow-sm"
                  style="font-size: {Math.max(11, Math.round((localPublic.schedule_time_font_size ?? 16) * 0.65))}px;"
                >
                  19:30
                </span>
              </div>
            {/if}

            <h4
              class="font-black leading-tight tracking-tight drop-shadow-md"
              style="font-size: {Math.max(14, Math.round((localPublic.schedule_title_max_font_size ?? 96) * 0.28))}px; color: {currentSelectedPreset.textColor || '#ffffff'};"
            >
              Culto de Oração e Estudo da Palavra
            </h4>

            <p
              class="mt-2 font-medium leading-relaxed opacity-90"
              style="font-size: {Math.max(11, Math.round((localPublic.schedule_details_font_size ?? 22) * 0.5))}px; color: {currentSelectedPreset.textColor || '#ffffff'};"
            >
              Momento de oração com toda a igreja e reflexão bíblica no templo central.
            </p>
          </div>

          {#if localPublic.schedule_show_footer !== false}
            <div
              class="mt-2 pt-1 border-t border-white/20 w-full flex items-center justify-between text-white/80 font-bold uppercase tracking-wider text-[11px]"
              style="max-width: {localPublic.schedule_max_width_percent ?? 90}%;"
            >
              <span>Programação Semanal</span>
              <span>2026</span>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Editor do Fundo Selecionado -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div class="p-4 rounded-xl border border-border bg-card space-y-4">
      <div class="flex items-center justify-between gap-2">
        <h4 class="text-xs font-semibold text-foreground uppercase tracking-wider flex items-center gap-1.5">
          <Type size={14} class="text-primary" />
          <span>Cor do Texto de Projeção</span>
        </h4>
        {#if localBgStyles.overrides[currentSelectedPreset.id] && !currentSelectedPreset.isCustom}
          <button
            type="button"
            onclick={() => restorePresetDefault(currentSelectedPreset.id)}
            class="min-h-7 px-2 rounded-md text-[11px] font-medium text-amber-700 dark:text-amber-300 hover:bg-amber-500/10 flex items-center gap-1 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            title="Restaurar valores padrão deste fundo"
          >
            <RotateCcw size={12} />
            <span>Restaurar Padrão</span>
          </button>
        {/if}
      </div>

      <div>
        <label for="bg-text-color-hex" class="text-[11px] text-muted-foreground mb-1.5 block">
          Seletor de Cor Hexadecimal
        </label>
        <div class="flex items-center gap-2">
          <input
            type="color"
            aria-label="Seletor visual da cor do texto"
            value={currentSelectedPreset.textColor || '#ffffff'}
            oninput={(e) =>
              patchSelectedPreset({ textColor: (e.target as HTMLInputElement).value })}
            class="w-9 h-9 rounded-lg border border-border bg-background p-0.5 cursor-pointer shrink-0"
          />
          <input
            id="bg-text-color-hex"
            type="text"
            value={currentSelectedPreset.textColor || '#ffffff'}
            oninput={(e) =>
              patchSelectedPreset({ textColor: (e.target as HTMLInputElement).value })}
            placeholder="#FFFFFF"
            class="flex-1 h-9 px-3 rounded-lg border border-border bg-background font-mono text-xs text-foreground uppercase focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>
      </div>

      <div>
        <span class="text-[11px] text-muted-foreground mb-1.5 block">
          Atalhos de Contraste Recomendados
        </span>
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-1.5">
          {#each QUICK_TEXT_COLORS as color}
            {@const isCurrent =
              (currentSelectedPreset.textColor || '#ffffff').toLowerCase() ===
              color.hex.toLowerCase()}
            <button
              type="button"
              aria-pressed={isCurrent}
              onclick={() => patchSelectedPreset({ textColor: color.hex })}
              class="min-h-7 flex items-center gap-1.5 px-2 py-1.5 rounded-lg border text-[11px] font-medium transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isCurrent
                ? 'border-primary bg-primary/10 text-foreground font-semibold'
                : 'border-border bg-background/70 text-muted-foreground hover:text-foreground'}"
            >
              <span
                class="w-3 h-3 rounded-full border border-border shrink-0"
                style="background-color: {color.hex};"
              ></span>
              <span class="truncate">{color.label}</span>
            </button>
          {/each}
        </div>
      </div>

      <div class="pt-2 border-t border-border flex flex-wrap items-center justify-between gap-2">
        <button
          type="button"
          aria-pressed={localBgStyles.defaultBackgroundId === currentSelectedPreset.id}
          onclick={() => setDefaultBackground(currentSelectedPreset.id)}
          class="min-h-8 px-3 py-1.5 rounded-lg text-xs font-medium border flex items-center gap-1.5 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {localBgStyles.defaultBackgroundId ===
          currentSelectedPreset.id
            ? 'bg-primary text-primary-foreground border-primary'
            : 'border-border bg-secondary text-secondary-foreground hover:bg-secondary/80'}"
        >
          <Check size={13} />
          <span>
            {localBgStyles.defaultBackgroundId === currentSelectedPreset.id
              ? 'Fundo Padrão Ativo'
              : 'Definir como Padrão de Letras'}
          </span>
        </button>

        {#if currentSelectedPreset.isCustom}
          <button
            type="button"
            onclick={() => requestDeleteCustomPreset(currentSelectedPreset)}
            class="min-h-8 px-2.5 py-1.5 rounded-lg text-xs text-red-700 dark:text-red-300 hover:bg-red-500/10 border border-red-500/30 flex items-center gap-1 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          >
            <Trash2 size={13} />
            <span>Excluir</span>
          </button>
        {/if}
      </div>
    </div>

    <div class="p-4 rounded-xl border border-border bg-card space-y-4">
      <h4 class="text-xs font-semibold text-foreground uppercase tracking-wider flex items-center gap-1.5">
        <Sliders size={14} class="text-primary" />
        <span>Propriedades do Fundo CSS</span>
      </h4>

      {#if currentSelectedPreset.isCustom}
        <div>
          <label for="bg-custom-name" class="text-[11px] text-muted-foreground mb-1.5 block">
            Nome do Fundo
          </label>
          <input
            id="bg-custom-name"
            type="text"
            value={currentSelectedPreset.name}
            oninput={(e) =>
              patchSelectedPreset({ name: (e.target as HTMLInputElement).value })}
            placeholder="Nome do estilo..."
            class="w-full h-8 px-3 rounded-lg border border-border bg-background text-xs text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>
      {/if}

      <div>
        <label for="bg-anim-select" class="text-[11px] text-muted-foreground mb-1.5 block">
          Efeito de Animação CSS
        </label>
        <select
          id="bg-anim-select"
          value={currentSelectedPreset.animationClass || 'none'}
          onchange={(e) => {
            const val = (e.target as HTMLSelectElement).value;
            patchSelectedPreset({ animationClass: val === 'none' ? undefined : val });
          }}
          class="w-full h-8 px-2.5 rounded-lg border border-border bg-background text-xs text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <option value="none">Nenhuma animação (Estático)</option>
          <option value="animate-bg-pulse">Pulsar Brilho Sutil (animate-bg-pulse)</option>
          <option value="animate-bg-flow">Fluxo Gradiente Dinâmico (animate-bg-flow)</option>
          <option value="animate-bg-aurora">Aurora Boreal Cósmica (animate-bg-aurora)</option>
          <option value="animate-bg-nebula">Nebulosa Cósmica (animate-bg-nebula)</option>
        </select>
      </div>

      <div>
        <div class="flex items-center justify-between gap-2 mb-1.5">
          <label for="bg-css-style" class="text-[11px] text-muted-foreground">
            CSS da Propriedade `background`
          </label>
          <span class="text-[11px] text-muted-foreground font-mono">linear / radial / solid</span>
        </div>
        <textarea
          id="bg-css-style"
          value={currentSelectedPreset.style}
          oninput={(e) =>
            patchSelectedPreset({ style: (e.target as HTMLTextAreaElement).value })}
          rows="3"
          class="w-full p-2.5 rounded-lg border border-border bg-background font-mono text-[11px] text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring resize-y"
        ></textarea>
      </div>

      <div>
        <span class="text-[11px] text-muted-foreground mb-1.5 block">Modelos de Gradiente Rápidos</span>
        <div class="flex flex-wrap gap-1.5">
          {#each QUICK_GRADIENTS as tpl}
            <button
              type="button"
              onclick={() => patchSelectedPreset({ style: tpl.css })}
              class="min-h-7 text-[11px] px-2.5 py-1 rounded-md border border-border bg-background text-muted-foreground hover:text-foreground hover:bg-muted/50 transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              {tpl.name}
            </button>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <!-- Grade de Navegação de Fundos -->
  <div class="rounded-xl border border-border bg-card p-4 space-y-3">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
      <h4 class="text-xs font-semibold text-foreground">Escolha um Fundo para Personalizar</h4>

      <div class="flex flex-wrap items-center gap-2">
        <input
          type="text"
          bind:value={bgSearchTerm}
          aria-label="Pesquisar fundo de letras"
          placeholder="Pesquisar fundo..."
          class="h-7 px-2.5 rounded-md border border-border bg-background text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring w-36 sm:w-48"
        />
        <div class="flex items-center rounded-lg border border-border bg-muted/50 p-0.5 text-xs">
          {#each [
            { id: 'all', label: 'Todos' },
            { id: 'gradients', label: 'Gradientes' },
            { id: 'mesh', label: 'Mesh' },
            { id: 'animated', label: 'Animados' },
            { id: 'custom', label: 'Custom' },
          ] as cat}
            <button
              type="button"
              aria-pressed={bgCategoryFilter === cat.id}
              onclick={() =>
                (bgCategoryFilter = cat.id as 'all' | 'gradients' | 'mesh' | 'animated' | 'custom')}
              class="min-h-7 px-2.5 py-0.5 rounded-md text-[11px] font-medium transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {bgCategoryFilter ===
              cat.id
                ? 'bg-primary text-primary-foreground font-semibold'
                : 'text-muted-foreground hover:text-foreground'}"
            >
              {cat.label}
            </button>
          {/each}
        </div>
      </div>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2.5 max-h-60 overflow-y-auto pr-1">
      {#each filteredBgList as preset (preset.id)}
        {@const isSelected = preset.id === currentSelectedPreset.id}
        {@const isOverridden = !!localBgStyles.overrides[preset.id]}
        {@const isDefault = localBgStyles.defaultBackgroundId === preset.id}
        <button
          type="button"
          aria-pressed={isSelected}
          onclick={() => (selectedPresetId = preset.id)}
          class="group relative rounded-lg border overflow-hidden flex flex-col text-left transition-all cursor-pointer bg-card focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isSelected
            ? 'border-primary ring-2 ring-primary/80 shadow-md'
            : 'border-border hover:border-foreground/40'}"
        >
          <div class="aspect-video w-full relative overflow-hidden flex items-center justify-center bg-zinc-950">
            <div
              class="absolute inset-0 w-full h-full {preset.animationClass === 'none'
                ? ''
                : preset.animationClass || ''}"
              style="background: {preset.style};"
            ></div>
            <span
              class="relative z-10 text-xs font-bold drop-shadow-[0_1px_4px_rgba(0,0,0,0.9)]"
              style="color: {preset.textColor || '#ffffff'};"
            >
              Aa
            </span>

            {#if isDefault}
              <span class="absolute top-1 left-1 bg-primary text-primary-foreground text-[11px] font-bold px-1.5 rounded shadow">
                Padrão
              </span>
            {/if}
            {#if isOverridden}
              <span class="absolute top-1 right-1 bg-amber-400 text-zinc-950 text-[11px] font-bold px-1.5 rounded shadow">
                Editado
              </span>
            {:else if preset.isCustom}
              <span class="absolute top-1 right-1 bg-emerald-400 text-zinc-950 text-[11px] font-bold px-1.5 rounded shadow">
                Custom
              </span>
            {/if}
          </div>

          <div class="p-1.5 flex items-center justify-between w-full">
            <span class="text-[11px] font-medium text-foreground truncate">{preset.name}</span>
          </div>
        </button>
      {/each}
    </div>
  </div>
</div>

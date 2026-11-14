<script lang="ts">
  import {
    Laptop,
    Moon,
    Sun,
    Check,
    LayoutGrid,
    Rows3,
    Bell,
    BellOff,
    Sparkles,
  } from 'lucide-svelte';
  import {
    themeStore,
    setTheme,
    baseColorStore,
    setBaseColor,
    BASE_COLOR_PRESETS,
    accentThemeStore,
    setAccentTheme,
    ACCENT_THEME_PRESETS,
    previewLabelsStore,
    setPreviewLabels,
    setlistViewModeStore,
    setSetlistViewMode,
    toastDndStore,
    setToastDnd,
    toastPositionStore,
    setToastPosition,
    type BaseColor,
    type AccentTheme,
    type ToastPosition,
  } from '$lib/settings';
  import { notifyInfo } from '$lib/toast';
  import SettingsToggle from '../SettingsToggle.svelte';
  import SettingsSelectMenu from '../SettingsSelectMenu.svelte';

  interface Props {
    openSelectId: string | null;
    onSetOpenSelect: (id: string | null) => void;
  }

  let { openSelectId, onSetOpenSelect }: Props = $props();

  const currentBaseColorPreset = $derived(
    BASE_COLOR_PRESETS.find((p) => p.id === $baseColorStore) || BASE_COLOR_PRESETS[0]
  );

  const currentAccentPreset = $derived(
    ACCENT_THEME_PRESETS.find((p) => p.id === $accentThemeStore) || ACCENT_THEME_PRESETS[0]
  );

  const baseColorItems = $derived(
    BASE_COLOR_PRESETS.map((p) => ({
      id: p.id,
      label: p.label,
      subtitle: p.description,
      swatchClass:
        p.id === 'neutral'
          ? 'bg-neutral-800 dark:bg-neutral-300'
          : p.id === 'stone'
            ? 'bg-stone-700'
            : p.id === 'zinc'
              ? 'bg-zinc-700'
              : p.id === 'mauve'
                ? 'bg-purple-900'
                : p.id === 'olive'
                  ? 'bg-emerald-900'
                  : p.id === 'mist'
                    ? 'bg-slate-700'
                    : 'bg-amber-950',
    }))
  );

  const accentThemeItems = $derived(
    ACCENT_THEME_PRESETS.map((p) => ({
      id: p.id,
      label: p.label,
      swatchClass:
        p.id === 'neutral'
          ? 'bg-foreground'
          : p.id === 'amber'
            ? 'bg-amber-500'
            : p.id === 'blue'
              ? 'bg-blue-500'
              : p.id === 'cyan'
                ? 'bg-cyan-500'
                : p.id === 'emerald'
                  ? 'bg-emerald-500'
                  : p.id === 'fuchsia'
                    ? 'bg-fuchsia-500'
                    : p.id === 'green'
                      ? 'bg-green-500'
                      : p.id === 'indigo'
                        ? 'bg-indigo-500'
                        : p.id === 'lime'
                          ? 'bg-lime-500'
                          : p.id === 'orange'
                            ? 'bg-orange-500'
                            : p.id === 'pink'
                              ? 'bg-pink-500'
                              : p.id === 'purple'
                                ? 'bg-purple-500'
                                : p.id === 'rose'
                                  ? 'bg-rose-500'
                                  : p.id === 'violet'
                                    ? 'bg-violet-500'
                                    : 'bg-yellow-500',
    }))
  );
</script>

<div class="space-y-6 w-full max-w-4xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Modo de Exibição</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Alterne entre modo escuro, claro ou automático conforme o sistema operacional.
    </p>
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5" role="radiogroup" aria-label="Modo de exibição">
    {#each [
      { id: 'system' as const, label: 'Sistema', hint: 'Automático conforme o sistema', Icon: Laptop },
      { id: 'dark' as const, label: 'Escuro', hint: 'Otimizado para cabines escuras', Icon: Moon },
      { id: 'light' as const, label: 'Claro', hint: 'Ideal para ambientes iluminados', Icon: Sun },
    ] as option (option.id)}
      {@const active = $themeStore === option.id}
      {@const ModeIcon = option.Icon}
      <button
        type="button"
        role="radio"
        aria-checked={active}
        onclick={() => setTheme(option.id)}
        class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-border bg-card/60 hover:bg-muted/60'}"
      >
        <div class="flex items-center justify-between mb-3">
          <ModeIcon size={20} class={active ? 'text-primary' : 'text-muted-foreground'} />
          {#if active}
            <Check size={16} class="text-primary" />
          {/if}
        </div>
        <div>
          <div class="text-xs font-semibold text-foreground">{option.label}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">{option.hint}</div>
        </div>
      </button>
    {/each}
  </div>

  <!-- Base Color e Cor de Destaque -->
  <div class="pt-4 border-t border-border space-y-4">
    <div>
      <h3 class="text-sm font-semibold text-foreground">Cores e Temas</h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Personalize os tons neutros de superfície e a cor de destaque primária da interface.
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <SettingsSelectMenu
        id="appearance-base-color"
        label="Família de Superfície (Base)"
        selectedId={$baseColorStore}
        selectedLabel={`${currentBaseColorPreset.label} — ${currentBaseColorPreset.description}`}
        selectedSwatchClass={baseColorItems.find((i) => i.id === $baseColorStore)?.swatchClass || ''}
        items={baseColorItems}
        open={openSelectId === 'appearance-base-color'}
        onToggleOpen={() => onSetOpenSelect(openSelectId === 'appearance-base-color' ? null : 'appearance-base-color')}
        onClose={() => onSetOpenSelect(null)}
        onSelect={(id) => setBaseColor(id as BaseColor)}
      />

      <SettingsSelectMenu
        id="appearance-accent-theme"
        label="Cor de Destaque Primária"
        selectedId={$accentThemeStore}
        selectedLabel={currentAccentPreset.label}
        selectedSwatchClass={accentThemeItems.find((i) => i.id === $accentThemeStore)?.swatchClass || ''}
        items={accentThemeItems}
        open={openSelectId === 'appearance-accent-theme'}
        onToggleOpen={() => onSetOpenSelect(openSelectId === 'appearance-accent-theme' ? null : 'appearance-accent-theme')}
        onClose={() => onSetOpenSelect(null)}
        onSelect={(id) => setAccentTheme(id as AccentTheme)}
      />
    </div>
  </div>

  <!-- Controles da Interface do Operador -->
  <div class="pt-4 border-t border-border space-y-4">
    <div>
      <h3 class="text-sm font-semibold text-foreground">Botões da Interface do Operador</h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Personalize a exibição dos rótulos na barra de navegação de slides, controles de saídas e cabeçalho.
      </p>
    </div>

    <SettingsToggle
      checked={$previewLabelsStore}
      label="Exibir textos (labels) nos botões"
      description="Quando desativado, exibe apenas os ícones compactos para ganhar espaço de tela e manter o visual limpo."
      ariaLabel="Alternar exibição de textos nos botões"
      ontoggle={() => setPreviewLabels(!$previewLabelsStore)}
    />
  </div>

  <!-- Visualização da Programação (Setlist) -->
  <div class="pt-4 border-t border-border space-y-4">
    <div>
      <h3 class="text-sm font-semibold text-foreground">Visualização da Programação (Setlist)</h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Escolha a forma padrão como a programação e os slides são apresentados no painel central do operador.
      </p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5" role="radiogroup" aria-label="Visualização da programação">
      <button
        type="button"
        role="radio"
        aria-checked={$setlistViewModeStore === 'item'}
        onclick={() => setSetlistViewMode('item')}
        class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {$setlistViewModeStore === 'item' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-border bg-card/60 hover:bg-muted/60'}"
      >
        <div class="flex items-center justify-between mb-3">
          <LayoutGrid size={20} class={$setlistViewModeStore === 'item' ? 'text-primary' : 'text-muted-foreground'} />
          {#if $setlistViewModeStore === 'item'}
            <Check size={16} class="text-primary" />
          {/if}
        </div>
        <div>
          <div class="text-xs font-semibold text-foreground">Item Focado (Tradicional)</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">
            Exibe na tela central apenas a grade de slides da mídia selecionada na setlist.
          </div>
        </div>
      </button>

      <button
        type="button"
        role="radio"
        aria-checked={$setlistViewModeStore === 'flow'}
        onclick={() => setSetlistViewMode('flow')}
        class="p-4 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {$setlistViewModeStore === 'flow' ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-border bg-card/60 hover:bg-muted/60'}"
      >
        <div class="flex items-center justify-between mb-3">
          <Rows3 size={20} class={$setlistViewModeStore === 'flow' ? 'text-primary' : 'text-muted-foreground'} />
          {#if $setlistViewModeStore === 'flow'}
            <Check size={16} class="text-primary" />
          {/if}
        </div>
        <div>
          <div class="text-xs font-semibold text-foreground">Fluxo Contínuo (Sequencial)</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">
            Exibe todas as seções e slides em sequência vertical contínua, com colapso e apresentação por seção.
          </div>
        </div>
      </button>
    </div>
  </div>

  <!-- Notificações (Toasts) -->
  <div class="pt-4 border-t border-border space-y-4">
    <div>
      <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
        <Bell size={16} class="text-primary" />
        Notificações do Sistema (Toasts)
      </h3>
      <p class="text-xs text-muted-foreground mt-0.5">
        Configure os avisos flutuantes de conexão de aparelhos, início de apresentação e operações do sistema.
      </p>
    </div>

    <SettingsToggle
      checked={$toastDndStore}
      tone="amber"
      label={$toastDndStore ? 'Modo Não Perturbe (Silenciado)' : 'Modo Não Perturbe'}
      description="Oculta todos os toasts de notificação na tela do operador para evitar distrações durante o culto ao vivo."
      ariaLabel="Alternar modo Não Perturbe"
      ontoggle={() => setToastDnd(!$toastDndStore)}
    >
      {#snippet icon()}
        {#if $toastDndStore}
          <BellOff size={14} class="text-amber-600 dark:text-amber-400" />
        {:else}
          <Bell size={14} class="text-muted-foreground" />
        {/if}
      {/snippet}
    </SettingsToggle>

    <div class="space-y-2">
      <div class="flex items-center justify-between gap-2">
        <span class="block text-[11px] font-medium text-muted-foreground">
          Posição das Notificações na Tela
        </span>
        <button
          type="button"
          onclick={() => {
            if ($toastDndStore) {
              notifyInfo('Não Perturbe ativado', 'Desative o Não Perturbe para visualizar notificações.');
            } else {
              notifyInfo(
                'Notificação de Teste',
                `Exibida no canto: ${
                  $toastPositionStore === 'top-left'
                    ? 'Superior Esquerdo'
                    : $toastPositionStore === 'top-right'
                      ? 'Superior Direito'
                      : $toastPositionStore === 'bottom-right'
                        ? 'Inferior Direito'
                        : 'Inferior Esquerdo'
                }`
              );
            }
          }}
          class="min-h-7 px-2 py-1 rounded text-[11px] font-medium text-primary hover:underline cursor-pointer flex items-center gap-1 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          <Sparkles size={12} />
          Testar Notificação
        </button>
      </div>

      <div class="grid grid-cols-2 gap-2.5" role="radiogroup" aria-label="Posição das notificações">
        {#each [
          { id: 'top-left', label: 'Superior Esquerdo', hint: 'Canto alto à esquerda' },
          { id: 'top-right', label: 'Superior Direito', hint: 'Canto alto à direita (Padrão)' },
          { id: 'bottom-left', label: 'Inferior Esquerdo', hint: 'Canto baixo à esquerda' },
          { id: 'bottom-right', label: 'Inferior Direito', hint: 'Canto baixo à direita' },
        ] as pos (pos.id)}
          {@const active = $toastPositionStore === pos.id}
          <button
            type="button"
            role="radio"
            aria-checked={active}
            onclick={() => setToastPosition(pos.id as ToastPosition)}
            class="p-3 rounded-xl border text-left flex flex-col justify-between transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active ? 'border-primary bg-primary/10 ring-1 ring-primary/40' : 'border-border bg-card/60 hover:bg-muted/60'}"
          >
            <div class="flex items-center justify-between mb-1.5">
              <span class="text-xs font-semibold text-foreground">{pos.label}</span>
              {#if active}
                <Check size={14} class="text-primary" />
              {/if}
            </div>
            <div class="text-[11px] text-muted-foreground">
              {pos.hint}
            </div>
          </button>
        {/each}
      </div>
    </div>
  </div>
</div>

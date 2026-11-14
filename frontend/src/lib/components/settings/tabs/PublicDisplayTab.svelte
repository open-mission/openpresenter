<script lang="ts">
  import {
    Music,
    BookOpen,
    CalendarDays,
    Clock,
    Sliders,
    Type,
    Maximize2,
    Info,
    Layers,
    RotateCcw,
    Sun,
    Image as ImageIcon,
    Sparkles,
  } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import SettingsToggle from '../SettingsToggle.svelte';
  import SettingsRangeField from '../SettingsRangeField.svelte';
  import { DEFAULT_PUBLIC_SETTINGS } from '$lib/displaySettings';
  import type { ImageFitMode, PublicDisplaySettings } from '$lib/types';
  import type { PublicSectionId } from '../types';

  interface Props {
    localPublic: PublicDisplaySettings;
    onUpdatePublic: (next: PublicDisplaySettings) => void;
    onSelectWallpaperFit: (fit: ImageFitMode | 'repeat') => void;
  }

  let { localPublic, onUpdatePublic, onSelectWallpaperFit }: Props = $props();

  let publicSection = $state<PublicSectionId>('lyrics');

  function patchPublic(patch: Partial<PublicDisplaySettings>) {
    onUpdatePublic({ ...localPublic, ...patch });
  }

  const sectionTabs: Array<{ id: PublicSectionId; label: string; icon: typeof Music }> = [
    { id: 'lyrics', label: 'Letras', icon: Music },
    { id: 'scripture', label: 'Bíblia', icon: BookOpen },
    { id: 'schedule', label: 'Agenda', icon: CalendarDays },
    { id: 'timer_audio', label: 'Timer & Áudio', icon: Clock },
    { id: 'general', label: 'Margens', icon: Sliders },
  ];

  const fitWallOptions: Array<{ id: ImageFitMode | 'repeat'; label: string; desc: string }> = [
    { id: 'contain', label: 'Ajustar (Conter)', desc: '100% visível, sem cortes (TV)' },
    { id: 'cover', label: 'Preencher (Cobrir)', desc: 'Preenche cortando bordas' },
    { id: 'fill', label: 'Esticar', desc: 'Preenche 100% da tela' },
    { id: 'center', label: 'Centralizar', desc: 'Tamanho original sem zoom' },
    { id: 'repeat', label: 'Repetir (Mosaico)', desc: 'Repete padrão pela tela' },
  ];
</script>

<div class="space-y-6 max-w-4xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Configuração da Projeção Pública</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Personalize de forma independente o comportamento de letras de louvor, textos bíblicos, agenda e margens seguras.
    </p>
  </div>

  <!-- Sub-navegação da Projeção Pública -->
  <div
    class="p-1 bg-muted/50 rounded-xl border border-border grid grid-cols-2 sm:grid-cols-5 gap-1"
    role="tablist"
    aria-label="Subseções da Tela Pública"
  >
    {#each sectionTabs as sec}
      {@const Icon = sec.icon}
      {@const active = publicSection === sec.id}
      <button
        type="button"
        role="tab"
        aria-selected={active}
        onclick={() => (publicSection = sec.id)}
        class="min-h-8 flex items-center justify-center gap-1.5 py-2 px-2.5 rounded-lg text-xs font-semibold transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
          ? 'bg-primary text-primary-foreground shadow-xs'
          : 'text-muted-foreground hover:text-foreground hover:bg-background/60'}"
      >
        <Icon size={13} />
        <span>{sec.label}</span>
      </button>
    {/each}
  </div>

  {#if publicSection === 'lyrics'}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Sliders size={14} class="text-primary" />
          Alinhamento Padrão das Letras
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <div class="space-y-1.5">
            <span class="text-[11px] text-muted-foreground block">Horizontal</span>
            <div class="grid grid-cols-4 gap-1" role="radiogroup" aria-label="Alinhamento horizontal das letras">
              {#each [
                { id: 'left', label: 'Esq' },
                { id: 'center', label: 'Centro' },
                { id: 'right', label: 'Dir' },
                { id: 'justify', label: 'Just' },
              ] as opt}
                {@const active = (localPublic.lyrics_align_h || 'center') === opt.id}
                <button
                  type="button"
                  role="radio"
                  aria-checked={active}
                  onclick={() =>
                    patchPublic({ lyrics_align_h: opt.id as PublicDisplaySettings['lyrics_align_h'] })}
                  class="min-h-7 py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                    ? 'bg-primary text-primary-foreground font-bold border-primary'
                    : 'bg-background border-border text-muted-foreground hover:text-foreground'}"
                >
                  {opt.label}
                </button>
              {/each}
            </div>
          </div>

          <div class="space-y-1.5">
            <span class="text-[11px] text-muted-foreground block">Vertical</span>
            <div class="grid grid-cols-3 gap-1" role="radiogroup" aria-label="Alinhamento vertical das letras">
              {#each [
                { id: 'top', label: 'Topo' },
                { id: 'center', label: 'Centro' },
                { id: 'bottom', label: 'Base' },
              ] as opt}
                {@const active = (localPublic.lyrics_align_v || 'center') === opt.id}
                <button
                  type="button"
                  role="radio"
                  aria-checked={active}
                  onclick={() =>
                    patchPublic({ lyrics_align_v: opt.id as PublicDisplaySettings['lyrics_align_v'] })}
                  class="min-h-7 py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                    ? 'bg-primary text-primary-foreground font-bold border-primary'
                    : 'bg-background border-border text-muted-foreground hover:text-foreground'}"
                >
                  {opt.label}
                </button>
              {/each}
            </div>
          </div>
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Type size={14} class="text-primary" />
            Auto-scale das Letras na Projeção
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            O tamanho da fonte se adapta automaticamente ao volume do texto de cada estrofe.
          </p>
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <SettingsRangeField
            id="pub-lyrics-max-font"
            label="Tamanho Máximo (Alvo)"
            value={(localPublic.lyrics_max_font_size || localPublic.max_font_size) ?? 128}
            min={64}
            max={192}
            step={8}
            unit="px"
            onchange={(val) => patchPublic({ lyrics_max_font_size: val, max_font_size: val })}
          />

          <SettingsRangeField
            id="pub-lyrics-min-font"
            label="Tamanho Mínimo"
            value={(localPublic.lyrics_min_font_size || localPublic.min_font_size) ?? 24}
            min={16}
            max={64}
            step={4}
            unit="px"
            onchange={(val) => patchPublic({ lyrics_min_font_size: val, min_font_size: val })}
          />
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Maximize2 size={14} class="text-primary" />
            Preencher o Máximo de Espaço nas Letras
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Amplia as letras para preencher a tela, garantindo alta legibilidade no fundo do templo.
          </p>
        </div>
        <SettingsToggle
          checked={Boolean(localPublic.fill_space)}
          ariaLabel="Alternar preenchimento máximo de espaço nas letras"
          onToggle={() => patchPublic({ fill_space: !localPublic.fill_space })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Type size={14} class="text-primary" />
            Permitir Quebra de Linhas Longas (Word Wrap)
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Se desativado, o texto preserva os versos originais em linha única e o auto-scale reduz a fonte para caber sem reticências (...).
          </p>
        </div>
        <SettingsToggle
          checked={Boolean(localPublic.word_wrap)}
          ariaLabel="Alternar quebra de linhas longas"
          onToggle={() => patchPublic({ word_wrap: !localPublic.word_wrap })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Info size={14} class="text-primary" />
            Exibir Metadados do Louvor
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Exibe o título da música e artista/compositor em uma legenda discreta no rodapé da projeção pública.
          </p>
        </div>
        <SettingsToggle
          checked={Boolean(localPublic.show_metadata)}
          ariaLabel="Alternar exibição de metadados do louvor"
          onToggle={() => patchPublic({ show_metadata: !localPublic.show_metadata })}
        />
      </div>
    </div>
  {:else if publicSection === 'scripture'}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div class="text-xs font-semibold text-foreground flex items-center gap-2">
          <BookOpen size={14} class="text-primary" />
          Alinhamento das Passagens Bíblicas
        </div>
        <p class="text-[11px] text-muted-foreground">
          Configure o posicionamento do texto dos versículos na tela pública. O padrão recomendado é Centralizado.
        </p>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <div class="space-y-1.5">
            <span class="text-[11px] text-muted-foreground block">Horizontal</span>
            <div class="grid grid-cols-4 gap-1" role="radiogroup" aria-label="Alinhamento horizontal da Bíblia">
              {#each [
                { id: 'left', label: 'Esq' },
                { id: 'center', label: 'Centro' },
                { id: 'right', label: 'Dir' },
                { id: 'justify', label: 'Just' },
              ] as opt}
                {@const active = (localPublic.scripture_align_h || 'center') === opt.id}
                <button
                  type="button"
                  role="radio"
                  aria-checked={active}
                  onclick={() =>
                    patchPublic({
                      scripture_align_h: opt.id as PublicDisplaySettings['scripture_align_h'],
                    })}
                  class="min-h-7 py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                    ? 'bg-primary text-primary-foreground font-bold border-primary'
                    : 'bg-background border-border text-muted-foreground hover:text-foreground'}"
                >
                  {opt.label}
                </button>
              {/each}
            </div>
          </div>

          <div class="space-y-1.5">
            <span class="text-[11px] text-muted-foreground block">Vertical</span>
            <div class="grid grid-cols-3 gap-1" role="radiogroup" aria-label="Alinhamento vertical da Bíblia">
              {#each [
                { id: 'top', label: 'Topo' },
                { id: 'center', label: 'Centro' },
                { id: 'bottom', label: 'Base' },
              ] as opt}
                {@const active = (localPublic.scripture_align_v || 'center') === opt.id}
                <button
                  type="button"
                  role="radio"
                  aria-checked={active}
                  onclick={() =>
                    patchPublic({
                      scripture_align_v: opt.id as PublicDisplaySettings['scripture_align_v'],
                    })}
                  class="min-h-7 py-1 px-1.5 rounded-lg border text-xs font-medium cursor-pointer transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                    ? 'bg-primary text-primary-foreground font-bold border-primary'
                    : 'bg-background border-border text-muted-foreground hover:text-foreground'}"
                >
                  {opt.label}
                </button>
              {/each}
            </div>
          </div>
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Type size={14} class="text-primary" />
            Auto-scale Especializado para Versículos
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Defina teto máximo e piso mínimo próprios para blocos bíblicos na tela.
          </p>
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <SettingsRangeField
            id="pub-scripture-max-font"
            label="Tamanho Máximo Bíblico"
            value={localPublic.scripture_max_font_size ?? 140}
            min={80}
            max={200}
            step={8}
            unit="px"
            minLabel="80px"
            midLabel="140px (Padrão)"
            maxLabel="200px"
            onchange={(v) => patchPublic({ scripture_max_font_size: v })}
          />

          <SettingsRangeField
            id="pub-scripture-min-font"
            label="Tamanho Mínimo Bíblico"
            value={localPublic.scripture_min_font_size ?? 28}
            min={18}
            max={64}
            step={2}
            unit="px"
            minLabel="18px"
            midLabel="28px (Padrão)"
            maxLabel="64px"
            onchange={(v) => patchPublic({ scripture_min_font_size: v })}
          />
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Maximize2 size={14} class="text-primary" />
            Aproveitar 100% da Largura da Tela na Bíblia
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Permite que os versículos usem toda a largura útil da projeção, acomodando mais palavras por linha.
          </p>
        </div>
        <SettingsToggle
          checked={localPublic.scripture_fill_space !== false}
          ariaLabel="Alternar preenchimento de largura da Bíblia"
          onToggle={() =>
            patchPublic({ scripture_fill_space: localPublic.scripture_fill_space === false })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div class="flex items-center justify-between gap-4">
          <div>
            <div class="text-xs font-semibold text-foreground flex items-center gap-2">
              <BookOpen size={14} class="text-primary" />
              Exibir Referência Bíblica e Versão (Ex: João 3:16 • ARA)
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Mostra o livro, capítulo, versículo e a sigla da tradução utilizada na projeção pública.
            </p>
          </div>
          <SettingsToggle
            checked={localPublic.scripture_show_reference !== false}
            ariaLabel="Alternar referência bíblica na tela"
            onToggle={() =>
              patchPublic({
                scripture_show_reference: localPublic.scripture_show_reference === false,
              })}
          />
        </div>

        {#if localPublic.scripture_show_reference !== false}
          <div class="pt-3 border-t border-border space-y-2.5">
            <span class="text-xs font-medium text-foreground block">Posição da Referência na Tela</span>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2" role="radiogroup" aria-label="Posição da referência bíblica">
              <button
                type="button"
                role="radio"
                aria-checked={(localPublic.scripture_reference_position ?? 'footer') === 'footer'}
                onclick={() => patchPublic({ scripture_reference_position: 'footer' })}
                class="p-2.5 rounded-lg border text-left transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {(localPublic.scripture_reference_position ??
                  'footer') === 'footer'
                  ? 'border-primary bg-primary/10 text-foreground'
                  : 'border-border bg-background text-muted-foreground hover:text-foreground'}"
              >
                <div class="text-xs font-semibold">
                  No Rodapé (Inferior da Tela)
                </div>
                <div class="text-[11px] opacity-80 mt-0.5">
                  Fixado na parte inferior com linha divisória discreta
                </div>
              </button>

              <button
                type="button"
                role="radio"
                aria-checked={localPublic.scripture_reference_position === 'below_text'}
                onclick={() => patchPublic({ scripture_reference_position: 'below_text' })}
                class="p-2.5 rounded-lg border text-left transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {localPublic.scripture_reference_position ===
                'below_text'
                  ? 'border-primary bg-primary/10 text-foreground'
                  : 'border-border bg-background text-muted-foreground hover:text-foreground'}"
              >
                <div class="text-xs font-semibold">
                  Abaixo do Texto (Junto ao Versículo)
                </div>
                <div class="text-[11px] opacity-80 mt-0.5">
                  Acompanha o versículo centralizado verticalmente
                </div>
              </button>
            </div>
          </div>

          {#if (localPublic.scripture_reference_position ?? 'footer') === 'footer'}
            <div class="max-w-md">
              <SettingsRangeField
                id="pub-scripture-ref-font"
                label="Tamanho da Fonte da Referência no Rodapé"
                value={localPublic.scripture_reference_font_size ?? 28}
                min={16}
                max={64}
                step={2}
                unit="px"
                minLabel="16px (Discreto)"
                midLabel="28px (Padrão)"
                maxLabel="64px (Grande)"
                onchange={(v) => patchPublic({ scripture_reference_font_size: v })}
              />
            </div>
          {/if}
        {/if}
      </div>
    </div>
  {:else if publicSection === 'schedule'}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
              <Layers size={14} class="text-primary" />
              Container e Fundo do Card da Agenda
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Personalize a caixa visual onde as informações do evento são exibidas no telão.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7"
            onclick={() =>
              patchPublic({
                schedule_box_background: DEFAULT_PUBLIC_SETTINGS.schedule_box_background,
                schedule_bg_opacity: DEFAULT_PUBLIC_SETTINGS.schedule_bg_opacity,
                schedule_max_width_percent: DEFAULT_PUBLIC_SETTINGS.schedule_max_width_percent,
                schedule_padding: DEFAULT_PUBLIC_SETTINGS.schedule_padding,
                schedule_align_h: DEFAULT_PUBLIC_SETTINGS.schedule_align_h,
                schedule_title_min_font_size: DEFAULT_PUBLIC_SETTINGS.schedule_title_min_font_size,
                schedule_title_max_font_size: DEFAULT_PUBLIC_SETTINGS.schedule_title_max_font_size,
                schedule_day_font_size: DEFAULT_PUBLIC_SETTINGS.schedule_day_font_size,
                schedule_time_font_size: DEFAULT_PUBLIC_SETTINGS.schedule_time_font_size,
                schedule_details_font_size: DEFAULT_PUBLIC_SETTINGS.schedule_details_font_size,
                schedule_show_badges: DEFAULT_PUBLIC_SETTINGS.schedule_show_badges,
                schedule_show_footer: DEFAULT_PUBLIC_SETTINGS.schedule_show_footer,
              })}
          >
            Restaurar Padrão
          </Button>
        </div>

        <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
          <div>
            <div class="text-xs font-semibold text-foreground">Exibir Fundo no Card de Agenda</div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Adiciona uma placa translúcida com borda e efeito de vidro atrás das informações.
            </p>
          </div>
          <SettingsToggle
            checked={localPublic.schedule_box_background !== false}
            ariaLabel="Alternar fundo no card da agenda"
            onToggle={() =>
              patchPublic({ schedule_box_background: localPublic.schedule_box_background === false })}
          />
        </div>

        <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
          <div>
            <div class="text-xs font-semibold text-foreground">Exibir Fundo das Pílulas (Dia e Horário)</div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Quando desativado, exibe apenas os textos limpos sem pílulas arredondadas atrás.
            </p>
          </div>
          <SettingsToggle
            checked={localPublic.schedule_badge_bg !== false}
            ariaLabel="Alternar fundo das pílulas de dia e horário"
            onToggle={() =>
              patchPublic({ schedule_badge_bg: localPublic.schedule_badge_bg === false })}
          />
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="pub-sched-day-color" class="text-xs font-semibold text-foreground block">
                Cor do Dia da Semana
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localPublic.schedule_day_color || '#fcd34d'}
              </div>
            </div>
            <input
              id="pub-sched-day-color"
              type="color"
              value={localPublic.schedule_day_color || '#fcd34d'}
              oninput={(e) =>
                patchPublic({ schedule_day_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="pub-sched-time-color" class="text-xs font-semibold text-foreground block">
                Cor do Horário
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localPublic.schedule_time_color || '#ffffff'}
              </div>
            </div>
            <input
              id="pub-sched-time-color"
              type="color"
              value={localPublic.schedule_time_color || '#ffffff'}
              oninput={(e) =>
                patchPublic({ schedule_time_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>
        </div>

        {#if localPublic.schedule_box_background !== false}
          <SettingsRangeField
            id="pub-sched-bg-opacity"
            label="Opacidade do Fundo"
            value={localPublic.schedule_bg_opacity ?? 80}
            min={10}
            max={100}
            step={5}
            unit="%"
            minLabel="10% (Transparente)"
            midLabel="80% (Padrão)"
            maxLabel="100% (Sólido)"
            onchange={(v) => patchPublic({ schedule_bg_opacity: v })}
          />
        {/if}

        <SettingsRangeField
          id="pub-sched-max-width"
          label="Largura Máxima do Container"
          value={localPublic.schedule_max_width_percent ?? 90}
          min={50}
          max={100}
          step={5}
          unit="%"
          minLabel="50% (Estreito)"
          midLabel="90% (Padrão)"
          maxLabel="100% (Largura total)"
          onchange={(v) => patchPublic({ schedule_max_width_percent: v })}
        />

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div class="p-3 rounded-lg border border-border bg-background/70 space-y-2">
            <span class="text-xs font-semibold text-foreground block">Espaçamento Interno (Padding)</span>
            <div class="grid grid-cols-3 gap-1" role="radiogroup" aria-label="Padding do card da agenda">
              {#each [
                { id: 'compact', label: 'Compacto' },
                { id: 'normal', label: 'Normal' },
                { id: 'spacious', label: 'Amplo' },
              ] as padOpt}
                {@const active = (localPublic.schedule_padding ?? 'normal') === padOpt.id}
                <button
                  type="button"
                  role="radio"
                  aria-checked={active}
                  onclick={() =>
                    patchPublic({
                      schedule_padding: padOpt.id as PublicDisplaySettings['schedule_padding'],
                    })}
                  class="min-h-7 py-1.5 px-2 rounded-md text-[11px] font-medium transition-colors cursor-pointer text-center focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                    ? 'bg-primary text-primary-foreground font-semibold shadow-xs'
                    : 'bg-card border border-border text-muted-foreground hover:text-foreground'}"
                >
                  {padOpt.label}
                </button>
              {/each}
            </div>
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 space-y-2">
            <span class="text-xs font-semibold text-foreground block">Alinhamento Horizontal</span>
            <div class="grid grid-cols-3 gap-1" role="radiogroup" aria-label="Alinhamento horizontal da agenda">
              {#each [
                { id: 'left', label: 'Esquerda' },
                { id: 'center', label: 'Centro' },
                { id: 'right', label: 'Direita' },
              ] as alignOpt}
                {@const active = (localPublic.schedule_align_h ?? 'center') === alignOpt.id}
                <button
                  type="button"
                  role="radio"
                  aria-checked={active}
                  onclick={() =>
                    patchPublic({
                      schedule_align_h: alignOpt.id as PublicDisplaySettings['schedule_align_h'],
                    })}
                  class="min-h-7 py-1.5 px-2 rounded-md text-[11px] font-medium transition-colors cursor-pointer text-center focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                    ? 'bg-primary text-primary-foreground font-semibold shadow-xs'
                    : 'bg-card border border-border text-muted-foreground hover:text-foreground'}"
                >
                  {alignOpt.label}
                </button>
              {/each}
            </div>
          </div>
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Type size={14} class="text-primary" />
          Tipografia e Tamanho dos Textos
        </h4>
        <p class="text-[11px] text-muted-foreground">
          Ajuste fino dos tamanhos de fontes do título, data/hora e detalhes do evento na projeção pública.
        </p>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <SettingsRangeField
            id="pub-sched-title-min"
            label="Título (Mínimo Auto-Scale)"
            value={localPublic.schedule_title_min_font_size ?? 32}
            min={18}
            max={Math.max(64, localPublic.schedule_title_min_font_size ?? 32)}
            step={2}
            unit="px"
            allowNumberInput
            numberMin={10}
            numberMax={150}
            numberStep={2}
            onchange={(v) => patchPublic({ schedule_title_min_font_size: v })}
          />

          <SettingsRangeField
            id="pub-sched-title-max"
            label="Título (Máximo Auto-Scale)"
            value={localPublic.schedule_title_max_font_size ?? 110}
            min={48}
            max={Math.max(160, localPublic.schedule_title_max_font_size ?? 110)}
            step={4}
            unit="px"
            allowNumberInput
            numberMin={30}
            numberMax={300}
            numberStep={4}
            onchange={(v) => patchPublic({ schedule_title_max_font_size: v })}
          />

          <SettingsRangeField
            id="pub-sched-day-font"
            label="Texto do Dia da Semana"
            value={localPublic.schedule_day_font_size ?? 20}
            min={12}
            max={Math.max(80, localPublic.schedule_day_font_size ?? 20)}
            step={1}
            unit="px"
            allowNumberInput
            numberMin={8}
            numberMax={200}
            numberStep={1}
            onchange={(v) => patchPublic({ schedule_day_font_size: v })}
          />

          <SettingsRangeField
            id="pub-sched-time-font"
            label="Texto do Horário"
            value={localPublic.schedule_time_font_size ?? 20}
            min={12}
            max={Math.max(80, localPublic.schedule_time_font_size ?? 20)}
            step={1}
            unit="px"
            allowNumberInput
            numberMin={8}
            numberMax={200}
            numberStep={1}
            onchange={(v) => patchPublic({ schedule_time_font_size: v })}
          />
        </div>

        <SettingsRangeField
          id="pub-sched-details-font"
          label="Texto de Detalhes / Descrição"
          value={localPublic.schedule_details_font_size ?? 20}
          min={12}
          max={Math.max(80, localPublic.schedule_details_font_size ?? 20)}
          step={1}
          unit="px"
          allowNumberInput
          numberMin={8}
          numberMax={200}
          numberStep={1}
          onchange={(v) => patchPublic({ schedule_details_font_size: v })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Sliders size={14} class="text-primary" />
          Visibilidade de Elementos
        </h4>

        <div class="space-y-2">
          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
            <div>
              <div class="text-xs font-semibold text-foreground">
                Exibir Badges de Dia da Semana e Horário
              </div>
              <p class="text-[11px] text-muted-foreground mt-0.5">
                Exibe os distintivos coloridos com dia (ex: QUARTA-FEIRA) e horário (ex: 19:30).
              </p>
            </div>
            <SettingsToggle
              checked={localPublic.schedule_show_badges !== false}
              ariaLabel="Alternar badges de dia e horário da agenda"
              onToggle={() =>
                patchPublic({ schedule_show_badges: localPublic.schedule_show_badges === false })}
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
            <div>
              <div class="text-xs font-semibold text-foreground">
                Exibir Rodapé com Nome da Programação
              </div>
              <p class="text-[11px] text-muted-foreground mt-0.5">
                Exibe a linha divisória e o título geral da agenda no rodapé da projeção pública.
              </p>
            </div>
            <SettingsToggle
              checked={localPublic.schedule_show_footer !== false}
              ariaLabel="Alternar rodapé com nome da programação"
              onToggle={() =>
                patchPublic({ schedule_show_footer: localPublic.schedule_show_footer === false })}
            />
          </div>
        </div>
      </div>
    </div>
  {:else if publicSection === 'timer_audio'}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
              <Clock size={14} class="text-primary" />
              Aparência do Cronômetro / Timer na Tela Pública
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Personalize a caixa visual, opacidade de fundo, cores e tamanho dos textos ao projetar cronômetros e timers no telão.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7 gap-1"
            onclick={() =>
              patchPublic({
                timer_box_background: DEFAULT_PUBLIC_SETTINGS.timer_box_background,
                timer_bg_opacity: DEFAULT_PUBLIC_SETTINGS.timer_bg_opacity,
                timer_digits_font_size: DEFAULT_PUBLIC_SETTINGS.timer_digits_font_size,
                timer_title_font_size: DEFAULT_PUBLIC_SETTINGS.timer_title_font_size,
                timer_message_font_size: DEFAULT_PUBLIC_SETTINGS.timer_message_font_size,
                timer_text_color: DEFAULT_PUBLIC_SETTINGS.timer_text_color,
              })}
          >
            <RotateCcw size={12} />
            <span>Padrão</span>
          </Button>
        </div>

        <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
          <div>
            <div class="text-xs font-semibold text-foreground">
              Caixa de Fundo (Placa com Efeito Vidro)
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Adiciona container translúcido com desfoque atrás dos dígitos e mensagens. Se desativado, o texto fica diretamente sobre o fundo.
            </p>
          </div>
          <SettingsToggle
            checked={localPublic.timer_box_background !== false}
            ariaLabel="Alternar caixa de fundo do timer"
            onToggle={() =>
              patchPublic({ timer_box_background: localPublic.timer_box_background === false })}
          />
        </div>

        {#if localPublic.timer_box_background !== false}
          <SettingsRangeField
            id="pub-timer-bg-opacity"
            label="Opacidade do Fundo"
            value={localPublic.timer_bg_opacity ?? 65}
            min={10}
            max={100}
            step={5}
            unit="%"
            minLabel="10% (Sutil)"
            midLabel="65% (Padrão)"
            maxLabel="100% (Sólido)"
            onchange={(v) => patchPublic({ timer_bg_opacity: v })}
          />
        {/if}

        <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
          <div>
            <label for="pub-timer-text-color" class="text-xs font-semibold text-foreground block">
              Cor do Texto e Dígitos
            </label>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Cor dos números do cronômetro, título e mensagem na tela de projeção.
            </p>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <span class="font-mono text-[11px] text-muted-foreground uppercase">
              {localPublic.timer_text_color || '#ffffff'}
            </span>
            <input
              id="pub-timer-text-color"
              type="color"
              value={localPublic.timer_text_color || '#ffffff'}
              oninput={(e) =>
                patchPublic({ timer_text_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-1">
          <SettingsRangeField
            id="pub-timer-digits-size"
            label="Dígitos do Timer"
            value={localPublic.timer_digits_font_size ?? 130}
            min={50}
            max={220}
            step={5}
            unit="px"
            allowNumberInput
            numberMin={50}
            numberMax={240}
            numberStep={5}
            onchange={(v) => patchPublic({ timer_digits_font_size: v })}
          />

          <SettingsRangeField
            id="pub-timer-title-size"
            label="Título do Momento"
            value={localPublic.timer_title_font_size ?? 22}
            min={12}
            max={60}
            step={2}
            unit="px"
            allowNumberInput
            numberMin={12}
            numberMax={80}
            numberStep={2}
            onchange={(v) => patchPublic({ timer_title_font_size: v })}
          />

          <SettingsRangeField
            id="pub-timer-msg-size"
            label="Mensagem Abaixo"
            value={localPublic.timer_message_font_size ?? 18}
            min={10}
            max={48}
            step={2}
            unit="px"
            allowNumberInput
            numberMin={10}
            numberMax={60}
            numberStep={2}
            onchange={(v) => patchPublic({ timer_message_font_size: v })}
          />
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Music size={14} class="text-primary" />
          Reprodução de Áudio na Projeção Pública
        </h4>

        <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
          <div>
            <div class="text-xs font-semibold text-foreground">
              Exibir Arte com Waveforms Sonoras Animadas
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Por padrão desativado: mantém o papel de parede limpo. Ative para exibir um card com equalizador animado e metadados da faixa.
            </p>
          </div>
          <SettingsToggle
            checked={Boolean(localPublic.audio_show_waveform_on_public)}
            ariaLabel="Alternar waveforms de áudio na tela pública"
            onToggle={() =>
              patchPublic({
                audio_show_waveform_on_public: !localPublic.audio_show_waveform_on_public,
              })}
          />
        </div>
      </div>
    </div>
  {:else}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Sun size={14} class="text-primary" />
            Escurecer Plano de Fundo Geral (Backdrop Completo)
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Aplica uma película escura translúcida sobre o papel de parede quando houver slide ativo para melhorar o contraste.
          </p>
        </div>
        <SettingsToggle
          checked={Boolean(localPublic.darken_background)}
          ariaLabel="Alternar escurecimento do plano de fundo"
          onToggle={() => patchPublic({ darken_background: !localPublic.darken_background })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Layers size={14} class="text-primary" />
            Fundo nas Caixas de Texto (Card de Leitura)
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Adiciona uma placa translúcida com cantos arredondados atrás dos textos e versículos bíblicos.
          </p>
        </div>
        <SettingsToggle
          checked={Boolean(localPublic.text_box_background)}
          ariaLabel="Alternar fundo nas caixas de texto"
          onToggle={() => patchPublic({ text_box_background: !localPublic.text_box_background })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <ImageIcon size={14} class="text-primary" />
            Projetar Imagens em Tela Cheia (Sem Margens de Texto)
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Quando ativado, imagens 16:9 ocupam 100% da tela sem aplicar as margens seguras de texto.
          </p>
        </div>
        <SettingsToggle
          checked={localPublic.ignore_margins_for_media !== false}
          ariaLabel="Alternar margens para imagens"
          onToggle={() =>
            patchPublic({
              ignore_margins_for_media: localPublic.ignore_margins_for_media === false,
            })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-2.5">
        <div class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Maximize2 size={14} class="text-primary" />
          Encaixe do Papel de Parede na Projeção (TV / Chromecast)
        </div>
        <p class="text-[11px] text-muted-foreground">
          Para TVs ou Chromecast com corte de borda (overscan), utilize "Ajustar (Conter)". Sincronizado com Saídas & Telas.
        </p>
        <div
          class="grid grid-cols-1 sm:grid-cols-3 lg:grid-cols-5 gap-2 pt-1"
          role="radiogroup"
          aria-label="Encaixe do papel de parede"
        >
          {#each fitWallOptions as wallFitOpt}
            {@const active = (localPublic.wallpaper_fit || 'contain') === wallFitOpt.id}
            <button
              type="button"
              role="radio"
              aria-checked={active}
              onclick={() => onSelectWallpaperFit(wallFitOpt.id)}
              class="p-2.5 rounded-lg border text-left transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                ? 'border-primary bg-primary/10 text-foreground'
                : 'border-border bg-background text-muted-foreground hover:text-foreground'}"
            >
              <div class="text-xs font-semibold">{wallFitOpt.label}</div>
              <div class="text-[11px] opacity-80 mt-0.5">{wallFitOpt.desc}</div>
            </button>
          {/each}
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card flex items-center justify-between gap-4">
        <div>
          <div class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Sliders size={14} class="text-primary" />
            Margens Seguras no Papel de Parede (Anti-Overscan TV)
          </div>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Aplica as margens seguras também ao plano de fundo, evitando corte de bordas em TVs ou Chromecast.
          </p>
        </div>
        <SettingsToggle
          checked={Boolean(localPublic.wallpaper_safe_margins)}
          ariaLabel="Alternar margens seguras no papel de parede"
          onToggle={() =>
            patchPublic({ wallpaper_safe_margins: !localPublic.wallpaper_safe_margins })}
        />
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-2.5">
        <div class="text-xs font-semibold text-foreground flex items-center gap-2">
          <ImageIcon size={14} class="text-primary" />
          Encaixe Padrão de Imagens de Mídia (Slides/Fotos)
        </div>
        <div
          class="grid grid-cols-2 sm:grid-cols-4 gap-2 pt-1"
          role="radiogroup"
          aria-label="Encaixe padrão de imagens de mídia"
        >
          {#each [
            { id: 'contain', label: 'Ajustar (Conter)', desc: 'Sem cortar' },
            { id: 'cover', label: 'Preencher', desc: 'Corta excedente' },
            { id: 'fill', label: 'Esticar', desc: 'Preenche 100%' },
            { id: 'center', label: 'Original', desc: 'Sem zoom' },
          ] as fitOption}
            {@const active = (localPublic.image_fit || 'contain') === fitOption.id}
            <button
              type="button"
              role="radio"
              aria-checked={active}
              onclick={() => patchPublic({ image_fit: fitOption.id as ImageFitMode })}
              class="p-2.5 rounded-lg border text-left transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
                ? 'border-primary bg-primary/10 text-foreground'
                : 'border-border bg-background text-muted-foreground hover:text-foreground'}"
            >
              <div class="text-xs font-semibold">{fitOption.label}</div>
              <div class="text-[11px] opacity-80 mt-0.5">{fitOption.desc}</div>
            </button>
          {/each}
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div class="flex items-center justify-between gap-4">
          <div>
            <div class="text-xs font-semibold text-foreground flex items-center gap-2">
              <Sparkles size={14} class="text-primary" />
              Transição com Fade Suave (Imagens e Papel de Parede)
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Aplica desvanecimento suave ao projetar imagens e ao retornar para o papel de parede.
            </p>
          </div>
          <SettingsToggle
            checked={localPublic.image_fade_enabled !== false}
            ariaLabel="Alternar transição suave de imagens"
            onToggle={() =>
              patchPublic({ image_fade_enabled: localPublic.image_fade_enabled === false })}
          />
        </div>

        {#if localPublic.image_fade_enabled !== false}
          <SettingsRangeField
            id="pub-image-fade-duration"
            label="Duração da Transição"
            value={localPublic.image_fade_duration_ms ?? 500}
            min={50}
            max={2000}
            step={50}
            unit="ms"
            allowNumberInput
            numberMin={50}
            numberMax={3000}
            numberStep={50}
            minLabel="100ms (Rápido)"
            midLabel="500ms (Recomendado)"
            maxLabel="2000ms (Lento)"
            onchange={(v) => patchPublic({ image_fade_duration_ms: v })}
          />
        {/if}
      </div>

      <!-- Margens Safe Area -->
      <div class="pt-4 border-t border-border space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h4 class="text-xs font-semibold text-foreground">
              Margens da Projeção Pública (Safe Area)
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Evite que as letras encostem nas bordas do telão ou sofram corte por overscan do projetor/TV.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7"
            onclick={() =>
              patchPublic({
                margin_top: DEFAULT_PUBLIC_SETTINGS.margin_top,
                margin_bottom: DEFAULT_PUBLIC_SETTINGS.margin_bottom,
                margin_horizontal: DEFAULT_PUBLIC_SETTINGS.margin_horizontal,
              })}
          >
            Restaurar Margens
          </Button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
          <SettingsRangeField
            id="pub-margin-top"
            label="Margem Topo"
            value={localPublic.margin_top}
            min={0}
            max={160}
            step={4}
            unit="px"
            onchange={(v) => patchPublic({ margin_top: v })}
          />

          <SettingsRangeField
            id="pub-margin-bottom"
            label="Margem Base"
            value={localPublic.margin_bottom}
            min={0}
            max={160}
            step={4}
            unit="px"
            onchange={(v) => patchPublic({ margin_bottom: v })}
          />

          <SettingsRangeField
            id="pub-margin-horizontal"
            label="Laterais"
            value={localPublic.margin_horizontal}
            min={0}
            max={240}
            step={4}
            unit="px"
            onchange={(v) => patchPublic({ margin_horizontal: v })}
          />
        </div>

        <!-- Prévia das Margens -->
        <div class="p-4 rounded-xl border border-border bg-card flex flex-col items-center justify-center">
          <div class="text-[11px] text-muted-foreground mb-2 font-mono uppercase">
            Simulação de Área Útil (16:9)
          </div>
          <div class="w-72 h-40 rounded border border-border bg-zinc-950 flex items-center justify-center p-2 relative overflow-hidden">
            <div
              class="w-full h-full border border-dashed border-primary/70 bg-primary/10 rounded flex items-center justify-center text-center p-1"
              style="margin-top: {localPublic.margin_top / 6}px; margin-bottom: {localPublic.margin_bottom / 6}px; margin-left: {localPublic.margin_horizontal / 6}px; margin-right: {localPublic.margin_horizontal / 6}px;"
            >
              <span class="text-[11px] text-primary font-medium">Área de Letra e Mídia</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

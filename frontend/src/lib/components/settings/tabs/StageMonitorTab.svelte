<script lang="ts">
  import {
    Music,
    BookOpen,
    CalendarDays,
    Sliders,
    Type,
    Clock,
    MessageSquare,
  } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import StagePreviewBox from '$lib/components/StagePreviewBox.svelte';
  import SettingsToggle from '../SettingsToggle.svelte';
  import SettingsRangeField from '../SettingsRangeField.svelte';
  import { DEFAULT_STAGE_SETTINGS } from '$lib/displaySettings';
  import type { StageDisplaySettings } from '$lib/types';
  import type { StageSectionId } from '../types';

  interface Props {
    localStage: StageDisplaySettings;
    onUpdateStage: (next: StageDisplaySettings) => void;
  }

  let { localStage, onUpdateStage }: Props = $props();

  let stageSection = $state<StageSectionId>('lyrics');

  const stageTabs: Array<{ id: StageSectionId; label: string; icon: typeof Music }> = [
    { id: 'lyrics', label: 'Letras de Louvor', icon: Music },
    { id: 'scripture', label: 'Textos Bíblicos', icon: BookOpen },
    { id: 'schedule', label: 'Agenda', icon: CalendarDays },
    { id: 'general', label: 'Layout & Tempo', icon: Sliders },
  ];

  const isSideColumn = $derived(localStage.stage_layout === 'side-column');

  function patchStage(patch: Partial<StageDisplaySettings>) {
    onUpdateStage({ ...localStage, ...patch });
  }

  function updateStageRatio(
    field: 'current_slide_percent' | 'next_slide_percent' | 'footer_time_percent',
    newVal: number
  ) {
    const cur = localStage.current_slide_percent ?? 50;
    const next = localStage.next_slide_percent ?? 30;
    const foot = localStage.footer_time_percent ?? 20;

    if (field === 'current_slide_percent') {
      const a = Math.max(20, Math.min(80, newVal));
      const rem = 100 - a;
      const otherTotal = next + foot || 1;
      let b = Math.round(rem * (next / otherTotal));
      b = Math.max(10, Math.min(rem - 10, b));
      const c = 100 - a - b;
      patchStage({ current_slide_percent: a, next_slide_percent: b, footer_time_percent: c });
    } else if (field === 'next_slide_percent') {
      const b = Math.max(10, Math.min(60, newVal));
      const rem = 100 - b;
      const otherTotal = cur + foot || 1;
      let a = Math.round(rem * (cur / otherTotal));
      a = Math.max(20, Math.min(rem - 10, a));
      const c = 100 - b - a;
      patchStage({ current_slide_percent: a, next_slide_percent: b, footer_time_percent: c });
    } else {
      const c = Math.max(10, Math.min(50, newVal));
      const rem = 100 - c;
      const otherTotal = cur + next || 1;
      let a = Math.round(rem * (cur / otherTotal));
      a = Math.max(20, Math.min(rem - 10, a));
      const b = 100 - c - a;
      patchStage({ current_slide_percent: a, next_slide_percent: b, footer_time_percent: c });
    }
  }
</script>

<div class="space-y-6 max-w-4xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Layout e Cores do Monitor de Retorno</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Configure de forma independente o retorno de letras de louvor, versículos bíblicos e o layout/timer do palco.
    </p>
  </div>

  <!-- Sub-navegação do Monitor de Palco -->
  <div
    class="p-1 bg-muted/50 rounded-xl border border-border grid grid-cols-2 sm:grid-cols-4 gap-1"
    role="tablist"
    aria-label="Subseções do Monitor de Palco"
  >
    {#each stageTabs as sec}
      {@const Icon = sec.icon}
      {@const active = stageSection === sec.id}
      <button
        type="button"
        role="tab"
        aria-selected={active}
        onclick={() => (stageSection = sec.id)}
        class="min-h-8 flex items-center justify-center gap-1.5 py-2 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {active
          ? 'bg-primary text-primary-foreground shadow-xs'
          : 'text-muted-foreground hover:text-foreground hover:bg-background/60'}"
      >
        <Icon size={13} />
        <span>{sec.label}</span>
      </button>
    {/each}
  </div>

  {#if stageSection === 'lyrics'}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div>
          <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Type size={14} class="text-primary" />
            Tamanho Mínimo da Fonte para Louvores
          </h4>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Define o menor tamanho aceitável para letras no monitor de retorno, evitando textos pequenos demais para a equipe.
          </p>
        </div>
        <div class="max-w-md">
          <SettingsRangeField
            id="stage-lyrics-min-font"
            label="Tamanho Mínimo das Letras"
            value={localStage.lyrics_min_font_size ?? localStage.min_font_size ?? 24}
            min={14}
            max={64}
            step={2}
            unit="px"
            minLabel="14px (Mais linhas)"
            midLabel="24px (Padrão)"
            maxLabel="64px (Super legível)"
            onchange={(v) => patchStage({ lyrics_min_font_size: v, min_font_size: v })}
          />
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
              <Sliders size={14} class="text-primary" />
              Cores das Letras e Molduras do Palco
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Personalize as cores do slide ao vivo e do próximo slide para contraste com a iluminação do palco.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7"
            onclick={() =>
              patchStage({
                current_text_color: DEFAULT_STAGE_SETTINGS.current_text_color,
                next_text_color: DEFAULT_STAGE_SETTINGS.next_text_color,
                current_border_color: DEFAULT_STAGE_SETTINGS.current_border_color,
                next_border_color: DEFAULT_STAGE_SETTINGS.next_border_color,
              })}
          >
            Restaurar Cores
          </Button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-cur-text-color" class="text-xs font-semibold text-foreground block">
                Texto do Slide Atual
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.current_text_color}
              </div>
            </div>
            <input
              id="stage-cur-text-color"
              type="color"
              value={localStage.current_text_color}
              oninput={(e) =>
                patchStage({ current_text_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-cur-border-color" class="text-xs font-semibold text-foreground block">
                Moldura do Slide Atual
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.current_border_color}
              </div>
            </div>
            <input
              id="stage-cur-border-color"
              type="color"
              value={localStage.current_border_color}
              oninput={(e) =>
                patchStage({ current_border_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-next-text-color" class="text-xs font-semibold text-foreground block">
                Texto do Próximo Slide
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.next_text_color}
              </div>
            </div>
            <input
              id="stage-next-text-color"
              type="color"
              value={localStage.next_text_color}
              oninput={(e) => patchStage({ next_text_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-next-border-color" class="text-xs font-semibold text-foreground block">
                Moldura do Próximo Slide
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.next_border_color}
              </div>
            </div>
            <input
              id="stage-next-border-color"
              type="color"
              value={localStage.next_border_color}
              oninput={(e) =>
                patchStage({ next_border_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>
        </div>
      </div>
    </div>
  {:else if stageSection === 'scripture'}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div>
          <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Type size={14} class="text-primary" />
            Tamanho Mínimo da Fonte para Passagens Bíblicas
          </h4>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Ajuste o piso mínimo para garantir leitura confortável no púlpito mesmo em versículos extensos.
          </p>
        </div>
        <div class="max-w-md">
          <SettingsRangeField
            id="stage-scripture-min-font"
            label="Tamanho Mínimo Bíblico"
            value={localStage.scripture_min_font_size ?? 24}
            min={14}
            max={64}
            step={2}
            unit="px"
            minLabel="14px (Mais compacto)"
            midLabel="24px (Padrão)"
            maxLabel="64px (Máxima visibilidade)"
            onchange={(v) => patchStage({ scripture_min_font_size: v })}
          />
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
              <Sliders size={14} class="text-primary" />
              Cores do Versículo no Monitor de Palco
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Defina cores personalizadas para as passagens bíblicas no retorno do pregador.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7"
            onclick={() =>
              patchStage({
                scripture_text_color: DEFAULT_STAGE_SETTINGS.scripture_text_color,
                scripture_border_color: DEFAULT_STAGE_SETTINGS.scripture_border_color,
              })}
          >
            Restaurar Padrão
          </Button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-scripture-text-color" class="text-xs font-semibold text-foreground block">
                Cor do Texto do Versículo
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.scripture_text_color || '#FFFFFF'}
              </div>
            </div>
            <input
              id="stage-scripture-text-color"
              type="color"
              value={localStage.scripture_text_color || '#FFFFFF'}
              oninput={(e) =>
                patchStage({ scripture_text_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-scripture-border-color" class="text-xs font-semibold text-foreground block">
                Moldura do Bloco Bíblico
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.scripture_border_color || '#3B82F6'}
              </div>
            </div>
            <input
              id="stage-scripture-border-color"
              type="color"
              value={localStage.scripture_border_color || '#3B82F6'}
              oninput={(e) =>
                patchStage({ scripture_border_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>
        </div>
      </div>
    </div>
  {:else if stageSection === 'schedule'}
    <div class="space-y-4">
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
              <Type size={14} class="text-primary" />
              Slide Atual da Agenda (Ao Vivo no Palco)
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Ajuste as fontes do evento ativo exibido em destaque para os ministrantes e equipe de apoio.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7"
            onclick={() =>
              patchStage({
                stage_schedule_title_min_font_size:
                  DEFAULT_STAGE_SETTINGS.stage_schedule_title_min_font_size,
                stage_schedule_title_max_font_size:
                  DEFAULT_STAGE_SETTINGS.stage_schedule_title_max_font_size,
                stage_schedule_day_font_size: DEFAULT_STAGE_SETTINGS.stage_schedule_day_font_size,
                stage_schedule_time_font_size: DEFAULT_STAGE_SETTINGS.stage_schedule_time_font_size,
                stage_schedule_details_font_size:
                  DEFAULT_STAGE_SETTINGS.stage_schedule_details_font_size,
                stage_schedule_next_min_font_size:
                  DEFAULT_STAGE_SETTINGS.stage_schedule_next_min_font_size,
                stage_schedule_next_max_font_size:
                  DEFAULT_STAGE_SETTINGS.stage_schedule_next_max_font_size,
              })}
          >
            Restaurar Padrão
          </Button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <SettingsRangeField
            id="stage-sched-title-min"
            label="Título (Mínimo Auto-Scale)"
            value={localStage.stage_schedule_title_min_font_size ?? 28}
            min={14}
            max={Math.max(100, localStage.stage_schedule_title_min_font_size ?? 28)}
            step={2}
            unit="px"
            allowNumberInput
            numberMin={12}
            numberMax={200}
            numberStep={2}
            onchange={(v) => patchStage({ stage_schedule_title_min_font_size: v })}
          />

          <SettingsRangeField
            id="stage-sched-title-max"
            label="Título (Máximo Auto-Scale)"
            value={localStage.stage_schedule_title_max_font_size ?? 120}
            min={40}
            max={Math.max(240, localStage.stage_schedule_title_max_font_size ?? 120)}
            step={4}
            unit="px"
            allowNumberInput
            numberMin={20}
            numberMax={300}
            numberStep={4}
            onchange={(v) => patchStage({ stage_schedule_title_max_font_size: v })}
          />

          <SettingsRangeField
            id="stage-sched-day-font"
            label="Badge do Dia da Semana"
            value={localStage.stage_schedule_day_font_size ?? 18}
            min={12}
            max={Math.max(80, localStage.stage_schedule_day_font_size ?? 18)}
            step={1}
            unit="px"
            allowNumberInput
            numberMin={8}
            numberMax={200}
            numberStep={1}
            onchange={(v) => patchStage({ stage_schedule_day_font_size: v })}
          />

          <SettingsRangeField
            id="stage-sched-time-font"
            label="Badge do Horário"
            value={localStage.stage_schedule_time_font_size ?? 18}
            min={12}
            max={Math.max(80, localStage.stage_schedule_time_font_size ?? 18)}
            step={1}
            unit="px"
            allowNumberInput
            numberMin={8}
            numberMax={200}
            numberStep={1}
            onchange={(v) => patchStage({ stage_schedule_time_font_size: v })}
          />
        </div>

        <SettingsRangeField
          id="stage-sched-details-font"
          label="Texto de Detalhes / Descrição"
          value={localStage.stage_schedule_details_font_size ?? 18}
          min={12}
          max={Math.max(80, localStage.stage_schedule_details_font_size ?? 18)}
          step={1}
          unit="px"
          allowNumberInput
          numberMin={8}
          numberMax={200}
          numberStep={1}
          onchange={(v) => patchStage({ stage_schedule_details_font_size: v })}
        />

        <div class="pt-3 border-t border-border space-y-3">
          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-4">
            <div>
              <div class="text-xs font-semibold text-foreground">Exibir Fundo das Pílulas no Palco</div>
              <p class="text-[11px] text-muted-foreground mt-0.5">
                Adiciona molduras arredondadas atrás do dia e horário no retorno dos ministrantes.
              </p>
            </div>
            <SettingsToggle
              checked={localStage.stage_schedule_badge_bg !== false}
              ariaLabel="Alternar fundo das pílulas no palco"
              onToggle={() =>
                patchStage({ stage_schedule_badge_bg: localStage.stage_schedule_badge_bg === false })}
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
              <div>
                <label for="stage-sched-day-color" class="text-xs font-semibold text-foreground block">
                  Cor do Dia no Palco
                </label>
                <div class="text-[11px] text-muted-foreground font-mono uppercase">
                  {localStage.stage_schedule_day_color || '#fcd34d'}
                </div>
              </div>
              <input
                id="stage-sched-day-color"
                type="color"
                value={localStage.stage_schedule_day_color || '#fcd34d'}
                oninput={(e) =>
                  patchStage({ stage_schedule_day_color: (e.target as HTMLInputElement).value })}
                class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
              />
            </div>

            <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
              <div>
                <label for="stage-sched-time-color" class="text-xs font-semibold text-foreground block">
                  Cor do Horário no Palco
                </label>
                <div class="text-[11px] text-muted-foreground font-mono uppercase">
                  {localStage.stage_schedule_time_color || '#ffffff'}
                </div>
              </div>
              <input
                id="stage-sched-time-color"
                type="color"
                value={localStage.stage_schedule_time_color || '#ffffff'}
                oninput={(e) =>
                  patchStage({ stage_schedule_time_color: (e.target as HTMLInputElement).value })}
                class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
              />
            </div>
          </div>
        </div>
      </div>

      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Sliders size={14} class="text-primary" />
          Próximo Slide da Agenda (A Seguir no Palco)
        </h4>
        <p class="text-[11px] text-muted-foreground">
          Configure os limites de auto-scale para o título do próximo evento da agenda no bloco "A Seguir".
        </p>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <SettingsRangeField
            id="stage-sched-next-min"
            label="Título Mínimo A Seguir"
            value={localStage.stage_schedule_next_min_font_size ?? 18}
            min={12}
            max={Math.max(80, localStage.stage_schedule_next_min_font_size ?? 18)}
            step={2}
            unit="px"
            allowNumberInput
            numberMin={10}
            numberMax={150}
            numberStep={2}
            onchange={(v) => patchStage({ stage_schedule_next_min_font_size: v })}
          />

          <SettingsRangeField
            id="stage-sched-next-max"
            label="Título Máximo A Seguir"
            value={localStage.stage_schedule_next_max_font_size ?? 64}
            min={30}
            max={Math.max(160, localStage.stage_schedule_next_max_font_size ?? 64)}
            step={4}
            unit="px"
            allowNumberInput
            numberMin={20}
            numberMax={300}
            numberStep={4}
            onchange={(v) => patchStage({ stage_schedule_next_max_font_size: v })}
          />
        </div>
      </div>

      <!-- Simulação do Retorno (Agenda) -->
      <div class="p-4 rounded-xl border border-border bg-card flex flex-col items-center justify-center">
        <div class="flex items-center justify-between w-full max-w-sm mb-2">
          <span class="text-[11px] text-muted-foreground font-mono uppercase">
            Simulação do Retorno (Agenda)
          </span>
          <span class="text-[11px] px-2 py-0.5 rounded bg-primary/15 text-primary font-mono font-semibold uppercase">
            Layout: {isSideColumn ? 'Coluna' : '3 Linhas'}
          </span>
        </div>

        <div class="w-80 max-w-full">
          <StagePreviewBox
            stageSettings={localStage}
            activeSlide={{
              id: 'demo_sched_cur',
              label: 'Culto de Oração',
              content: 'Momento de oração com toda a igreja e estudo bíblico',
              metadata: {
                schedule_day: 'Quarta-Feira',
                schedule_time: '19:30',
                schedule_details: 'Momento de oração com toda a igreja e estudo bíblico',
              },
            }}
            nextSlide={{
              id: 'demo_sched_next',
              label: 'Culto da Família',
              content: 'Celebração com santa ceia e ministração da palavra',
              metadata: {
                schedule_day: 'Domingo',
                schedule_time: '18:00',
                schedule_details: 'Celebração com santa ceia e ministração da palavra',
              },
            }}
            activeMedia={{
              id: 'demo_sched_med',
              title: 'Programação da Semana',
              kind: 'schedule',
              artist_or_ref: '',
              slides: [],
            }}
            timer={{
              duration_seconds: 600,
              remaining_seconds: 450,
              running: true,
              is_overtime: false,
              allow_overtime: false,
              warning_threshold_seconds: 60,
              stage_message: 'Aviso Palco',
            }}
            stageClock="19:45:00"
          />
        </div>
      </div>
    </div>
  {:else}
    <div class="space-y-4">
      <!-- Banner do Layout Ativo (Dinâmico: 3 Linhas vs Coluna Lateral) -->
      <div class="p-4 rounded-xl border border-primary/50 bg-primary/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div class="space-y-1">
          <div class="flex flex-wrap items-center gap-2">
            <div class="text-sm font-bold text-foreground">
              {isSideColumn ? 'Coluna Lateral Direita (Clássico)' : 'Grid 3 Linhas Horizontais'}
            </div>
            <span class="px-2 py-0.5 rounded text-[11px] font-semibold bg-emerald-500/15 text-emerald-700 dark:text-emerald-400 border border-emerald-500/35">
              Layout Ativo
            </span>
          </div>
          <p class="text-xs text-muted-foreground">
            {#if isSideColumn}
              Slides ocupam a coluna esquerda em grande escala, enquanto aviso, relógio e cronômetro ficam na barra lateral direita.
            {:else}
              Slide atual, próximo slide e barra unificada de aviso/relógio/cronômetro ocupam 100% da largura útil em 3 linhas empilhadas.
            {/if}
          </p>
        </div>

        <!-- Diagrama Estrutural Sincronizado -->
        <div
          aria-hidden="true"
          class="h-16 w-40 bg-zinc-950 rounded-md border border-border p-1.5 shrink-0 {isSideColumn
            ? 'flex gap-1'
            : 'flex flex-col gap-1'}"
        >
          {#if isSideColumn}
            <div class="flex-[2] flex flex-col gap-1">
              <div class="flex-1 rounded-xs border border-blue-500/60 bg-blue-500/20"></div>
              <div class="h-4 rounded-xs border border-amber-500/60 bg-amber-500/20"></div>
            </div>
            <div class="flex-1 rounded-xs border border-zinc-600 bg-zinc-900 flex flex-col items-center justify-center gap-1 p-1">
              <span class="w-full h-1.5 rounded-full bg-emerald-400/80"></span>
              <span class="w-3/4 h-1 rounded-full bg-zinc-400/70"></span>
              <span class="w-3/4 h-1.5 rounded-full bg-red-400/80"></span>
            </div>
          {:else}
            <div class="flex-1 rounded-xs border border-blue-500/60 bg-blue-500/20"></div>
            <div class="h-3.5 rounded-xs border border-amber-500/60 bg-amber-500/20"></div>
            <div class="h-2.5 rounded-xs border border-zinc-600 bg-zinc-900 grid grid-cols-3 gap-1 p-0.5">
              <span class="rounded-xs bg-emerald-400/80"></span>
              <span class="rounded-xs bg-zinc-400/70"></span>
              <span class="rounded-xs bg-red-400/80"></span>
            </div>
          {/if}
        </div>
      </div>

      <!-- Cores do Timer e Relógio -->
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
          <Clock size={14} class="text-primary" />
          Cores do Relógio e Cronômetro
        </h4>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-timer-color" class="text-xs font-semibold text-foreground block">
                Cor do Timer / Cronômetro
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.timer_color}
              </div>
            </div>
            <input
              id="stage-timer-color"
              type="color"
              value={localStage.timer_color}
              oninput={(e) => patchStage({ timer_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-2">
            <div>
              <label for="stage-clock-color" class="text-xs font-semibold text-foreground block">
                Cor do Relógio
              </label>
              <div class="text-[11px] text-muted-foreground font-mono uppercase">
                {localStage.clock_color}
              </div>
            </div>
            <input
              id="stage-clock-color"
              type="color"
              value={localStage.clock_color}
              oninput={(e) => patchStage({ clock_color: (e.target as HTMLInputElement).value })}
              class="w-8 h-8 rounded border border-border cursor-pointer bg-transparent p-0.5"
            />
          </div>
        </div>
      </div>

      <!-- Relógio Oficial -->
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div>
          <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
            <Clock size={14} class="text-primary" />
            Formato e Exibição do Relógio Oficial
          </h4>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Defina a visibilidade dos segundos e da data no rodapé do palco e na tela de descanso.
          </p>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-3">
            <div>
              <div class="text-xs font-semibold text-foreground">Exibir Segundos no Horário</div>
              <div class="text-[11px] text-muted-foreground">
                {localStage.clock_show_seconds !== false
                  ? 'Formato completo (ex: 10:42:15)'
                  : 'Formato limpo (ex: 10:42)'}
              </div>
            </div>
            <SettingsToggle
              checked={localStage.clock_show_seconds !== false}
              ariaLabel="Alternar exibição de segundos no relógio do palco"
              onToggle={() =>
                patchStage({ clock_show_seconds: localStage.clock_show_seconds === false })}
            />
          </div>

          <div class="p-3 rounded-lg border border-border bg-background/70 flex items-center justify-between gap-3">
            <div>
              <div class="text-xs font-semibold text-foreground">Exibir Data Abaixo do Horário</div>
              <div class="text-[11px] text-muted-foreground">
                {localStage.clock_show_date !== false ? 'Data por extenso visível' : 'Data oculta'}
              </div>
            </div>
            <SettingsToggle
              checked={localStage.clock_show_date !== false}
              ariaLabel="Alternar exibição da data no relógio do palco"
              onToggle={() => patchStage({ clock_show_date: localStage.clock_show_date === false })}
            />
          </div>
        </div>
      </div>

      <!-- Tamanhos das Fontes do Palco -->
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
              <Sliders size={14} class="text-primary" />
              Tamanhos das Fontes de Palco (Relógio, Timer & Aviso)
            </h4>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Ajuste o tamanho dos números e textos para leitura confortável à distância no retorno.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7"
            onclick={() =>
              patchStage({
                clock_font_size: DEFAULT_STAGE_SETTINGS.clock_font_size,
                timer_font_size: DEFAULT_STAGE_SETTINGS.timer_font_size,
                alert_font_size: DEFAULT_STAGE_SETTINGS.alert_font_size,
                standby_alert_font_size: DEFAULT_STAGE_SETTINGS.standby_alert_font_size,
              })}
          >
            Restaurar Padrão
          </Button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3.5">
          <SettingsRangeField
            id="stage-timer-font"
            label="Timer / Cronômetro"
            value={localStage.timer_font_size ?? 56}
            min={20}
            max={Math.max(120, localStage.timer_font_size ?? 56)}
            step={2}
            unit="px"
            hint="Contagem e tempo"
            allowNumberInput
            numberMin={14}
            numberMax={200}
            numberStep={2}
            onchange={(v) => patchStage({ timer_font_size: v })}
          />

          <SettingsRangeField
            id="stage-clock-font"
            label="Relógio Oficial"
            value={localStage.clock_font_size ?? 48}
            min={20}
            max={Math.max(120, localStage.clock_font_size ?? 48)}
            step={2}
            unit="px"
            hint="Horário de parede"
            allowNumberInput
            numberMin={14}
            numberMax={200}
            numberStep={2}
            onchange={(v) => patchStage({ clock_font_size: v })}
          />

          <SettingsRangeField
            id="stage-alert-font"
            label="Aviso (Ao Vivo)"
            value={localStage.alert_font_size ?? 28}
            min={14}
            max={Math.max(80, localStage.alert_font_size ?? 28)}
            step={2}
            unit="px"
            hint="Rodapé com slides"
            allowNumberInput
            numberMin={12}
            numberMax={150}
            numberStep={2}
            onchange={(v) => patchStage({ alert_font_size: v })}
          />

          <SettingsRangeField
            id="stage-standby-alert-font"
            label="Aviso (Espera)"
            value={localStage.standby_alert_font_size ?? 36}
            min={16}
            max={Math.max(100, localStage.standby_alert_font_size ?? 36)}
            step={2}
            unit="px"
            hint="Banner em descanso"
            allowNumberInput
            numberMin={14}
            numberMax={150}
            numberStep={2}
            onchange={(v) => patchStage({ standby_alert_font_size: v })}
          />
        </div>
      </div>

      <!-- Aviso em Tela Cheia Temporária -->
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div class="flex items-center justify-between gap-4">
          <div class="space-y-0.5">
            <h4 class="text-xs font-semibold text-foreground flex items-center gap-2">
              <MessageSquare size={14} class="text-primary" />
              Aviso para o Palco em Tela Cheia Temporária
            </h4>
            <p class="text-[11px] text-muted-foreground">
              Ao disparar uma nova mensagem, exibe o texto em tela cheia em destaque antes de recolher para o rodapé.
            </p>
          </div>
          <SettingsToggle
            checked={localStage.alert_fullscreen_flash !== false}
            ariaLabel="Alternar aviso em tela cheia temporária"
            onToggle={() =>
              patchStage({ alert_fullscreen_flash: localStage.alert_fullscreen_flash === false })}
          />
        </div>

        {#if localStage.alert_fullscreen_flash !== false}
          <SettingsRangeField
            id="stage-alert-flash-duration"
            label="Tempo de exibição em tela cheia"
            value={localStage.alert_flash_duration_seconds ?? 5}
            min={2}
            max={20}
            step={1}
            unit="s"
            hint="Após {localStage.alert_flash_duration_seconds ?? 5} segundos (ou com um clique na tela), o aviso volta à caixa do rodapé."
            onchange={(v) => patchStage({ alert_flash_duration_seconds: v })}
          />
        {/if}
      </div>

      <!-- Seletor de Modo de Layout -->
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-3">
        <div>
          <h4 class="text-xs font-semibold text-foreground">Modo de Layout do Monitor de Palco</h4>
          <p class="text-[11px] text-muted-foreground mt-0.5">
            Escolha a disposição visual dos slides e das informações de tempo no retorno.
          </p>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3" role="radiogroup" aria-label="Modo de layout do monitor de palco">
          <button
            type="button"
            role="radio"
            aria-checked={!isSideColumn}
            onclick={() => patchStage({ stage_layout: 'stacked-rows' })}
            class="p-3 rounded-lg border text-left transition-all cursor-pointer flex flex-col justify-between focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {!isSideColumn
              ? 'border-primary bg-primary/10 ring-1 ring-primary/40'
              : 'border-border bg-background/70 hover:bg-muted/40'}"
          >
            <div class="space-y-1">
              <div class="flex items-center justify-between gap-2">
                <span class="text-xs font-bold text-foreground">3 Linhas Horizontais (Recomendado)</span>
                {#if !isSideColumn}
                  <span class="w-2.5 h-2.5 rounded-full bg-primary shrink-0"></span>
                {/if}
              </div>
              <p class="text-[11px] text-muted-foreground">
                Aviso, relógio e timer ficam alinhados abaixo do próximo slide, aproveitando 100% da largura útil da tela para os textos.
              </p>
            </div>
          </button>

          <button
            type="button"
            role="radio"
            aria-checked={isSideColumn}
            onclick={() => patchStage({ stage_layout: 'side-column' })}
            class="p-3 rounded-lg border text-left transition-all cursor-pointer flex flex-col justify-between focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isSideColumn
              ? 'border-primary bg-primary/10 ring-1 ring-primary/40'
              : 'border-border bg-background/70 hover:bg-muted/40'}"
          >
            <div class="space-y-1">
              <div class="flex items-center justify-between gap-2">
                <span class="text-xs font-bold text-foreground">Coluna Lateral Direita (Clássico)</span>
                {#if isSideColumn}
                  <span class="w-2.5 h-2.5 rounded-full bg-primary shrink-0"></span>
                {/if}
              </div>
              <p class="text-[11px] text-muted-foreground">
                Slides na coluna esquerda e uma coluna lateral dedicada à direita para aviso, relógio e timer.
              </p>
            </div>
          </button>
        </div>
      </div>

      <!-- Proporções de Altura -->
      <div class="p-3.5 rounded-xl border border-border bg-card space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <div class="flex flex-wrap items-center gap-2">
              <h4 class="text-xs font-semibold text-foreground">Distribuição de Altura na Tela</h4>
              <span class="px-2 py-0.5 rounded-full text-[11px] font-mono font-bold bg-emerald-500/15 text-emerald-700 dark:text-emerald-300 border border-emerald-500/35">
                Soma: 100%
              </span>
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              Ao ajustar qualquer linha, as demais recalculam automaticamente para totalizar 100%.
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            class="text-[11px] h-7"
            onclick={() =>
              patchStage({
                current_slide_percent: DEFAULT_STAGE_SETTINGS.current_slide_percent,
                next_slide_percent: DEFAULT_STAGE_SETTINGS.next_slide_percent,
                footer_time_percent: DEFAULT_STAGE_SETTINGS.footer_time_percent,
              })}
          >
            Restaurar Padrão (50/30/20)
          </Button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
          <SettingsRangeField
            id="stage-ratio-current"
            label="1. Slide Atual"
            value={localStage.current_slide_percent ?? 50}
            min={20}
            max={80}
            step={1}
            unit="%"
            hint="Área principal do texto ao vivo"
            onchange={(v) => updateStageRatio('current_slide_percent', v)}
          />

          <SettingsRangeField
            id="stage-ratio-next"
            label="2. Próximo Slide"
            value={localStage.next_slide_percent ?? 30}
            min={10}
            max={60}
            step={1}
            unit="%"
            hint="Prévia do próximo trecho"
            onchange={(v) => updateStageRatio('next_slide_percent', v)}
          />

          <SettingsRangeField
            id="stage-ratio-footer"
            label="3. Aviso, Relógio & Timer"
            value={localStage.footer_time_percent ?? 20}
            min={10}
            max={50}
            step={1}
            unit="%"
            hint="Barra auxiliar inferior"
            onchange={(v) => updateStageRatio('footer_time_percent', v)}
          />
        </div>

        <div class="p-4 rounded-xl border border-border bg-card flex flex-col items-center justify-center">
          <div class="flex items-center justify-between w-full max-w-sm mb-2">
            <span class="text-[11px] text-muted-foreground font-mono uppercase">Simulação do Retorno</span>
            <span class="text-[11px] px-2 py-0.5 rounded bg-primary/15 text-primary font-mono font-semibold uppercase">
              Layout: {isSideColumn ? 'Coluna' : '3 Linhas'}
            </span>
          </div>

          <div class="w-80 max-w-full">
            <StagePreviewBox
              stageSettings={localStage}
              activeSlide={{
                id: 'demo_cur',
                label: 'João 3:16',
                content: 'Porque Deus amou o mundo de tal maneira que deu o seu Filho unigênito',
              }}
              nextSlide={{
                id: 'demo_next',
                label: 'João 3:17',
                content: 'Para que todo aquele que nele crê não pereça, mas tenha a vida eterna',
              }}
              activeMedia={{
                id: 'demo_med',
                title: 'Evangelho de João',
                kind: 'scripture',
                artist_or_ref: 'ARA',
                slides: [],
              }}
              timer={{
                duration_seconds: 600,
                remaining_seconds: 450,
                running: true,
                is_overtime: false,
                allow_overtime: false,
                warning_threshold_seconds: 60,
                stage_message: 'Aviso Palco',
              }}
              stageClock="19:45:00"
            />
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

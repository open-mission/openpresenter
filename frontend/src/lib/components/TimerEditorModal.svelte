<script lang="ts">
  import { cn } from '$lib/utils';
  import type { MediaItem, Slide } from '$lib/types';
  import Button from './ui/Button.svelte';
  import { X, Clock, Hourglass, Maximize2, Minimize2, Sparkles, Check, ListPlus } from 'lucide-svelte';

  interface Props {
    open: boolean;
    initialItem?: MediaItem | null;
    onclose: () => void;
    onsave: (item: MediaItem) => void;
    onaddtosetlist?: (item: MediaItem) => void;
  }

  let { open = false, initialItem = null, onclose, onsave, onaddtosetlist }: Props = $props();

  let isMaximized = $state(false);
  let title = $state('');
  let minutes = $state(5);
  let seconds = $state(0);
  let subtitle = $state('O culto começará em instantes. Prepare o seu coração!');
  let allowOvertime = $state(true);
  let warningSeconds = $state(60);
  let prevOpen = false;

  const totalDurationSeconds = $derived(Math.max(0, minutes * 60 + seconds));

  const PRESETS = [
    { label: '1 min', mins: 1, secs: 0 },
    { label: '2 min', mins: 2, secs: 0 },
    { label: '3 min', mins: 3, secs: 0 },
    { label: '5 min', mins: 5, secs: 0 },
    { label: '10 min', mins: 10, secs: 0 },
    { label: '15 min', mins: 15, secs: 0 },
    { label: '30 min', mins: 30, secs: 0 },
  ];

  function applyPreset(pMins: number, pSecs: number) {
    minutes = pMins;
    seconds = pSecs;
  }

  function formatDisplayTime(totSecs: number): string {
    const m = Math.floor(totSecs / 60);
    const s = totSecs % 60;
    return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
  }

  $effect(() => {
    if (open && !prevOpen) {
      if (initialItem && initialItem.kind === 'timer') {
        title = initialItem.title;
        const dur = parseInt(initialItem.slides[0]?.content || '300', 10);
        if (!isNaN(dur) && dur >= 0) {
          minutes = Math.floor(dur / 60);
          seconds = dur % 60;
        } else {
          minutes = 5;
          seconds = 0;
        }
        subtitle = initialItem.slides[0]?.lower_content || '';
      } else {
        title = 'Contagem Regressiva — Início do Culto';
        minutes = 5;
        seconds = 0;
        subtitle = 'O culto começará em instantes. Prepare o seu coração!';
      }
    }
    prevOpen = open;
  });

  function buildMediaItem(): MediaItem {
    const itemTitle = title.trim() || 'Contagem Regressiva';
    const totalSecs = Math.max(1, minutes * 60 + seconds);
    const formatted = formatDisplayTime(totalSecs);

    const slide: Slide = {
      id: `timer-slide-${Date.now()}`,
      label: 'Contagem Regressiva',
      content: String(totalSecs),
      lower_content: subtitle.trim(),
    };

    return {
      id: initialItem?.id || `timer-${Date.now()}`,
      title: itemTitle,
      artist_or_ref: formatted,
      kind: 'timer',
      slides: [slide],
    };
  }

  function handleSaveOnly() {
    const item = buildMediaItem();
    onsave(item);
    onclose();
  }

  function handleSaveAndAddToSetlist() {
    const item = buildMediaItem();
    onsave(item);
    onaddtosetlist?.(item);
    onclose();
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-xs animate-in fade-in duration-150 {isMaximized ? 'p-0' : 'p-4'}"
    onclick={onclose}
  >
    <div
      class={cn(
        'bg-neutral-900 border border-neutral-800 shadow-2xl flex flex-col text-neutral-100 select-none overflow-hidden transition-all duration-150',
        isMaximized ? 'w-full h-full rounded-none border-0' : 'w-full max-w-xl rounded-2xl max-h-[90vh]'
      )}
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Cabeçalho -->
      <div class="h-14 px-6 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-900/60">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-400 flex items-center justify-center">
            <Hourglass size={16} />
          </div>
          <div>
            <h3 class="text-sm font-bold text-white flex items-center gap-1.5">
              <span>{initialItem ? 'Editar Cronômetro' : 'Novo Cronômetro / Timer'}</span>
              <span class="text-[10px] font-semibold bg-amber-500/10 text-amber-400 border border-amber-500/25 px-1.5 py-0.2 rounded">
                Tela Pública + Palco
              </span>
            </h3>
            <p class="text-[11px] text-neutral-400">
              Projeta contagem regressiva em destaque no telão sincronizada com o stage
            </p>
          </div>
        </div>

        <div class="flex items-center gap-1">
          <button
            type="button"
            onclick={() => (isMaximized = !isMaximized)}
            class="w-8 h-8 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
            title={isMaximized ? 'Restaurar janela' : 'Maximizar'}
          >
            {#if isMaximized}
              <Minimize2 size={15} />
            {:else}
              <Maximize2 size={15} />
            {/if}
          </button>
          <button
            type="button"
            onclick={onclose}
            class="w-8 h-8 rounded-lg text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
            title="Fechar"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      <!-- Conteúdo do Formulário -->
      <div class="p-6 space-y-5 overflow-y-auto flex-1">
        <!-- Título -->
        <div>
          <label for="timer-title" class="block text-xs font-bold text-neutral-300 uppercase tracking-wider mb-1.5">
            Título / Identificação do Timer
          </label>
          <input
            id="timer-title"
            type="text"
            bind:value={title}
            placeholder="Ex.: Contagem Regressiva — Início do Culto"
            class="w-full h-10 px-3.5 bg-neutral-950 border border-neutral-800 rounded-xl text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-amber-500 transition-colors"
          />
        </div>

        <!-- Seletor de Duração com Presets e Inputs -->
        <div class="p-4 rounded-2xl bg-neutral-950/70 border border-neutral-800/80 space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-neutral-300 uppercase tracking-wider flex items-center gap-1.5">
              <Clock size={14} class="text-amber-400" />
              <span>Duração da Contagem</span>
            </span>
            <div class="text-sm font-mono font-bold text-amber-300 bg-amber-500/10 px-2.5 py-0.5 rounded-lg border border-amber-500/25">
              {formatDisplayTime(totalDurationSeconds)}
            </div>
          </div>

          <!-- Presets Rápidos -->
          <div class="flex flex-wrap gap-1.5">
            {#each PRESETS as p}
              {@const isSelected = minutes === p.mins && seconds === p.secs}
              <button
                type="button"
                onclick={() => applyPreset(p.mins, p.secs)}
                class={cn(
                  'px-2.5 py-1 rounded-lg text-xs font-semibold transition-all cursor-pointer border',
                  isSelected
                    ? 'bg-amber-500 text-black border-amber-400 font-bold shadow-xs'
                    : 'bg-neutral-900 hover:bg-neutral-800 text-neutral-300 border-neutral-800'
                )}
              >
                {p.label}
              </button>
            {/each}
          </div>

          <!-- Inputs Numéricos (Minutos : Segundos) -->
          <div class="grid grid-cols-2 gap-3 pt-1">
            <div>
              <label for="timer-mins" class="block text-[11px] font-semibold text-neutral-400 mb-1">
                Minutos
              </label>
              <input
                id="timer-mins"
                type="number"
                min="0"
                max="599"
                bind:value={minutes}
                class="w-full h-10 px-3 bg-neutral-900 border border-neutral-800 rounded-xl text-sm font-mono text-white text-center focus:outline-none focus:border-amber-500"
              />
            </div>
            <div>
              <label for="timer-secs" class="block text-[11px] font-semibold text-neutral-400 mb-1">
                Segundos
              </label>
              <input
                id="timer-secs"
                type="number"
                min="0"
                max="59"
                bind:value={seconds}
                class="w-full h-10 px-3 bg-neutral-900 border border-neutral-800 rounded-xl text-sm font-mono text-white text-center focus:outline-none focus:border-amber-500"
              />
            </div>
          </div>
        </div>

        <!-- Mensagem de Apoio / Subtítulo no Telão -->
        <div>
          <label for="timer-subtitle" class="block text-xs font-bold text-neutral-300 uppercase tracking-wider mb-1.5">
            Mensagem de Apoio / Subtítulo (opcional)
          </label>
          <input
            id="timer-subtitle"
            type="text"
            bind:value={subtitle}
            placeholder="Ex.: O culto começará em instantes. Prepare o seu coração!"
            class="w-full h-10 px-3.5 bg-neutral-950 border border-neutral-800 rounded-xl text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-amber-500 transition-colors"
          />
          <p class="text-[11px] text-neutral-400 mt-1">
            Esta frase é projetada logo abaixo dos dígitos da contagem na tela pública.
          </p>
        </div>

        <!-- Opções Avançadas: Overtime & Alerta -->
        <div class="p-3.5 rounded-xl bg-neutral-950/60 border border-neutral-800 text-xs flex items-center justify-between gap-4">
          <div class="space-y-0.5">
            <span class="font-bold text-neutral-200 block">Permitir Overtime</span>
            <p class="text-[11px] text-neutral-400">
              Continua contando com sinal negativo após chegar a zero (-00:01, -00:02...)
            </p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer shrink-0" aria-label="Permitir overtime">
            <input type="checkbox" bind:checked={allowOvertime} class="sr-only peer" />
            <div class="w-9 h-5 bg-neutral-800 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-neutral-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-amber-500"></div>
          </label>
        </div>
      </div>

      <!-- Rodapé com Ações -->
      <div class="h-16 px-6 border-t border-neutral-800 bg-neutral-900/60 flex items-center justify-between shrink-0">
        <Button variant="ghost" size="sm" onclick={onclose} class="text-xs">
          Cancelar
        </Button>

        <div class="flex items-center gap-2">
          <Button
            variant="secondary"
            size="sm"
            onclick={handleSaveOnly}
            class="text-xs"
          >
            Salvar na Biblioteca
          </Button>

          <Button
            variant="default"
            size="sm"
            onclick={handleSaveAndAddToSetlist}
            class="text-xs bg-amber-500 hover:bg-amber-400 text-black font-bold gap-1.5 shadow-sm"
          >
            <ListPlus size={14} />
            <span>Salvar e Adicionar à Setlist</span>
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import {
    X,
    CalendarDays,
    Plus,
    Trash2,
    ArrowUp,
    ArrowDown,
    Clock,
    Calendar,
    Sparkles,
    Check,
    Eye,
  } from 'lucide-svelte';
  import Button from './ui/Button.svelte';
  import Badge from './ui/Badge.svelte';
  import type { MediaItem, Slide, ScheduleEvent } from '$lib/types';

  interface Props {
    open: boolean;
    initialItem?: MediaItem | null;
    onclose: () => void;
    onsave: (item: MediaItem) => void;
  }

  let { open = false, initialItem = null, onclose, onsave }: Props = $props();

  let title = $state('Programação Semanal');
  let artistOrRef = $state('Agenda');
  let events = $state<ScheduleEvent[]>([]);
  let selectedEventIndex = $state(0);
  let error = $state<string | null>(null);

  // Inicializa o formulário com dados existentes ou template padrão
  $effect(() => {
    if (open) {
      error = null;
      if (initialItem && initialItem.kind === 'schedule') {
        title = initialItem.title || 'Programação Semanal';
        artistOrRef = initialItem.artist_or_ref || 'Agenda';

        const parsedEvents: ScheduleEvent[] = (initialItem.slides || []).map((s, idx) => {
          const lines = (s.content || '').split('\n').map((l) => l.trim()).filter(Boolean);
          let time = '';
          let eventTitle = '';
          let details = '';

          if (lines.length >= 3) {
            time = lines[0];
            eventTitle = lines[1];
            details = lines.slice(2).join(' • ');
          } else if (lines.length === 2) {
            // Verifica se a primeira linha tem formato de horário (ex: 19h, 19:30, 20h00)
            if (/\d+[:hH]/.test(lines[0])) {
              time = lines[0];
              eventTitle = lines[1];
            } else {
              eventTitle = lines[0];
              details = lines[1];
            }
          } else if (lines.length === 1) {
            eventTitle = lines[0];
          }

          return {
            id: s.id || `event-${Date.now()}-${idx}`,
            dayOrDate: s.label || `Culto ${idx + 1}`,
            time: time || '19h30',
            title: eventTitle || 'Culto de Celebração',
            details: details || '',
          };
        });

        events = parsedEvents.length > 0 ? parsedEvents : getDefaultEvents();
      } else {
        title = 'Programação Semanal';
        artistOrRef = 'Agenda';
        events = getDefaultEvents();
      }
      selectedEventIndex = 0;
    }
  });

  function getDefaultEvents(): ScheduleEvent[] {
    return [
      {
        id: `ev-${Date.now()}-1`,
        dayOrDate: 'Quarta-feira',
        time: '19h30',
        title: 'Culto de Oração e Doutrina',
        details: 'Templo Central',
      },
      {
        id: `ev-${Date.now()}-2`,
        dayOrDate: 'Sábado',
        time: '19h30',
        title: 'Culto de Jovens e Adolescentes',
        details: 'Anexo de Eventos',
      },
      {
        id: `ev-${Date.now()}-3`,
        dayOrDate: 'Domingo',
        time: '09h00 e 18h30',
        title: 'Culto da Família e Celebração',
        details: 'Com Escola Bíblica e Santa Ceia',
      },
    ];
  }

  function addEmptyEvent(presetDay?: string, presetTime?: string, presetTitle?: string) {
    const newEvent: ScheduleEvent = {
      id: `ev-${Date.now()}-${events.length + 1}`,
      dayOrDate: presetDay || 'Novo Culto / Evento',
      time: presetTime || '19h30',
      title: presetTitle || 'Culto de Celebração',
      details: '',
    };
    events = [...events, newEvent];
    selectedEventIndex = events.length - 1;
  }

  function removeEvent(index: number) {
    if (events.length <= 1) {
      error = 'A agenda deve conter ao menos um culto ou evento.';
      return;
    }
    events = events.filter((_, idx) => idx !== index);
    if (selectedEventIndex >= events.length) {
      selectedEventIndex = Math.max(0, events.length - 1);
    }
  }

  function moveEvent(index: number, direction: 'up' | 'down') {
    const targetIdx = direction === 'up' ? index - 1 : index + 1;
    if (targetIdx < 0 || targetIdx >= events.length) return;

    const copy = [...events];
    const temp = copy[index];
    copy[index] = copy[targetIdx];
    copy[targetIdx] = temp;
    events = copy;
    selectedEventIndex = targetIdx;
  }

  function handleSave() {
    if (!title.trim()) {
      error = 'Informe um título geral para a programação.';
      return;
    }

    if (events.length === 0) {
      error = 'Adicione ao menos um evento à agenda.';
      return;
    }

    for (let i = 0; i < events.length; i++) {
      const ev = events[i];
      if (!ev.dayOrDate.trim() || !ev.title.trim()) {
        error = `Preencha o dia/data e o título do evento na posição #${i + 1}.`;
        selectedEventIndex = i;
        return;
      }
    }

    // Converte os eventos estruturados em slides compatíveis com o motor do OpenPresenter
    const slides: Slide[] = events.map((ev, idx) => {
      const parts: string[] = [];
      if (ev.time.trim()) parts.push(ev.time.trim());
      parts.push(ev.title.trim());
      if (ev.details?.trim()) parts.push(ev.details.trim());

      return {
        id: ev.id || `slide-${Date.now()}-${idx + 1}`,
        label: ev.dayOrDate.trim(),
        content: parts.join('\n'),
        align_h: 'center',
        align_v: 'center',
      };
    });

    const item: MediaItem = {
      id: initialItem?.id || `schedule-${Date.now()}`,
      title: title.trim(),
      artist_or_ref: artistOrRef.trim() || 'Agenda',
      kind: 'schedule',
      slides,
      align_h: 'center',
      align_v: 'center',
    };

    onsave(item);
    onclose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.stopPropagation();
      onclose();
    } else if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      handleSave();
    }
  }

  const activePreviewEvent = $derived(events[selectedEventIndex] || events[0]);
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-md p-3 sm:p-5 animate-in fade-in duration-150"
    onclick={onclose}
    role="presentation"
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="w-full max-w-4xl max-h-[92vh] rounded-2xl border border-neutral-800 bg-neutral-900 shadow-2xl flex flex-col overflow-hidden text-neutral-100"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <!-- Header do Modal -->
      <div class="h-16 px-6 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-900/80">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-amber-500/10 border border-amber-500/20 text-amber-400 flex items-center justify-center shrink-0">
            <CalendarDays size={20} />
          </div>
          <div>
            <h2 class="text-base font-bold text-white flex items-center gap-2">
              <span>{initialItem ? 'Editar Programação / Agenda' : 'Nova Programação / Agenda'}</span>
              <Badge variant="outline" class="text-[10px] text-amber-300 border-amber-500/30 font-mono">
                {events.length} {events.length === 1 ? 'evento' : 'eventos'}
              </Badge>
            </h2>
            <p class="text-xs text-neutral-400">
              Cadastre os cultos da semana ou eventos futuros. Cada dia vira um slide conectado à tela pública e ao palco.
            </p>
          </div>
        </div>

        <button
          type="button"
          onclick={onclose}
          class="w-9 h-9 rounded-xl text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
          title="Fechar (Esc)"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Mensagem de erro caso exista -->
      {#if error}
        <div class="px-6 py-2.5 bg-rose-950/50 border-b border-rose-500/30 text-rose-200 text-xs flex items-center justify-between shrink-0">
          <span>{error}</span>
          <button type="button" onclick={() => (error = null)} class="text-rose-400 hover:text-white cursor-pointer">
            <X size={14} />
          </button>
        </div>
      {/if}

      <!-- Corpo Principal em Grid de 2 Colunas -->
      <div class="flex-1 min-h-0 overflow-y-auto p-6 grid grid-cols-1 lg:grid-cols-12 gap-6">
        <!-- Coluna da Esquerda: Formulário e Lista de Dias (7 colunas) -->
        <div class="lg:col-span-7 flex flex-col gap-4">
          <!-- Metadados da Agenda -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 p-3.5 rounded-xl bg-neutral-950/60 border border-neutral-800/80">
            <div class="sm:col-span-2 space-y-1">
              <label for="sched-title" class="text-[11px] font-semibold text-neutral-400 block uppercase tracking-wider">
                Título da Programação
              </label>
              <input
                id="sched-title"
                type="text"
                bind:value={title}
                placeholder="Ex: Programação Semanal dos Cultos"
                class="w-full px-3 py-1.5 rounded-lg bg-neutral-900 border border-neutral-700/80 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-amber-400 transition-colors"
              />
            </div>
            <div class="space-y-1">
              <label for="sched-ref" class="text-[11px] font-semibold text-neutral-400 block uppercase tracking-wider">
                Categoria / Tag
              </label>
              <input
                id="sched-ref"
                type="text"
                bind:value={artistOrRef}
                placeholder="Ex: Agenda Semanal"
                class="w-full px-3 py-1.5 rounded-lg bg-neutral-900 border border-neutral-700/80 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-amber-400 transition-colors"
              />
            </div>
          </div>

          <!-- Atalhos Rápidos para Inserir Dias Comuns -->
          <div class="space-y-1.5">
            <div class="flex items-center justify-between">
              <span class="text-[11px] font-semibold text-neutral-400 uppercase tracking-wider flex items-center gap-1.5">
                <Sparkles size={12} class="text-amber-400" />
                <span>Adicionar Atalho Rápido:</span>
              </span>
            </div>
            <div class="flex flex-wrap gap-1.5">
              <button
                type="button"
                onclick={() => addEmptyEvent('Domingo', '09h00 e 18h30', 'Culto de Celebração e Família')}
                class="px-2.5 py-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-medium border border-neutral-700 hover:border-amber-500/50 transition-colors cursor-pointer"
              >
                + Domingo
              </button>
              <button
                type="button"
                onclick={() => addEmptyEvent('Quarta-feira', '19h30', 'Culto de Ensino da Palavra')}
                class="px-2.5 py-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-medium border border-neutral-700 hover:border-amber-500/50 transition-colors cursor-pointer"
              >
                + Quarta
              </button>
              <button
                type="button"
                onclick={() => addEmptyEvent('Sábado', '19h30', 'Culto de Jovens')}
                class="px-2.5 py-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-medium border border-neutral-700 hover:border-amber-500/50 transition-colors cursor-pointer"
              >
                + Sábado
              </button>
              <button
                type="button"
                onclick={() => addEmptyEvent('Terça-feira', '19h30', 'Culto de Oração e Intercessão')}
                class="px-2.5 py-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-medium border border-neutral-700 hover:border-amber-500/50 transition-colors cursor-pointer"
              >
                + Terça
              </button>
              <button
                type="button"
                onclick={() => addEmptyEvent('Segunda-feira', '19h30', 'Reunião de Líderes')}
                class="px-2.5 py-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-medium border border-neutral-700 hover:border-amber-500/50 transition-colors cursor-pointer"
              >
                + Segunda
              </button>
              <button
                type="button"
                onclick={() => addEmptyEvent('Quinta-feira', '19h30', 'Culto nos Lares / PG')}
                class="px-2.5 py-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-medium border border-neutral-700 hover:border-amber-500/50 transition-colors cursor-pointer"
              >
                + Quinta
              </button>
              <button
                type="button"
                onclick={() => addEmptyEvent('Sexta-feira', '20h00', 'Vigília de Oração')}
                class="px-2.5 py-1 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-xs font-medium border border-neutral-700 hover:border-amber-500/50 transition-colors cursor-pointer"
              >
                + Sexta
              </button>
              <button
                type="button"
                onclick={() => addEmptyEvent('15 a 17 de Novembro', '19h00', 'Conferência Anual da Igreja')}
                class="px-2.5 py-1 rounded-lg bg-amber-500/10 hover:bg-amber-500/20 text-amber-300 text-xs font-medium border border-amber-500/30 transition-colors cursor-pointer"
              >
                + Evento / Festividade Futura
              </button>
            </div>
          </div>

          <!-- Lista de Eventos Cadastrados -->
          <div class="space-y-2">
            <div class="flex items-center justify-between text-xs text-neutral-400">
              <span class="font-semibold uppercase tracking-wider">Cultos e Eventos da Agenda:</span>
              <button
                type="button"
                onclick={() => addEmptyEvent()}
                class="inline-flex items-center gap-1 text-amber-400 hover:text-amber-300 font-semibold cursor-pointer"
              >
                <Plus size={14} />
                <span>Novo Evento</span>
              </button>
            </div>

            <div class="space-y-2.5">
              {#each events as event, index (event.id)}
                <div
                  class="p-3.5 rounded-xl border transition-all {selectedEventIndex === index
                    ? 'bg-neutral-950 border-amber-500/60 ring-1 ring-amber-500/30 shadow-md'
                    : 'bg-neutral-950/60 border-neutral-800/80 hover:border-neutral-700'}"
                  onclick={() => (selectedEventIndex = index)}
                  role="presentation"
                >
                  <div class="flex items-center justify-between gap-2 mb-2 pb-2 border-b border-neutral-800/70">
                    <div class="flex items-center gap-2">
                      <span class="w-5 h-5 rounded-full bg-neutral-800 text-neutral-300 text-[10px] font-mono font-bold flex items-center justify-center">
                        {index + 1}
                      </span>
                      <span class="text-xs font-semibold text-white">
                        {event.dayOrDate || 'Sem dia definido'}
                      </span>
                    </div>

                    <!-- Botões de Reordenação e Exclusão -->
                    <div class="flex items-center gap-1">
                      <button
                        type="button"
                        disabled={index === 0}
                        onclick={(e) => {
                          e.stopPropagation();
                          moveEvent(index, 'up');
                        }}
                        class="p-1 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 disabled:opacity-30 disabled:hover:bg-transparent cursor-pointer"
                        title="Mover para cima"
                      >
                        <ArrowUp size={13} />
                      </button>
                      <button
                        type="button"
                        disabled={index === events.length - 1}
                        onclick={(e) => {
                          e.stopPropagation();
                          moveEvent(index, 'down');
                        }}
                        class="p-1 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 disabled:opacity-30 disabled:hover:bg-transparent cursor-pointer"
                        title="Mover para baixo"
                      >
                        <ArrowDown size={13} />
                      </button>
                      <button
                        type="button"
                        onclick={(e) => {
                          e.stopPropagation();
                          removeEvent(index);
                        }}
                        class="p-1 rounded text-neutral-400 hover:text-rose-400 hover:bg-rose-950/30 transition-colors ml-1 cursor-pointer"
                        title="Remover evento"
                      >
                        <Trash2 size={13} />
                      </button>
                    </div>
                  </div>

                  <!-- Campos de Edição do Evento -->
                  <div class="grid grid-cols-1 sm:grid-cols-12 gap-2.5">
                    <div class="sm:col-span-5 space-y-1">
                      <label for={`ev-day-${event.id}`} class="text-[10px] font-medium text-neutral-400 flex items-center gap-1">
                        <Calendar size={10} class="text-amber-400" />
                        <span>Dia da Semana / Data</span>
                      </label>
                      <input
                        id={`ev-day-${event.id}`}
                        type="text"
                        bind:value={event.dayOrDate}
                        placeholder="Ex: Quarta-feira, 20 de Dezembro"
                        class="w-full px-2.5 py-1 rounded bg-neutral-900 border border-neutral-700/80 text-xs text-white focus:outline-none focus:border-amber-400"
                      />
                    </div>

                    <div class="sm:col-span-7 space-y-1">
                      <label for={`ev-time-${event.id}`} class="text-[10px] font-medium text-neutral-400 flex items-center gap-1">
                        <Clock size={10} class="text-amber-400" />
                        <span>Horário</span>
                      </label>
                      <input
                        id={`ev-time-${event.id}`}
                        type="text"
                        bind:value={event.time}
                        placeholder="Ex: 19h30 ou 09h00 e 18h30"
                        class="w-full px-2.5 py-1 rounded bg-neutral-900 border border-neutral-700/80 text-xs text-white focus:outline-none focus:border-amber-400"
                      />
                    </div>

                    <div class="sm:col-span-12 space-y-1">
                      <label for={`ev-title-${event.id}`} class="text-[10px] font-medium text-neutral-400 block">
                        Nome do Culto / Festividade
                      </label>
                      <input
                        id={`ev-title-${event.id}`}
                        type="text"
                        bind:value={event.title}
                        placeholder="Ex: Culto de Oração e Doutrina"
                        class="w-full px-2.5 py-1 rounded bg-neutral-900 border border-neutral-700/80 text-xs text-white font-medium focus:outline-none focus:border-amber-400"
                      />
                    </div>

                    <div class="sm:col-span-12 space-y-1">
                      <label for={`ev-det-${event.id}`} class="text-[10px] font-medium text-neutral-400 block">
                        Detalhes / Local / Preletores (Opcional)
                      </label>
                      <input
                        id={`ev-det-${event.id}`}
                        type="text"
                        bind:value={event.details}
                        placeholder="Ex: Templo Central • Com Santa Ceia"
                        class="w-full px-2.5 py-1 rounded bg-neutral-900 border border-neutral-700/80 text-xs text-neutral-300 focus:outline-none focus:border-amber-400"
                      />
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </div>

        <!-- Coluna da Direita: Prévia Visual ao Vivo na Projeção e Palco (5 colunas) -->
        <div class="lg:col-span-5 flex flex-col gap-4">
          <div class="p-4 rounded-xl bg-neutral-950/80 border border-neutral-800 flex flex-col gap-3 sticky top-0">
            <div class="flex items-center justify-between">
              <span class="text-xs font-semibold text-neutral-300 flex items-center gap-1.5">
                <Eye size={14} class="text-amber-400" />
                <span>Prévia do Slide Selecionado</span>
              </span>
              <Badge variant="outline" class="text-[10px] text-neutral-400">
                Slide {selectedEventIndex + 1} de {events.length}
              </Badge>
            </div>

            <!-- Preview 16:9 Estilizado da Tela Pública -->
            <div class="w-full aspect-video rounded-xl bg-gradient-to-br from-neutral-900 via-neutral-950 to-black border border-neutral-800 p-4 flex flex-col items-center justify-center text-center relative overflow-hidden shadow-inner select-none">
              <!-- Grid sutil de fundo para simular tela de projeção -->
              <div class="absolute inset-0 bg-[radial-gradient(#ffffff08_1px,transparent_1px)] [background-size:12px_12px] opacity-70 pointer-events-none"></div>

              {#if activePreviewEvent}
                <div class="relative z-10 max-w-[90%] space-y-2 flex flex-col items-center">
                  <!-- Badges superiores de Dia e Horário -->
                  <div class="flex items-center gap-1.5 flex-wrap justify-center">
                    <span class="px-2.5 py-0.5 rounded-full bg-amber-500/20 text-amber-300 border border-amber-500/40 text-[10px] font-bold uppercase tracking-wider">
                      {activePreviewEvent.dayOrDate || 'Dia do Evento'}
                    </span>
                    {#if activePreviewEvent.time}
                      <span class="px-2 py-0.5 rounded-full bg-neutral-800/90 text-neutral-200 border border-neutral-700 text-[10px] font-mono flex items-center gap-1">
                        <Clock size={10} class="text-amber-400" />
                        <span>{activePreviewEvent.time}</span>
                      </span>
                    {/if}
                  </div>

                  <!-- Título com Destaque -->
                  <h3 class="text-sm sm:text-base font-black text-white leading-tight tracking-tight drop-shadow-md">
                    {activePreviewEvent.title || 'Nome do Culto'}
                  </h3>

                  <!-- Detalhes do Evento -->
                  {#if activePreviewEvent.details}
                    <p class="text-[11px] text-neutral-400 font-medium leading-snug line-clamp-2">
                      {activePreviewEvent.details}
                    </p>
                  {/if}
                </div>
              {:else}
                <span class="text-xs text-neutral-500">Nenhum evento selecionado</span>
              {/if}
            </div>

            <!-- Demonstração de como aparece no Monitor de Palco -->
            <div class="pt-3 border-t border-neutral-800/80 space-y-2">
              <span class="text-[11px] font-semibold text-neutral-400 block uppercase tracking-wider">
                Comportamento no Monitor de Palco (/stage):
              </span>
              <div class="grid grid-cols-2 gap-2 text-[10px] font-mono">
                <div class="p-2 rounded bg-zinc-900 border border-zinc-800">
                  <span class="text-amber-400 font-bold block mb-0.5">SLIDE ATUAL:</span>
                  <div class="text-white font-semibold truncate">{activePreviewEvent?.dayOrDate || '-'}</div>
                  <div class="text-zinc-400 truncate text-[9px]">{activePreviewEvent?.time} • {activePreviewEvent?.title}</div>
                </div>
                <div class="p-2 rounded bg-zinc-900 border border-zinc-800">
                  <span class="text-yellow-400 font-bold block mb-0.5">A SEGUIR:</span>
                  {#if events[selectedEventIndex + 1]}
                    <div class="text-white font-semibold truncate">{events[selectedEventIndex + 1].dayOrDate}</div>
                    <div class="text-zinc-400 truncate text-[9px]">{events[selectedEventIndex + 1].time} • {events[selectedEventIndex + 1].title}</div>
                  {:else}
                    <div class="text-zinc-500 italic">Fim da agenda</div>
                  {/if}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Rodapé com Botões de Ação -->
      <div class="h-16 px-6 border-t border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-900/90">
        <span class="text-xs text-neutral-500 hidden sm:inline">
          Dica: Use <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 font-mono text-[10px]">Ctrl</kbd> + <kbd class="px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 font-mono text-[10px]">Enter</kbd> para salvar
        </span>

        <div class="flex items-center gap-2.5 ml-auto">
          <Button variant="ghost" size="sm" onclick={onclose} class="text-neutral-400 hover:text-white text-xs">
            Cancelar
          </Button>
          <Button
            variant="default"
            size="sm"
            onclick={handleSave}
            class="bg-amber-500 hover:bg-amber-600 text-neutral-950 font-bold text-xs px-4"
          >
            <Check size={14} class="mr-1.5" />
            <span>Salvar Agenda na Setlist</span>
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}

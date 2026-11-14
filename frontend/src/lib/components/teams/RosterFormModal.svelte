<script lang="ts">
  import type { ServiceRoster, UpsertServiceRoster } from '$lib/types';
  import { saveRoster } from '$lib/teams';
  import { notifySuccess } from '$lib/toast';
  import TeamsSubDialog from './TeamsSubDialog.svelte';

  interface Props {
    open: boolean;
    roster?: ServiceRoster | null;
    onclose: () => void;
    onSaved: (roster: ServiceRoster, isNew: boolean) => void;
  }

  let { open, roster = null, onclose, onSaved }: Props = $props();

  const inputClass =
    'w-full h-9 px-3 rounded-lg border border-border bg-background text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50';

  function today(): string {
    const now = new Date();
    const month = `${now.getMonth() + 1}`.padStart(2, '0');
    const day = `${now.getDate()}`.padStart(2, '0');
    return `${now.getFullYear()}-${month}-${day}`;
  }

  let title = $state('');
  let serviceDate = $state('');
  let serviceTime = $state('19:00');
  let notes = $state('');
  let isSaving = $state(false);
  let errorMessage = $state('');

  let wasOpen = false;
  $effect(() => {
    const isOpen = open;
    if (isOpen === wasOpen) return;
    wasOpen = isOpen;
    if (!isOpen) return;
    title = roster?.title ?? 'Culto de Domingo';
    serviceDate = roster?.service_date ?? today();
    serviceTime = roster?.service_time ?? '19:00';
    notes = roster?.notes ?? '';
    isSaving = false;
    errorMessage = '';
  });

  const isValid = $derived(title.trim().length > 0 && serviceDate.length > 0);

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!isValid || isSaving) return;
    isSaving = true;
    errorMessage = '';
    const isNew = !roster?.id;
    try {
      const payload: UpsertServiceRoster = {
        title: title.trim(),
        service_date: serviceDate,
        service_time: serviceTime || null,
        notes: notes.trim() || null,
        is_active: roster?.is_active ?? null,
      };
      const saved = await saveRoster(roster?.id ?? null, payload);
      notifySuccess(isNew ? 'Escala criada' : 'Escala atualizada', saved.title);
      onSaved(saved, isNew);
    } catch (err) {
      errorMessage =
        err instanceof Error ? err.message : 'Não foi possível salvar a escala. Tente novamente.';
    } finally {
      isSaving = false;
    }
  }
</script>

<TeamsSubDialog
  {open}
  title={roster ? 'Editar Escala do Culto' : 'Nova Escala de Culto'}
  description="Defina o culto, a data e o horário usados na escala da equipe de mídia."
  {onclose}
>
  <form id="roster-form" class="space-y-3" onsubmit={handleSubmit}>
    {#if errorMessage}
      <p
        role="alert"
        class="text-xs text-red-700 dark:text-red-300 bg-red-500/10 border border-red-500/35 rounded-lg px-3 py-2"
      >
        {errorMessage}
      </p>
    {/if}

    <div>
      <label for="roster-title" class="block text-xs font-semibold text-foreground mb-1">
        Nome do Culto / Evento <span class="text-primary">*</span>
      </label>
      <input
        id="roster-title"
        type="text"
        bind:value={title}
        placeholder="Ex.: Culto de Domingo - Noite"
        class={inputClass}
      />
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <div>
        <label for="roster-date" class="block text-xs font-semibold text-foreground mb-1">
          Data <span class="text-primary">*</span>
        </label>
        <input id="roster-date" type="date" bind:value={serviceDate} class={inputClass} />
      </div>
      <div>
        <label for="roster-time" class="block text-xs font-semibold text-foreground mb-1">
          Horário
        </label>
        <input id="roster-time" type="time" bind:value={serviceTime} class={inputClass} />
      </div>
    </div>

    <div>
      <label for="roster-notes" class="block text-xs font-semibold text-foreground mb-1">
        Observações (opcional)
      </label>
      <textarea
        id="roster-notes"
        bind:value={notes}
        rows="2"
        placeholder="Ex.: Chegar 40 minutos antes para alinhamento..."
        class="w-full p-2.5 rounded-lg border border-border bg-background text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50 resize-none"
      ></textarea>
    </div>
  </form>

  {#snippet footer()}
    <button
      type="button"
      onclick={onclose}
      disabled={isSaving}
      class="h-9 px-3 rounded-lg border border-border bg-background hover:bg-muted text-xs font-medium text-foreground transition-colors cursor-pointer disabled:opacity-50"
    >
      Cancelar
    </button>
    <button
      type="submit"
      form="roster-form"
      disabled={!isValid || isSaving}
      class="h-9 px-3.5 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer disabled:opacity-50 disabled:pointer-events-none"
    >
      {isSaving ? 'Salvando...' : roster ? 'Salvar Escala' : 'Criar Escala'}
    </button>
  {/snippet}
</TeamsSubDialog>

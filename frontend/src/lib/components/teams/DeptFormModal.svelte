<script lang="ts">
  import type { TeamDepartment, UpsertTeamDepartment } from '$lib/types';
  import { saveDepartment } from '$lib/teams';
  import { notifySuccess } from '$lib/toast';
  import TeamsSubDialog from './TeamsSubDialog.svelte';
  import { PRESET_COLORS, PRESET_ICONS } from './teamConstants';

  interface Props {
    open: boolean;
    department?: TeamDepartment | null;
    onclose: () => void;
    onSaved: () => void;
  }

  let { open, department = null, onclose, onSaved }: Props = $props();

  const inputClass =
    'w-full h-9 px-3 rounded-lg border border-border bg-background text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50';

  let name = $state('');
  let color = $state('#3b82f6');
  let icon = $state('Users');
  let description = $state('');
  let isSaving = $state(false);
  let errorMessage = $state('');

  let wasOpen = false;
  $effect(() => {
    const isOpen = open;
    if (isOpen === wasOpen) return;
    wasOpen = isOpen;
    if (!isOpen) return;
    name = department?.name ?? '';
    color = department?.color ?? '#3b82f6';
    icon = department?.icon ?? 'Users';
    description = department?.description ?? '';
    isSaving = false;
    errorMessage = '';
  });

  const isValid = $derived(name.trim().length > 0);
  const selectedIcon = $derived(PRESET_ICONS.find((item) => item.name === icon)?.label ?? 'Equipe');

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!isValid || isSaving) return;
    isSaving = true;
    errorMessage = '';
    try {
      const payload: UpsertTeamDepartment = {
        name: name.trim(),
        color,
        icon,
        description: description.trim() || null,
        display_order: department?.display_order ?? null,
      };
      await saveDepartment(department?.id ?? '', payload);
      notifySuccess(
        department ? 'Departamento atualizado' : 'Departamento criado',
        name.trim()
      );
      onSaved();
    } catch (err) {
      errorMessage =
        err instanceof Error
          ? err.message
          : 'Não foi possível salvar o departamento. Tente novamente.';
    } finally {
      isSaving = false;
    }
  }
</script>

<TeamsSubDialog
  {open}
  title={department ? 'Editar Departamento' : 'Novo Departamento'}
  description="Nome, cor de identificação e ícone usados nas escalas e nos cards."
  {onclose}
>
  <form id="dept-form" class="space-y-3" onsubmit={handleSubmit}>
    {#if errorMessage}
      <p
        role="alert"
        class="text-xs text-red-700 dark:text-red-300 bg-red-500/10 border border-red-500/35 rounded-lg px-3 py-2"
      >
        {errorMessage}
      </p>
    {/if}

    <div>
      <label for="dept-name" class="block text-xs font-semibold text-foreground mb-1">
        Nome do Departamento <span class="text-primary">*</span>
      </label>
      <input
        id="dept-name"
        type="text"
        bind:value={name}
        placeholder="Ex.: Fotografia, Stories & Redes..."
        class={inputClass}
      />
    </div>

    <div>
      <span class="block text-xs font-semibold text-foreground mb-1.5">Cor de Destaque</span>
      <div class="flex items-center gap-2 flex-wrap">
        {#each PRESET_COLORS as preset}
          <button
            type="button"
            aria-label="Selecionar cor {preset}"
            aria-pressed={color === preset}
            onclick={() => (color = preset)}
            class="w-8 h-8 rounded-full border-2 transition-transform cursor-pointer {color === preset
              ? 'scale-110 border-foreground shadow-md'
              : 'border-transparent hover:scale-105'}"
            style="background-color: {preset};"
          ></button>
        {/each}
        <label class="inline-flex items-center gap-2 text-[11px] text-muted-foreground">
          <span class="sr-only">Cor personalizada</span>
          <input
            type="color"
            bind:value={color}
            aria-label="Cor personalizada do departamento"
            class="w-8 h-8 rounded-full overflow-hidden cursor-pointer bg-transparent border border-border"
          />
          <span class="font-mono uppercase">{color}</span>
        </label>
      </div>
    </div>

    <fieldset>
      <legend class="block text-xs font-semibold text-foreground mb-1.5">
        Ícone — {selectedIcon}
      </legend>
      <div class="grid grid-cols-5 gap-1.5">
        {#each PRESET_ICONS as preset}
          {@const IconComponent = preset.icon}
          <button
            type="button"
            aria-pressed={icon === preset.name}
            aria-label="Selecionar ícone {preset.label}"
            onclick={() => (icon = preset.name)}
            class="min-h-[52px] p-1.5 rounded-lg border text-[11px] flex flex-col items-center justify-center gap-1 transition-colors cursor-pointer {icon ===
            preset.name
              ? 'bg-primary/10 border-primary/50 text-primary'
              : 'bg-background border-border text-muted-foreground hover:text-foreground hover:bg-muted'}"
          >
            <IconComponent size={15} />
            <span class="truncate max-w-full leading-none">{preset.label}</span>
          </button>
        {/each}
      </div>
    </fieldset>

    <div>
      <label for="dept-desc" class="block text-xs font-semibold text-foreground mb-1">
        Descrição (opcional)
      </label>
      <input
        id="dept-desc"
        type="text"
        bind:value={description}
        placeholder="Ex.: Responsável pelas fotos da celebração..."
        class={inputClass}
      />
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
      form="dept-form"
      disabled={!isValid || isSaving}
      class="h-9 px-3.5 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer disabled:opacity-50 disabled:pointer-events-none"
    >
      {isSaving ? 'Salvando...' : 'Salvar Departamento'}
    </button>
  {/snippet}
</TeamsSubDialog>

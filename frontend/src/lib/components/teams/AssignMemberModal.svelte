<script lang="ts">
  import { Crown, Shield, Users } from 'lucide-svelte';
  import type {
    RosterRole,
    ServiceRoster,
    TeamDepartment,
    TeamMember,
    UpsertServiceRosterItem,
  } from '$lib/types';
  import { addRosterItem } from '$lib/teams';
  import { notifySuccess } from '$lib/toast';
  import TeamsSubDialog from './TeamsSubDialog.svelte';

  interface Props {
    open: boolean;
    departments: TeamDepartment[];
    members: TeamMember[];
    roster: ServiceRoster | null;
    defaultDeptId?: string;
    onclose: () => void;
    onSaved: () => void;
  }

  let { open, departments, members, roster, defaultDeptId = '', onclose, onSaved }: Props = $props();

  const inputClass =
    'w-full h-9 px-3 rounded-lg border border-border bg-background text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-ring/50';

  const ROLES: { value: RosterRole; label: string; hint: string; icon: typeof Users }[] = [
    { value: 'lead', label: 'Titular', hint: 'Responsável pelo departamento', icon: Crown },
    { value: 'assistant', label: 'Auxiliar', hint: 'Apoia a operação do culto', icon: Users },
    { value: 'backup', label: 'Apoio', hint: 'Reserva para imprevistos', icon: Shield },
  ];

  let deptId = $state('');
  let memberId = $state('');
  let role = $state<RosterRole>('lead');
  let notes = $state('');
  let isSaving = $state(false);
  let errorMessage = $state('');

  let wasOpen = false;
  $effect(() => {
    const isOpen = open;
    if (isOpen === wasOpen) return;
    wasOpen = isOpen;
    if (!isOpen) return;
    deptId = defaultDeptId || (departments[0]?.id ?? '');
    memberId = '';
    role = 'lead';
    notes = '';
    isSaving = false;
    errorMessage = '';
  });

  const activeMembers = $derived(members.filter((m) => m.is_active));
  const selectedMember = $derived(activeMembers.find((m) => m.id === memberId) ?? null);
  const memberBelongsToDept = $derived(
    !!selectedMember && (selectedMember.department_ids || []).includes(deptId)
  );
  const alreadyAssigned = $derived(
    !!roster &&
      !!memberId &&
      roster.items.some((item) => item.department_id === deptId && item.member_id === memberId)
  );

  const isValid = $derived(!!deptId && !!memberId && !alreadyAssigned);

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!isValid || !roster || isSaving) return;
    isSaving = true;
    errorMessage = '';
    try {
      const payload: UpsertServiceRosterItem = {
        department_id: deptId,
        member_id: memberId,
        role,
        status: 'pending',
        role_notes: notes.trim() || null,
      };
      await addRosterItem(roster.id, payload);
      notifySuccess('Voluntário escalado', 'Presença registrada como pendente.');
      onSaved();
    } catch (err) {
      errorMessage =
        err instanceof Error ? err.message : 'Não foi possível escalar o voluntário. Tente novamente.';
    } finally {
      isSaving = false;
    }
  }
</script>

<TeamsSubDialog
  {open}
  title="Escalar Voluntário"
  description="Defina departamento, função e observações para este culto."
  {onclose}
>
  <form id="assign-form" class="space-y-3" onsubmit={handleSubmit}>
    {#if errorMessage}
      <p
        role="alert"
        class="text-xs text-red-700 dark:text-red-300 bg-red-500/10 border border-red-500/35 rounded-lg px-3 py-2"
      >
        {errorMessage}
      </p>
    {/if}

    <div>
      <label for="assign-dept" class="block text-xs font-semibold text-foreground mb-1">
        Departamento <span class="text-primary">*</span>
      </label>
      <select id="assign-dept" bind:value={deptId} class="{inputClass} cursor-pointer">
        {#each departments as dept}
          <option value={dept.id}>{dept.name}</option>
        {/each}
      </select>
    </div>

    <div>
      <label for="assign-member" class="block text-xs font-semibold text-foreground mb-1">
        Voluntário <span class="text-primary">*</span>
      </label>
      <select id="assign-member" bind:value={memberId} class="{inputClass} cursor-pointer">
        <option value="">Selecione um voluntário...</option>
        {#each activeMembers as member}
          <option value={member.id}>
            {member.name}{(member.department_ids || []).includes(deptId) ? ' — do departamento' : ''}
          </option>
        {/each}
      </select>

      {#if selectedMember}
        <p class="text-[11px] text-muted-foreground mt-1">
          {memberBelongsToDept
            ? 'Este voluntário atua neste departamento.'
            : 'Este voluntário não está vinculado a este departamento — a escala é permitida mesmo assim.'}
        </p>
      {/if}

      {#if alreadyAssigned}
        <p
          role="alert"
          class="text-[11px] text-amber-700 dark:text-amber-300 bg-amber-500/10 border border-amber-500/35 rounded-lg px-2.5 py-1.5 mt-2"
        >
          Este voluntário já está escalado neste departamento.
        </p>
      {/if}
    </div>

    <fieldset>
      <legend class="block text-xs font-semibold text-foreground mb-1.5">
        Função na Escala
      </legend>
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
        {#each ROLES as option}
          {@const RoleIcon = option.icon}
          <button
            type="button"
            aria-pressed={role === option.value}
            onclick={() => (role = option.value)}
            class="min-h-[52px] px-2 py-2 rounded-lg border text-xs font-semibold flex flex-col items-center justify-center gap-1 transition-colors cursor-pointer {role ===
            option.value
              ? 'bg-primary/10 border-primary/50 text-primary'
              : 'bg-background border-border text-muted-foreground hover:text-foreground hover:bg-muted'}"
          >
            <RoleIcon size={14} />
            <span>{option.label}</span>
            <span class="sr-only">{option.hint}</span>
          </button>
        {/each}
      </div>
      <p class="text-[11px] text-muted-foreground mt-1.5">
        {ROLES.find((option) => option.value === role)?.hint}
      </p>
    </fieldset>

    <div>
      <label for="assign-notes" class="block text-xs font-semibold text-foreground mb-1">
        Observações (opcional)
      </label>
      <input
        id="assign-notes"
        type="text"
        bind:value={notes}
        placeholder="Ex.: Levar câmera reserva..."
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
      form="assign-form"
      disabled={!isValid || isSaving}
      class="h-9 px-3.5 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer disabled:opacity-50 disabled:pointer-events-none"
    >
      {isSaving ? 'Escalando...' : 'Confirmar Escala'}
    </button>
  {/snippet}
</TeamsSubDialog>

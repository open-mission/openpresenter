<script lang="ts">
  import { Crown, Lock, MonitorPlay, Shield, User } from 'lucide-svelte';
  import type { TeamDepartment, TeamMember, TeamMemberRole, UpsertTeamMember } from '$lib/types';
  import { saveMember } from '$lib/teams';
  import { notifySuccess } from '$lib/toast';
  import TeamsSubDialog from './TeamsSubDialog.svelte';

  interface Props {
    open: boolean;
    member?: TeamMember | null;
    departments: TeamDepartment[];
    onclose: () => void;
    onSaved: () => void;
  }

  let { open, member = null, departments, onclose, onSaved }: Props = $props();

  const inputClass =
    'w-full h-9 px-3 rounded-lg border border-border bg-background text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50';

  const MEMBER_ROLES: { value: TeamMemberRole; label: string; icon: typeof Crown }[] = [
    { value: 'leader', label: 'Líder', icon: Crown },
    { value: 'vice_leader', label: 'Vice-Líder', icon: Shield },
    { value: 'member', label: 'Membro', icon: User },
  ];

  let name = $state('');
  let phone = $state('');
  let role = $state<TeamMemberRole>('member');
  let notes = $state('');
  let departmentIds = $state<string[]>([]);
  let canControlRemote = $state(true);
  let isActive = $state(true);
  let pin = $state('');
  let isSaving = $state(false);
  let errorMessage = $state('');

  let wasOpen = false;
  $effect(() => {
    const isOpen = open;
    if (isOpen === wasOpen) return;
    wasOpen = isOpen;
    if (!isOpen) return;
    name = member?.name ?? '';
    phone = member?.phone ?? '';
    role = (member?.role as TeamMemberRole) ?? 'member';
    notes = member?.notes ?? '';
    departmentIds = [...(member?.department_ids ?? [])];
    canControlRemote = member?.can_control_remote ?? false;
    isActive = member?.is_active ?? true;
    pin = member?.pin ?? '';
    isSaving = false;
    errorMessage = '';
  });

  const isValid = $derived(name.trim().length > 0 && pin.length <= 8);

  function toggleDepartment(id: string, checked: boolean) {
    departmentIds = checked
      ? [...new Set([...departmentIds, id])]
      : departmentIds.filter((deptId) => deptId !== id);
  }

  function handlePinInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const digits = input.value.replace(/\D/g, '').slice(0, 8);
    pin = digits;
    input.value = digits;
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!isValid || isSaving) return;
    isSaving = true;
    errorMessage = '';
    try {
      const payload: UpsertTeamMember = {
        name: name.trim(),
        phone: phone.trim() || null,
        role,
        can_control_remote: canControlRemote,
        pin: pin || null,
        notes: notes.trim() || null,
        department_ids: departmentIds,
        is_active: isActive,
      };
      await saveMember(member?.id ?? null, payload);
      notifySuccess(member ? 'Voluntário atualizado' : 'Voluntário cadastrado', name.trim());
      onSaved();
    } catch (err) {
      errorMessage =
        err instanceof Error ? err.message : 'Não foi possível salvar o voluntário. Tente novamente.';
    } finally {
      isSaving = false;
    }
  }
</script>

<TeamsSubDialog
  {open}
  size="lg"
  title={member ? 'Editar Voluntário' : 'Novo Voluntário'}
  description="Dados pessoais, cargo, departamentos e permissões de acesso ao controle remoto."
  {onclose}
>
  <form id="member-form" class="space-y-4" onsubmit={handleSubmit}>
    {#if errorMessage}
      <p
        role="alert"
        class="text-xs text-red-700 dark:text-red-300 bg-red-500/10 border border-red-500/35 rounded-lg px-3 py-2"
      >
        {errorMessage}
      </p>
    {/if}

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-start">
      <div class="space-y-3">
        <div>
          <label for="member-name" class="block text-xs font-semibold text-foreground mb-1">
            Nome Completo <span class="text-primary">*</span>
          </label>
          <input
            id="member-name"
            type="text"
            bind:value={name}
            placeholder="Ex.: Claudio Silva"
            class={inputClass}
          />
        </div>

        <div>
          <label for="member-phone" class="block text-xs font-semibold text-foreground mb-1">
            WhatsApp / Telefone (com DDD)
          </label>
          <input
            id="member-phone"
            type="tel"
            bind:value={phone}
            placeholder="Ex.: (11) 98765-4321"
            class={inputClass}
          />
        </div>

        <fieldset>
          <legend class="block text-xs font-semibold text-foreground mb-1.5">Cargo na Equipe</legend>
          <div class="grid grid-cols-3 gap-2">
            {#each MEMBER_ROLES as option}
              {@const RoleIcon = option.icon}
              <button
                type="button"
                aria-pressed={role === option.value}
                onclick={() => (role = option.value)}
                class="h-9 px-2 rounded-lg border text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer {role ===
                option.value
                  ? 'bg-primary/10 border-primary/50 text-primary'
                  : 'bg-background border-border text-muted-foreground hover:text-foreground hover:bg-muted'}"
              >
                <RoleIcon size={13} />
                <span>{option.label}</span>
              </button>
            {/each}
          </div>
        </fieldset>

        <div>
          <label for="member-notes" class="block text-xs font-semibold text-foreground mb-1">
            Observações (opcional)
          </label>
          <textarea
            id="member-notes"
            bind:value={notes}
            rows="3"
            placeholder="Ex.: Disponível aos domingos à noite..."
            class="w-full p-2.5 rounded-lg border border-border bg-background text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50 resize-none"
          ></textarea>
        </div>
      </div>

      <div class="space-y-3">
        <fieldset>
          <legend class="block text-xs font-semibold text-foreground mb-1.5">
            Departamentos em que atua
          </legend>
          <div
            class="grid grid-cols-1 sm:grid-cols-2 gap-1.5 max-h-40 overflow-y-auto p-1.5 rounded-lg bg-muted/40 border border-border"
          >
            {#each departments as dept (dept.id)}
              <label
                class="flex items-center gap-2 p-1.5 rounded-md hover:bg-muted cursor-pointer select-none text-[11px] text-foreground"
              >
                <input
                  type="checkbox"
                  checked={departmentIds.includes(dept.id)}
                  onchange={(e) => toggleDepartment(dept.id, e.currentTarget.checked)}
                  class="rounded accent-primary"
                />
                <span
                  class="w-2 h-2 rounded-full shrink-0"
                  style="background-color: {dept.color};"
                ></span>
                <span class="truncate">{dept.name}</span>
              </label>
            {/each}
            {#if departments.length === 0}
              <p class="text-[11px] text-muted-foreground p-1.5">
                Nenhum departamento cadastrado.
              </p>
            {/if}
          </div>
        </fieldset>

        <div class="p-3 rounded-xl bg-muted/40 border border-border space-y-3">
          <div class="flex items-start justify-between gap-3">
            <div class="space-y-0.5 min-w-0">
              <div class="flex items-center gap-1.5">
                <MonitorPlay
                  size={14}
                  class={canControlRemote ? 'text-emerald-700 dark:text-emerald-300' : 'text-muted-foreground'}
                />
                <span class="text-xs font-bold text-foreground">Controle Remoto (/remote)</span>
              </div>
              <p class="text-[11px] text-muted-foreground leading-relaxed">
                Permite disparar slides, músicas e bíblia pelo celular.
              </p>
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={canControlRemote}
              aria-label="Permitir controle remoto"
              onclick={() => (canControlRemote = !canControlRemote)}
              class="relative inline-flex h-6 w-11 shrink-0 items-center rounded-full border transition-colors cursor-pointer mt-0.5 {canControlRemote
                ? 'bg-primary border-primary'
                : 'bg-muted border-border'}"
            >
              <span
                class="inline-block h-4 w-4 rounded-full bg-background shadow transition-transform {canControlRemote
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </div>

          <p
            class="inline-flex items-center gap-1 text-[11px] font-medium px-2 py-1 rounded-md border {canControlRemote
              ? 'text-emerald-700 dark:text-emerald-300 bg-emerald-500/10 border-emerald-500/35'
              : 'text-muted-foreground bg-background border-border'}"
          >
            {#if canControlRemote}
              <MonitorPlay size={11} />
              <span>Operador de projeção liberado</span>
            {:else}
              <Lock size={11} />
              <span>Apenas visualização da escala</span>
            {/if}
          </p>

          <div class="pt-2 border-t border-border">
            <label for="member-pin" class="block text-xs font-semibold text-foreground mb-1">
              PIN de Acesso Pessoal (opcional, até 8 dígitos)
            </label>
            <input
              id="member-pin"
              type="text"
              inputmode="numeric"
              autocomplete="off"
              maxlength="8"
              value={pin}
              oninput={handlePinInput}
              placeholder="Sem PIN (acesso com 1 toque)"
              class="w-full h-9 px-2.5 rounded-lg border border-border bg-background text-sm font-mono tracking-widest text-foreground placeholder:text-muted-foreground placeholder:tracking-normal focus:outline-none focus:ring-2 focus:ring-ring/50"
            />
            <p class="text-[11px] text-muted-foreground mt-1">
              Protege o perfil deste voluntário no login do <code>/remote</code>.
            </p>
          </div>
        </div>

        <div class="flex items-center justify-between gap-3 p-3 rounded-xl bg-muted/40 border border-border">
          <div class="min-w-0">
            <span class="block text-xs font-bold text-foreground">Voluntário ativo</span>
            <p class="text-[11px] text-muted-foreground">
              Inativos ficam ocultos das novas escalas.
            </p>
          </div>
          <button
            type="button"
            role="switch"
            aria-checked={isActive}
            aria-label="Voluntário ativo no cadastro"
            onclick={() => (isActive = !isActive)}
            class="relative inline-flex h-6 w-11 shrink-0 items-center rounded-full border transition-colors cursor-pointer {isActive
              ? 'bg-primary border-primary'
              : 'bg-muted border-border'}"
          >
            <span
              class="inline-block h-4 w-4 rounded-full bg-background shadow transition-transform {isActive
                ? 'translate-x-6'
                : 'translate-x-1'}"
            ></span>
          </button>
        </div>
      </div>
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
      form="member-form"
      disabled={!isValid || isSaving}
      class="h-9 px-3.5 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer disabled:opacity-50 disabled:pointer-events-none"
    >
      {isSaving ? 'Salvando...' : 'Salvar Voluntário'}
    </button>
  {/snippet}
</TeamsSubDialog>

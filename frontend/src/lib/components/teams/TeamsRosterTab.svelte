<script lang="ts">
  import { onDestroy } from 'svelte';
  import {
    Calendar,
    Check,
    CheckCircle2,
    Crown,
    Edit2,
    LayoutGrid,
    MessageCircle,
    Plus,
    Trash2,
    Workflow,
  } from 'lucide-svelte';
  import type {
    RosterStatus,
    ServiceRoster,
    ServiceRosterSummary,
    TeamDepartment,
    TeamMember,
  } from '$lib/types';
  import {
    formatRosterRoleLabel,
    formatRosterWhatsApp,
    getWhatsAppUrl,
    isRosterLeadRole,
  } from '$lib/teams';
  import { notifyError, notifySuccess } from '$lib/toast';
  import RosterFlowCanvas from './RosterFlowCanvas.svelte';
  import {
    getContrastTextColor,
    getDeptIconComponent,
    getInitials,
    getNextRosterStatus,
    getRosterStatusMeta,
  } from './teamConstants';

  interface Props {
    rosterSummaries: ServiceRosterSummary[];
    selectedRosterId: string | null;
    roster: ServiceRoster | null;
    departments: TeamDepartment[];
    members: TeamMember[];
    isLoading: boolean;
    errorMessage: string;
    viewMode: 'flow' | 'grid';
    onSelectRoster: (id: string) => void;
    onCreateRoster: () => void;
    onEditRoster: () => void;
    onAssign: (deptId?: string) => void;
    onActivateRoster: () => void | Promise<void>;
    onDeleteRoster: () => void;
    onRemoveItem: (itemId: string) => void;
    onUpdateItemStatus: (itemId: string, status: RosterStatus) => void | Promise<void>;
    onSetViewMode: (mode: 'flow' | 'grid') => void;
    onAddMemberToDept: (deptId: string) => void;
    onRetry: () => void;
  }

  let {
    rosterSummaries,
    selectedRosterId,
    roster,
    departments,
    members,
    isLoading,
    errorMessage,
    viewMode,
    onSelectRoster,
    onCreateRoster,
    onEditRoster,
    onAssign,
    onActivateRoster,
    onDeleteRoster,
    onRemoveItem,
    onUpdateItemStatus,
    onSetViewMode,
    onAddMemberToDept,
    onRetry,
  }: Props = $props();

  const memberMap = $derived.by(() => {
    const map = new Map<string, TeamMember>();
    for (const member of members) map.set(member.id, member);
    return map;
  });

  const counts = $derived.by(() => {
    const base = { confirmed: 0, pending: 0, declined: 0 };
    if (!roster) return base;
    for (const item of roster.items) {
      if (item.status === 'confirmed') base.confirmed += 1;
      else if (item.status === 'declined') base.declined += 1;
      else base.pending += 1;
    }
    return base;
  });

  let copied = $state(false);
  let copyTimer: ReturnType<typeof setTimeout> | null = null;

  onDestroy(() => {
    if (copyTimer) clearTimeout(copyTimer);
  });

  async function handleCopyWhatsApp() {
    if (!roster) return;
    try {
      const text = formatRosterWhatsApp(roster, departments, members);
      await navigator.clipboard.writeText(text);
      copied = true;
      if (copyTimer) clearTimeout(copyTimer);
      copyTimer = setTimeout(() => {
        copied = false;
      }, 2500);
      notifySuccess('Escala copiada', 'Cole a mensagem no grupo do WhatsApp.');
    } catch {
      notifyError(
        'Não foi possível copiar',
        'Seu navegador bloqueou o acesso à área de transferência.'
      );
    }
  }

  function itemsForDept(deptId: string) {
    return roster?.items.filter((item) => item.department_id === deptId) ?? [];
  }

  function whatsappFor(member: TeamMember | undefined, deptName: string): string {
    if (!member || !roster) return '';
    return getWhatsAppUrl(
      member.phone,
      `Olá ${member.name}, confirmando sua escala no culto de ${roster.service_date} em ${deptName}!`
    );
  }
</script>

<div class="flex flex-col flex-1 min-h-0">
  <div
    class="px-4 sm:px-6 py-3 border-b border-border bg-muted/30 flex flex-wrap items-center justify-between gap-2 shrink-0"
  >
    <div class="flex flex-wrap items-center gap-2 min-w-0">
      <label for="roster-select" class="text-xs font-semibold text-foreground">Escala</label>
      <select
        id="roster-select"
        aria-label="Selecionar escala do culto"
        class="h-9 px-2.5 min-w-[190px] max-w-[260px] rounded-lg border border-border bg-background text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-ring/50 cursor-pointer"
        value={selectedRosterId ?? ''}
        onchange={(e) => onSelectRoster(e.currentTarget.value)}
      >
        {#if rosterSummaries.length === 0}
          <option value="">Nenhuma escala cadastrada</option>
        {:else}
          {#each rosterSummaries as summary (summary.id)}
            <option value={summary.id}>
              {summary.title} ({summary.service_date}) {summary.is_active ? '★ Ativa' : ''}
            </option>
          {/each}
        {/if}
      </select>

      <button
        type="button"
        onclick={onCreateRoster}
        class="inline-flex items-center gap-1.5 h-9 px-3 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer"
      >
        <Plus size={14} />
        <span>Nova Escala</span>
      </button>

      {#if roster}
        <button
          type="button"
          onclick={onEditRoster}
          class="inline-flex items-center gap-1.5 h-9 px-3 rounded-lg border border-border bg-background hover:bg-muted text-xs font-semibold text-foreground transition-colors cursor-pointer"
        >
          <Edit2 size={14} />
          <span>Editar Escala</span>
        </button>

        <button
          type="button"
          onclick={() => onAssign()}
          class="inline-flex items-center gap-1.5 h-9 px-3 rounded-lg border border-border bg-background hover:bg-muted text-xs font-semibold text-foreground transition-colors cursor-pointer"
        >
          <Plus size={14} />
          <span>Escalar Voluntário</span>
        </button>
      {/if}
    </div>

    {#if roster}
      <div class="flex flex-wrap items-center gap-2">
        {#if roster.is_active}
          <span
            class="inline-flex items-center gap-1.5 min-h-[32px] px-2.5 rounded-lg bg-emerald-500/10 border border-emerald-500/35 text-emerald-700 dark:text-emerald-300 text-xs font-semibold"
          >
            <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
            Escala Ativa
          </span>
        {:else}
          <button
            type="button"
            onclick={onActivateRoster}
            class="inline-flex items-center gap-1.5 min-h-[32px] px-2.5 rounded-lg border border-border bg-background hover:bg-emerald-500/10 hover:border-emerald-500/35 text-xs font-semibold text-foreground transition-colors cursor-pointer"
          >
            <CheckCircle2 size={13} />
            <span>Tornar Ativa</span>
          </button>
        {/if}

        <button
          type="button"
          onclick={handleCopyWhatsApp}
          class="inline-flex items-center gap-1.5 min-h-[32px] px-2.5 rounded-lg border border-border bg-background hover:bg-muted text-xs font-semibold text-foreground transition-colors cursor-pointer"
          title="Copiar escala formatada para colar no grupo do WhatsApp"
        >
          {#if copied}
            <Check size={13} class="text-emerald-700 dark:text-emerald-300" />
            <span>Copiado!</span>
          {:else}
            <MessageCircle size={13} />
            <span>Copiar WhatsApp</span>
          {/if}
        </button>

        <div
          class="flex items-center p-0.5 rounded-lg bg-muted border border-border"
          role="group"
          aria-label="Modo de visualização da escala"
        >
          <button
            type="button"
            aria-pressed={viewMode === 'flow'}
            aria-label="Visualização em fluxo"
            onclick={() => onSetViewMode('flow')}
            class="w-8 h-8 rounded-md flex items-center justify-center transition-colors cursor-pointer {viewMode ===
            'flow'
              ? 'bg-card text-foreground shadow-sm'
              : 'text-muted-foreground hover:text-foreground'}"
          >
            <Workflow size={15} />
          </button>
          <button
            type="button"
            aria-pressed={viewMode === 'grid'}
            aria-label="Visualização em grade de cartões"
            onclick={() => onSetViewMode('grid')}
            class="w-8 h-8 rounded-md flex items-center justify-center transition-colors cursor-pointer {viewMode ===
            'grid'
              ? 'bg-card text-foreground shadow-sm'
              : 'text-muted-foreground hover:text-foreground'}"
          >
            <LayoutGrid size={15} />
          </button>
        </div>

        <button
          type="button"
          onclick={onDeleteRoster}
          aria-label="Excluir esta escala"
          title="Excluir esta escala"
          class="w-8 h-8 rounded-lg border border-border bg-background hover:bg-red-500/10 hover:border-red-500/35 hover:text-red-700 dark:hover:text-red-300 text-muted-foreground flex items-center justify-center transition-colors cursor-pointer"
        >
          <Trash2 size={15} />
        </button>
      </div>
    {/if}
  </div>

  {#if errorMessage}
    <div class="px-4 sm:px-6 pt-3 shrink-0">
      <p
        role="alert"
        class="flex flex-wrap items-center gap-2 text-xs text-red-700 dark:text-red-300 bg-red-500/10 border border-red-500/35 rounded-lg px-3 py-2"
      >
        <span class="flex-1 min-w-[180px]">{errorMessage}</span>
        <button
          type="button"
          onclick={onRetry}
          class="shrink-0 min-h-[26px] px-2.5 py-1 rounded-md border border-red-500/35 bg-background/60 text-xs font-semibold text-red-700 dark:text-red-300 hover:bg-red-500/15 transition-colors cursor-pointer"
        >
          Tentar novamente
        </button>
      </p>
    </div>
  {/if}

  {#if roster}
    <div
      class="px-4 sm:px-6 py-2 border-b border-border flex flex-wrap items-center gap-2 text-[11px] shrink-0"
    >
      <span
        class="inline-flex items-center gap-1.5 px-2 py-1 rounded-md border border-emerald-500/35 bg-emerald-500/10 text-emerald-700 dark:text-emerald-300 font-semibold"
      >
        <CheckCircle2 size={11} />
        {counts.confirmed} confirmado{counts.confirmed === 1 ? '' : 's'}
      </span>
      <span
        class="inline-flex items-center gap-1.5 px-2 py-1 rounded-md border border-amber-500/35 bg-amber-500/10 text-amber-700 dark:text-amber-300 font-semibold"
      >
        <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
        {counts.pending} pendente{counts.pending === 1 ? '' : 's'}
      </span>
      <span
        class="inline-flex items-center gap-1.5 px-2 py-1 rounded-md border border-rose-500/35 bg-rose-500/10 text-rose-700 dark:text-rose-300 font-semibold"
      >
        <span class="w-1.5 h-1.5 rounded-full bg-rose-500"></span>
        {counts.declined} ausente{counts.declined === 1 ? '' : 's'}
      </span>
      <span class="text-muted-foreground ml-auto">
        {roster.items.length} alocaç{roster.items.length === 1 ? 'ão' : 'ões'} no total
      </span>
    </div>
  {/if}

  <div class="flex-1 min-h-0 p-3 sm:p-4 overflow-hidden relative">
    {#if isLoading}
      <div class="h-full min-h-[240px] flex items-center justify-center text-muted-foreground">
        <span class="text-sm">Carregando dados da equipe...</span>
      </div>
    {:else if !roster}
      <div class="h-full min-h-full flex flex-col items-center justify-center text-center p-8">
        <div
          class="w-14 h-14 rounded-2xl bg-muted border border-border flex items-center justify-center text-muted-foreground mb-4"
        >
          <Calendar size={26} />
        </div>
        <h3 class="text-base font-bold text-foreground mb-1">Nenhuma escala selecionada</h3>
        <p class="text-xs text-muted-foreground max-w-sm mb-4">
          Crie uma escala de culto para alocar os voluntários da equipe de mídia por departamento.
        </p>
        <button
          type="button"
          onclick={onCreateRoster}
          class="inline-flex items-center gap-2 h-9 px-3.5 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer"
        >
          <Plus size={15} />
          <span>Criar Primeira Escala</span>
        </button>
      </div>
    {:else if viewMode === 'flow'}
      <RosterFlowCanvas
        roster={roster}
        {departments}
        allMembers={members}
        onUpdateRosterItemStatus={onUpdateItemStatus}
        onRemoveRosterItem={onRemoveItem}
        onAddMemberToDept={onAddMemberToDept}
      />
    {:else}
      <div class="h-full overflow-auto pb-2">
        <div class="flex items-start gap-3 min-w-max">
          {#each departments as dept (dept.id)}
            {@const deptItems = itemsForDept(dept.id)}
            {@const DeptIcon = getDeptIconComponent(dept.icon)}
            <div
              class="w-72 rounded-2xl bg-card border border-border p-3 flex flex-col gap-3 shrink-0"
            >
              <div class="flex items-center justify-between gap-2">
                <div class="flex items-center gap-2 min-w-0">
                  <div
                    class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0"
                    style="background-color: {dept.color}; color: {getContrastTextColor(dept.color)};"
                  >
                    <DeptIcon size={15} />
                  </div>
                  <div class="min-w-0">
                    <h4 class="text-xs font-bold text-foreground truncate">{dept.name}</h4>
                    <span class="text-[11px] text-muted-foreground">
                      {deptItems.length}
                      {deptItems.length === 1 ? 'voluntário' : 'voluntários'}
                    </span>
                  </div>
                </div>

                <button
                  type="button"
                  onclick={() => onAddMemberToDept(dept.id)}
                  aria-label="Escalar voluntário para {dept.name}"
                  title="Escalar voluntário para {dept.name}"
                  class="w-8 h-8 shrink-0 rounded-lg border border-border bg-background hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer"
                >
                  <Plus size={14} />
                </button>
              </div>

              <div class="flex flex-col gap-2 min-h-[80px]">
                {#if deptItems.length === 0}
                  <p
                    class="p-3 rounded-xl border border-dashed border-border text-center text-[11px] text-muted-foreground"
                  >
                    Nenhum voluntário escalado
                  </p>
                {:else}
                  {#each deptItems as item (item.id)}
                    {@const member = memberMap.get(item.member_id)}
                    {@const memberName = member?.name ?? 'Voluntário'}
                    {@const status = getRosterStatusMeta(item.status)}
                    {@const StatusIcon = status.icon}
                    {@const waUrl = whatsappFor(member, dept.name)}
                    <div
                      class="p-2.5 rounded-xl bg-background border border-border flex flex-col gap-2"
                    >
                      <div class="flex items-center justify-between gap-2">
                        <div class="flex items-center gap-2 min-w-0">
                          <div
                            class="w-8 h-8 rounded-full flex items-center justify-center text-[11px] font-bold text-foreground shrink-0 border"
                            style="background-color: {dept.color}22; border-color: {dept.color};"
                          >
                            {getInitials(memberName)}
                          </div>
                          <div class="min-w-0">
                            <div class="flex items-center gap-1">
                              <span class="text-xs font-semibold text-foreground truncate">
                                {memberName}
                              </span>
                              {#if isRosterLeadRole(item.role)}
                                <span
                                  class="text-amber-700 dark:text-amber-300 shrink-0"
                                  title="Titular"
                                >
                                  <Crown size={11} />
                                </span>
                              {/if}
                            </div>
                            <span class="text-[11px] text-muted-foreground">
                              {formatRosterRoleLabel(item.role)}
                            </span>
                          </div>
                        </div>

                        <div class="flex items-center gap-1 shrink-0">
                          {#if waUrl}
                            <a
                              href={waUrl}
                              target="_blank"
                              rel="noopener noreferrer"
                              aria-label="Conversar com {memberName} no WhatsApp"
                              title="WhatsApp"
                              class="w-7 h-7 rounded-md border border-border bg-background hover:bg-emerald-500/10 hover:border-emerald-500/35 text-emerald-700 dark:text-emerald-300 flex items-center justify-center transition-colors"
                            >
                              <MessageCircle size={13} />
                            </a>
                          {/if}
                          <button
                            type="button"
                            onclick={() => onRemoveItem(item.id)}
                            aria-label="Remover {memberName} da escala"
                            title="Remover da escala"
                            class="w-7 h-7 rounded-md border border-border bg-background hover:bg-red-500/10 hover:border-red-500/35 text-muted-foreground hover:text-red-700 dark:hover:text-red-300 flex items-center justify-center transition-colors cursor-pointer"
                          >
                            <Trash2 size={13} />
                          </button>
                        </div>
                      </div>

                      <div class="pt-1.5 border-t border-border">
                        <button
                          type="button"
                          onclick={() => onUpdateItemStatus(item.id, getNextRosterStatus(item.status))}
                          aria-label="Alternar presença de {memberName}: {status.label}"
                          title="Clique para alternar status (Confirmado / Não comparece / Pendente)"
                          class="inline-flex items-center gap-1 min-h-[26px] px-2.5 py-1 rounded-full border text-[11px] font-semibold transition-colors cursor-pointer {status.chipClass}"
                        >
                          <StatusIcon size={11} />
                          <span>{status.label}</span>
                        </button>
                      </div>
                    </div>
                  {/each}
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

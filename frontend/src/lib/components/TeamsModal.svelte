<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import type {
    RosterStatus,
    ServiceRoster,
    ServiceRosterSummary,
    TeamDepartment,
    TeamMember,
  } from '$lib/types';
  import {
    activateRoster,
    fetchDepartments,
    fetchMembers,
    fetchRoster,
    fetchRosters,
    updateRosterItemStatus,
  } from '$lib/teams';
  import { teamUpdatedStore } from '$lib/ws';
  import { notifyError, notifySuccess } from '$lib/toast';
  import { cn } from '$lib/utils';
  import TeamsHeader from './teams/TeamsHeader.svelte';
  import TeamsRosterTab from './teams/TeamsRosterTab.svelte';
  import TeamsMembersTab from './teams/TeamsMembersTab.svelte';
  import TeamsDepartmentsTab from './teams/TeamsDepartmentsTab.svelte';
  import RosterFormModal from './teams/RosterFormModal.svelte';
  import AssignMemberModal from './teams/AssignMemberModal.svelte';
  import MemberFormModal from './teams/MemberFormModal.svelte';
  import DeptFormModal from './teams/DeptFormModal.svelte';
  import TeamsConfirmDialog from './teams/TeamsConfirmDialog.svelte';
  import type { ConfirmTarget, TeamsTabId } from './teams/teamConstants';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open = false, onclose }: Props = $props();

  type SubDialog = 'roster-form' | 'assign' | 'member-form' | 'dept-form' | null;

  let activeTab = $state<TeamsTabId>('roster');
  let visitedTabs = $state<Record<TeamsTabId, boolean>>({
    roster: true,
    members: false,
    departments: false,
  });
  let viewMode = $state<'flow' | 'grid'>('flow');
  let isMaximized = $state(false);

  let departments = $state<TeamDepartment[]>([]);
  let members = $state<TeamMember[]>([]);
  let rosterSummaries = $state<ServiceRosterSummary[]>([]);
  let selectedRosterId = $state<string | null>(null);
  let selectedRoster = $state<ServiceRoster | null>(null);
  let isLoading = $state(false);
  let loadError = $state('');

  let subDialog = $state<SubDialog>(null);
  let editingRoster = $state<ServiceRoster | null>(null);
  let assignDeptId = $state('');
  let editingMember = $state<TeamMember | null>(null);
  let editingDepartment = $state<TeamDepartment | null>(null);
  let confirmTarget = $state<ConfirmTarget | null>(null);

  let requestSeq = 0;
  let abortController: AbortController | null = null;
  let returnFocus: HTMLElement | null = null;
  let wasOpen = false;

  const shellClass = $derived(
    cn(
      'bg-card text-foreground shadow-2xl flex flex-col overflow-hidden',
      isMaximized
        ? 'w-full h-full rounded-none border-0'
        : 'w-full max-w-6xl h-[92vh] sm:h-[90vh] rounded-none sm:rounded-2xl border-0 sm:border border-border'
    )
  );

  function toMessage(err: unknown, fallback: string): string {
    return err instanceof Error && err.message ? err.message : fallback;
  }

  function nextRequest() {
    abortController?.abort();
    abortController = new AbortController();
    requestSeq += 1;
    return { seq: requestSeq, signal: abortController.signal };
  }

  async function loadAllData(showSpinner = true) {
    const { seq, signal } = nextRequest();
    if (showSpinner) isLoading = true;
    loadError = '';
    try {
      const [depts, mems, rosters] = await Promise.all([
        fetchDepartments(signal),
        fetchMembers(true, signal),
        fetchRosters(signal),
      ]);
      if (seq !== requestSeq) return;
      departments = depts;
      members = mems;
      rosterSummaries = rosters;
      const preferredId =
        selectedRosterId && rosters.some((roster) => roster.id === selectedRosterId)
          ? selectedRosterId
          : ((rosters.find((roster) => roster.is_active) ?? rosters[0])?.id ?? null);
      selectedRosterId = preferredId;
      if (!preferredId) {
        selectedRoster = null;
        return;
      }
      const detail = await fetchRoster(preferredId, signal);
      if (seq !== requestSeq) return;
      selectedRoster = detail;
    } catch (err) {
      if (seq !== requestSeq || (err instanceof DOMException && err.name === 'AbortError')) return;
      loadError = toMessage(err, 'Não foi possível carregar os dados da equipe.');
    } finally {
      if (showSpinner && seq === requestSeq) isLoading = false;
    }
  }

  async function selectRoster(id: string) {
    selectedRosterId = id;
    const { seq, signal } = nextRequest();
    isLoading = true;
    loadError = '';
    try {
      const detail = await fetchRoster(id, signal);
      if (seq === requestSeq) selectedRoster = detail;
    } catch (err) {
      if (seq !== requestSeq || (err instanceof DOMException && err.name === 'AbortError')) return;
      loadError = toMessage(err, 'Não foi possível carregar a escala selecionada.');
    } finally {
      if (seq === requestSeq) isLoading = false;
    }
  }

  function resetDialogs() {
    subDialog = null;
    editingRoster = null;
    editingMember = null;
    editingDepartment = null;
    assignDeptId = '';
    confirmTarget = null;
  }

  $effect(() => {
    const isOpen = open;
    if (isOpen === wasOpen) return;
    wasOpen = isOpen;
    untrack(() => {
      if (isOpen) {
        returnFocus =
          typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
        resetDialogs();
        void loadAllData();
        return;
      }
      abortController?.abort();
      requestSeq += 1;
      isLoading = false;
      loadError = '';
      resetDialogs();
      if (returnFocus) {
        returnFocus.focus?.();
        returnFocus = null;
      }
    });
  });

  onMount(() => {
    const unsubscribe = teamUpdatedStore.subscribe((update) => {
      if (update && open) void loadAllData(false);
    });
    return unsubscribe;
  });

  function selectTab(id: TeamsTabId) {
    activeTab = id;
    if (!visitedTabs[id]) visitedTabs = { ...visitedTabs, [id]: true };
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (!open || e.key !== 'Escape') return;
    e.preventDefault();
    e.stopPropagation();
    if (confirmTarget) {
      confirmTarget = null;
    } else if (subDialog) {
      subDialog = null;
    } else {
      onclose();
    }
  }

  function openRosterForm(roster: ServiceRoster | null = null) {
    editingRoster = roster;
    subDialog = 'roster-form';
  }

  function openAssign(deptId = '') {
    assignDeptId = deptId;
    subDialog = 'assign';
  }

  function openMemberForm(member: TeamMember | null = null) {
    editingMember = member;
    subDialog = 'member-form';
  }

  function openDeptForm(department: TeamDepartment | null = null) {
    editingDepartment = department;
    subDialog = 'dept-form';
  }

  async function afterSubDialog(saved?: ServiceRoster) {
    subDialog = null;
    if (saved) selectedRosterId = saved.id;
    await loadAllData(false);
  }

  function requestDeleteRoster() {
    if (!selectedRoster) return;
    confirmTarget = { kind: 'roster', id: selectedRoster.id, label: selectedRoster.title };
  }

  function requestRemoveItem(itemId: string) {
    if (!selectedRoster) return;
    const item = selectedRoster.items.find((entry) => entry.id === itemId);
    const member = members.find((entry) => entry.id === item?.member_id);
    confirmTarget = {
      kind: 'item',
      rosterId: selectedRoster.id,
      itemId,
      label: member?.name ?? 'voluntário',
    };
  }

  function requestDeleteMember(member: TeamMember) {
    confirmTarget = { kind: 'member', id: member.id, label: member.name };
  }

  function requestDeleteDepartment(department: TeamDepartment) {
    confirmTarget = { kind: 'department', id: department.id, label: department.name };
  }

  async function handleActivateRoster() {
    if (!selectedRosterId) return;
    try {
      await activateRoster(selectedRosterId);
      notifySuccess('Escala ativada', 'Ela já aparece como escala ativa no remoto.');
      await loadAllData(false);
    } catch (err) {
      notifyError('Não foi possível ativar a escala', toMessage(err, 'Tente novamente.'));
    }
  }

  async function handleUpdateItemStatus(itemId: string, status: RosterStatus) {
    if (!selectedRosterId) return;
    try {
      await updateRosterItemStatus(selectedRosterId, itemId, status);
      await loadAllData(false);
    } catch (err) {
      notifyError('Não foi possível atualizar a presença', toMessage(err, 'Tente novamente.'));
    }
  }
</script>

<svelte:window onkeydown={handleWindowKeydown} />
{#if open}
  <div class="fixed inset-0 z-40 flex items-center justify-center bg-black/70 p-0 {isMaximized ? '' : 'sm:p-4'}"
    role="presentation">
    <div class={shellClass} role="dialog" aria-modal="true" aria-labelledby="teams-modal-title"
      tabindex="-1">
      <TeamsHeader {activeTab} membersCount={members.length} departmentsCount={departments.length}
        {isMaximized} onSelectTab={selectTab} onToggleMaximize={() => (isMaximized = !isMaximized)}
        onClose={onclose} />

      <div class="flex-1 min-h-0 flex flex-col">
        <div class="flex-1 min-h-0 flex flex-col" role="tabpanel" id="teams-panel-roster"
          aria-labelledby="teams-tab-roster" class:hidden={activeTab !== 'roster'}>
          <TeamsRosterTab
            {rosterSummaries}
            {selectedRosterId}
            roster={selectedRoster}
            {departments}
            {members}
            {isLoading}
            errorMessage={loadError}
            {viewMode}
            onSelectRoster={selectRoster}
            onCreateRoster={() => openRosterForm()}
            onEditRoster={() => openRosterForm(selectedRoster)}
            onAssign={openAssign}
            onActivateRoster={handleActivateRoster}
            onDeleteRoster={requestDeleteRoster}
            onRemoveItem={requestRemoveItem}
            onUpdateItemStatus={handleUpdateItemStatus}
            onSetViewMode={(mode) => (viewMode = mode)}
            onAddMemberToDept={openAssign}
            onRetry={() => loadAllData()}
          />
        </div>
        <div class="flex-1 min-h-0 flex flex-col" role="tabpanel" id="teams-panel-members"
          aria-labelledby="teams-tab-members" class:hidden={activeTab !== 'members'}>
          {#if visitedTabs.members}
            <TeamsMembersTab
              {members}
              {departments}
              {isLoading}
              onNewMember={() => openMemberForm()}
              onEditMember={(member) => openMemberForm(member)}
              onDeleteMember={requestDeleteMember}
            />
          {/if}
        </div>
        <div class="flex-1 min-h-0 flex flex-col" role="tabpanel" id="teams-panel-departments"
          aria-labelledby="teams-tab-departments" class:hidden={activeTab !== 'departments'}>
          {#if visitedTabs.departments}
            <TeamsDepartmentsTab
              {departments}
              {members}
              {isLoading}
              onNewDepartment={() => openDeptForm()}
              onEditDepartment={(department) => openDeptForm(department)}
              onDeleteDepartment={requestDeleteDepartment}
            />
          {/if}
        </div>
      </div>
    </div>
  </div>

  <RosterFormModal open={subDialog === 'roster-form'} roster={editingRoster}
    onclose={() => (subDialog = null)} onSaved={(saved) => afterSubDialog(saved)} />

  <AssignMemberModal open={subDialog === 'assign'} {departments} {members} roster={selectedRoster}
    defaultDeptId={assignDeptId} onclose={() => (subDialog = null)} onSaved={() => afterSubDialog()} />

  <MemberFormModal open={subDialog === 'member-form'} member={editingMember} {departments}
    onclose={() => (subDialog = null)} onSaved={() => afterSubDialog()} />

  <DeptFormModal open={subDialog === 'dept-form'} department={editingDepartment}
    onclose={() => (subDialog = null)} onSaved={() => afterSubDialog()} />

  <TeamsConfirmDialog target={confirmTarget} onclose={() => (confirmTarget = null)}
    oncompleted={() => loadAllData(false)} />
{/if}

<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Users,
    Calendar,
    Clock,
    Plus,
    Trash2,
    Edit2,
    Check,
    CheckCircle2,
    X,
    Search,
    MessageCircle,
    ExternalLink,
    Crown,
    Shield,
    User,
    Copy,
    Workflow,
    LayoutGrid,
    Sparkles,
    AlertCircle,
    MonitorPlay,
    Camera,
    Smartphone,
    Video,
    Volume2,
    Radio,
    Mic,
    Music,
    Maximize2,
    Minimize2,
  } from 'lucide-svelte';
  import type {
    ServiceRoster,
    ServiceRosterSummary,
    TeamDepartment,
    TeamMember,
    TeamMemberRole,
    UpsertServiceRoster,
    UpsertServiceRosterItem,
    UpsertTeamDepartment,
    UpsertTeamMember,
  } from '$lib/types';
  import {
    fetchDepartments,
    fetchMembers,
    fetchRosters,
    fetchRoster,
    saveRoster,
    activateRoster,
    deleteRoster,
    addRosterItem,
    removeRosterItem,
    updateRosterItemStatus,
    saveDepartment,
    deleteDepartment,
    saveMember,
    deleteMember,
    formatRosterWhatsApp,
    getWhatsAppUrl,
  } from '$lib/teams';
  import { teamUpdatedStore } from '$lib/ws';
  import RosterFlowCanvas from './teams/RosterFlowCanvas.svelte';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open = false, onclose }: Props = $props();

  // Abas e visualizações
  let activeTab = $state<'roster' | 'members' | 'departments'>('roster');
  let viewMode = $state<'flow' | 'grid'>('flow');
  let isMaximized = $state(false);

  // Dados principais
  let departments = $state<TeamDepartment[]>([]);
  let members = $state<TeamMember[]>([]);
  let rosterSummaries = $state<ServiceRosterSummary[]>([]);
  let selectedRosterId = $state<string | null>(null);
  let selectedRoster = $state<ServiceRoster | null>(null);
  let isLoading = $state(false);
  let copySuccess = $state(false);
  let errorMessage = $state('');

  // Filtros de membros
  let memberSearch = $state('');
  let memberDeptFilter = $state<string>('all');

  // Submodais
  let isRosterModalOpen = $state(false);
  let isAssignModalOpen = $state(false);
  let isMemberModalOpen = $state(false);
  let isDeptModalOpen = $state(false);

  // Formulário Nova Escala
  let rosterFormTitle = $state('');
  let rosterFormDate = $state(new Date().toISOString().split('T')[0]);
  let rosterFormTime = $state('19:00');
  let rosterFormNotes = $state('');

  // Formulário Alocar Voluntário
  let assignDeptId = $state('');
  let assignMemberId = $state('');
  let assignRole = $state<'lead' | 'helper'>('lead');
  let assignNotes = $state('');

  // Formulário Membro
  let editingMemberId = $state<string | null>(null);
  let memberFormName = $state('');
  let memberFormPhone = $state('');
  let memberFormRole = $state<TeamMemberRole>('member');
  let memberFormNotes = $state('');
  let memberFormDepts = $state<string[]>([]);
  let memberFormIsActive = $state(true);

  // Formulário Departamento
  let editingDeptId = $state<string | null>(null);
  let deptFormName = $state('');
  let deptFormColor = $state('#3b82f6');
  let deptFormIcon = $state('Users');
  let deptFormDescription = $state('');

  const PRESET_COLORS = [
    '#3b82f6', // Azul
    '#10b981', // Esmeralda
    '#ec4899', // Rosa
    '#8b5cf6', // Roxo
    '#f59e0b', // Âmbar
    '#06b6d4', // Ciano
    '#ef4444', // Vermelho
    '#6366f1', // Índigo
  ];

  const PRESET_ICONS = [
    { name: 'MonitorPlay', label: 'Projeção', icon: MonitorPlay },
    { name: 'Camera', label: 'Foto', icon: Camera },
    { name: 'Smartphone', label: 'Redes', icon: Smartphone },
    { name: 'Video', label: 'Transmissão', icon: Video },
    { name: 'Volume2', label: 'Som / Áudio', icon: Volume2 },
    { name: 'Users', label: 'Equipe', icon: Users },
    { name: 'Radio', label: 'Live / Rádio', icon: Radio },
    { name: 'Mic', label: 'Vocal / Microfone', icon: Mic },
    { name: 'Music', label: 'Música / Louvor', icon: Music },
    { name: 'Sparkles', label: 'Especial', icon: Sparkles },
  ];

  $effect(() => {
    if (open) {
      loadAllData();
    }
  });

  onMount(() => {
    const unsub = teamUpdatedStore.subscribe((update) => {
      if (update && open) {
        loadAllData();
      }
    });
    return unsub;
  });

  async function loadAllData() {
    isLoading = true;
    errorMessage = '';
    try {
      const [depts, mems, rosters] = await Promise.all([
        fetchDepartments(),
        fetchMembers(true),
        fetchRosters(),
      ]);
      departments = depts;
      members = mems;
      rosterSummaries = rosters;

      if (!selectedRosterId && rosters.length > 0) {
        const active = rosters.find((r) => r.is_active);
        selectedRosterId = active ? active.id : rosters[0].id;
      }

      if (selectedRosterId) {
        await loadSelectedRoster(selectedRosterId);
      }
    } catch (err: any) {
      errorMessage = err.message || 'Erro ao carregar dados da equipe';
    } finally {
      isLoading = false;
    }
  }

  async function loadSelectedRoster(id: string) {
    try {
      selectedRoster = await fetchRoster(id);
    } catch (err: any) {
      console.error('Erro ao carregar escala detalhada:', err);
    }
  }

  async function handleSelectRoster(id: string) {
    selectedRosterId = id;
    await loadSelectedRoster(id);
  }

  // Ações de Escala
  function openNewRosterModal() {
    rosterFormTitle = 'Culto de Domingo';
    rosterFormDate = new Date().toISOString().split('T')[0];
    rosterFormTime = '19:00';
    rosterFormNotes = '';
    isRosterModalOpen = true;
  }

  async function handleSaveRoster() {
    if (!rosterFormTitle.trim() || !rosterFormDate) return;
    try {
      const payload: UpsertServiceRoster = {
        title: rosterFormTitle.trim(),
        service_date: rosterFormDate,
        service_time: rosterFormTime || null,
        notes: rosterFormNotes.trim() || null,
      };
      const created = await saveRoster(null, payload);
      isRosterModalOpen = false;
      await loadAllData();
      selectedRosterId = created.id;
      await loadSelectedRoster(created.id);
    } catch (err: any) {
      alert(`Erro ao criar escala: ${err.message}`);
    }
  }

  async function handleActivateRoster() {
    if (!selectedRosterId) return;
    try {
      await activateRoster(selectedRosterId);
      await loadAllData();
      if (selectedRosterId) await loadSelectedRoster(selectedRosterId);
    } catch (err: any) {
      alert(`Erro ao ativar escala: ${err.message}`);
    }
  }

  async function handleDeleteRoster() {
    if (!selectedRosterId || !selectedRoster) return;
    if (!confirm(`Deseja realmente excluir a escala "${selectedRoster.title}"?`)) return;
    try {
      await deleteRoster(selectedRosterId);
      selectedRosterId = null;
      selectedRoster = null;
      await loadAllData();
    } catch (err: any) {
      alert(`Erro ao excluir escala: ${err.message}`);
    }
  }

  function handleCopyWhatsApp() {
    if (!selectedRoster) return;
    const text = formatRosterWhatsApp(selectedRoster, departments, members);
    navigator.clipboard.writeText(text);
    copySuccess = true;
    setTimeout(() => {
      copySuccess = false;
    }, 2500);
  }

  // Ações de Itens da Escala
  function openAssignModal(deptId?: string) {
    assignDeptId = deptId || (departments[0]?.id ?? '');
    assignMemberId = '';
    assignRole = 'lead';
    assignNotes = '';
    isAssignModalOpen = true;
  }

  async function handleConfirmAssign() {
    if (!selectedRosterId || !assignDeptId || !assignMemberId) return;
    try {
      const payload: UpsertServiceRosterItem = {
        department_id: assignDeptId,
        member_id: assignMemberId,
        role: assignRole,
        status: 'pending',
        role_notes: assignNotes.trim() || null,
      };
      await addRosterItem(selectedRosterId, payload);
      isAssignModalOpen = false;
      await loadSelectedRoster(selectedRosterId);
    } catch (err: any) {
      alert(`Erro ao escalar voluntário: ${err.message}`);
    }
  }

  async function handleUpdateItemStatus(
    itemId: string,
    status: 'confirmed' | 'pending' | 'declined'
  ) {
    if (!selectedRosterId) return;
    try {
      await updateRosterItemStatus(selectedRosterId, itemId, status);
      await loadSelectedRoster(selectedRosterId);
    } catch (err: any) {
      alert(`Erro ao atualizar presença: ${err.message}`);
    }
  }

  async function handleRemoveItem(itemId: string) {
    if (!selectedRosterId) return;
    try {
      await removeRosterItem(selectedRosterId, itemId);
      await loadSelectedRoster(selectedRosterId);
    } catch (err: any) {
      alert(`Erro ao remover voluntário: ${err.message}`);
    }
  }

  // Membros CRUD
  function openNewMemberModal() {
    editingMemberId = null;
    memberFormName = '';
    memberFormPhone = '';
    memberFormRole = 'member';
    memberFormNotes = '';
    memberFormDepts = [];
    memberFormIsActive = true;
    isMemberModalOpen = true;
  }

  function openEditMemberModal(m: TeamMember) {
    editingMemberId = m.id;
    memberFormName = m.name;
    memberFormPhone = m.phone || '';
    memberFormRole = m.role || 'member';
    memberFormNotes = m.notes || '';
    memberFormDepts = [...(m.department_ids || m.departments || [])];
    memberFormIsActive = m.is_active;
    isMemberModalOpen = true;
  }

  async function handleSaveMember() {
    if (!memberFormName.trim()) return;
    try {
      const payload: UpsertTeamMember = {
        name: memberFormName.trim(),
        phone: memberFormPhone.trim() || null,
        role: memberFormRole,
        notes: memberFormNotes.trim() || null,
        department_ids: memberFormDepts,
        departments: memberFormDepts,
        is_active: memberFormIsActive,
      };
      await saveMember(editingMemberId, payload);
      isMemberModalOpen = false;
      await loadAllData();
    } catch (err: any) {
      alert(`Erro ao salvar membro: ${err.message}`);
    }
  }

  async function handleDeleteMember(id: string, name: string) {
    if (!confirm(`Deseja realmente excluir o membro "${name}"?`)) return;
    try {
      await deleteMember(id);
      await loadAllData();
    } catch (err: any) {
      alert(`Erro ao excluir membro: ${err.message}`);
    }
  }

  // Departamentos CRUD
  function openNewDeptModal() {
    editingDeptId = null;
    deptFormName = '';
    deptFormColor = '#3b82f6';
    deptFormIcon = 'Users';
    deptFormDescription = '';
    isDeptModalOpen = true;
  }

  function openEditDeptModal(d: TeamDepartment) {
    editingDeptId = d.id;
    deptFormName = d.name;
    deptFormColor = d.color;
    deptFormIcon = d.icon;
    deptFormDescription = d.description || '';
    isDeptModalOpen = true;
  }

  async function handleSaveDept() {
    if (!deptFormName.trim()) return;
    try {
      const payload: UpsertTeamDepartment = {
        name: deptFormName.trim(),
        color: deptFormColor,
        icon: deptFormIcon,
        description: deptFormDescription.trim() || null,
        is_active: true,
      };
      await saveDepartment(editingDeptId || '', payload);
      isDeptModalOpen = false;
      await loadAllData();
    } catch (err: any) {
      alert(`Erro ao salvar departamento: ${err.message}`);
    }
  }

  async function handleDeleteDept(id: string, name: string) {
    if (!confirm(`Deseja realmente excluir o departamento "${name}"?`)) return;
    try {
      await deleteDepartment(id);
      await loadAllData();
    } catch (err: any) {
      alert(`Erro ao excluir departamento: ${err.message}`);
    }
  }

  // Utilitários
  function getInitials(name?: string) {
    if (!name) return '?';
    const parts = name.trim().split(/\s+/);
    if (parts.length === 1) return parts[0].substring(0, 2).toUpperCase();
    return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
  }

  function getDeptIconComponent(iconName?: string) {
    const found = PRESET_ICONS.find((i) => i.name === iconName);
    return found ? found.icon : Users;
  }

  // Filtro de membros
  const filteredMembers = $derived.by(() => {
    let result = members;
    if (memberSearch.trim()) {
      const q = memberSearch.toLowerCase().trim();
      result = result.filter(
        (m) => m.name.toLowerCase().includes(q) || (m.phone && m.phone.includes(q))
      );
    }
    if (memberDeptFilter !== 'all') {
      result = result.filter((m) =>
        (m.department_ids || m.departments || []).includes(memberDeptFilter)
      );
    }
    return result;
  });
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm animate-in fade-in duration-200 {isMaximized ? 'p-0' : 'p-4'}"
    role="presentation"
  >
    <div
      class="{isMaximized
        ? 'w-full h-full max-w-none max-h-none rounded-none border-0'
        : 'w-full max-w-6xl h-[90vh] rounded-2xl border border-neutral-800'} bg-neutral-900 shadow-2xl flex flex-col overflow-hidden text-neutral-100 transition-all duration-150"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => {
        if (e.key === 'Escape') onclose();
      }}
      role="dialog"
      tabindex="-1"
    >
      <!-- Cabeçalho Principal -->
      <div
        class="h-16 px-6 border-b border-neutral-800 flex items-center justify-between shrink-0 bg-neutral-900/90"
      >
        <div class="flex items-center gap-4">
          <div
            class="w-10 h-10 rounded-xl bg-purple-500/15 border border-purple-500/30 text-purple-400 flex items-center justify-center shadow-inner"
          >
            <Users size={20} />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h2 class="text-base font-bold text-white">Equipe de Mídia & Escala de Culto</h2>
              <span
                class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-purple-500/20 text-purple-300 border border-purple-500/30"
              >
                Teams v1.0
              </span>
            </div>
            <p class="text-xs text-neutral-400">
              Gerencie voluntários, departamentos e escalas visuais dos cultos
            </p>
          </div>
        </div>

        <!-- Abas de Navegação -->
        <div class="flex items-center gap-1.5 p-1 rounded-xl bg-neutral-950/80 border border-neutral-800">
          <button
            type="button"
            onclick={() => (activeTab = 'roster')}
            class="px-3.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5 {activeTab ===
            'roster'
              ? 'bg-neutral-800 text-white shadow-sm'
              : 'text-neutral-400 hover:text-white hover:bg-neutral-900'}"
          >
            <Calendar size={13} />
            <span>Escala do Culto</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'members')}
            class="px-3.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5 {activeTab ===
            'members'
              ? 'bg-neutral-800 text-white shadow-sm'
              : 'text-neutral-400 hover:text-white hover:bg-neutral-900'}"
          >
            <Users size={13} />
            <span>Membros ({members.length})</span>
          </button>

          <button
            type="button"
            onclick={() => (activeTab = 'departments')}
            class="px-3.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5 {activeTab ===
            'departments'
              ? 'bg-neutral-800 text-white shadow-sm'
              : 'text-neutral-400 hover:text-white hover:bg-neutral-900'}"
          >
            <Sparkles size={13} />
            <span>Departamentos ({departments.length})</span>
          </button>
        </div>

        <!-- Ações Auxiliares (Docs Offline + Maximizar + Fechar) -->
        <div class="flex items-center gap-1.5">
          <a
            href="/docs"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium text-neutral-400 hover:text-white bg-neutral-800/60 hover:bg-neutral-800 border border-neutral-700/50 transition-colors"
            title="Documentação OpenAPI interativa (100% offline via Scalar)"
          >
            <ExternalLink size={13} />
            <span class="hidden sm:inline">API Docs</span>
          </a>

          <button
            type="button"
            onclick={() => (isMaximized = !isMaximized)}
            class="w-9 h-9 rounded-xl text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
            title={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
            aria-label={isMaximized ? 'Restaurar tamanho' : 'Maximizar'}
          >
            {#if isMaximized}
              <Minimize2 size={16} />
            {:else}
              <Maximize2 size={16} />
            {/if}
          </button>

          <button
            type="button"
            onclick={onclose}
            class="w-9 h-9 rounded-xl text-neutral-400 hover:text-white hover:bg-neutral-800 flex items-center justify-center transition-colors cursor-pointer"
            title="Fechar (Esc)"
          >
            <X size={18} />
          </button>
        </div>
      </div>

      <!-- CONTEÚDO DA ABA 1: ESCALA DO CULTO -->
      {#if activeTab === 'roster'}
        <div class="flex flex-col flex-1 min-h-0">
          <!-- Barra de Ações da Escala -->
          <div
            class="px-6 py-3 border-b border-neutral-800/80 bg-neutral-950/40 flex flex-wrap items-center justify-between gap-3 shrink-0"
          >
            <div class="flex items-center gap-3">
              <!-- Seletor de Escala -->
              <div class="flex items-center gap-2">
                <span class="text-xs font-semibold text-neutral-400">Escala:</span>
                <select
                  class="h-9 px-3 bg-neutral-900 border border-neutral-700 rounded-lg text-xs text-white focus:outline-none focus:border-purple-500 cursor-pointer max-w-[220px]"
                  value={selectedRosterId}
                  onchange={(e) => handleSelectRoster((e.target as HTMLSelectElement).value)}
                >
                  {#if rosterSummaries.length === 0}
                    <option value="">Nenhuma escala cadastrada</option>
                  {:else}
                    {#each rosterSummaries as r}
                      <option value={r.id}>
                        {r.title} ({r.service_date}) {r.is_active ? '★ Ativa' : ''}
                      </option>
                    {/each}
                  {/if}
                </select>
              </div>

              <!-- Botão Nova Escala -->
              <button
                type="button"
                onclick={openNewRosterModal}
                class="inline-flex items-center gap-1.5 h-9 px-3 rounded-lg text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white shadow-sm transition-colors cursor-pointer"
              >
                <Plus size={14} />
                <span>Nova Escala</span>
              </button>

              {#if selectedRoster}
                <!-- Botão Escalar Voluntário -->
                <button
                  type="button"
                  onclick={() => openAssignModal()}
                  class="inline-flex items-center gap-1.5 h-9 px-3 rounded-lg text-xs font-semibold bg-neutral-800 hover:bg-neutral-700 text-white border border-neutral-700 transition-colors cursor-pointer"
                >
                  <Plus size={14} />
                  <span>Escalar Voluntário</span>
                </button>
              {/if}
            </div>

            {#if selectedRoster}
              <div class="flex items-center gap-2">
                <!-- Status de Ativação -->
                {#if selectedRoster.is_active}
                  <div
                    class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-emerald-500/15 border border-emerald-500/30 text-emerald-400 text-xs font-semibold"
                  >
                    <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                    <span>Escala Ativa no Telão</span>
                  </div>
                {:else}
                  <button
                    type="button"
                    onclick={handleActivateRoster}
                    class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-neutral-800 hover:bg-emerald-600/30 text-neutral-300 hover:text-emerald-300 border border-neutral-700 transition-colors text-xs font-semibold cursor-pointer"
                  >
                    <CheckCircle2 size={13} />
                    <span>Tornar Ativa</span>
                  </button>
                {/if}

                <!-- Copiar para WhatsApp -->
                <button
                  type="button"
                  onclick={handleCopyWhatsApp}
                  class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-emerald-600/20 hover:bg-emerald-600/30 text-emerald-400 border border-emerald-500/30 transition-colors text-xs font-semibold cursor-pointer"
                  title="Copiar escala formatada para colar no grupo do WhatsApp"
                >
                  {#if copySuccess}
                    <Check size={13} class="text-emerald-400" />
                    <span>Copiado!</span>
                  {:else}
                    <MessageCircle size={13} />
                    <span>Copiar WhatsApp</span>
                  {/if}
                </button>

                <!-- Alternador de Modo: Flow vs Grade -->
                <div class="flex items-center p-0.5 rounded-lg bg-neutral-900 border border-neutral-800">
                  <button
                    type="button"
                    onclick={() => (viewMode = 'flow')}
                    class="p-1.5 rounded-md text-xs transition-colors cursor-pointer {viewMode === 'flow'
                      ? 'bg-neutral-800 text-white shadow-sm'
                      : 'text-neutral-400 hover:text-white'}"
                    title="Visualização Svelte Flow (Fluxo Interativo)"
                  >
                    <Workflow size={15} />
                  </button>
                  <button
                    type="button"
                    onclick={() => (viewMode = 'grid')}
                    class="p-1.5 rounded-md text-xs transition-colors cursor-pointer {viewMode === 'grid'
                      ? 'bg-neutral-800 text-white shadow-sm'
                      : 'text-neutral-400 hover:text-white'}"
                    title="Visualização Grade de Cartões"
                  >
                    <LayoutGrid size={15} />
                  </button>
                </div>

                <!-- Excluir Escala -->
                <button
                  type="button"
                  onclick={handleDeleteRoster}
                  class="p-2 rounded-lg text-neutral-400 hover:text-rose-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                  title="Excluir esta escala"
                >
                  <Trash2 size={15} />
                </button>
              </div>
            {/if}
          </div>

          <!-- Área Visual da Escala -->
          <div class="flex-1 min-h-0 p-4 overflow-hidden relative">
            {#if isLoading}
              <div class="h-full flex items-center justify-center text-neutral-400">
                <span class="text-sm">Carregando dados da equipe...</span>
              </div>
            {:else if !selectedRoster}
              <div class="h-full flex flex-col items-center justify-center text-center p-8">
                <div
                  class="w-16 h-16 rounded-2xl bg-neutral-800/80 border border-neutral-700/80 flex items-center justify-center text-neutral-400 mb-4"
                >
                  <Calendar size={28} />
                </div>
                <h3 class="text-base font-bold text-white mb-1">Nenhuma escala ativa selecionada</h3>
                <p class="text-xs text-neutral-400 max-w-sm mb-4">
                  Crie uma escala de culto para alocar os voluntários da equipe de mídia por departamento.
                </p>
                <button
                  type="button"
                  onclick={openNewRosterModal}
                  class="inline-flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white shadow-lg transition-colors cursor-pointer"
                >
                  <Plus size={15} />
                  <span>Criar Primeira Escala</span>
                </button>
              </div>
            {:else if viewMode === 'flow'}
              <!-- Visualização com Svelte Flow -->
              <RosterFlowCanvas
                roster={selectedRoster}
                {departments}
                allMembers={members}
                onUpdateRosterItemStatus={handleUpdateItemStatus}
                onRemoveRosterItem={handleRemoveItem}
                onAddMemberToDept={openAssignModal}
              />
            {:else}
              <!-- Visualização Clássica em Grade / Cartões -->
              <div class="h-full overflow-x-auto overflow-y-auto pb-4">
                <div class="flex items-start gap-4 min-w-max">
                  {#each departments as dept}
                    {@const deptItems = selectedRoster.items.filter(
                      (it) => it.department_id === dept.id
                    )}
                    {@const DeptIcon = getDeptIconComponent(dept.icon)}
                    <div
                      class="w-72 rounded-2xl bg-neutral-950/80 border border-neutral-800 p-3.5 flex flex-col gap-3 shrink-0"
                    >
                      <!-- Topo do Departamento -->
                      <div class="flex items-center justify-between">
                        <div class="flex items-center gap-2">
                          <div
                            class="w-7 h-7 rounded-lg flex items-center justify-center text-white"
                            style="background-color: {dept.color};"
                          >
                            <DeptIcon size={14} />
                          </div>
                          <div>
                            <h4 class="text-xs font-bold text-white">{dept.name}</h4>
                            <span class="text-[10px] text-neutral-400">
                              {deptItems.length} {deptItems.length === 1 ? 'voluntário' : 'voluntários'}
                            </span>
                          </div>
                        </div>

                        <button
                          type="button"
                          onclick={() => openAssignModal(dept.id)}
                          class="w-7 h-7 rounded-lg bg-neutral-900 hover:bg-neutral-800 text-neutral-300 hover:text-white flex items-center justify-center border border-neutral-800 transition-colors cursor-pointer"
                          title="Escalar voluntário para {dept.name}"
                        >
                          <Plus size={14} />
                        </button>
                      </div>

                      <!-- Lista de Voluntários Escalados -->
                      <div class="flex flex-col gap-2 min-h-[100px]">
                        {#if deptItems.length === 0}
                          <div
                            class="p-4 rounded-xl border border-dashed border-neutral-800 text-center text-[11px] text-neutral-500"
                          >
                            Nenhum voluntário escalado
                          </div>
                        {:else}
                          {#each deptItems as item}
                            {@const member = members.find((m) => m.id === item.member_id)}
                            {@const memberName = member?.name || 'Voluntário'}
                            {@const memberPhone = member?.phone}
                            {@const waUrl = getWhatsAppUrl(
                              memberPhone,
                              `Olá ${memberName}, confirmando sua escala no culto de ${selectedRoster.service_date} em ${dept.name}!`
                            )}
                            <div
                              class="p-2.5 rounded-xl bg-neutral-900/90 border border-neutral-800 hover:border-neutral-700 transition-all flex flex-col gap-2"
                            >
                              <div class="flex items-center justify-between gap-2">
                                <div class="flex items-center gap-2 min-w-0">
                                  <div
                                    class="w-7 h-7 rounded-full flex items-center justify-center text-xs font-bold text-white shrink-0"
                                    style="background-color: {dept.color}44; border: 1.5px solid {dept.color};"
                                  >
                                    {getInitials(memberName)}
                                  </div>
                                  <div class="min-w-0">
                                    <div class="flex items-center gap-1">
                                      <span class="text-xs font-semibold text-white truncate">
                                        {memberName}
                                      </span>
                                      {#if item.role === 'lead'}
                                        <span class="text-amber-400 shrink-0" title="Titular">
                                          <Crown size={11} />
                                        </span>
                                      {/if}
                                    </div>
                                    <span class="text-[10px] text-neutral-400 capitalize">
                                      {item.role === 'lead' ? 'Titular' : 'Auxiliar'}
                                    </span>
                                  </div>
                                </div>

                                <div class="flex items-center gap-1">
                                  {#if waUrl}
                                    <a
                                      href={waUrl}
                                      target="_blank"
                                      rel="noopener noreferrer"
                                      class="w-6 h-6 rounded-md bg-emerald-500/10 hover:bg-emerald-500 text-emerald-400 hover:text-white flex items-center justify-center transition-colors"
                                      title="WhatsApp"
                                    >
                                      <MessageCircle size={12} />
                                    </a>
                                  {/if}
                                  <button
                                    type="button"
                                    onclick={() => handleRemoveItem(item.id)}
                                    class="w-6 h-6 rounded-md bg-neutral-800 hover:bg-rose-600 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
                                    title="Remover da escala"
                                  >
                                    <Trash2 size={12} />
                                  </button>
                                </div>
                              </div>

                              <!-- Botão de Status Alternável -->
                              <div class="flex items-center justify-between pt-1.5 border-t border-neutral-800/80">
                                <button
                                  type="button"
                                  onclick={() => {
                                    const nextStatus =
                                      item.status === 'confirmed'
                                        ? 'declined'
                                        : item.status === 'declined'
                                          ? 'pending'
                                          : 'confirmed';
                                    handleUpdateItemStatus(item.id, nextStatus);
                                  }}
                                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-medium border cursor-pointer {item.status ===
                                  'confirmed'
                                    ? 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30'
                                    : item.status === 'declined'
                                      ? 'bg-rose-500/15 text-rose-400 border-rose-500/30'
                                      : 'bg-amber-500/15 text-amber-400 border-amber-500/30'}"
                                >
                                  {#if item.status === 'confirmed'}
                                    <CheckCircle2 size={10} />
                                    <span>Confirmado</span>
                                  {:else if item.status === 'declined'}
                                    <X size={10} />
                                    <span>Não Comparece</span>
                                  {:else}
                                    <Clock size={10} />
                                    <span>Pendente</span>
                                  {/if}
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
      {/if}

      <!-- CONTEÚDO DA ABA 2: MEMBROS -->
      {#if activeTab === 'members'}
        <div class="flex flex-col flex-1 min-h-0">
          <!-- Filtros de Membros -->
          <div
            class="px-6 py-3 border-b border-neutral-800/80 bg-neutral-950/40 flex items-center justify-between gap-4 shrink-0"
          >
            <div class="flex items-center gap-3 flex-1 max-w-lg">
              <!-- Busca -->
              <div class="relative flex-1">
                <Search
                  size={14}
                  class="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-500 pointer-events-none"
                />
                <input
                  type="text"
                  bind:value={memberSearch}
                  placeholder="Buscar voluntário por nome ou telefone..."
                  class="w-full h-9 pl-9 pr-3 bg-neutral-900 border border-neutral-700/80 rounded-lg text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-purple-500"
                />
              </div>

              <!-- Filtro de Departamento -->
              <select
                bind:value={memberDeptFilter}
                class="h-9 px-3 bg-neutral-900 border border-neutral-700/80 rounded-lg text-xs text-white focus:outline-none focus:border-purple-500 cursor-pointer"
              >
                <option value="all">Todos os Departamentos</option>
                {#each departments as d}
                  <option value={d.id}>{d.name}</option>
                {/each}
              </select>
            </div>

            <!-- Botão Novo Membro -->
            <button
              type="button"
              onclick={openNewMemberModal}
              class="inline-flex items-center gap-1.5 h-9 px-3.5 rounded-lg text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white shadow-sm transition-colors cursor-pointer"
            >
              <Plus size={14} />
              <span>Novo Voluntário</span>
            </button>
          </div>

          <!-- Lista de Membros -->
          <div class="flex-1 min-h-0 p-6 overflow-y-auto">
            {#if filteredMembers.length === 0}
              <div class="h-full flex flex-col items-center justify-center text-center p-8">
                <p class="text-sm font-semibold text-neutral-400">Nenhum voluntário encontrado.</p>
                <p class="text-xs text-neutral-500 mt-1">
                  Cadastre os membros da equipe de mídia para poder escalá-los nos cultos.
                </p>
              </div>
            {:else}
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3.5">
                {#each filteredMembers as member}
                  {@const waUrl = getWhatsAppUrl(
                    member.phone,
                    `Olá ${member.name}! Mensagem da equipe de mídia.`
                  )}
                  <div
                    class="p-4 rounded-2xl bg-neutral-950/60 border border-neutral-800/80 hover:border-neutral-700 transition-all flex flex-col justify-between gap-3"
                  >
                    <div>
                      <div class="flex items-start justify-between gap-2">
                        <div class="flex items-center gap-3">
                          <div
                            class="w-10 h-10 rounded-full bg-neutral-800 border border-neutral-700 flex items-center justify-center text-sm font-bold text-white shadow-sm shrink-0"
                          >
                            {getInitials(member.name)}
                          </div>
                          <div>
                            <div class="flex items-center gap-2">
                              <h4 class="text-sm font-bold text-white">{member.name}</h4>
                              {#if member.role === 'leader'}
                                <span
                                  class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-semibold bg-amber-500/20 text-amber-300 border border-amber-500/40"
                                >
                                  <Crown size={10} />
                                  Líder
                                </span>
                              {:else if member.role === 'vice_leader'}
                                <span
                                  class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-semibold bg-indigo-500/20 text-indigo-300 border border-indigo-500/40"
                                >
                                  <Shield size={10} />
                                  Vice-Líder
                                </span>
                              {:else}
                                <span
                                  class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-medium bg-neutral-800 text-neutral-400 border border-neutral-700/60"
                                >
                                  Membro
                                </span>
                              {/if}
                            </div>
                            {#if member.phone}
                              <div class="flex items-center gap-1 text-xs text-neutral-400 mt-0.5">
                                <span>{member.phone}</span>
                              </div>
                            {:else}
                              <span class="text-[11px] text-neutral-500 italic">Sem telefone</span>
                            {/if}
                          </div>
                        </div>

                        <!-- Ações Rápidas -->
                        <div class="flex items-center gap-1">
                          {#if waUrl}
                            <a
                              href={waUrl}
                              target="_blank"
                              rel="noopener noreferrer"
                              class="w-7 h-7 rounded-lg bg-emerald-500/10 hover:bg-emerald-500 text-emerald-400 hover:text-white flex items-center justify-center transition-colors"
                              title="Abrir no WhatsApp"
                            >
                              <MessageCircle size={14} />
                            </a>
                          {/if}

                          <button
                            type="button"
                            onclick={() => openEditMemberModal(member)}
                            class="w-7 h-7 rounded-lg bg-neutral-900 hover:bg-neutral-800 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
                            title="Editar voluntário"
                          >
                            <Edit2 size={13} />
                          </button>

                          <button
                            type="button"
                            onclick={() => handleDeleteMember(member.id, member.name)}
                            class="w-7 h-7 rounded-lg bg-neutral-900 hover:bg-rose-600 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
                            title="Excluir voluntário"
                          >
                            <Trash2 size={13} />
                          </button>
                        </div>
                      </div>

                      <!-- Departamentos Vinculados -->
                      <div class="flex flex-wrap items-center gap-1.5 mt-3">
                        {#if (member.department_ids || member.departments || []).length === 0}
                          <span class="text-[10px] text-neutral-500">Sem departamentos</span>
                        {:else}
                          {#each (member.department_ids || member.departments || []) as dId}
                            {@const dept = departments.find((d) => d.id === dId)}
                            {#if dept}
                              <span
                                class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-semibold text-white/90"
                                style="background-color: {dept.color}33; border: 1px solid {dept.color}88;"
                              >
                                <span
                                  class="w-1.5 h-1.5 rounded-full"
                                  style="background-color: {dept.color};"
                                ></span>
                                {dept.name}
                              </span>
                            {/if}
                          {/each}
                        {/if}
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- CONTEÚDO DA ABA 3: DEPARTAMENTOS -->
      {#if activeTab === 'departments'}
        <div class="flex flex-col flex-1 min-h-0">
          <!-- Topo Departamentos -->
          <div
            class="px-6 py-3 border-b border-neutral-800/80 bg-neutral-950/40 flex items-center justify-between gap-4 shrink-0"
          >
            <div>
              <p class="text-xs text-neutral-400">
                Departamentos da equipe (ex: Projeção, Fotografia, Redes, Áudio, Transmissão).
              </p>
            </div>

            <!-- Botão Novo Departamento -->
            <button
              type="button"
              onclick={openNewDeptModal}
              class="inline-flex items-center gap-1.5 h-9 px-3.5 rounded-lg text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white shadow-sm transition-colors cursor-pointer"
            >
              <Plus size={14} />
              <span>Novo Departamento</span>
            </button>
          </div>

          <!-- Lista de Departamentos -->
          <div class="flex-1 min-h-0 p-6 overflow-y-auto">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {#each departments as dept}
                {@const DeptIcon = getDeptIconComponent(dept.icon)}
                {@const deptMembers = members.filter((m) =>
                  (m.department_ids || m.departments || []).includes(dept.id)
                )}
                <div
                  class="p-4 rounded-2xl bg-neutral-950/60 border-2 transition-all flex flex-col justify-between gap-4"
                  style="border-color: {dept.color}66;"
                >
                  <div class="flex items-start justify-between gap-3">
                    <div class="flex items-center gap-3">
                      <div
                        class="w-10 h-10 rounded-xl flex items-center justify-center text-white shadow-md shrink-0"
                        style="background-color: {dept.color};"
                      >
                        <DeptIcon size={18} />
                      </div>
                      <div>
                        <h4 class="text-sm font-bold text-white">{dept.name}</h4>
                        <span class="text-xs text-neutral-400 font-medium">
                          {deptMembers.length} {deptMembers.length === 1 ? 'membro cadastrado' : 'membros cadastrados'}
                        </span>
                      </div>
                    </div>

                    <div class="flex items-center gap-1">
                      <button
                        type="button"
                        onclick={() => openEditDeptModal(dept)}
                        class="w-7 h-7 rounded-lg bg-neutral-900 hover:bg-neutral-800 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
                        title="Editar departamento"
                      >
                        <Edit2 size={13} />
                      </button>

                      <button
                        type="button"
                        onclick={() => handleDeleteDept(dept.id, dept.name)}
                        class="w-7 h-7 rounded-lg bg-neutral-900 hover:bg-rose-600 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
                        title="Excluir departamento"
                      >
                        <Trash2 size={13} />
                      </button>
                    </div>
                  </div>

                  {#if dept.description}
                    <p class="text-xs text-neutral-400 leading-relaxed">
                      {dept.description}
                    </p>
                  {/if}

                  <div class="pt-2 border-t border-neutral-800/80 flex items-center justify-between text-[11px] text-neutral-500">
                    <span class="inline-flex items-center gap-1">
                      <span class="w-2 h-2 rounded-full" style="background-color: {dept.color};"></span>
                      Cor da identificação
                    </span>
                    <span>Ícone: {dept.icon}</span>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- SUBMODAL: NOVA ESCALA -->
  {#if isRosterModalOpen}
    <div
      class="fixed inset-0 z-60 flex items-center justify-center bg-black/70 backdrop-blur-xs p-4"
      role="presentation"
    >
      <div
        class="w-full max-w-md rounded-2xl border border-neutral-800 bg-neutral-900 shadow-2xl p-6 text-white space-y-4"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => {
          if (e.key === 'Escape') isRosterModalOpen = false;
        }}
        role="dialog"
        tabindex="-1"
      >
        <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
          <h3 class="text-sm font-bold">Criar Nova Escala de Culto</h3>
          <button
            type="button"
            onclick={() => (isRosterModalOpen = false)}
            class="text-neutral-400 hover:text-white"
          >
            <X size={16} />
          </button>
        </div>

        <div class="space-y-3 text-xs">
          <div>
            <label for="roster-title" class="block font-semibold text-neutral-300 mb-1">
              Nome do Culto / Evento <span class="text-purple-400">*</span>
            </label>
            <input
              id="roster-title"
              type="text"
              bind:value={rosterFormTitle}
              placeholder="Ex.: Culto de Domingo - Noite"
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            />
          </div>

          <div class="grid grid-cols-2 gap-3">
            <div>
              <label for="roster-date" class="block font-semibold text-neutral-300 mb-1">
                Data <span class="text-purple-400">*</span>
              </label>
              <input
                id="roster-date"
                type="date"
                bind:value={rosterFormDate}
                class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
              />
            </div>
            <div>
              <label for="roster-time" class="block font-semibold text-neutral-300 mb-1">
                Horário
              </label>
              <input
                id="roster-time"
                type="time"
                bind:value={rosterFormTime}
                class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
              />
            </div>
          </div>

          <div>
            <label for="roster-notes" class="block font-semibold text-neutral-300 mb-1">
              Observações (opcional)
            </label>
            <textarea
              id="roster-notes"
              bind:value={rosterFormNotes}
              rows="2"
              placeholder="Ex.: Chegar 40 minutos antes para alinhamento..."
              class="w-full p-2.5 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            ></textarea>
          </div>
        </div>

        <div class="flex items-center justify-end gap-2 pt-2 border-t border-neutral-800">
          <button
            type="button"
            onclick={() => (isRosterModalOpen = false)}
            class="px-4 py-2 rounded-lg text-xs font-medium text-neutral-300 hover:bg-neutral-800"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={handleSaveRoster}
            class="px-4 py-2 rounded-lg text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white"
          >
            Criar Escala
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- SUBMODAL: ESCALAR VOLUNTÁRIO -->
  {#if isAssignModalOpen}
    <div
      class="fixed inset-0 z-60 flex items-center justify-center bg-black/70 backdrop-blur-xs p-4"
      role="presentation"
    >
      <div
        class="w-full max-w-md rounded-2xl border border-neutral-800 bg-neutral-900 shadow-2xl p-6 text-white space-y-4"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => {
          if (e.key === 'Escape') isAssignModalOpen = false;
        }}
        role="dialog"
        tabindex="-1"
      >
        <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
          <h3 class="text-sm font-bold">Escalar Voluntário</h3>
          <button
            type="button"
            onclick={() => (isAssignModalOpen = false)}
            class="text-neutral-400 hover:text-white"
          >
            <X size={16} />
          </button>
        </div>

        <div class="space-y-3 text-xs">
          <!-- Departamento -->
          <div>
            <label for="assign-dept" class="block font-semibold text-neutral-300 mb-1">
              Departamento <span class="text-purple-400">*</span>
            </label>
            <select
              id="assign-dept"
              bind:value={assignDeptId}
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500 cursor-pointer"
            >
              {#each departments as d}
                <option value={d.id}>{d.name}</option>
              {/each}
            </select>
          </div>

          <!-- Voluntário -->
          <div>
            <label for="assign-member" class="block font-semibold text-neutral-300 mb-1">
              Membro da Equipe <span class="text-purple-400">*</span>
            </label>
            <select
              id="assign-member"
              bind:value={assignMemberId}
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500 cursor-pointer"
            >
              <option value="">Selecione um voluntário...</option>
              {#each members as m}
                {@const isDeptMember = (m.department_ids || m.departments || []).includes(
                  assignDeptId
                )}
                <option value={m.id}>
                  {m.name} {isDeptMember ? '✓ (do departamento)' : ''}
                </option>
              {/each}
            </select>
          </div>

          <!-- Função -->
          <div>
            <span class="block font-semibold text-neutral-300 mb-1">Função na Escala</span>
            <div class="grid grid-cols-2 gap-2">
              <button
                type="button"
                onclick={() => (assignRole = 'lead')}
                class="py-2 px-3 rounded-lg border text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer {assignRole ===
                'lead'
                  ? 'bg-amber-500/20 text-amber-300 border-amber-500/40'
                  : 'bg-neutral-950 border-neutral-800 text-neutral-400 hover:text-white'}"
              >
                <Crown size={13} />
                <span>Titular (Responsável)</span>
              </button>
              <button
                type="button"
                onclick={() => (assignRole = 'helper')}
                class="py-2 px-3 rounded-lg border text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer {assignRole ===
                'helper'
                  ? 'bg-purple-500/20 text-purple-300 border-purple-500/40'
                  : 'bg-neutral-950 border-neutral-800 text-neutral-400 hover:text-white'}"
              >
                <Users size={13} />
                <span>Auxiliar / Apoio</span>
              </button>
            </div>
          </div>

          <!-- Observação -->
          <div>
            <label for="assign-notes" class="block font-semibold text-neutral-300 mb-1">
              Observações (opcional)
            </label>
            <input
              id="assign-notes"
              type="text"
              bind:value={assignNotes}
              placeholder="Ex.: Levar câmera reserva..."
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            />
          </div>
        </div>

        <div class="flex items-center justify-end gap-2 pt-2 border-t border-neutral-800">
          <button
            type="button"
            onclick={() => (isAssignModalOpen = false)}
            class="px-4 py-2 rounded-lg text-xs font-medium text-neutral-300 hover:bg-neutral-800"
          >
            Cancelar
          </button>
          <button
            type="button"
            disabled={!assignMemberId}
            onclick={handleConfirmAssign}
            class="px-4 py-2 rounded-lg text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white disabled:opacity-50 cursor-pointer"
          >
            Confirmar Escala
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- SUBMODAL: CADASTRO / EDIÇÃO DE MEMBRO -->
  {#if isMemberModalOpen}
    <div
      class="fixed inset-0 z-60 flex items-center justify-center bg-black/70 backdrop-blur-xs p-4"
      role="presentation"
    >
      <div
        class="w-full max-w-md rounded-2xl border border-neutral-800 bg-neutral-900 shadow-2xl p-6 text-white space-y-4"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => {
          if (e.key === 'Escape') isMemberModalOpen = false;
        }}
        role="dialog"
        tabindex="-1"
      >
        <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
          <h3 class="text-sm font-bold">
            {editingMemberId ? 'Editar Voluntário' : 'Novo Voluntário'}
          </h3>
          <button
            type="button"
            onclick={() => (isMemberModalOpen = false)}
            class="text-neutral-400 hover:text-white"
          >
            <X size={16} />
          </button>
        </div>

        <div class="space-y-3 text-xs">
          <div>
            <label for="member-name" class="block font-semibold text-neutral-300 mb-1">
              Nome Completo <span class="text-purple-400">*</span>
            </label>
            <input
              id="member-name"
              type="text"
              bind:value={memberFormName}
              placeholder="Ex.: Claudio Silva"
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            />
          </div>

          <div>
            <label for="member-phone" class="block font-semibold text-neutral-300 mb-1">
              WhatsApp / Telefone (opcional)
            </label>
            <input
              id="member-phone"
              type="text"
              bind:value={memberFormPhone}
              placeholder="Ex.: (11) 98765-4321"
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            />
          </div>

          <div>
            <span class="block font-semibold text-neutral-300 mb-1.5">Cargo na Equipe</span>
            <div class="grid grid-cols-3 gap-2">
              <button
                type="button"
                onclick={() => (memberFormRole = 'leader')}
                class="py-2 px-2.5 rounded-lg border text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer {memberFormRole ===
                'leader'
                  ? 'border-amber-500 bg-amber-500/20 text-amber-300 shadow-sm'
                  : 'border-neutral-800 bg-neutral-950 text-neutral-400 hover:border-neutral-700'}"
              >
                <Crown size={13} />
                <span>Líder</span>
              </button>

              <button
                type="button"
                onclick={() => (memberFormRole = 'vice_leader')}
                class="py-2 px-2.5 rounded-lg border text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer {memberFormRole ===
                'vice_leader'
                  ? 'border-indigo-500 bg-indigo-500/20 text-indigo-300 shadow-sm'
                  : 'border-neutral-800 bg-neutral-950 text-neutral-400 hover:border-neutral-700'}"
              >
                <Shield size={13} />
                <span>Vice-Líder</span>
              </button>

              <button
                type="button"
                onclick={() => (memberFormRole = 'member')}
                class="py-2 px-2.5 rounded-lg border text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer {memberFormRole ===
                'member'
                  ? 'border-purple-500 bg-purple-500/20 text-purple-300 shadow-sm'
                  : 'border-neutral-800 bg-neutral-950 text-neutral-400 hover:border-neutral-700'}"
              >
                <User size={13} />
                <span>Membro</span>
              </button>
            </div>
          </div>

          <div>
            <span class="block font-semibold text-neutral-300 mb-1.5">
              Departamentos em que atua
            </span>
            <div class="grid grid-cols-2 gap-2 max-h-36 overflow-y-auto p-1 rounded-lg bg-neutral-950 border border-neutral-800">
              {#each departments as d}
                {@const isChecked = memberFormDepts.includes(d.id)}
                <label
                  class="flex items-center gap-2 p-2 rounded-md hover:bg-neutral-900 cursor-pointer select-none text-[11px]"
                >
                  <input
                    type="checkbox"
                    checked={isChecked}
                    onchange={(e) => {
                      if ((e.target as HTMLInputElement).checked) {
                        memberFormDepts = [...memberFormDepts, d.id];
                      } else {
                        memberFormDepts = memberFormDepts.filter((id) => id !== d.id);
                      }
                    }}
                    class="rounded accent-purple-500"
                  />
                  <span class="w-2 h-2 rounded-full shrink-0" style="background-color: {d.color};"></span>
                  <span class="truncate">{d.name}</span>
                </label>
              {/each}
            </div>
          </div>

          <div>
            <label for="member-notes" class="block font-semibold text-neutral-300 mb-1">
              Observações
            </label>
            <textarea
              id="member-notes"
              bind:value={memberFormNotes}
              rows="2"
              placeholder="Ex.: Disponível aos domingos à noite..."
              class="w-full p-2.5 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            ></textarea>
          </div>
        </div>

        <div class="flex items-center justify-end gap-2 pt-2 border-t border-neutral-800">
          <button
            type="button"
            onclick={() => (isMemberModalOpen = false)}
            class="px-4 py-2 rounded-lg text-xs font-medium text-neutral-300 hover:bg-neutral-800"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={handleSaveMember}
            class="px-4 py-2 rounded-lg text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white"
          >
            Salvar Voluntário
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- SUBMODAL: CADASTRO / EDIÇÃO DE DEPARTAMENTO -->
  {#if isDeptModalOpen}
    <div
      class="fixed inset-0 z-60 flex items-center justify-center bg-black/70 backdrop-blur-xs p-4"
      role="presentation"
    >
      <div
        class="w-full max-w-md rounded-2xl border border-neutral-800 bg-neutral-900 shadow-2xl p-6 text-white space-y-4"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => {
          if (e.key === 'Escape') isDeptModalOpen = false;
        }}
        role="dialog"
        tabindex="-1"
      >
        <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
          <h3 class="text-sm font-bold">
            {editingDeptId ? 'Editar Departamento' : 'Novo Departamento'}
          </h3>
          <button
            type="button"
            onclick={() => (isDeptModalOpen = false)}
            class="text-neutral-400 hover:text-white"
          >
            <X size={16} />
          </button>
        </div>

        <div class="space-y-3 text-xs">
          <div>
            <label for="dept-name" class="block font-semibold text-neutral-300 mb-1">
              Nome do Departamento <span class="text-purple-400">*</span>
            </label>
            <input
              id="dept-name"
              type="text"
              bind:value={deptFormName}
              placeholder="Ex.: Fotografia, Stories & Redes..."
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            />
          </div>

          <div>
            <span class="block font-semibold text-neutral-300 mb-1">Cor de Destaque</span>
            <div class="flex items-center gap-2 flex-wrap">
              {#each PRESET_COLORS as color}
                <button
                  type="button"
                  aria-label="Selecionar cor {color}"
                  onclick={() => (deptFormColor = color)}
                  class="w-7 h-7 rounded-full border-2 transition-transform cursor-pointer {deptFormColor ===
                  color
                    ? 'scale-125 border-white shadow-lg'
                    : 'border-transparent hover:scale-110'}"
                  style="background-color: {color};"
                ></button>
              {/each}
              <input
                type="color"
                bind:value={deptFormColor}
                class="w-7 h-7 rounded-full overflow-hidden cursor-pointer bg-transparent border-0"
                title="Cor personalizada"
              />
            </div>
          </div>

          <div>
            <span class="block font-semibold text-neutral-300 mb-1">Ícone</span>
            <div class="grid grid-cols-5 gap-2">
              {#each PRESET_ICONS as item}
                {@const IconComp = item.icon}
                <button
                  type="button"
                  onclick={() => (deptFormIcon = item.name)}
                  class="flex flex-col items-center justify-center p-2 rounded-xl border transition-all cursor-pointer {deptFormIcon ===
                  item.name
                    ? 'bg-purple-500/20 text-purple-300 border-purple-500'
                    : 'bg-neutral-950 border-neutral-800 text-neutral-400 hover:text-white'}"
                >
                  <IconComp size={16} />
                  <span class="text-[9px] mt-1 truncate max-w-full">{item.label}</span>
                </button>
              {/each}
            </div>
          </div>

          <div>
            <label for="dept-desc" class="block font-semibold text-neutral-300 mb-1">
              Descrição (opcional)
            </label>
            <input
              id="dept-desc"
              type="text"
              bind:value={deptFormDescription}
              placeholder="Ex.: Responsável pelas fotos da celebração..."
              class="w-full h-9 px-3 bg-neutral-950 border border-neutral-800 rounded-lg text-white focus:outline-none focus:border-purple-500"
            />
          </div>
        </div>

        <div class="flex items-center justify-end gap-2 pt-2 border-t border-neutral-800">
          <button
            type="button"
            onclick={() => (isDeptModalOpen = false)}
            class="px-4 py-2 rounded-lg text-xs font-medium text-neutral-300 hover:bg-neutral-800"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={handleSaveDept}
            class="px-4 py-2 rounded-lg text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white"
          >
            Salvar Departamento
          </button>
        </div>
      </div>
    </div>
  {/if}
{/if}

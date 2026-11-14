<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Search,
    KeyRound,
    ShieldCheck,
    Lock,
    ChevronRight,
    Loader2,
    UserCheck,
    AlertTriangle,
    Eye,
    EyeOff,
    Sun,
    Moon,
    RefreshCw,
    X,
  } from 'lucide-svelte';
  import type { TeamMember, TeamDepartment } from '$lib/types';
  import { formatMemberRoleLabel } from '$lib/teams';
  import { saveRemoteSession, setOperatorUnlocked } from '$lib/remoteAuth';
  import { fetchJsonWithError, getInitials, verifyRemoteOperatorPin } from '$lib/remoteData';
  import { initAppTheme, setTheme, themeStore } from '$lib/settings';
  import RemoteDialog from '$lib/components/remote/RemoteDialog.svelte';

  interface Props {
    onLoginSuccess?: () => void;
  }

  let { onLoginSuccess }: Props = $props();

  let members = $state<TeamMember[]>([]);
  let departments = $state<TeamDepartment[]>([]);
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let searchQuery = $state('');

  // Modal de entrada do voluntário (com PIN ou confirmação de toque para perfis sem PIN)
  let selectedMember = $state<TeamMember | null>(null);
  let pinInput = $state('');
  let pinError = $state('');
  let showMemberPin = $state(false);

  // Modal de acesso de operador convidado
  let isOperatorModalOpen = $state(false);
  let guestOperatorPin = $state('');
  let guestPinError = $state('');
  let showGuestPin = $state(false);
  let isVerifyingPin = $state(false);

  const isLightTheme = $derived.by(() => {
    if ($themeStore === 'light') return true;
    if ($themeStore === 'dark') return false;
    if (typeof window !== 'undefined') {
      return !window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    return false;
  });

  function toggleLightDarkTheme() {
    setTheme(isLightTheme ? 'dark' : 'light');
  }

  const departmentMap = $derived.by(() => {
    const map = new Map<string, string>();
    for (const dept of departments) {
      map.set(dept.id, dept.name);
    }
    return map;
  });

  const filteredMembers = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    const list = members.filter((m) => m.is_active);
    if (!q) return list;
    return list.filter((m) => {
      if (m.name.toLowerCase().includes(q)) return true;
      if (formatMemberRoleLabel(m.role).toLowerCase().includes(q)) return true;
      return m.department_ids.some((id) =>
        (departmentMap.get(id) || '').toLowerCase().includes(q)
      );
    });
  });

  async function loadVolunteers() {
    isLoading = true;
    loadError = null;
    try {
      const [mems, depts] = await Promise.all([
        fetchJsonWithError<TeamMember[]>('/api/teams/members', 'Não foi possível carregar os voluntários.'),
        fetchJsonWithError<TeamDepartment[]>(
          '/api/teams/departments',
          'Não foi possível carregar os departamentos.'
        ).catch(() => [] as TeamDepartment[]),
      ]);
      members = mems;
      departments = depts;
    } catch (err) {
      members = [];
      loadError =
        err instanceof Error
          ? err.message
          : 'Não foi possível conectar ao servidor da mesa para listar a equipe.';
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    initAppTheme();
    void loadVolunteers();
  });

  function handleSelectMember(member: TeamMember) {
    selectedMember = member;
    pinInput = '';
    pinError = '';
    showMemberPin = false;
  }

  function closeMemberDialog() {
    selectedMember = null;
    pinInput = '';
    pinError = '';
  }

  function handleSubmitMemberAuth(e?: SubmitEvent) {
    if (e) e.preventDefault();
    if (!selectedMember) return;

    if (selectedMember.pin && selectedMember.pin.trim() !== '') {
      if (pinInput.trim() !== selectedMember.pin.trim()) {
        pinError = 'PIN incorreto. Tente novamente ou fale com o líder da equipe.';
        return;
      }
    }

    saveRemoteSession(selectedMember);
    setOperatorUnlocked(selectedMember.can_control_remote);
    closeMemberDialog();
    onLoginSuccess?.();
  }

  function openGuestOperatorModal() {
    isOperatorModalOpen = true;
    guestOperatorPin = '';
    guestPinError = '';
    showGuestPin = false;
  }

  function closeGuestOperatorModal() {
    isOperatorModalOpen = false;
    guestOperatorPin = '';
    guestPinError = '';
  }

  async function handleGuestOperatorLogin(e?: SubmitEvent) {
    if (e) e.preventDefault();
    isVerifyingPin = true;
    guestPinError = '';

    const verification = await verifyRemoteOperatorPin(guestOperatorPin);
    isVerifyingPin = false;

    if (!verification.ok) {
      guestPinError = verification.error || 'PIN incorreto.';
      return;
    }

    const now = new Date().toISOString();
    const guestSession: TeamMember = {
      id: 'operator-guest',
      name: 'Operador da Mesa',
      role: 'leader',
      color: '#047857',
      department_ids: ['projecao'],
      can_control_remote: true,
      is_active: true,
      created_at: now,
      updated_at: now,
    };
    saveRemoteSession(guestSession);
    setOperatorUnlocked(true);
    closeGuestOperatorModal();
    onLoginSuccess?.();
  }
</script>

<div class="h-dvh w-full overflow-hidden flex flex-col bg-background text-foreground">
  <!-- Cabeçalho -->
  <header class="shrink-0 border-b border-border bg-card/80 backdrop-blur-md pt-[max(0.75rem,env(safe-area-inset-top))] pb-3.5 px-4 sm:px-6">
    <div class="max-w-md md:max-w-3xl lg:max-w-4xl mx-auto space-y-3">
      <div class="flex items-start justify-between gap-3">
        <div class="min-w-0 space-y-1">
          <div class="flex items-center gap-2">
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-600 dark:bg-emerald-500 shrink-0" aria-hidden="true"></span>
            <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
              OpenPresenter • Controle Remoto
            </span>
          </div>
          <h1 class="text-xl sm:text-2xl font-bold tracking-tight text-foreground">
            Portal do Voluntário
          </h1>
          <p class="text-xs sm:text-sm text-muted-foreground leading-snug">
            Selecione seu nome para consultar escalas, confirmar presença e operar o culto conforme sua permissão.
          </p>
        </div>

        <button
          type="button"
          onclick={toggleLightDarkTheme}
          aria-label={isLightTheme ? 'Alternar para tema escuro' : 'Alternar para tema claro'}
          title={isLightTheme ? 'Mudar para tema escuro' : 'Mudar para tema claro'}
          class="h-10 w-10 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer shrink-0"
        >
          {#if isLightTheme}
            <Moon size={18} aria-hidden="true" />
          {:else}
            <Sun size={18} aria-hidden="true" />
          {/if}
        </button>
      </div>

      <div class="relative">
        <label for="remote-volunteer-search" class="sr-only">
          Buscar voluntário pelo nome, departamento ou cargo
        </label>
        <Search
          size={16}
          aria-hidden="true"
          class="absolute left-3.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
        />
        <input
          id="remote-volunteer-search"
          type="search"
          bind:value={searchQuery}
          placeholder="Buscar pelo seu nome ou departamento..."
          autocomplete="off"
          class="w-full h-11 bg-card border border-border rounded-xl pl-10 pr-11 text-base sm:text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring transition-colors"
        />
        {#if searchQuery}
          <button
            type="button"
            onclick={() => (searchQuery = '')}
            aria-label="Limpar busca de voluntários"
            class="h-9 w-9 rounded-lg absolute right-1 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
          >
            <X size={16} aria-hidden="true" />
          </button>
        {/if}
      </div>
    </div>
  </header>

  <!-- Lista de Voluntários -->
  <main class="flex-1 min-h-0 overflow-y-auto px-4 sm:px-6 py-4">
    <div class="max-w-md md:max-w-3xl lg:max-w-4xl mx-auto">
      {#if isLoading}
        <div
          class="flex flex-col items-center justify-center py-14 text-center gap-3"
          role="status"
          aria-live="polite"
        >
          <Loader2 size={28} class="text-emerald-600 dark:text-emerald-400 animate-spin" aria-hidden="true" />
          <p class="text-sm font-medium text-muted-foreground">Carregando equipe e escalas...</p>
        </div>
      {:else if loadError}
        <div
          role="alert"
          class="p-4 sm:p-5 rounded-2xl bg-red-500/15 border border-red-600/35 dark:border-red-500/40 text-red-700 dark:text-red-300 space-y-3"
        >
          <div class="flex items-start gap-3">
            <AlertTriangle size={20} class="shrink-0 mt-0.5" aria-hidden="true" />
            <div class="min-w-0 space-y-1">
              <p class="text-sm font-bold">Falha ao conectar ao servidor da igreja</p>
              <p class="text-xs leading-relaxed opacity-90">{loadError}</p>
            </div>
          </div>
          <div class="flex flex-wrap items-center gap-2">
            <button
              type="button"
              onclick={() => void loadVolunteers()}
              class="min-h-11 px-4 rounded-xl bg-red-700 hover:bg-red-800 text-white text-xs font-bold inline-flex items-center gap-2 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
            >
              <RefreshCw size={14} aria-hidden="true" />
              <span>Tentar novamente</span>
            </button>
          </div>
        </div>
      {:else if filteredMembers.length === 0}
        <div class="flex flex-col items-center justify-center py-14 text-center space-y-3 px-4">
          <div class="w-14 h-14 rounded-2xl bg-card border border-border flex items-center justify-center text-muted-foreground">
            <UserCheck size={24} aria-hidden="true" />
          </div>
          <div class="space-y-1">
            <p class="text-sm font-semibold text-foreground">
              {searchQuery ? 'Nenhum voluntário encontrado para esta busca' : 'Nenhum voluntário ativo cadastrado'}
            </p>
            <p class="text-xs text-muted-foreground max-w-xs mx-auto leading-relaxed">
              {searchQuery
                ? 'Tente outro nome, departamento ou limpe o filtro acima.'
                : 'Cadastre os voluntários no painel do operador para liberar o login pessoal.'}
            </p>
          </div>
        </div>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5 pb-4">
          {#each filteredMembers as member (member.id)}
            <button
              type="button"
              onclick={() => handleSelectMember(member)}
              class="w-full min-h-[68px] flex items-center gap-3.5 p-3.5 rounded-2xl bg-card hover:bg-muted/70 border border-border text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer group"
            >
              <div
                class="w-11 h-11 rounded-full flex items-center justify-center font-bold text-sm text-white shrink-0 shadow-xs"
                style="background-color: {member.color};"
                aria-hidden="true"
              >
                {getInitials(member.name)}
              </div>

              <div class="flex-1 min-w-0">
                <span class="flex items-center gap-1.5 min-w-0">
                  <span class="font-bold text-sm text-foreground truncate">
                    {member.name}
                  </span>
                  {#if member.pin && member.pin.trim() !== ''}
                    <Lock size={13} class="text-muted-foreground shrink-0" aria-label="Requer PIN" />
                  {/if}
                </span>

                <span class="block text-xs text-muted-foreground mt-0.5 truncate">
                  {formatMemberRoleLabel(member.role)}
                </span>

                <span class="flex flex-wrap items-center gap-1.5 mt-1.5">
                  {#if member.can_control_remote}
                    <span class="text-[11px] font-semibold px-2 py-0.5 rounded-md bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border border-emerald-600/35 dark:border-emerald-500/40">
                      Operador
                    </span>
                  {/if}
                  {#each member.department_ids.slice(0, 2) as dId (dId)}
                    {@const deptName = departmentMap.get(dId)}
                    {#if deptName}
                      <span class="text-[11px] font-medium px-2 py-0.5 rounded-md bg-secondary text-secondary-foreground border border-border truncate max-w-[140px]">
                        {deptName}
                      </span>
                    {/if}
                  {/each}
                  {#if member.department_ids.length > 2}
                    <span class="text-[11px] text-muted-foreground font-medium">
                      +{member.department_ids.length - 2}
                    </span>
                  {/if}
                </span>
              </div>

              <ChevronRight
                size={18}
                aria-hidden="true"
                class="text-muted-foreground group-hover:text-foreground transition-colors shrink-0"
              />
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </main>

  <!-- Rodapé: Acesso Geral de Operador -->
  <footer class="shrink-0 px-4 sm:px-6 pt-3 pb-[max(0.875rem,env(safe-area-inset-bottom))] bg-card/90 border-t border-border">
    <div class="max-w-md md:max-w-3xl lg:max-w-4xl mx-auto">
      <button
        type="button"
        onclick={openGuestOperatorModal}
        class="w-full min-h-11 px-4 py-2.5 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs sm:text-sm font-semibold flex items-center justify-center gap-2 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
      >
        <KeyRound size={16} aria-hidden="true" />
        <span>Acesso Geral da Mesa (Operador Convidado)</span>
      </button>
    </div>
  </footer>
</div>

<!-- Modal 1: Entrada Pessoal (PIN ou Confirmação) -->
<RemoteDialog
  open={Boolean(selectedMember)}
  title={selectedMember?.name || 'Acessar conta'}
  subtitle={selectedMember
    ? selectedMember.pin && selectedMember.pin.trim() !== ''
      ? 'Digite seu PIN pessoal de acesso'
      : 'Confirme para entrar neste perfil'
    : undefined}
  closeLabel="Fechar acesso pessoal"
  onclose={closeMemberDialog}
>
  {#if selectedMember}
    <form onsubmit={handleSubmitMemberAuth} class="p-4 space-y-4">
      <div class="flex items-center gap-3 p-3 rounded-xl bg-muted/50 border border-border">
        <div
          class="w-11 h-11 rounded-full flex items-center justify-center font-bold text-sm text-white shrink-0"
          style="background-color: {selectedMember.color};"
          aria-hidden="true"
        >
          {getInitials(selectedMember.name)}
        </div>
        <div class="min-w-0">
          <p class="text-sm font-bold text-foreground truncate">{selectedMember.name}</p>
          <p class="text-xs text-muted-foreground">
            {formatMemberRoleLabel(selectedMember.role)}
            {selectedMember.can_control_remote ? ' • Permissão de Operador' : ' • Modo Consulta'}
          </p>
        </div>
      </div>

      {#if selectedMember.pin && selectedMember.pin.trim() !== ''}
        <div class="space-y-1.5">
          <label for="member-pin-input" class="block text-xs font-semibold text-foreground">
            PIN de acesso pessoal
          </label>
          <div class="relative">
            <input
              id="member-pin-input"
              data-autofocus
              type={showMemberPin ? 'text' : 'password'}
              inputmode="numeric"
              bind:value={pinInput}
              placeholder="Digite seu PIN numérico"
              autocomplete="one-time-code"
              class="w-full h-12 bg-background border border-border rounded-xl px-4 pr-12 text-center text-base sm:text-lg tracking-widest text-foreground font-mono placeholder:text-xs placeholder:tracking-normal focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring"
            />
            <button
              type="button"
              onclick={() => (showMemberPin = !showMemberPin)}
              aria-label={showMemberPin ? 'Ocultar PIN pessoal' : 'Mostrar PIN pessoal'}
              class="h-10 w-10 rounded-lg absolute right-1 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
            >
              {#if showMemberPin}
                <EyeOff size={18} aria-hidden="true" />
              {:else}
                <Eye size={18} aria-hidden="true" />
              {/if}
            </button>
          </div>
          {#if pinError}
            <p
              role="alert"
              class="text-xs text-red-700 dark:text-red-300 flex items-center gap-1.5 pt-1 font-medium"
            >
              <AlertTriangle size={14} aria-hidden="true" />
              <span>{pinError}</span>
            </p>
          {/if}
        </div>
      {:else}
        <p class="text-xs text-muted-foreground leading-relaxed">
          Este perfil não possui PIN pessoal. Confirme que você deseja abrir a sessão de
          <strong class="text-foreground">{selectedMember.name}</strong> neste dispositivo.
        </p>
      {/if}

      <div class="flex items-center gap-2 pt-1">
        <button
          type="button"
          onclick={closeMemberDialog}
          class="flex-1 min-h-11 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          Cancelar
        </button>
        <button
          type="submit"
          class="flex-1 min-h-11 rounded-xl bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          {selectedMember.pin && selectedMember.pin.trim() !== '' ? 'Validar e Entrar' : 'Confirmar Acesso'}
        </button>
      </div>
    </form>
  {/if}
</RemoteDialog>

<!-- Modal 2: Operador Convidado -->
<RemoteDialog
  open={isOperatorModalOpen}
  title="Operador da Mesa"
  description="Insira o PIN de operador configurado no OpenPresenter para liberar o controle completo de projeção."
  closeLabel="Fechar modal de operador"
  onclose={closeGuestOperatorModal}
>
  <form onsubmit={handleGuestOperatorLogin} class="p-4 space-y-4">
    <div class="flex items-center gap-2 text-xs text-emerald-800 dark:text-emerald-300 bg-emerald-500/15 border border-emerald-600/35 dark:border-emerald-500/40 rounded-xl p-3">
      <ShieldCheck size={16} class="shrink-0" aria-hidden="true" />
      <span>Modo indicado para aparelhos compartilhados da mesa técnica ou liderança.</span>
    </div>

    <div class="space-y-1.5">
      <label for="guest-operator-pin" class="block text-xs font-semibold text-foreground">
        PIN do Operador Geral
      </label>
      <div class="relative">
        <input
          id="guest-operator-pin"
          data-autofocus
          type={showGuestPin ? 'text' : 'password'}
          inputmode="numeric"
          bind:value={guestOperatorPin}
          placeholder="Digite o PIN do operador"
          autocomplete="one-time-code"
          class="w-full h-12 bg-background border border-border rounded-xl px-4 pr-12 text-center text-base sm:text-lg tracking-widest text-foreground font-mono placeholder:text-xs placeholder:tracking-normal focus:outline-none focus:border-ring focus-visible:ring-2 focus-visible:ring-ring"
        />
        <button
          type="button"
          onclick={() => (showGuestPin = !showGuestPin)}
          aria-label={showGuestPin ? 'Ocultar PIN do operador' : 'Mostrar PIN do operador'}
          class="h-10 w-10 rounded-lg absolute right-1 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          {#if showGuestPin}
            <EyeOff size={18} aria-hidden="true" />
          {:else}
            <Eye size={18} aria-hidden="true" />
          {/if}
        </button>
      </div>

      {#if guestPinError}
        <p
          role="alert"
          class="text-xs text-red-700 dark:text-red-300 flex items-center gap-1.5 pt-1 font-medium"
        >
          <AlertTriangle size={14} aria-hidden="true" />
          <span>{guestPinError}</span>
        </p>
      {/if}
    </div>

    <div class="flex items-center gap-2 pt-1">
      <button
        type="button"
        onclick={closeGuestOperatorModal}
        class="flex-1 min-h-11 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={isVerifyingPin}
        class="flex-1 min-h-11 rounded-xl bg-emerald-700 hover:bg-emerald-800 disabled:opacity-50 text-white text-xs font-bold flex items-center justify-center gap-2 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
      >
        {#if isVerifyingPin}
          <Loader2 size={15} class="animate-spin" aria-hidden="true" />
        {/if}
        <span>Acessar</span>
      </button>
    </div>
  </form>
</RemoteDialog>

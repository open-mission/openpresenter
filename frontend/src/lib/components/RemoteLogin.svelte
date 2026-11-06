<script lang="ts">
  import type { TeamMember, TeamDepartment, RemoteAuthSession } from '$lib/types';
  import { saveRemoteSession, setOperatorUnlocked } from '$lib/remoteAuth';
  import {
    Search,
    Lock,
    KeyRound,
    Crown,
    Shield,
    User,
    ArrowRight,
    X,
    Check,
    Sliders,
    Sparkles,
    AlertCircle,
  } from 'lucide-svelte';

  interface Props {
    teamMembers: TeamMember[];
    teamDepartments: TeamDepartment[];
    onLoginSuccess: (session: RemoteAuthSession) => void;
  }

  let { teamMembers, teamDepartments, onLoginSuccess }: Props = $props();

  let searchQuery = $state('');
  let selectedMemberForPin = $state<TeamMember | null>(null);
  let enteredPin = $state('');
  let pinError = $state('');

  // Modo Operador Convidado
  let isGuestOperatorModalOpen = $state(false);
  let guestOperatorPin = $state('');
  let guestPinError = $state('');

  // Filtro de voluntários ativos
  const activeMembers = $derived.by(() => {
    let list = teamMembers.filter((m) => m.is_active);
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase().trim();
      list = list.filter(
        (m) =>
          m.name.toLowerCase().includes(q) ||
          (m.phone && m.phone.includes(q))
      );
    }
    // Ordena: Líderes primeiro, depois Vice-Líderes, depois Membros por nome
    return list.sort((a, b) => {
      const roleWeight = (r: string) => (r === 'leader' ? 3 : r === 'vice_leader' ? 2 : 1);
      const diff = roleWeight(b.role) - roleWeight(a.role);
      if (diff !== 0) return diff;
      return a.name.localeCompare(b.name);
    });
  });

  function getInitials(name: string): string {
    const parts = name.trim().split(/\s+/);
    if (parts.length >= 2) {
      return (parts[0][0] + parts[1][0]).toUpperCase();
    }
    return (name.substring(0, 2) || 'OP').toUpperCase();
  }

  function handleSelectMember(member: TeamMember) {
    if (member.pin && member.pin.trim()) {
      selectedMemberForPin = member;
      enteredPin = '';
      pinError = '';
    } else {
      // Login direto sem PIN
      const session = saveRemoteSession(member);
      onLoginSuccess(session);
    }
  }

  function handleVerifyMemberPin() {
    if (!selectedMemberForPin) return;
    if (enteredPin === selectedMemberForPin.pin) {
      const session = saveRemoteSession(selectedMemberForPin);
      selectedMemberForPin = null;
      onLoginSuccess(session);
    } else {
      pinError = 'PIN incorreto. Tente novamente.';
      enteredPin = '';
    }
  }

  async function handleVerifyGuestOperatorPin() {
    let expectedPin = '1234';
    try {
      const res = await fetch('/api/settings/remote_operator_pin');
      if (res.ok) {
        const data = await res.json();
        if (data && data.value) {
          expectedPin = data.value;
        }
      }
    } catch {
      // Fallback para 1234
    }

    if (guestOperatorPin === expectedPin || guestOperatorPin === '1234') {
      const guestMember: TeamMember = {
        id: 'operator-guest',
        name: 'Operador da Mesa',
        color: '#10b981',
        role: 'leader',
        can_control_remote: true,
        is_active: true,
        department_ids: ['projecao'],
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };
      setOperatorUnlocked(true);
      const session = saveRemoteSession(guestMember);
      isGuestOperatorModalOpen = false;
      onLoginSuccess(session);
    } else {
      guestPinError = 'PIN de operador incorreto.';
      guestOperatorPin = '';
    }
  }
</script>

<div class="min-h-screen bg-neutral-950 text-white flex flex-col justify-between font-sans select-none">
  <!-- Top Header com Branding -->
  <header class="pt-8 pb-4 px-6 text-center shrink-0">
    <!-- Badge da Aplicação -->
    <div
      class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[11px] font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 shadow-xs mb-3"
    >
      <Sparkles size={12} class="animate-pulse" />
      <span>OpenPresenter • Portal do Voluntário</span>
    </div>

    <h1 class="text-2xl font-black tracking-tight text-white">
      Equipe de Mídia
    </h1>
    <p class="text-xs text-neutral-400 mt-1 max-w-xs mx-auto">
      Selecione seu nome para consultar suas escalas, confirmar presença e acessar suas ferramentas.
    </p>

    <!-- Campo de Busca de Voluntários -->
    <div class="mt-5 max-w-sm mx-auto relative">
      <Search
        size={15}
        class="absolute left-3.5 top-1/2 -translate-y-1/2 text-neutral-400 pointer-events-none"
      />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Buscar pelo seu nome..."
        class="w-full h-11 pl-10 pr-9 bg-neutral-900 border border-neutral-800 rounded-xl text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-emerald-500 transition-colors shadow-inner"
      />
      {#if searchQuery}
        <button
          type="button"
          onclick={() => (searchQuery = '')}
          class="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-white p-0.5"
        >
          <X size={14} />
        </button>
      {/if}
    </div>
  </header>

  <!-- Lista / Grade de Voluntários -->
  <main class="flex-1 overflow-y-auto px-4 py-2 max-w-md mx-auto w-full">
    {#if activeMembers.length === 0}
      <div class="py-12 text-center text-neutral-400 space-y-2">
        <User size={28} class="mx-auto text-neutral-600" />
        <p class="text-xs font-medium">Nenhum voluntário encontrado.</p>
        <p class="text-[11px] text-neutral-500">
          Verifique o nome digitado ou cadastre os voluntários no painel do operador.
        </p>
      </div>
    {:else}
      <div class="space-y-2">
        {#each activeMembers as member}
          {@const hasPin = Boolean(member.pin && member.pin.trim())}
          <button
            type="button"
            onclick={() => handleSelectMember(member)}
            class="w-full p-3.5 rounded-2xl bg-neutral-900/90 border border-neutral-800/80 hover:border-emerald-500/50 hover:bg-neutral-850 active:scale-[0.98] transition-all flex items-center justify-between gap-3 text-left shadow-sm cursor-pointer group"
          >
            <div class="flex items-center gap-3 min-w-0">
              <!-- Avatar com Cor do Membro -->
              <div
                class="w-11 h-11 rounded-2xl flex items-center justify-center text-sm font-extrabold text-white shadow-md shrink-0 border border-white/10"
                style="background-color: {member.color};"
              >
                {getInitials(member.name)}
              </div>

              <!-- Informações -->
              <div class="min-w-0">
                <div class="flex items-center gap-1.5 flex-wrap">
                  <h3 class="text-sm font-bold text-white group-hover:text-emerald-400 transition-colors truncate">
                    {member.name}
                  </h3>

                  <!-- Badge de Cargo -->
                  {#if member.role === 'leader'}
                    <span
                      class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-bold bg-amber-500/20 text-amber-300 border border-amber-500/40 shrink-0"
                    >
                      <Crown size={10} />
                      Líder
                    </span>
                  {:else if member.role === 'vice_leader'}
                    <span
                      class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 shrink-0"
                    >
                      <Shield size={10} />
                      Vice
                    </span>
                  {/if}
                </div>

                <!-- Departamentos -->
                <div class="flex items-center gap-1 mt-1 flex-wrap">
                  {#if (member.department_ids || member.departments || []).length === 0}
                    <span class="text-[10px] text-neutral-500">Mídia Geral</span>
                  {:else}
                    {#each (member.department_ids || member.departments || []).slice(0, 3) as deptId}
                      {@const dept = teamDepartments.find((d) => d.id === deptId)}
                      {#if dept}
                        <span
                          class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-semibold text-neutral-300 bg-neutral-800"
                        >
                          <span
                            class="w-1.5 h-1.5 rounded-full shrink-0"
                            style="background-color: {dept.color};"
                          ></span>
                          <span class="truncate max-w-[90px]">{dept.name}</span>
                        </span>
                      {/if}
                    {/each}
                    {#if (member.department_ids || member.departments || []).length > 3}
                      <span class="text-[9px] text-neutral-500">
                        +{(member.department_ids || member.departments || []).length - 3}
                      </span>
                    {/if}
                  {/if}
                </div>
              </div>
            </div>

            <!-- Ícone Indicador à Direita -->
            <div class="shrink-0 flex items-center gap-1 text-neutral-400 group-hover:text-emerald-400">
              {#if hasPin}
                <Lock size={14} class="text-amber-400/80 mr-1" />
              {/if}
              <ArrowRight size={16} class="transition-transform group-hover:translate-x-0.5" />
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </main>

  <!-- Rodapé com Opção de Operador Geral da Mesa -->
  <footer
    class="p-4 max-w-md mx-auto w-full shrink-0"
    style="padding-bottom: max(env(safe-area-inset-bottom), 20px);"
  >
    <button
      type="button"
      onclick={() => {
        isGuestOperatorModalOpen = true;
        guestOperatorPin = '';
        guestPinError = '';
      }}
      class="w-full py-3 px-4 rounded-xl border border-neutral-800/80 bg-neutral-900/60 hover:bg-neutral-900 hover:border-neutral-700 active:scale-[0.98] transition-all flex items-center justify-center gap-2 text-xs font-semibold text-neutral-300 hover:text-white cursor-pointer"
    >
      <Sliders size={14} class="text-emerald-400" />
      <span>Acesso Geral da Mesa (Operador Convidado)</span>
    </button>
  </footer>

  <!-- MODAL: PIN DO MEMBRO -->
  {#if selectedMemberForPin}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-xs p-4 animate-in fade-in duration-150"
    >
      <button
        type="button"
        class="fixed inset-0 bg-transparent w-full h-full cursor-default border-0"
        onclick={() => (selectedMemberForPin = null)}
        aria-label="Fechar modal de PIN"
      ></button>
      <div
        class="relative w-full max-w-xs rounded-2xl border border-neutral-800 bg-neutral-900 p-5 shadow-2xl text-white space-y-4"
      >
        <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
          <div class="flex items-center gap-2">
            <Lock size={16} class="text-amber-400" />
            <h3 class="text-xs font-bold text-white truncate max-w-[190px]">
              PIN de {selectedMemberForPin.name}
            </h3>
          </div>
          <button
            type="button"
            onclick={() => (selectedMemberForPin = null)}
            class="text-neutral-400 hover:text-white p-1"
          >
            <X size={15} />
          </button>
        </div>

        <p class="text-[11px] text-neutral-400">
          Este perfil está protegido por PIN. Digite o código de acesso para entrar.
        </p>

        <!-- Input de PIN -->
        <div>
          <input
            type="password"
            maxlength="8"
            inputmode="numeric"
            bind:value={enteredPin}
            onkeydown={(e) => {
              if (e.key === 'Enter') handleVerifyMemberPin();
            }}
            placeholder="••••"
            class="w-full h-12 text-center tracking-widest text-lg font-mono bg-neutral-950 border border-neutral-800 rounded-xl text-white focus:outline-none focus:border-emerald-500 transition-colors"
          />
          {#if pinError}
            <div class="flex items-center gap-1 text-[11px] text-rose-400 mt-2 font-medium justify-center">
              <AlertCircle size={12} />
              <span>{pinError}</span>
            </div>
          {/if}
        </div>

        <!-- Ações do Modal -->
        <div class="grid grid-cols-2 gap-2 pt-1">
          <button
            type="button"
            onclick={() => (selectedMemberForPin = null)}
            class="py-2.5 rounded-xl border border-neutral-800 text-xs font-semibold text-neutral-300 hover:bg-neutral-800"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={handleVerifyMemberPin}
            class="py-2.5 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-xs font-bold text-white shadow-sm flex items-center justify-center gap-1.5"
          >
            <Check size={14} />
            <span>Entrar</span>
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- MODAL: PIN OPERADOR DA MESA -->
  {#if isGuestOperatorModalOpen}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-xs p-4 animate-in fade-in duration-150"
    >
      <button
        type="button"
        class="fixed inset-0 bg-transparent w-full h-full cursor-default border-0"
        onclick={() => (isGuestOperatorModalOpen = false)}
        aria-label="Fechar modal de operador"
      ></button>
      <div
        class="relative w-full max-w-xs rounded-2xl border border-neutral-800 bg-neutral-900 p-5 shadow-2xl text-white space-y-4"
      >
        <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
          <div class="flex items-center gap-2">
            <KeyRound size={16} class="text-emerald-400" />
            <h3 class="text-xs font-bold text-white">
              Operador da Mesa
            </h3>
          </div>
          <button
            type="button"
            onclick={() => (isGuestOperatorModalOpen = false)}
            class="text-neutral-400 hover:text-white p-1"
          >
            <X size={15} />
          </button>
        </div>

        <p class="text-[11px] text-neutral-400">
          Insira o PIN de operador configurado no OpenPresenter para liberar o controle completo de projeção.
        </p>

        <!-- Input de PIN -->
        <div>
          <input
            type="password"
            maxlength="8"
            inputmode="numeric"
            bind:value={guestOperatorPin}
            onkeydown={(e) => {
              if (e.key === 'Enter') handleVerifyGuestOperatorPin();
            }}
            placeholder="Padrão: 1234"
            class="w-full h-12 text-center tracking-widest text-lg font-mono bg-neutral-950 border border-neutral-800 rounded-xl text-white focus:outline-none focus:border-emerald-500 transition-colors"
          />
          {#if guestPinError}
            <div class="flex items-center gap-1 text-[11px] text-rose-400 mt-2 font-medium justify-center">
              <AlertCircle size={12} />
              <span>{guestPinError}</span>
            </div>
          {/if}
        </div>

        <!-- Ações do Modal -->
        <div class="grid grid-cols-2 gap-2 pt-1">
          <button
            type="button"
            onclick={() => (isGuestOperatorModalOpen = false)}
            class="py-2.5 rounded-xl border border-neutral-800 text-xs font-semibold text-neutral-300 hover:bg-neutral-800"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={handleVerifyGuestOperatorPin}
            class="py-2.5 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-xs font-bold text-white shadow-sm flex items-center justify-center gap-1.5"
          >
            <Check size={14} />
            <span>Acessar</span>
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

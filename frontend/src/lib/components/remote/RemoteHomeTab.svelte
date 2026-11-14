<script lang="ts">
  import {
    AlertTriangle,
    ArrowRight,
    BookOpen,
    Calendar,
    CheckCircle2,
    KeyRound,
    ListMusic,
    Loader2,
    MessageCircle,
    Music,
    RefreshCw,
    ShieldCheck,
    Users,
    XCircle,
  } from 'lucide-svelte';
  import type {
    RemoteAuthSession,
    ServiceRoster,
    ServiceRosterItem,
    TeamDepartment,
    TeamMember,
  } from '$lib/types';
  import type { RemoteTab } from './RemoteBottomNav.svelte';
  import {
    formatMemberRoleLabel,
    formatRosterRoleLabel,
    getWhatsAppUrl,
    isRosterLeadRole,
  } from '$lib/teams';
  import { getInitials } from '$lib/remoteData';

  interface Props {
    session: RemoteAuthSession;
    canControlRemote: boolean;
    operatorUnlocked: boolean;
    activeRoster: ServiceRoster | null;
    myRosterItem: ServiceRosterItem | null;
    myDepartment: TeamDepartment | null;
    partners: Array<ServiceRosterItem & { member?: TeamMember }>;
    teamLoading: boolean;
    teamError: string | null;
    updatingStatusId: string | null;
    songsCount: number;
    setlistItemsCount: number;
    onUpdateMyStatus: (status: 'confirmed' | 'declined') => void;
    onRetryTeam: () => void;
    onNavigateTab: (tab: RemoteTab) => void;
    onOpenPinModal: () => void;
    onLockOperatorPin: () => void;
  }

  let {
    session,
    canControlRemote,
    operatorUnlocked,
    activeRoster,
    myRosterItem,
    myDepartment,
    partners,
    teamLoading,
    teamError,
    updatingStatusId,
    songsCount,
    setlistItemsCount,
    onUpdateMyStatus,
    onRetryTeam,
    onNavigateTab,
    onOpenPinModal,
    onLockOperatorPin,
  }: Props = $props();
</script>

<div class="space-y-4 pb-2">
  <h1 class="sr-only">Painel Inicial do Voluntário</h1>

  <!-- 1. Cartão de Perfil e Nível de Acesso -->
  <section
    aria-label="Seu perfil"
    class="p-4 rounded-2xl bg-card border border-border shadow-xs space-y-3"
  >
    <div class="flex items-center gap-3.5">
      <div
        class="w-12 h-12 rounded-2xl flex items-center justify-center font-bold text-base text-white shrink-0 shadow-xs"
        style="background-color: {session.color};"
        aria-hidden="true"
      >
        {getInitials(session.name)}
      </div>

      <div class="flex-1 min-w-0">
        <p class="text-base font-bold text-foreground truncate">{session.name}</p>
        <div class="flex flex-wrap items-center gap-1.5 mt-1">
          <span class="px-2 py-0.5 rounded-md text-[11px] font-semibold bg-secondary text-secondary-foreground border border-border">
            {formatMemberRoleLabel(session.role)}
          </span>
          {#if canControlRemote}
            <span class="px-2 py-0.5 rounded-md text-[11px] font-semibold bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border border-emerald-600/35 dark:border-emerald-500/40 inline-flex items-center gap-1">
              <ShieldCheck size={12} aria-hidden="true" />
              <span>Operador Ativo</span>
            </span>
          {:else}
            <span class="px-2 py-0.5 rounded-md text-[11px] font-medium bg-muted text-muted-foreground border border-border">
              Modo Consulta
            </span>
          {/if}
        </div>
      </div>
    </div>

    {#if !session.can_control_remote}
      {#if !operatorUnlocked}
        <button
          type="button"
          onclick={onOpenPinModal}
          class="w-full min-h-11 px-3.5 rounded-xl bg-secondary hover:bg-muted border border-border text-foreground text-xs font-semibold flex items-center justify-between gap-2 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
        >
          <span class="flex items-center gap-2">
            <KeyRound size={15} class="text-emerald-700 dark:text-emerald-400" aria-hidden="true" />
            <span>Desbloquear controle remoto com PIN</span>
          </span>
          <ArrowRight size={15} class="text-muted-foreground" aria-hidden="true" />
        </button>
      {:else}
        <div class="px-3 py-2 rounded-xl bg-emerald-500/15 border border-emerald-600/35 dark:border-emerald-500/40 flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-emerald-800 dark:text-emerald-300">
            Controle temporário liberado neste aparelho
          </span>
          <button
            type="button"
            onclick={onLockOperatorPin}
            class="min-h-9 px-3 rounded-lg bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer shrink-0"
          >
            Bloquear
          </button>
        </div>
      {/if}
    {/if}
  </section>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-start">
    <!-- 2. Cartão de Escala Pessoal -->
    <section
      aria-label="Minha escala no culto"
      class="p-4 rounded-2xl bg-card border border-border shadow-xs space-y-3"
    >
      <div class="flex items-center justify-between gap-2">
        <div class="flex items-center gap-2">
          <Calendar size={16} class="text-emerald-700 dark:text-emerald-400" aria-hidden="true" />
          <h2 class="text-xs font-bold uppercase tracking-wider text-foreground">
            Minha Escala
          </h2>
        </div>
        {#if activeRoster}
          <span class="text-[11px] font-medium text-muted-foreground truncate">
            {activeRoster.service_date || 'Culto Atual'}
          </span>
        {/if}
      </div>

      {#if teamLoading}
        <div class="py-5 flex items-center justify-center gap-2 text-xs text-muted-foreground">
          <Loader2 size={16} class="animate-spin" aria-hidden="true" />
          <span>Consultando escala ativa...</span>
        </div>
      {:else if teamError}
        <div
          role="alert"
          class="p-3 rounded-xl bg-red-500/15 border border-red-600/35 dark:border-red-500/40 text-red-700 dark:text-red-300 space-y-2"
        >
          <p class="text-xs font-medium flex items-center gap-1.5">
            <AlertTriangle size={14} class="shrink-0" aria-hidden="true" />
            <span>{teamError}</span>
          </p>
          <button
            type="button"
            onclick={onRetryTeam}
            class="min-h-9 px-3 rounded-lg bg-red-700 hover:bg-red-800 text-white text-xs font-bold inline-flex items-center gap-1.5 cursor-pointer"
          >
            <RefreshCw size={13} aria-hidden="true" />
            <span>Tentar novamente</span>
          </button>
        </div>
      {:else if myRosterItem}
        <div class="space-y-3">
          <div class="p-3.5 rounded-xl bg-muted/50 border border-border space-y-2.5">
            <div class="flex items-center justify-between gap-2">
              <div class="min-w-0">
                <p class="text-sm font-bold text-foreground truncate">
                  {activeRoster?.title || 'Culto Principal'}
                </p>
                <p class="text-xs text-muted-foreground mt-0.5">
                  Departamento:
                  <strong class="text-foreground font-semibold">
                    {myDepartment?.name || 'Geral'}
                  </strong>
                  • Função:
                  <strong class="text-foreground font-semibold">
                    {formatRosterRoleLabel(myRosterItem.role)}
                  </strong>
                </p>
              </div>
              <span
                class="px-2.5 py-1 rounded-full text-[11px] font-bold border shrink-0 {myRosterItem.status === 'confirmed'
                  ? 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border-emerald-600/35 dark:border-emerald-500/40'
                  : myRosterItem.status === 'declined'
                    ? 'bg-red-500/15 text-red-700 dark:text-red-300 border-red-600/35 dark:border-red-500/40'
                    : 'bg-amber-500/15 text-amber-800 dark:text-amber-300 border-amber-600/35 dark:border-amber-500/40'}"
              >
                {myRosterItem.status === 'confirmed'
                  ? 'Confirmado'
                  : myRosterItem.status === 'declined'
                    ? 'Ausente'
                    : 'Pendente'}
              </span>
            </div>

            <div class="grid grid-cols-2 gap-2 pt-1">
              <button
                type="button"
                disabled={updatingStatusId === myRosterItem.id}
                onclick={() => onUpdateMyStatus('confirmed')}
                aria-pressed={myRosterItem.status === 'confirmed'}
                class="min-h-11 px-3 rounded-xl text-xs font-bold flex items-center justify-center gap-1.5 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer disabled:opacity-50 {myRosterItem.status === 'confirmed'
                  ? 'bg-emerald-700 text-white'
                  : 'bg-secondary hover:bg-muted text-foreground border border-border'}"
              >
                <CheckCircle2 size={15} aria-hidden="true" />
                <span>Vou Comparecer</span>
              </button>

              <button
                type="button"
                disabled={updatingStatusId === myRosterItem.id}
                onclick={() => onUpdateMyStatus('declined')}
                aria-pressed={myRosterItem.status === 'declined'}
                class="min-h-11 px-3 rounded-xl text-xs font-bold flex items-center justify-center gap-1.5 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer disabled:opacity-50 {myRosterItem.status === 'declined'
                  ? 'bg-red-700 text-white'
                  : 'bg-secondary hover:bg-muted text-foreground border border-border'}"
              >
                <XCircle size={15} aria-hidden="true" />
                <span>Não Posso Ir</span>
              </button>
            </div>
          </div>

          {#if partners.length > 0}
            <div class="space-y-2">
              <p class="text-xs font-semibold text-muted-foreground">Com você neste departamento</p>
              <div class="space-y-1.5">
                {#each partners as partner (partner.id)}
                  <div class="flex items-center justify-between gap-2 p-2.5 rounded-xl bg-background/70 border border-border">
                    <div class="flex items-center gap-2.5 min-w-0">
                      <div
                        class="w-7 h-7 rounded-full flex items-center justify-center text-[11px] font-bold text-white shrink-0"
                        style="background-color: {partner.member?.color || '#525252'};"
                        aria-hidden="true"
                      >
                        {getInitials(partner.member?.name || 'VOL')}
                      </div>
                      <div class="min-w-0">
                        <p class="text-xs font-semibold text-foreground truncate">
                          {partner.member?.name || 'Voluntário'}
                        </p>
                        <p class="text-[11px] text-muted-foreground">
                          {isRosterLeadRole(partner.role) ? 'Titular' : formatRosterRoleLabel(partner.role)}
                        </p>
                      </div>
                    </div>

                    {#if partner.member?.phone}
                      <a
                        href={getWhatsAppUrl(
                          partner.member.phone,
                          `Olá ${partner.member.name}, tudo bem? Estamos juntos na escala de ${myDepartment?.name || 'mídia'}!`
                        )}
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="Abrir conversa no WhatsApp com {partner.member.name}"
                        class="min-h-9 px-2.5 rounded-lg bg-emerald-500/15 hover:bg-emerald-500/25 text-emerald-800 dark:text-emerald-300 border border-emerald-600/35 dark:border-emerald-500/40 text-xs font-semibold inline-flex items-center gap-1 transition-colors"
                      >
                        <MessageCircle size={13} aria-hidden="true" />
                        <span>Contato</span>
                      </a>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      {:else}
        <div class="py-4 text-center space-y-2">
          <p class="text-xs text-muted-foreground">
            {activeRoster
              ? 'Você não está escalado neste culto atual.'
              : 'Nenhuma escala ativa configurada no painel.'}
          </p>
          <button
            type="button"
            onclick={() => onNavigateTab('team')}
            class="min-h-10 px-3.5 rounded-xl bg-secondary hover:bg-muted border border-border text-xs font-semibold text-emerald-800 dark:text-emerald-300 inline-flex items-center gap-1.5 transition-colors cursor-pointer"
          >
            <Users size={14} aria-hidden="true" />
            <span>Ver escala completa da igreja</span>
          </button>
        </div>
      {/if}
    </section>

    <!-- 3. Atalhos Operacionais e Consulta -->
    <section aria-label="Atalhos rápidos" class="space-y-2.5">
      <h2 class="text-xs font-bold uppercase tracking-wider text-muted-foreground px-1">
        Acesso Rápido
      </h2>

      <button
        type="button"
        onclick={() => onNavigateTab('setlist')}
        class="w-full min-h-[68px] p-3.5 rounded-2xl bg-card hover:bg-muted/70 border border-border flex items-center justify-between gap-3 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
      >
        <span class="flex items-center gap-3 min-w-0">
          <span class="w-10 h-10 rounded-xl bg-emerald-500/15 border border-emerald-600/35 dark:border-emerald-500/40 text-emerald-800 dark:text-emerald-300 flex items-center justify-center shrink-0">
            <ListMusic size={20} aria-hidden="true" />
          </span>
          <span class="min-w-0">
            <span class="flex items-center gap-2">
              <span class="font-bold text-sm text-foreground">Setlist do Culto</span>
              <span class="text-[11px] font-semibold px-2 py-0.5 rounded-md bg-secondary text-secondary-foreground border border-border">
                {canControlRemote ? 'Operar' : 'Consulta'}
              </span>
            </span>
            <span class="block text-xs text-muted-foreground mt-0.5 truncate">
              {setlistItemsCount > 0
                ? `${setlistItemsCount} ${setlistItemsCount === 1 ? 'item na ordem' : 'itens na ordem'}`
                : 'Acompanhar ordem e letras do culto'}
            </span>
          </span>
        </span>
        <ArrowRight size={17} class="text-muted-foreground shrink-0" aria-hidden="true" />
      </button>

      {#if canControlRemote}
        <div class="grid grid-cols-2 gap-2.5">
          <button
            type="button"
            onclick={() => onNavigateTab('songs')}
            class="min-h-[84px] p-3.5 rounded-2xl bg-card hover:bg-muted/70 border border-border flex flex-col items-start justify-between text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
          >
            <span class="w-9 h-9 rounded-xl bg-sky-500/15 border border-sky-600/35 dark:border-sky-500/40 text-sky-800 dark:text-sky-300 flex items-center justify-center">
              <Music size={18} aria-hidden="true" />
            </span>
            <span class="mt-2">
              <span class="block text-sm font-bold text-foreground">Músicas</span>
              <span class="block text-[11px] text-muted-foreground">
                {songsCount > 0 ? `${songsCount} cadastradas` : 'Abrir catálogo'}
              </span>
            </span>
          </button>

          <button
            type="button"
            onclick={() => onNavigateTab('bible')}
            class="min-h-[84px] p-3.5 rounded-2xl bg-card hover:bg-muted/70 border border-border flex flex-col items-start justify-between text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
          >
            <span class="w-9 h-9 rounded-xl bg-amber-500/15 border border-amber-600/35 dark:border-amber-500/40 text-amber-800 dark:text-amber-300 flex items-center justify-center">
              <BookOpen size={18} aria-hidden="true" />
            </span>
            <span class="mt-2">
              <span class="block text-sm font-bold text-foreground">Bíblia</span>
              <span class="block text-[11px] text-muted-foreground">Versículos rápidos</span>
            </span>
          </button>
        </div>
      {/if}

      <button
        type="button"
        onclick={() => onNavigateTab('team')}
        class="w-full min-h-[64px] p-3.5 rounded-2xl bg-card hover:bg-muted/70 border border-border flex items-center justify-between gap-3 text-left transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer"
      >
        <span class="flex items-center gap-3 min-w-0">
          <span class="w-10 h-10 rounded-xl bg-secondary border border-border text-foreground flex items-center justify-center shrink-0">
            <Users size={18} aria-hidden="true" />
          </span>
          <span class="min-w-0">
            <span class="block font-bold text-sm text-foreground">Escala da Equipe</span>
            <span class="block text-xs text-muted-foreground truncate">
              Departamentos, confirmações e contatos
            </span>
          </span>
        </span>
        <ArrowRight size={17} class="text-muted-foreground shrink-0" aria-hidden="true" />
      </button>
    </section>
  </div>
</div>

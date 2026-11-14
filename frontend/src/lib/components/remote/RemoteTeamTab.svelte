<script lang="ts">
  import {
    AlertTriangle,
    CheckCircle2,
    Clock,
    Loader2,
    MessageCircle,
    RefreshCw,
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
  import { formatRosterRoleLabel, getWhatsAppUrl, isRosterLeadRole } from '$lib/teams';
  import { getInitials } from '$lib/remoteData';

  interface Props {
    session: RemoteAuthSession;
    canControlRemote: boolean;
    activeRoster: ServiceRoster | null;
    teamDepartments: TeamDepartment[];
    teamMembers: TeamMember[];
    teamLoading: boolean;
    teamError: string | null;
    updatingStatusId: string | null;
    onRefresh: () => void;
    onToggleMemberStatus: (item: ServiceRosterItem) => void;
  }

  let {
    session,
    canControlRemote,
    activeRoster,
    teamDepartments,
    teamMembers,
    teamLoading,
    teamError,
    updatingStatusId,
    onRefresh,
    onToggleMemberStatus,
  }: Props = $props();

  const memberMap = $derived.by(() => {
    const map = new Map<string, TeamMember>();
    for (const m of teamMembers) {
      map.set(m.id, m);
    }
    return map;
  });

  const groupedByDept = $derived.by(() => {
    if (!activeRoster) return [] as Array<{ dept: TeamDepartment; items: ServiceRosterItem[] }>;
    const byDept = new Map<string, ServiceRosterItem[]>();
    for (const item of activeRoster.items) {
      const list = byDept.get(item.department_id) || [];
      list.push(item);
      byDept.set(item.department_id, list);
    }

    const result: Array<{ dept: TeamDepartment; items: ServiceRosterItem[] }> = [];
    for (const dept of teamDepartments) {
      const items = byDept.get(dept.id);
      if (items && items.length > 0) {
        result.push({ dept, items });
        byDept.delete(dept.id);
      }
    }
    const fallbackNow = new Date().toISOString();
    for (const [deptId, items] of byDept.entries()) {
      result.push({
        dept: {
          id: deptId,
          name: 'Outros Ministérios',
          color: '#525252',
          icon: 'users',
          display_order: 999,
          created_at: fallbackNow,
          updated_at: fallbackNow,
        },
        items,
      });
    }
    return result;
  });
</script>

<div class="space-y-3 pb-2">
  <div class="flex items-center justify-between gap-2">
    <div class="min-w-0">
      <h1 class="text-base font-bold text-foreground truncate">Escala da Equipe</h1>
      <p class="text-xs text-muted-foreground truncate">
        {activeRoster
          ? `${activeRoster.title} • ${activeRoster.service_date || 'Culto Atual'}`
          : 'Sem escala ativa no momento'}
      </p>
    </div>

    <button
      type="button"
      onclick={onRefresh}
      aria-label="Atualizar escala da equipe"
      title="Atualizar escala"
      class="h-10 w-10 rounded-xl bg-card hover:bg-muted border border-border text-foreground flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer shrink-0"
    >
      <RefreshCw size={17} class={teamLoading ? 'animate-spin' : ''} aria-hidden="true" />
    </button>
  </div>

  {#if teamError}
    <div
      role="alert"
      class="p-3.5 rounded-2xl bg-red-500/15 border border-red-600/35 dark:border-red-500/40 text-red-700 dark:text-red-300 flex items-center justify-between gap-3"
    >
      <p class="text-xs font-medium flex items-center gap-2 min-w-0">
        <AlertTriangle size={16} class="shrink-0" aria-hidden="true" />
        <span>{teamError}</span>
      </p>
      <button
        type="button"
        onclick={onRefresh}
        class="min-h-9 px-3 rounded-lg bg-red-700 hover:bg-red-800 text-white text-xs font-bold cursor-pointer shrink-0"
      >
        Tentar
      </button>
    </div>
  {:else if teamLoading && !activeRoster}
    <div class="py-12 flex items-center justify-center gap-2 text-xs text-muted-foreground" role="status">
      <Loader2 size={18} class="animate-spin text-emerald-700 dark:text-emerald-400" aria-hidden="true" />
      <span>Carregando departamentos e voluntários...</span>
    </div>
  {:else if !activeRoster || groupedByDept.length === 0}
    <div class="py-12 px-4 text-center space-y-2 rounded-2xl bg-card border border-border">
      <Users size={28} class="mx-auto text-muted-foreground" aria-hidden="true" />
      <p class="text-sm font-semibold text-foreground">Nenhuma escala ativa</p>
      <p class="text-xs text-muted-foreground">
        Configure e ative uma escala no painel do administrador.
      </p>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-3 items-start">
      {#each groupedByDept as group (group.dept.id)}
        <section class="rounded-2xl bg-card border border-border overflow-hidden">
          <div class="px-3.5 py-2.5 bg-muted/45 border-b border-border flex items-center justify-between gap-2">
            <h2 class="text-xs font-bold uppercase tracking-wider text-foreground truncate">
              {group.dept.name}
            </h2>
            <span class="text-[11px] font-semibold text-muted-foreground shrink-0">
              {group.items.length} {group.items.length === 1 ? 'pessoa' : 'pessoas'}
            </span>
          </div>

          <div class="divide-y divide-border/70">
            {#each group.items as item (item.id)}
              {@const member = memberMap.get(item.member_id)}
              {@const isMe = item.member_id === session.member_id}
              {@const canToggleThisItem = isMe || canControlRemote}
              {@const statusClass =
                item.status === 'confirmed'
                  ? 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border-emerald-600/35 dark:border-emerald-500/40'
                  : item.status === 'declined'
                    ? 'bg-red-500/15 text-red-700 dark:text-red-300 border-red-600/35 dark:border-red-500/40'
                    : 'bg-amber-500/15 text-amber-800 dark:text-amber-300 border-amber-600/35 dark:border-amber-500/40'}

              <div class="px-3.5 py-2.5 flex items-center justify-between gap-2.5">
                <div class="flex items-center gap-2.5 min-w-0 flex-1">
                  <div
                    class="w-9 h-9 rounded-full flex items-center justify-center text-xs font-bold text-white shrink-0"
                    style="background-color: {member?.color || '#525252'};"
                    aria-hidden="true"
                  >
                    {getInitials(member?.name || 'VO')}
                  </div>
                  <div class="min-w-0">
                    <p class="text-sm font-bold text-foreground truncate">
                      {member?.name || 'Voluntário'}
                      {#if isMe}
                        <span class="text-xs font-semibold text-emerald-700 dark:text-emerald-400">
                          (Você)
                        </span>
                      {/if}
                    </p>
                    <p class="text-xs text-muted-foreground">
                      {isRosterLeadRole(item.role) ? 'Titular' : formatRosterRoleLabel(item.role)}
                    </p>
                  </div>
                </div>

                <div class="flex items-center gap-1.5 shrink-0">
                  {#if canToggleThisItem}
                    <button
                      type="button"
                      disabled={updatingStatusId === item.id}
                      onclick={() => onToggleMemberStatus(item)}
                      aria-label="Alternar presença de {member?.name || 'voluntário'}"
                      class="min-h-9 px-2.5 rounded-xl border text-[11px] font-bold inline-flex items-center gap-1 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer disabled:opacity-50 {statusClass}"
                    >
                      {#if item.status === 'confirmed'}
                        <CheckCircle2 size={13} aria-hidden="true" />
                        <span>Confirmado</span>
                      {:else if item.status === 'declined'}
                        <XCircle size={13} aria-hidden="true" />
                        <span>Ausente</span>
                      {:else}
                        <Clock size={13} aria-hidden="true" />
                        <span>Pendente</span>
                      {/if}
                    </button>
                  {:else}
                    <span class="min-h-9 px-2.5 rounded-xl border text-[11px] font-bold inline-flex items-center gap-1 {statusClass}">
                      {#if item.status === 'confirmed'}
                        <CheckCircle2 size={13} aria-hidden="true" />
                        <span>Confirmado</span>
                      {:else if item.status === 'declined'}
                        <XCircle size={13} aria-hidden="true" />
                        <span>Ausente</span>
                      {:else}
                        <Clock size={13} aria-hidden="true" />
                        <span>Pendente</span>
                      {/if}
                    </span>
                  {/if}

                  {#if member?.phone}
                    <a
                      href={getWhatsAppUrl(member.phone)}
                      target="_blank"
                      rel="noopener noreferrer"
                      aria-label="Enviar mensagem no WhatsApp para {member.name}"
                      class="h-10 w-10 rounded-xl bg-emerald-500/15 hover:bg-emerald-500/25 text-emerald-800 dark:text-emerald-300 border border-emerald-600/35 dark:border-emerald-500/40 flex items-center justify-center transition-colors"
                    >
                      <MessageCircle size={16} aria-hidden="true" />
                    </a>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/each}
    </div>
  {/if}
</div>

<script lang="ts">
  import {
    Crown,
    Edit2,
    KeyRound,
    Lock,
    MessageCircle,
    MonitorPlay,
    Phone,
    Plus,
    Search,
    Shield,
    Trash2,
    User,
    Users,
    X,
  } from 'lucide-svelte';
  import type { TeamDepartment, TeamMember } from '$lib/types';
  import { getWhatsAppUrl } from '$lib/teams';
  import { getContrastTextColor, getInitials } from './teamConstants';

  interface Props {
    members: TeamMember[];
    departments: TeamDepartment[];
    isLoading: boolean;
    onNewMember: () => void;
    onEditMember: (member: TeamMember) => void;
    onDeleteMember: (member: TeamMember) => void;
  }

  let { members, departments, isLoading, onNewMember, onEditMember, onDeleteMember }: Props =
    $props();

  let search = $state('');
  let deptFilter = $state('all');

  const departmentMap = $derived.by(() => {
    const map = new Map<string, TeamDepartment>();
    for (const dept of departments) map.set(dept.id, dept);
    return map;
  });

  const filteredMembers = $derived.by(() => {
    const query = search.trim().toLowerCase();
    return members.filter((member) => {
      if (deptFilter !== 'all' && !(member.department_ids || []).includes(deptFilter)) {
        return false;
      }
      if (!query) return true;
      if (member.name.toLowerCase().includes(query)) return true;
      if (member.phone && member.phone.includes(query)) return true;
      return false;
    });
  });

  const activeCount = $derived(members.filter((member) => member.is_active).length);

  function initialsColor(member: TeamMember): string {
    return member.color || '#3b82f6';
  }

  function whatsappFor(member: TeamMember): string {
    return getWhatsAppUrl(member.phone, `Olá ${member.name}! Mensagem da equipe de mídia.`);
  }
</script>

<div class="flex flex-col flex-1 min-h-0">
  <div
    class="px-4 sm:px-6 py-3 border-b border-border bg-muted/30 flex flex-wrap items-center justify-between gap-2 shrink-0"
  >
    <div class="flex flex-wrap items-center gap-2 flex-1 min-w-0 max-w-2xl">
      <div class="relative flex-1 min-w-[200px]">
        <Search
          size={14}
          class="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
        />
        <input
          type="text"
          bind:value={search}
          aria-label="Buscar voluntário por nome ou telefone"
          placeholder="Buscar por nome ou telefone..."
          class="w-full h-9 pl-8 pr-8 rounded-lg border border-border bg-background text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50"
        />
        {#if search}
          <button
            type="button"
            onclick={() => (search = '')}
            aria-label="Limpar busca"
            title="Limpar busca"
            class="absolute right-1 top-1/2 -translate-y-1/2 w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer"
          >
            <X size={13} />
          </button>
        {/if}
      </div>

      <select
        bind:value={deptFilter}
        aria-label="Filtrar voluntários por departamento"
        class="h-9 px-2.5 max-w-[220px] rounded-lg border border-border bg-background text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-ring/50 cursor-pointer"
      >
        <option value="all">Todos os Departamentos</option>
        {#each departments as dept (dept.id)}
          <option value={dept.id}>{dept.name}</option>
        {/each}
      </select>
    </div>

    <div class="flex items-center gap-2 shrink-0">
      <span class="hidden sm:inline text-[11px] text-muted-foreground">
        {activeCount} ativo{activeCount === 1 ? '' : 's'} de {members.length}
      </span>
      <button
        type="button"
        onclick={onNewMember}
        class="inline-flex items-center gap-1.5 h-9 px-3 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer"
      >
        <Plus size={14} />
        <span>Novo Voluntário</span>
      </button>
    </div>
  </div>

  <div class="flex-1 min-h-0 p-4 sm:p-6 overflow-y-auto">
    {#if isLoading}
      <div class="h-full min-h-[240px] flex items-center justify-center text-muted-foreground">
        <span class="text-sm">Carregando voluntários...</span>
      </div>
    {:else if filteredMembers.length === 0}
      <div class="min-h-full flex flex-col items-center justify-center text-center p-4 sm:p-8">
        <div
          class="shrink-0 w-14 h-14 rounded-2xl bg-muted border border-border flex items-center justify-center text-muted-foreground mb-4"
        >
          <Users size={26} />
        </div>
        <p class="text-sm font-semibold text-foreground">Nenhum voluntário encontrado</p>
        <p class="text-xs text-muted-foreground mt-1 max-w-sm">
          Cadastre os membros da equipe de mídia para poder escalá-los nos cultos.
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(290px,1fr))] gap-3">
        {#each filteredMembers as member (member.id)}
          {@const waUrl = whatsappFor(member)}
          {@const deptIds = member.department_ids || []}
          <article
            class="p-3.5 rounded-2xl bg-card border border-border flex flex-col gap-3 shadow-sm hover:border-primary/40 transition-colors {member.is_active
              ? ''
              : 'opacity-75'}"
          >
            <div class="flex items-start justify-between gap-3">
              <div class="flex items-center gap-3 min-w-0">
                <div
                  class="w-10 h-10 rounded-xl flex items-center justify-center text-xs font-bold shrink-0"
                  style="background-color: {initialsColor(member)}; color: {getContrastTextColor(
                    initialsColor(member)
                  )};"
                  aria-hidden="true"
                >
                  {getInitials(member.name)}
                </div>
                <div class="min-w-0">
                  <h4 class="text-sm font-bold text-foreground truncate" title={member.name}>
                    {member.name}
                  </h4>
                  {#if member.phone}
                    <div class="flex items-center gap-1 text-[11px] text-muted-foreground mt-0.5">
                      <Phone size={11} class="shrink-0" />
                      <span class="truncate">{member.phone}</span>
                    </div>
                  {:else}
                    <span class="text-[11px] text-muted-foreground italic block mt-0.5">
                      Sem telefone
                    </span>
                  {/if}
                </div>
              </div>

              <div class="flex items-center gap-1 shrink-0">
                {#if waUrl}
                  <a
                    href={waUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Conversar com {member.name} no WhatsApp"
                    title="Conversar no WhatsApp"
                    class="w-8 h-8 rounded-lg border border-border bg-background hover:bg-emerald-500/10 hover:border-emerald-500/35 text-emerald-700 dark:text-emerald-300 flex items-center justify-center transition-colors"
                  >
                    <MessageCircle size={14} />
                  </a>
                {/if}

                <button
                  type="button"
                  onclick={() => onEditMember(member)}
                  aria-label="Editar {member.name}"
                  title="Editar voluntário"
                  class="w-8 h-8 rounded-lg border border-border bg-background hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer"
                >
                  <Edit2 size={14} />
                </button>

                <button
                  type="button"
                  onclick={() => onDeleteMember(member)}
                  aria-label="Excluir {member.name}"
                  title="Excluir voluntário"
                  class="w-8 h-8 rounded-lg border border-border bg-background hover:bg-red-500/10 hover:border-red-500/35 text-muted-foreground hover:text-red-700 dark:hover:text-red-300 flex items-center justify-center transition-colors cursor-pointer"
                >
                  <Trash2 size={14} />
                </button>
              </div>
            </div>

            <div class="flex flex-wrap items-center gap-1.5">
              {#if member.role === 'leader'}
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-bold bg-amber-500/10 text-amber-700 dark:text-amber-300 border border-amber-500/35"
                >
                  <Crown size={11} />
                  <span>Líder</span>
                </span>
              {:else if member.role === 'vice_leader'}
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-bold bg-sky-500/10 text-sky-700 dark:text-sky-300 border border-sky-500/35"
                >
                  <Shield size={11} />
                  <span>Vice-Líder</span>
                </span>
              {:else}
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-muted text-foreground border border-border"
                >
                  <User size={11} />
                  <span>Membro</span>
                </span>
              {/if}

              {#if member.can_control_remote}
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-bold bg-emerald-500/10 text-emerald-700 dark:text-emerald-300 border border-emerald-500/35"
                  title="Permissão para disparar projeções e slides no /remote"
                >
                  <MonitorPlay size={11} />
                  <span>Controle Remoto</span>
                </span>
              {:else}
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-muted text-muted-foreground border border-border"
                  title="Apenas visualização da escala no /remote"
                >
                  <Lock size={11} />
                  <span>Sem Controle</span>
                </span>
              {/if}

              {#if member.pin}
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-semibold bg-sky-500/10 text-sky-700 dark:text-sky-300 border border-sky-500/35"
                  title="Possui PIN numérico cadastrado"
                >
                  <KeyRound size={11} />
                  <span>PIN Ativo</span>
                </span>
              {/if}

              {#if !member.is_active}
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-semibold bg-muted text-muted-foreground border border-border"
                >
                  Inativo
                </span>
              {/if}
            </div>

            <div class="flex flex-wrap items-center gap-1.5">
              {#if deptIds.length === 0}
                <span class="text-[11px] text-muted-foreground italic">
                  Sem departamentos vinculados
                </span>
              {:else}
                {#each deptIds as deptId (deptId)}
                  {@const dept = departmentMap.get(deptId)}
                  {#if dept}
                    <span
                      class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-lg text-[11px] font-medium text-foreground bg-muted/60 border border-border"
                    >
                      <span
                        class="w-1.5 h-1.5 rounded-full shrink-0"
                        style="background-color: {dept.color};"
                        aria-hidden="true"
                      ></span>
                      <span>{dept.name}</span>
                    </span>
                  {/if}
                {/each}
              {/if}
            </div>

            {#if member.notes}
              <p
                class="text-[11px] text-muted-foreground italic bg-muted/40 px-2.5 py-1.5 rounded-lg border border-border line-clamp-2"
              >
                "{member.notes}"
              </p>
            {/if}
          </article>
        {/each}
      </div>
    {/if}
  </div>
</div>

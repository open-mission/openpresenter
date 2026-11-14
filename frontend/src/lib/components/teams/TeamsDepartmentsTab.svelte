<script lang="ts">
  import { Edit2, Plus, Trash2, Users } from 'lucide-svelte';
  import type { TeamDepartment, TeamMember } from '$lib/types';
  import { getContrastTextColor, getDeptIconComponent, getDeptIconLabel } from './teamConstants';

  interface Props {
    departments: TeamDepartment[];
    members: TeamMember[];
    isLoading: boolean;
    onNewDepartment: () => void;
    onEditDepartment: (department: TeamDepartment) => void;
    onDeleteDepartment: (department: TeamDepartment) => void;
  }

  let {
    departments,
    members,
    isLoading,
    onNewDepartment,
    onEditDepartment,
    onDeleteDepartment,
  }: Props = $props();

  const memberCounts = $derived.by(() => {
    const counts = new Map<string, number>();
    for (const member of members) {
      for (const deptId of member.department_ids || []) {
        counts.set(deptId, (counts.get(deptId) ?? 0) + 1);
      }
    }
    return counts;
  });
</script>

<div class="flex flex-col flex-1 min-h-0">
  <div
    class="px-4 sm:px-6 py-3 border-b border-border bg-muted/30 flex flex-wrap items-center justify-between gap-2 shrink-0"
  >
    <p class="text-xs text-muted-foreground">
      Departamentos da equipe (projeção, fotografia, redes, transmissão, som).
    </p>

    <button
      type="button"
      onclick={onNewDepartment}
      class="inline-flex items-center gap-1.5 h-9 px-3 rounded-lg bg-primary text-primary-foreground hover:opacity-90 text-xs font-semibold transition-opacity cursor-pointer"
    >
      <Plus size={14} />
      <span>Novo Departamento</span>
    </button>
  </div>

  <div class="flex-1 min-h-0 p-4 sm:p-6 overflow-y-auto">
    {#if isLoading}
      <div class="h-full min-h-[240px] flex items-center justify-center text-muted-foreground">
        <span class="text-sm">Carregando departamentos...</span>
      </div>
    {:else if departments.length === 0}
      <div class="min-h-full flex flex-col items-center justify-center text-center p-4 sm:p-8">
        <div
          class="shrink-0 w-14 h-14 rounded-2xl bg-muted border border-border flex items-center justify-center text-muted-foreground mb-4"
        >
          <Users size={26} />
        </div>
        <p class="text-sm font-semibold text-foreground">Nenhum departamento cadastrado</p>
        <p class="text-xs text-muted-foreground mt-1 max-w-sm">
          Crie departamentos para organizar a escala por área de atuação.
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(260px,1fr))] gap-3">
        {#each departments as dept (dept.id)}
          {@const DeptIcon = getDeptIconComponent(dept.icon)}
          {@const count = memberCounts.get(dept.id) ?? 0}
          <article
            class="p-3.5 rounded-2xl bg-card border-2 flex flex-col gap-3"
            style="border-color: {dept.color}66;"
          >
            <div class="flex items-start justify-between gap-3">
              <div class="flex items-center gap-3 min-w-0">
                <div
                  class="w-10 h-10 rounded-xl flex items-center justify-center shrink-0"
                  style="background-color: {dept.color}; color: {getContrastTextColor(dept.color)};"
                  aria-hidden="true"
                >
                  <DeptIcon size={18} />
                </div>
                <div class="min-w-0">
                  <h4 class="text-sm font-bold text-foreground truncate">{dept.name}</h4>
                  <span class="text-[11px] text-muted-foreground">
                    {count} {count === 1 ? 'membro cadastrado' : 'membros cadastrados'}
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-1 shrink-0">
                <button
                  type="button"
                  onclick={() => onEditDepartment(dept)}
                  aria-label="Editar departamento {dept.name}"
                  title="Editar departamento"
                  class="w-8 h-8 rounded-lg border border-border bg-background hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer"
                >
                  <Edit2 size={14} />
                </button>

                <button
                  type="button"
                  onclick={() => onDeleteDepartment(dept)}
                  aria-label="Excluir departamento {dept.name}"
                  title="Excluir departamento"
                  class="w-8 h-8 rounded-lg border border-border bg-background hover:bg-red-500/10 hover:border-red-500/35 text-muted-foreground hover:text-red-700 dark:hover:text-red-300 flex items-center justify-center transition-colors cursor-pointer"
                >
                  <Trash2 size={14} />
                </button>
              </div>
            </div>

            {#if dept.description}
              <p class="text-xs text-muted-foreground leading-relaxed line-clamp-2">
                {dept.description}
              </p>
            {/if}

            <div
              class="pt-2 border-t border-border flex items-center justify-between text-[11px] text-muted-foreground"
            >
              <span class="inline-flex items-center gap-1.5">
                <span
                  class="w-2 h-2 rounded-full"
                  style="background-color: {dept.color};"
                  aria-hidden="true"
                ></span>
                Cor de identificação
              </span>
              <span class="truncate">{getDeptIconLabel(dept.icon)}</span>
            </div>
          </article>
        {/each}
      </div>
    {/if}
  </div>
</div>

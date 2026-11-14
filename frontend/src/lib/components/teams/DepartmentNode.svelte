<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
  import { AlertCircle, CheckCircle2, Plus, Users } from 'lucide-svelte';
  import { getContrastTextColor, getDeptIconComponent } from './teamConstants';

  interface DepartmentNodeData {
    id: string;
    name: string;
    color: string;
    icon: string;
    memberCount: number;
    hasLead: boolean;
    onAddMember?: (deptId: string) => void;
  }

  let { data }: NodeProps = $props();
  const deptData = $derived(data as unknown as DepartmentNodeData);
  const IconComponent = $derived(getDeptIconComponent(deptData?.icon));
</script>

<div
  class="px-3.5 py-3 rounded-2xl bg-card/95 border-2 shadow-lg min-w-[210px] text-foreground select-none transition-colors"
  style="border-color: {deptData?.color || 'var(--border)'};"
>
  <Handle
    type="target"
    position={Position.Top}
    class="!w-2.5 !h-2.5 !bg-muted-foreground !border-2 !border-card"
  />

  <div class="flex items-center justify-between gap-2 mb-2">
    <div class="flex items-center gap-2 min-w-0">
      <div
        class="w-7 h-7 rounded-xl flex items-center justify-center shrink-0"
        style="background-color: {deptData?.color || '#3b82f6'}; color: {getContrastTextColor(
          deptData?.color || '#3b82f6'
        )};"
      >
        <IconComponent size={15} />
      </div>
      <div class="font-bold text-xs text-foreground truncate max-w-[130px]">
        {deptData?.name || 'Departamento'}
      </div>
    </div>

    {#if deptData?.onAddMember}
      <button
        type="button"
        onclick={() => deptData.onAddMember?.(deptData.id)}
        aria-label="Escalar voluntário para {deptData?.name}"
        title="Escalar voluntário para {deptData?.name}"
        class="w-7 h-7 shrink-0 rounded-lg border border-border bg-background hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer"
      >
        <Plus size={13} />
      </button>
    {/if}
  </div>

  <div class="flex items-center justify-between gap-2 pt-2 border-t border-border text-[11px]">
    <div class="flex items-center gap-1 font-medium text-muted-foreground">
      <Users size={12} />
      <span>{deptData?.memberCount || 0} {deptData?.memberCount === 1 ? 'voluntário' : 'voluntários'}</span>
    </div>

    {#if deptData?.hasLead}
      <span
        class="inline-flex items-center gap-1 text-[11px] font-semibold text-emerald-700 dark:text-emerald-300"
      >
        <CheckCircle2 size={11} />
        Titular
      </span>
    {:else}
      <span
        class="inline-flex items-center gap-1 text-[11px] font-semibold text-amber-700 dark:text-amber-300"
      >
        <AlertCircle size={11} />
        Sem titular
      </span>
    {/if}
  </div>

  <Handle
    type="source"
    position={Position.Bottom}
    class="!w-2.5 !h-2.5 !bg-muted-foreground !border-2 !border-card"
  />
</div>

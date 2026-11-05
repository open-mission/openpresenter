<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
  import {
    MonitorPlay,
    Camera,
    Smartphone,
    Video,
    Volume2,
    Users,
    Plus,
    CheckCircle2,
    AlertCircle,
  } from 'lucide-svelte';

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

  function getIcon(iconName?: string) {
    switch (iconName) {
      case 'MonitorPlay':
        return MonitorPlay;
      case 'Camera':
        return Camera;
      case 'Smartphone':
        return Smartphone;
      case 'Video':
        return Video;
      case 'Volume2':
        return Volume2;
      default:
        return Users;
    }
  }

  const IconComponent = $derived(getIcon(deptData?.icon));
</script>

<div
  class="px-3.5 py-3 rounded-2xl bg-neutral-900/95 border-2 shadow-xl backdrop-blur-md min-w-[210px] text-foreground select-none transition-all hover:scale-[1.02]"
  style="border-color: {deptData?.color || '#3b82f6'};"
>
  <!-- Alça de Entrada (Conecta do Culto) -->
  <Handle
    type="target"
    position={Position.Top}
    class="!w-2.5 !h-2.5 !bg-white !border-2 !border-neutral-900"
  />

  <div class="flex items-center justify-between gap-2 mb-2">
    <div class="flex items-center gap-2 min-w-0">
      <div
        class="w-7 h-7 rounded-xl flex items-center justify-center shrink-0 text-white shadow-sm"
        style="background-color: {deptData?.color || '#3b82f6'};"
      >
        <IconComponent size={15} />
      </div>
      <div class="font-bold text-xs text-white truncate max-w-[130px]">
        {deptData?.name || 'Departamento'}
      </div>
    </div>

    <!-- Botão Rápido para Escalar -->
    {#if deptData?.onAddMember}
      <button
        type="button"
        onclick={() => deptData.onAddMember?.(deptData.id)}
        class="w-6 h-6 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
        title="Escalar voluntário para {deptData?.name}"
      >
        <Plus size={13} />
      </button>
    {/if}
  </div>

  <div class="flex items-center justify-between pt-2 border-t border-neutral-800 text-[11px]">
    <div class="flex items-center gap-1 font-medium text-neutral-300">
      <Users size={12} class="text-neutral-500" />
      <span>{deptData?.memberCount || 0} {deptData?.memberCount === 1 ? 'voluntário' : 'voluntários'}</span>
    </div>

    {#if deptData?.hasLead}
      <span class="inline-flex items-center gap-0.5 text-[10px] font-semibold text-emerald-400">
        <CheckCircle2 size={11} />
        Titular
      </span>
    {:else}
      <span class="inline-flex items-center gap-0.5 text-[10px] font-semibold text-amber-400 animate-pulse">
        <AlertCircle size={11} />
        Sem titular
      </span>
    {/if}
  </div>

  <!-- Alça de Saída para os Voluntários Alocados -->
  <Handle
    type="source"
    position={Position.Bottom}
    class="!w-2.5 !h-2.5 !bg-white !border-2 !border-neutral-900"
  />
</div>

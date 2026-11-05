<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
  import {
    CheckCircle2,
    Clock,
    XCircle,
    Trash2,
    MessageCircle,
    Crown,
    User,
  } from 'lucide-svelte';
  import { getWhatsAppUrl } from '$lib/teams';

  interface MemberNodeData {
    id: string; // roster item id
    memberId: string;
    memberName: string;
    memberPhone?: string | null;
    role: 'lead' | 'helper';
    status: 'confirmed' | 'pending' | 'declined';
    departmentColor?: string;
    onUpdateStatus?: (itemId: string, newStatus: 'confirmed' | 'pending' | 'declined') => void;
    onRemove?: (itemId: string) => void;
  }

  let { data }: NodeProps = $props();
  const itemData = $derived(data as unknown as MemberNodeData);

  function getInitials(name?: string) {
    if (!name) return '?';
    const parts = name.trim().split(/\s+/);
    if (parts.length === 1) return parts[0].substring(0, 2).toUpperCase();
    return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
  }

  function cycleStatus(e: MouseEvent) {
    e.stopPropagation();
    if (!itemData?.onUpdateStatus || !itemData?.id) return;
    const nextStatus: Record<string, 'confirmed' | 'pending' | 'declined'> = {
      confirmed: 'declined',
      declined: 'pending',
      pending: 'confirmed',
    };
    const next = nextStatus[itemData.status || 'pending'] || 'confirmed';
    itemData.onUpdateStatus(itemData.id, next);
  }

  function handleRemove(e: MouseEvent) {
    e.stopPropagation();
    if (itemData?.onRemove && itemData?.id) {
      itemData.onRemove(itemData.id);
    }
  }

  const statusConfig = $derived.by(() => {
    switch (itemData?.status) {
      case 'confirmed':
        return {
          label: 'Confirmado',
          color: 'text-emerald-400 bg-emerald-500/15 border-emerald-500/30 hover:bg-emerald-500/25',
          icon: CheckCircle2,
        };
      case 'declined':
        return {
          label: 'Não Comparece',
          color: 'text-rose-400 bg-rose-500/15 border-rose-500/30 hover:bg-rose-500/25',
          icon: XCircle,
        };
      case 'pending':
      default:
        return {
          label: 'Pendente',
          color: 'text-amber-400 bg-amber-500/15 border-amber-500/30 hover:bg-amber-500/25',
          icon: Clock,
        };
    }
  });

  const StatusIcon = $derived(statusConfig.icon);

  const whatsappHref = $derived.by(() => {
    if (!itemData?.memberPhone) return null;
    return getWhatsAppUrl(
      itemData.memberPhone,
      `Olá ${itemData.memberName}, você foi escalado(a) na equipe de mídia!`
    );
  });
</script>

<div
  class="group relative px-3 py-2.5 rounded-xl bg-neutral-900/90 border border-neutral-700/80 shadow-lg backdrop-blur-md min-w-[200px] text-foreground select-none transition-all hover:border-neutral-500 hover:shadow-xl hover:scale-[1.02]"
>
  <!-- Alça de Conexão Superior (recebe do Departamento) -->
  <Handle
    type="target"
    position={Position.Top}
    class="!w-2 !h-2 !bg-neutral-400 !border-2 !border-neutral-900"
  />

  <div class="flex items-center gap-2.5">
    <!-- Avatar -->
    <div
      class="w-8 h-8 rounded-full flex items-center justify-center font-bold text-xs shrink-0 text-white shadow-inner"
      style="background-color: {itemData?.departmentColor ? `${itemData.departmentColor}44` : '#374151'}; border: 1.5px solid {itemData?.departmentColor || '#6b7280'};"
    >
      {getInitials(itemData?.memberName)}
    </div>

    <!-- Informações do Voluntário -->
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-1.5">
        <span class="font-semibold text-xs text-neutral-100 truncate">
          {itemData?.memberName || 'Voluntário'}
        </span>
        {#if itemData?.role === 'lead'}
          <span
            class="inline-flex items-center text-amber-400 shrink-0"
            title="Titular / Responsável"
          >
            <Crown size={12} />
          </span>
        {/if}
      </div>

      <div class="flex items-center gap-1.5 mt-0.5">
        <span class="text-[10px] text-neutral-400 font-medium capitalize">
          {itemData?.role === 'lead' ? 'Titular' : 'Auxiliar'}
        </span>
      </div>
    </div>

    <!-- Ações no Hover -->
    <div class="flex items-center gap-1 opacity-80 group-hover:opacity-100 transition-opacity">
      {#if whatsappHref}
        <a
          href={whatsappHref}
          target="_blank"
          rel="noopener noreferrer"
          class="w-6 h-6 rounded-md bg-emerald-600/20 hover:bg-emerald-600 text-emerald-400 hover:text-white flex items-center justify-center transition-colors"
          title="Falar no WhatsApp"
          onclick={(e) => e.stopPropagation()}
        >
          <MessageCircle size={12} />
        </a>
      {/if}

      {#if itemData?.onRemove}
        <button
          type="button"
          onclick={handleRemove}
          class="w-6 h-6 rounded-md bg-neutral-800 hover:bg-rose-600 text-neutral-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
          title="Remover da escala"
        >
          <Trash2 size={12} />
        </button>
      {/if}
    </div>
  </div>

  <!-- Barra de Status Interativa (Clica para alternar) -->
  <div class="mt-2 pt-2 border-t border-neutral-800/80 flex items-center justify-between">
    <button
      type="button"
      onclick={cycleStatus}
      class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full border text-[10px] font-medium transition-colors cursor-pointer {statusConfig.color}"
      title="Clique para alternar status (Confirmado / Não Comparece / Pendente)"
    >
      <StatusIcon size={11} />
      <span>{statusConfig.label}</span>
    </button>

    {#if itemData?.memberPhone}
      <span class="text-[10px] text-neutral-500 font-mono">
        {itemData.memberPhone.slice(-4)}
      </span>
    {/if}
  </div>
</div>

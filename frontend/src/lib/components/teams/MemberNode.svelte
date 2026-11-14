<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
  import { Crown, MessageCircle, Trash2 } from 'lucide-svelte';
  import type { RosterStatus } from '$lib/types';
  import { formatRosterRoleLabel, getWhatsAppUrl, isRosterLeadRole } from '$lib/teams';
  import { getInitials, getNextRosterStatus, getRosterStatusMeta } from './teamConstants';

  interface MemberNodeData {
    id: string;
    memberId: string;
    memberName: string;
    memberPhone?: string | null;
    role: string;
    status: string;
    departmentColor?: string;
    onUpdateStatus?: (itemId: string, newStatus: RosterStatus) => void;
    onRemove?: (itemId: string) => void;
  }

  let { data }: NodeProps = $props();
  const itemData = $derived(data as unknown as MemberNodeData);

  const statusMeta = $derived(getRosterStatusMeta(itemData?.status));
  const StatusIcon = $derived(statusMeta.icon);

  const whatsappHref = $derived.by(() => {
    if (!itemData?.memberPhone) return null;
    return getWhatsAppUrl(
      itemData.memberPhone,
      `Olá ${itemData.memberName}, você foi escalado(a) na equipe de mídia!`
    );
  });

  function cycleStatus(e: MouseEvent) {
    e.stopPropagation();
    if (!itemData?.onUpdateStatus || !itemData?.id) return;
    itemData.onUpdateStatus(itemData.id, getNextRosterStatus(itemData.status));
  }

  function handleRemove(e: MouseEvent) {
    e.stopPropagation();
    if (itemData?.onRemove && itemData?.id) itemData.onRemove(itemData.id);
  }
</script>

<div
  class="group relative px-3 py-2.5 rounded-xl bg-card/95 border border-border shadow-md min-w-[210px] text-foreground select-none transition-colors hover:border-primary/50"
>
  <Handle
    type="target"
    position={Position.Top}
    class="!w-2 !h-2 !bg-muted-foreground !border-2 !border-card"
  />

  <div class="flex items-center gap-2.5">
    <div
      class="w-8 h-8 rounded-full flex items-center justify-center font-bold text-[11px] shrink-0 text-foreground border"
      style="background-color: {itemData?.departmentColor
        ? `${itemData.departmentColor}22`
        : 'var(--muted)'}; border-color: {itemData?.departmentColor || 'var(--border)'};"
      aria-hidden="true"
    >
      {getInitials(itemData?.memberName)}
    </div>

    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-1.5">
        <span class="font-semibold text-xs text-foreground truncate">
          {itemData?.memberName || 'Voluntário'}
        </span>
        {#if isRosterLeadRole(itemData?.role)}
          <span
            class="inline-flex items-center text-amber-700 dark:text-amber-300 shrink-0"
            title="Titular / Responsável"
          >
            <Crown size={12} />
          </span>
        {/if}
      </div>

      <span class="block text-[11px] text-muted-foreground font-medium mt-0.5">
        {formatRosterRoleLabel(itemData?.role)}
      </span>
    </div>

    <div class="flex items-center gap-1 shrink-0">
      {#if whatsappHref}
        <a
          href={whatsappHref}
          target="_blank"
          rel="noopener noreferrer"
          aria-label="Conversar com {itemData.memberName} no WhatsApp"
          title="Falar no WhatsApp"
          class="w-7 h-7 rounded-md border border-border bg-background hover:bg-emerald-500/10 hover:border-emerald-500/35 text-emerald-700 dark:text-emerald-300 flex items-center justify-center transition-colors"
          onclick={(e) => e.stopPropagation()}
        >
          <MessageCircle size={13} />
        </a>
      {/if}

      {#if itemData?.onRemove}
        <button
          type="button"
          onclick={handleRemove}
          aria-label="Remover {itemData.memberName} da escala"
          title="Remover da escala"
          class="w-7 h-7 rounded-md border border-border bg-background hover:bg-red-500/10 hover:border-red-500/35 text-muted-foreground hover:text-red-700 dark:hover:text-red-300 flex items-center justify-center transition-colors cursor-pointer"
        >
          <Trash2 size={13} />
        </button>
      {/if}
    </div>
  </div>

  <div class="mt-2 pt-2 border-t border-border flex items-center justify-between gap-2">
    <button
      type="button"
      onclick={cycleStatus}
      aria-label="Alternar presença de {itemData?.memberName}"
      title="Clique para alternar status (Confirmado / Não comparece / Pendente)"
      class="inline-flex items-center gap-1 min-h-[26px] px-2.5 py-1 rounded-full border text-[11px] font-semibold transition-colors cursor-pointer {statusMeta.chipClass}"
    >
      <StatusIcon size={11} />
      <span>{statusMeta.label}</span>
    </button>

    {#if itemData?.memberPhone}
      <span class="text-[11px] text-muted-foreground font-mono">
        {itemData.memberPhone.slice(-4)}
      </span>
    {/if}
  </div>
</div>

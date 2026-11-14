<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
  import { Calendar, Clock, Sparkles } from 'lucide-svelte';

  interface ServiceNodeData {
    title: string;
    date: string;
    time?: string | null;
    isActive: boolean;
  }

  let { data }: NodeProps = $props();
  const serviceData = $derived(data as unknown as ServiceNodeData);

  function formatDate(value?: string) {
    if (!value) return '';
    const [year, month, day] = value.split('-');
    if (day && month && year) return `${day}/${month}/${year}`;
    return value;
  }
</script>

<div
  class="px-4 py-3 rounded-2xl bg-card/95 border-2 border-primary/40 shadow-lg min-w-[240px] text-foreground select-none transition-colors hover:border-primary/70"
>
  <div class="flex items-center justify-between gap-2 mb-1.5">
    <div class="flex items-center gap-1.5 text-[11px] font-bold text-primary tracking-wide uppercase">
      <Sparkles size={13} />
      <span>Culto / Evento</span>
    </div>
    {#if serviceData?.isActive}
      <span
        class="inline-flex items-center gap-1 text-[11px] font-semibold px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-700 dark:text-emerald-300 border border-emerald-500/35"
      >
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
        Ativa
      </span>
    {/if}
  </div>

  <div class="text-sm font-bold text-foreground truncate max-w-[260px]">
    {serviceData?.title || 'Culto'}
  </div>

  <div class="flex items-center gap-3 mt-2 text-[11px] text-muted-foreground font-medium">
    {#if serviceData?.date}
      <div class="flex items-center gap-1">
        <Calendar size={12} class="text-muted-foreground" />
        <span>{formatDate(serviceData.date)}</span>
      </div>
    {/if}
    {#if serviceData?.time}
      <div class="flex items-center gap-1">
        <Clock size={12} class="text-muted-foreground" />
        <span>{serviceData.time}</span>
      </div>
    {/if}
  </div>

  <Handle
    type="source"
    position={Position.Bottom}
    class="!w-3 !h-3 !bg-primary !border-2 !border-card transition-transform hover:!scale-125"
  />
</div>

<script lang="ts">
  import { Handle, Position, type NodeProps } from '@xyflow/svelte';
  import { Calendar, Clock, CheckCircle2, Sparkles } from 'lucide-svelte';

  interface ServiceNodeData {
    title: string;
    date: string;
    time?: string | null;
    isActive: boolean;
  }

  let { data }: NodeProps = $props();
  const serviceData = $derived(data as unknown as ServiceNodeData);

  function formatDate(d?: string) {
    if (!d) return '';
    const [year, month, day] = d.split('-');
    if (day && month && year) return `${day}/${month}/${year}`;
    return d;
  }
</script>

<div class="px-4 py-3 rounded-2xl bg-neutral-900/95 border-2 border-primary/50 shadow-2xl backdrop-blur-md min-w-[240px] text-foreground select-none transition-all hover:border-primary">
  <div class="flex items-center justify-between gap-2 mb-1.5">
    <div class="flex items-center gap-1.5 text-xs font-bold text-primary tracking-wide uppercase">
      <Sparkles size={13} class="animate-pulse" />
      <span>Culto / Evento</span>
    </div>
    {#if serviceData?.isActive}
      <span class="inline-flex items-center gap-1 text-[10px] font-semibold px-2 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
        Ativo
      </span>
    {/if}
  </div>

  <div class="text-sm font-bold text-white truncate max-w-[260px]">
    {serviceData?.title || 'Culto'}
  </div>

  <div class="flex items-center gap-3 mt-2 text-xs text-neutral-400 font-medium">
    {#if serviceData?.date}
      <div class="flex items-center gap-1">
        <Calendar size={12} class="text-neutral-500" />
        <span>{formatDate(serviceData.date)}</span>
      </div>
    {/if}
    {#if serviceData?.time}
      <div class="flex items-center gap-1">
        <Clock size={12} class="text-neutral-500" />
        <span>{serviceData.time}</span>
      </div>
    {/if}
  </div>

  <!-- Alça de Conexão de Saída para os Departamentos -->
  <Handle
    type="source"
    position={Position.Bottom}
    class="!w-3 !h-3 !bg-primary !border-2 !border-neutral-900 transition-transform hover:!scale-125"
  />
</div>

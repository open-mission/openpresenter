<script lang="ts">
  import '@xyflow/svelte/dist/style.css';
  import {
    SvelteFlow,
    Background,
    Controls,
    MiniMap,
    type Node,
    type Edge,
  } from '@xyflow/svelte';
  import type { ServiceRoster, TeamDepartment, TeamMember } from '$lib/types';
  import ServiceNode from './ServiceNode.svelte';
  import DepartmentNode from './DepartmentNode.svelte';
  import MemberNode from './MemberNode.svelte';

  interface Props {
    roster: ServiceRoster | null;
    departments: TeamDepartment[];
    allMembers: TeamMember[];
    onUpdateRosterItemStatus: (itemId: string, status: 'confirmed' | 'pending' | 'declined') => Promise<void> | void;
    onRemoveRosterItem: (itemId: string) => Promise<void> | void;
    onAddMemberToDept: (deptId: string) => void;
  }

  let {
    roster,
    departments,
    allMembers,
    onUpdateRosterItemStatus,
    onRemoveRosterItem,
    onAddMemberToDept,
  }: Props = $props();

  const nodeTypes = {
    service: ServiceNode,
    department: DepartmentNode,
    member: MemberNode,
  };

  let nodes = $state.raw<Node[]>([]);
  let edges = $state.raw<Edge[]>([]);

  // Calcula os nós e arestas de forma reativa a partir da escala atual
  const flowElements = $derived.by(() => {
    if (!roster) {
      return { nodes: [] as Node[], edges: [] as Edge[] };
    }

    const calculatedNodes: Node[] = [];
    const calculatedEdges: Edge[] = [];

    const activeDepts = departments;
    const deptCount = Math.max(activeDepts.length, 1);
    const colSpacing = 270;
    const totalWidth = deptCount * colSpacing;
    const startX = 60;
    const serviceX = startX + totalWidth / 2 - 120;

    // 1. Nó Raiz: Culto / Serviço
    calculatedNodes.push({
      id: 'service-root',
      type: 'service',
      position: { x: serviceX, y: 30 },
      data: {
        title: roster.title,
        date: roster.service_date,
        time: roster.service_time,
        isActive: roster.is_active,
      },
      selectable: false,
    });

    // 2. Nós dos Departamentos e Voluntários
    activeDepts.forEach((dept, index) => {
      const deptX = startX + index * colSpacing;
      const deptY = 160;

      const deptItems = roster.items.filter((it) => it.department_id === dept.id);
      const hasLead = deptItems.some((it) => it.role === 'lead');

      calculatedNodes.push({
        id: `dept-${dept.id}`,
        type: 'department',
        position: { x: deptX, y: deptY },
        data: {
          id: dept.id,
          name: dept.name,
          color: dept.color,
          icon: dept.icon,
          memberCount: deptItems.length,
          hasLead,
          onAddMember: (deptId: string) => onAddMemberToDept(deptId),
        },
        selectable: false,
      });

      // Aresta Culto -> Departamento
      calculatedEdges.push({
        id: `edge-root-dept-${dept.id}`,
        source: 'service-root',
        target: `dept-${dept.id}`,
        animated: roster.is_active,
        style: `stroke: ${dept.color || '#3b82f6'}; stroke-width: 2px;`,
      });

      // 3. Voluntários escalados neste departamento
      deptItems.forEach((item, itemIdx) => {
        const memberY = 280 + itemIdx * 86;
        const memberX = deptX + 5;
        const member = allMembers.find((m) => m.id === item.member_id);

        calculatedNodes.push({
          id: `item-${item.id}`,
          type: 'member',
          position: { x: memberX, y: memberY },
          data: {
            id: item.id,
            memberId: item.member_id,
            memberName: member?.name || 'Voluntário',
            memberPhone: member?.phone,
            role: item.role,
            status: item.status,
            departmentColor: dept.color,
            onUpdateStatus: (id: string, st: 'confirmed' | 'pending' | 'declined') =>
              onUpdateRosterItemStatus(id, st),
            onRemove: (id: string) => onRemoveRosterItem(id),
          },
        });

        // Aresta Departamento -> Voluntário
        calculatedEdges.push({
          id: `edge-dept-${dept.id}-item-${item.id}`,
          source: `dept-${dept.id}`,
          target: `item-${item.id}`,
          style: `stroke: ${dept.color || '#6b7280'}77; stroke-width: 1.5px; stroke-dasharray: 3 3;`,
        });
      });
    });

    return { nodes: calculatedNodes, edges: calculatedEdges };
  });

  $effect(() => {
    nodes = flowElements.nodes;
    edges = flowElements.edges;
  });
</script>

<div class="w-full h-full min-h-[520px] rounded-2xl overflow-hidden bg-neutral-950/80 border border-neutral-800/80 relative">
  {#if !roster}
    <div class="h-full flex flex-col items-center justify-center p-8 text-center text-neutral-400">
      <p class="text-sm font-medium">Nenhuma escala selecionada para visualização.</p>
    </div>
  {:else}
    <SvelteFlow
      bind:nodes
      bind:edges
      {nodeTypes}
      fitView
      fitViewOptions={{ padding: 0.2 }}
      minZoom={0.3}
      maxZoom={1.5}
      proOptions={{ hideAttribution: true }}
      class="bg-neutral-950"
    >
      <Background color="#262626" gap={20} size={1.2} />
      <Controls class="!bg-neutral-900 !border-neutral-800 !text-neutral-200 !shadow-xl [&>button]:!border-neutral-800 hover:[&>button]:!bg-neutral-800" />
      <MiniMap
        nodeColor={(n) => {
          if (n.type === 'service') return '#3b82f6';
          if (n.type === 'department') return (n.data as any)?.color || '#10b981';
          return '#6b7280';
        }}
        class="!bg-neutral-900/90 !border-neutral-800 !rounded-xl overflow-hidden shadow-2xl"
      />
    </SvelteFlow>
  {/if}
</div>

<style>
  :global(.svelte-flow) {
    background-color: transparent !important;
  }
  :global(.svelte-flow__handle) {
    border-radius: 50% !important;
  }
</style>

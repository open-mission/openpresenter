<script lang="ts">
  import '@xyflow/svelte/dist/style.css';
  import { onMount } from 'svelte';
  import { SvelteFlow, Background, Controls, MiniMap, type Node, type Edge } from '@xyflow/svelte';
  import type { RosterStatus, ServiceRoster, TeamDepartment, TeamMember } from '$lib/types';
  import { isRosterLeadRole } from '$lib/teams';
  import { themeStore } from '$lib/settings';
  import ServiceNode from './ServiceNode.svelte';
  import DepartmentNode from './DepartmentNode.svelte';
  import MemberNode from './MemberNode.svelte';

  interface Props {
    roster: ServiceRoster | null;
    departments: TeamDepartment[];
    allMembers: TeamMember[];
    onUpdateRosterItemStatus: (itemId: string, status: RosterStatus) => Promise<void> | void;
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

  let systemPrefersDark = $state(false);

  onMount(() => {
    const query = window.matchMedia('(prefers-color-scheme: dark)');
    systemPrefersDark = query.matches;
    const onChange = (event: MediaQueryListEvent) => {
      systemPrefersDark = event.matches;
    };
    query.addEventListener('change', onChange);
    return () => query.removeEventListener('change', onChange);
  });

  const isLightMode = $derived(
    $themeStore === 'light' ? true : $themeStore === 'dark' ? false : !systemPrefersDark
  );

  // Calcula os nós e arestas de forma reativa a partir da escala atual
  const flowElements = $derived.by(() => {
    if (!roster) {
      return { nodes: [] as Node[], edges: [] as Edge[] };
    }

    const calculatedNodes: Node[] = [];
    const calculatedEdges: Edge[] = [];

    const deptCount = Math.max(departments.length, 1);
    const colSpacing = 270;
    const totalWidth = deptCount * colSpacing;
    const startX = 60;
    const serviceX = startX + totalWidth / 2 - 120;

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

    departments.forEach((dept, index) => {
      const deptX = startX + index * colSpacing;
      const deptItems = roster.items.filter((item) => item.department_id === dept.id);
      const hasLead = deptItems.some((item) => isRosterLeadRole(item.role));

      calculatedNodes.push({
        id: `dept-${dept.id}`,
        type: 'department',
        position: { x: deptX, y: 160 },
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

      calculatedEdges.push({
        id: `edge-root-dept-${dept.id}`,
        source: 'service-root',
        target: `dept-${dept.id}`,
        animated: roster.is_active,
        style: `stroke: ${dept.color || 'var(--primary)'}; stroke-width: 2px;`,
      });

      deptItems.forEach((item, itemIdx) => {
        const member = allMembers.find((entry) => entry.id === item.member_id);

        calculatedNodes.push({
          id: `item-${item.id}`,
          type: 'member',
          position: { x: deptX + 5, y: 280 + itemIdx * 86 },
          data: {
            id: item.id,
            memberId: item.member_id,
            memberName: member?.name || 'Voluntário',
            memberPhone: member?.phone,
            role: item.role,
            status: item.status,
            departmentColor: dept.color,
            onUpdateStatus: (id: string, status: RosterStatus) =>
              onUpdateRosterItemStatus(id, status),
            onRemove: (id: string) => onRemoveRosterItem(id),
          },
        });

        calculatedEdges.push({
          id: `edge-dept-${dept.id}-item-${item.id}`,
          source: `dept-${dept.id}`,
          target: `item-${item.id}`,
          style: `stroke: ${dept.color || 'var(--border)'}; stroke-width: 1.5px; stroke-dasharray: 3 3;`,
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

<div class="flow-shell w-full h-full min-h-[420px] rounded-2xl overflow-hidden bg-card border border-border relative">
  {#if !roster}
    <div class="h-full flex flex-col items-center justify-center p-8 text-center text-muted-foreground">
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
      colorMode={isLightMode ? 'light' : 'dark'}
    >
      <Background patternColor={isLightMode ? '#d4d4d8' : '#262626'} gap={20} size={1.2} />
      <Controls />
      <MiniMap
        nodeColor={(node) => {
          if (node.type === 'service') return '#3b82f6';
          if (node.type === 'department') return (node.data as { color?: string })?.color ?? '#10b981';
          return '#6b7280';
        }}
      />
    </SvelteFlow>
  {/if}
</div>

<style>
  .flow-shell :global(.svelte-flow) {
    background-color: transparent !important;
  }
  .flow-shell :global(.svelte-flow__handle) {
    border-radius: 50% !important;
  }
  .flow-shell :global(.svelte-flow__controls) {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    overflow: hidden;
    box-shadow: none;
  }
  .flow-shell :global(.svelte-flow__controls-button) {
    background: var(--card);
    border-bottom: 1px solid var(--border);
    color: var(--foreground);
  }
  .flow-shell :global(.svelte-flow__controls-button:hover) {
    background: var(--muted);
  }
  .flow-shell :global(.svelte-flow__controls-button:disabled) {
    opacity: 0.5;
  }
  .flow-shell :global(.svelte-flow__minimap) {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    overflow: hidden;
  }
  .flow-shell :global(.svelte-flow__minimap-mask) {
    fill: var(--muted);
    opacity: 0.45;
  }
</style>

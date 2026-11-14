<script lang="ts">
  import { Cast, ExternalLink } from 'lucide-svelte';
  import type { ShareOutputTab } from './types';

  interface Props {
    activeTab: ShareOutputTab;
    localIp: string;
    onopenexternal: () => void;
  }

  let { activeTab, localIp, onopenexternal }: Props = $props();
</script>

<div class="px-5 py-3.5 bg-card/60 border-t border-border flex items-center justify-between text-xs text-muted-foreground shrink-0 select-none">
  <span class="text-xs">
    IP da máquina: <strong class="text-foreground font-mono">{localIp || '127.0.0.1'}</strong>
  </span>

  {#if activeTab === 'cast'}
    <span class="text-xs text-muted-foreground flex items-center gap-1.5 font-medium">
      <Cast size={14} class="text-primary" aria-hidden="true" />
      <span>Google Cast SDK</span>
    </span>
  {:else}
    <button
      type="button"
      onclick={onopenexternal}
      class="text-xs text-primary hover:underline flex items-center gap-1.5 cursor-pointer font-semibold min-h-[28px] px-1.5 rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      aria-label="Abrir link em nova aba do navegador"
    >
      <span>Abrir link</span>
      <ExternalLink size={14} aria-hidden="true" />
    </button>
  {/if}
</div>

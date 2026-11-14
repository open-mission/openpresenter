<script lang="ts">
  import { Cast, QrCode, X } from 'lucide-svelte';
  import type { ShareOutputTab } from './types';

  interface Props {
    activeTab: ShareOutputTab;
    onclose: () => void;
  }

  let { activeTab, onclose }: Props = $props();

  const title = $derived(
    activeTab === 'cast'
      ? 'Transmissão sem Fio (Google Cast / Chromecast)'
      : 'Compartilhar Saídas na Rede Local'
  );

  const subtitle = $derived(
    activeTab === 'cast'
      ? 'Projete telas em TVs e projetores sem precisar de cabos HDMI'
      : 'Abra o retorno ou projeção em tablets, celulares e navegadores'
  );
</script>

<div class="px-5 py-3.5 border-b border-border flex items-center justify-between shrink-0 bg-card/60 select-none">
  <div class="flex items-center gap-3 min-w-0">
    <div
      class="w-10 h-10 rounded-xl bg-primary/10 border border-primary/25 text-primary flex items-center justify-center shrink-0"
      aria-hidden="true"
    >
      {#if activeTab === 'cast'}
        <Cast size={20} />
      {:else}
        <QrCode size={20} />
      {/if}
    </div>
    <div class="min-w-0">
      <h2 id="share-outputs-title" class="text-sm sm:text-base font-bold text-foreground truncate">
        {title}
      </h2>
      <p id="share-outputs-desc" class="text-xs text-muted-foreground truncate">
        {subtitle}
      </p>
    </div>
  </div>

  <button
    type="button"
    onclick={onclose}
    class="w-8 h-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
    title="Fechar (Esc)"
    aria-label="Fechar compartilhamento de saídas"
  >
    <X size={18} aria-hidden="true" />
  </button>
</div>

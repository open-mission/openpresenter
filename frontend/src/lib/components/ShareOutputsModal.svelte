<script lang="ts">
  import { onDestroy } from 'svelte';
  import CastControl from './CastControl.svelte';
  import { ShareOutputsController } from './share-outputs/controller.svelte';
  import ShareOutputsHeader from './share-outputs/ShareOutputsHeader.svelte';
  import ShareOutputsTabs from './share-outputs/ShareOutputsTabs.svelte';
  import ShareOutputsQrView from './share-outputs/ShareOutputsQrView.svelte';
  import ShareOutputsFooter from './share-outputs/ShareOutputsFooter.svelte';
  import type { ShareOutputsModalProps } from './share-outputs/types';

  let { open = false, initialTab = 'stage', onclose }: ShareOutputsModalProps = $props();

  const controller = new ShareOutputsController({
    onclose: () => onclose(),
  });

  $effect(() => {
    controller.syncOpen(open, initialTab);
  });

  onDestroy(() => {
    controller.restoreFocus();
  });

  function handleKeydown(e: KeyboardEvent) {
    controller.handleKeydown(e);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-background/80 backdrop-blur-md animate-in fade-in duration-150"
    role="presentation"
    onclick={() => controller.close()}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      bind:this={controller.dialogEl}
      class="bg-card text-card-foreground border border-border rounded-2xl w-full max-w-lg flex flex-col shadow-2xl overflow-hidden max-h-[90vh] animate-in zoom-in-95 duration-150"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-labelledby="share-outputs-title"
      aria-describedby="share-outputs-desc"
      tabindex="-1"
    >
      <!-- Cabeçalho -->
      <ShareOutputsHeader
        activeTab={controller.activeTab}
        onclose={() => controller.close()}
      />

      <!-- Seletor de Abas -->
      <ShareOutputsTabs
        activeTab={controller.activeTab}
        onselecttab={(tab) => controller.selectTab(tab)}
      />

      <!-- Conteúdo da Aba Ativa -->
      <div class="overflow-y-auto min-h-0 flex-1">
        {#if controller.activeTab === 'cast'}
          <!-- Aba Chromecast -->
          <div
            class="p-5"
            role="tabpanel"
            id="share-panel-cast"
            aria-labelledby="share-tab-cast"
          >
            <CastControl />
          </div>
        {:else}
          <!-- Abas com QR Code e URL (Palco, Pública, Remoto, OBS) -->
          <div
            role="tabpanel"
            id={`share-panel-${controller.activeTab}`}
            aria-labelledby={`share-tab-${controller.activeTab}`}
          >
            <ShareOutputsQrView
              qrCodeDataUrl={controller.qrCodeDataUrl}
              description={controller.currentDescription}
              activeUrl={controller.activeUrl}
              isCopied={controller.isCopied}
              oncopy={() => controller.copyUrl()}
              interfaces={controller.networkInfo?.interfaces ?? []}
              selectedInterfaceId={controller.selectedInterfaceId}
              onselectinterface={(id) => controller.selectInterface(id)}
            />
          </div>
        {/if}
      </div>

      <!-- Rodapé -->
      <ShareOutputsFooter
        activeTab={controller.activeTab}
        localIp={controller.selectedInterface?.ip || controller.networkInfo?.local_ip || '127.0.0.1'}
        onopenexternal={() => controller.openExternal()}
      />
    </div>
  </div>
{/if}

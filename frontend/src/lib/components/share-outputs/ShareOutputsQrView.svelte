<script lang="ts">
  import {
    Wifi,
    ShieldCheck,
    Globe,
    Laptop,
    Network,
    ScanLine,
    Copy,
    Check,
    RefreshCw,
  } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import {
    NETWORK_KIND_LABEL,
    networkHintFor,
    networkOptionsFrom,
    resolveNetworkInterface,
  } from './network';
  import type { NetworkInterfaceInfo, NetworkInterfaceKind } from './types';

  interface Props {
    qrCodeDataUrl: string;
    description: string;
    activeUrl: string;
    isCopied: boolean;
    oncopy: () => void;
    interfaces?: NetworkInterfaceInfo[];
    selectedInterfaceId: string;
    onselectinterface: (id: string) => void;
  }

  let {
    qrCodeDataUrl,
    description,
    activeUrl,
    isCopied,
    oncopy,
    interfaces = [],
    selectedInterfaceId,
    onselectinterface,
  }: Props = $props();

  const NETWORK_KIND_ICON: Record<NetworkInterfaceKind, typeof Wifi> = {
    lan: Wifi,
    tailscale: ShieldCheck,
    external: Globe,
    localhost: Laptop,
  };

  const NETWORK_KIND_ICON_CLASS: Record<NetworkInterfaceKind, string> = {
    lan: 'text-emerald-600 dark:text-emerald-400',
    tailscale: 'text-sky-600 dark:text-sky-400',
    external: 'text-amber-600 dark:text-amber-400',
    localhost: 'text-muted-foreground',
  };

  let groupEl = $state<HTMLDivElement | null>(null);

  const options = $derived(networkOptionsFrom(interfaces));
  const selected = $derived(resolveNetworkInterface(interfaces, selectedInterfaceId));
  const hint = $derived(networkHintFor(selected?.kind));

  function handleChipKeydown(e: KeyboardEvent) {
    const navigationKeys = ['ArrowRight', 'ArrowDown', 'ArrowLeft', 'ArrowUp', 'Home', 'End'];
    if (!navigationKeys.includes(e.key)) return;

    const buttons = Array.from(
      groupEl?.querySelectorAll<HTMLButtonElement>('[data-network-option]') ?? []
    );
    if (buttons.length === 0) return;
    e.preventDefault();

    const currentIndex = buttons.findIndex(
      (button) => button.getAttribute('aria-checked') === 'true'
    );
    let nextIndex = currentIndex;

    if (e.key === 'Home') {
      nextIndex = 0;
    } else if (e.key === 'End') {
      nextIndex = buttons.length - 1;
    } else if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
      nextIndex = (Math.max(currentIndex, 0) + 1) % buttons.length;
    } else {
      nextIndex = (currentIndex <= 0 ? buttons.length : currentIndex) - 1;
    }

    const target = buttons[nextIndex];
    if (!target) return;
    target.focus();
    const id = target.dataset.networkOption;
    if (id) onselectinterface(id);
  }
</script>

<div class="p-6 flex flex-col items-center text-center space-y-4">
  {#if options.length > 0}
    <div class="w-full space-y-2">
      {#if options.length > 1}
        <div
          class="flex items-center justify-center gap-1.5 text-[11px] font-bold uppercase tracking-wider text-muted-foreground"
        >
          <Network size={13} aria-hidden="true" />
          <span id="share-network-label">Rede deste link</span>
        </div>
        <div
          bind:this={groupEl}
          role="radiogroup"
          aria-labelledby="share-network-label"
          class="flex flex-wrap items-center justify-center gap-1.5"
        >
          {#each options as option (option.id)}
            {@const isSelected = selected?.id === option.id}
            {@const Icon = NETWORK_KIND_ICON[option.kind] ?? Globe}
            <button
              type="button"
              role="radio"
              aria-checked={isSelected}
              tabindex={isSelected ? 0 : -1}
              data-network-option={option.id}
              title={option.label}
              onclick={() => onselectinterface(option.id)}
              onkeydown={handleChipKeydown}
              class="flex items-center justify-center gap-1.5 px-3 min-h-[34px] rounded-xl border text-xs font-semibold transition-all cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {isSelected
                ? 'border-primary bg-primary text-primary-foreground shadow-xs'
                : 'border-border bg-muted/40 text-muted-foreground hover:bg-muted hover:text-foreground'}"
            >
              <Icon
                size={14}
                class="{isSelected
                  ? 'text-primary-foreground'
                  : NETWORK_KIND_ICON_CLASS[option.kind]} shrink-0"
                aria-hidden="true"
              />
              <span>{NETWORK_KIND_LABEL[option.kind] ?? option.name}</span>
            </button>
          {/each}
        </div>
      {/if}

      {#if hint}
        <p aria-live="polite" class="text-xs text-muted-foreground leading-relaxed">
          {hint}
        </p>
      {/if}
    </div>
  {/if}

  <!-- QR Code renderizado -->
  <div
    class="p-3 bg-white rounded-2xl shadow-xl flex items-center justify-center border border-border/40"
  >
    {#if qrCodeDataUrl}
      <img
        src={qrCodeDataUrl}
        alt="QR Code da Saída Selecionada"
        class="w-52 h-52 object-contain rounded-lg"
      />
    {:else}
      <div
        class="w-52 h-52 flex flex-col items-center justify-center text-muted-foreground text-xs gap-2"
      >
        <RefreshCw size={22} class="animate-spin text-primary" aria-hidden="true" />
        <span>Gerando QR Code...</span>
      </div>
    {/if}
  </div>

  <!-- Instruções -->
  <div class="space-y-1.5 max-w-sm">
    <div class="flex items-center justify-center gap-1.5 text-xs text-foreground font-semibold">
      <ScanLine size={14} class="text-primary" aria-hidden="true" />
      <span>Aponte a câmera do celular para o QR Code</span>
    </div>
    <p class="text-xs text-muted-foreground leading-relaxed">
      {description}
    </p>
  </div>

  <!-- Caixa com a URL e botão de Copiar -->
  <div class="w-full flex items-center gap-2 p-2 rounded-xl bg-muted/40 border border-border text-left">
    <div
      class="flex-1 min-w-0 px-2 font-mono text-xs text-foreground truncate select-all"
      title={activeUrl}
    >
      {activeUrl}
    </div>
    <Button
      variant="secondary"
      size="sm"
      onclick={oncopy}
      class="text-xs min-h-[32px] px-3 shrink-0 gap-1.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      title="Copiar link para a área de transferência"
      aria-label="Copiar link da saída"
    >
      {#if isCopied}
        <Check size={14} class="text-emerald-600 dark:text-emerald-400" aria-hidden="true" />
        <span class="text-emerald-600 dark:text-emerald-400 font-semibold">Copiado</span>
      {:else}
        <Copy size={14} aria-hidden="true" />
        <span>Copiar</span>
      {/if}
    </Button>
  </div>

  <!-- Live Region para leitores de tela -->
  <div aria-live="polite" class="sr-only">
    {isCopied ? 'Link copiado para a área de transferência com sucesso.' : ''}
  </div>
</div>

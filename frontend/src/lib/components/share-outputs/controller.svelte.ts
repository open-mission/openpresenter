import QRCode from 'qrcode';
import type { NetworkInfoResponse } from '$lib/types';
import { focusInitialElement, trapTabKey } from '../focusTrap';
import { SHARE_OUTPUT_TABS } from './defaults';
import type { ShareOutputsCallbacks, ShareOutputTab } from './types';

export class ShareOutputsController {
  open = $state(false);
  activeTab = $state<ShareOutputTab>('stage');
  networkInfo = $state<NetworkInfoResponse | null>(null);
  isLoading = $state(false);
  qrCodeDataUrl = $state<string>('');
  isCopied = $state(false);

  dialogEl = $state<HTMLDivElement | null>(null);

  #callbacks: ShareOutputsCallbacks = { onclose: () => {} };
  #wasOpen = false;
  #previouslyFocused: HTMLElement | null = null;
  #copiedTimeout: ReturnType<typeof setTimeout> | null = null;

  activeUrl = $derived.by(() => {
    if (!this.networkInfo) return '';
    if (this.activeTab === 'public') return this.networkInfo.public_url;
    if (this.activeTab === 'remote') return this.networkInfo.remote_url;
    if (this.activeTab === 'lower') return this.networkInfo.lower_third_url;
    return this.networkInfo.stage_url;
  });

  currentDescription = $derived(
    SHARE_OUTPUT_TABS.find((t) => t.id === this.activeTab)?.description || ''
  );

  constructor(callbacks: ShareOutputsCallbacks) {
    this.#callbacks = callbacks;
  }

  syncOpen(open: boolean, initialTab: ShareOutputTab = 'stage') {
    this.open = open;

    if (open && !this.#wasOpen) {
      this.#wasOpen = true;
      if (initialTab) {
        this.activeTab = initialTab;
      }
      this.isCopied = false;
      this.loadNetworkInfo();

      this.#previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;

      setTimeout(() => {
        if (this.dialogEl) {
          focusInitialElement(this.dialogEl, '[role="tab"][aria-selected="true"]', this.dialogEl);
        }
      }, 50);
    } else if (!open && this.#wasOpen) {
      this.#wasOpen = false;
      if (this.#copiedTimeout) {
        clearTimeout(this.#copiedTimeout);
        this.#copiedTimeout = null;
      }
      this.restoreFocus();
    }
  }

  restoreFocus() {
    if (this.#previouslyFocused && document.contains(this.#previouslyFocused)) {
      this.#previouslyFocused.focus();
    }
    this.#previouslyFocused = null;
  }

  async loadNetworkInfo() {
    this.isLoading = true;
    try {
      const res = await fetch('/api/system/network-info');
      if (res.ok) {
        const data: NetworkInfoResponse = await res.json();
        if (
          (data.local_ip === '127.0.0.1' || data.local_ip === '0.0.0.0') &&
          typeof window !== 'undefined' &&
          window.location.hostname !== 'localhost' &&
          window.location.hostname !== '127.0.0.1'
        ) {
          data.local_ip = window.location.hostname;
          const p = data.port || 18080;
          data.public_url = `http://${data.local_ip}:${p}/public`;
          data.stage_url = `http://${data.local_ip}:${p}/stage`;
          data.remote_url = `http://${data.local_ip}:${p}/remote`;
          data.lower_third_url = `http://${data.local_ip}:${p}/lower-third`;
        }
        this.networkInfo = data;
      } else {
        this.fallbackNetworkInfo();
      }
    } catch {
      this.fallbackNetworkInfo();
    } finally {
      this.isLoading = false;
      this.refreshQrCode();
    }
  }

  fallbackNetworkInfo() {
    if (typeof window === 'undefined') return;
    const origin = window.location.origin;
    this.networkInfo = {
      local_ip: window.location.hostname,
      port: Number(window.location.port) || 3000,
      public_url: `${origin}/public`,
      stage_url: `${origin}/stage`,
      remote_url: `${origin}/remote`,
      lower_third_url: `${origin}/lower-third`,
    };
  }

  async refreshQrCode() {
    if (this.activeTab === 'cast' || !this.activeUrl) {
      this.qrCodeDataUrl = '';
      return;
    }
    try {
      this.qrCodeDataUrl = await QRCode.toDataURL(this.activeUrl, {
        width: 320,
        margin: 2,
        color: {
          dark: '#000000',
          light: '#ffffff',
        },
      });
    } catch (err) {
      console.error('Erro ao gerar QR Code:', err);
    }
  }

  selectTab(tab: ShareOutputTab) {
    this.activeTab = tab;
    this.refreshQrCode();
  }

  async copyUrl() {
    if (!this.activeUrl) return;
    try {
      await navigator.clipboard.writeText(this.activeUrl);
      this.isCopied = true;
      if (this.#copiedTimeout) clearTimeout(this.#copiedTimeout);
      this.#copiedTimeout = setTimeout(() => {
        this.isCopied = false;
      }, 2000);
    } catch (err) {
      console.error('Falha ao copiar:', err);
    }
  }

  openExternal() {
    if (!this.activeUrl || typeof window === 'undefined') return;
    window.open(this.activeUrl, '_blank');
  }

  close() {
    this.#callbacks.onclose();
  }

  handleKeydown(e: KeyboardEvent) {
    if (!this.open) return;

    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      this.close();
      return;
    }

    if (this.dialogEl) {
      trapTabKey(e, this.dialogEl);
    }
  }
}

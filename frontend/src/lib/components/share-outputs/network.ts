import type { NetworkInterfaceInfo, NetworkInterfaceKind } from '$lib/types';

/** Chave de persistência da rede escolhida para os links e QR Codes. */
export const NETWORK_SELECTION_STORAGE_KEY = 'openpresenter_selected_network';

/** Valor sentinela para a rede padrão (primeira interface, sempre a LAN quando disponível). */
export const DEFAULT_NETWORK_SELECTION = 'default';

const KIND_RANK: Record<NetworkInterfaceKind, number> = {
  lan: 0,
  tailscale: 1,
  external: 2,
  localhost: 3,
};

export const NETWORK_KIND_LABEL: Record<NetworkInterfaceKind, string> = {
  lan: 'Rede Local',
  tailscale: 'Tailscale',
  external: 'IP Externo',
  localhost: 'Este computador',
};

const NETWORK_KIND_HINT: Record<NetworkInterfaceKind, string> = {
  lan: 'Conecte o smartphone no mesmo Wi-Fi da igreja para abrir o link.',
  tailscale: 'O smartphone precisa estar conectado na mesma Tailnet.',
  external: 'Requer porta 18080 redirecionada no roteador da igreja.',
  localhost: 'Este link funciona apenas neste computador.',
};

/**
 * Mantém uma única interface por tipo de rede (LAN, Tailscale, Externo, local),
 * preservando a ordenação de prioridade devolvida pelo backend.
 */
export function networkOptionsFrom(
  interfaces: NetworkInterfaceInfo[] | undefined
): NetworkInterfaceInfo[] {
  if (!interfaces || interfaces.length === 0) return [];

  const byKind = new Map<NetworkInterfaceKind, NetworkInterfaceInfo>();
  for (const networkInterface of interfaces) {
    if (!byKind.has(networkInterface.kind)) {
      byKind.set(networkInterface.kind, networkInterface);
    }
  }

  return [...byKind.values()].sort(
    (a, b) => (KIND_RANK[a.kind] ?? Number.MAX_SAFE_INTEGER) - (KIND_RANK[b.kind] ?? Number.MAX_SAFE_INTEGER)
  );
}

/**
 * Resolve a interface efetivamente usada nos links. A seleção `default` aponta
 * para a primeira opção (LAN > Tailscale > Externo), e um id desconhecido
 * (interface removida do sistema) também recai na padrão.
 */
export function resolveNetworkInterface(
  interfaces: NetworkInterfaceInfo[] | undefined,
  selectedInterfaceId: string
): NetworkInterfaceInfo | null {
  const options = networkOptionsFrom(interfaces);
  if (options.length === 0) return null;

  if (selectedInterfaceId && selectedInterfaceId !== DEFAULT_NETWORK_SELECTION) {
    const match = options.find((option) => option.id === selectedInterfaceId);
    if (match) return match;
  }

  return options[0];
}

export function networkHintFor(kind: NetworkInterfaceKind | null | undefined): string {
  return kind ? NETWORK_KIND_HINT[kind] : '';
}

export function readStoredNetworkSelection(): string {
  if (typeof window === 'undefined') return DEFAULT_NETWORK_SELECTION;
  try {
    return window.localStorage.getItem(NETWORK_SELECTION_STORAGE_KEY) || DEFAULT_NETWORK_SELECTION;
  } catch {
    // Ignora navegadores com localStorage bloqueado.
    return DEFAULT_NETWORK_SELECTION;
  }
}

export function storeNetworkSelection(id: string) {
  if (typeof window === 'undefined') return;
  try {
    if (id === DEFAULT_NETWORK_SELECTION) {
      window.localStorage.removeItem(NETWORK_SELECTION_STORAGE_KEY);
    } else {
      window.localStorage.setItem(NETWORK_SELECTION_STORAGE_KEY, id);
    }
  } catch {
    // Ignora navegadores com localStorage bloqueado.
  }
}

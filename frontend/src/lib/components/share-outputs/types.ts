export type { NetworkInterfaceInfo, NetworkInterfaceKind } from '$lib/types';

export type ShareOutputTab = 'stage' | 'public' | 'remote' | 'lower' | 'cast';

export interface ShareOutputsCallbacks {
  onclose: () => void;
}

export interface ShareOutputsModalProps {
  open: boolean;
  initialTab?: ShareOutputTab;
  onclose: () => void;
}

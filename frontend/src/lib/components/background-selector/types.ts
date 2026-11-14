export type BackgroundCategory = 'all' | 'gradients' | 'mesh' | 'animated' | 'custom' | 'local';

export interface BackgroundSelectorCallbacks {
  onselect: (encodedValue: string) => void;
  onclose: () => void;
}

export interface BackgroundSelectorProps {
  open: boolean;
  currentValue?: string;
  onselect: (encodedValue: string) => void;
  onclose: () => void;
}

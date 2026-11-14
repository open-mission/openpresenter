export type CommandTab = 'local' | 'bible' | 'online';

export const COMMAND_TABS: readonly CommandTab[] = ['local', 'bible', 'online'];

export const LISTBOX_ID = 'command-palette-listbox';

export function panelId(tab: CommandTab): string {
  return `command-palette-panel-${tab}`;
}

export function tabId(tab: CommandTab): string {
  return `command-palette-tab-${tab}`;
}

export function optionId(tab: CommandTab, index: number): string {
  return `command-palette-option-${tab}-${index}`;
}

export function optionIdPrefix(tab: CommandTab): string {
  return `command-palette-option-${tab}`;
}

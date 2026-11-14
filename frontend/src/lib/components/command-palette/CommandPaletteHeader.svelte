<script lang="ts">
  import { Loader2, Search, X } from 'lucide-svelte';
  import { LISTBOX_ID } from './types';

  interface Props {
    query: string;
    placeholder: string;
    loading?: boolean;
    activedescendant?: string;
    onquerychange: (value: string) => void;
    onclear: () => void;
    onclose: () => void;
  }

  let {
    query,
    placeholder,
    loading = false,
    activedescendant,
    onquerychange,
    onclear,
    onclose,
  }: Props = $props();

  let inputElement = $state<HTMLInputElement | null>(null);

  export function focus() {
    inputElement?.focus();
  }

  function clearSearch() {
    onclear();
    inputElement?.focus();
  }
</script>

<div class="flex h-14 shrink-0 items-center gap-3 border-b border-border bg-card px-4">
  <Search size={18} class="shrink-0 text-muted-foreground" aria-hidden="true" />

  <input
    bind:this={inputElement}
    type="text"
    value={query}
    oninput={(event) => onquerychange(event.currentTarget.value)}
    {placeholder}
    role="combobox"
    aria-label="Busca global"
    aria-autocomplete="list"
    aria-expanded="true"
    aria-haspopup="listbox"
    aria-controls={LISTBOX_ID}
    aria-activedescendant={activedescendant}
    autocomplete="off"
    spellcheck="false"
    class="min-h-9 flex-1 border-0 bg-transparent text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-0"
  />

  {#if loading}
    <Loader2 size={16} class="shrink-0 animate-spin text-emerald-600 dark:text-emerald-400" aria-hidden="true" />
    <span class="sr-only" role="status">Buscando resultados</span>
  {/if}

  {#if query}
    <button
      type="button"
      onclick={clearSearch}
      aria-label="Limpar busca"
      title="Limpar busca"
      class="flex h-7 w-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
    >
      <X size={15} />
    </button>
  {/if}

  <kbd
    class="hidden shrink-0 rounded border border-border bg-muted px-1.5 py-0.5 font-mono text-[11px] text-muted-foreground sm:inline-block"
  >
    Esc
  </kbd>

  <button
    type="button"
    onclick={onclose}
    aria-label="Fechar paleta de comandos"
    title="Fechar (Esc)"
    class="flex h-7 w-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
  >
    <X size={16} />
  </button>
</div>

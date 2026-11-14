<script lang="ts">
  import { FileText, Globe, RefreshCw } from 'lucide-svelte';
  import type { OnlineLyricHit } from '../../types';
  import Badge from '../ui/Badge.svelte';
  import { LISTBOX_ID, optionId } from './types';

  interface Props {
    query: string;
    loading: boolean;
    error: string | null;
    results: OnlineLyricHit[];
    selectedIndex: number;
    onselect: (index: number) => void;
    onactivate: (index: number) => void;
    onretry: () => void;
  }

  let { query, loading, error, results, selectedIndex, onselect, onactivate, onretry }: Props =
    $props();

  function optionClasses(index: number): string {
    return selectedIndex === index ? 'bg-muted ring-1 ring-ring/60' : 'hover:bg-muted/60';
  }

  function handleRowKeydown(event: KeyboardEvent, index: number) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onactivate(index);
    }
  }
</script>

{#if error}
  <div
    role="alert"
    class="mb-2 flex flex-wrap items-center justify-between gap-2 rounded-lg border border-red-500/35 bg-red-500/10 px-3 py-2 text-[11px] font-medium text-red-700 dark:text-red-300"
  >
    <span class="min-w-0">{error}</span>
    <button
      type="button"
      onclick={onretry}
      class="flex min-h-7 shrink-0 cursor-pointer items-center gap-1.5 rounded border border-red-500/35 px-2 text-[11px] font-semibold text-red-700 transition-colors hover:bg-red-500/15 dark:text-red-300"
    >
      <RefreshCw size={12} aria-hidden="true" />
      Tentar novamente
    </button>
  </div>
{/if}

<div id={LISTBOX_ID} role="listbox" aria-label="Letras na internet" class="space-y-0.5">
  {#if results.length > 0}
    <div
      role="group"
      aria-label={`Resultados da internet (${results.length})`}
      class="space-y-0.5"
    >
      <div
        aria-hidden="true"
        class="px-2 py-1 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground"
      >
        Resultados da internet ({results.length})
      </div>

      {#each results as hit, index (hit.provider_id || index)}
        <div
          role="group"
          onmouseenter={() => onselect(index)}
          class="flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2.5 transition-colors {optionClasses(
            index
          )}"
        >
          <div
            id={optionId('online', index)}
            role="option"
            aria-selected={selectedIndex === index}
            tabindex="-1"
            onclick={() => onactivate(index)}
            onkeydown={(event) => handleRowKeydown(event, index)}
            class="flex min-w-0 flex-1 cursor-pointer items-start gap-2.5 pr-2 text-left"
          >
            <div
              class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded bg-sky-500/15 text-sky-700 dark:text-sky-300"
            >
              <Globe size={13} aria-hidden="true" />
            </div>
            <div class="min-w-0">
              <div class="flex flex-wrap items-center gap-2">
                <span class="truncate text-xs font-medium text-foreground">{hit.track_name}</span>
                {#if hit.provider === 'holyrics'}
                  <Badge variant="success" class="font-mono">Holyrics</Badge>
                {:else}
                  <Badge variant="outline" class="font-mono text-sky-700 dark:text-sky-300">
                    Vagalume
                  </Badge>
                {/if}
              </div>
              <div class="truncate text-[11px] text-muted-foreground">
                {hit.artist_name || 'Desconhecido'}{hit.album_name ? ` — ${hit.album_name}` : ''}
              </div>
            </div>
          </div>

          <button
            type="button"
            onclick={() => onactivate(index)}
            aria-label={`Importar e editar ${hit.track_name}`}
            class="flex min-h-7 shrink-0 cursor-pointer items-center gap-1.5 rounded border border-border bg-muted px-2.5 text-[11px] font-medium text-foreground transition-colors hover:bg-sky-600 hover:text-sky-50"
          >
            <FileText size={12} aria-hidden="true" />
            <span>Importar e Editar</span>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if results.length === 0}
  {#if loading}
    <div class="p-8 text-center text-xs text-muted-foreground" role="status">
      Buscando letras na internet (Holyrics e Vagalume)...
    </div>
  {:else if query}
    <div class="p-8 text-center text-xs text-muted-foreground">
      Nenhum resultado online para “{query}”. Tente buscar por outros termos ou crie a música
      manualmente.
    </div>
  {:else}
    <div class="p-8 text-center text-xs text-muted-foreground">
      Digite o nome de um louvor, artista ou trecho para pesquisar online.
    </div>
  {/if}
{/if}

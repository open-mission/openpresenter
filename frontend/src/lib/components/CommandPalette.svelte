<script lang="ts">
  import { onDestroy } from 'svelte';
  import type { ParsedReference, Song } from '../types';
  import BibleSearchResults from './command-palette/BibleSearchResults.svelte';
  import CommandPaletteHeader from './command-palette/CommandPaletteHeader.svelte';
  import CommandPaletteShortcuts from './command-palette/CommandPaletteShortcuts.svelte';
  import CommandPaletteTabs from './command-palette/CommandPaletteTabs.svelte';
  import LocalSongsResults from './command-palette/LocalSongsResults.svelte';
  import OnlineLyricsResults from './command-palette/OnlineLyricsResults.svelte';
  import { CommandPaletteController } from './command-palette/controller.svelte';
  import { panelId, tabId, COMMAND_TABS } from './command-palette/types';

  interface Props {
    open: boolean;
    onclose: () => void;
    oncreatenew: () => void;
    onimportsong: (song: Song) => void;
    onopenbible?: (ref: ParsedReference) => void;
    onpresentscripture?: (ref: ParsedReference) => void;
  }

  let {
    open = false,
    onclose,
    oncreatenew,
    onimportsong,
    onopenbible,
    onpresentscripture,
  }: Props = $props();

  let header = $state<{ focus: () => void } | null>(null);

  const palette = new CommandPaletteController({
    close: () => onclose(),
    createNew: () => oncreatenew(),
    importSong: (song) => onimportsong(song),
    openBible: (ref) => onopenbible?.(ref),
    presentScripture: (ref) => onpresentscripture?.(ref),
  });

  palette.attachInputFocus(() => header?.focus());

  const placeholder = $derived(
    palette.activeTab === 'local'
      ? 'Buscar música por título ou letra, ou escolha uma ação rápida...'
      : palette.activeTab === 'bible'
        ? 'Buscar passagem (ex: João 3:16, Sl 23:1-4) ou palavras-chave...'
        : 'Buscar na internet (Holyrics, Vagalume)...'
  );

  $effect(() => {
    palette.syncOpen(open);
  });

  $effect(() => {
    const isOpen = open;
    const tab = palette.activeTab;
    const value = palette.query;
    return palette.syncQuery(isOpen, tab, value);
  });

  onDestroy(() => palette.destroy());
</script>

<svelte:window onkeydown={(event) => open && palette.handleKeydown(event)} />

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-start justify-center bg-background/80 px-4 pt-[10vh] backdrop-blur-sm"
    role="presentation"
    onclick={(event) => {
      if (event.target === event.currentTarget) onclose();
    }}
  >
    <div
      class="flex w-full max-w-2xl flex-col overflow-hidden rounded-xl border border-border bg-popover text-popover-foreground shadow-2xl"
      role="dialog"
      aria-modal="true"
      aria-label="Paleta de comandos"
      tabindex="-1"
    >
      <CommandPaletteHeader
        bind:this={header}
        query={palette.query}
        loading={palette.isLoading}
        activedescendant={palette.activeDescendant}
        {placeholder}
        onquerychange={(value) => (palette.query = value)}
        onclear={() => (palette.query = '')}
        onclose={onclose}
      />

      <div class="flex shrink-0 flex-wrap items-center justify-between gap-2 border-b border-border bg-muted/30 px-4 py-1.5">
        <CommandPaletteTabs
          activeTab={palette.activeTab}
          localCount={palette.localResults.length}
          bibleCount={palette.bibleCount}
          onlineCount={palette.onlineResults.length}
          onselect={palette.switchTab}
        />
        <span class="hidden text-[11px] text-muted-foreground sm:inline">Busca global ⌘K</span>
      </div>

      {#each COMMAND_TABS as tab (tab)}
        <div
          id={panelId(tab)}
          role="tabpanel"
          aria-labelledby={tabId(tab)}
          hidden={palette.activeTab !== tab}
          class="max-h-[min(60vh,34rem)] overflow-y-auto p-2"
        >
          {#if palette.activeTab === tab}
            {#if tab === 'local'}
              <LocalSongsResults
                query={palette.query}
                results={palette.localResults}
                selectedIndex={palette.selectedIndex}
                addedIds={palette.addedIds}
                showQuickActions={palette.showQuickActions}
                internetFallback={palette.internetFallback}
                detectedReference={palette.parsedBibleRef}
                onselect={(index) => (palette.selectedIndex = index)}
                onactivate={palette.activateLocal}
                onaddsong={palette.addToSetlist}
                onopenreference={palette.openDetectedReference}
                onpresentreference={palette.presentDetectedReference}
              />
            {:else if tab === 'bible'}
              <BibleSearchResults
                query={palette.query}
                loading={palette.isLoading}
                error={palette.bibleError}
                bibles={palette.bibles}
                selectedBibleId={palette.selectedBibleId}
                currentBible={palette.currentBible}
                passageRef={palette.biblePassageRef}
                verses={palette.bibleVerses}
                results={palette.bibleResults}
                selectedIndex={palette.selectedIndex}
                addedKeys={palette.addedVerseKeys}
                onversionchange={palette.handleVersionChange}
                onselect={(index) => (palette.selectedIndex = index)}
                onactivate={palette.handleBibleActivate}
                onaddsecondary={palette.handleBibleAdd}
                onretry={palette.retry}
                onsearch={palette.searchExample}
              />
            {:else}
              <OnlineLyricsResults
                query={palette.query}
                loading={palette.isLoading}
                error={palette.onlineError}
                results={palette.onlineResults}
                selectedIndex={palette.selectedIndex}
                onselect={(index) => (palette.selectedIndex = index)}
                onactivate={palette.activateOnline}
                onretry={palette.retry}
              />
            {/if}
          {/if}
        </div>
      {/each}

      <CommandPaletteShortcuts activeTab={palette.activeTab} />
    </div>
  </div>
{/if}

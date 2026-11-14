<script lang="ts">
  import { FolderPlus, FolderOpen, Plus, RefreshCw, X } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import { createMediaDirectory } from '../../media';
  import { notifyError, notifySuccess } from '../../toast';

  interface Props {
    open: boolean;
    kind: 'images' | 'videos';
    folderId: string;
    parentSubfolder: string | null;
    locationLabel: string;
    oncreated?: (folderId: string) => Promise<void> | void;
    onclose: () => void;
  }

  let {
    open,
    kind,
    folderId,
    parentSubfolder,
    locationLabel,
    oncreated,
    onclose,
  }: Props = $props();

  let newFolderName = $state('');
  let isCreating = $state(false);
  let dialogEl: HTMLDivElement | null = $state(null);
  let inputEl: HTMLInputElement | null = $state(null);
  let previouslyFocused: HTMLElement | null = null;

  $effect(() => {
    if (open) {
      newFolderName = '';
      previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
      setTimeout(() => {
        inputEl?.focus();
      }, 20);
    } else if (previouslyFocused) {
      previouslyFocused.focus();
      previouslyFocused = null;
    }
  });

  function handleKeyDown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      if (!isCreating) onclose();
      return;
    }
    if (e.key === 'Tab' && dialogEl) {
      const focusable = Array.from(
        dialogEl.querySelectorAll<HTMLElement>(
          'button:not([disabled]), input:not([disabled]), [tabindex]:not([tabindex="-1"])'
        )
      );
      if (focusable.length === 0) return;
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  }

  async function handleSubmit() {
    const name = newFolderName.trim();
    if (!name || isCreating) return;

    isCreating = true;
    try {
      const resolvedFolderId = folderId && folderId !== 'all' ? folderId : 'app';
      await createMediaDirectory(name, kind, resolvedFolderId, parentSubfolder);
      notifySuccess('Pasta Criada', `A pasta "${name}" foi criada com sucesso.`);
      newFolderName = '';
      onclose();
      await oncreated?.(resolvedFolderId);
    } catch (err: any) {
      console.error('Erro ao criar pasta:', err);
      notifyError('Erro ao Criar Pasta', err?.message || 'Falha ao criar diretório.');
    } finally {
      isCreating = false;
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4 animate-in fade-in duration-150"
    onclick={() => !isCreating && onclose()}
    onkeydown={handleKeyDown}
    role="presentation"
  >
    <div
      bind:this={dialogEl}
      class="w-full max-w-sm rounded-xl border border-border bg-card text-card-foreground shadow-2xl flex flex-col overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeyDown}
      role="dialog"
      aria-modal="true"
      aria-labelledby="create-folder-title"
      tabindex="-1"
    >
      <div class="h-12 px-4 border-b border-border flex items-center justify-between shrink-0 bg-muted/40">
        <div class="flex items-center gap-2">
          <div
            class="w-7 h-7 rounded-lg border flex items-center justify-center {kind === 'videos' ? 'bg-purple-500/15 border-purple-500/30 text-purple-700 dark:text-purple-300' : 'bg-amber-500/15 border-amber-500/30 text-amber-700 dark:text-amber-300'}"
          >
            <FolderPlus size={15} />
          </div>
          <h3 id="create-folder-title" class="text-xs font-bold text-foreground">Nova Pasta</h3>
        </div>
        <button
          type="button"
          onclick={() => !isCreating && onclose()}
          aria-label="Fechar janela de nova pasta"
          class="w-8 h-8 rounded-md hover:bg-muted flex items-center justify-center text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
        >
          <X size={15} />
        </button>
      </div>

      <form
        onsubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
        class="p-4 space-y-3.5"
      >
        <div class="space-y-1">
          <span class="text-[11px] text-muted-foreground uppercase font-bold tracking-wider">
            Localização:
          </span>
          <div class="p-2 rounded-lg bg-muted/60 border border-border text-[11px] text-foreground font-mono truncate flex items-center gap-1.5">
            <FolderOpen
              size={12}
              class="{kind === 'videos' ? 'text-purple-700 dark:text-purple-300' : 'text-amber-700 dark:text-amber-300'} shrink-0"
            />
            <span class="truncate">{locationLabel}</span>
          </div>
        </div>

        <div class="space-y-1.5">
          <label for="new-folder-input" class="text-xs font-semibold text-foreground block">
            Nome da Pasta
          </label>
          <input
            id="new-folder-input"
            type="text"
            bind:this={inputEl}
            bind:value={newFolderName}
            placeholder="Ex: Culto da Noite, Anúncios, 2026..."
            maxlength={60}
            class="w-full h-8 px-3 rounded-lg bg-background border border-input text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
          />
        </div>

        <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/60">
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onclick={() => !isCreating && onclose()}
            disabled={isCreating}
            class="h-8 text-xs"
          >
            Cancelar
          </Button>
          <Button
            type="submit"
            variant="default"
            size="sm"
            disabled={!newFolderName.trim() || isCreating}
            class="h-8 text-xs font-semibold gap-1.5 {kind === 'videos' ? 'bg-purple-600 hover:bg-purple-700 text-white' : ''}"
          >
            {#if isCreating}
              <RefreshCw size={12} class="animate-spin" />
              <span>Criando...</span>
            {:else}
              <Plus size={13} />
              <span>Criar Pasta</span>
            {/if}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}

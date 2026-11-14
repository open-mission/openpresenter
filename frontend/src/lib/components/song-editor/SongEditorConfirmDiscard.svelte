<script lang="ts">
  import { onDestroy } from 'svelte';
  import { AlertTriangle, X } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import { focusInitialElement, trapTabKey } from '../focusTrap';

  interface Props {
    open: boolean;
    onconfirm: () => void;
    oncancel: () => void;
  }

  let { open, onconfirm, oncancel }: Props = $props();

  const uid = $props.id();
  const titleId = `${uid}-title`;
  const descriptionId = `${uid}-description`;

  let dialogEl: HTMLDivElement | null = $state(null);
  let cancelButtonEl: HTMLButtonElement | null = $state(null);
  let previouslyFocused: HTMLElement | null = null;

  function restoreFocus() {
    if (previouslyFocused && document.contains(previouslyFocused)) {
      previouslyFocused.focus();
    }
    previouslyFocused = null;
  }

  $effect(() => {
    if (open) {
      previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
      const timer = setTimeout(() => {
        if (dialogEl) focusInitialElement(dialogEl, '[data-autofocus]', cancelButtonEl);
      }, 10);
      return () => clearTimeout(timer);
    }

    restoreFocus();
  });

  onDestroy(restoreFocus);

  function handleKeydown(event: KeyboardEvent) {
    if (!open) return;
    if (event.key === 'Escape') {
      event.preventDefault();
      event.stopPropagation();
      oncancel();
      return;
    }
    if (dialogEl) trapTabKey(event, dialogEl);
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-[60] bg-background/80 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="presentation"
    onclick={(event) => {
      event.stopPropagation();
      oncancel();
    }}
  >
    <div
      bind:this={dialogEl}
      class="w-full max-w-md rounded-xl border border-border bg-card text-card-foreground shadow-2xl flex flex-col overflow-hidden"
      onclick={(event) => event.stopPropagation()}
      onkeydown={handleKeydown}
      role="alertdialog"
      aria-modal="true"
      aria-labelledby={titleId}
      aria-describedby={descriptionId}
      tabindex="-1"
    >
      <div class="px-4 py-3 border-b border-border flex items-start justify-between gap-3 shrink-0 bg-muted/40">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="w-8 h-8 rounded-lg border border-red-500/35 bg-red-500/10 text-red-700 dark:text-red-300 flex items-center justify-center shrink-0"
            aria-hidden="true"
          >
            <AlertTriangle size={16} />
          </div>
          <div class="min-w-0">
            <h3 id={titleId} class="text-sm font-bold text-foreground">Descartar alterações?</h3>
            <p class="text-[11px] text-muted-foreground">
              Esta ação não pode ser desfeita.
            </p>
          </div>
        </div>
        <button
          type="button"
          onclick={oncancel}
          class="w-8 h-8 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          aria-label="Continuar editando"
          title="Continuar editando"
        >
          <X size={15} />
        </button>
      </div>

      <div class="p-4 space-y-4">
        <p id={descriptionId} class="text-xs text-muted-foreground leading-relaxed">
          Existem alterações não salvas neste conteúdo. Se você sair agora, tudo o que foi digitado
          desde a última gravação será perdido.
        </p>

        <div class="flex flex-wrap items-center justify-end gap-2 pt-2 border-t border-border/60">
          <button
            type="button"
            bind:this={cancelButtonEl}
            data-autofocus
            onclick={oncancel}
            class="h-9 px-3 rounded-lg border border-border bg-background hover:bg-muted text-xs font-medium text-foreground transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          >
            Continuar editando
          </button>
          <Button
            type="button"
            variant="destructive"
            size="sm"
            onclick={onconfirm}
            class="h-9 px-3 text-xs font-semibold"
          >
            Descartar e sair
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import { AlertTriangle, X } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';

  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    destructive?: boolean;
    isProcessing?: boolean;
    onconfirm: () => void | Promise<void>;
    onclose: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = 'Confirmar',
    cancelLabel = 'Cancelar',
    destructive = true,
    isProcessing = false,
    onconfirm,
    onclose,
  }: Props = $props();

  let dialogEl: HTMLDivElement | null = $state(null);
  let cancelBtnEl: HTMLButtonElement | null = $state(null);
  let previouslyFocused: HTMLElement | null = null;

  $effect(() => {
    if (open) {
      previouslyFocused =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
      setTimeout(() => {
        cancelBtnEl?.focus();
      }, 10);
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
      if (!isProcessing) onclose();
      return;
    }
    if (e.key === 'Tab' && dialogEl) {
      const focusable = Array.from(
        dialogEl.querySelectorAll<HTMLElement>(
          'button:not([disabled]), [href], input:not([disabled]), [tabindex]:not([tabindex="-1"])'
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
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4 animate-in fade-in duration-150"
    onclick={() => !isProcessing && onclose()}
    onkeydown={handleKeyDown}
    role="presentation"
  >
    <div
      bind:this={dialogEl}
      class="w-full max-w-sm rounded-xl border border-border bg-card text-card-foreground shadow-2xl flex flex-col overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeyDown}
      role="alertdialog"
      aria-modal="true"
      aria-labelledby="confirm-modal-title"
      aria-describedby="confirm-modal-desc"
      tabindex="-1"
    >
      <div class="h-12 px-4 border-b border-border flex items-center justify-between shrink-0 bg-muted/40">
        <div class="flex items-center gap-2 min-w-0">
          <div
            class="w-7 h-7 rounded-lg border flex items-center justify-center shrink-0 {destructive ? 'bg-red-500/15 border-red-500/35 text-red-700 dark:text-red-300' : 'bg-primary/15 border-primary/30 text-primary'}"
          >
            <AlertTriangle size={15} />
          </div>
          <h3 id="confirm-modal-title" class="text-xs font-bold text-foreground truncate">
            {title}
          </h3>
        </div>
        <button
          type="button"
          onclick={() => !isProcessing && onclose()}
          aria-label="Fechar janela de confirmação"
          class="w-8 h-8 rounded-md hover:bg-muted flex items-center justify-center text-muted-foreground hover:text-foreground transition-colors cursor-pointer shrink-0"
        >
          <X size={15} />
        </button>
      </div>

      <div class="p-4 space-y-4">
        <p id="confirm-modal-desc" class="text-xs text-muted-foreground leading-relaxed whitespace-pre-line">
          {message}
        </p>

        <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/60">
          <button
            type="button"
            bind:this={cancelBtnEl}
            onclick={() => !isProcessing && onclose()}
            disabled={isProcessing}
            class="h-8 px-3 rounded-lg border border-border bg-background hover:bg-muted text-xs font-medium text-foreground transition-colors cursor-pointer disabled:opacity-50"
          >
            {cancelLabel}
          </button>
          <Button
            type="button"
            variant={destructive ? 'destructive' : 'default'}
            size="sm"
            onclick={() => onconfirm()}
            disabled={isProcessing}
            class="h-8 px-3 text-xs font-semibold"
          >
            {isProcessing ? 'Processando...' : confirmLabel}
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}

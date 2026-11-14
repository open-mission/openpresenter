<script lang="ts">
  import { onDestroy } from 'svelte';
  import { AlertTriangle } from 'lucide-svelte';
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
  const descId = `${uid}-desc`;

  let dialogEl: HTMLDivElement | null = $state(null);
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
        if (dialogEl) focusInitialElement(dialogEl, '#sched-discard-cancel-btn', dialogEl);
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
      aria-describedby={descId}
      tabindex="-1"
    >
      <div class="p-5 flex items-start gap-3.5">
        <div
          class="w-10 h-10 rounded-full bg-amber-500/15 border border-amber-600/35 text-amber-700 dark:text-amber-300 flex items-center justify-center shrink-0"
          aria-hidden="true"
        >
          <AlertTriangle size={20} />
        </div>
        <div class="min-w-0 flex-1">
          <h3 id={titleId} class="text-sm font-semibold text-foreground">
            Descartar alterações na agenda?
          </h3>
          <p id={descId} class="text-xs text-muted-foreground mt-1.5 leading-relaxed">
            Existem alterações não salvas na programação semanal. Se você sair agora, todos os dados modificados serão perdidos.
          </p>
        </div>
      </div>

      <div class="px-5 py-3 border-t border-border bg-muted/40 flex items-center justify-end gap-2 shrink-0">
        <Button
          variant="ghost"
          size="sm"
          id="sched-discard-cancel-btn"
          onclick={oncancel}
          class="text-xs min-h-[32px]"
        >
          Continuar editando
        </Button>
        <Button
          variant="destructive"
          size="sm"
          onclick={onconfirm}
          class="text-xs min-h-[32px]"
        >
          Descartar e sair
        </Button>
      </div>
    </div>
  </div>
{/if}

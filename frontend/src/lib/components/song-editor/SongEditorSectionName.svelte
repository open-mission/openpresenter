<script lang="ts">
  import { onDestroy } from 'svelte';
  import { Hash, X } from 'lucide-svelte';
  import Button from '../ui/Button.svelte';
  import { focusInitialElement, trapTabKey } from '../focusTrap';

  interface Props {
    open: boolean;
    value?: string;
    error?: string;
    isInsert?: boolean;
    oninput?: () => void;
    onconfirm: () => void;
    oncancel: () => void;
  }

  let {
    open,
    value = $bindable(''),
    error = '',
    isInsert = true,
    oninput,
    onconfirm,
    oncancel,
  }: Props = $props();

  const uid = $props.id();
  const titleId = `${uid}-title`;
  const descriptionId = `${uid}-description`;
  const inputId = `${uid}-input`;
  const errorId = `${uid}-error`;

  let dialogEl: HTMLDivElement | null = $state(null);
  let inputEl: HTMLInputElement | null = $state(null);
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
        if (dialogEl) focusInitialElement(dialogEl, `#${inputId}`, inputEl);
        inputEl?.select();
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

  function submit(event: SubmitEvent) {
    event.preventDefault();
    onconfirm();
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
      role="dialog"
      aria-modal="true"
      aria-labelledby={titleId}
      aria-describedby={descriptionId}
      tabindex="-1"
    >
      <div
        class="px-4 py-3 border-b border-border flex items-start justify-between gap-3 shrink-0 bg-muted/40"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="w-8 h-8 rounded-lg border border-primary/35 bg-primary/10 text-primary flex items-center justify-center shrink-0"
            aria-hidden="true"
          >
            <Hash size={16} />
          </div>
          <div class="min-w-0">
            <h3 id={titleId} class="text-sm font-bold text-foreground">Nova seção</h3>
            <p class="text-[11px] text-muted-foreground">
              {isInsert ? 'Insere a tag no cursor da letra.' : 'Aplica a nova seção ao slide.'}
            </p>
          </div>
        </div>
        <button
          type="button"
          onclick={oncancel}
          class="w-8 h-8 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors cursor-pointer shrink-0 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          aria-label="Cancelar nova seção"
          title="Cancelar"
        >
          <X size={15} />
        </button>
      </div>

      <form class="p-4 space-y-4" onsubmit={submit}>
        <div>
          <p id={descriptionId} class="text-xs text-muted-foreground leading-relaxed mb-2">
            Digite um nome para a seção. Exemplos: Ministração, Espontâneo, Refrão 2, Solo.
          </p>
          <label for={inputId} class="block text-[11px] font-medium text-muted-foreground mb-1">
            Nome da seção
          </label>
          <input
            id={inputId}
            bind:this={inputEl}
            bind:value
            oninput={oninput}
            type="text"
            maxlength="40"
            placeholder="Ex: Ministração"
            class="w-full h-9 px-3 rounded-lg bg-background border border-border text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary/40"
            aria-invalid={Boolean(error)}
            aria-describedby={error ? errorId : descriptionId}
          />
          {#if error}
            <p id={errorId} role="alert" class="mt-1 text-[11px] text-red-700 dark:text-red-300">
              {error}
            </p>
          {/if}
        </div>

        <div class="flex flex-wrap items-center justify-end gap-2 pt-2 border-t border-border/60">
          <button
            type="button"
            onclick={oncancel}
            class="h-9 px-3 rounded-lg border border-border bg-background hover:bg-muted text-xs font-medium text-foreground transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          >
            Cancelar
          </button>
          <Button type="submit" variant="default" size="sm" class="h-9 px-3 text-xs font-semibold">
            {isInsert ? 'Inserir seção' : 'Aplicar seção'}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}

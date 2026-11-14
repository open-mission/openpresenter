<script lang="ts">
  import { tick } from 'svelte';
  import { BookOpen, RefreshCw } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import SettingsToggle from './SettingsToggle.svelte';

  interface Props {
    open: boolean;
    name: string;
    abbr: string;
    isDefault: boolean;
    error: string;
    isSaving: boolean;
    onUpdateName: (val: string) => void;
    onUpdateAbbr: (val: string) => void;
    onUpdateIsDefault: (val: boolean) => void;
    onCancel: () => void;
    onSave: () => void;
  }

  let {
    open,
    name,
    abbr,
    isDefault,
    error,
    isSaving,
    onUpdateName,
    onUpdateAbbr,
    onUpdateIsDefault,
    onCancel,
    onSave,
  }: Props = $props();

  let nameInputEl = $state<HTMLInputElement | null>(null);
  let lastFocusedBeforeEdit: HTMLElement | null = null;
  let wasOpen = false;

  $effect(() => {
    if (open && !wasOpen) {
      wasOpen = true;
      lastFocusedBeforeEdit =
        typeof document !== 'undefined' ? (document.activeElement as HTMLElement | null) : null;
      tick().then(() => {
        nameInputEl?.focus();
      });
    } else if (!open && wasOpen) {
      wasOpen = false;
      tick().then(() => {
        lastFocusedBeforeEdit?.focus?.();
        lastFocusedBeforeEdit = null;
      });
    }
  });

  function handleWindowKeydown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      onCancel();
    }
  }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

{#if open}
  <div
    class="fixed inset-0 z-[60] bg-black/75 backdrop-blur-sm flex items-center justify-center p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="bible-edit-modal-title"
    tabindex="-1"
    onclick={(e) => {
      if (e.target === e.currentTarget) onCancel();
    }}
    onkeydown={(e) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        e.stopPropagation();
        onCancel();
      }
    }}
  >
    <div class="w-full max-w-sm bg-card text-foreground border border-border rounded-xl shadow-2xl overflow-hidden flex flex-col">
      <div class="px-5 py-3.5 border-b border-border bg-card">
        <h4 id="bible-edit-modal-title" class="text-sm font-bold text-foreground flex items-center gap-2">
          <BookOpen size={16} class="text-primary shrink-0" />
          Editar Versão da Bíblia
        </h4>
        <p class="text-xs text-muted-foreground mt-0.5">
          Ajuste o nome de exibição e a sigla canônica da versão (ex: NVI, ARA, ARC, NVT).
        </p>
      </div>

      <form
        class="p-5 space-y-3.5"
        onsubmit={(e) => {
          e.preventDefault();
          onSave();
        }}
      >
        <div class="space-y-1">
          <label for="edit-bible-name" class="block text-xs font-semibold text-foreground">
            Nome da Versão
          </label>
          <input
            id="edit-bible-name"
            bind:this={nameInputEl}
            type="text"
            value={name}
            oninput={(e) => onUpdateName((e.target as HTMLInputElement).value)}
            placeholder="Ex: Almeida Revista e Atualizada"
            class="w-full h-8 px-2.5 rounded-md bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>

        <div class="space-y-1">
          <label for="edit-bible-abbr" class="block text-xs font-semibold text-foreground">
            Sigla / Abreviação
          </label>
          <input
            id="edit-bible-abbr"
            type="text"
            value={abbr}
            oninput={(e) => onUpdateAbbr((e.target as HTMLInputElement).value.toUpperCase())}
            placeholder="Ex: ARA"
            maxlength="10"
            class="w-full h-8 px-2.5 rounded-md bg-background border border-border text-xs text-foreground placeholder:text-muted-foreground uppercase font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>

        <div class="flex items-center justify-between gap-3 pt-1">
          <label for="edit-bible-default-switch" class="text-xs font-medium text-foreground cursor-pointer">
            Definir como versão padrão
          </label>
          <SettingsToggle
            id="edit-bible-default-switch"
            checked={isDefault}
            ariaLabel="Definir como versão padrão"
            onToggle={() => onUpdateIsDefault(!isDefault)}
          />
        </div>

        {#if error}
          <div
            role="alert"
            class="p-2.5 rounded-lg bg-red-500/10 border border-red-500/35 text-red-700 dark:text-red-300 text-xs font-medium"
          >
            {error}
          </div>
        {/if}

        <div class="flex items-center justify-end gap-2 pt-3 border-t border-border">
          <Button type="button" variant="ghost" size="sm" onclick={onCancel} class="text-xs">
            Cancelar
          </Button>
          <Button
            type="submit"
            variant="default"
            size="sm"
            disabled={isSaving}
            class="text-xs font-semibold gap-1.5"
          >
            {#if isSaving}
              <RefreshCw size={12} class="animate-spin" />
            {/if}
            Salvar Alterações
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}

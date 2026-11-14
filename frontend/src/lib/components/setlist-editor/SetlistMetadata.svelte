<script lang="ts">
  import { AlertCircle } from 'lucide-svelte';

  interface Props {
    title: string;
    description: string;
    errorMessage?: string;
  }

  let { title = $bindable(), description = $bindable(), errorMessage = '' }: Props = $props();
</script>

<div class="space-y-4">
  {#if errorMessage}
    <div
      role="alert"
      class="p-3 rounded-lg bg-destructive/10 border border-destructive/30 text-xs text-destructive flex items-center gap-2.5 animate-in fade-in duration-150"
    >
      <AlertCircle size={16} class="shrink-0" aria-hidden="true" />
      <span>{errorMessage}</span>
    </div>
  {/if}

  <!-- Título -->
  <div class="space-y-1.5">
    <label for="setlist-title" class="text-xs font-semibold text-foreground flex items-center gap-1">
      Título da Setlist
      <span class="text-emerald-600 dark:text-emerald-400 font-bold" aria-hidden="true">*</span>
      <span class="sr-only">(obrigatório)</span>
    </label>
    <input
      id="setlist-title"
      type="text"
      bind:value={title}
      placeholder="Ex.: Culto de Domingo - Noite, Culto da Família, Celebração..."
      required
      aria-required="true"
      class="w-full h-10 px-3 bg-background border border-border rounded-lg text-xs sm:text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/30 transition-colors"
    />
  </div>

  <!-- Descrição -->
  <div class="space-y-1.5">
    <label for="setlist-desc" class="text-xs font-semibold text-foreground">
      Descrição / Observações <span class="text-muted-foreground font-normal">(opcional)</span>
    </label>
    <textarea
      id="setlist-desc"
      bind:value={description}
      rows="2"
      placeholder="Ex.: Ministrante: Pr. Paulo. Tema: O Amor de Deus. Equipe de Louvor 1."
      class="w-full p-3 bg-background border border-border rounded-lg text-xs sm:text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/30 transition-colors resize-none leading-relaxed"
    ></textarea>
  </div>
</div>

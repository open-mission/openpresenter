<script lang="ts">
  interface Props {
    subtitle: string;
    onupdatesubtitle: (val: string) => void;
  }

  let { subtitle, onupdatesubtitle }: Props = $props();

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onupdatesubtitle(target.value);
  }
</script>

<div>
  <label for="timer-subtitle" class="block text-xs font-bold text-muted-foreground uppercase tracking-wider mb-1.5">
    Mensagem de Apoio / Subtítulo (opcional)
  </label>
  <input
    id="timer-subtitle"
    type="text"
    value={subtitle}
    oninput={handleInput}
    placeholder="Ex.: O culto começará em instantes. Prepare o seu coração!"
    class="w-full h-10 px-3.5 bg-background border border-border rounded-xl text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:border-amber-500 transition-colors"
  />
  <p class="text-xs text-muted-foreground mt-1">
    Esta frase é projetada logo abaixo dos dígitos da contagem na tela pública.
  </p>
</div>

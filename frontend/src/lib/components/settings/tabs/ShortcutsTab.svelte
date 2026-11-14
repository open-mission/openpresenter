<script lang="ts">
  const col1 = [
    { label: 'Apresentar slide selecionado', keys: ['Enter'], tone: 'emerald' },
    { label: 'Parar apresentação ao vivo', keys: ['Esc'], tone: 'danger' },
    { label: 'Avançar para o próximo slide', keys: ['→', 'Espaço'], tone: 'neutral' },
    { label: 'Retornar ao slide anterior', keys: ['←'], tone: 'neutral' },
    { label: 'Pular direto para slide ordinal', keys: ['[Número]', 'Enter'], joiner: '+', tone: 'neutral' },
    { label: 'Abrir Paleta de Comandos', keys: ['⌘K', 'Ctrl+K'], tone: 'neutral' },
    { label: 'Blackout (tela preta total)', keys: ['.', 'Alt+B'], joiner: 'ou', tone: 'primary' },
  ];

  const col2 = [
    { label: 'Pular para o Refrão', keys: ['R'], tone: 'emerald' },
    { label: 'Pular para o Verso', keys: ['V'], tone: 'blue' },
    { label: 'Pular para a Ponte', keys: ['P'], tone: 'amber' },
    { label: 'Pular para a Introdução', keys: ['I'], tone: 'cyan' },
    { label: 'Pular para o Final / Encerramento', keys: ['F'], tone: 'purple' },
    { label: 'Limpar Texto (mantém o fundo)', keys: [',', 'Alt+C'], joiner: 'ou', tone: 'primary' },
    { label: 'Papel de Parede (Tela de Repouso)', keys: ['/', 'Alt+W'], joiner: 'ou', tone: 'primary' },
  ];

  function kbdToneClass(tone: string): string {
    switch (tone) {
      case 'emerald':
        return 'text-emerald-700 dark:text-emerald-300 border-emerald-500/35';
      case 'danger':
        return 'text-red-700 dark:text-red-300 border-red-500/35';
      case 'primary':
        return 'text-primary font-bold border-primary/40';
      case 'blue':
        return 'text-blue-700 dark:text-blue-300 border-blue-500/35';
      case 'amber':
        return 'text-amber-800 dark:text-amber-300 border-amber-500/35';
      case 'cyan':
        return 'text-cyan-700 dark:text-cyan-300 border-cyan-500/35';
      case 'purple':
        return 'text-purple-700 dark:text-purple-300 border-purple-500/35';
      default:
        return 'text-foreground border-border';
    }
  }
</script>

<div class="space-y-4 w-full max-w-5xl">
  <div>
    <h3 class="text-sm font-semibold text-foreground">Atalhos Operacionais de Teclado</h3>
    <p class="text-xs text-muted-foreground mt-0.5">
      Agilidade durante o culto e eventos ao vivo sem tirar as mãos do teclado.
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-3.5 text-xs">
    {#each [col1, col2] as group}
      <div class="rounded-xl border border-border bg-card/60 divide-y divide-border">
        {#each group as item}
          <div class="p-3 flex items-center justify-between gap-3">
            <span class="text-foreground/90">{item.label}</span>
            <div class="flex items-center gap-1.5 shrink-0 flex-wrap justify-end">
              {#each item.keys as k, idx}
                {#if idx > 0 && item.joiner}
                  <span class="text-[11px] text-muted-foreground">{item.joiner}</span>
                {/if}
                <kbd class="px-2 py-0.5 rounded bg-muted border font-mono text-[11px] {kbdToneClass(item.tone)}">
                  {k}
                </kbd>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>

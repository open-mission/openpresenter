# Plano de Implementação: Ajustes de Layout e Proporções do Preset Modern Grid no Stage

## 1. Etapas

### Etapa 1: Atualização de `StageView.svelte`
- Ajustar font-size do próximo slide em `modern-grid` para `text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black`.
- Ajustar proporções da coluna lateral para:
  - Sem aviso: Horário (50%) e Cronômetro (50%).
  - Com aviso: Aviso (60%), Horário (20%) e Cronômetro (20%).
- Aumentar font-size da mensagem de aviso para `text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-black`.

### Etapa 2: Atualização de `StagePreviewBox.svelte`
- Refletir as mesmas proporções de 50%/50% e 60%/20%/20% na coluna lateral do preview.
- Equiparar tamanhos de texto do slide atual e próximo.

### Etapa 3: Documentação e Portões de Qualidade
- Atualizar `docs/README.md` e `CHANGELOG.md`.
- Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Executar `bun run check` e `bun run build`.

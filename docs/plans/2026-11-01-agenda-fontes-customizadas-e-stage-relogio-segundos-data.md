# Plano — Fontes da Agenda, Relógio sem Segundos/Data no Palco, Fade de Imagens e Abas Agrupadas

**Data:** 2026-11-01  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-01-agenda-fontes-customizadas-e-stage-relogio-segundos-data.md`](../specs/2026-11-01-agenda-fontes-customizadas-e-stage-relogio-segundos-data.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Atualização de Tipos e Configurações Padrão
- [x] Em [`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts):
  - [x] Adicionar `clock_show_seconds?: boolean;` e `clock_show_date?: boolean;` a `StageDisplaySettings`.
  - [x] Adicionar `image_fade_enabled?: boolean;` e `image_fade_duration_ms?: number;` a `PublicDisplaySettings`.
- [x] Em [`frontend/src/lib/displaySettings.ts`](../../frontend/src/lib/displaySettings.ts):
  - [x] Adicionar `clock_show_seconds: true` e `clock_show_date: true` a `DEFAULT_STAGE_SETTINGS`.
  - [x] Adicionar `image_fade_enabled: true` e `image_fade_duration_ms: 500` a `DEFAULT_PUBLIC_SETTINGS`.
  - [x] Harmonizar `schedule_time_font_size: 20` com `schedule_day_font_size: 20` em `DEFAULT_PUBLIC_SETTINGS` e `DEFAULT_STAGE_SETTINGS`.

### Etapa 2: Implementação no StageView e StagePreviewBox
- [x] Em [`frontend/src/lib/views/StageView.svelte`](../../frontend/src/lib/views/StageView.svelte):
  - [x] Atualizar `updateClock` para formatar `currentTime` respeitando `stageSettings.clock_show_seconds`.
  - [x] Condicionar a renderização de `currentDate` a `stageSettings.clock_show_date !== false` na tela de descanso e no rodapé normal (`clockBox`).
- [x] Em [`frontend/src/lib/components/StagePreviewBox.svelte`](../../frontend/src/lib/components/StagePreviewBox.svelte):
  - [x] Atualizar `stageClock` para formatar horário com ou sem segundos de acordo com `stageSettings.clock_show_seconds`.

### Etapa 3: Implementação de Fade em Imagens no PublicView e PublicPreviewBox
- [x] Em [`frontend/src/lib/views/PublicView.svelte`](../../frontend/src/lib/views/PublicView.svelte):
  - [x] Importar `fade` de `svelte/transition`.
  - [x] Aplicar `transition:fade={{ duration: imageFadeDuration }}` no container da imagem e em volta do chaveamento de conteúdo.
- [x] Em [`frontend/src/lib/components/PublicPreviewBox.svelte`](../../frontend/src/lib/components/PublicPreviewBox.svelte):
  - [x] Adicionar suporte a transição de fade suave no preview de imagens.

### Etapa 4: Interface de Configuração no SettingsModal
- [x] Em [`frontend/src/lib/components/SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - [x] Agrupamento temático na barra lateral de abas (Telas & Saídas, Conteúdo & Mídias, Geral & Sistema).
  - [x] Na aba "Projeção Pública > Agenda":
    - [x] Adicionar campos numéricos (`<input type="number">`) acoplados aos sliders para Dia da Semana, Horário, Detalhes e Título.
    - [x] Equalizar o range máximo de Dia da Semana e Horário (`min="12"` e `max={Math.max(80, value)}`).
  - [x] Na aba "Projeção Pública > Geral":
    - [x] Adicionar controle de transição com fade suave para imagens e retorno ao papel de parede (switch liga/desliga, slider e input numérico de duração em ms).
  - [x] Na aba "Monitor de Palco > Agenda":
    - [x] Adicionar campos numéricos acoplados aos sliders e equalizar range de Dia da Semana e Horário.
  - [x] Na aba "Monitor de Palco > Layout & Tempo":
    - [x] Adicionar opções/switches para "Exibir Segundos no Horário" (`clock_show_seconds`) e "Exibir Data Abaixo do Horário" (`clock_show_date`).
    - [x] Adicionar campos numéricos aos sliders de tamanho do relógio, timer e aviso.

### Etapa 5: Validação dos Portões de Qualidade e Documentação
- [x] Compilar frontend: `cd frontend && bun run build`.
- [x] Validar Rust: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- [x] Atualizar [`docs/README.md`](../README.md), [`CHANGELOG.md`](../../CHANGELOG.md) e [`TODO.md`](../../TODO.md).

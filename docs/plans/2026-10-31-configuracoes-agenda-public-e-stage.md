# Plano — Customizações e Configurações Visuais da Agenda (Public e Stage)

**Data:** 2026-10-31  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-10-31-configuracoes-agenda-public-e-stage.md`](../specs/2026-10-31-configuracoes-agenda-public-e-stage.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Definição de Tipos e Valores Padrão no Frontend
- [x] Em [`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts):
  - [x] Adicionar campos de agenda em `PublicDisplaySettings` (`schedule_box_background`, `schedule_bg_opacity`, `schedule_max_width_percent`, `schedule_padding`, `schedule_title_min_font_size`, `schedule_title_max_font_size`, `schedule_day_font_size`, `schedule_time_font_size`, `schedule_details_font_size`, `schedule_show_badges`, `schedule_show_footer`, `schedule_align_h`).
  - [x] Adicionar campos de agenda em `StageDisplaySettings` (`stage_schedule_title_min_font_size`, `stage_schedule_title_max_font_size`, `stage_schedule_day_font_size`, `stage_schedule_time_font_size`, `stage_schedule_details_font_size`, `stage_schedule_next_min_font_size`, `stage_schedule_next_max_font_size`).
- [x] Em [`frontend/src/lib/displaySettings.ts`](../../frontend/src/lib/displaySettings.ts):
  - [x] Definir os valores padrão em `DEFAULT_PUBLIC_SETTINGS` e `DEFAULT_STAGE_SETTINGS`.

### Etapa 2: Aplicação das Configurações na Tela Pública
- [x] Em [`frontend/src/lib/views/PublicView.svelte`](../../frontend/src/lib/views/PublicView.svelte):
  - [x] Vincular estilo inline do container com largura máxima, cor/opacidade de fundo ou transparência total e padding.
  - [x] Aplicar tamanhos configurados de fontes para dia, hora, detalhes e auto-scale do título.
  - [x] Respeitar flags de exibição de badges e rodapé, e alinhamento horizontal.
- [x] Em [`frontend/src/lib/components/PublicPreviewBox.svelte`](../../frontend/src/lib/components/PublicPreviewBox.svelte):
  - [x] Replicar fielmente o mesmo comportamento com escala proporcional.

### Etapa 3: Aplicação das Configurações no Monitor de Palco
- [x] Em [`frontend/src/lib/views/StageView.svelte`](../../frontend/src/lib/views/StageView.svelte):
  - [x] Aplicar tamanhos de fonte do slide atual (título min/max, dia, horário, detalhes).
  - [x] Aplicar tamanhos de fonte do próximo slide (título min/max).
- [x] Em [`frontend/src/lib/components/StagePreviewBox.svelte`](../../frontend/src/lib/components/StagePreviewBox.svelte):
  - [x] Replicar fielmente as configurações com escala proporcional do preview.

### Etapa 4: Controles Interativos e Prévia no `SettingsModal.svelte`
- [x] Em [`frontend/src/lib/components/SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - [x] Atualizar `publicSection` para `'lyrics' | 'scripture' | 'schedule' | 'general'`.
  - [x] Atualizar `stageSection` para `'lyrics' | 'scripture' | 'schedule' | 'general'`.
  - [x] Atualizar `bgPreviewMode` para `'lyrics' | 'scripture' | 'schedule'` adicionando o botão `[📅 Agenda]`.
  - [x] Construir os cards de configuração da agenda na aba de Projeção Pública (container, opacidade, largura, padding, alinhamento, tamanhos de fontes e toggles de visibilidade).
  - [x] Construir os cards de configuração da agenda na aba de Monitor de Palco (fontes do slide atual e próximo slide).
  - [x] Implementar a renderização da prévia da agenda no mockup 16:9.

### Etapa 5: Validação e Portões de Qualidade
- [x] `cd frontend && bun run build`
- [x] `cargo fmt --all`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.

# Plano — Tipo de Mídia: Agenda (Schedule)

**Data:** 2026-10-31  
**Status:** Em execução  
**Spec correspondente:** [`docs/specs/2026-10-31-tipo-de-midia-agenda-schedule.md`](../specs/2026-10-31-tipo-de-midia-agenda-schedule.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Backend Rust (`op-core`)
- Adicionar `Schedule` ao enum `MediaKind` em [`crates/op-core/src/lib.rs`](../../crates/op-core/src/lib.rs).
- Implementar `label_pt_br()` retornando `"Agenda"`.
- Adicionar teste unitário de serialização e deserialização para `MediaKind::Schedule`.
- Executar `cargo test -p op-core`.

### Etapa 2: Definições de Tipos no Frontend
- Atualizar `MediaKind` em [`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts) incluindo `'schedule'`.
- Definir interface `ScheduleEvent` para facilitar o gerenciamento no formulário do modal:
  ```typescript
  export interface ScheduleEvent {
    id: string;
    dayOrDate: string;
    time: string;
    title: string;
    details: string;
  }
  ```

### Etapa 3: Componente `ScheduleEditorModal.svelte`
- Criar [`frontend/src/lib/components/ScheduleEditorModal.svelte`](../../frontend/src/lib/components/ScheduleEditorModal.svelte).
- Implementar campos: Título Geral da Agenda, Lista de Eventos (Dia/Data, Horário, Nome, Detalhes).
- Adicionar botões de atalho rápido de dias da semana (`+ Domingo`, `+ Quarta`, `+ Sábado`, `+ Segunda`, `+ Terça`, `+ Quinta`, `+ Sexta`, `+ Evento Futuro`).
- Ações de mover para cima/baixo, excluir evento e validação de campos.
- Conversão de/para formato `MediaItem` com `kind: 'schedule'`.

### Etapa 4: Integração com o Painel do Operador (`AdminView.svelte`)
- Adicionar item "Agenda / Programação..." no dropdown `+` da setlist (sidebar) com ícone `CalendarDays`.
- Adicionar ação "Editar Agenda..." no menu de contexto de itens da setlist quando `item.kind === 'schedule'`.
- Vincular estados `isScheduleEditorOpen`, `scheduleItemToEdit` e handler `handleSaveSchedule`.

### Etapa 5: Renderização nas Telas de Saída
- Em [`PublicView.svelte`](../../frontend/src/lib/views/PublicView.svelte):
  - Detectar `activeMedia?.kind === 'schedule'`.
  - Extrair dia (label), horário, título e detalhes de `activeSlide.content`.
  - Renderizar card estilizado em tela cheia sobre o papel de parede ativo com `autoScale` no título.
- Em [`StageView.svelte`](../../frontend/src/lib/views/StageView.svelte) e [`StagePreviewBox.svelte`](../../frontend/src/lib/components/StagePreviewBox.svelte):
  - Renderizar de forma semântica os eventos da agenda para Slide Atual e A Seguir.

### Etapa 6: Validação, Testes e Documentação
- Executar compilação do frontend: `cd frontend && bun run build`.
- Executar portões de qualidade: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Atualizar `CHANGELOG.md`, `TODO.md` e registrar commit no GitFlow.

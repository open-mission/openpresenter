# Plano de Execução — Refatoração do Módulo 8 (Cronogramas & Agendas de Culto)

## Fases de Execução

### Fase 1: Fundação de Tipos, Defaults e Controlador
- Criar `frontend/src/lib/components/schedule-editor/types.ts` com interfaces `ScheduleEvent`, callbacks e payloads.
- Criar `frontend/src/lib/components/schedule-editor/defaults.ts` com a lista inicial de eventos e presets de dias.
- Criar `frontend/src/lib/components/schedule-editor/controller.svelte.ts`:
  - Estados: `open`, `title`, `artistOrRef`, `events`, `selectedEventIndex`, `error`, `isMaximized`, `showDiscardConfirm`.
  - Métodos: `syncOpen`, `addEvent`, `removeEvent`, `moveEvent`, `selectEvent`, `validate`, `toMediaItem`, `requestClose`, `confirmDiscard`, `cancelDiscard`.

### Fase 2: Componentes Auxiliares e Submódulos
- Criar `frontend/src/lib/components/schedule-editor/ScheduleEditorHeader.svelte`:
  - Ícone de calendário, título dinâmico (Novo vs Editar), badge de contagem de eventos, atalho `⌘Enter`, botões Maximizar e Fechar.
- Criar `frontend/src/lib/components/schedule-editor/ScheduleEditorMetadata.svelte`:
  - Campos de Título da Programação e Categoria/Tag com tokens semânticos e labels acessíveis.
- Criar `frontend/src/lib/components/schedule-editor/ScheduleEditorPresets.svelte`:
  - Botões de atalho rápido (+ Domingo, + Quarta, etc.) com alvos táteis confortáveis `>= 28px`.
- Criar `frontend/src/lib/components/schedule-editor/ScheduleEditorEventCard.svelte`:
  - Card de evento acessível (`role="group"` com `aria-labelledby`), badges numerados, botões de reordenação (ArrowUp / ArrowDown) e exclusão com `aria-label` e alvos `>= 28×28px`, campos de Dia/Data, Horário, Nome e Detalhes com labels `>= 11px`.
- Criar `frontend/src/lib/components/schedule-editor/ScheduleEditorEventsList.svelte`:
  - Container da lista, cabeçalho da seção com botão "+ Novo Evento" e renderização dos cards com animação suave.
- Criar `frontend/src/lib/components/schedule-editor/ScheduleEditorPreview.svelte`:
  - Card sticky de prévia:
    - Slide 16:9 da projeção pública com simulação de proporção e badges contrastantes.
    - Bloco de simulação do monitor de retorno de palco (`/stage`) com "SLIDE ATUAL" e "A SEGUIR", com tipografia no piso `>= 11px`.
- Criar `frontend/src/lib/components/schedule-editor/ScheduleEditorConfirmDiscard.svelte`:
  - Modal acessível de confirmação de descarte de alterações com `role="alertdialog"`, trap de foco via `focusTrap.ts` e suporte a `Escape`.

### Fase 3: Refatoração do Orquestrador `ScheduleEditorModal.svelte`
- Substituir o conteúdo monolítico pelo novo orquestrador conectando o controlador e os submódulos da pasta `schedule-editor/`.
- Garantir `role="dialog"`, `aria-modal="true"`, `aria-labelledby`, `tabindex="-1"`, `inert` quando o diálogo de confirmação estiver aberto, e armadilha de foco.
- Reduzir o orquestrador para menos de 180 linhas.

### Fase 4: Verificação e Portões de Qualidade
- `npm run check` (0 erros e 0 warnings).
- `npm run build` (sucesso).
- `npx impeccable detect frontend/src --json` (0 apontamentos).
- `cargo test -p op-db -p op-admin` (todos os testes passando).
- Auditoria e pós-fix do Impeccable Critique (`.impeccable/critique/`).
- Atualização da documentação (`TODO.md`, `CHANGELOG.md`, `docs/README.md`).

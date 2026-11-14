# Plano de Execução — Refatoração do Módulo 14 (Itens Rápidos, Divisores & Avisos)

## Fases de Execução

### Fase 1: Fundação de Tipos, Defaults e Controlador
- Criar `frontend/src/lib/components/quick-item/types.ts`:
  - `QuickItemMode`: `'header' | 'announcement'`.
  - `QuickItemCallbacks`: `onclose: () => void; onsave: (item: MediaItem) => void;`.
  - `QuickItemModalProps`: `open: boolean; mode: QuickItemMode; initialItem?: MediaItem | null; onclose: () => void; onsave: (item: MediaItem) => void;`.
- Criar `frontend/src/lib/components/quick-item/defaults.ts`:
  - Constantes de configuração e textos contextuais por modo (`HEADER_MODE_CONFIG`, `ANNOUNCEMENT_MODE_CONFIG`).
- Criar `frontend/src/lib/components/quick-item/controller.svelte.ts`:
  - Runes Svelte 5 (`$state`, `$derived`).
  - Estados: `open`, `mode`, `isMaximized`, `title`, `content`, `initialItem`, `isDiscardOpen`, `dialogEl`, `#previouslyFocused`.
  - Derivados: `slideCount`, `isDirty`, `isEditing`, `modeConfig`.
  - Métodos: `syncOpen`, `requestClose`, `confirmDiscard`, `cancelDiscard`, `buildMediaItem`, `save`, `handleKeydown`, `close`, `restoreFocus`.

### Fase 2: Componentes Auxiliares e Submódulos
- Criar `frontend/src/lib/components/quick-item/QuickItemHeader.svelte`:
  - Ícone dinâmico (`Bookmark` vs `FileText`), título dinâmico, descrição contextual, botões Maximizar e Fechar com acessibilidade e foco visível.
- Criar `frontend/src/lib/components/quick-item/QuickItemForm.svelte`:
  - Campo de título/nome de seção com label associada e placeholder contextual.
  - Textarea de conteúdo de slides para avisos com contador dinâmico de slides e dica textual acessível `>= 11px`.
- Criar `frontend/src/lib/components/quick-item/QuickItemFooter.svelte`:
  - Botão Cancelar (dispara `requestClose`).
  - Botão Salvar com verificação de título não vazio e indicação de atalho `⌘Enter` / `Ctrl+Enter`.
- Criar `frontend/src/lib/components/quick-item/QuickItemConfirmDiscard.svelte`:
  - Modal acessível com `role="alertdialog"`, `aria-modal="true"`, foco preso e botões "Descartar Alterações" e "Continuar Editando", eliminando 100% o `confirm()` nativo.

### Fase 3: Refatoração do Orquestrador `QuickItemModal.svelte` e Ajuste no `AdminView.svelte`
- Substituir o conteúdo monolítico de `QuickItemModal.svelte` pelo novo orquestrador conectando o controlador e submódulos de `quick-item/`.
- Garantir `role="dialog"`, `aria-modal="true"`, `aria-labelledby="quick-item-title"`, `aria-describedby="quick-item-desc"`, `tabindex="-1"`, trap de foco com `focusTrap.ts`, Escape hierárquico e atalhos `⌘Enter` / `Ctrl+Enter`.
- No `AdminView.svelte`, proteger `if (isQuickItemModalOpen) return;` no handler global de `Escape` para delegar o fechamento ao modal.
- Reduzir o orquestrador para menos de 115 linhas.

### Fase 4: Verificação e Portões de Qualidade
- `npm run check` (0 erros e 0 warnings).
- `npm run build` (sucesso limpo).
- `npx impeccable detect frontend/src --json` (0 apontamentos).
- `cargo test -p op-db -p op-admin` (78 testes passando).
- Auditoria e pós-fix do Impeccable Critique (`.impeccable/critique/`).
- Atualização da documentação (`CHANGELOG.md`, `docs/README.md`).

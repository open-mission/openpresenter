# Plano — Aba "Outros" no Painel de Mídias (Agenda, Textos e Avisos)

**Data:** 2026-11-01  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-01-aba-outros-painel-de-midias.md`](../specs/2026-11-01-aba-outros-painel-de-midias.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Módulo de Catálogo e Persistência de Itens Customizados
- [x] Criar [`frontend/src/lib/customItems.ts`](../../frontend/src/lib/customItems.ts):
  - [x] Definir `customItemsStore` (store de `MediaItem[]`).
  - [x] Funções `loadCustomItems()`, `saveCustomItems()`, `saveOrUpdateCustomItem(item)`, `deleteCustomItem(id)` e `importItemsFromSetlist(items)`.
  - [x] Seeding inicial com itens de exemplo (Programação Semanal, Boas-Vindas, Avisos Gerais).
  - [x] Escuta reativa a atualizações via WebSocket.

### Etapa 2: Callbacks e Integração em `AdminView.svelte`
- [x] Em [`frontend/src/lib/views/AdminView.svelte`](../../frontend/src/lib/views/AdminView.svelte):
  - [x] Integrar salvamento automático de itens no catálogo de `customItemsStore` ao salvar agenda, texto ou aviso.
  - [x] Adicionar handlers para o `BottomPanel`:
    - `handleAddItemToSetlist`: adiciona item customizado à setlist via `/api/setlist/add`.
    - `handlePresentCustomItem`: projeta o item/slide imediatamente usando `quickPresentMedia`.
    - `handleEditCustomItem`: abre o modal correspondente (`ScheduleEditorModal`, `SongEditorModal` ou `QuickItemModal`).
    - `handleOpenCreateSchedule`, `handleOpenCreateText`, `handleOpenCreateAnnouncement`.

### Etapa 3: Construção da Aba "Outros" em `BottomPanel.svelte`
- [x] Em [`frontend/src/lib/components/BottomPanel.svelte`](../../frontend/src/lib/components/BottomPanel.svelte):
  - [x] Adicionar o botão "Outros" na barra de abas (`role="tab"`, `id="tab-others"`).
  - [x] Expandir o estado `activeTab` para incluir `'others'`.
  - [x] Construir a barra de ações: campo de busca, chips de categoria (`Todos`, `📅 Agenda`, `📝 Textos`, `📢 Avisos`, `🔖 Seções`), botão recarregar, botão de importar da setlist e botões de criação rápida.
  - [x] Construir a lista de cards à esquerda com tags de tipo, slides, resumo e ações rápidas.
  - [x] Construir a visualização detalhada à direita com visualização 16:9 dos slides para agenda e texto.

### Etapa 4: Validação e Portões de Qualidade
- [x] `cd frontend && bun run build`
- [x] `cargo fmt --all`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.

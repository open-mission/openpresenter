# Plano — Inserção no Cursor e Menus de Contexto no Editor de Músicas e Aba Outros

**Data:** 2026-11-01  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-01-context-menu-editor-musicas-e-aba-outros.md`](../specs/2026-11-01-context-menu-editor-musicas-e-aba-outros.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Correção de Inserção no Cursor em `SongEditorModal.svelte`
- [x] Em [`frontend/src/lib/components/SongEditorModal.svelte`](../../frontend/src/lib/components/SongEditorModal.svelte):
  - [x] Refatorar `insertSectionTag(tag)` para usar `selectionStart` e `selectionEnd` de `textareaEl`.
  - [x] Inserir espaçamento e prefixo `## ` adequado e devolver o foco com o cursor posicionado após a tag.

### Etapa 2: Menu de Contexto nos Slides de Prévia do `SongEditorModal.svelte`
- [x] Em [`frontend/src/lib/components/SongEditorModal.svelte`](../../frontend/src/lib/components/SongEditorModal.svelte):
  - [x] Importar [`ContextMenu.svelte`](../../frontend/src/lib/components/ContextMenu.svelte) e `type MenuItem`.
  - [x] Remover a barra estática de botões de atalho de seção sobre o textarea, adicionando botão dropdown compacto `[+ Seção ▾]` na barra de ferramentas.
  - [x] Implementar handler de clique direito nos cards de slide da prévia (`handleSlideContextMenu(e, slide, index)`).
  - [x] Criar função para alterar a seção do slide no texto (`setSlideSection(slide, newLabel)`) e ações de duplicar/excluir slide.

### Etapa 3: Menu de Contexto na Aba "Outros" em `BottomPanel.svelte`
- [x] Em [`frontend/src/lib/components/BottomPanel.svelte`](../../frontend/src/lib/components/BottomPanel.svelte):
  - [x] Importar [`ContextMenu.svelte`](../../frontend/src/lib/components/ContextMenu.svelte) e `type MenuItem`.
  - [x] Implementar `handleOtherItemContextMenu(e, item)` nos cards da lista.
  - [x] Implementar `handleOtherSlideContextMenu(e, slide, index)` nos slides da prévia 16:9.
  - [x] Implementar `handleOtherEmptyContextMenu(e)` na área vazia da lista.
  - [x] Adicionar ação de duplicar item customizado (`handleDuplicateCustomItem`).

### Etapa 4: Validação e Portões de Qualidade
- [x] `cd frontend && bun run build`
- [x] `cargo fmt --all`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.

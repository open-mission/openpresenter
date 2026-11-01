# Plano — Context Menu e Exclusão de Papéis de Parede Locais no Painel de Fundos

**Data:** 2026-11-01  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-01-context-menu-e-exclusao-de-fundos.md`](../specs/2026-11-01-context-menu-e-exclusao-de-fundos.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Endpoint de Exclusão no Backend (`op-admin`)
- [x] Em [`crates/op-admin/src/routes.rs`](../../crates/op-admin/src/routes.rs):
  - [x] Registrar rota `DELETE /api/media/wallpapers/{filename}` chamando `delete_wallpaper`.
  - [x] Implementar `delete_wallpaper` com sanitização, proteção de defaults e remoção assíncrona.
  - [x] Adicionar fallback de reset de apresentação caso o papel de parede excluído esteja em exibição ativa.
- [x] Em [`crates/op-admin/src/main.rs`](../../crates/op-admin/src/main.rs):
  - [x] Adicionar teste unitário validando a exclusão de wallpaper e a proteção dos defaults.

### Etapa 2: Integração e Menus de Contexto no Frontend (`BottomPanel.svelte`)
- [x] Em [`frontend/src/lib/components/BottomPanel.svelte`](../../frontend/src/lib/components/BottomPanel.svelte):
  - [x] Implementar `handleDeleteWallpaper(wall: WallpaperItem)` com confirmação e chamada `DELETE`.
  - [x] Adicionar botão de exclusão rápida (ícone de lixeira) nos cartões de papéis de parede locais não padrão.
  - [x] Implementar `handleLocalWallpaperContextMenu(e, wall)` nos cartões de papéis de parede locais.
  - [x] Implementar `handlePresetContextMenu(e, preset)` nos cartões de presets CSS.
  - [x] Implementar `handleBackgroundsEmptyContextMenu(e)` na grade/área vazia da aba de fundos.
  - [x] Interceptar `oncontextmenu` com `e.preventDefault()`, evitando o menu padrão do navegador.

### Etapa 3: Validação e Portões de Qualidade
- [x] `cd frontend && bun run build`
- [x] `cargo fmt --all`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.

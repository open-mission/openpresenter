# Plano de Implementação: Limpeza de Itens da Setlist Ativa

## 1. Etapas

### Etapa 1: Backend em Rust
- Implementar `clear_active_setlist_items(&self)` em `crates/op-admin/src/state.rs`.
- Registrar rota `POST /api/setlists/items/clear` e função de handler em `crates/op-admin/src/routes.rs`.
- Criar teste de integração em `crates/op-admin/src/main.rs` validando o esvaziamento da setlist ativa e a persistência no banco.

### Etapa 2: Funções do Frontend
- Adicionar `clearSetlistItems()` em `frontend/src/lib/ws.ts`.

### Etapa 3: Interface no Painel de Setlist
- Em `frontend/src/lib/views/AdminView.svelte`:
  - Adicionar handler `handleClearSetlist()` com diálogo de confirmação.
  - Adicionar botão de limpeza com ícone intuitivo (`Eraser` ou `Trash2`) no cabeçalho da sidebar da setlist.
  - Adicionar opção "Limpar Itens da Setlist" no dropdown de opções de setlist.

### Etapa 4: Documentação e Portões de Qualidade
- Atualizar `docs/README.md` e `CHANGELOG.md`.
- Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Executar `cd frontend && bun run check && bun run build`.

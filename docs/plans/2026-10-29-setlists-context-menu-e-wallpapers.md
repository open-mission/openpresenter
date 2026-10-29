# Plano de Implementação — Múltiplas Setlists, Context Menu, Papel de Parede e Armazenamento de Mídias

Este plano orienta a implementação das funcionalidades solicitadas, divididas em etapas incrementais e testáveis.

## Etapas

### Etapa 1: Modelos e Migração do Banco de Dados (`op-core` e `op-db`)
- **Objetivo:** Adicionar tipos e persistência para Setlist, divisores de seção e wallpapers.
- **Tarefas:**
  1. Adicionar `MediaKind::Header` em `crates/op-core/src/lib.rs`.
  2. Adicionar estrutura `Setlist` e `SetlistSummary` em `crates/op-core/src/lib.rs`.
  3. Adicionar campos opcionais `active_setlist_id`, `wallpaper_path` e `wallpaper_mode` em `PresentationState`.
  4. Adicionar mensagens `WsMessage::SetlistUpdated`, `WsMessage::SelectSetlist`, etc.
  5. Criar migração `crates/op-db/migrations/0004_setlists.sql` criando a tabela `setlists`.
  6. Implementar métodos em `crates/op-db/src/lib.rs`: `save_setlist`, `get_setlist`, `list_setlists`, `delete_setlist`, `set_active_setlist`, `get_active_setlist`.
  7. Adicionar testes unitários para o repositório de setlists em `op-db`.

### Etapa 2: Armazenamento de Mídias no Sistema e Rotas de API (`op-admin`)
- **Objetivo:** Gerenciar pasta de mídias no sistema operacional e expor endpoints REST e WebSocket.
- **Tarefas:**
  1. Criar e gerenciar a subpasta `<data_dir>/media/wallpapers/` na inicialização da sessão, populando com wallpapers padrão se vazia.
  2. Implementar rota `GET /api/media/files/*subpath` para entrega dos arquivos locais com mime type apropriado.
  3. Implementar rotas `GET /api/media/wallpapers` e `POST /api/media/upload`.
  4. Implementar rotas REST de Setlists: `GET /api/setlists`, `POST /api/setlists`, `GET /api/setlists/{id}`, `DELETE /api/setlists/{id}`, `POST /api/setlists/{id}/active`, `POST /api/setlists/items/remove`, `POST /api/setlists/items/reorder`.
  5. Sincronizar eventos de setlist via canal WebSocket.

### Etapa 3: Menus de Contexto, Gestão de Setlists e Logo no Frontend (`frontend`)
- **Objetivo:** Construir a interface do usuário fluida, responsiva e em conformidade com o pedido.
- **Tarefas:**
  1. Corrigir o logo no header do `AdminView.svelte` (eliminar ícone duplicado e redimensionar para altura legível).
  2. Implementar tema Neutro (Neutral) em `settings.ts` e seletor de paletas neutras em `SettingsModal.svelte`.
  3. Criar componente `SetlistModal.svelte` para criar/editar setlists (título, descrição, papel de parede e modo de ajuste).
  4. Criar componente `ContextMenu.svelte` reutilizável para o clique direito nos itens da setlist e em área vazia.
  5. Atualizar o cabeçalho da Setlist com seletor de setlists e menu suspenso no botão `+` (Música, Seção, Texto Livre, Versículo).
  6. Suportar ordenação (Fixar no topo), Renomear, Editar e Remoção nos itens da setlist.
  7. Atualizar `PublicView.svelte` para exibir o papel de parede ativo do setlist em modo repouso e `cleartext`, respeitando o modo de ajuste configurado.

### Etapa 4: Validação, Portões de Qualidade e Documentação
- **Tarefas:**
  1. Executar `cargo fmt --all`.
  2. Executar `cargo clippy --workspace --all-targets -- -D warnings`.
  3. Executar `cargo test --workspace`.
  4. Executar `cd frontend && bun run check && bun run build`.
  5. Atualizar `CHANGELOG.md` e `docs/architecture/overview.md`.
  6. Realizar commits atômicos e descritivos.

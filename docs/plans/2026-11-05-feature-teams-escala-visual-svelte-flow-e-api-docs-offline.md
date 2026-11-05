# Plano — Funcionalidade Teams (Equipe, Escala Visual com Svelte Flow) e API Docs 100% Offline

**Data:** 2026-11-05  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-05-feature-teams-escala-visual-svelte-flow-e-api-docs-offline.md`](../specs/2026-11-05-feature-teams-escala-visual-svelte-flow-e-api-docs-offline.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Camada de Domínio (`crates/op-core`)
- [x] Criar `crates/op-core/src/teams.rs` com structs:
  - [x] `TeamDepartment`, `UpsertTeamDepartment`
  - [x] `TeamMember`, `UpsertTeamMember`
  - [x] `ServiceRosterItem`, `UpsertServiceRosterItem`
  - [x] `ServiceRoster`, `ServiceRosterSummary`, `UpsertServiceRoster`
- [x] Em `crates/op-core/src/lib.rs`:
  - [x] Re-exportar módulo `teams`.
  - [x] Adicionar variante `WsMessage::TeamUpdated { entity: String, action: String }`.
- [x] Testes unitários de serialização JSON em `crates/op-core`.

### Etapa 2: Camada de Persistência (`crates/op-db`)
- [x] Criar migração `crates/op-db/migrations/0010_teams.sql`:
  - [x] Tabelas `team_departments`, `team_members`, `team_member_departments`, `service_rosters`, `service_roster_items`.
  - [x] Índices de performance.
  - [x] Inserir os 5 departamentos padrão (Projeção, Fotografia, Stories & Redes, Transmissão, Áudio & Som; sem iluminação).
- [x] Atualizar `crates/op-db/src/lib.rs` com métodos de CRUD:
  - [x] Departamentos: `list_team_departments`, `get_team_department`, `save_team_department`, `delete_team_department`.
  - [x] Membros: `list_team_members`, `get_team_member`, `save_team_member`, `delete_team_member`.
  - [x] Escalas: `list_service_rosters`, `get_service_roster`, `get_active_service_roster`, `save_service_roster`, `set_active_service_roster`, `delete_service_roster`, `assign_service_roster_item`, `remove_service_roster_item`, `update_service_roster_item_status`.
- [x] Implementar testes unitários em `crates/op-db` cobrindo migrações e operações de teams.

### Etapa 3: API Docs 100% Offline e Rotas REST (`crates/op-admin`)
- [x] Baixar e organizar os assets standalone offline em `crates/op-admin/assets/api-docs/`:
  - [x] `scalar.standalone.js`
  - [x] `swagger-ui-bundle.js`, `swagger-ui-standalone-preset.js`, `swagger-ui.css`
- [x] Criar módulo `crates/op-admin/src/api_docs.rs`:
  - [x] Especificação OpenAPI 3.0.3 completa em JSON para `/api/openapi.json`.
  - [x] Handlers offline para `/docs`, `/scalar`, `/swagger` e `/docs/assets/*` via `rust-embed`.
- [x] Em `crates/op-admin/src/routes.rs`:
  - [x] Registrar rotas `/api/teams/*` e conectar aos métodos de banco.
  - [x] Emitir eventos WebSocket e registrar em `ActionLog`.
  - [x] Registrar rotas de documentação da API.
- [x] Testes de integração em `crates/op-admin`.

### Etapa 4: Frontend — Tipos, API e Svelte Flow
- [x] Instalar `@xyflow/svelte` no frontend: `npm install @xyflow/svelte`.
- [x] Atualizar `frontend/src/lib/types.ts` com tipos de Teams.
- [x] Criar `frontend/src/lib/teams.ts` com client de API e gerador de mensagem formatada para WhatsApp.
- [x] Criar nós customizados do Svelte Flow:
  - [x] `ServiceNode.svelte`
  - [x] `DepartmentNode.svelte`
  - [x] `MemberNode.svelte`
- [x] Criar `RosterFlowCanvas.svelte` integrando Svelte Flow com minimap, controls e background.
- [x] Criar `TeamsModal.svelte` com abas (Escala do Culto em Modo Híbrido, Membros e Departamentos).
- [x] Integrar botão no header de `AdminView.svelte`.
- [x] Integrar no `SettingsModal.svelte`.
- [x] Integrar card da escala ativa em `RemoteView.svelte`.

### Etapa 5: Validação dos Portões de Qualidade e Documentação
- [x] Executar `cargo fmt --all`.
- [x] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Executar `cargo test --workspace`.
- [x] Executar `cd frontend && npm run build`.
- [x] Atualizar `docs/README.md`, `CHANGELOG.md` e `TODO.md`.

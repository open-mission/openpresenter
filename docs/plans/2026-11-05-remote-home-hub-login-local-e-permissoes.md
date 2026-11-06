# Plano — Home do Remote (Portal do Voluntário), Login Local e Controle de Acesso

**Data:** 2026-11-05  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-05-remote-home-hub-login-local-e-permissoes.md`](../specs/2026-11-05-remote-home-hub-login-local-e-permissoes.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Camada de Domínio (`crates/op-core`)
- [x] Atualizar `TeamMember` e `UpsertTeamMember` em `crates/op-core/src/teams.rs`:
  - [x] Adicionar `can_control_remote: bool` (default `true`) e `pin: Option<String>`.
  - [x] Atualizar testes unitários de serialização em `crates/op-core`.

### Etapa 2: Camada de Persistência (`crates/op-db`)
- [x] Criar migração `crates/op-db/migrations/0012_team_member_remote_access.sql`.
- [x] Atualizar `crates/op-db/src/lib.rs`:
  - [x] Registrar migração 12 no vetor `migrations`.
  - [x] Atualizar consultas e mapeamentos em `list_team_members`, `get_team_member` e `save_team_member`.
  - [x] Atualizar asserções de `schema_version` (de 11 para 12) nos testes.

### Etapa 3: Camada de API & Sessão (`crates/op-admin`)
- [x] Atualizar asserções de `schema_version` em `crates/op-admin/src/session.rs` (de 11 para 12).
- [x] Atualizar schemas OpenAPI em `crates/op-admin/src/api_docs.rs` para incluir `can_control_remote` e `pin`.
- [x] Atualizar testes unitários em `crates/op-admin/src/main.rs`.

### Etapa 4: Frontend Types e Admin Member Form
- [x] Atualizar `frontend/src/lib/types.ts` (`can_control_remote` e `pin` em `TeamMember` e `UpsertTeamMember`).
- [x] Atualizar `TeamsModal.svelte` para exibir os campos de permissão de controle remoto e PIN no cadastro/edição de voluntários.

### Etapa 5: Frontend Remote — Sistema de Login Local & Home Hub
- [x] Criar gerenciamento de sessão local no remote (`remoteAuthStore` / `localStorage`).
- [x] Implementar tela de seleção de perfil / login com PIN em `RemoteView.svelte` (ou componente dedicado).
- [x] Implementar a aba `Início` (Home Hub) no `RemoteView.svelte`:
  - [x] Hero card com saudação, avatar e cargo do voluntário.
  - [x] Card de escala pessoal no culto ativo com botões de presença (Confirmado, Pendente, Não Comparece).
  - [x] Card interativo "Controle Remoto de Mídia" (com verificação de `can_control_remote` e desbloqueio por PIN).
  - [x] Card "Equipe & Escala Completa".
  - [x] Card "Chat da Equipe de Mídia" (teaser com badge "Em Breve").
- [x] Atualizar navegação inferior de 5 abas (`Início`, `Setlist`, `Músicas`, `Bíblia`, `Equipe`).

### Etapa 6: Validação de Qualidade e Documentação
- [x] Executar `cargo fmt --all`.
- [x] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Executar `cargo test --workspace`.
- [x] Executar `cd frontend && npm run build`.
- [x] Atualizar `docs/README.md`, `CHANGELOG.md` e `TODO.md`.

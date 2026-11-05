# Spec — Funcionalidade Teams (Equipe de Mídia, Escala Visual com Svelte Flow) e API Docs 100% Offline

## 1. Intenção

Introduzir no OpenPresenter o gerenciamento completo da equipe de mídia da igreja e a montagem visual da escala de voluntários por culto (com suporte a Svelte Flow e modo grade), além de disponibilizar a documentação OpenAPI 3.0 completa da aplicação através do Scalar e Swagger UI embutidos 100% offline, preparando o sistema para o futuro chat e integrações externas (estilo Planning Center ou Voluts).

---

## 2. Contexto e Problema

1. **Gestão de Equipe Fragmentada:** O operador e a liderança de mídia das igrejas organizavam escalas e funções técnicas (projeção, fotografia, stories, transmissão e som) em aplicativos externos ou planilhas de WhatsApp. Faltava uma forma integrada no OpenPresenter para cadastrar os voluntários, associar suas áreas de atuação e definir a escala do culto ativo.
2. **Construção Visual da Escala:** Listas estáticas em texto tornam a montagem de escalas monótona e propensa a falhas de cobertura. Uma experiência baseada em grafo visual (usando o Svelte Flow — `@xyflow/svelte`) permite visualizar o culto, os departamentos e as pessoas conectadas, sabendo instantaneamente se algum posto está vago, quem é o titular e quem é o auxiliar.
3. **Integração com Sistemas Externos & Restrição Offline:** O OpenPresenter é frequentemente operado em máquinas de igrejas com conexões de rede fechadas ou sem internet estável. Para que sistemas externos (estilo Planning Center ou Voluts) possam se integrar facilmente à API do OpenPresenter, é fundamental fornecer documentação interativa (Scalar e Swagger UI) com schemas e exemplos, porém **100% offline**, sem nenhuma dependência de CDNs externas.

---

## 3. Escopo e Solução

### 3.1 Camada de Domínio (`crates/op-core`)
- Módulo `teams.rs` com structs serializáveis:
  - `TeamDepartment` e `UpsertTeamDepartment`: identificador, nome, descrição, cor, ícone, ordem de exibição e timestamps.
  - `TeamMember` e `UpsertTeamMember`: identificador UUID, nome, cargo na equipe (`role`: `leader`, `vice_leader`, `member`), telefone opcional, e-mail, cor de identificação, observações, status ativo e lista de `department_ids`.
  - `ServiceRosterItem` e `UpsertServiceRosterItem`: vínculo do voluntário a um departamento no culto, papel (`lead` / Titular, `assistant` / Auxiliar, `backup` / Apoio), status de confirmação (`confirmed`, `pending`, `declined`) e observações.
  - `ServiceRoster`, `ServiceRosterSummary` e `UpsertServiceRoster`: título do culto, data, horário, escala ativa (`is_active`) e lista de itens escalados.
- Mensagem de evento WebSocket `WsMessage::TeamUpdated { entity: String, action: String }`.

### 3.2 Camada de Persistência (`crates/op-db`)
- Nova migração aditiva numerada `0010_teams.sql`:
  - `team_departments`: departamentos da equipe de mídia. Seed inicial com 5 departamentos (Projeção, Fotografia, Stories & Redes, Transmissão, Áudio & Som; sem iluminação).
  - `team_members`: voluntários cadastrados.
  - `team_member_departments`: tabela associativa N:N (um membro pode atuar em múltiplos departamentos).
  - `service_rosters`: escalas de culto/eventos.
  - `service_roster_items`: membros escalados por departamento com papel e status de confirmação.
- Nova migração aditiva numerada `0011_team_member_role.sql`:
  - Adição da coluna `role TEXT NOT NULL DEFAULT 'member'` na tabela `team_members`.
- Métodos CRUD completos no `Database` em `crates/op-db/src/lib.rs`.

### 3.3 API REST e Documentação Offline (`crates/op-admin`)
- Módulo `crates/op-admin/src/api_docs.rs`:
  - Endpoint `GET /api/openapi.json`: especificação OpenAPI 3.0.3 completa e validada com todas as rotas do OpenPresenter.
  - Endpoints `GET /docs` e `GET /scalar`: documentação moderna do Scalar 100% offline.
  - Endpoint `GET /swagger`: documentação clássica do Swagger UI 100% offline.
  - Endpoint `GET /docs/assets/*`: entrega dos arquivos estáticos locais (`scalar.standalone.js`, `swagger-ui-bundle.js`, `swagger-ui.css`, `swagger-ui-standalone-preset.js`) embutidos no binário Rust via `rust-embed` (`EmbeddedApiDocs`). Zero requisições a CDNs externas.
- Rotas `/api/teams/*` em `crates/op-admin/src/routes.rs`:
  - Gestão de departamentos (`/api/teams/departments`).
  - Gestão de membros (`/api/teams/members`).
  - Gestão de escalas de culto (`/api/teams/rosters`, ativação, inclusão e remoção de itens).
  - Notificação de eventos via WebSocket broadcast.

### 3.4 Interface do Usuário (Frontend Svelte 5)
- Instalação e integração do `@xyflow/svelte` (Svelte Flow).
- Modal `TeamsModal.svelte` com 3 abas:
  1. **Escala do Culto:**
     - Alternador no topo: **[⚡ Visual Flow]** (Svelte Flow) e **[📋 Grade / Cards]**.
     - No Visual Flow: nós customizados (`ServiceNode`, `DepartmentNode`, `MemberNode`), arestas suaves, minimap, controles de zoom, fit view e background com dots.
     - Suporte a múltiplos membros por departamento (Titular e Auxiliar).
     - Suporte a um membro estar em mais de um departamento no mesmo culto.
     - Badge interativo de confirmação com 3 estados: 🟢 Confirmado, 🟡 Pendente, 🔴 Não Comparece.
     - Botão direto de WhatsApp para cada voluntário.
     - Botão "Copiar Escala Formatada para WhatsApp" pronta para colar no grupo da igreja.
     - Botão para definir a escala como ativa no culto corrente.
  2. **Membros da Equipe:**
     - Busca em tempo real e filtro por departamento.
     - Cards com avatar colorido, telefone com link direto de chamada e WhatsApp, badges de departamentos e ações de editar/excluir com dirty guard.
  3. **Departamentos:**
     - Listagem das áreas com total de voluntários em cada uma, seletor de cores e ícones Lucide.
- Header de `AdminView.svelte`: botão `[👥 Equipe]` com indicador de escala ativa.
- Command Palette (`⌘K`): comandos para abrir a equipe e a documentação da API.
- Configurações (`SettingsModal.svelte`): atalho para abrir a documentação da API no navegador.
- Remote Mobile (`RemoteView.svelte`): card visualizando a escala ativa do culto.

---

## 4. Critérios de Aceite

1. A migração SQLite `0010_teams.sql` executa com sucesso, cria as tabelas relacionais com chaves estrangeiras e popula os 5 departamentos padrão (sem iluminação).
2. É possível cadastrar, editar e excluir departamentos personalizados com cor e ícone.
3. É possível cadastrar, editar e excluir membros da equipe com nome, telefone opcional (com link direto para WhatsApp) e múltiplos departamentos.
4. É possível criar escalas de culto, alocar titulares e auxiliares nos departamentos, alterar o status de confirmação (Confirmado, Pendente, Não Comparece) e alternar entre o modo **Svelte Flow** e o modo **Grade/Cards**.
5. O botão "Copiar Escala para WhatsApp" gera texto formatado com emojis de cada departamento e situação dos voluntários.
6. A escala marcada como ativa é refletida imediatamente no WebSocket e visível no Remote Mobile (`/remote`).
7. As rotas `/docs`, `/scalar` e `/swagger` carregam a documentação interativa OpenAPI 3.0.3 sem qualquer conexão com a internet (100% offline via `rust-embed`).
8. Todos os portões de qualidade (`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm run build`) passam sem erros.

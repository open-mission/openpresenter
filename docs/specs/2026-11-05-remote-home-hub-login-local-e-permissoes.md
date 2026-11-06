# Spec — Home do Remote (Portal do Voluntário), Login Local e Controle de Acesso

## 1. Intenção

Prover no Controle Remoto Mobile (`/remote`) uma experiência moderna de Portal do Voluntário (**Home Hub**), permitindo que membros da equipe acessem o aplicativo em seus smartphones ou tablets para:
1. Identificar-se através de um sistema de login local simples, intuitivo e persistente (com seleção de perfil e PIN opcional).
2. Visualizar em destaque quando estão escalados para o culto ativo/próximo, com sua função, departamento e botões instantâneos de confirmação de presença (Confirmado, Pendente, Não Comparece).
3. Acessar o controle remoto de projeção (músicas, bíblia, setlist, blackout) caso possuam permissão de operador (`can_control_remote` ou Líderes/Vice-Líderes), ou desbloquear mediante PIN de operador da mesa.
4. Ter o hub preparado para o futuro Chat da Equipe de Mídia, com card informativo e canais de comunicação.

---

## 2. Contexto e Problema

1. **Acesso Direto Sem Identificação:** Anteriormente, abrir `/remote` em qualquer celular abria diretamente os controles de projeção e a setlist, sem saber quem era o voluntário acessando.
2. **Voluntários de Outras Áreas (Fotografia, Stories, etc.):** Nem todo voluntário da equipe de mídia precisa ou deve ter acesso livre para trocar slides e acionar blackout no meio do culto. Um voluntário de fotografia quer apenas saber sua escala, confirmar presença e acompanhar a ordem do culto.
3. **Falta de uma Tela Inicial (Home Hub):** Faltava um painel central acolhedor ("Home") que desse as boas-vindas ao voluntário, resumisse sua função no culto de hoje e direcionasse com clareza para as ferramentas certas.
4. **Preparação para o Futuro Chat:** A equipe precisa de um local natural no mobile onde o chat ao vivo entre operadores se integrará.

---

## 3. Escopo e Solução

### 3.1 Camada de Domínio (`crates/op-core`)
- Em `crates/op-core/src/teams.rs`:
  - `TeamMember`:
    - `can_control_remote: bool` (padrão `true`): indica se o membro tem permissão direta de operar a projeção.
    - `pin: Option<String>`: PIN numérico opcional (4 dígitos) para login seguro.
  - `UpsertTeamMember`:
    - `can_control_remote: Option<bool>`
    - `pin: Option<String>`

### 3.2 Camada de Persistência (`crates/op-db`)
- Nova migração aditiva `0012_team_member_remote_access.sql`:
  ```sql
  ALTER TABLE team_members ADD COLUMN can_control_remote INTEGER NOT NULL DEFAULT 1;
  ALTER TABLE team_members ADD COLUMN pin TEXT;
  ```
- Atualização dos métodos `list_team_members`, `get_team_member` e `save_team_member` no `Database` em `crates/op-db/src/lib.rs`.
- Schema version passa para `12`.

### 3.3 API REST & OpenAPI (`crates/op-admin`)
- Atualização da especificação OpenAPI 3.0.3 em `crates/op-admin/src/api_docs.rs` refletindo `can_control_remote` e `pin`.
- Rota para validação de PIN de operador (ou verificação local segura via endpoint / settings).

### 3.4 Interface do Usuário (Frontend Svelte 5 & Shadcn/Tailwind)
- **Tela de Identificação / Login Local (`/remote`)**:
  - Exibida quando não há sessão salva em `localStorage` (`openpresenter:remote-auth-session`).
  - Lista de voluntários da equipe cadastrados com avatars coloridos, iniciais, nomes, badges de cargo (Líder, Vice-Líder, Membro) e departamentos.
  - Campo de busca instantânea de voluntários.
  - Entrada de PIN quando o voluntário possui PIN configurado.
  - Opção "Operador da Mesa / Convidado" com inserção de PIN mestre.
- **Home Hub do Voluntário (`activeTab === 'home'`)**:
  - Nova aba inicial na barra inferior: `[🏠 Início]` | `[📋 Setlist]` | `[🎵 Músicas]` | `[📖 Bíblia]` | `[👥 Equipe]`.
  - **Hero Card de Boas-Vindas:**
    - Avatar, saudação "Olá, [Nome]", badge cromático de cargo, indicador de status online e botão "Trocar Usuário / Sair".
  - **Card de Escala Pessoal ("Minha Escala"):**
    - Identificação do culto ativo/próximo.
    - Se escalado: departamento com cor e ícone, função (Titular ou Auxiliar/Apoio), e botões interativos de presença de 1 toque (🟢 Confirmado, 🟡 Pendente, 🔴 Não Comparece) com persistência via WebSocket.
    - Se não escalado: aviso claro e botão para consultar escala geral.
  - **Hub de Ferramentas (Cards Interativos com Impeccable Shape):**
    - **Controle de Mídia:** Se autorizado, atalho direto para a setlist ao vivo com indicador do que está sendo projetado; se restrito, aviso amigável com opção de desbloqueio via PIN de operador.
    - **Equipe Completa:** Atalho para a aba de equipe e lista de contatos WhatsApp.
    - **Chat da Equipe (Em Breve):** Card teaser elegante com badge "Em Breve" e descrição das futuras salas de comunicação por departamento.
- **Modal de Membro no Admin (`TeamsModal.svelte`)**:
  - Adição dos campos "Pode controlar apresentação remota" (checkbox) e "PIN de Acesso Remoto" (campo numérico opcional de 4 dígitos).

---

## 4. Critérios de Aceite

1. A migração SQLite `0012_team_member_remote_access.sql` executa sem erros e o `schema_version()` do banco passa a ser `12`.
2. Os membros cadastrados possuem as propriedades `can_control_remote` e `pin` salvas e recuperadas corretamente via API e banco de dados.
3. Ao acessar `/remote` pela primeira vez ou sem sessão, o usuário visualiza a tela de login local com os membros da equipe e pode selecionar seu perfil (com ou sem PIN).
4. A sessão do membro permanece salva no `localStorage`, restaurando o perfil automaticamente em futuros acessos no mesmo dispositivo.
5. A nova aba "Início" exibe o card de escala pessoal indicando se o membro está escalado no culto ativo, qual departamento, e permite alternar o status de presença (Confirmado, Pendente, Não Comparece) com sincronização em tempo real.
6. Voluntários com `can_control_remote = true` acessam o controle de mídia diretamente; voluntários sem permissão visualizam o status restrito e podem desbloquear digitando o PIN de operador.
7. O card de Chat da Equipe é exibido com layout profissional e badge "Em Breve".
8. Todos os portões de qualidade (`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm run build`) passam com zero erros.

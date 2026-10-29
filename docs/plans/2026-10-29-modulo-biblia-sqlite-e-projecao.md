# Plano — Módulo de Bíblias SQLite, Navegação em 3 Colunas, Parser de Referências e Projeção

## Visão Geral

Este plano detalha a implementação da funcionalidade de Bíblias no OpenPresenter, dividida em fases lógicas: modelos e parser em `op-core`, catálogo e leitor SQLite em `op-db`, rotas de API em `op-admin`, interface completa no frontend (Settings, BottomPanel, StageView e CommandPalette), e portões de qualidade rigorosos.

Link para a spec: [`docs/specs/2026-10-29-modulo-biblia-sqlite-e-projecao.md`](../specs/2026-10-29-modulo-biblia-sqlite-e-projecao.md)  
Link para o ADR: [`docs/decisions/0010-modulo-biblia-sqlite-parser-referencias-e-stage.md`](../decisions/0010-modulo-biblia-sqlite-parser-referencias-e-stage.md)

---

## Fases de Implementação

### Fase 1: Domínio e Parser de Referências (`op-core`)

1. **Tipos de Dados Bíblicos:**
   - Adicionar estruturas `BibleVersionSummary`, `BibleBook`, `BibleVerse`, `BibleSearchResult`, `ParsedReference`.
2. **Tabela Canônica de Livros e Abreviações:**
   - Mapear os 66 livros da Bíblia protestante com IDs, nomes canônicos em português, abreviações usuais (Gn, Ex, Lv, etc.) e aliases normalizados (sem acentos, minúsculos).
3. **Parser de Referências Bíblicas:**
   - Implementar `parse_bible_reference(input: &str) -> Option<ParsedReference>` capaz de reconhecer referências completas (`joao 3.16`, `jo 3 16`, `joão 3:16`), intervalos (`1 co 13:4-7`) e capítulos isolados (`salmos 91`, `sl 91` focando no verso 1).
4. **Testes Unitários:**
   - Testar o parser com diversas variações de grafia, pontuação e intervalos.

### Fase 2: Banco de Dados e Leitor SQLite (`op-db`)

1. **Migração do Catálogo de Bíblias:**
   - Criar `0007_bibles.sql` para cadastrar as versões instaladas no banco `open-presenter.db`.
2. **Métodos no `Database`:**
   - Métodos CRUD para gerenciar a tabela `bibles`: listagem, recuperação, atualização de nome/sigla, deleção e definição de bíblia padrão.
3. **Leitor Otimizado de Arquivos SQLite de Bíblia:**
   - Implementar leitor para arquivos de Bíblia (com validação de schema `book`/`verse` e tabela `metadata`).
   - Leitura de livros e contagem de capítulos.
   - Consulta de versículos por livro e capítulo, ou intervalo de versículos.
   - Busca textual por palavra-chave com ordenação canônica.
   - Criação automática de índice `idx_verse_book_chapter` na importação.
4. **Testes Unitários:**
   - Testes de migração do schema para v7 e testes do leitor SQLite.

### Fase 3: Backend e Rotas REST (`op-admin`)

1. **Armazenamento de Arquivos:**
   - Suporte a pasta `<data_dir>/bibles/` no módulo de sessão.
2. **Rotas da API de Bíblias:**
   - `GET /api/bibles`: lista versões cadastradas.
   - `POST /api/bibles/import`: aceita upload ou caminho local de `.sqlite`/`.db`, copia para `<data_dir>/bibles/`, insere no catálogo e retorna os metadados.
   - `PUT /api/bibles/{id}`: atualiza nome, sigla e padrão.
   - `DELETE /api/bibles/{id}`: remove registro e arquivo físico.
   - `GET /api/bibles/{id}/books`: lista livros com contagem de capítulos.
   - `GET /api/bibles/{id}/books/{book_id}/chapters`: lista capítulos.
   - `GET /api/bibles/{id}/books/{book_id}/chapters/{chapter}/verses`: lista versículos do capítulo.
   - `GET /api/bibles/{id}/search?q=...`: busca textual e por referência.
   - `GET /api/bibles/parse-reference?q=...`: endpoint para análise da referência informada.
3. **Apresentação Rápida com Índice Inicial:**
   - Ajustar `quick_present_media` para aceitar `slide_index: Option<usize>` no payload da requisição.

### Fase 4: Interface do Usuário (Svelte)

1. **Aba de Bíblias no Modal de Configurações (`SettingsModal.svelte`):**
   - Nova aba lateral "Bíblias" com ícone de livro.
   - Lista das versões importadas com badges de sigla, contagem de livros/versículos e indicação de versão padrão.
   - Botão para importação de arquivos `.sqlite`.
   - Diálogo para renomear nome e sigla da versão (ex: ajustar ARA, NVI).
   - Botão para definir como versão padrão ou excluir.
2. **Correção do Preview de Proporções do Stage:**
   - Substituição da prévia fixa por renderização dinâmica baseada em `localStage.preset`, reagindo aos sliders de proporção e seletores de cor.
3. **Novo Painel de Bíblia em 3 Colunas (`BiblePanel.svelte` e `BottomPanel.svelte`):**
   - Nova aba "Bíblia" no rodapé expansível.
   - 3 colunas com barras de redimensionamento (divisores com drag):
     - Coluna 1: Lista de livros com seletor de testamentos e alternância entre nomes e abreviações canônicas.
     - Coluna 2: Grade de botões numéricos de capítulos.
     - Coluna 3: Grade de slides de versículos em 2 colunas.
   - Barra superior com seletor de versão bíblica e campo de busca/parser de referência.
   - Interações:
     - Duplo clique no versículo: inicia projeção ao vivo.
     - Teclas de setas Left/Right: navegam entre versículos na projeção.
     - Duplo clique em outro slide: salta imediatamente para ele.
     - Shift+clique: ativa seleção múltipla de versículos, exibindo botão e menu de contexto para "Adicionar à Setlist".
4. **Integração no Stage View e Public View:**
   - Exibição destacada da referência e texto dos versículos em todos os presets de palco e na tela pública.
5. **Integração na Command Palette (`CommandPalette.svelte`):**
   - Reconhecimento automático de referências bíblicas na busca global, com ações de abrir no painel (`Enter`) ou apresentar imediatamente (`Shift+Enter`).

### Fase 5: Validação e Portões de Qualidade

1. Executar `bun run check` no frontend.
2. Executar `cargo fmt --all`.
3. Executar `cargo clippy --workspace --all-targets -- -D warnings`.
4. Executar `cargo test --workspace`.
5. Atualizar documentação e changelog (`CHANGELOG.md`, `docs/architecture/overview.md`, `docs/README.md`).

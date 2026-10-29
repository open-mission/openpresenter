# Changelog

Todas as mudanças relevantes deste projeto são registradas neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e o projeto adere a [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

## [0.4.0] - 2026-10-28

### Adicionado

- **Command Palette Global (`⌘K` / `Ctrl+K`):**
  - Acessível via atalho global de teclado, clique na barra de busca superior ou no botão `+` da Setlist.
  - Busca instantânea em tempo real no banco de dados local com **SQLite FTS5** (`unicode61 remove_diacritics 2`) pesquisando simultaneamente por título e letra completa, com geração de snippets contextualizados.
  - Ação rápida para adicionar qualquer música encontrada diretamente à Setlist ativa com o atalho `Shift + Enter` ou botão dedicado, disparando sincronização WebSocket em tempo real.
  - Ações rápidas de criação manual e busca na web integradas na mesma paleta.
- **Busca de Músicas Online (Holyrics e Vagalume):**
  - Módulo `online_lyrics` em Rust com suporte nativo aos provedores Holyrics e Vagalume.
  - Implementação de handshake, token HMAC-SHA512 + MD5 e descompressão gzip (`.txt.gz`) para o CDN do Holyrics.
  - Importação de letras da internet diretamente para o formulário de cadastro com metadados pré-preenchidos (Título, Artista, Tom, BPM e Compositor).
- **Modal de Cadastro e Edição de Músicas:**
  - Interface ampla ocupando 95% da tela (`95vw x 95vh`) com layout dividido em dois painéis.
  - **Painel esquerdo:** formulário de metadados completos e editor de texto da letra com suporte à sintaxe padrão do Holyrics (`## Verso 1`, `## Refrão`, `## Ponte`, quebra de slides por linha em branco) e botões rápidos para inserção de partes.
  - **Painel direito:** visualização em tempo real dos slides gerados como cartões 16:9 (`aspect-video`), com tags de seções coloridas inspiradas no ProPresenter e numeração sequencial.
  - Opção para adicionar à Setlist automaticamente ao salvar.
- **Renomeação da barra lateral para Setlist:**
  - Painel lateral renomeado de "Biblioteca de Mídias" para "Setlist".
  - Botão `+` no cabeçalho da Setlist para abertura rápida da paleta de comandos.
- **Migração do Banco de Dados v3:**
  - Migração `0003_songs_fts.sql` criando as tabelas relacionais `songs`, `song_sections`, `song_lyrics` e a tabela virtual `songs_fts` via SQLite FTS5.
  - Funções de repositório `save_song`, `get_song`, `search_songs` e `delete_song` com suporte a busca fonética/sem acentos e fallback resiliente.

## [0.3.0] - 2026-10-28

### Adicionado

- **Arquitetura Web completa:** transição para backend em Rust + Axum e frontend em Svelte 5 + TypeScript + Vite.
- **Painel do Operador (Admin) via navegador:**
  - Layout de 3 colunas inspirado em ProPresenter e Holyrics.
  - **Sidebar de mídias:** catálogo de itens de mídia (louvores, textos bíblicos, avisos) com busca, filtros por categoria e itens no estilo shadcn-svelte (`MediaItemRow`).
  - **Painel central:** grade de slides da mídia selecionada com proporção 16:9, numeração de estrofes/versos, indicador visual de slide ao vivo e navegação rápida por teclado (`←`, `→`, `Espaço`).
  - **Painel de saídas e preview:** previews do **Public** e do **Stage** com aspect ratio 16:9 estrito, botões de ação para abrir as telas dedicadas em novas janelas do navegador e controles rápidos de projeção (**Ao Vivo**, **Blackout (F1)**, **Limpar Texto (F2)**).
- **Tela de Projeção Pública (`/public`):**
  - Rota dedicada e limpa para projetores e captura em OBS, com proporção 16:9, fundo escuro, tipografia de alto contraste e suporte a tela cheia nativa (`F`).
  - Atualização em tempo real via WebSocket a partir dos disparos do operador.
- **Tela de Retorno de Palco (`/stage`):**
  - Rota dedicada para monitor de retorno com relógio digital em tempo real (HH:MM:SS), data por extenso em português, slide atual destacado e pré-visualização do próximo slide a ser cantado/lido.
- **Backend Rust + Axum (`op-admin`):**
  - Servidor HTTP com rotas REST (`/api/status`, `/api/media`, `/api/presentation/live`, etc.).
  - Servidor WebSocket bidirecional em `/ws` com sincronização inicial e broadcast de eventos em tempo real (`tokio::sync::broadcast`).
  - Estado global em memória (`AppState`) thread-safe.
  - Serviço de entrega de estáticos compilados do frontend (`frontend/dist`) com fallback SPA.
- **Limpeza do projeto:** remoção definitiva dos arquivos de UI desktop legados (Slint, componentes vendorizados e scripts de build).

## [0.2.0] - 2026-10-28

### Adicionado

- Saídas **Public** e **Stage** como janelas independentes, com placeholders de marca; o Stage inclui relógio e data por extenso.
- Janela de **Configurações** independente, com a aba **Saídas**.
- Ativação, escolha de monitor e escolha de modo de exibição por saída, com aplicação imediata.
- Restauração das saídas habilitadas na inicialização, cada uma no monitor salvo.
- Migração v2 com a tabela `output_config` e as operações `load_outputs`/`save_output`.
- Estados degradados para banco indisponível, falha ao salvar e falha ao abrir saídas.

## [0.1.0] - 2026-10-28

### Adicionado

- Workspace Rust com `op-core`, `op-db` e `op-admin`.
- Banco SQLite com PRAGMAs WAL, integridade referencial e migrações numeradas.
- Registro de metadados da aplicação e controle de primeira e última execução em `app_meta`.

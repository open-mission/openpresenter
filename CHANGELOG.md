# Changelog

Todas as mudanças relevantes deste projeto são registradas neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e o projeto adere a [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

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

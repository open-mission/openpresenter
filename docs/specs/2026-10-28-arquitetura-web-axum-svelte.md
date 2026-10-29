# Spec — Transição para Arquitetura Web (Rust/Axum + Svelte/shadcn-svelte)

- **Status:** aprovada
- **Data:** 2026-10-28
- **Autores:** OpenPresenter Core Team
- **Decisão relacionada:** [ADR 0004](../decisions/0004-arquitetura-web-axum-svelte.md)

## 1. Intenção

Migrar o OpenPresenter Minimal para uma arquitetura baseada em tecnologias web, permitindo que os painéis **Admin**, **Public** e **Stage** sejam acessíveis diretamente pelo navegador web. O backend em Rust fornecerá APIs REST, conexão WebSocket em tempo real, persistência em SQLite e estado global em memória. O frontend será desenvolvido com Svelte, TypeScript, Vite e componentes no estilo shadcn-svelte, inspirado nos fluxos operacionais consagrados do ProPresenter e Holyrics.

## 2. Escopo

### Incluído

1. **Backend (`crates/op-admin`):**
   - Servidor HTTP e WebSocket com Axum (`0.8`) e Tokio.
   - Estado global compartilhado (`AppState`):
     - Mídia ativa e catálogo de mídias pré-carregadas.
     - Índice do slide atual projetado no Public.
     - Modos de tela: `Live` (normal), `Blackout` (tela preta total), `ClearText` (mantém fundo/logo, oculta texto).
     - Estado das saídas Public e Stage.
   - Rotas REST:
     - `GET /api/status`: metadados da sessão, versão, status do SQLite e saídas.
     - `GET /api/media`: lista de itens de mídia com seus respectivos slides.
     - `POST /api/presentation/live`: define a mídia e slide ativos.
     - `POST /api/presentation/blackout`: alterna estado de blackout.
     - `POST /api/presentation/clear`: alterna estado de ocultação de texto.
   - Endpoint WebSocket `/ws`:
     - Handshake e canal bidirecional.
     - Broadcast em tempo real de eventos (`state_changed`, `slide_changed`, `blackout_toggled`, etc.).
   - Serviço de arquivos estáticos: entrega dos arquivos compilados do frontend (`frontend/dist`) ou página de fallback informativa caso o build ainda não tenha sido executado.

2. **Frontend (`frontend/`):**
   - Aplicação Svelte 5 + TypeScript + Vite + Tailwind CSS.
   - Componentes modulares inspirados no padrão shadcn-svelte (`$lib/components/ui/`).
   - Roteamento e visões:
     - **Painel Admin (`/` ou `/admin`):**
       - **Sidebar de mídias (esquerda):** lista de itens de mídia (louvores, leituras bíblicas, anúncios) com categoria, título, contagem de slides e busca rápida.
       - **Painel central:** cabeçalho da mídia selecionada e grade/lista de slides com preview visual de cada estrofe/verso. Clique em um slide envia imediatamente para a saída ao vivo.
       - **Painel de saídas e preview (direita):**
         - Preview do **Public** em aspecto 16:9.
         - Preview do **Stage** em aspecto 16:9 (mostrando relógio, slide atual e próximo slide).
         - Botões de ação para abrir as telas dedicadas: **Abrir Public** e **Abrir Stage** (abre em nova aba ou janela para mover para o projetor/retorno).
         - Ações rápidas de projeção: **Ao Vivo**, **Blackout (F1)** e **Limpar Texto (F2)**.
     - **Tela Public (`/public`):**
       - Viewport limpo e sem bordas, proporção 16:9, fundo escuro, texto com tipografia legível e centralizada.
       - Atualização instantânea via WebSocket quando o operador muda de slide ou aciona blackout.
     - **Tela Stage (`/stage`):**
       - Monitor de retorno para palco com relógio digital (HH:MM:SS) em tempo real, data em português, exibição destacada do slide atual e cartão com prévia do próximo slide.

3. **Limpeza do projeto:**
   - Remoção de todos os arquivos de UI legados (Slint, vendor/material, build.rs antigo).
   - Atualização de dependências do workspace no `Cargo.toml`.

### Não incluído nesta etapa

- Editor avançado de temas WYSIWYG no navegador (será abordado em etapas posteriores).
- Renderização de vídeo em segundo plano por hardware acelerado avançado (será adicionado posteriormente).

## 3. Decisões técnicas

1. **Separação de responsabilidades:**
   - `op-core` contém tipos puros de domínio (incluindo modelos de mídia e slides serializáveis).
   - `op-db` gerencia o banco SQLite, histórico e metadados.
   - `op-admin` é o servidor Axum que orquestra HTTP, WebSocket e integra com o banco.
   - `frontend` é o aplicativo cliente Svelte servido pelo Axum ou executado via Vite no ciclo de desenvolvimento.
2. **Sincronização em tempo real:**
   - Ao conectar em `/ws`, o cliente recebe imediatamente o estado completo atual (`InitialState`).
   - Qualquer comando emitido pelo Admin atualiza o `AppState` atômico no backend e emite um evento broadcast para todos os clientes conectados (incluindo telas `/public` e `/stage`).
3. **Proporção 16:9 garantida:**
   - Nos painéis de preview e nas saídas de projeção, o container utiliza `aspect-video` (16:9) do Tailwind para garantir fidelidade de visualização.

## 4. Critérios de aceite

1. O comando `cargo run` inicia com sucesso o servidor Axum na porta 3000 (ou porta definida).
2. O usuário pode abrir `http://localhost:3000` no navegador e visualizar o painel Admin.
3. No painel Admin, o usuário visualiza:
   - Sidebar com itens de mídia usando componentes de lista/item.
   - Painel central com os slides da mídia selecionada.
   - Painel direito com os previews do Public e do Stage em proporção 16:9.
   - Botões para iniciar/abrir o Public (`/public`) e o Stage (`/stage`).
4. Ao clicar nos botões para abrir Public e Stage, novas abas ou janelas são abertas exibindo a projeção pública e o retorno de palco.
5. Ao selecionar slides no painel Admin, a atualização é refletida em tempo real via WebSocket nos previews do Admin, na tela `/public` e na tela `/stage`.
6. Arquivos e dependências legadas de interface foram removidos.
7. Comandos de validação do Rust executam com sucesso:
   - `cargo fmt --all`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
8. O frontend compila sem erros (`bun run build` ou `npm run build`).

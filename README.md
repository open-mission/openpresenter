# OpenPresenter Minimal

Aplicativo web e servidor de projeção para igrejas e eventos inspirado em ProPresenter e Holyrics. Esta é a versão **0.3.0**: backend robusto em **Rust + Axum** (com API REST, WebSocket em tempo real e SQLite) e frontend moderno em **Svelte 5 + TypeScript + Vite + Tailwind CSS + shadcn-svelte**, permitindo acesso a todas as telas diretamente pelo navegador web.

## Stack

- **Backend:** Rust 2024 (MSRV 1.95) com Axum 0.8, Tokio 1.44 e Tower-HTTP
- **Tempo Real:** WebSocket bidirecional em `/ws` via `tokio::sync::broadcast`
- **Banco de Dados:** SQLite via `rusqlite` (`bundled`) com WAL e `rusqlite_migration`
- **Frontend:** Svelte 5, TypeScript, Vite, Tailwind CSS e componentes no padrão `shadcn-svelte`

## Visões e Rotas

- **Painel do Operador (Admin):** `http://localhost:3000` (ou `/admin`)
  - **Sidebar de Mídias:** lista de louvores, escrituras bíblicas e avisos usando itens no estilo Shadui (`MediaItemRow`), com busca e filtros por categoria.
  - **Painel Central:** visualização dos slides da mídia selecionada com proporção 16:9, indicador de slide ao vivo e navegação rápida (`←`, `→`, `Espaço`).
  - **Painel de Saídas & Previews:** pré-visualização em tempo real do **Public** e do **Stage** respeitando a proporção 16:9, botões para abrir as telas dedicadas e atalhos de projeção (**Ao Vivo**, **Blackout (F1)**, **Limpar Texto (F2)**).
- **Tela de Projeção Pública:** `http://localhost:3000/public`
  - Saída limpa para projetor e captura no OBS, com tipografia em alto contraste, proporção 16:9 e tela cheia nativa (`F`).
- **Tela de Retorno do Palco:** `http://localhost:3000/stage`
  - Retorno para o palco com relógio digital em tempo real (HH:MM:SS), data por extenso em português, slide atual destacado e prévia do próximo slide.

## Como Executar

### 1. Construir o frontend:

```bash
cd frontend
bun install   # ou npm install / pnpm install
bun run build # ou npm run build
cd ..
```

### 2. Iniciar o servidor OpenPresenter:

```bash
cargo run -p op-admin
```

O servidor iniciará em `http://localhost:3000`.

### Modo Desenvolvimento com Hot Reload:

Em um terminal:
```bash
cargo run -p op-admin
```

Em outro terminal:
```bash
cd frontend
bun run dev
```
O Vite proxy roteará automaticamente `/api` e `/ws` para `http://localhost:3000`.

## Portões de Qualidade

Antes de qualquer entrega, todos os testes e verificações estritas devem passar:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd frontend && bun run check && bun run build
```

## Dados

O banco `open-presenter.db` fica no diretório local de dados do sistema:

- macOS: `~/Library/Application Support/OpenPresenter`
- Windows: `%LOCALAPPDATA%\OpenPresenter`
- Linux: `~/.local/share/OpenPresenter`

Para desenvolvimento em diretório isolado:

```bash
OPENPRESENTER_DATA_DIR=/tmp/openpresenter-dev cargo run -p op-admin
```

## Estrutura do Repositório

```text
crates/
├── op-core/    # domínio puro: constantes, modelos de mídia, slides, AppMeta e mensagens WsMessage
├── op-db/      # SQLite, PRAGMAs, migrações numeradas e persistência
└── op-admin/   # binário open-presenter: Axum, WebSocket, rotas e serviço de arquivos estáticos
frontend/       # aplicação cliente Svelte + TypeScript + Vite + Tailwind CSS
assets/branding/ # assets visuais da marca OpenPresenter
docs/           # especificações, planos de execução e decisões arquiteturais (ADRs)
```

A direção de dependências é: `frontend` → `op-admin` → `op-db` → `op-core`.

## Documentação

O índice completo está em [`docs/README.md`](docs/README.md).

## Licença

Código distribuído sob licença **GPL-3.0-only** — veja [`LICENSE`](LICENSE). A política de marca e nome está detalhada em [`TRADEMARK.md`](TRADEMARK.md).

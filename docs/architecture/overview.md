# Visão da arquitetura

Esta página descreve a arquitetura vigente do OpenPresenter Minimal. Ela deve ser atualizada sempre que a estrutura, o fluxo de inicialização ou as dependências mudarem.

## Estrutura do Repositório

```text
.
├── crates/
│   ├── op-core/    # domínio puro: constantes, modelos de mídia, slides, Song, SongSection, SongSummary, AppMeta, SessionInfo, Theme, saídas e mensagens WsMessage
│   ├── op-db/      # SQLite, PRAGMAs, migrações numeradas (v1, v2, v3 com FTS5) e persistência da sessão, saídas e catálogo de músicas
│   └── op-admin/   # binário open-presenter: servidor HTTP (Axum), busca online de letras (Holyrics, Vagalume), WebSocket, estado global e serviço de estáticos
├── frontend/       # aplicação cliente em Svelte 5 + TypeScript + Vite + Tailwind CSS + shadcn-svelte
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/ui/  # componentes no padrão shadcn-svelte (Button, Badge, Card, MediaItemRow)
│   │   │   ├── components/     # CommandPalette.svelte, SongEditorModal.svelte
│   │   │   ├── views/          # visões principais: AdminView, PublicView, StageView
│   │   │   ├── songParser.ts   # segmentador de letras estilo Holyrics (## Partes, quebras de slides)
│   │   │   ├── types.ts        # tipagens e contratos de dados sincronizados com op-core
│   │   │   ├── utils.ts        # utilitários de classe (cn, twMerge, clsx)
│   │   │   └── ws.ts           # cliente WebSocket reativo e disparo de ações
│   │   ├── App.svelte          # roteamento SPA para /, /admin, /public e /stage
│   │   └── main.ts             # ponto de montagem do Svelte
└── docs/           # documentação técnica, specs, planos e decisões arquiteturais (ADRs)
```

`op-core` não conhece web, UI nem SQLite. `op-db` encapsula `rusqlite` e não expõe APIs de UI ou rede. `op-admin` é o servidor Axum que orquestra HTTP, WebSocket, resolve o diretório de dados e integra com `op-db`. O `frontend` consome as APIs e o canal WebSocket.

Direção de dependências:

```text
frontend (HTTP/WS) → op-admin → op-db → op-core
```

`op-admin` depende de `axum` 0.8, `tokio` 1.44, `tower-http` 0.6, `serde` e `serde_json`. `op-core` e `op-db` continuam sem conhecer rede, UI ou plataforma.

## Módulos de `op-admin`

```text
crates/op-admin/src/
├── main.rs         # ponto de entrada do binário, setup do listener Axum e roteador principal
├── routes.rs       # rotas REST para status, setlist, catálogo FTS5 e busca online
├── online_lyrics.rs# clientes para busca e download de letras em Holyrics e Vagalume
├── state.rs        # AppState global thread-safe e acervo inicial de demonstração
├── ws.rs           # handler WebSocket (/ws) com sincronização inicial e broadcast de eventos
├── session.rs      # inicialização de sessão, data directory e metadados no SQLite
└── static_files.rs # serviço dos arquivos estáticos compilados (frontend/dist) com fallback SPA
```

## Fluxo de Inicialização

1. `op-admin` resolve o diretório de dados: `OPENPRESENTER_DATA_DIR` ou `BaseDirs::data_local_dir()/OpenPresenter`.
2. Cria o diretório se inexistente, abre `open-presenter.db`, aplica os PRAGMAs e roda as migrações numeradas do SQLite.
3. Lê o registro anterior de `app_meta`, se existir.
4. Grava a sessão atual: preserva `first_run_at` e as preferências, atualiza `last_run_at`, `app_version` e `updated_at`.
5. Monta `SessionView` com versão, schema, caminho do banco, primeira execução e última execução anterior.
6. Carrega `OutputsConfig` do banco; sem linhas salvas, usa os padrões definidos pela especificação.
7. Inicializa `AppState` em memória contendo `SessionView`, `OutputsConfig`, `PresentationState`, a biblioteca inicial de mídias e o canal de transmissão `broadcast::Sender<WsMessage>`.
8. Configura o roteador Axum integrando as rotas de API, o endpoint WebSocket `/ws`, middlewares de CORS e tracing, e o handler de entrega dos arquivos estáticos do frontend.
9. Inicia o listener TCP na porta configurada (padrão: 3000) e exibe as URLs de acesso no terminal.
10. Ao acessar `http://localhost:3000` pelo navegador:
    - O frontend Svelte carrega a visão **Admin** por padrão.
    - O cliente estabelece conexão WebSocket com `/ws`, envia identificação e recebe imediatamente o estado completo (`SyncState`).
    - O operador pode navegar pelas mídias na sidebar (com componentes `MediaItemRow` do Shadui), selecionar slides no painel central e acompanhar previews 16:9 em tempo real do **Public** e do **Stage**.
    - Clicar em "Abrir Public" abre a rota `/public` em nova janela ou aba para projetor ou OBS.
    - Clicar em "Abrir Stage" abre a rota `/stage` em nova janela ou aba para retorno do palco com relógio digital em tempo real.

## Banco de Dados

- Arquivo: `open-presenter.db`.
- PRAGMAs: `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`.
- Schema v1: tabela singleton `app_meta` (`id = 1`) com `app_version`, `first_run_at`, `last_run_at`, `theme`, `locale` e `updated_at`.
- Schema v2: tabela `output_config`, com uma linha por saída (`public` ou `stage`), `enabled`, `monitor_key`, `monitor_name`, `mode` e `updated_at`.
- Schema v3: tabelas `songs`, `song_sections`, `song_lyrics` e virtual table `songs_fts` usando SQLite FTS5 (`unicode61 remove_diacritics 2`) para busca full-text instantânea por título e letra com snippets.
- As migrações são estritamente aditivas e numeradas.
- Datas em RFC 3339 UTC com milissegundos.

## Protocolo WebSocket

As mensagens entre o painel do operador, as telas de saída e o servidor utilizam o formato JSON sob o tipo `WsMessage`:

- `sync_state`: enviado pelo servidor ao cliente assim que a conexão WebSocket é aberta.
- `identify`: enviado pelo cliente ao conectar (`client_type: "admin" | "public" | "stage"`).
- `select_slide`: enviado pelo operador para definir mídia ativa e índice do slide projetado.
- `set_display_mode`: enviado pelo operador para alternar modo de exibição (`live`, `blackout`, `cleartext`).
- `state_updated`: broadcast emitido a todos os clientes conectados informando alterações de estado.
- `ping` / `pong`: manutenção de conexão ativa contra timeouts de rede.

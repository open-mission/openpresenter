# Visão da arquitetura

Esta página descreve a arquitetura vigente do OpenPresenter Minimal. Ela deve ser atualizada sempre que a estrutura, o fluxo de inicialização ou as dependências mudarem.

## Estrutura do Repositório

```text
.
├── crates/
│   ├── op-core/    # domínio puro: constantes, modelos de mídia, slides, Setlist, SetlistSummary, Song, SongSection, SongSummary, MediaFolder, TimerState, AppMeta, SessionInfo, Theme, saídas e mensagens WsMessage
│   ├── op-db/      # SQLite, PRAGMAs, migrações numeradas (v1 a v6 app_settings) e persistência da sessão, saídas, catálogo, setlists, pastas mapeadas e configurações
│   └── op-admin/   # binário open-presenter: servidor HTTP (Axum), armazenamento e streaming de mídias, busca online de letras (Holyrics, Vagalume), WebSocket, loop de timer global, estado global e estáticos
├── frontend/       # aplicação cliente em Svelte 5 + TypeScript + Vite + Tailwind CSS + shadcn-svelte
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/ui/  # componentes no padrão shadcn-svelte (Button, Badge, Card, MediaItemRow)
│   │   │   ├── components/     # CommandPalette, SongEditorModal (Música/Texto), SettingsModal, SetlistModal, QuickItemModal, ContextMenu, BottomPanel, TimerControlPanel
│   │   │   ├── views/          # visões principais: AdminView, PublicView, StageView (com 4 presets)
│   │   │   ├── settings.ts     # gerenciamento de tema (dark/light/system), Base Colors e Theme Accents (Shadcn), seções customizadas
│   │   │   ├── displaySettings.ts # gerenciamento persistente de margens públicas, marca da igreja e presets do monitor de palco
│   │   │   ├── songParser.ts   # segmentador de letras e textos estilo Holyrics (## Partes, quebras de slides)
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
├── main.rs         # ponto de entrada, event loop nativo (tao) e inicialização Tokio em background
├── tray.rs         # System Tray nativo (tray-icon + muda) e diálogos de configuração de porta
├── routes.rs       # rotas REST para status, setlist, catálogo FTS5, bíblias e busca online
├── online_lyrics.rs# clientes para busca e download de letras em Holyrics e Vagalume
├── state.rs        # AppState global thread-safe e acervo inicial de demonstração
├── ws.rs           # handler WebSocket (/ws) com sincronização inicial e broadcast de eventos
├── session.rs      # inicialização de sessão, data directory e metadados no SQLite
├── static_files.rs # serviço de assets embutidos no binário (rust-embed) com fallback SPA
└── updater.rs      # verificação de releases no GitHub, download e auto-update in-app
```

## Fluxo de Inicialização

1. `op-admin` resolve o diretório de dados: `OPENPRESENTER_DATA_DIR` ou `BaseDirs::data_local_dir()/OpenPresenter`.
2. Cria o diretório se inexistente, abre `open-presenter.db`, aplica os PRAGMAs e roda as migrações numeradas do SQLite.
3. Lê o registro anterior de `app_meta`, se existir.
4. Grava a sessão atual: preserva `first_run_at` e as preferências, atualiza `last_run_at`, `app_version` e `updated_at`.
5. Monta `SessionView` com versão, schema, caminho do banco, primeira execução e última execução anterior.
6. Carrega `OutputsConfig` do banco; sem linhas salvas, usa os padrões definidos pela especificação.
7. Resolve a porta do servidor: verifica variável `PORT`, depois `app_settings.server_port` no SQLite, utilizando a porta `18080` como padrão seguro e livre de conflitos.
8. Inicializa `AppState` em memória contendo `SessionView`, `OutputsConfig`, `PresentationState`, a biblioteca inicial de mídias e o canal de transmissão `broadcast::Sender<WsMessage>`.
9. Inicia o servidor HTTP Axum em uma runtime assíncrona Tokio dedicada em background, servindo os assets compilados do frontend embutidos diretamente no executável via `rust-embed`.
10. Se a flag `--headless` estiver ativa, o servidor opera em modo terminal/daemon puro.
11. Em modo padrão desktop (Windows, macOS e Linux):
    - A thread principal executa o event loop nativo (`tao`).
    - O ícone oficial da marca é exibido na bandeja do sistema (**System Tray / Menu Bar**).
    - O navegador padrão é aberto automaticamente em `http://localhost:18080`.
    - O menu de contexto na bandeja oferece opções diretas para abrir o Painel Admin, a Tela Pública, o Retorno de Palco, configurar a porta do servidor e encerrar a aplicação.

## Banco de Dados

- Arquivo: `open-presenter.db`.
- PRAGMAs: `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`.
- Schema v1: tabela singleton `app_meta` (`id = 1`) com `app_version`, `first_run_at`, `last_run_at`, `theme`, `locale` e `updated_at`.
- Schema v2: tabela `output_config`, com uma linha por saída (`public` ou `stage`), `enabled`, `monitor_key`, `monitor_name`, `mode` e `updated_at`.
- Schema v3: tabelas `songs`, `song_sections`, `song_lyrics` e virtual table `songs_fts` usando SQLite FTS5 (`unicode61 remove_diacritics 2`) para busca full-text instantânea por título e letra com snippets.
- Schema v4: tabelas `setlists` e `setlist_items` para suporte a múltiplas listas de louvor, papel de parede e metadados.
- Schema v5: tabela `media_folders` para mapeamento externo de diretórios do computador (zero-copy).
- Schema v6: tabela `app_settings` para armazenamento de pares chave/valor de configurações de telas (`public_display`, `stage_display`) e marca da congregação (`church_brand`).
- Schema v7: tabela `bibles` para catálogo das versões da Bíblia importadas (`id`, `name`, `abbreviation`, `filename`, `is_default`, `created_at`, `updated_at`). Arquivos `.sqlite` individuais são armazenados em `<data_dir>/bibles/`.
- As migrações são estritamente aditivas e numeradas.
- Datas em RFC 3339 UTC com milissegundos.

## Protocolo WebSocket

As mensagens entre o painel do operador, as telas de saída e o servidor utilizam o formato JSON sob o tipo `WsMessage`:

- `sync_state`: enviado pelo servidor ao cliente assim que a conexão WebSocket é aberta.
- `identify`: enviado pelo cliente ao conectar (`client_type: "admin" | "public" | "stage"`).
- `select_slide`: enviado pelo operador para definir mídia ativa e índice do slide projetado.
- `set_display_mode`: enviado pelo operador para alternar modo de exibição (`live`, `blackout`, `cleartext`).
- `stop_presentation`: limpa a projeção atual e remove indicadores de ao vivo.
- `set_wallpaper`: atualiza o papel de parede e gradientes de fundo no ar.
- `start_timer`, `pause_timer`, `reset_timer`, `set_timer`: controle do cronômetro sincronizado com overtime.
- `set_stage_message`: envia mensagem instantânea ou aviso para o monitor de palco.
- `timer_updated`: broadcast com tick a cada 1 segundo com tempo restante e alerta de overtime.
- `state_updated`: broadcast emitido a todos os clientes conectados informando alterações de estado.
- `ping` / `pong`: manutenção de conexão ativa contra timeouts de rede.

## Atualização Automática (Auto-Updater)

- Verificação periódica ou sob demanda via API do GitHub Releases (`open-mission/openpresenter`).
- Comparação semântica de versões (`semver`) contra a versão do binário em execução.
- Download atômico do pacote compactado correspondente ao SO (`.zip` no Windows, `.tar.gz` no macOS e Linux) para diretório temporário.
- Extração e substituição atômica no caminho de `current_exe()`, tratando locks de arquivo no Windows através de renomeação temporária `.old.exe`.
- Preservação total de dados: banco SQLite (`open-presenter.db`), mídias e bíblias continuam intactos no diretório de dados do usuário.

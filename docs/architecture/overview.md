# Visão da arquitetura

Esta página descreve a arquitetura vigente do OpenPresenter Minimal. Ela deve ser atualizada sempre que a estrutura, o fluxo de inicialização ou as dependências mudarem.

## Workspace

```text
crates/
├── op-core/    # domínio puro: constantes, AppMeta, SessionInfo, Theme e configuração das saídas
├── op-db/      # SQLite, PRAGMAs, migrações e persistência da sessão e das saídas
└── op-admin/   # binário open-presenter: sessão, monitores, janelas Slint e Configurações
```

`op-core` não conhece Slint nem SQLite. `op-db` encapsula `rusqlite` e não expõe APIs de UI. `op-admin` é o único crate que conhece Slint, resolve o diretório de dados e enumera monitores.

Direção de dependências:

```text
op-admin → op-db → op-core
```

`op-admin` também depende de `display-info` 0.5 para enumerar monitores. A dependência é usada apenas nesse crate; `op-core` e `op-db` continuam sem conhecer plataforma ou UI.

## Módulos de `op-admin`

```text
crates/op-admin/src/
├── main.rs        # inclui os componentes Slint e inicia a UI
├── session.rs     # carrega a sessão e mantém a conexão viva
├── monitors.rs    # enumeração, normalização e geometria dos monitores
├── outputs.rs     # controlador das janelas Public/Stage e do relógio do Stage
├── settings.rs    # estado, callbacks e atualização da janela de Configurações
└── ui.rs          # monta o painel, o menu, os timers e o loop de eventos
```

Componentes Slint em `crates/op-admin/ui/`:

- `main.slint` — painel administrativo, botão de menu ☰, `PopupMenu` com **Configurações…** e reexportação das janelas.
- `settings.slint` — janela **Configurações — OpenPresenter**, independente, com a aba **Saídas**.
- `outputs.slint` — janelas `PublicWindow` e `StageWindow`, com placeholders de marca.
- `theme.slint` — tokens de cor e tipografia do painel e das superfícies de saída.

## Fluxo de inicialização

1. `op-admin` resolve o diretório de dados: `OPENPRESENTER_DATA_DIR` ou `BaseDirs::data_local_dir()/OpenPresenter`.
2. Cria o diretório, abre `open-presenter.db`, aplica os PRAGMAs e roda as migrações.
3. Lê o registro anterior de `app_meta`, se existir.
4. Grava a sessão atual: preserva `first_run_at` e as preferências, atualiza `last_run_at`, `app_version` e `updated_at`.
5. Monta `SessionView` com versão, schema, caminho do banco, primeira execução e última execução anterior.
6. Carrega `OutputsConfig` do banco; sem linhas, usa os padrões. Uma falha de leitura vira erro de banco do controlador.
7. Cria o `OutputsController` e a janela principal, preenche a sessão e mostra o painel; em seguida liga o menu e os callbacks de abertura e fechamento.
8. Depois de mostrar o painel, enumera os monitores na thread principal. Falha ou lista vazia resultam em lista vazia e aviso; o app continua.
9. Abre as saídas habilitadas, mantendo os handles fortes no controlador. A geometria é aplicada depois que cada janela existe; maximizar ou entrar em tela cheia vem em seguida. O relógio do Stage só é criado se a janela do Stage abrir.
10. Inicia um timer de 500 ms que atualiza o estado da janela de Configurações enquanto ela estiver visível.
11. Executa um único loop de eventos do Slint. Fechar o painel esconde as Configurações, fecha todas as saídas e chama `quit_event_loop`.

## Banco de dados

- Arquivo: `open-presenter.db`.
- PRAGMAs: `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`.
- Schema v1: tabela singleton `app_meta` (`id = 1`) com `app_version`, `first_run_at`, `last_run_at`, `theme`, `locale` e `updated_at`.
- O schema v2 acrescenta `output_config`, com uma linha por saída (`public` ou `stage`), `enabled`, `monitor_key`, `monitor_name`, `mode` e `updated_at`.
- A migração `0002_output_config.sql` é aditiva e não altera `app_meta`; a atualização v1 → v2 é automática na abertura.
- `load_outputs` sempre devolve uma configuração completa: ausência de linha usa os padrões e valores inválidos de `mode` ou `monitor_key` caem para `Auto`.
- `save_output` faz upsert por `output`, preservando a outra saída.
- Datas em RFC 3339 UTC com milissegundos.
- Migrações em `crates/op-db/migrations/`, embutidas com `include_str!` e controladas por `PRAGMA user_version`.

## Monitores e geometria

`monitors::list()` chama `DisplayInfo::all()`, converte cada monitor para `MonitorInfo` e ordena por `x`, depois `y`. Cada item guarda a chave nativa, o nome amigável, o rótulo com resolução, o retângulo, a escala e os indicadores de principal e embutido.

- O `display-info` devolve coordenadas lógicas no macOS e no Linux (X11 e Wayland) e físicas no Windows. `normalize_rect` converte tudo para pixels físicos antes de chamar a API do Slint, que trabalha com posição física. Escalas não finitas, zero ou negativas são tratadas como `1.0`.
- `resolve` combina a preferência (`Auto`, `Primary` ou `Display { id, name }`) com a lista de monitores e devolve o monitor escolhido, o modo efetivo e um aviso opcional.
- `Auto` prefere o primeiro monitor externo; sem externo, o principal; sem principal, o primeiro da lista. `Display` tenta o `id`, depois o nome salvo e, por fim, o principal ou o primeiro disponível, sinalizando o fallback. A preferência original não é sobrescrita.
- `effective_mode`: `Auto` abre o Public em tela cheia quando o monitor não é o principal e em janela quando é; o Stage em `Auto` abre sempre em janela. Os modos explícitos (`Windowed`, `Maximized`, `Fullscreen`) são aplicados diretamente. Sem monitor, o modo cai para janela.
- `placement` usa tamanhos lógicos — 960×540 para o Public e 720×480 para o Stage — converte pela escala do monitor, centraliza no retângulo físico e aplica cascata de 64 px (Public) e 128 px (Stage) quando o destino é o principal. A posição e o tamanho são limitados ao monitor.
- Não há watcher de hot-plug: a lista é recalculada na inicialização e ao abrir as Configurações.

## Saídas e ciclo de vida

`OutputsController` mantém a configuração em memória, a lista de monitores, os handles das janelas, o estado de visibilidade, as falhas de abertura e o timer do relógio do Stage.

- **Inicialização:** abre apenas as saídas com `enabled = true`, uma única vez.
- **Ativar:** abre a janela imediatamente e persiste `enabled = true`.
- **Desativar:** fecha a janela, descarta o timer do Stage quando aplicável e persiste `enabled = false`.
- **Fechar pelo X:** `on_close_requested` apenas oculta a janela nesta sessão; `enabled` permanece verdadeiro e o estado exibido é **Oculta**. A saída volta na próxima inicialização.
- **Abrir agora:** reabre uma saída habilitada que foi ocultada.
- **Mudar monitor ou modo:** persiste e reaplica a geometria apenas se a janela estiver visível; uma janela oculta usa a nova configuração na próxima abertura.
- **Fechar o painel:** encerra o loop de eventos e todas as saídas. A janela de Configurações pode ser fechada sem afetar as saídas.

Os placeholders do Public e do Stage não têm controles interativos. O Stage mantém um relógio de 1 s e a data por extenso em português, ativos apenas enquanto a janela existe.

## Configurações

- A janela é criada sob demanda na primeira abertura do menu e o handle é reutilizado nas próximas.
- Ao abrir, o controlador relê os monitores e a janela é preenchida com modelos, índices, estados e avisos.
- Os callbacks chamam o controlador; a mudança é aplicada em memória, persistida e refletida na janela.
- O polling de 500 ms atualiza apenas estados, visibilidade e erros, sem recriar modelos nem fechar dropdowns.
- Falha de persistência aparece em uma faixa separada e não desfaz a mudança em memória.

## Versões

- Versão do app: SemVer em `[workspace.package] version`; os crates herdam e a UI lê `op_core::VERSION` (origem: `CARGO_PKG_VERSION`).
- Versão do schema: independente, controlada por `PRAGMA user_version` e pelas migrações numeradas. A versão atual é 2.
- Migrações já aplicadas são imutáveis; mudanças criam um novo arquivo numerado.

## Estado degradado

`op-db` expõe erros tipados de abertura, migração, leitura e gravação. `op-admin` converte o erro em uma mensagem curta para a interface e registra o detalhe em `stderr`.

| Situação | Comportamento |
|----------|---------------|
| Banco indisponível ou falha ao carregar as saídas | Painel abre; nenhuma saída abre; Configurações mostram o erro e os controles ficam desabilitados |
| Falha ao enumerar monitores ou lista vazia | Lista vazia; saídas abrem como janelas na posição padrão do sistema; aviso nas Configurações |
| Monitor salvo ausente | Usa o principal ou o primeiro disponível; aviso no cartão; preferência preservada |
| Falha ao criar ou mostrar uma janela | Registra em `stderr`; estado **Falha ao abrir**; o painel continua |
| Falha ao salvar a configuração | Faixa de erro nas Configurações; a configuração em memória continua aplicada |
| Modo ou monitor inválido no banco | Cai para `Auto`, sem impedir a abertura |

O banco existente nunca é apagado ou recriado automaticamente. Os testes de ciclo de vida usam o backend headless do Slint; a validação visual de monitores e janelas é manual.

# Visão da arquitetura

Esta página descreve a arquitetura vigente do OpenPresenter Minimal. Ela deve ser atualizada sempre que a estrutura, o fluxo de inicialização ou as dependências mudarem.

## Workspace

```text
crates/
├── op-core/    # domínio puro: constantes, AppMeta, SessionInfo, Theme
├── op-db/      # SQLite, PRAGMAs, migrações e persistência da sessão
└── op-admin/   # binário open-presenter: sessão, Slint Material e janela
```

`op-core` não conhece Slint nem SQLite. `op-db` encapsula `rusqlite` e não expõe APIs de UI. `op-admin` é o único crate que conhece Slint e resolve o diretório de dados.

Direção de dependências:

```text
op-admin → op-db → op-core
```

## Fluxo de inicialização

1. `op-admin` resolve o diretório de dados: `OPENPRESENTER_DATA_DIR` ou `BaseDirs::data_local_dir()/OpenPresenter`.
2. Cria o diretório, abre `open-presenter.db`, aplica os PRAGMAs e roda as migrações.
3. Lê o registro anterior de `app_meta`, se existir.
4. Grava a sessão atual: preserva `first_run_at` e as preferências, atualiza `last_run_at`, `app_version` e `updated_at`.
5. Monta `SessionView` com versão, schema, caminho do banco, primeira execução e última execução anterior.
6. Constrói e exibe a janela Slint.
7. Se qualquer etapa de banco falhar, a janela abre em estado degradado com o erro e o caminho tentado; o app não fecha silenciosamente.

## Banco de dados

- Arquivo: `open-presenter.db`.
- PRAGMAs: `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`.
- Schema v1: tabela singleton `app_meta` (`id = 1`) com `app_version`, `first_run_at`, `last_run_at`, `theme`, `locale` e `updated_at`.
- Datas em RFC 3339 UTC com milissegundos.
- Migrações em `crates/op-db/migrations/`, embutidas com `include_str!` e controladas por `PRAGMA user_version`.

## Versões

- Versão do app: SemVer em `[workspace.package] version`; os crates herdam e a UI lê `op_core::VERSION` (origem: `CARGO_PKG_VERSION`).
- Versão do schema: independente, controlada por `PRAGMA user_version` e pelas migrações numeradas.
- Migrações já aplicadas são imutáveis; mudanças criam um novo arquivo numerado.

## Estado degradado

`op-db` expõe erros tipados de abertura, migração, leitura e gravação. `op-admin` converte o erro em uma mensagem curta para a interface e registra o detalhe em `stderr`. A falha não impede a janela de abrir e não apaga nem recria o banco existente.

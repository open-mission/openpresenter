# OpenPresenter Minimal

Aplicativo desktop de projeção para igrejas e eventos. Esta é a fundação técnica da versão **0.1.0**: uma janela nativa em Slint Material que apresenta o aplicativo e o estado do banco SQLite local.

## Stack

- Rust 2024 (MSRV 1.95)
- Slint 1.18.1 com Material Components 1.1.0 vendorizado
- SQLite via `rusqlite` (`bundled`) e `rusqlite_migration`

## Requisitos

- Rust 1.95 ou superior (validado com 1.96.0)
- Ferramentas de build da plataforma exigidas pelo Slint (Xcode Command Line Tools no macOS)

## Como executar

```bash
cargo run -p op-admin
```

## Qualidade

```bash
cargo fmt --all
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Dados

O banco `open-presenter.db` fica no diretório local de dados do sistema:

- macOS: `~/Library/Application Support/OpenPresenter`
- Windows: `%LOCALAPPDATA%\OpenPresenter`
- Linux: `~/.local/share/OpenPresenter`

Para desenvolvimento, use `OPENPRESENTER_DATA_DIR` para apontar para outro diretório:

```bash
OPENPRESENTER_DATA_DIR=/tmp/openpresenter-dev cargo run -p op-admin
```

O banco local, os arquivos `-wal`/`-shm` e os dados de usuário nunca são versionados.

## Estrutura

```text
crates/
├── op-core/    # tipos e constantes de domínio, sem UI ou SQLite
├── op-db/      # SQLite, PRAGMAs e migrações
└── op-admin/   # janela Slint Material e carregamento da sessão
assets/branding/ # SVGs originais da marca
vendor/          # Material Components vendorizado
docs/            # specs, planos, arquitetura e decisões
```

A direção de dependências é `op-admin` → `op-db` → `op-core`.

## Versionamento

- A versão do aplicativo segue SemVer e é definida apenas em `[workspace.package] version` no `Cargo.toml` raiz.
- A versão do schema é independente e controlada por `PRAGMA user_version`; cada mudança de schema cria uma nova migração numerada em `crates/op-db/migrations/`.

## Documentação

O índice completo está em [`docs/README.md`](docs/README.md). A política de nome e identidade visual está em [`TRADEMARK.md`](TRADEMARK.md).

## Licença

Código distribuído sob **GPL-3.0-only** — veja [`LICENSE`](LICENSE). A licença não concede direitos sobre o nome ou a identidade visual do OpenPresenter; veja [`TRADEMARK.md`](TRADEMARK.md).

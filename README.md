# OpenPresenter Minimal

Aplicativo desktop de projeção para igrejas e eventos. Esta é a versão **0.2.0**: um painel administrativo em Slint Material com duas saídas independentes — **Public** e **Stage** — configuráveis por monitor, além do estado do banco SQLite local.

## Stack

- Rust 2024 (MSRV 1.95)
- Slint 1.18.1 com Material Components 1.1.0 vendorizado
- `display-info` 0.5 para enumeração de monitores
- SQLite via `rusqlite` (`bundled`) e `rusqlite_migration`

## Requisitos

- Rust 1.95 ou superior (validado com 1.96.0)
- Ferramentas de build da plataforma exigidas pelo Slint (Xcode Command Line Tools no macOS)

## Como executar

```bash
cargo run -p op-admin
```

## Saídas Public e Stage

- **Public** — saída visível ao público. Vem ativa, com monitor **Automático** e modo **Automático**; nesse modo, abre em tela cheia quando o monitor não é o principal e em janela quando é o principal.
- **Stage** — retorno para o palco e o apresentador. É opcional, vem desativado, com monitor **Principal** e modo **Janela**, e mostra relógio e data por extenso.

Configure as saídas em **☰ → Configurações… → Saídas**:

- ativar ou desativar cada saída;
- escolher o monitor e o modo: **Automático**, **Janela**, **Maximizada** ou **Tela cheia**;
- reabrir uma saída habilitada que foi oculta com **Abrir agora**.

As mudanças são aplicadas imediatamente e persistidas no banco. As saídas habilitadas reabrem na inicialização no monitor salvo; se ele não estiver disponível, a saída usa o principal e mostra um aviso, sem apagar a preferência. Fechar uma saída pelo X apenas a oculta nesta sessão e mantém `enabled`; desativar fecha a janela e persiste a mudança. Com um único monitor, as saídas abrem como janelas normais. Não há detecção de hot-plug nesta versão.

Nesta versão as saídas exibem **placeholders de marca**, não conteúdo de apresentação. A identidade do monitor é *best-effort* entre plataformas; quando ela muda, vale o fallback para o principal.

| Saída | Ativa | Monitor | Modo |
|-------|-------|---------|------|
| Public | sim | Automático | Automático |
| Stage | não | Principal | Janela |

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

O banco usa o **schema v2**, com as tabelas `app_meta` e `output_config`. A migração v1 → v2 é automática e preserva o histórico de execução.

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
└── op-admin/   # painel Slint, monitores, saídas e Configurações
assets/branding/ # SVGs originais da marca
vendor/          # Material Components vendorizado
docs/            # specs, planos, arquitetura e decisões
```

A direção de dependências é `op-admin` → `op-db` → `op-core`.

## Versionamento

- A versão do aplicativo segue SemVer e é definida apenas em `[workspace.package] version` no `Cargo.toml` raiz.
- A versão do schema é independente e controlada por `PRAGMA user_version`; cada mudança de schema cria uma nova migração numerada em `crates/op-db/migrations/`. A versão atual é 2.

## Documentação

O índice completo está em [`docs/README.md`](docs/README.md). A política de nome e identidade visual está em [`TRADEMARK.md`](TRADEMARK.md).

## Licença

Código distribuído sob **GPL-3.0-only** — veja [`LICENSE`](LICENSE). A licença não concede direitos sobre o nome ou a identidade visual do OpenPresenter; veja [`TRADEMARK.md`](TRADEMARK.md).

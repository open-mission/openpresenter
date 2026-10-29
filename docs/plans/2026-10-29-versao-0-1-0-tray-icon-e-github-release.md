# Plano de Execução: Versão 0.1.0 — Executável Autocontido com Tray Icon, Frontend Embutido e CI/CD Multiplataforma

## Etapas de Execução

### Etapa 1: Atualização das Versões do Workspace para 0.1.0
1. Atualizar `Cargo.toml` raiz para `version = "0.1.0"`.
2. Atualizar `frontend/package.json` para `"version": "0.1.0"`.

### Etapa 2: Adição de Dependências Rust para Tray, Menu e Frontend Embutido
1. Adicionar em `Cargo.toml` (dependências do workspace):
   - `tray-icon = "0.26"`
   - `muda = "0.21"`
   - `tao = "0.37"`
   - `open = "5.3"`
   - `rust-embed = "8.5"`
   - `image = { version = "0.25", default-features = false, features = ["png"] }`
2. Adicionar as dependências no crate `crates/op-admin/Cargo.toml`.

### Etapa 3: Frontend Embutido no Binário (`static_files.rs`)
1. Implementar struct `EmbeddedAssets` com `#[derive(RustEmbed)]` apontando para `frontend/dist`.
2. Atualizar `static_fallback_handler` para servir do `EmbeddedAssets` com prioridade ou fallback em dev.
3. Testar compilação e entrega de arquivos.

### Etapa 4: Módulo de Tray Icon, Menu Nativo e Diálogo de Porta
1. Criar `crates/op-admin/src/tray.rs`:
   - Construir o menu nativo com `muda` (Admin, Public, Stage, Porta, Sair).
   - Manipulador de eventos de menu com `open::that(...)`.
   - Diálogo nativo multiplataforma para alterar porta (`osascript` no macOS, `powershell` no Windows, `zenity`/`kdialog` no Linux).
   - Gerar ícone RGBA da marca a partir de bytes PNG da marca.
2. Atualizar `crates/op-admin/src/main.rs`:
   - Suporte à flag `--headless` e variável `OPENPRESENTER_HEADLESS=1`.
   - Ler porta de `PORT` -> `db.get_setting("server_port")` -> `18080`.
   - Iniciar o runtime Tokio em thread de background com o servidor Axum.
   - Executar o event loop do `tao` na thread principal para sustentar o `TrayIcon`.

### Etapa 5: Workflow de CI/CD do GitHub Actions (`.github/workflows/release.yml`)
1. Criar `.github/workflows/release.yml` com matrix para `windows-latest`, `macos-latest` e `ubuntu-latest`.
2. Gerar executáveis autocontidos e compactar em `.zip` e `.tar.gz`.
3. Adicionar step para anexar os artefatos na release do GitHub usando `softprops/action-gh-release@v2`.

### Etapa 6: Documentação, Changelog e Quality Gates
1. Atualizar `docs/architecture/overview.md`.
2. Atualizar `CHANGELOG.md` fechando a versão `[0.1.0] - 2026-10-29`.
3. Atualizar `docs/README.md`.
4. Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `bun run build`.

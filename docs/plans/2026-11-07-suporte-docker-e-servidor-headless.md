# Plano de Implementação — Suporte a Docker e Execução em Servidor Headless sem System Tray

**Data:** 2026-11-07  
**Status:** Em execução  
**Spec de referência:** [`docs/specs/2026-11-07-suporte-docker-e-servidor-headless.md`](../specs/2026-11-07-suporte-docker-e-servidor-headless.md)  
**ADR de referência:** [`docs/decisions/0015-suporte-docker-e-servidor-headless-sem-system-tray.md`](../decisions/0015-suporte-docker-e-servidor-headless-sem-system-tray.md)

---

## 1. Etapas de Execução

### Etapa 1: Decisão Arquitetural (ADR 0015) e Índices de Documentação
- [x] Criar o ADR 0015 em `docs/decisions/0015-suporte-docker-e-servidor-headless-sem-system-tray.md`.
- [x] Atualizar `docs/decisions/README.md` com o link para o ADR 0015.
- [x] Atualizar `docs/README.md` com a nova seção contendo links da spec, plano e ADR.

### Etapa 2: Desacoplamento da Feature `tray` no `op-admin`
- [x] Atualizar `crates/op-admin/Cargo.toml`:
  - Declarar `[features]` com `default = ["tray"]` e `tray = ["dep:muda", "dep:open", "dep:tao", "dep:tray-icon"]`.
  - Marcar `muda`, `open`, `tao`, `tray-icon` como opcionais.
- [x] Criar `crates/op-admin/src/process.rs` contendo `restart_application()` isolada de dependências gráficas.
- [x] Re-exportar `restart_application` em `tray.rs` para retrocompatibilidade.
- [x] Atualizar `crates/op-admin/src/routes.rs` para utilizar `crate::process::restart_application()`.
- [x] Atualizar `crates/op-admin/src/main.rs`:
  - Condicionalizar `mod tray;`, imports de `tao`, `muda`, `tray_icon` e `UserEvent` com `#[cfg(feature = "tray")]`.
  - Implementar função de shutdown gracioso `wait_for_shutdown_signal()` com escuta a `Ctrl+C` e `SIGTERM` em Unix.
  - Ramificar `main()` para modo headless puro quando a feature `tray` não estiver ativa.

### Etapa 3: Criação dos Artefatos Docker
- [x] Criar `.dockerignore` excluindo diretórios de compilação, dependências locais e dados locais.
- [x] Criar `Dockerfile` multi-stage:
  - Estágio 1: compilação do frontend Svelte (Node 22 Alpine / `npm run build`).
  - Estágio 2: compilação do backend Rust headless (Rust Slim Bookworm estável / `cargo build --release --locked --no-default-features`).
  - Estágio 3: imagem de runtime Debian Bookworm Slim com usuário não-root, diretório `/data`, healthcheck e entrypoint.
- [x] Criar `docker-compose.yml` pronto para uso com volume `/data` e portas configuradas.

### Etapa 4: Validação dos Portões de Qualidade e Documentação
- [x] Executar `cargo fmt --all`.
- [x] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Executar `cargo clippy --package op-admin --no-default-features --all-targets -- -D warnings`.
- [x] Executar `cargo test --workspace`.
- [x] Executar `cargo test --package op-admin --no-default-features`.
- [x] Atualizar `docs/architecture/overview.md`.
- [x] Atualizar `README.md` com seção de instruções de execução via Docker e Docker Compose.
- [x] Atualizar `CHANGELOG.md` na seção `[Unreleased]`.
- [x] Atualizar `TODO.md` e realizar commits organizados no Git.

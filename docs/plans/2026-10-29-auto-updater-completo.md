# Plano de Implementação: Sistema de Atualização Automática Completa (Auto-Updater)

## Etapas

1. **Configuração de Dependências no Workspace:**
   - Adicionar `tar = "0.4"`, `zip = { version = "2.2", default-features = false, features = ["deflate"] }`, e `semver = "1.0"` ao `Cargo.toml` raiz e ao `crates/op-admin/Cargo.toml`.

2. **Módulo Backend do Auto-Updater (`crates/op-admin/src/updater.rs`):**
   - Estruturas de dados: `ReleaseInfo`, `UpdateCheckResponse`, `UpdateStatus`, `UpdateProgress`.
   - Função `check_github_update()`: consulta `https://api.github.com/repos/open-mission/openpresenter/releases/latest`, faz parse com `serde_json`, compara tags SemVer e mapeia o asset correto para `OS` e `ARCH`.
   - Função `download_and_apply_update()`: baixa os bytes com progresso percentual, extrai o binário e aplica a substituição atômica no caminho de `std::env::current_exe()`.
   - Suporte multiplataforma para substituição do binário (Unix `chmod` + rename; Windows `.old` swap).

3. **Integração com AppState e Rotas Axum (`crates/op-admin`):**
   - Estado compartilhado em `AppState`: `updater_state: Arc<RwLock<UpdaterState>>`.
   - Rotas em `crates/op-admin/src/routes.rs`:
     - `GET /api/updater/check`
     - `POST /api/updater/install`
     - `GET /api/updater/status`

4. **Integração no Frontend Svelte:**
   - Adicionar cliente de updater em `frontend/src/lib/updater.ts`.
   - Adicionar checagem automática ao iniciar a sessão e no modal de Configurações (aba "Sobre" ou "Atualizações").
   - Exibir badge indicador no header quando houver atualização disponível.
   - Criar interface com exibição das notas de versão (Markdown/Changelog), botão de atualização, progresso visual e botão de reinício.

5. **Testes e Portões de Qualidade:**
   - Testes unitários para verificação de versão semver e correspondência de assets.
   - Testes de integração para as rotas da API `/api/updater/*`.
   - Validação com `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `cd frontend && bun run build`.

# Fundação Técnica do OpenPresenter Minimal — Plano de Implementação

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Entregar a base v0.1.0 do OpenPresenter Minimal: workspace Rust, janela Slint Material branded, SQLite inicializado com o estado do app e documentação organizada.

**Architecture:** Workspace com três crates: `op-core` (domínio puro), `op-db` (SQLite + migrações) e `op-admin` (UI Slint). O admin resolve o diretório de dados, abre o banco, registra a sessão e monta a tela inicial. Falhas de banco não impedem a janela de abrir; a UI mostra um estado degradado.

**Tech Stack:** Rust 2024 (MSRV 1.95), Slint 1.18.1, Material Components 1.1.0 vendorizado, rusqlite 0.40 (`bundled`), rusqlite_migration 2.6, chrono, directories 6, GPL-3.0-only.

**Spec:** `docs/specs/2026-10-28-fundacao-tecnica.md`

## Global Constraints

- Versão do app: `0.1.0`, definida apenas em `[workspace.package] version`.
- Versão do schema: `1`, controlada por `PRAGMA user_version`; independente da versão do app.
- Slint `1.18.1`; Material `1.1.0` vendorizado em `vendor/material-1.1.0/`, carregado via `build.rs` com `with_library_paths`.
- Rust edition 2024, MSRV `1.95`.
- SQLite local, `rusqlite` com feature `bundled`; PRAGMAs `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`.
- Diretório de dados: `OPENPRESENTER_DATA_DIR` ou `BaseDirs::data_local_dir()/OpenPresenter`; banco `open-presenter.db`.
- Licença `GPL-3.0-only`; nome e identidade visual protegidos por `TRADEMARK.md`.
- Documentação e UI em pt-BR; código e identificadores em inglês.
- Todo texto de interface usa os valores da spec: “OpenPresenter”, “Projeção para igrejas e eventos”, “A fundação está pronta.”, “Esta é a primeira execução”, “Banco indisponível”, “Conectado”.
- Nenhum banco local, `-wal`, `-shm`, segredo ou dado de usuário é versionado.

## Review Focus

- **Banco existente com data inválida ou schema mais novo:** o app deve abrir em estado degradado, sem pânico e sem apagar o banco. Teste em Task 4 (timestamp inválido e `user_version` futuro).
- **Diretório de dados ausente, somente leitura ou ocupado por arquivo:** a janela deve abrir mostrando o caminho tentado e a mensagem de erro. Teste em Task 5.
- **Segunda execução:** `first_run_at` não pode ser sobrescrito; a tela deve mostrar a execução anterior como “Última execução”. Testes em Task 4 e Task 5.
- **Tema do sistema alternando em tempo real:** superfícies, textos e wordmark devem permanecer legíveis, sem contraste invertido. Verificação visual em Task 6.
- **Caminho de banco muito longo:** deve quebrar em várias linhas e não empurrar o restante do layout para fora da janela. Verificação visual em Task 6.

---

### Task 1: Bootstrap do workspace, licença, documentação e assets

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `LICENSE`
- Create: `TRADEMARK.md`
- Create: `README.md`
- Create: `CHANGELOG.md`
- Create: `AGENTS.md`
- Create: `GEMINI.md`
- Create: `crates/op-core/Cargo.toml`
- Create: `crates/op-core/src/lib.rs`
- Create: `crates/op-db/Cargo.toml`
- Create: `crates/op-db/src/lib.rs`
- Create: `crates/op-admin/Cargo.toml`
- Create: `crates/op-admin/src/main.rs`
- Create: `assets/branding/README.md`
- Copy: `assets/branding/*.svg`
- Create: `vendor/README.md`
- Extract: `vendor/material-1.1.0/`
- Create: `docs/README.md`
- Create: `docs/architecture/overview.md`
- Create: `docs/decisions/README.md`
- Create: `docs/decisions/0001-workspace-rusqlite-slint.md`
- Create: `docs/decisions/0002-licenca-gpl-3.0.md`

**Interfaces:**
- Consumes: nothing.
- Produces: workspace root with members `op-core`, `op-db`, `op-admin`; `cargo check --workspace` starting point.

- [ ] **Step 1: Create the workspace manifest and crate stubs**

`Cargo.toml` must declare:
- `[workspace] members = ["crates/op-core", "crates/op-db", "crates/op-admin"]`, `resolver = "3"`.
- `[workspace.package] version = "0.1.0"`, `edition = "2024"`, `rust-version = "1.95"`, `license = "GPL-3.0-only"`.
- `[workspace.dependencies]` with `anyhow = "1.0"`, `chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }`, `directories = "6.0"`, `op-core = { path = "crates/op-core" }`, `op-db = { path = "crates/op-db" }`, `rusqlite = { version = "0.40", features = ["bundled"] }`, `rusqlite_migration = "2.6"`, `slint = "1.18.1"`, `slint-build = "1.18.1"`, `tempfile = "3.0"`, `thiserror = "2.0"`.

Each crate manifest inherits `version`, `edition`, `rust-version`, `license`, sets `publish = false`, and uses workspace dependencies:
- `op-core`: `chrono`.
- `op-db`: `chrono`, `op-core`, `rusqlite`, `rusqlite_migration`, `thiserror`; dev-dependency `tempfile`.
- `op-admin`: `anyhow`, `chrono`, `directories`, `op-core`, `op-db`, `slint`; build-dependency `slint-build`; dev-dependency `tempfile` (added in Task 5 if not already present).

`op-admin` defines `[[bin]] name = "open-presenter"` and path `src/main.rs`. Create empty `lib.rs` files and `fn main() {}` in `op-admin/src/main.rs` so the workspace compiles. Create `.gitignore` with `/target`, `*.db`, `*.db-wal`, `*.db-shm`, `.DS_Store`, and `.env`.

- [ ] **Step 2: Add the GPL-3.0-only license and trademark policy**

Run:
```bash
curl -fsSL https://www.gnu.org/licenses/gpl-3.0.txt -o LICENSE
head -1 LICENSE
```
Expected: `                    GNU GENERAL PUBLIC LICENSE`.

Write `TRADEMARK.md` stating: the code is GPL-3.0-only; the license does not grant rights to the “OpenPresenter” name, logo, or visual identity; forks and derivative distributions must use their own name and branding and must not imply official endorsement. Link it from `README.md`.

- [ ] **Step 3: Copy branding assets and vendor Material**

Run:
```bash
mkdir -p assets/branding
cp /Users/claudio/cafg-second-brain/Projects/OpenPresenter/media-assets/*.svg assets/branding/
curl -fsSL https://material.slint.dev/zip/material-1.1.0.zip -o /tmp/material-1.1.0.zip
mkdir -p vendor
unzip -q /tmp/material-1.1.0.zip -d vendor
test -f vendor/material-1.1.0/material.slint
ls assets/branding/*.svg | wc -l
```
Expected: `test` succeeds and the count is `8`.

`assets/branding/README.md` records the source path, copy date `2026-10-28`, the eight file names, usage rules (wordmark claro/escuro, app icon, no recoloring/distortion), and “não editar os SVGs originais”.

`vendor/README.md` records: Slint Material Components `1.1.0`, source URL `https://material.slint.dev/zip/material-1.1.0.zip`, extraction date `2026-10-28`, unmodified contents, license note (Slint tri-license; project uses GPL-3.0-only), and upgrade procedure (replace the directory, rebuild, run tests, update this README).

- [ ] **Step 4: Write the root project documentation**

Write `README.md` with: what the app is, current version 0.1.0, stack, requirements, commands (`cargo run -p op-admin`, `cargo test --workspace`, `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`), data directory and override, project structure, links to `docs/README.md` and `TRADEMARK.md`, versioning policy (SemVer for app, numbered migrations for schema). Write `CHANGELOG.md` with the Keep a Changelog header, `## [Unreleased]`, and `## [0.1.0] - 2026-10-28` with the foundation items.

- [ ] **Step 5: Write the documentation index, architecture and ADRs**

Write `docs/README.md` as the index linking the spec, plan, architecture, decisions, root README, branding README and vendor README; include the documentation rules summary. Write `docs/architecture/overview.md` per spec §7 — workspace tree, dependency direction, startup flow, data directory, schema v1, versioning, error state. Write `docs/decisions/README.md` with the ADR convention (`NNNN-slug.md`, context/decision/consequences, accepted ADRs immutable). Write `docs/decisions/0001-workspace-rusqlite-slint.md` and `docs/decisions/0002-licenca-gpl-3.0.md` with context, decision, alternatives and consequences.

- [ ] **Step 6: Write the permanent agent instructions**

Write `AGENTS.md` and `GEMINI.md` with the intro “Estas instruções são permanentes e se aplicam a todas as sessões”, the docs index pointer, the quality-gate commands, and the 13 rules from spec §8.2 verbatim in substance. Add a note that `AGENTS.md` is canonical and the two files must stay synchronized.

- [ ] **Step 7: Verify the workspace and docs links**

Run:
```bash
cargo check --workspace
rg -n "docs/specs|docs/plans|docs/architecture|docs/decisions|AGENTS.md|GEMINI.md|TRADEMARK.md" README.md docs/README.md
git status --short
```
Expected: `cargo check` succeeds; `rg` finds the links; `git status` shows only intended files. `Cargo.lock` is generated and must be added.

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml Cargo.lock .gitignore LICENSE TRADEMARK.md README.md CHANGELOG.md AGENTS.md GEMINI.md assets vendor crates docs
git commit -m "chore: bootstrap workspace, licensing, and documentation"
```

---

### Task 2: Tipos de domínio em `op-core`

**Files:**
- Modify: `crates/op-core/src/lib.rs`
- Test: inline `#[cfg(test)]` module in `crates/op-core/src/lib.rs`

**Interfaces:**
- Consumes: `chrono`.
- Produces:
  - `pub const APP_NAME: &str = "OpenPresenter";`
  - `pub const TAGLINE: &str = "Projeção para igrejas e eventos";`
  - `pub const STACK: &str = "Rust · Slint · SQLite";`
  - `pub const VERSION: &str = env!("CARGO_PKG_VERSION");`
  - `pub const DEFAULT_LOCALE: &str = "pt-BR";`
  - `pub enum Theme { System, Light, Dark }` with `Default` = `System`, `pub const fn as_str(self) -> &'static str`, `pub fn from_db_str(value: &str) -> Option<Self>`, `pub const fn label_pt_br(self) -> &'static str`.
  - `pub struct AppMeta { pub app_version: String, pub first_run_at: DateTime<Utc>, pub last_run_at: DateTime<Utc>, pub theme: Theme, pub locale: String, pub updated_at: DateTime<Utc> }`
  - `pub struct SessionInfo { pub current: AppMeta, pub previous_last_run_at: Option<DateTime<Utc>> }`

- [ ] **Step 1: Write the failing tests**

Add tests asserting:
```rust
#[test]
fn theme_round_trips_through_db_string() {
    for theme in [Theme::System, Theme::Light, Theme::Dark] {
        assert_eq!(Theme::from_db_str(theme.as_str()), Some(theme));
    }
    assert_eq!(Theme::from_db_str("invalid"), None);
}

#[test]
fn theme_labels_are_portuguese() {
    assert_eq!(Theme::System.label_pt_br(), "Sistema");
    assert_eq!(Theme::Light.label_pt_br(), "Claro");
    assert_eq!(Theme::Dark.label_pt_br(), "Escuro");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-core`
Expected: FAIL because `Theme` and the constants do not exist yet.

- [ ] **Step 3: Implement the domain types**

Implement the exact interfaces above. `Theme::as_str` returns `"system"`, `"light"`, `"dark"`; `from_db_str` accepts only those strings; `Default` is `System`. Derive `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` for `Theme`; `Debug`, `Clone`, `PartialEq`, `Eq` for `AppMeta` and `SessionInfo`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-core`
Expected: PASS with 2 tests.

- [ ] **Step 5: Commit**

```bash
git add crates/op-core
git commit -m "feat(core): add app metadata and theme types"
```

---

### Task 3: Banco, schema e PRAGMAs em `op-db`

**Files:**
- Create: `crates/op-db/migrations/0001_initial.sql`
- Modify: `crates/op-db/src/lib.rs`
- Test: inline `#[cfg(test)]` module in `crates/op-db/src/lib.rs`

**Interfaces:**
- Consumes: `op-core` (not yet used in this task), `rusqlite`, `rusqlite_migration`, `thiserror`.
- Produces:
  - `pub struct Database { conn: Connection, path: PathBuf }`
  - `pub fn open(path: &Path) -> Result<Self, DbError>`
  - `pub fn schema_version(&self) -> Result<u32, DbError>`
  - `pub fn path(&self) -> &Path`
  - `pub enum DbError { Open(#[source] rusqlite::Error), Migration(#[from] rusqlite_migration::Error), Read(#[source] rusqlite::Error) }` with `thiserror` messages in pt-BR.

- [ ] **Step 1: Write the failing tests**

Add tests:
```rust
#[test]
fn open_creates_schema_and_applies_pragmas() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("open-presenter.db");
    let db = Database::open(&path).unwrap();

    assert_eq!(db.schema_version().unwrap(), 1);
    let table_count: i64 = db.conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'app_meta'",
        [], |row| row.get(0),
    ).unwrap();
    assert_eq!(table_count, 1);
    let journal_mode: String = db.conn.query_row("PRAGMA journal_mode", [], |row| row.get(0)).unwrap();
    assert_eq!(journal_mode.to_lowercase(), "wal");
    let foreign_keys: i64 = db.conn.query_row("PRAGMA foreign_keys", [], |row| row.get(0)).unwrap();
    assert_eq!(foreign_keys, 1);
    let busy_timeout: i64 = db.conn.query_row("PRAGMA busy_timeout", [], |row| row.get(0)).unwrap();
    assert_eq!(busy_timeout, 5000);
    let synchronous: i64 = db.conn.query_row("PRAGMA synchronous", [], |row| row.get(0)).unwrap();
    assert_eq!(synchronous, 1);
}

#[test]
fn open_fails_when_parent_directory_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing").join("db.sqlite");
    assert!(matches!(Database::open(&path), Err(DbError::Open(_))));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-db`
Expected: FAIL because `Database`, `DbError` and the migration do not exist.

- [ ] **Step 3: Implement schema, open and PRAGMAs**

Create `0001_initial.sql` exactly as spec §7.5. In `lib.rs`, open the connection, run `PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL; PRAGMA foreign_keys = ON;`, call `conn.busy_timeout(Duration::from_secs(5))`, and apply `Migrations::new(vec![M::up(include_str!("../migrations/0001_initial.sql"))])` with `to_latest`. `schema_version` reads `PRAGMA user_version` and returns `u32`. Map open/pragma errors to `DbError::Open`, migration errors to `DbError::Migration`, and user-version read errors to `DbError::Read`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-db`
Expected: PASS with 2 tests.

- [ ] **Step 5: Commit**

```bash
git add crates/op-db
git commit -m "feat(db): create sqlite schema and migrations"
```

---

### Task 4: Persistência do estado inicial em `op-db`

**Files:**
- Modify: `crates/op-db/src/lib.rs`
- Test: inline `#[cfg(test)]` module in `crates/op-db/src/lib.rs`

**Interfaces:**
- Consumes: `op_core::{AppMeta, SessionInfo, Theme, DEFAULT_LOCALE}`, `chrono::{DateTime, Utc}`.
- Produces:
  - `pub fn begin_session(&self, app_version: &str, now: DateTime<Utc>) -> Result<SessionInfo, DbError>`
  - `fn load_meta(&self) -> Result<Option<AppMeta>, DbError>`
  - Additional `DbError` variants: `Write(#[source] rusqlite::Error)`, `InvalidTimestamp(String)`.

- [ ] **Step 1: Write the failing tests**

Add tests using `chrono::TimeZone` and `Utc.with_ymd_and_hms(...)`:
```rust
#[test]
fn first_session_creates_meta() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
    let now = ts(2026, 10, 28, 10, 0);
    let info = db.begin_session("0.1.0", now).unwrap();

    assert_eq!(info.previous_last_run_at, None);
    assert_eq!(info.current.app_version, "0.1.0");
    assert_eq!(info.current.first_run_at, now);
    assert_eq!(info.current.last_run_at, now);
    assert_eq!(info.current.theme, Theme::System);
    assert_eq!(info.current.locale, "pt-BR");
    assert!(db.load_meta().unwrap().is_some());
}

#[test]
fn second_session_preserves_first_run_and_reports_previous() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("open-presenter.db");
    let first = ts(2026, 10, 28, 10, 0);
    {
        let db = Database::open(&path).unwrap();
        db.begin_session("0.1.0", first).unwrap();
        db.conn.execute("UPDATE app_meta SET theme = 'dark', locale = 'en-US' WHERE id = 1", []).unwrap();
    }
    let second = ts(2026, 10, 29, 18, 30);
    let db = Database::open(&path).unwrap();
    let info = db.begin_session("0.2.0", second).unwrap();

    assert_eq!(info.previous_last_run_at, Some(first));
    assert_eq!(info.current.first_run_at, first);
    assert_eq!(info.current.last_run_at, second);
    assert_eq!(info.current.app_version, "0.2.0");
    assert_eq!(info.current.theme, Theme::Dark);
    assert_eq!(info.current.locale, "en-US");
    let count: i64 = db.conn.query_row("SELECT COUNT(*) FROM app_meta", [], |row| row.get(0)).unwrap();
    assert_eq!(count, 1);
}

#[test]
fn invalid_timestamp_is_reported_without_panicking() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
    db.conn.execute(
        "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
         VALUES (1, '0.1.0', 'not-a-date', 'not-a-date', 'system', 'pt-BR', 'not-a-date')",
        [],
    ).unwrap();
    assert!(matches!(db.begin_session("0.1.0", ts(2026, 10, 28, 10, 0)), Err(DbError::InvalidTimestamp(_))));
}

#[test]
fn newer_schema_is_reported_as_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("open-presenter.db");
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.pragma_update(None, "user_version", 99).unwrap();
    }
    assert!(matches!(Database::open(&path), Err(DbError::Migration(_))));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-db`
Expected: FAIL because `begin_session`, `load_meta`, `InvalidTimestamp` and `Write` do not exist.

- [ ] **Step 3: Implement session persistence**

Implement `load_meta` reading the singleton row and parsing RFC 3339 timestamps. Implement `begin_session` to read the previous meta, preserve `first_run_at` and preferences, compute the current meta with `last_run_at = now` and `updated_at = now`, upsert the singleton row with `ON CONFLICT(id) DO UPDATE` (never updating `first_run_at`), and return `SessionInfo { current, previous_last_run_at }`. Format timestamps with `to_rfc3339_opts(SecondsFormat::Millis, true)`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-db`
Expected: PASS with 6 tests.

- [ ] **Step 5: Commit**

```bash
git add crates/op-db
git commit -m "feat(db): persist startup state"
```

---

### Task 5: Carregamento da sessão em `op-admin`

**Files:**
- Create: `crates/op-admin/src/session.rs`
- Modify: `crates/op-admin/src/main.rs`
- Modify: `crates/op-admin/Cargo.toml` (add `tempfile` dev-dependency)
- Test: inline `#[cfg(test)]` module in `crates/op-admin/src/session.rs`

**Interfaces:**
- Consumes: `op_core::{APP_NAME, TAGLINE, STACK, VERSION, DEFAULT_LOCALE, Theme}`, `op_db::Database`, `chrono::{DateTime, Local, Utc}`, `directories::BaseDirs`, `anyhow`.
- Produces:
  - `pub struct SessionView { pub app_version: String, pub tagline: String, pub stack: String, pub db_path: String, pub db_ready: bool, pub db_error: String, pub schema_version: i32, pub theme_label: String, pub locale: String, pub first_run_at: String, pub last_run_at: String, pub is_first_run: bool }`
  - `pub fn load_session() -> SessionView`
  - `fn load_session_from_dir(dir: &Path) -> SessionView`
  - `fn resolve_data_dir() -> anyhow::Result<PathBuf>`
  - `fn format_timestamp(dt: &DateTime<Utc>) -> String`

- [ ] **Step 1: Write the failing tests**

Add tests:
```rust
#[test]
fn first_session_view_is_ready_and_reports_version() {
    let dir = tempfile::tempdir().unwrap();
    let view = load_session_from_dir(dir.path());
    assert!(view.db_ready);
    assert_eq!(view.app_version, op_core::VERSION);
    assert_eq!(view.schema_version, 1);
    assert!(view.is_first_run);
    assert!(view.db_path.ends_with("open-presenter.db"));
    assert!(!view.first_run_at.is_empty());
}

#[test]
fn second_session_view_keeps_first_run_and_fills_last_run() {
    let dir = tempfile::tempdir().unwrap();
    let first = load_session_from_dir(dir.path());
    let second = load_session_from_dir(dir.path());
    assert!(second.db_ready);
    assert!(!second.is_first_run);
    assert_eq!(second.first_run_at, first.first_run_at);
    assert!(!second.last_run_at.is_empty());
}

#[test]
fn unwritable_data_dir_returns_degraded_view() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("not-a-dir");
    std::fs::write(&file, "x").unwrap();
    let view = load_session_from_dir(&file);
    assert!(!view.db_ready);
    assert!(!view.db_error.is_empty());
    assert_eq!(view.schema_version, 0);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-admin`
Expected: FAIL because `session.rs` and `load_session_from_dir` do not exist.

- [ ] **Step 3: Implement session loading**

Implement `resolve_data_dir` (env override, else `BaseDirs::data_local_dir().join("OpenPresenter")`), `load_session` (maps resolution errors to a degraded view), and `load_session_from_dir` (creates the directory, opens `open-presenter.db`, reads schema, calls `begin_session(VERSION, Utc::now())`, maps the result into `SessionView`). On any error, return a degraded view with the attempted path, `db_ready = false`, `schema_version = 0`, the error text, and default theme/locale. Format timestamps as `%d/%m/%Y às %H:%M` in local time. In `main.rs`, wire `mod session;` and temporarily print the session fields so the crate has a runnable binary before the UI task.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-admin`
Expected: PASS with 3 tests. Also run `cargo run -p op-admin` and confirm it prints a ready session without panicking.

- [ ] **Step 5: Commit**

```bash
git add crates/op-admin
git commit -m "feat(admin): load startup session state"
```

---

### Task 6: Janela Slint Material branded

**Files:**
- Create: `crates/op-admin/build.rs`
- Create: `crates/op-admin/ui/theme.slint`
- Create: `crates/op-admin/ui/main.slint`
- Create: `crates/op-admin/src/ui.rs`
- Modify: `crates/op-admin/src/main.rs`

**Interfaces:**
- Consumes: `SessionView` from Task 5, `op_core` constants, Slint/Material.
- Produces: `pub fn run() -> anyhow::Result<()>` in `ui.rs`; `MainWindow` generated from `ui/main.slint` with the properties listed below.

- [ ] **Step 1: Configure the Slint build**

Create `build.rs` using `slint_build::CompilerConfiguration::new().with_library_paths(HashMap::from([("material".to_string(), manifest_dir.join("../../vendor/material-1.1.0/material.slint"))]))` and `slint_build::compile_with_config("ui/main.slint", config).unwrap()`. Use `CARGO_MANIFEST_DIR` to resolve the path.

- [ ] **Step 2: Define the brand tokens**

Create `ui/theme.slint` exporting `global Brand` with `dark` from `Palette.color_scheme == ColorScheme.dark` and these values:
- page: dark `#0B0D12`, light `#F4F4F2`
- surface: dark `#12151C`, light `#FFFFFF`
- surface-muted: dark `#1A1E27`, light `#F0F1F3`
- ink: dark `#F5F6F8`, light `#111318`
- muted: dark `#A1A7B3`, light `#6B7280`
- subtle: dark `#6B7280`, light `#9CA3AF`
- outline: dark `#252A34`, light `#E3E5E9`
- success: dark `#4ADE80`, light `#15803D`
- warning: dark `#FBBF24`, light `#B45309`
- radius: `18px`

- [ ] **Step 3: Build the welcome window**

Create `ui/main.slint` with `export component MainWindow inherits MaterialWindow`, imports `{ Brand } from "theme.slint";` and `{ MaterialWindow } from "@material";`. Declare the properties `app-name`, `app-tagline`, `app-version`, `db-path`, `db-ready`, `db-error`, `schema-version`, `first-run-at`, `last-run-at`, `is-first-run`, `theme-label`, `locale`, `stack` with the types and defaults from the spec. Set `title: app-name`, `icon: @image-url("../../../assets/branding/open-presenter-app-icon.svg")`, `width: 960px`, `height: 640px`, `background: Brand.page`.

Layout, top to bottom:
1. Header: `HorizontalLayout` with spacing 16px: wordmark image 210×68 (dark variant when `Brand.dark`, light variant otherwise), a transparent `Rectangle { horizontal-stretch: 1; }` spacer, and a version pill with `"v" + app-version`.
2. Hero: “A fundação está pronta.” at 38px/700, tagline at 17px, and the supporting sentence “Interface nativa, banco local e versões organizadas. O próximo passo é construir a experiência de projeção.” at 14px with `wrap: word-wrap`.
3. Two cards side by side (equal `horizontal-stretch`), each `Rectangle` with `border-radius: Brand.radius`, 1px outline, `Brand.surface` background and 24px padding:
   - “APLICAÇÃO” / “Sobre”: rows Versão, Tema (`theme-label + " · " + (Brand.dark ? "escuro" : "claro")`), Idioma, Stack.
   - “BANCO DE DADOS” / “Estado local”: status dot + “Conectado”/“Banco indisponível”, rows Arquivo (`wrap: char-wrap`), Schema (`db-ready ? "v" + schema-version : "—"`), Primeira execução (`db-ready ? first-run-at : "—"`), Última execução (`db-ready ? (is-first-run ? "Esta é a primeira execução" : last-run-at) : "—"`). If `!db-ready`, show `db-error` in `Brand.warning` with `wrap: word-wrap`.
4. Footer: `HorizontalLayout` with “Dados salvos localmente” on the left, a spacer, and “OpenPresenter v” + app-version on the right, both in `Brand.subtle` at 11px.

Use `InfoRow` helper components for label/value rows (label 13px muted, value 13px ink, right-aligned, `char-wrap` for the path). Use semantic colors only for status; the rest monochrome.

- [ ] **Step 4: Wire the UI to the session data**

Create `ui.rs` with `slint::include_modules!();`, `use slint::ComponentHandle;`, and `pub fn run() -> anyhow::Result<()>`. Load `SessionView`, construct `MainWindow`, set every property, print the error to `stderr` when `!db_ready`, call `ui.run()?`, return `Ok(())`. Modify `main.rs` to `mod session; mod ui; fn main() -> anyhow::Result<()> { ui::run() }`.

- [ ] **Step 5: Build and inspect the window**

Run:
```bash
cargo build -p op-admin
OPENPRESENTER_DATA_DIR=/tmp/openpresenter-ui-test cargo run -p op-admin
```
Expected: a 960×640 window titled “OpenPresenter” with the wordmark, version pill, hero, two cards, footer, and a connected database. Verify light and dark system appearance; verify the wordmark switches and all text remains readable. Close the app.

- [ ] **Step 6: Verify persistence and degraded state manually**

Run the app twice with `OPENPRESENTER_DATA_DIR=/tmp/openpresenter-ui-test`; the second run must show the previous execution as “Última execução”. Then create a regular file at `/tmp/openpresenter-broken` and run with `OPENPRESENTER_DATA_DIR=/tmp/openpresenter-broken`; the window must open with “Banco indisponível”, the error message and the attempted path.

- [ ] **Step 7: Commit**

```bash
git add crates/op-admin
git commit -m "feat(admin): add branded slint welcome window"
```

---

### Task 7: Documentação final e verificação da v0.1.0

**Files:**
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `docs/README.md`
- Modify: `docs/architecture/overview.md`
- Modify: `AGENTS.md`
- Modify: `GEMINI.md`
- Test: workspace commands + manual acceptance

**Interfaces:**
- Consumes: all previous tasks.
- Produces: v0.1.0 foundation verified and documented.

- [ ] **Step 1: Update docs with the implemented reality**

Read the final code and confirm `docs/architecture/overview.md` matches the actual workspace, dependency graph, data path, schema and startup flow. Update `README.md` with the real commands and version. Update `CHANGELOG.md` if any item changed during implementation. Ensure `docs/README.md` links every doc, including this plan. Re-read `AGENTS.md` and `GEMINI.md` and confirm they contain the 13 rules, the docs index pointer and the quality gates; keep them synchronized.

- [ ] **Step 2: Run formatting and lint gates**

Run:
```bash
cargo fmt --all
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```
Expected: no diffs and no clippy warnings.

- [ ] **Step 3: Run the full test suite**

Run:
```bash
cargo test --workspace
```
Expected: all tests pass; at least 2 in `op-core`, 6 in `op-db`, and 3 in `op-admin`.

- [ ] **Step 4: Run the end-to-end acceptance checks**

Run:
```bash
rm -rf /tmp/openpresenter-acceptance
OPENPRESENTER_DATA_DIR=/tmp/openpresenter-acceptance cargo run -p op-admin
sqlite3 "/tmp/openpresenter-acceptance/open-presenter.db" "SELECT app_version, first_run_at, last_run_at, theme, locale FROM app_meta;"
```
Expected: window opens with version 0.1.0, schema 1, “Esta é a primeira execução”; the query returns one row with `0.1.0`, `system`, `pt-BR`. Run the app a second time and confirm first_run_at is unchanged and the previous execution appears. Verify the degraded state with a file path as in Task 6.

- [ ] **Step 5: Verify documentation and repository state**

Run:
```bash
test -f LICENSE && test -f TRADEMARK.md && test -f AGENTS.md && test -f GEMINI.md
test -f vendor/material-1.1.0/material.slint
test -f assets/branding/open-presenter-app-icon.svg
rg -n "0\\.1\\.0" Cargo.toml CHANGELOG.md README.md
git status --short
```
Expected: all files exist, version 0.1.0 is consistent, and only intended changes remain.

- [ ] **Step 6: Commit**

```bash
git add README.md CHANGELOG.md docs AGENTS.md GEMINI.md crates
git commit -m "docs: finalize foundation documentation for v0.1.0"
```

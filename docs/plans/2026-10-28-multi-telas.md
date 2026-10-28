# Multi-telas: saídas Public e Stage — Plano de Implementação

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Entregar a v0.2.0 do OpenPresenter Minimal com duas saídas independentes — Public e Stage — configuráveis por monitor, restauradas na inicialização e persistidas em SQLite.

**Architecture:** O domínio ganha `OutputConfig`/`OutputsConfig`; o banco ganha a migração v2 com `output_config`; o admin ganha enumeração de monitores via `display-info`, um controlador de janelas Slint (Public/Stage) e uma janela de Configurações independente. O painel continua sendo a janela principal; as saídas são janelas separadas mantidas por handles fortes e o loop de eventos é único.

**Tech Stack:** Rust 2024 (MSRV 1.95), Slint 1.18.1, Material Components 1.1.0 vendorizado, `display-info` 0.5, `rusqlite` 0.40 (`bundled`), `rusqlite_migration` 2.6, chrono, GPL-3.0-only.

**Spec:** `docs/specs/2026-10-28-multi-telas.md`

## Global Constraints

- Versão do app: `0.2.0`, definida apenas em `[workspace.package] version`.
- Versão do schema: `2`, controlada por `PRAGMA user_version`; independente da versão do app.
- Slint `1.18.1`; Material `1.1.0` vendorizado em `vendor/material-1.1.0/`.
- `display-info` `0.5` (resolvido em `0.5.10`) apenas em `op-admin`.
- Rust edition 2024, MSRV `1.95`.
- Direção de dependências: `op-admin` → `op-db` → `op-core`; `op-core` não conhece UI nem SQLite.
- Não editar `crates/op-db/migrations/0001_initial.sql`; a mudança de schema é a nova migração `0002_output_config.sql`.
- UI e documentação em pt-BR; código e identificadores em inglês.
- Public: ativo, monitor `Auto`, modo `Auto`. Stage: inativo, monitor `Primary`, modo `Windowed`.
- Saídas habilitadas reabrem na inicialização; fechar pelo X apenas oculta e mantém `enabled`.
- Configurações em janela independente, aberta pelo menu interno do app; não há menu nativo do sistema.
- Sem conteúdo real de apresentação, sem hot-plug, sem NDI/OBS, sem atalhos globais nesta versão.
- Testes de ciclo de vida usam o backend headless `i-slint-backend-testing` `1.18.1` (dev-dependency).
- Cada task termina com `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` verdes antes do commit.

## Review Focus

- **Monitor salvo desconectado:** a saída deve cair para o principal com aviso, sem apagar a preferência salva. Teste em Task 4 e verificação manual em Task 6/7.
- **Enumeração de monitores vazia ou com erro:** o app não pode entrar em pânico nem impedir o painel; as saídas abrem como janelas na posição padrão. Testes em Task 4 e Task 5.
- **Banco indisponível ou falha ao salvar:** banco indisponível não abre saídas, mostra o erro e desabilita as Configurações; falha de gravação mostra uma faixa separada, mantém a configuração em memória e não desabilita os controles. Testes em Task 2, Task 3 e Task 5.
- **Fechamento do app:** o painel principal encerra o loop com `quit_event_loop()` e descarta as janelas; fechar uma saída pelo X não encerra nada. Teste em Task 5 e verificação manual em Task 6/7.
- **Escala e coordenadas:** tamanhos lógicos convertidos para físicos, janela centralizada e limitada ao monitor; Retina não pode deixar a janela fora da tela ou com metade do tamanho. Testes em Task 4.
- **Fechar pelo X versus desativar:** X oculta sem mudar `enabled`; desativar fecha e persiste `false`; reiniciar restaura apenas as habilitadas. Teste em Task 5 e verificação manual em Task 6/7.

---

### Task 1: Tipos de domínio das saídas e versão 0.2.0

**Files:**
- Modify: `Cargo.toml` (versão do workspace)
- Modify: `crates/op-admin/ui/main.slint` (versão padrão exibida)
- Modify: `crates/op-core/src/lib.rs`
- Test: inline `#[cfg(test)]` em `crates/op-core/src/lib.rs`

**Interfaces:**
- Consumes: `chrono` (já existente).
- Produces:
  - `pub enum OutputKind { Public, Stage }`, `Copy`, com `pub const ALL: [OutputKind; 2] = [Public, Stage]`, `pub fn as_str(&self) -> &'static str`, `pub fn from_db_str(value: &str) -> Option<Self>`, `pub fn label_pt_br(&self) -> &'static str` e `pub fn description_pt_br(&self) -> &'static str`.
  - `pub enum OutputMode { Auto, Windowed, Maximized, Fullscreen }`, `Copy + Default = Auto`, com `pub const ALL: [OutputMode; 4] = [Auto, Windowed, Maximized, Fullscreen]`, `pub fn as_str(&self) -> &'static str`, `pub fn from_db_str(value: &str) -> Option<Self>` e `pub fn label_pt_br(&self) -> &'static str`.
  - `pub enum MonitorSelection { Auto, Primary, Display { id: String, name: String } }`, `Default = Auto`, com `pub fn as_db_key(&self) -> String`, `pub fn from_db_key(key: &str, name: String) -> Self` e `pub fn label_pt_br(&self) -> &str`.
  - `pub struct OutputConfig { pub kind: OutputKind, pub enabled: bool, pub monitor: MonitorSelection, pub mode: OutputMode }` e `pub fn default_for(kind: OutputKind) -> Self`.
  - `pub struct OutputsConfig { pub public: OutputConfig, pub stage: OutputConfig }` com `Default`, `pub fn get(&self, kind: OutputKind) -> &OutputConfig`, `pub fn get_mut(&mut self, kind: OutputKind) -> &mut OutputConfig` e `pub fn iter(&self) -> impl Iterator<Item = &OutputConfig>`.

- [ ] **Step 1: Write the failing domain tests**

Adicione ao módulo de testes de `op-core`:

```rust
#[test]
fn output_kinds_round_trip_and_have_portuguese_labels() {
    for kind in OutputKind::ALL {
        assert_eq!(OutputKind::from_db_str(kind.as_str()), Some(kind));
    }
    assert_eq!(OutputKind::from_db_str("invalid"), None);
    assert_eq!(OutputKind::Public.label_pt_br(), "Public");
    assert_eq!(OutputKind::Stage.label_pt_br(), "Stage");
    assert_eq!(OutputKind::Public.description_pt_br(), "Saída visível ao público.");
    assert_eq!(OutputKind::Stage.description_pt_br(), "Retorno para o palco e o apresentador.");
}

#[test]
fn output_modes_round_trip_and_have_portuguese_labels() {
    for mode in OutputMode::ALL {
        assert_eq!(OutputMode::from_db_str(mode.as_str()), Some(mode));
    }
    assert_eq!(OutputMode::from_db_str("invalid"), None);
    assert_eq!(OutputMode::Auto.label_pt_br(), "Automático");
    assert_eq!(OutputMode::Windowed.label_pt_br(), "Janela");
    assert_eq!(OutputMode::Maximized.label_pt_br(), "Maximizada");
    assert_eq!(OutputMode::Fullscreen.label_pt_br(), "Tela cheia");
    assert_eq!(OutputMode::default(), OutputMode::Auto);
}

#[test]
fn monitor_selection_round_trips_through_db_key() {
    let display = MonitorSelection::Display {
        id: "42".into(),
        name: "DELL U2720Q".into(),
    };
    assert_eq!(display.as_db_key(), "id:42");
    assert_eq!(
        MonitorSelection::from_db_key("id:42", "DELL U2720Q".into()),
        display
    );
    assert_eq!(
        MonitorSelection::from_db_key("auto", String::new()),
        MonitorSelection::Auto
    );
    assert_eq!(
        MonitorSelection::from_db_key("primary", String::new()),
        MonitorSelection::Primary
    );
    assert_eq!(
        MonitorSelection::from_db_key("bogus", String::new()),
        MonitorSelection::Auto
    );
    assert_eq!(MonitorSelection::Auto.label_pt_br(), "Automático");
    assert_eq!(MonitorSelection::Primary.label_pt_br(), "Principal");
    assert_eq!(display.label_pt_br(), "DELL U2720Q");
}

#[test]
fn output_defaults_match_the_spec() {
    let outputs = OutputsConfig::default();

    assert!(outputs.public.enabled);
    assert_eq!(outputs.public.monitor, MonitorSelection::Auto);
    assert_eq!(outputs.public.mode, OutputMode::Auto);

    assert!(!outputs.stage.enabled);
    assert_eq!(outputs.stage.monitor, MonitorSelection::Primary);
    assert_eq!(outputs.stage.mode, OutputMode::Windowed);

    assert_eq!(outputs.get(OutputKind::Public).kind, OutputKind::Public);
    assert_eq!(outputs.get(OutputKind::Stage).kind, OutputKind::Stage);
    assert_eq!(outputs.iter().count(), 2);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-core`
Expected: FAIL because `OutputKind`, `OutputMode`, `MonitorSelection`, `OutputConfig` and `OutputsConfig` do not exist.

- [ ] **Step 3: Implement the domain types**

Implemente as interfaces acima em `crates/op-core/src/lib.rs`:

- `OutputKind::as_str` → `"public"`/`"stage"`; `from_db_str` aceita apenas esses valores; `ALL` tem exatamente os dois elementos na ordem Public, Stage.
- `OutputMode::as_str` → `"auto"`/`"windowed"`/`"maximized"`/`"fullscreen"`; `from_db_str` aceita apenas esses valores; `ALL` tem exatamente os quatro elementos na ordem Auto, Windowed, Maximized, Fullscreen.
- `MonitorSelection::as_db_key` → `"auto"`, `"primary"` ou `"id:<id>"`; `from_db_key` aceita `"auto"`, `"primary"`, `"id:<id>"` e devolve `Auto` para qualquer outro valor.
- `OutputConfig::default_for(kind)` aplica os padrões da tabela do spec.
- `OutputsConfig::iter` devolve primeiro Public, depois Stage; `get` e `get_mut` usam `match` sobre `OutputKind`, sem `unwrap`.
- Derive `Debug`, `Clone`, `PartialEq`, `Eq`; `Copy` para `OutputKind` e `OutputMode`; `OutputMode` e `MonitorSelection` implementam `Default`.
- Altere `[workspace.package] version` para `"0.2.0"` e o default `app-version` de `ui/main.slint` para `"0.2.0"`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-core`
Expected: PASS com os testes novos e os existentes de tema.

- [ ] **Step 5: Run the quality gates and commit**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
git add Cargo.toml Cargo.lock crates/op-core crates/op-admin/ui/main.slint
git commit -m "feat(core): add output configuration types"
```

---

### Task 2: Migração v2 e persistência da configuração de saídas

**Files:**
- Create: `crates/op-db/migrations/0002_output_config.sql`
- Modify: `crates/op-db/src/lib.rs`
- Modify: `crates/op-admin/src/session.rs` (expectativa de schema nos testes)
- Test: inline `#[cfg(test)]` em `crates/op-db/src/lib.rs`

**Interfaces:**
- Consumes: `op_core::{OutputConfig, OutputKind, OutputMode, MonitorSelection, OutputsConfig}`, `chrono::{DateTime, Utc}`.
- Produces:
  - `pub fn load_outputs(&self) -> Result<OutputsConfig, DbError>`
  - `pub fn save_output(&self, config: &OutputConfig, now: DateTime<Utc>) -> Result<(), DbError>`

- [ ] **Step 1: Write the failing database tests**

Adicione/substitua testes em `op-db`:

```rust
#[test]
fn open_creates_schema_v2_with_output_config() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

    assert_eq!(db.schema_version().unwrap(), 2);
    let table_count: i64 = db.conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'output_config'",
        [], |row| row.get(0),
    ).unwrap();
    assert_eq!(table_count, 1);
}

#[test]
fn migrates_v1_database_to_v2_preserving_app_meta() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("open-presenter.db");
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(include_str!("../migrations/0001_initial.sql")).unwrap();
        conn.pragma_update(None, "user_version", 1).unwrap();
        conn.execute(
            "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
             VALUES (1, '0.1.0', '2026-10-28T10:00:00.000Z', '2026-10-28T10:00:00.000Z', 'system', 'pt-BR', '2026-10-28T10:00:00.000Z')",
            [],
        ).unwrap();
    }

    let db = Database::open(&path).unwrap();
    assert_eq!(db.schema_version().unwrap(), 2);
    let meta = db.load_meta().unwrap().unwrap();
    assert_eq!(meta.first_run_at, ts(2026, 10, 28, 10, 0));
    let count: i64 = db.conn.query_row("SELECT COUNT(*) FROM output_config", [], |row| row.get(0)).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn load_outputs_without_rows_returns_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
    assert_eq!(db.load_outputs().unwrap(), OutputsConfig::default());
}

#[test]
fn saves_and_loads_public_and_stage_outputs() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
    let public = OutputConfig {
        kind: OutputKind::Public,
        enabled: false,
        monitor: MonitorSelection::Display { id: "7".into(), name: "DELL".into() },
        mode: OutputMode::Fullscreen,
    };
    let stage = OutputConfig {
        kind: OutputKind::Stage,
        enabled: true,
        monitor: MonitorSelection::Primary,
        mode: OutputMode::Windowed,
    };

    db.save_output(&public, ts(2026, 10, 28, 11, 0)).unwrap();
    db.save_output(&stage, ts(2026, 10, 28, 11, 1)).unwrap();

    let loaded = db.load_outputs().unwrap();
    assert_eq!(loaded.public, public);
    assert_eq!(loaded.stage, stage);
}

#[test]
fn updating_one_output_preserves_the_other() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
    let mut public = OutputConfig::default_for(OutputKind::Public);
    let stage = OutputConfig::default_for(OutputKind::Stage);
    db.save_output(&public, ts(2026, 10, 28, 11, 0)).unwrap();
    db.save_output(&stage, ts(2026, 10, 28, 11, 0)).unwrap();

    public.mode = OutputMode::Maximized;
    db.save_output(&public, ts(2026, 10, 28, 11, 5)).unwrap();

    let loaded = db.load_outputs().unwrap();
    assert_eq!(loaded.public.mode, OutputMode::Maximized);
    assert_eq!(loaded.stage, stage);
}

#[test]
fn invalid_stored_mode_and_monitor_fall_back_to_auto() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
    db.conn.pragma_update(None, "ignore_check_constraints", true).unwrap();
    db.conn.execute(
        "INSERT INTO output_config (output, enabled, monitor_key, monitor_name, mode, updated_at)
         VALUES ('public', 1, 'bogus', '', 'invalid', '2026-10-28T11:00:00.000Z')",
        [],
    ).unwrap();

    let loaded = db.load_outputs().unwrap();
    assert_eq!(loaded.public.monitor, MonitorSelection::Auto);
    assert_eq!(loaded.public.mode, OutputMode::Auto);
}
```

Atualize também:

- `open_creates_schema_and_applies_pragmas`: espere `schema_version() == 2`.
- `crates/op-admin/src/session.rs`, teste `first_session_view_is_ready_and_reports_version`: espere `schema_version == 2`.

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-db`
Expected: FAIL porque a migração v2 e os métodos `load_outputs`/`save_output` não existem.

- [ ] **Step 3: Implement the migration and persistence API**

Crie `0002_output_config.sql` exatamente como o spec §9.2. Em `op-db`:

- Inclua a migração no vetor: `M::up(include_str!("../migrations/0001_initial.sql"))` e `M::up(include_str!("../migrations/0002_output_config.sql"))`.
- `load_outputs` consulta `output, enabled, monitor_key, monitor_name, mode`; começa em `OutputsConfig::default()`, ignora linhas com `output` desconhecido, lê `enabled != 0`, converte `monitor_key`/`monitor_name` com `MonitorSelection::from_db_key` e `mode` com `OutputMode::from_db_str(...).unwrap_or_default()`.
- `save_output` faz upsert por `output` com `ON CONFLICT(output) DO UPDATE`, grava `monitor_key = config.monitor.as_db_key()`, `monitor_name` apenas para `Display`, `mode = config.mode.as_str()` e `updated_at` em RFC 3339 UTC com milissegundos.
- Reutilize `DbError::Read`/`DbError::Write`; não apague dados existentes.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-db`
Expected: PASS, incluindo a migração v1 → v2 e os padrões.

- [ ] **Step 5: Run the quality gates and commit**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
git add crates/op-db crates/op-admin/src/session.rs
git commit -m "feat(db): persist output configuration"
```

---

### Task 3: Sessão mantém a conexão com o banco viva

**Files:**
- Modify: `crates/op-admin/src/session.rs`
- Modify: `crates/op-admin/src/ui.rs`
- Test: inline `#[cfg(test)]` em `crates/op-admin/src/session.rs`

**Interfaces:**
- Consumes: `op_db::Database` e `SessionView`.
- Produces:
  - `pub struct LoadedSession { pub view: SessionView, pub db: Option<Database> }`
  - `pub fn load_session() -> LoadedSession`
  - `fn load_session_from_dir(dir: &Path) -> LoadedSession`

- [ ] **Step 1: Write the failing session tests**

Atualize os testes existentes para usar `.view` e acrescente:

```rust
#[test]
fn successful_session_returns_live_database() {
    let dir = tempfile::tempdir().unwrap();
    let loaded = load_session_from_dir(dir.path());
    assert!(loaded.view.db_ready);
    assert!(loaded.db.is_some());
    assert_eq!(loaded.view.schema_version, 2);
}
```

Atualize o teste existente `unwritable_data_dir_returns_degraded_view` para usar `.view` e acrescente `assert!(loaded.db.is_none());`. Não crie um segundo teste para o mesmo diretório inválido.

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-admin`
Expected: FAIL porque `load_session_from_dir` ainda devolve `SessionView`.

- [ ] **Step 3: Return the database handle from session loading**

Altere `load_session_from_dir` para devolver `LoadedSession`:

- Em sucesso, `Database` vai em `db: Some(db)`.
- Em qualquer falha, `db: None` e a visão degradada continua como hoje.
- `load_session` devolve `LoadedSession` e usa `db: None` quando o diretório não resolve.
- Em `ui.rs`, use `let loaded = load_session(); let _db = loaded.db; let view = loaded.view;` — o binding `_db` mantém a conexão viva e evita `dead_code` até a Task 5 consumir o handle.
- Mantenha `first_run_at`, `last_run_at` e o estado degradado intactos.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-admin`
Expected: PASS; os testes antigos continuam verdes com `.view`.

- [ ] **Step 5: Run the quality gates and commit**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
git add crates/op-admin
git commit -m "refactor(admin): keep database handle in session"
```

---

### Task 4: Monitores, resolução e geometria das janelas

**Files:**
- Modify: `Cargo.toml` (workspace dependency `display-info`)
- Modify: `crates/op-admin/Cargo.toml` (dependência `display-info`)
- Modify: `crates/op-admin/src/main.rs` (`mod monitors;`)
- Create: `crates/op-admin/src/monitors.rs`
- Test: inline `#[cfg(test)]` em `crates/op-admin/src/monitors.rs`

**Interfaces:**
- Consumes: `display_info::DisplayInfo`, `op_core::{MonitorSelection, OutputKind, OutputMode}`.
- Produces:
  - `pub struct MonitorInfo { pub key: String, pub name: String, pub label: String, pub x: i32, pub y: i32, pub width: u32, pub height: u32, pub scale_factor: f32, pub is_primary: bool, pub is_builtin: bool }`
  - `pub fn list() -> Result<Vec<MonitorInfo>, String>`
  - `#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub(crate) enum CoordinateSpace { Logical, Physical }` e `pub(crate) fn coordinate_space() -> CoordinateSpace`.
  - `pub(crate) fn normalize_rect(x: i32, y: i32, width: u32, height: u32, scale_factor: f32, space: CoordinateSpace) -> (i32, i32, u32, u32)`.
  - `pub enum EffectiveMode { Windowed, Maximized, Fullscreen }`
  - `pub enum MonitorWarning { NoMonitors, MissingDisplay { fallback_primary: bool } }` com `message_pt_br()`.
  - `pub struct Resolution { pub monitor: Option<MonitorInfo>, pub mode: EffectiveMode, pub warning: Option<MonitorWarning> }`
  - `pub fn resolve(kind: OutputKind, selection: &MonitorSelection, mode: OutputMode, monitors: &[MonitorInfo]) -> Resolution`
  - `pub fn effective_mode(kind: OutputKind, mode: OutputMode, monitor: Option<&MonitorInfo>) -> EffectiveMode`
  - `pub struct WindowPlacement { pub x: i32, pub y: i32, pub width: u32, pub height: u32 }`
  - `pub fn placement(monitor: &MonitorInfo, logical_width: f32, logical_height: f32, cascade_logical: f32) -> WindowPlacement`
  - `pub fn logical_size(kind: OutputKind) -> (f32, f32)` e `pub fn cascade_offset(kind: OutputKind) -> f32`.

- [ ] **Step 1: Write the failing monitor tests**

Adicione testes puros em `monitors.rs`:

```rust
fn monitor(key: &str, name: &str, x: i32, primary: bool, builtin: bool) -> MonitorInfo {
    MonitorInfo {
        key: key.into(),
        name: name.into(),
        label: format!("{name} — 1920×1080{}", if primary { " (principal)" } else { "" }),
        x,
        y: 0,
        width: 1920,
        height: 1080,
        scale_factor: 1.0,
        is_primary: primary,
        is_builtin: builtin,
    }
}

#[test]
fn normalizes_logical_coordinates_to_physical() {
    assert_eq!(
        normalize_rect(-1280, 0, 1280, 800, 2.0, CoordinateSpace::Logical),
        (-2560, 0, 2560, 1600)
    );
    assert_eq!(
        normalize_rect(-1280, 0, 1280, 800, 2.0, CoordinateSpace::Physical),
        (-1280, 0, 1280, 800)
    );
}

#[test]
fn auto_prefers_external_then_primary() {
    let builtin = monitor("1", "Built-in", 0, true, true);
    let external = monitor("2", "DELL", 1920, false, false);
    let monitors = vec![builtin.clone(), external.clone()];

    let resolution = resolve(
        OutputKind::Public,
        &MonitorSelection::Auto,
        OutputMode::Auto,
        &monitors,
    );
    assert_eq!(resolution.monitor.unwrap().key, "2");
    assert_eq!(resolution.mode, EffectiveMode::Fullscreen);
    assert!(resolution.warning.is_none());

    let resolution = resolve(
        OutputKind::Stage,
        &MonitorSelection::Auto,
        OutputMode::Auto,
        &monitors,
    );
    assert_eq!(resolution.monitor.unwrap().key, "2");
    assert_eq!(resolution.mode, EffectiveMode::Windowed);
}

#[test]
fn primary_and_display_selections_resolve() {
    let builtin = monitor("1", "Built-in", 0, true, true);
    let external = monitor("2", "DELL", 1920, false, false);
    let monitors = vec![builtin.clone(), external.clone()];

    let resolution = resolve(
        OutputKind::Public,
        &MonitorSelection::Primary,
        OutputMode::Auto,
        &monitors,
    );
    assert_eq!(resolution.monitor.unwrap().key, "1");
    assert_eq!(resolution.mode, EffectiveMode::Windowed);

    let by_id = resolve(
        OutputKind::Public,
        &MonitorSelection::Display { id: "2".into(), name: "DELL".into() },
        OutputMode::Windowed,
        &monitors,
    );
    assert_eq!(by_id.monitor.unwrap().key, "2");

    let by_name = resolve(
        OutputKind::Public,
        &MonitorSelection::Display { id: "missing".into(), name: "DELL".into() },
        OutputMode::Windowed,
        &monitors,
    );
    assert_eq!(by_name.monitor.unwrap().key, "2");
    assert!(by_name.warning.is_none());
}

#[test]
fn missing_display_falls_back_to_primary_with_warning() {
    let builtin = monitor("1", "Built-in", 0, true, true);
    let monitors = vec![builtin];

    let resolution = resolve(
        OutputKind::Public,
        &MonitorSelection::Display { id: "missing".into(), name: "Missing".into() },
        OutputMode::Windowed,
        &monitors,
    );
    assert_eq!(resolution.monitor.unwrap().key, "1");
    assert_eq!(
        resolution.warning.unwrap().message_pt_br(),
        "Monitor indisponível — usando o principal"
    );
}

#[test]
fn missing_display_without_primary_uses_first_monitor_with_warning() {
    let external = monitor("2", "DELL", 1920, false, false);

    let resolution = resolve(
        OutputKind::Public,
        &MonitorSelection::Display { id: "missing".into(), name: "Missing".into() },
        OutputMode::Windowed,
        &[external],
    );
    assert_eq!(resolution.monitor.unwrap().key, "2");
    assert_eq!(
        resolution.warning.unwrap().message_pt_br(),
        "Monitor indisponível — usando o primeiro monitor disponível"
    );
}

#[test]
fn auto_and_primary_fall_back_when_there_is_no_external_or_primary() {
    let builtin = monitor("1", "Built-in", 0, true, true);
    let auto = resolve(
        OutputKind::Public,
        &MonitorSelection::Auto,
        OutputMode::Windowed,
        &[builtin],
    );
    assert_eq!(auto.monitor.unwrap().key, "1");
    assert!(auto.warning.is_none());

    let external = monitor("2", "DELL", 1920, false, false);
    let primary = resolve(
        OutputKind::Public,
        &MonitorSelection::Primary,
        OutputMode::Windowed,
        &[external],
    );
    assert_eq!(primary.monitor.unwrap().key, "2");
    assert!(primary.warning.is_none());
}

#[test]
fn empty_monitor_list_is_windowed_without_panic() {
    let resolution = resolve(
        OutputKind::Public,
        &MonitorSelection::Auto,
        OutputMode::Fullscreen,
        &[],
    );
    assert!(resolution.monitor.is_none());
    assert_eq!(resolution.mode, EffectiveMode::Windowed);
    assert_eq!(
        resolution.warning.unwrap().message_pt_br(),
        "Nenhum monitor detectado — usando a posição padrão"
    );
}

#[test]
fn placement_centers_scales_and_clamps() {
    let retina = MonitorInfo {
        key: "1".into(),
        name: "Retina".into(),
        label: "Retina".into(),
        x: 0,
        y: 0,
        width: 3840,
        height: 2160,
        scale_factor: 2.0,
        is_primary: true,
        is_builtin: true,
    };
    let centered = placement(&retina, 960.0, 540.0, 0.0);
    assert_eq!((centered.x, centered.y, centered.width, centered.height), (960, 540, 1920, 1080));

    let cascaded = placement(&retina, 960.0, 540.0, 64.0);
    assert_eq!((cascaded.x, cascaded.y), (1088, 668));

    let small = MonitorInfo {
        key: "2".into(),
        name: "Small".into(),
        label: "Small".into(),
        x: 0,
        y: 0,
        width: 800,
        height: 600,
        scale_factor: 1.0,
        is_primary: false,
        is_builtin: false,
    };
    let clamped = placement(&small, 960.0, 540.0, 0.0);
    assert_eq!((clamped.x, clamped.y, clamped.width, clamped.height), (0, 30, 800, 540));
}

#[test]
fn placement_uses_the_monitor_origin() {
    let right = MonitorInfo {
        key: "2".into(),
        name: "DELL".into(),
        label: "DELL".into(),
        x: 1920,
        y: 0,
        width: 1920,
        height: 1080,
        scale_factor: 1.0,
        is_primary: false,
        is_builtin: false,
    };
    let centered = placement(&right, 960.0, 540.0, 0.0);
    assert_eq!(
        (centered.x, centered.y, centered.width, centered.height),
        (2400, 270, 960, 540)
    );
}

#[test]
fn placement_guards_invalid_scale_factors() {
    for scale_factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        let broken = MonitorInfo {
            key: "3".into(),
            name: "Broken".into(),
            label: "Broken".into(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            scale_factor,
            is_primary: true,
            is_builtin: false,
        };
        let placed = placement(&broken, 960.0, 540.0, 0.0);
        assert_eq!(
            (placed.x, placed.y, placed.width, placed.height),
            (480, 270, 960, 540)
        );
    }
}

#[test]
fn output_sizes_are_logical_pixels() {
    assert_eq!(logical_size(OutputKind::Public), (960.0, 540.0));
    assert_eq!(logical_size(OutputKind::Stage), (720.0, 480.0));
    assert_eq!(cascade_offset(OutputKind::Public), 64.0);
    assert_eq!(cascade_offset(OutputKind::Stage), 128.0);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p op-admin`
Expected: FAIL porque `monitors.rs` e `display-info` não existem.

- [ ] **Step 3: Add the dependency and implement monitor handling**

- Adicione `display-info = "0.5"` em `[workspace.dependencies]` e `display-info.workspace = true` em `op-admin`.
- Derive `Debug`, `Clone`, `PartialEq` (e `Eq`/`Copy` quando aplicável) em `MonitorInfo`, `EffectiveMode`, `MonitorWarning`, `Resolution` e `WindowPlacement`, para testes e comparações sem `unwrap` desnecessário.
- `list()` chama `DisplayInfo::all()`, converte cada item e ordena por `x`, depois `y`. Em erro, devolve `Err(error.to_string())`.
- `normalize_rect` recebe primitivas (não `DisplayInfo`) para ser testável em qualquer plataforma; `CoordinateSpace` é `pub(crate)` e `coordinate_space()` usa `Physical` no Windows e `Logical` nas demais plataformas. Escalas não finitas, zero ou negativas são tratadas como `1.0`; coordenadas e dimensões são multiplicadas pela escala e arredondadas para o inteiro mais próximo.
- `MonitorInfo.name` usa `friendly_name` quando não vazio, senão `name`; `key = id.to_string()`; `label` combina nome e resolução, com sufixo `(principal)`.
- `resolve`: `Auto` escolhe o primeiro externo (não principal e não builtin), senão principal, senão primeiro; `Primary` escolhe principal, senão primeiro; `Display` tenta chave, depois nome, e em último caso o principal (com `MissingDisplay { fallback_primary: true }`) ou o primeiro (`fallback_primary: false`). Lista vazia devolve `NoMonitors` e modo `Windowed`.
- `MonitorWarning::message_pt_br`: `NoMonitors` → “Nenhum monitor detectado — usando a posição padrão”; `MissingDisplay { fallback_primary: true }` → “Monitor indisponível — usando o principal”; `MissingDisplay { fallback_primary: false }` → “Monitor indisponível — usando o primeiro monitor disponível”.
- `effective_mode`: `Auto` Public em monitor não principal → `Fullscreen`; em principal → `Windowed`; Stage `Auto` → `Windowed`; os modos explícitos mapeiam direto; sem monitor → `Windowed`.
- `placement` converte tamanho e cascata lógicos para físicos pela escala (com a guarda acima), centraliza no retângulo do monitor, soma a cascata e limita origem e tamanho ao monitor, com largura/altura mínimas de 1 px. A cascata só é passada pelo chamador quando o destino é o principal; nos demais casos, `cascade_logical = 0.0`.
- Adicione `mod monitors;` em `main.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p op-admin`
Expected: PASS com os testes de monitores e geometria.

- [ ] **Step 5: Run the quality gates and commit**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
git add Cargo.toml Cargo.lock crates/op-admin
git commit -m "feat(admin): add monitor discovery and geometry"
```

---

### Task 5: Janelas Public/Stage, controlador e abertura na inicialização

**Files:**
- Create: `crates/op-admin/ui/outputs.slint`
- Modify: `crates/op-admin/ui/theme.slint`
- Modify: `crates/op-admin/ui/main.slint` (exportar as saídas)
- Modify: `crates/op-admin/src/main.rs` (`include_modules!`, módulos `outputs`)
- Create: `crates/op-admin/src/outputs.rs`
- Modify: `crates/op-admin/src/ui.rs` (abrir as saídas após o painel)
- Modify: `Cargo.toml` e `crates/op-admin/Cargo.toml` (dev-dependencies `i-slint-backend-testing` e `rusqlite`)
- Test: inline `#[cfg(test)]` em `crates/op-admin/src/outputs.rs`

**Interfaces:**
- Consumes: `monitors::{list, resolve, placement, logical_size, cascade_offset, MonitorInfo, EffectiveMode}`, `op_core::{OutputConfig, OutputKind, OutputMode, MonitorSelection, OutputsConfig}`, `op_db::Database`.
- Produces:
  - Componentes Slint `PublicWindow` e `StageWindow`; `StageWindow` tem `clock-time` e `date-label`.
  - `pub struct OutputsController` com:
    - `new(db: Option<Database>, database_error: Option<String>) -> Self`
    - `refresh_monitors(&mut self)`
    - `set_monitors(&mut self, monitors: Vec<MonitorInfo>)`
    - `start(&mut self)`
    - `apply_config(&mut self)`
    - `set_enabled(&mut self, kind: OutputKind, enabled: bool)`
    - `set_monitor(&mut self, kind: OutputKind, selection: MonitorSelection)`
    - `set_mode(&mut self, kind: OutputKind, mode: OutputMode)`
    - `show_now(&mut self, kind: OutputKind)`
    - `hide(&mut self, kind: OutputKind)`
    - `close_all(&mut self)`
    - `is_visible(&self, kind: OutputKind) -> bool`
    - `status(&self, kind: OutputKind) -> OutputStatus`
    - `selection_for_index(&self, index: i32) -> MonitorSelection`
    - `monitor_index(&self, kind: OutputKind) -> i32`
    - `#[cfg(test)] has_stage_timer(&self) -> bool` — usado apenas nos testes, para evitar `dead_code` no binário.
    - `config(&self) -> &OutputsConfig`
    - `monitors(&self) -> &[MonitorInfo]`
    - `monitor_warning(&self) -> Option<&str>`
    - `database_error(&self) -> Option<&str>`
    - `persistence_error(&self) -> Option<&str>`
  - `pub enum OutputStatus { Disabled, Open, Hidden, Warning(String), Failed }` com `text_pt_br()` e `kind()` (0 neutro, 1 sucesso, 2 aviso, 3 erro).

- [ ] **Step 1: Create the Slint output windows and theme tokens**

Em `ui/theme.slint`, adicione tokens fixos:

```slint
out property <color> output-bg: #0B0D12;
out property <color> output-surface: #12151C;
out property <color> output-ink: #F5F6F8;
out property <color> output-muted: #A1A7B3;
out property <color> output-accent: #4ADE80;
```

Em `ui/outputs.slint`:

- `PublicWindow inherits Window`: título “OpenPresenter — Público”, 960×540, fundo `Brand.output-bg`, ícone do app, wordmark claro centralizado, etiqueta “SAÍDA PÚBLICA” e “Aguardando conteúdo da apresentação”. Sem controles.
- `StageWindow inherits Window`: título “OpenPresenter — Palco”, 720×480, fundo `Brand.output-bg`, `in property <string> clock-time: "00:00:00"`, `in property <string> date-label: ""`, cabeçalho “RETORNO DE PALCO”, relógio grande, data, divisor, “Nenhum evento carregado” e “Aguardando apresentação”. Sem controles.

Em `ui/main.slint`, importe e reexporte os dois componentes para que `slint::include_modules!()` os gere.

- [ ] **Step 2: Move the generated modules and build the components**

Mova `slint::include_modules!();` de `ui.rs` para o topo de `main.rs`; em `ui.rs`, importe `crate::MainWindow`; em `outputs.rs`, importe `crate::{PublicWindow, StageWindow}`. Adicione `mod outputs;` em `main.rs`. Rode:

```bash
cargo build -p op-admin
```

Expected: build succeeds and the generated `PublicWindow`/`StageWindow` types exist.

- [ ] **Step 3: Write the failing controller tests**

Em `outputs.rs`, adicione:

```rust
#[test]
fn output_lifecycle_and_database_failure_are_safe() {
    i_slint_backend_testing::init_integration_test_with_mock_time();

    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
    let mut controller = OutputsController::new(Some(db), None);
    controller.set_monitors(vec![test_monitor()]);
    controller.start();
    controller.start();

    assert!(controller.is_visible(OutputKind::Public));
    assert!(!controller.is_visible(OutputKind::Stage));
    assert!(!controller.has_stage_timer());
    assert_eq!(controller.status(OutputKind::Public).text_pt_br(), "Aberta");
    assert_eq!(controller.status(OutputKind::Stage).text_pt_br(), "Desativada");

    controller.set_enabled(OutputKind::Stage, true);
    assert!(controller.is_visible(OutputKind::Stage));
    assert!(controller.has_stage_timer());

    controller.hide(OutputKind::Public);
    assert!(!controller.is_visible(OutputKind::Public));
    assert!(controller.config().public.enabled);
    assert_eq!(controller.status(OutputKind::Public).text_pt_br(), "Oculta");

    controller.set_monitor(
        OutputKind::Public,
        MonitorSelection::Display { id: "missing".into(), name: "Missing".into() },
    );
    controller.show_now(OutputKind::Public);
    assert!(controller.is_visible(OutputKind::Public));
    assert!(matches!(controller.status(OutputKind::Public), OutputStatus::Warning(_)));

    controller.set_enabled(OutputKind::Public, false);
    assert!(!controller.is_visible(OutputKind::Public));
    assert!(!controller.config().public.enabled);
    assert_eq!(controller.status(OutputKind::Public).text_pt_br(), "Desativada");

    controller.set_enabled(OutputKind::Stage, false);
    assert!(!controller.has_stage_timer());
    controller.close_all();

    let mut degraded = OutputsController::new(None, Some("Banco indisponível".into()));
    degraded.start();
    assert!(!degraded.is_visible(OutputKind::Public));
    assert_eq!(degraded.status(OutputKind::Public).text_pt_br(), "Desativada");
}

#[test]
fn missing_saved_monitor_keeps_the_preference_and_warns() {
    let mut controller = OutputsController::new(None, None);
    controller.set_monitors(vec![test_monitor()]);
    let saved = MonitorSelection::Display { id: "missing".into(), name: "Missing".into() };
    controller.set_monitor(OutputKind::Public, saved.clone());

    assert_eq!(controller.config().public.monitor, saved);
    assert_eq!(controller.monitor_index(OutputKind::Public), 0);
    assert!(matches!(controller.status(OutputKind::Public), OutputStatus::Warning(_)));
}

#[test]
fn selection_for_index_maps_dropdown_entries() {
    let mut controller = OutputsController::new(None, None);
    controller.set_monitors(vec![test_monitor()]);

    assert_eq!(controller.selection_for_index(0), MonitorSelection::Auto);
    assert_eq!(controller.selection_for_index(1), MonitorSelection::Primary);
    assert!(matches!(
        controller.selection_for_index(2),
        MonitorSelection::Display { .. }
    ));
    assert_eq!(controller.selection_for_index(99), MonitorSelection::Auto);
}

#[test]
fn persistence_failure_keeps_in_memory_config() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("open-presenter.db");
    let db = Database::open(&path).unwrap();
    let mut controller = OutputsController::new(Some(db), None);
    controller.set_monitors(vec![test_monitor()]);

    let blocker = rusqlite::Connection::open(&path).unwrap();
    blocker.execute("DROP TABLE output_config", []).unwrap();

    controller.set_mode(OutputKind::Public, OutputMode::Fullscreen);
    assert_eq!(controller.config().public.mode, OutputMode::Fullscreen);
    assert!(controller.persistence_error().is_some());
}

#[test]
fn stage_date_is_in_portuguese() {
    use chrono::TimeZone;
    let date = chrono::Local.with_ymd_and_hms(2026, 10, 28, 9, 30, 0).single().unwrap();
    assert_eq!(format_date_pt_br(&date), "quarta-feira, 28 de outubro de 2026");
}
```

Inclua o helper `test_monitor()` no módulo de testes com `key: "1"`, `name: "Built-in"`, `label: "Built-in — 1920×1080 (principal)"`, `x: 0`, `y: 0`, `width: 1920`, `height: 1080`, `scale_factor: 1.0`, `is_primary: true` e `is_builtin: true`.

Apenas `output_lifecycle_and_database_failure_are_safe` chama `init_integration_test_with_mock_time()` e cria janelas. Os demais testes de `outputs.rs` devem ser puros, sem inicializar backend nem abrir janelas; a inicialização do backend de teste só pode ocorrer uma vez por processo.

- [ ] **Step 4: Run tests to verify they fail**

Run: `cargo test -p op-admin`
Expected: FAIL porque `OutputsController`, `OutputStatus` e os componentes gerados ainda não existem.

- [ ] **Step 5: Implement the controller**

Implemente `OutputsController` mantendo:

- `OutputSlot<W: ComponentHandle>` com `window` e `visible: Rc<Cell<bool>>`; o handle forte vive no controlador. O controlador tem `public: Option<OutputSlot<PublicWindow>>`, `stage: Option<OutputSlot<StageWindow>>`, `failed: [bool; 2]`, `started: bool` e `stage_timer: Option<slint::Timer>`.
- `new`: carrega `OutputsConfig` quando há `Database`; se `load_outputs` falhar, usa os padrões e grava `database_error`; se `db` for `None`, usa os padrões e o erro recebido. `persistence_error` começa vazio.
- `refresh_monitors`: chama `monitors::list()`, passa o resultado para `set_monitors`, define `monitor_warning` global em erro ou lista vazia e limpa o aviso quando a enumeração funciona. Não reposiciona janelas já abertas.
- `set_monitors`: apenas substitui a lista de monitores; usado por `refresh_monitors` e pelos testes.
- `start`: idempotente via `started`; se `database_error` existir não abre nada; abre apenas as saídas com `enabled = true`.
- `open`: resolve monitor/modo; cria a janela; define o relógio do Stage antes de mostrar; registra `on_close_requested` para marcar `visible = false` e devolver `HideWindow`; aplica geometria; chama `show()`; depois aplica fullscreen/maximized. Falha vira `failed[index] = true`, `eprintln!` e janela descartada.
- `apply_config`: recalcula a resolução e reaplica geometria/modo apenas nas janelas visíveis. Antes de reposicionar, chama `set_fullscreen(false)` e `set_maximized(false)`; depois aplica o modo efetivo. `set_monitor` e `set_mode` chamam `apply_config` após persistir; se a janela estiver oculta, nada é reposicionado e a próxima abertura usa a nova configuração.
- `set_enabled`/`set_monitor`/`set_mode` atualizam a memória primeiro, tentam persistir com `save_output(Utc::now())` e aplicam imediatamente mesmo se a gravação falhar. Se o valor recebido já é o atual, retornam sem persistir (isso evita gravações extras quando a UI sincroniza os bindings). Sucesso limpa `persistence_error`; falha grava a mensagem, sem desfazer a mudança em memória. Com `database_error`, as mutações são no-op defensivos.
- `set_enabled(false)` descarta a janela e o timer do Stage, limpa `failed`; `set_enabled(true)` abre a janela se ela não existir. `hide` atualiza o flag e chama `hide()`. `show_now` não abre saída desativada, é no-op com `database_error`; se a janela não existir, chama `open`; se existir, chama `show()`, marca visível e então `apply_config()` para reaplicar a configuração.
- `status`: banco indisponível ou saída desativada → `Disabled`; `failed` → `Failed`; janela visível com aviso de resolução → `Warning`; janela visível → `Open`; janela existente oculta → `Hidden`; sem janela, habilitada e com aviso → `Warning`; habilitada sem janela → `Failed`; senão `Disabled`.
- `selection_for_index`: 0 → `Auto`, 1 → `Primary`, 2+ → monitor correspondente; fora da lista → `Auto`.
- `monitor_index`: `Auto` → 0, `Primary` → 1, `Display` encontrado → 2+posição, ausente → 0.
- O timer do Stage é um `slint::Timer` repetido de 1 s; capture um `slint::Weak<StageWindow>` no callback, atualize `clock-time` com `%H:%M:%S` e `date-label` com `format_date_pt_br`, e pare se o upgrade falhar. Ele é criado quando a janela do Stage abre e descartado quando ela fecha. `#[cfg(test)] has_stage_timer()` reflete a existência dele.
- `format_date_pt_br` usa os nomes portugueses de semana e mês, sem depender de locale do sistema.
- Geometria: com monitor, `Windowed` usa `placement` com cascata só se o monitor for principal; `Maximized`/`Fullscreen` usam o retângulo inteiro do monitor; sem monitor, usa `WindowSize::Logical` e não define posição. Erros de criação/show são registrados em `stderr` e refletidos como `Failed`; nunca causam pânico.
- `OutputStatus::kind`: `Disabled` e `Hidden` → 0, `Open` → 1, `Warning` → 2, `Failed` → 3. `text_pt_br`: `Open` → “Aberta”, `Hidden` → “Oculta”, `Disabled` → “Desativada”, `Failed` → “Falha ao abrir”, `Warning` → a mensagem de `MonitorWarning`.

- [ ] **Step 6: Wire startup in `ui.rs`**

- Use `let LoadedSession { view, db } = load_session();`; passe `db` e a mensagem de erro (`view.db_error` quando `!view.db_ready`) para `OutputsController::new`.
- Mantenha o painel principal como janela principal; chame `ui.show()?`, depois `refresh_monitors()` e `start()` em escopos curtos de `borrow_mut()`, e por fim `slint::run_event_loop()?`. Não use `ui.run()`, que bloquearia antes de abrir as saídas.
- Registre `ui.window().on_close_requested` para chamar `controller.borrow_mut().close_all()`, `slint::quit_event_loop()` e devolver `HideWindow`. Sem `quit_event_loop()`, fechar o painel deixaria o processo vivo por causa das janelas/loop.
- Mantenha o controlador vivo por `Rc<RefCell<OutputsController>>` até depois de `run_event_loop()`; o callback de fechamento usa um clone do `Rc`.

- [ ] **Step 7: Run tests to verify they pass**

Run: `cargo test -p op-admin`
Expected: PASS, incluindo o ciclo de vida headless e a data em português.

- [ ] **Step 8: Run the quality gates and commit**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
git add Cargo.toml Cargo.lock crates/op-admin
git commit -m "feat(admin): open public and stage outputs"
```

---

### Task 6: Janela de Configurações, menu e aplicação imediata

**Files:**
- Create: `crates/op-admin/ui/settings.slint`
- Modify: `crates/op-admin/ui/main.slint` (menu, copy do painel e export de `SettingsWindow`)
- Create: `crates/op-admin/src/settings.rs`
- Modify: `crates/op-admin/src/main.rs` (`mod settings;`)
- Modify: `crates/op-admin/src/ui.rs` (menu, janela e polling de status)
- Test: inline `#[cfg(test)]` em `crates/op-admin/src/settings.rs`

**Interfaces:**
- Consumes: `OutputsController`, `OutputStatus`, `OutputKind`, `OutputMode`, `MonitorSelection`.
- Produces:
  - `SettingsWindow` Slint com propriedades `public-enabled`, `stage-enabled`, `public-monitor-items: [MenuItem]`, `stage-monitor-items: [MenuItem]`, `mode-items: [MenuItem]`, `public-monitor-index`, `stage-monitor-index`, `public-mode-index`, `stage-mode-index`, `public-status`, `stage-status`, `public-status-kind`, `stage-status-kind`, `public-visible`, `stage-visible`, `single-monitor`, `monitor-warning`, `database-error`, `persistence-error`, `controls-enabled`.
  - Callbacks `public_enabled_changed(bool)`, `stage_enabled_changed(bool)`, `public_monitor_selected(int)`, `stage_monitor_selected(int)`, `public_mode_selected(int)`, `stage_mode_selected(int)`, `public_visibility_toggled()`, `stage_visibility_toggled()`, `close_requested()`.
  - `MainWindow` com callback `open-settings()`.
  - `settings::open(controller: Rc<RefCell<OutputsController>>, slot: Rc<RefCell<Option<SettingsWindow>>>) -> Result<(), slint::PlatformError>`
  - `settings::hide(slot: &Rc<RefCell<Option<SettingsWindow>>>)`
  - `settings::refresh_if_open(slot: &Rc<RefCell<Option<SettingsWindow>>>, controller: &OutputsController)` — apenas status/visibilidade.
  - `settings::refresh(window: &SettingsWindow, controller: &OutputsController)` — modelos, índices, estados e avisos.
  - `settings::refresh_status(window: &SettingsWindow, controller: &OutputsController)` — somente estados, visibilidade e erros.
  - `fn mode_index(mode: OutputMode) -> i32`, `fn mode_from_index(index: i32) -> OutputMode` e `fn monitor_items(monitors: &[MonitorInfo]) -> Vec<MenuItem>`.

- [ ] **Step 1: Create the Settings Slint component**

Em `ui/settings.slint`:

- `SettingsWindow inherits MaterialWindow`: título “Configurações — OpenPresenter”, 820×600, mínimo 720×540, fundo `Brand.page`. `public-enabled`, `stage-enabled`, `public-monitor-index`, `stage-monitor-index`, `public-mode-index` e `stage-mode-index` são `in-out`; modelos, status e avisos são `in`.
- Cabeçalho: ícone do app 40×40, título “Configurações”, espaçador, `IconButton` com `icon: @image-url("../../../vendor/material-1.1.0/ui/icons/close.svg")`; `clicked => root.close_requested()`. `Icons` não é exportado por `@material`, então use `@image-url` direto.
- Barra lateral de 190 px com o item “Saídas” selecionado; sem outras abas.
- Conteúdo: texto “Escolha onde cada saída deve aparecer. As alterações são aplicadas imediatamente.”; faixa de erro quando `database-error != ""`; faixa de erro separada quando `persistence-error != ""`; faixa de aviso quando `monitor-warning != ""`; nota “Com um único monitor, as saídas abrem como janelas.” quando `single-monitor`; cartões Public e Stage.
- Cada cartão usa um componente interno `OutputCard` com `in-out property <bool> enabled`, `in-out property <int> monitor-index`, `in-out property <int> mode-index`, `in property <[MenuItem]> monitor-items`, `in property <[MenuItem]> mode-items`, `in property <string> status`, `in property <int> status-kind`, `in property <bool> visible`, `in property <bool> controls-enabled`, além de título e descrição (de `OutputKind::label_pt_br`/`description_pt_br`). O cartão mostra `Switch` “Ativa”, `DropDownMenu` “Monitor” (`items: root.monitor-items`), `DropDownMenu` “Modo” (`items: root.mode-items`), chip de status e `TextButton` “Abrir agora”/“Fechar”.
- Use bindings bidirecionais: `Switch { checked <=> root.enabled; checked_state_changed(value) => { root.enabled-changed(value); } }` e `DropDownMenu { current_index <=> root.monitor-index; selected(index) => { root.monitor-selected(index); } }`. O `OutputCard` emite `enabled-changed(bool)`, `monitor-selected(int)`, `mode-selected(int)` e `visibility-toggled()`; a `SettingsWindow` liga cada cartão às suas propriedades `in-out` (`public-enabled`, etc.) com `<=>` e passa `controls-enabled` para ambos.
- `OutputCard` recebe `controls-enabled`; quando falso, todos os controles ficam desabilitados. O botão “Abrir agora” também fica desabilitado quando a saída está desativada (`!enabled`).
- Rodapé: “As alterações são aplicadas imediatamente.” à esquerda e `TextButton` “Fechar” à direita, emitindo `close_requested`.
- Use `MenuItem` do Material para os modelos; no Rust, construa com `MenuItem { text: "...".into(), enabled: true, ..Default::default() }`. `enabled` é `bool` sem default declarado e cai para `false`, então precisa ser definido explicitamente; os campos de ícone e `trailing_text` usam o default vazio e não precisam de SVG.

- [ ] **Step 2: Add the menu to the main window**

Em `ui/main.slint`:

- Adicione o callback `open-settings()`.
- No cabeçalho, antes do selo de versão, adicione um `IconButton` com `icon: @image-url("../../../vendor/material-1.1.0/ui/icons/menu.svg")`; `Icons` não é exportado por `@material`.
- Declare um `PopupMenu` ancorado abaixo do botão, com um único item “Configurações…”. Use `MenuItem { text: "Configurações…", enabled: true }` sem ícone; `enabled` precisa ser explícito porque o default de `bool` é `false`. Posicione com `x: menu-button.absolute-position.x + menu-button.width - settings-menu.width; y: menu-button.absolute-position.y + menu-button.height;` para alinhar à direita do botão e não sair da tela. Em `activated`, feche o popup com `close()` e emita `open-settings()`.
- Atualize a copy do painel para a v0.2.0: troque “A fundação está pronta.” por “Duas telas, uma apresentação.” e o texto auxiliar por “Configure o Public e o Stage em Configurações. As saídas habilitadas reabrem automaticamente na próxima execução.”
- Importe e reexporte `SettingsWindow` de `settings.slint` para o `include_modules!` gerar o tipo.

- [ ] **Step 3: Write the failing settings tests**

Em `settings.rs`, adicione testes puros:

```rust
#[test]
fn mode_index_matches_the_slint_order() {
    assert_eq!(mode_index(OutputMode::Auto), 0);
    assert_eq!(mode_index(OutputMode::Windowed), 1);
    assert_eq!(mode_index(OutputMode::Maximized), 2);
    assert_eq!(mode_index(OutputMode::Fullscreen), 3);
}

#[test]
fn mode_from_index_matches_the_slint_order_and_falls_back_to_auto() {
    for mode in OutputMode::ALL {
        assert_eq!(mode_from_index(mode_index(mode)), mode);
    }
    assert_eq!(mode_from_index(-1), OutputMode::Auto);
    assert_eq!(mode_from_index(4), OutputMode::Auto);
    assert_eq!(mode_from_index(99), OutputMode::Auto);
}
```

- [ ] **Step 4: Run tests to verify they fail**

Run: `cargo test -p op-admin`
Expected: FAIL porque `settings.rs` e `mode_index` não existem.

- [ ] **Step 5: Implement `settings.rs`**

- `open`: relê os monitores (`controller.refresh_monitors()`), reutiliza a janela existente ou cria uma nova, chama `refresh` e mostra. Importe `crate::{MenuItem, SettingsWindow}` e `crate::monitors::MonitorInfo`.
- `refresh`: preenche `public-monitor-items` e `stage-monitor-items` (Automático, Principal, um item por monitor, na ordem de `monitors()`), `mode-items` (na ordem de `OutputMode::ALL`: Automático, Janela, Maximizada, Tela cheia), os quatro índices (`monitor_index(kind)` e `mode_index(config().get(kind).mode)`), os estados, `single-monitor` (`monitors().len() == 1`), `monitor-warning`, `database-error`, `persistence-error` e `controls-enabled` (`database_error().is_none()`); chama `refresh_status` no fim para não duplicar a lógica de status. Todos os `MenuItem` usam `enabled: true`.
- `refresh_status`: atualiza apenas `public-status`, `stage-status`, os `*-status-kind`, `public-visible`, `stage-visible`, `persistence-error`, `monitor-warning` e `database-error`. Não troca modelos nem índices, para o polling de 500 ms não fechar dropdowns nem recriar itens.
- `refresh_if_open`: retorna sem fazer nada quando a janela não existe ou está oculta; caso contrário, chama apenas `refresh_status`.
- `mode_index` mapeia `OutputMode` para 0..3; `mode_from_index` aceita 0..3 e devolve `Auto` para qualquer outro valor; `monitor_items` monta os itens com `MenuItem { text, enabled: true, ..Default::default() }` e usa `ModelRc::from(items.as_slice())`.
- `wire`: use `window.as_weak()` em todos os callbacks para não criar ciclo forte entre a janela e seus próprios callbacks. Cada callback usa `try_borrow_mut()` no controlador (não `borrow_mut()`), aplica a mudança, solta o borrow, atualiza a janela com `refresh` e ignora o erro se a janela já tiver sido destruída. O `try_borrow_mut` evita pânico quando `refresh` altera `public-enabled`/`stage-enabled` e o `Switch` dispara `checked_state_changed` de forma síncrona; nesse caso o evento é ignorado, pois o valor veio do próprio controlador. Mapeamentos: `*_enabled_changed` → `set_enabled`; `*_monitor_selected(index)` → `set_monitor(kind, selection_for_index(index))`; `*_mode_selected(index)` → `set_mode(kind, mode_from_index(index))`; `*_visibility_toggled` → `hide` se `is_visible(kind)`, senão `show_now`. `close_requested` apenas esconde.
- `hide`: esconde a janela se existir; o slot continua com o handle forte.
- Mantenha a janela forte em `Rc<RefCell<Option<SettingsWindow>>>`; fechar pelo X não a destrói.

- [ ] **Step 6: Wire the menu and settings window in `ui.rs`**

- Crie `Rc<RefCell<Option<SettingsWindow>>>` e trate `ui.on_open_settings` chamando `settings::open(controller.clone(), slot.clone())`; o erro de abertura vai para `stderr`.
- No fechamento do painel principal, esconda as Configurações, chame `controller.borrow_mut().close_all()`, `slint::quit_event_loop()` e devolva `CloseRequestResponse::HideWindow`.
- Inicie um `slint::Timer` repetido de 500 ms que chama `settings::refresh_if_open` (somente status/visibilidade) enquanto o app estiver aberto, para refletir o X das saídas e mudanças de estado.
- Mantenha o timer vivo até depois de `slint::run_event_loop()`.

- [ ] **Step 7: Build and verify manually with one monitor**

```bash
OPENPRESENTER_DATA_DIR=/tmp/openpresenter-multitelas cargo run -p op-admin
```

Verifique:

- O menu ☰ abre “Configurações…” e a janela independente.
- Public aparece “Aberta”; Stage “Desativada”.
- Ativar Stage abre a janela; alternar Janela/Maximizada/Tela cheia funciona.
- Fechar Public pelo X deixa o switch “Ativa” ligado e o status “Oculta”; “Abrir agora” reabre.
- Desativar Public fecha a janela e persiste `enabled = 0`.
- Reiniciar o app restaura as saídas habilitadas.
- Com um monitor, a nota de monitor único aparece.

- [ ] **Step 8: Run the quality gates and commit**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
git add crates/op-admin
git commit -m "feat(admin): add output settings window"
```

---

### Task 7: Documentação, ADR e verificação final da v0.2.0

**Files:**
- Create: `docs/decisions/0003-saidas-multitela.md`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `docs/README.md`
- Modify: `docs/architecture/overview.md`
- Test: workspace commands + aceitação manual

**Interfaces:**
- Consumes: todas as tasks anteriores.
- Produces: v0.2.0 documentada e verificada; ADR 0003 aceito.

- [ ] **Step 1: Write ADR 0003**

Crie `docs/decisions/0003-saidas-multitela.md` com:

- **Contexto:** painel único da v0.1.0; necessidade de Public e Stage, monitores e restauração.
- **Decisão:** janelas Slint independentes, `display-info` para monitores, schema v2 `output_config`, janela de Configurações independente, Public ativo por padrão, Stage opcional, X apenas oculta.
- **Alternativas:** modal de configuração; JSON em vez de SQLite; `winit` direto; uma única janela com duas superfícies.
- **Consequências:** loop de eventos único com múltiplas janelas, normalização de coordenadas por plataforma, identidade de monitor best-effort, sem hot-plug, placeholders até o modelo de apresentação.

- [ ] **Step 2: Update the architecture overview**

Atualize `docs/architecture/overview.md` com:

- módulos `monitors.rs`, `outputs.rs`, `settings.rs` e `ui.rs`;
- `display-info` em `op-admin` e normalização para pixels físicos;
- fluxo de inicialização: sessão, configuração de saídas, painel, enumeração, abertura das saídas e loop;
- schema v2 com `output_config`;
- ciclo de vida (X oculta, desativar fecha, restaurar na inicialização);
- estados degradados de banco, monitores e janelas.

- [ ] **Step 3: Update README, changelog and docs index**

- `README.md`: versão 0.2.0, descrição das saídas Public/Stage, caminho do menu, comportamento padrão, schema v2 e comandos.
- `CHANGELOG.md`: `## [0.2.0] - 2026-10-28` com as saídas, Configurações, migração v2 e dependência `display-info`.
- `docs/README.md`: links para a spec, este plano e o ADR 0003.

- [ ] **Step 4: Run the full quality gates**

```bash
cargo fmt --all
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Expected: nenhum diff de formatação, nenhum warning de clippy e todos os testes verdes.

- [ ] **Step 5: Run the acceptance checks**

```bash
rm -rf /tmp/openpresenter-multitelas-accept
OPENPRESENTER_DATA_DIR=/tmp/openpresenter-multitelas-accept cargo run -p op-admin
sqlite3 "/tmp/openpresenter-multitelas-accept/open-presenter.db" \
  "PRAGMA user_version; SELECT output, enabled, monitor_key, monitor_name, mode FROM output_config ORDER BY output;"
```

Expected: painel abre; Public abre como janela; Stage não abre; schema `2`; linhas de `output_config` refletem as alterações feitas nas Configurações. Repita a execução e confirme a restauração. Teste o estado degradado apontando `OPENPRESENTER_DATA_DIR` para um arquivo comum: o painel abre, nenhuma saída abre e as Configurações mostram o erro com controles desabilitados.

- [ ] **Step 6: Verify versions and repository state**

```bash
rg -n "0\\.2\\.0" Cargo.toml README.md CHANGELOG.md crates/op-admin/ui/main.slint
rg -n "schema v2|output_config|display-info" README.md docs/architecture/overview.md docs/decisions/0003-saidas-multitela.md
git status --short
```

Expected: versão 0.2.0 consistente, documentos linkados e apenas mudanças intencionais.

- [ ] **Step 7: Commit**

```bash
git add README.md CHANGELOG.md docs
git commit -m "docs: document multi-output v0.2.0"
```

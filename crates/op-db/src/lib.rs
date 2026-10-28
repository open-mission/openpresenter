use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, SecondsFormat, Utc};
use op_core::{
    AppMeta, DEFAULT_LOCALE, MonitorSelection, OutputConfig, OutputKind, OutputMode, OutputsConfig,
    SessionInfo, Theme,
};
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};
use thiserror::Error;

/// Erros da camada de banco de dados.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("não foi possível abrir o banco de dados")]
    Open(#[source] rusqlite::Error),
    #[error("falha ao aplicar as migrações do banco de dados")]
    Migration(#[from] rusqlite_migration::Error),
    #[error("falha ao ler o estado do banco de dados")]
    Read(#[source] rusqlite::Error),
    #[error("falha ao gravar o estado do banco de dados")]
    Write(#[source] rusqlite::Error),
    #[error("data inválida no banco de dados: {0}")]
    InvalidTimestamp(String),
}

fn parse_timestamp(value: String) -> Result<DateTime<Utc>, DbError> {
    match DateTime::parse_from_rfc3339(&value) {
        Ok(dt) => Ok(dt.with_timezone(&Utc)),
        Err(_) => Err(DbError::InvalidTimestamp(value)),
    }
}

fn format_timestamp(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// Conexão SQLite já migrada, com o caminho do arquivo para diagnóstico.
pub struct Database {
    conn: Connection,
    path: PathBuf,
}

impl Database {
    /// Abre o banco, aplica os PRAGMAs e roda as migrações pendentes.
    pub fn open(path: &Path) -> Result<Self, DbError> {
        let mut conn = Connection::open(path).map_err(DbError::Open)?;

        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(DbError::Open)?;
        conn.pragma_update(None, "synchronous", "NORMAL")
            .map_err(DbError::Open)?;
        conn.pragma_update(None, "foreign_keys", "ON")
            .map_err(DbError::Open)?;
        conn.busy_timeout(Duration::from_secs(5))
            .map_err(DbError::Open)?;

        let migrations = Migrations::new(vec![
            M::up(include_str!("../migrations/0001_initial.sql")),
            M::up(include_str!("../migrations/0002_output_config.sql")),
        ]);
        migrations
            .to_latest(&mut conn)
            .map_err(DbError::Migration)?;

        Ok(Self {
            conn,
            path: path.to_path_buf(),
        })
    }

    /// Versão atual do schema, controlada por `PRAGMA user_version`.
    pub fn schema_version(&self) -> Result<u32, DbError> {
        let version: i64 = self
            .conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(DbError::Read)?;
        Ok(version as u32)
    }

    /// Caminho do arquivo de banco aberto.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Lê o estado persistido, se já existir.
    fn load_meta(&self) -> Result<Option<AppMeta>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT app_version, first_run_at, last_run_at, theme, locale, updated_at
                 FROM app_meta WHERE id = 1",
            )
            .map_err(DbError::Read)?;
        let mut rows = stmt.query([]).map_err(DbError::Read)?;

        let Some(row) = rows.next().map_err(DbError::Read)? else {
            return Ok(None);
        };

        let app_version: String = row.get(0).map_err(DbError::Read)?;
        let first_run_at = parse_timestamp(row.get(1).map_err(DbError::Read)?)?;
        let last_run_at = parse_timestamp(row.get(2).map_err(DbError::Read)?)?;
        let theme_value: String = row.get(3).map_err(DbError::Read)?;
        let locale: String = row.get(4).map_err(DbError::Read)?;
        let updated_at = parse_timestamp(row.get(5).map_err(DbError::Read)?)?;

        Ok(Some(AppMeta {
            app_version,
            first_run_at,
            last_run_at,
            // Uma preferência de tema desconhecida não deve impedir a abertura;
            // nesse caso, volta ao padrão do sistema.
            theme: Theme::from_db_str(&theme_value).unwrap_or_default(),
            locale,
            updated_at,
        }))
    }

    /// Registra a sessão atual, preservando a primeira execução e as preferências.
    pub fn begin_session(
        &self,
        app_version: &str,
        now: DateTime<Utc>,
    ) -> Result<SessionInfo, DbError> {
        let previous = self.load_meta()?;
        let previous_last_run_at = previous.as_ref().map(|meta| meta.last_run_at);
        let first_run_at = previous.as_ref().map_or(now, |meta| meta.first_run_at);
        let theme = previous.as_ref().map_or(Theme::System, |meta| meta.theme);
        let locale = previous
            .as_ref()
            .map_or_else(|| DEFAULT_LOCALE.to_string(), |meta| meta.locale.clone());

        let first_run_at_text = format_timestamp(first_run_at);
        let last_run_at_text = format_timestamp(now);
        let updated_at_text = format_timestamp(now);

        let current = AppMeta {
            app_version: app_version.to_string(),
            first_run_at,
            last_run_at: now,
            theme,
            locale,
            updated_at: now,
        };

        self.conn
            .execute(
                "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
                 VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(id) DO UPDATE SET
                     app_version = excluded.app_version,
                     last_run_at = excluded.last_run_at,
                     theme = excluded.theme,
                     locale = excluded.locale,
                     updated_at = excluded.updated_at",
                rusqlite::params![
                    &current.app_version,
                    &first_run_at_text,
                    &last_run_at_text,
                    current.theme.as_str(),
                    &current.locale,
                    &updated_at_text,
                ],
            )
            .map_err(DbError::Write)?;

        Ok(SessionInfo {
            current,
            previous_last_run_at,
        })
    }

    /// Lê a configuração das saídas, preenchendo ausências com os padrões.
    pub fn load_outputs(&self) -> Result<OutputsConfig, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT output, enabled, monitor_key, monitor_name, mode
                 FROM output_config",
            )
            .map_err(DbError::Read)?;
        let mut rows = stmt.query([]).map_err(DbError::Read)?;
        let mut outputs = OutputsConfig::default();

        while let Some(row) = rows.next().map_err(DbError::Read)? {
            let output_value: String = row.get(0).map_err(DbError::Read)?;
            // Linhas com saída desconhecida não devem impedir a abertura.
            let Some(kind) = OutputKind::from_db_str(&output_value) else {
                continue;
            };
            let enabled: i64 = row.get(1).map_err(DbError::Read)?;
            let monitor_key: String = row.get(2).map_err(DbError::Read)?;
            let monitor_name: String = row.get(3).map_err(DbError::Read)?;
            let mode_value: String = row.get(4).map_err(DbError::Read)?;

            let config = outputs.get_mut(kind);
            config.enabled = enabled != 0;
            // Valores inválidos, possíveis apenas por edição manual, caem para Auto.
            config.monitor = MonitorSelection::from_db_key(&monitor_key, monitor_name);
            config.mode = OutputMode::from_db_str(&mode_value).unwrap_or_default();
        }

        Ok(outputs)
    }

    /// Grava a configuração de uma saída, preservando a outra.
    pub fn save_output(&self, config: &OutputConfig, now: DateTime<Utc>) -> Result<(), DbError> {
        let monitor_name = match &config.monitor {
            MonitorSelection::Display { name, .. } => name.as_str(),
            MonitorSelection::Auto | MonitorSelection::Primary => "",
        };
        let enabled = i64::from(config.enabled);

        self.conn
            .execute(
                "INSERT INTO output_config (output, enabled, monitor_key, monitor_name, mode, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(output) DO UPDATE SET
                     enabled = excluded.enabled,
                     monitor_key = excluded.monitor_key,
                     monitor_name = excluded.monitor_name,
                     mode = excluded.mode,
                     updated_at = excluded.updated_at",
                rusqlite::params![
                    config.kind.as_str(),
                    enabled,
                    config.monitor.as_db_key(),
                    monitor_name,
                    config.mode.as_str(),
                    format_timestamp(now),
                ],
            )
            .map_err(DbError::Write)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use op_core::{MonitorSelection, OutputConfig, OutputKind, OutputMode, OutputsConfig, Theme};

    fn ts(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc
            .with_ymd_and_hms(year, month, day, hour, minute, 0)
            .single()
            .unwrap()
    }

    #[test]
    fn open_creates_schema_and_applies_pragmas() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("open-presenter.db");
        let db = Database::open(&path).unwrap();

        assert_eq!(db.schema_version().unwrap(), 2);
        let table_count: i64 = db
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'app_meta'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 1);
        let journal_mode: String = db
            .conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .unwrap();
        assert_eq!(journal_mode.to_lowercase(), "wal");
        let foreign_keys: i64 = db
            .conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(foreign_keys, 1);
        let busy_timeout: i64 = db
            .conn
            .query_row("PRAGMA busy_timeout", [], |row| row.get(0))
            .unwrap();
        assert_eq!(busy_timeout, 5000);
        let synchronous: i64 = db
            .conn
            .query_row("PRAGMA synchronous", [], |row| row.get(0))
            .unwrap();
        assert_eq!(synchronous, 1);
    }

    #[test]
    fn open_creates_schema_v2_with_output_config() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

        assert_eq!(db.schema_version().unwrap(), 2);
        let table_count: i64 = db
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'output_config'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 1);
    }

    #[test]
    fn migrates_v1_database_to_v2_preserving_app_meta() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("open-presenter.db");
        {
            let conn = rusqlite::Connection::open(&path).unwrap();
            conn.execute_batch(include_str!("../migrations/0001_initial.sql"))
                .unwrap();
            conn.pragma_update(None, "user_version", 1).unwrap();
            conn.execute(
                "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
                 VALUES (1, '0.1.0', '2026-10-28T10:00:00.000Z', '2026-10-28T10:00:00.000Z', 'system', 'pt-BR', '2026-10-28T10:00:00.000Z')",
                [],
            )
            .unwrap();
        }

        let db = Database::open(&path).unwrap();
        assert_eq!(db.schema_version().unwrap(), 2);
        let meta = db.load_meta().unwrap().unwrap();
        assert_eq!(meta.first_run_at, ts(2026, 10, 28, 10, 0));
        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM output_config", [], |row| row.get(0))
            .unwrap();
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
            monitor: MonitorSelection::Display {
                id: "7".into(),
                name: "DELL".into(),
            },
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
        db.conn
            .pragma_update(None, "ignore_check_constraints", true)
            .unwrap();
        db.conn
            .execute(
                "INSERT INTO output_config (output, enabled, monitor_key, monitor_name, mode, updated_at)
                 VALUES ('public', 1, 'bogus', '', 'invalid', '2026-10-28T11:00:00.000Z')",
                [],
            )
            .unwrap();

        let loaded = db.load_outputs().unwrap();
        assert_eq!(loaded.public.monitor, MonitorSelection::Auto);
        assert_eq!(loaded.public.mode, OutputMode::Auto);
    }

    #[test]
    fn open_fails_when_parent_directory_is_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing").join("db.sqlite");
        assert!(matches!(Database::open(&path), Err(DbError::Open(_))));
    }

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
            db.conn
                .execute(
                    "UPDATE app_meta SET theme = 'dark', locale = 'en-US' WHERE id = 1",
                    [],
                )
                .unwrap();
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
        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM app_meta", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn invalid_timestamp_is_reported_without_panicking() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
        db.conn
            .execute(
                "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
                 VALUES (1, '0.1.0', 'not-a-date', 'not-a-date', 'system', 'pt-BR', 'not-a-date')",
                [],
            )
            .unwrap();
        assert!(matches!(
            db.begin_session("0.1.0", ts(2026, 10, 28, 10, 0)),
            Err(DbError::InvalidTimestamp(_))
        ));
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
}

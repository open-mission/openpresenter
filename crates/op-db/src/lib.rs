use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, SecondsFormat, Utc};
use op_core::{AppMeta, DEFAULT_LOCALE, SessionInfo, Theme};
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

        let migrations =
            Migrations::new(vec![M::up(include_str!("../migrations/0001_initial.sql"))]);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use op_core::Theme;

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

        assert_eq!(db.schema_version().unwrap(), 1);
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

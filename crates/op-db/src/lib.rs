use std::path::{Path, PathBuf};
use std::time::Duration;

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

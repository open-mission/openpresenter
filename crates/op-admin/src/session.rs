use std::path::{Path, PathBuf};

use chrono::{DateTime, Local, Utc};
use directories::BaseDirs;
use op_core::{DEFAULT_LOCALE, STACK, TAGLINE, Theme, VERSION};
use op_db::Database;

/// Dados prontos para a tela inicial, inclusive em estado degradado.
#[derive(Debug, Clone)]
pub struct SessionView {
    pub app_version: String,
    pub tagline: String,
    pub stack: String,
    pub db_path: String,
    pub db_ready: bool,
    pub db_error: String,
    pub schema_version: i32,
    pub theme_label: String,
    pub locale: String,
    pub first_run_at: String,
    pub last_run_at: String,
    pub is_first_run: bool,
}

/// Carrega a sessão usando o diretório padrão ou o override de ambiente.
pub fn load_session() -> SessionView {
    match resolve_data_dir() {
        Ok(dir) => load_session_from_dir(&dir),
        Err(error) => degraded_view(Path::new("(não resolvido)"), &error.to_string()),
    }
}

fn resolve_data_dir() -> anyhow::Result<PathBuf> {
    if let Some(path) = std::env::var_os("OPENPRESENTER_DATA_DIR")
        && !path.is_empty()
    {
        return Ok(PathBuf::from(path));
    }

    let base_dirs = BaseDirs::new().ok_or_else(|| {
        anyhow::anyhow!("não foi possível resolver o diretório de dados do sistema")
    })?;
    Ok(base_dirs.data_local_dir().join("OpenPresenter"))
}

fn load_session_from_dir(dir: &Path) -> SessionView {
    let db_path = dir.join("open-presenter.db");

    if let Err(error) = std::fs::create_dir_all(dir) {
        return degraded_view(
            &db_path,
            &format!("não foi possível criar o diretório de dados: {error}"),
        );
    }

    let db = match Database::open(&db_path) {
        Ok(db) => db,
        Err(error) => return degraded_view(&db_path, &error_message(&error)),
    };

    let schema_version = match db.schema_version() {
        Ok(version) => version as i32,
        Err(error) => return degraded_view(&db_path, &error_message(&error)),
    };

    match db.begin_session(VERSION, Utc::now()) {
        Ok(info) => SessionView {
            app_version: VERSION.to_string(),
            tagline: TAGLINE.to_string(),
            stack: STACK.to_string(),
            db_path: db.path().display().to_string(),
            db_ready: true,
            db_error: String::new(),
            schema_version,
            theme_label: info.current.theme.label_pt_br().to_string(),
            locale: info.current.locale,
            first_run_at: format_timestamp(&info.current.first_run_at),
            last_run_at: info
                .previous_last_run_at
                .as_ref()
                .map(format_timestamp)
                .unwrap_or_default(),
            is_first_run: info.previous_last_run_at.is_none(),
        },
        Err(error) => degraded_view(&db_path, &error_message(&error)),
    }
}

fn degraded_view(db_path: &Path, error: &str) -> SessionView {
    SessionView {
        app_version: VERSION.to_string(),
        tagline: TAGLINE.to_string(),
        stack: STACK.to_string(),
        db_path: db_path.display().to_string(),
        db_ready: false,
        db_error: error.to_string(),
        schema_version: 0,
        theme_label: Theme::System.label_pt_br().to_string(),
        locale: DEFAULT_LOCALE.to_string(),
        first_run_at: String::new(),
        last_run_at: String::new(),
        is_first_run: false,
    }
}

fn error_message(error: &impl std::error::Error) -> String {
    let mut message = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}

fn format_timestamp(dt: &DateTime<Utc>) -> String {
    dt.with_timezone(&Local)
        .format("%d/%m/%Y às %H:%M")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

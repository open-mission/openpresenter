use std::path::{Path, PathBuf};

use chrono::{DateTime, Local, Utc};
use directories::BaseDirs;
use op_core::{DEFAULT_LOCALE, STACK, TAGLINE, Theme, VERSION};
use op_db::Database;

/// Dados prontos para a tela inicial, inclusive em estado degradado.
#[derive(Debug, Clone, serde::Serialize)]
#[allow(dead_code)]
pub struct SessionView {
    pub app_version: String,
    pub tagline: String,
    pub stack: String,
    pub db_path: String,
    pub media_path: String,
    pub db_ready: bool,
    pub db_error: String,
    pub schema_version: i32,
    pub theme_label: String,
    pub locale: String,
    pub first_run_at: String,
    pub last_run_at: String,
    pub is_first_run: bool,
}

/// Sessão carregada: a visão pronta para a tela e a conexão viva do banco.
pub struct LoadedSession {
    pub view: SessionView,
    pub db: Option<Database>,
}

/// Carrega a sessão usando o diretório padrão ou o override de ambiente.
pub fn load_session() -> LoadedSession {
    match resolve_data_dir() {
        Ok(dir) => load_session_from_dir(&dir),
        Err(error) => degraded_session(Path::new("(não resolvido)"), &error.to_string()),
    }
}

pub fn resolve_data_dir() -> anyhow::Result<PathBuf> {
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

pub fn resolve_media_dir() -> anyhow::Result<PathBuf> {
    let data_dir = resolve_data_dir()?;
    let media_dir = data_dir.join("media");
    let wallpapers_dir = media_dir.join("wallpapers");
    std::fs::create_dir_all(&wallpapers_dir)?;
    ensure_default_wallpapers(&wallpapers_dir);
    Ok(media_dir)
}

pub fn resolve_bibles_dir() -> anyhow::Result<PathBuf> {
    let data_dir = resolve_data_dir()?;
    let bibles_dir = data_dir.join("bibles");
    std::fs::create_dir_all(&bibles_dir)?;
    Ok(bibles_dir)
}

fn ensure_default_wallpapers(wallpapers_dir: &Path) {
    let dark_ambient = r###"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1920 1080" width="100%" height="100%">
  <defs>
    <radialGradient id="bg" cx="50%" cy="45%" r="70%">
      <stop offset="0%" stop-color="#18181b"/>
      <stop offset="60%" stop-color="#09090b"/>
      <stop offset="100%" stop-color="#000000"/>
    </radialGradient>
  </defs>
  <rect width="1920" height="1080" fill="url(#bg)"/>
</svg>"###;

    let worship_glow = r###"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1920 1080" width="100%" height="100%">
  <defs>
    <radialGradient id="glow" cx="50%" cy="35%" r="75%">
      <stop offset="0%" stop-color="#1e1b4b"/>
      <stop offset="50%" stop-color="#09090b"/>
      <stop offset="100%" stop-color="#020617"/>
    </radialGradient>
  </defs>
  <rect width="1920" height="1080" fill="url(#glow)"/>
</svg>"###;

    let cross_subtle = r###"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1920 1080" width="100%" height="100%">
  <rect width="1920" height="1080" fill="#09090b"/>
  <g opacity="0.08" transform="translate(960, 500)">
    <rect x="-8" y="-140" width="16" height="280" rx="8" fill="#ffffff"/>
    <rect x="-70" y="-70" width="140" height="16" rx="8" fill="#ffffff"/>
  </g>
</svg>"###;

    let f1 = wallpapers_dir.join("dark-ambient.svg");
    if !f1.exists() {
        let _ = std::fs::write(f1, dark_ambient);
    }
    let f2 = wallpapers_dir.join("worship-glow.svg");
    if !f2.exists() {
        let _ = std::fs::write(f2, worship_glow);
    }
    let f3 = wallpapers_dir.join("cross-subtle.svg");
    if !f3.exists() {
        let _ = std::fs::write(f3, cross_subtle);
    }
}

fn load_session_from_dir(dir: &Path) -> LoadedSession {
    let db_path = dir.join("open-presenter.db");
    let media_dir = dir.join("media");
    let wallpapers_dir = media_dir.join("wallpapers");
    let images_dir = media_dir.join("images");
    let videos_dir = media_dir.join("videos");
    let church_brand_dir = media_dir.join("church_brand");
    let fonts_dir = dir.join("fonts");
    let _ = std::fs::create_dir_all(&wallpapers_dir);
    let _ = std::fs::create_dir_all(&images_dir);
    let _ = std::fs::create_dir_all(&videos_dir);
    let _ = std::fs::create_dir_all(&church_brand_dir);
    let _ = std::fs::create_dir_all(&fonts_dir);
    ensure_default_wallpapers(&wallpapers_dir);

    if let Err(error) = std::fs::create_dir_all(dir) {
        return degraded_session(
            &db_path,
            &format!("não foi possível criar o diretório de dados: {error}"),
        );
    }

    let db = match Database::open(&db_path) {
        Ok(db) => db,
        Err(error) => return degraded_session(&db_path, &error_message(&error)),
    };

    let schema_version = match db.schema_version() {
        Ok(version) => version as i32,
        Err(error) => return degraded_session(&db_path, &error_message(&error)),
    };

    match db.begin_session(VERSION, Utc::now()) {
        Ok(info) => LoadedSession {
            view: SessionView {
                app_version: VERSION.to_string(),
                tagline: TAGLINE.to_string(),
                stack: STACK.to_string(),
                db_path: db.path().display().to_string(),
                media_path: media_dir.display().to_string(),
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
            db: Some(db),
        },
        Err(error) => degraded_session(&db_path, &error_message(&error)),
    }
}

fn degraded_session(db_path: &Path, error: &str) -> LoadedSession {
    LoadedSession {
        view: degraded_view(db_path, error),
        db: None,
    }
}

fn degraded_view(db_path: &Path, error: &str) -> SessionView {
    SessionView {
        app_version: VERSION.to_string(),
        tagline: TAGLINE.to_string(),
        stack: STACK.to_string(),
        db_path: db_path.display().to_string(),
        media_path: String::new(),
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
        let loaded = load_session_from_dir(dir.path());
        let view = loaded.view;
        assert!(view.db_ready);
        assert_eq!(view.app_version, op_core::VERSION);
        assert_eq!(view.schema_version, 7);
        assert!(view.is_first_run);
        assert!(view.db_path.ends_with("open-presenter.db"));
        assert!(!view.first_run_at.is_empty());
    }

    #[test]
    fn second_session_view_keeps_first_run_and_fills_last_run() {
        let dir = tempfile::tempdir().unwrap();
        let first = load_session_from_dir(dir.path()).view;
        let second = load_session_from_dir(dir.path()).view;
        assert!(second.db_ready);
        assert!(!second.is_first_run);
        assert_eq!(second.first_run_at, first.first_run_at);
        assert!(!second.last_run_at.is_empty());
    }

    #[test]
    fn successful_session_returns_live_database() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = load_session_from_dir(dir.path());
        assert!(loaded.view.db_ready);
        assert!(loaded.db.is_some());
        assert_eq!(loaded.view.schema_version, 7);
    }

    #[test]
    fn unwritable_data_dir_returns_degraded_view() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("not-a-dir");
        std::fs::write(&file, "x").unwrap();
        let loaded = load_session_from_dir(&file);
        let view = loaded.view;
        assert!(!view.db_ready);
        assert!(!view.db_error.is_empty());
        assert_eq!(view.schema_version, 0);
        assert!(loaded.db.is_none());
    }
}

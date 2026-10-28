use chrono::{DateTime, Utc};

/// Nome público do aplicativo.
pub const APP_NAME: &str = "OpenPresenter";
/// Tagline exibida na interface.
pub const TAGLINE: &str = "Projeção para igrejas e eventos";
/// Stack apresentada na tela inicial.
pub const STACK: &str = "Rust · Slint · SQLite";
/// Versão do aplicativo, herdada de `[workspace.package] version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Idioma padrão do aplicativo.
pub const DEFAULT_LOCALE: &str = "pt-BR";

/// Preferência de tema, espelhada na coluna `theme` de `app_meta`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Theme {
    /// Representação persistida no banco.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// Converte a representação persistida, rejeitando valores desconhecidos.
    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "system" => Some(Self::System),
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }

    /// Rótulo em português do Brasil para exibição.
    pub const fn label_pt_br(self) -> &'static str {
        match self {
            Self::System => "Sistema",
            Self::Light => "Claro",
            Self::Dark => "Escuro",
        }
    }
}

/// Estado persistido do aplicativo, correspondente ao registro único de `app_meta`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppMeta {
    pub app_version: String,
    pub first_run_at: DateTime<Utc>,
    pub last_run_at: DateTime<Utc>,
    pub theme: Theme,
    pub locale: String,
    pub updated_at: DateTime<Utc>,
}

/// Resultado de uma abertura de sessão: o estado atual e a execução anterior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionInfo {
    pub current: AppMeta,
    pub previous_last_run_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

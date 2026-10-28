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

/// Saída do aplicativo, espelhada na coluna `output` de `output_config`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputKind {
    Public,
    Stage,
}

impl OutputKind {
    /// Todas as saídas na ordem de exibição.
    pub const ALL: [OutputKind; 2] = [OutputKind::Public, OutputKind::Stage];

    /// Representação persistida no banco.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Stage => "stage",
        }
    }

    /// Converte a representação persistida, rejeitando valores desconhecidos.
    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "public" => Some(Self::Public),
            "stage" => Some(Self::Stage),
            _ => None,
        }
    }

    /// Rótulo em português do Brasil para exibição.
    pub const fn label_pt_br(&self) -> &'static str {
        match self {
            Self::Public => "Public",
            Self::Stage => "Stage",
        }
    }

    /// Descrição em português do Brasil para exibição.
    pub const fn description_pt_br(&self) -> &'static str {
        match self {
            Self::Public => "Saída visível ao público.",
            Self::Stage => "Retorno para o palco e o apresentador.",
        }
    }
}

/// Modo de exibição de uma saída, espelhado na coluna `mode` de `output_config`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OutputMode {
    #[default]
    Auto,
    Windowed,
    Maximized,
    Fullscreen,
}

impl OutputMode {
    /// Todos os modos na ordem de exibição.
    pub const ALL: [OutputMode; 4] = [
        OutputMode::Auto,
        OutputMode::Windowed,
        OutputMode::Maximized,
        OutputMode::Fullscreen,
    ];

    /// Representação persistida no banco.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Windowed => "windowed",
            Self::Maximized => "maximized",
            Self::Fullscreen => "fullscreen",
        }
    }

    /// Converte a representação persistida, rejeitando valores desconhecidos.
    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "auto" => Some(Self::Auto),
            "windowed" => Some(Self::Windowed),
            "maximized" => Some(Self::Maximized),
            "fullscreen" => Some(Self::Fullscreen),
            _ => None,
        }
    }

    /// Rótulo em português do Brasil para exibição.
    pub const fn label_pt_br(&self) -> &'static str {
        match self {
            Self::Auto => "Automático",
            Self::Windowed => "Janela",
            Self::Maximized => "Maximizada",
            Self::Fullscreen => "Tela cheia",
        }
    }
}

/// Preferência de monitor de uma saída, persistida como chave textual.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum MonitorSelection {
    #[default]
    Auto,
    Primary,
    Display {
        id: String,
        name: String,
    },
}

impl MonitorSelection {
    /// Chave persistida no banco: `auto`, `primary` ou `id:<id>`.
    pub fn as_db_key(&self) -> String {
        match self {
            Self::Auto => "auto".to_owned(),
            Self::Primary => "primary".to_owned(),
            Self::Display { id, .. } => format!("id:{id}"),
        }
    }

    /// Converte a chave persistida; valores desconhecidos caem para `Auto`.
    pub fn from_db_key(key: &str, name: String) -> Self {
        match key {
            "auto" => Self::Auto,
            "primary" => Self::Primary,
            _ => match key.strip_prefix("id:") {
                Some(id) => Self::Display {
                    id: id.to_owned(),
                    name,
                },
                None => Self::Auto,
            },
        }
    }

    /// Rótulo em português do Brasil para exibição.
    pub fn label_pt_br(&self) -> &str {
        match self {
            Self::Auto => "Automático",
            Self::Primary => "Principal",
            Self::Display { name, .. } => name,
        }
    }
}

/// Configuração de uma saída.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputConfig {
    pub kind: OutputKind,
    pub enabled: bool,
    pub monitor: MonitorSelection,
    pub mode: OutputMode,
}

impl OutputConfig {
    /// Padrão da especificação para a saída indicada.
    pub fn default_for(kind: OutputKind) -> Self {
        match kind {
            OutputKind::Public => Self {
                kind,
                enabled: true,
                monitor: MonitorSelection::Auto,
                mode: OutputMode::Auto,
            },
            OutputKind::Stage => Self {
                kind,
                enabled: false,
                monitor: MonitorSelection::Primary,
                mode: OutputMode::Windowed,
            },
        }
    }
}

/// Configuração das duas saídas do aplicativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputsConfig {
    pub public: OutputConfig,
    pub stage: OutputConfig,
}

impl OutputsConfig {
    /// Acesso imutável à configuração da saída indicada.
    pub fn get(&self, kind: OutputKind) -> &OutputConfig {
        match kind {
            OutputKind::Public => &self.public,
            OutputKind::Stage => &self.stage,
        }
    }

    /// Acesso mutável à configuração da saída indicada.
    pub fn get_mut(&mut self, kind: OutputKind) -> &mut OutputConfig {
        match kind {
            OutputKind::Public => &mut self.public,
            OutputKind::Stage => &mut self.stage,
        }
    }

    /// Itera pelas saídas na ordem Public, Stage.
    pub fn iter(&self) -> impl Iterator<Item = &OutputConfig> {
        [&self.public, &self.stage].into_iter()
    }
}

impl Default for OutputsConfig {
    fn default() -> Self {
        Self {
            public: OutputConfig::default_for(OutputKind::Public),
            stage: OutputConfig::default_for(OutputKind::Stage),
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

    #[test]
    fn output_kinds_round_trip_and_have_portuguese_labels() {
        for kind in OutputKind::ALL {
            assert_eq!(OutputKind::from_db_str(kind.as_str()), Some(kind));
        }
        assert_eq!(OutputKind::from_db_str("invalid"), None);
        assert_eq!(OutputKind::Public.label_pt_br(), "Public");
        assert_eq!(OutputKind::Stage.label_pt_br(), "Stage");
        assert_eq!(
            OutputKind::Public.description_pt_br(),
            "Saída visível ao público."
        );
        assert_eq!(
            OutputKind::Stage.description_pt_br(),
            "Retorno para o palco e o apresentador."
        );
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
}

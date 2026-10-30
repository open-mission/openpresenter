use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod bible;
pub use bible::*;

/// Nome público do aplicativo.
pub const APP_NAME: &str = "OpenPresenter";
/// Tagline exibida na interface.
pub const TAGLINE: &str = "Projeção para igrejas e eventos";
/// Stack apresentada na tela inicial e metadados.
pub const STACK: &str = "Rust · Axum · Svelte · SQLite";
/// Versão do aplicativo, herdada de `[workspace.package] version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Idioma padrão do aplicativo.
pub const DEFAULT_LOCALE: &str = "pt-BR";

/// Preferência de tema, espelhada na coluna `theme` de `app_meta`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppMeta {
    pub app_version: String,
    pub first_run_at: DateTime<Utc>,
    pub last_run_at: DateTime<Utc>,
    pub theme: Theme,
    pub locale: String,
    pub updated_at: DateTime<Utc>,
}

/// Resultado de uma abertura de sessão: o estado atual e a execução anterior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionInfo {
    pub current: AppMeta,
    pub previous_last_run_at: Option<DateTime<Utc>>,
}

/// Tipo ou categoria de um item de mídia.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaKind {
    Song,
    Scripture,
    Announcement,
    Media,
    Header,
    Image,
    Text,
}

impl MediaKind {
    pub const fn label_pt_br(&self) -> &'static str {
        match self {
            Self::Song => "Música",
            Self::Scripture => "Bíblia",
            Self::Announcement => "Aviso",
            Self::Media => "Mídia",
            Self::Header => "Seção",
            Self::Image => "Imagem",
            Self::Text => "Texto",
        }
    }
}

/// Tipo de seção de uma música.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SectionType {
    #[default]
    Verse,
    PreChorus,
    Chorus,
    Bridge,
    Intro,
    Outro,
    Instrumental,
    Tag,
    Ending,
    Other,
}

impl<'de> Deserialize<'de> for SectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SectionType::from_label(&s).unwrap_or(SectionType::Other))
    }
}

impl SectionType {
    pub fn from_label(label: &str) -> Option<Self> {
        let normalized = label.trim().to_lowercase();
        let key = normalized
            .trim_end_matches(|c: char| c.is_ascii_digit())
            .trim_end();
        match key {
            "verso" | "estrofe" | "verse" => Some(Self::Verse),
            "refrão" | "refrao" | "coro" | "chorus" => Some(Self::Chorus),
            "pré-refrão" | "pre-refrao" | "pre-refrão" | "pre-chorus" | "pre chorus" => {
                Some(Self::PreChorus)
            }
            "ponte" | "bridge" => Some(Self::Bridge),
            "introdução" | "introducao" | "intro" => Some(Self::Intro),
            "final" | "outro" => Some(Self::Outro),
            "instrumental" => Some(Self::Instrumental),
            "tag" => Some(Self::Tag),
            "encerramento" | "ending" => Some(Self::Ending),
            "other" => Some(Self::Other),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verse => "verse",
            Self::PreChorus => "pre-chorus",
            Self::Chorus => "chorus",
            Self::Bridge => "bridge",
            Self::Intro => "intro",
            Self::Outro => "outro",
            Self::Instrumental => "instrumental",
            Self::Tag => "tag",
            Self::Ending => "ending",
            Self::Other => "other",
        }
    }

    pub const fn as_label(self) -> &'static str {
        match self {
            Self::Verse => "Verso",
            Self::PreChorus => "Pré-refrão",
            Self::Chorus => "Refrão",
            Self::Bridge => "Ponte",
            Self::Intro => "Introdução",
            Self::Outro => "Final",
            Self::Instrumental => "Instrumental",
            Self::Tag => "Tag",
            Self::Ending => "Encerramento",
            Self::Other => "Outro",
        }
    }
}

/// Seção estruturada de uma música.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongSection {
    #[serde(default)]
    pub section_type: Option<SectionType>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub lines: Vec<String>,
}

/// Modelo de música persistido no banco de dados.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    #[serde(default)]
    pub album: Option<String>,
    #[serde(default)]
    pub composer: Option<String>,
    #[serde(default)]
    pub song_key: Option<String>,
    #[serde(default)]
    pub bpm: Option<i32>,
    #[serde(default)]
    pub sections: Vec<SongSection>,
    pub lyrics_raw: String,
}

impl Song {
    pub fn into_media_item(self) -> MediaItem {
        let (_, slides) = parse_raw_lyrics(&self.lyrics_raw);
        MediaItem {
            id: self.id,
            title: self.title,
            artist_or_ref: if self.artist.is_empty() {
                None
            } else {
                Some(self.artist)
            },
            kind: MediaKind::Song,
            slides,
            align_h: None,
            align_v: None,
        }
    }
}

/// Resumo de uma música retornado nas buscas do catálogo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongSummary {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub song_key: Option<String>,
    pub bpm: Option<i32>,
    pub composer: Option<String>,
    pub album: Option<String>,
    pub preview_snippet: Option<String>,
}

/// Resultado de busca de letra em provedores online (Holyrics, Vagalume).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLyricHit {
    pub track_name: String,
    pub artist_name: String,
    pub album_name: Option<String>,
    pub provider: Option<String>,
    pub provider_id: Option<String>,
}

/// Converte o texto bruto da letra (Holyrics / ProPresenter style) em seções e slides.
///
/// Regras:
/// - Linhas iniciando com `##` definem o rótulo da seção (ex: `## Verso 1`, `## Refrão`).
/// - Linhas em branco (`\n\n`) separam slides dentro da seção.
pub fn parse_raw_lyrics(raw: &str) -> (Vec<SongSection>, Vec<Slide>) {
    let normalized = raw.replace("\r\n", "\n").replace('\r', "\n");
    let mut sections: Vec<SongSection> = Vec::new();
    let mut slides: Vec<Slide> = Vec::new();

    let mut current_label = "Verso 1".to_string();
    let mut current_section_lines: Vec<String> = Vec::new();
    let mut current_slide_lines: Vec<String> = Vec::new();
    let mut slide_idx = 1;

    for line in normalized.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("##") {
            // Nova seção detectada
            if !current_slide_lines.is_empty() {
                let content = current_slide_lines.join("\n");
                slides.push(Slide::new(
                    format!("slide-{}", slide_idx),
                    current_label.clone(),
                    content,
                ));
                slide_idx += 1;
                current_slide_lines.clear();
            }

            if !current_section_lines.is_empty() {
                let st = SectionType::from_label(&current_label);
                sections.push(SongSection {
                    section_type: st,
                    label: current_label.clone(),
                    lines: std::mem::take(&mut current_section_lines),
                });
            }

            let label_text = trimmed.trim_start_matches('#').trim();
            if !label_text.is_empty() {
                current_label = label_text.to_string();
            }
            continue;
        }

        if trimmed.is_empty() {
            // Linha em branco: quebra de slide
            if !current_slide_lines.is_empty() {
                let content = current_slide_lines.join("\n");
                slides.push(Slide::new(
                    format!("slide-{}", slide_idx),
                    current_label.clone(),
                    content,
                ));
                slide_idx += 1;
                current_slide_lines.clear();
            }
            continue;
        }

        current_slide_lines.push(trimmed.to_string());
        current_section_lines.push(trimmed.to_string());
    }

    // Flush final
    if !current_slide_lines.is_empty() {
        let content = current_slide_lines.join("\n");
        slides.push(Slide::new(
            format!("slide-{}", slide_idx),
            current_label.clone(),
            content,
        ));
    }

    if !current_section_lines.is_empty() {
        let st = SectionType::from_label(&current_label);
        sections.push(SongSection {
            section_type: st,
            label: current_label,
            lines: current_section_lines,
        });
    }

    (sections, slides)
}

/// Slide individual de um item de mídia.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slide {
    pub id: String,
    pub label: String,
    pub content: String,
    #[serde(default)]
    pub align_h: Option<String>,
    #[serde(default)]
    pub align_v: Option<String>,
}

impl Slide {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            content: content.into(),
            align_h: None,
            align_v: None,
        }
    }
}

/// Item de mídia na biblioteca do operador (ex.: louvor, texto bíblico, aviso, divisor de seção, imagem).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: String,
    pub title: String,
    pub artist_or_ref: Option<String>,
    pub kind: MediaKind,
    pub slides: Vec<Slide>,
    #[serde(default)]
    pub align_h: Option<String>,
    #[serde(default)]
    pub align_v: Option<String>,
}

/// Pasta de mídias mapeada no sistema de arquivos local do usuário.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaFolder {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(default = "default_media_folder_kind")]
    pub kind: String,
    pub created_at: String,
}

fn default_media_folder_kind() -> String {
    "images".to_string()
}

/// Modelo completo de uma Setlist (repertório ordenado de um culto ou evento).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setlist {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub wallpaper_path: Option<String>,
    pub wallpaper_mode: Option<String>,
    pub items: Vec<MediaItem>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Resumo compacto de uma Setlist para listagem e seleção rápida.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetlistSummary {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub wallpaper_path: Option<String>,
    pub wallpaper_mode: Option<String>,
    pub item_count: usize,
    pub is_active: bool,
    pub updated_at: String,
}

/// Modo de apresentação na tela pública.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    #[default]
    Live,
    Blackout,
    ClearText,
}

impl DisplayMode {
    pub const fn label_pt_br(&self) -> &'static str {
        match self {
            Self::Live => "Ao Vivo",
            Self::Blackout => "Blackout",
            Self::ClearText => "Limpar Texto",
        }
    }
}

/// Estado completo de projeção em tempo real.
/// Estado do cronômetro / timer global compartilhado entre operador e palco.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimerState {
    pub running: bool,
    pub duration_seconds: i64,
    pub remaining_seconds: i64,
    pub label: Option<String>,
    pub stage_message: Option<String>,
    pub warning_threshold_seconds: i64,
    #[serde(default)]
    pub allow_overtime: bool,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            running: false,
            duration_seconds: 600,
            remaining_seconds: 600,
            label: Some("Cronômetro".into()),
            stage_message: None,
            warning_threshold_seconds: 60,
            allow_overtime: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationState {
    pub active_media_id: Option<String>,
    pub active_slide_index: Option<usize>,
    pub display_mode: DisplayMode,
    pub public_connected: bool,
    pub stage_connected: bool,
    #[serde(default)]
    pub active_setlist_id: Option<String>,
    #[serde(default)]
    pub wallpaper_path: Option<String>,
    #[serde(default)]
    pub wallpaper_mode: Option<String>,
    #[serde(default)]
    pub setlist_wallpaper_path: Option<String>,
    #[serde(default)]
    pub setlist_wallpaper_mode: Option<String>,
    #[serde(default)]
    pub lyrics_background_path: Option<String>,
    #[serde(default)]
    pub lyrics_background_mode: Option<String>,
    #[serde(default)]
    pub timer: TimerState,
    #[serde(default)]
    pub quick_media: Option<MediaItem>,
}

impl Default for PresentationState {
    fn default() -> Self {
        Self {
            active_media_id: None,
            active_slide_index: None,
            display_mode: DisplayMode::Live,
            public_connected: false,
            stage_connected: false,
            active_setlist_id: None,
            wallpaper_path: None,
            wallpaper_mode: Some("cover".into()),
            setlist_wallpaper_path: None,
            setlist_wallpaper_mode: Some("cover".into()),
            lyrics_background_path: Some(
                "css:none|linear-gradient(135deg, #022c22 0%, #064e3b 50%, #021a14 100%)".into(),
            ),
            lyrics_background_mode: Some("cover".into()),
            timer: TimerState::default(),
            quick_media: None,
        }
    }
}

/// Mensagem transmitida via WebSocket entre Admin, Public, Stage e servidor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum WsMessage {
    /// Sincronização inicial ou completa enviada pelo backend ao conectar.
    SyncState {
        presentation: PresentationState,
        media_items: Vec<MediaItem>,
    },
    /// Comando para disparar um slide específico ao vivo.
    SelectSlide {
        media_id: String,
        slide_index: usize,
    },
    /// Comando para alterar o modo de exibição (Live, Blackout, ClearText).
    SetDisplayMode {
        mode: DisplayMode,
    },
    /// Comando para parar ou cancelar a apresentação ao vivo.
    StopPresentation,
    /// Comando para definir o papel de parede ou fundo ao vivo da apresentação.
    SetWallpaper {
        path: Option<String>,
        mode: Option<String>,
    },
    /// Comando para iniciar a contagem regressiva do timer.
    StartTimer,
    /// Comando para pausar o timer.
    PauseTimer,
    /// Comando para reiniciar o timer ao seu valor original.
    ResetTimer,
    /// Comando para configurar a duração e rótulo do timer.
    SetTimer {
        duration_seconds: i64,
        label: Option<String>,
        #[serde(default)]
        allow_overtime: Option<bool>,
    },
    /// Comando para alternar permissão de overtime contínuo do timer.
    SetTimerOvertime {
        allow_overtime: bool,
    },
    /// Comando para apresentar mídia avulsa rapidamente sem salvar na setlist permanente.
    QuickPresentMedia {
        item: MediaItem,
        #[serde(default)]
        slide_index: Option<usize>,
    },
    /// Comando para enviar ou limpar mensagem para o palco (Stage Message).
    SetStageMessage {
        message: Option<String>,
    },
    /// Identificação do cliente ao se conectar (admin, public, stage).
    Identify {
        client_type: String,
    },
    /// Notificação de atualização enviada aos clientes.
    StateUpdated {
        presentation: PresentationState,
    },
    /// Notificação de alteração ou reordenação dos itens da setlist.
    MediaItemsUpdated {
        media_items: Vec<MediaItem>,
    },
    /// Notificação de setlist selecionada ou atualizada.
    SetlistUpdated {
        setlist: Setlist,
    },
    /// Notificação de configuração salva/atualizada em tempo real.
    SettingUpdated {
        key: String,
        value: Option<String>,
    },
    /// Comando para forçar recarga das telas conectadas (ex: /public ou /stage).
    ReloadClients {
        #[serde(default)]
        target: Option<String>,
    },
    /// Ping / Heartbeat para manter conexão viva.
    Ping,
    Pong,
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

    #[test]
    fn media_and_presentation_state_serialization() {
        let media = MediaItem {
            id: "song-1".into(),
            title: "Porque Ele Vive".into(),
            artist_or_ref: Some("Harpa Cristã".into()),
            kind: MediaKind::Song,
            slides: vec![Slide::new(
                "s1",
                "Verso 1",
                "Deus enviou Seu Filho amado\nPara morrer em meu lugar",
            )],
            align_h: None,
            align_v: None,
        };

        let json = serde_json::to_string(&media).expect("serializes");
        let parsed: MediaItem = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(parsed, media);

        let ws_msg = WsMessage::SelectSlide {
            media_id: "song-1".into(),
            slide_index: 0,
        };
        let ws_json = serde_json::to_string(&ws_msg).expect("serializes");
        assert!(ws_json.contains("select_slide"));
    }

    #[test]
    fn raw_lyrics_parsing_splits_slides_and_identifies_sections() {
        let raw = r#"
## Verso 1
Deus enviou Seu Filho amado
Para morrer em meu lugar

Na cruz pagou por meus pecados
Mas o sepulcro vazio está

## Refrão
Porque Ele vive, posso crer no amanhã
Porque Ele vive, temor não há
"#;

        let (sections, slides) = parse_raw_lyrics(raw);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].label, "Verso 1");
        assert_eq!(sections[0].section_type, Some(SectionType::Verse));
        assert_eq!(sections[0].lines.len(), 4);

        assert_eq!(sections[1].label, "Refrão");
        assert_eq!(sections[1].section_type, Some(SectionType::Chorus));
        assert_eq!(sections[1].lines.len(), 2);

        // Slides devem ter sido divididos pelas linhas em branco:
        assert_eq!(slides.len(), 3);
        assert_eq!(slides[0].label, "Verso 1");
        assert_eq!(
            slides[0].content,
            "Deus enviou Seu Filho amado\nPara morrer em meu lugar"
        );
        assert_eq!(slides[1].label, "Verso 1");
        assert_eq!(
            slides[1].content,
            "Na cruz pagou por meus pecados\nMas o sepulcro vazio está"
        );
        assert_eq!(slides[2].label, "Refrão");
        assert_eq!(
            slides[2].content,
            "Porque Ele vive, posso crer no amanhã\nPorque Ele vive, temor não há"
        );
    }

    #[test]
    fn section_type_deserialization_is_lenient_and_handles_various_labels() {
        let json = r#"["verso 1", "refrão", "chorus", "ponte", "desconhecido"]"#;
        let types: Vec<SectionType> = serde_json::from_str(json).expect("deserializes safely");
        assert_eq!(types[0], SectionType::Verse);
        assert_eq!(types[1], SectionType::Chorus);
        assert_eq!(types[2], SectionType::Chorus);
        assert_eq!(types[3], SectionType::Bridge);
        assert_eq!(types[4], SectionType::Other);

        // Testa deserialização de Song com seções flexíveis
        let song_json = r#"{
            "id": "song-abc",
            "title": "Teste",
            "artist": "Autor",
            "sections": [
                {
                    "section_type": "verso 1",
                    "label": "Verso 1",
                    "lines": ["linha 1", "linha 2"]
                }
            ],
            "lyrics_raw": "estrofe 1"
        }"#;

        let song: Song = serde_json::from_str(song_json).expect("deserializes song");
        assert_eq!(song.sections.len(), 1);
        assert_eq!(song.sections[0].section_type, Some(SectionType::Verse));
    }
}

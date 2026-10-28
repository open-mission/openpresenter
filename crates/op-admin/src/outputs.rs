//! Controlador das janelas de saída Public e Stage.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use chrono::{DateTime, Datelike, Local, Utc, Weekday};
use op_core::{MonitorSelection, OutputKind, OutputMode, OutputsConfig};
use op_db::Database;
use slint::{
    CloseRequestResponse, ComponentHandle, LogicalSize, PhysicalPosition, PhysicalSize, TimerMode,
    WindowSize,
};

use crate::monitors::{self, EffectiveMode, MonitorInfo, Resolution};
use crate::{PublicWindow, StageWindow};

/// Estado de uma saída exibido na janela de Configurações.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputStatus {
    /// A saída está desativada na configuração.
    Disabled,
    /// A janela está aberta e visível.
    Open,
    /// A janela existe, mas foi ocultada nesta sessão.
    Hidden,
    /// A saída abriu em um monitor alternativo; a mensagem explica o motivo.
    Warning(String),
    /// A janela não pôde ser criada ou mostrada.
    Failed,
}

impl OutputStatus {
    /// Texto em português do Brasil para exibição.
    pub fn text_pt_br(&self) -> String {
        match self {
            Self::Disabled => "Desativada".to_owned(),
            Self::Open => "Aberta".to_owned(),
            Self::Hidden => "Oculta".to_owned(),
            Self::Warning(message) => message.clone(),
            Self::Failed => "Falha ao abrir".to_owned(),
        }
    }

    /// Código do tom visual: 0 neutro, 1 sucesso, 2 aviso, 3 erro.
    pub const fn kind(&self) -> i32 {
        match self {
            Self::Disabled | Self::Hidden => 0,
            Self::Open => 1,
            Self::Warning(_) => 2,
            Self::Failed => 3,
        }
    }
}

/// Janela de saída mantida viva pelo controlador durante toda a sessão.
struct OutputSlot<W: ComponentHandle> {
    window: W,
    visible: Rc<Cell<bool>>,
}

impl<W: ComponentHandle> OutputSlot<W> {
    /// Oculta a janela e registra falhas sem interromper o aplicativo.
    fn hide(&self) {
        self.visible.set(false);
        if let Err(error) = self.window.hide() {
            eprintln!("Não foi possível ocultar uma janela de saída: {error}");
        }
    }
}

/// Controla as janelas Public e Stage, sua geometria e o relógio do palco.
pub struct OutputsController {
    db: Option<Database>,
    database_error: Option<String>,
    config: OutputsConfig,
    monitors: Vec<MonitorInfo>,
    monitor_warning: Option<String>,
    persistence_error: Option<String>,
    public: Option<OutputSlot<PublicWindow>>,
    stage: Option<OutputSlot<StageWindow>>,
    failed: [bool; 2],
    started: bool,
    public_timer: Option<Rc<slint::Timer>>,
    stage_timer: Option<Rc<slint::Timer>>,
}

impl OutputsController {
    /// Cria o controlador carregando a configuração persistida quando possível.
    pub fn new(db: Option<Database>, database_error: Option<String>) -> Self {
        let mut database_error = database_error;
        let config = match db.as_ref() {
            Some(db) => match db.load_outputs() {
                Ok(config) => config,
                Err(error) => {
                    database_error = Some(format!(
                        "Falha ao carregar a configuração das saídas: {error}"
                    ));
                    OutputsConfig::default()
                }
            },
            None => OutputsConfig::default(),
        };

        Self {
            db,
            database_error,
            config,
            monitors: Vec::new(),
            monitor_warning: None,
            persistence_error: None,
            public: None,
            stage: None,
            failed: [false; 2],
            started: false,
            public_timer: None,
            stage_timer: None,
        }
    }

    /// Enumera os monitores na thread principal, sem reposicionar janelas já abertas.
    pub fn refresh_monitors(&mut self) {
        match monitors::list() {
            Ok(list) => {
                let empty = list.is_empty();
                self.set_monitors(list);
                self.monitor_warning = empty.then(|| {
                    "Nenhum monitor detectado — as saídas abrirão na posição padrão do sistema"
                        .to_owned()
                });
            }
            Err(error) => {
                self.set_monitors(Vec::new());
                self.monitor_warning =
                    Some(format!("Não foi possível enumerar os monitores: {error}"));
            }
        }
    }

    /// Substitui a lista de monitores conhecida.
    pub fn set_monitors(&mut self, monitors: Vec<MonitorInfo>) {
        self.monitors = monitors;
    }

    /// Abre uma única vez as saídas habilitadas.
    pub fn start(&mut self) {
        if self.started {
            return;
        }
        self.started = true;

        if self.database_error.is_some() {
            return;
        }

        for kind in OutputKind::ALL {
            if self.config.get(kind).enabled {
                self.open(kind);
            }
        }
    }

    /// Reaplica geometria e modo exclusivamente à saída indicada, se visível.
    pub fn apply_config_for(&mut self, kind: OutputKind) {
        match kind {
            OutputKind::Public => {
                if let Some(slot) = self.public.as_ref().filter(|slot| slot.visible.get()) {
                    let resolution = self.resolution(OutputKind::Public);
                    reposition(OutputKind::Public, &slot.window, &resolution);
                }
            }
            OutputKind::Stage => {
                if let Some(slot) = self.stage.as_ref().filter(|slot| slot.visible.get()) {
                    let resolution = self.resolution(OutputKind::Stage);
                    reposition(OutputKind::Stage, &slot.window, &resolution);
                }
            }
        }
    }

    /// Ativa ou desativa uma saída, persistindo e aplicando a mudança.
    pub fn set_enabled(&mut self, kind: OutputKind, enabled: bool) {
        if self.database_error.is_some() || self.config.get(kind).enabled == enabled {
            return;
        }

        self.config.get_mut(kind).enabled = enabled;
        self.persist(kind);

        if enabled {
            self.open(kind);
        } else {
            self.close(kind);
        }
    }

    /// Define o monitor de uma saída, persistindo e aplicando a mudança.
    pub fn set_monitor(&mut self, kind: OutputKind, selection: MonitorSelection) {
        if self.database_error.is_some() || self.config.get(kind).monitor == selection {
            return;
        }

        self.config.get_mut(kind).monitor = selection;
        self.persist(kind);
        self.apply_config_for(kind);
    }

    /// Define o modo de exibição de uma saída, persistindo e aplicando a mudança.
    pub fn set_mode(&mut self, kind: OutputKind, mode: OutputMode) {
        if self.database_error.is_some() || self.config.get(kind).mode == mode {
            return;
        }

        self.config.get_mut(kind).mode = mode;
        self.persist(kind);
        self.apply_config_for(kind);
    }

    /// Mostra uma saída habilitada, criando a janela se necessário.
    pub fn show_now(&mut self, kind: OutputKind) {
        if self.database_error.is_some() || !self.config.get(kind).enabled {
            return;
        }

        match kind {
            OutputKind::Public => self.show_public_now(),
            OutputKind::Stage => self.show_stage_now(),
        }
    }

    /// Oculta uma saída sem alterar `enabled`.
    pub fn hide(&mut self, kind: OutputKind) {
        match kind {
            OutputKind::Public => {
                if let Some(slot) = self.public.as_ref() {
                    slot.hide();
                }
            }
            OutputKind::Stage => {
                if let Some(slot) = self.stage.as_ref() {
                    slot.hide();
                }
            }
        }
    }

    /// Fecha todas as janelas e descarta seus handles.
    pub fn close_all(&mut self) {
        self.public_timer = None;
        self.stage_timer = None;
        self.failed = [false; 2];

        if let Some(slot) = self.public.take() {
            slot.hide();
        }
        if let Some(slot) = self.stage.take() {
            slot.hide();
        }
    }

    /// Indica se a janela da saída está visível nesta sessão.
    pub fn is_visible(&self, kind: OutputKind) -> bool {
        match kind {
            OutputKind::Public => self.public.as_ref().is_some_and(|slot| slot.visible.get()),
            OutputKind::Stage => self.stage.as_ref().is_some_and(|slot| slot.visible.get()),
        }
    }

    /// Calcula o estado exibido na janela de Configurações.
    pub fn status(&self, kind: OutputKind) -> OutputStatus {
        if self.database_error.is_some() || !self.config.get(kind).enabled {
            return OutputStatus::Disabled;
        }

        if self.failed[output_index(kind)] {
            return OutputStatus::Failed;
        }

        let resolution = self.resolution(kind);

        let visible = match kind {
            OutputKind::Public => self.public.as_ref().map(|slot| slot.visible.get()),
            OutputKind::Stage => self.stage.as_ref().map(|slot| slot.visible.get()),
        };

        if let Some(visible) = visible {
            if visible {
                return match resolution.warning {
                    Some(warning) => OutputStatus::Warning(warning.message_pt_br().to_owned()),
                    None => OutputStatus::Open,
                };
            }
            return OutputStatus::Hidden;
        }

        match resolution.warning {
            Some(warning) => OutputStatus::Warning(warning.message_pt_br().to_owned()),
            None => OutputStatus::Failed,
        }
    }

    /// Converte a posição do seletor em uma preferência de monitor.
    pub fn selection_for_index(&self, index: i32) -> MonitorSelection {
        match index {
            0 => MonitorSelection::Auto,
            1 => MonitorSelection::Primary,
            _ => index
                .checked_sub(2)
                .and_then(|position| usize::try_from(position).ok())
                .and_then(|position| self.monitors.get(position))
                .map_or(MonitorSelection::Auto, |monitor| {
                    MonitorSelection::Display {
                        id: monitor.key.clone(),
                        name: monitor.name.clone(),
                    }
                }),
        }
    }

    /// Converte a preferência de monitor em uma posição do seletor.
    pub fn monitor_index(&self, kind: OutputKind) -> i32 {
        match &self.config.get(kind).monitor {
            MonitorSelection::Auto => 0,
            MonitorSelection::Primary => 1,
            MonitorSelection::Display { id, name } => self
                .monitors
                .iter()
                .position(|monitor| monitor.key.as_str() == id.as_str())
                .or_else(|| {
                    self.monitors
                        .iter()
                        .position(|monitor| monitor.name.as_str() == name.as_str())
                })
                .map_or(0, |position| position as i32 + 2),
        }
    }

    /// Indica se o relógio do Stage está ativo. Usado apenas nos testes.
    #[cfg(test)]
    pub fn has_stage_timer(&self) -> bool {
        self.stage_timer.is_some()
    }

    /// Indica se o sincronizador do Public está ativo. Usado apenas nos testes.
    #[cfg(test)]
    pub fn has_public_timer(&self) -> bool {
        self.public_timer.is_some()
    }

    /// Configuração em memória das duas saídas.
    pub fn config(&self) -> &OutputsConfig {
        &self.config
    }

    /// Monitores conhecidos, na ordem de enumeração.
    pub fn monitors(&self) -> &[MonitorInfo] {
        &self.monitors
    }

    /// Aviso global de enumeração de monitores, quando houver.
    pub fn monitor_warning(&self) -> Option<&str> {
        self.monitor_warning.as_deref()
    }

    /// Erro que impede carregar ou gravar a configuração no banco.
    pub fn database_error(&self) -> Option<&str> {
        self.database_error.as_deref()
    }

    /// Falha da última tentativa de gravação, mantendo a configuração em memória.
    pub fn persistence_error(&self) -> Option<&str> {
        self.persistence_error.as_deref()
    }

    fn resolution(&self, kind: OutputKind) -> Resolution {
        let config = self.config.get(kind);
        monitors::resolve(kind, &config.monitor, config.mode, &self.monitors)
    }

    fn open(&mut self, kind: OutputKind) {
        if self.database_error.is_some() || !self.config.get(kind).enabled {
            return;
        }

        match kind {
            OutputKind::Public => self.open_public(),
            OutputKind::Stage => self.open_stage(),
        }
    }

    fn open_public(&mut self) {
        if self.public.is_some() {
            return;
        }

        let resolution = self.resolution(OutputKind::Public);
        let window = match PublicWindow::new() {
            Ok(window) => window,
            Err(error) => {
                self.mark_failed(OutputKind::Public, &error);
                return;
            }
        };

        let visible = Rc::new(Cell::new(true));
        let flag = visible.clone();
        window.window().on_close_requested(move || {
            flag.set(false);
            CloseRequestResponse::HideWindow
        });

        apply_geometry(OutputKind::Public, &window, &resolution);
        if let Err(error) = window.show() {
            self.mark_failed(OutputKind::Public, &error);
            return;
        }
        apply_mode(&window, resolution.mode);

        self.failed[output_index(OutputKind::Public)] = false;
        self.start_public_timer(&window);
        self.public = Some(OutputSlot { window, visible });
    }

    fn open_stage(&mut self) {
        if self.stage.is_some() {
            return;
        }

        let resolution = self.resolution(OutputKind::Stage);
        let window = match StageWindow::new() {
            Ok(window) => window,
            Err(error) => {
                self.mark_failed(OutputKind::Stage, &error);
                return;
            }
        };

        let now = Local::now();
        window.set_clock_time(now.format("%H:%M:%S").to_string().into());
        window.set_date_label(format_date_pt_br(&now).into());

        let visible = Rc::new(Cell::new(true));
        let flag = visible.clone();
        window.window().on_close_requested(move || {
            flag.set(false);
            CloseRequestResponse::HideWindow
        });

        apply_geometry(OutputKind::Stage, &window, &resolution);
        if let Err(error) = window.show() {
            self.mark_failed(OutputKind::Stage, &error);
            return;
        }
        apply_mode(&window, resolution.mode);

        self.failed[output_index(OutputKind::Stage)] = false;
        self.start_stage_timer(&window);
        self.stage = Some(OutputSlot { window, visible });
    }

    fn show_public_now(&mut self) {
        if self.public.is_none() {
            self.open_public();
            return;
        }

        let result = self.public.as_ref().map(|slot| slot.window.show());
        match result {
            Some(Ok(())) => {
                if let Some(slot) = self.public.as_ref() {
                    slot.visible.set(true);
                }
                self.apply_config_for(OutputKind::Public);
            }
            Some(Err(error)) => {
                self.public = None;
                self.mark_failed(OutputKind::Public, &error);
            }
            None => {}
        }
    }

    fn show_stage_now(&mut self) {
        if self.stage.is_none() {
            self.open_stage();
            return;
        }

        let result = self.stage.as_ref().map(|slot| slot.window.show());
        match result {
            Some(Ok(())) => {
                if let Some(slot) = self.stage.as_ref() {
                    slot.visible.set(true);
                }
                self.apply_config_for(OutputKind::Stage);
            }
            Some(Err(error)) => {
                self.stage_timer = None;
                self.stage = None;
                self.mark_failed(OutputKind::Stage, &error);
            }
            None => {}
        }
    }

    fn close(&mut self, kind: OutputKind) {
        self.failed[output_index(kind)] = false;

        match kind {
            OutputKind::Public => {
                self.public_timer = None;
                if let Some(slot) = self.public.take() {
                    slot.hide();
                }
            }
            OutputKind::Stage => {
                self.stage_timer = None;
                if let Some(slot) = self.stage.take() {
                    slot.hide();
                }
            }
        }
    }

    fn start_public_timer(&mut self, window: &PublicWindow) {
        let weak = window.as_weak();
        let timer = Rc::new(slint::Timer::default());
        let timer_handle = Rc::downgrade(&timer);

        timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
            let Some(window) = weak.upgrade() else {
                if let Some(timer) = timer_handle.upgrade() {
                    timer.stop();
                }
                return;
            };

            sync_viewport_to_window(&window);
        });

        self.public_timer = Some(timer);
    }

    fn start_stage_timer(&mut self, window: &StageWindow) {
        let weak = window.as_weak();
        let timer = Rc::new(slint::Timer::default());
        let timer_handle = Rc::downgrade(&timer);

        timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
            let Some(window) = weak.upgrade() else {
                // A janela foi descartada; encerra o relógio em vez de mantê-lo girando.
                if let Some(timer) = timer_handle.upgrade() {
                    timer.stop();
                }
                return;
            };

            sync_viewport_to_window(&window);

            let now = Local::now();
            window.set_clock_time(now.format("%H:%M:%S").to_string().into());
            window.set_date_label(format_date_pt_br(&now).into());
        });

        self.stage_timer = Some(timer);
    }

    fn persist(&mut self, kind: OutputKind) {
        let Some(db) = self.db.as_ref() else {
            // Sem banco e sem erro registrado: a sessão apenas mantém a configuração em memória.
            return;
        };

        let config = self.config.get(kind).clone();
        match db.save_output(&config, Utc::now()) {
            Ok(()) => self.persistence_error = None,
            Err(error) => {
                self.persistence_error =
                    Some(format!("Falha ao salvar a configuração da saída: {error}"));
            }
        }
    }

    fn mark_failed(&mut self, kind: OutputKind, error: &slint::PlatformError) {
        self.failed[output_index(kind)] = true;
        eprintln!(
            "Não foi possível abrir a saída {}: {error}",
            kind.label_pt_br()
        );
    }
}

const fn output_index(kind: OutputKind) -> usize {
    match kind {
        OutputKind::Public => 0,
        OutputKind::Stage => 1,
    }
}

pub(crate) trait OutputWindow: ComponentHandle {
    fn set_viewport(&self, width: f32, height: f32);
    fn get_viewport(&self) -> (f32, f32);
}

impl OutputWindow for PublicWindow {
    fn set_viewport(&self, width: f32, height: f32) {
        self.set_viewport_width(width);
        self.set_viewport_height(height);
    }

    fn get_viewport(&self) -> (f32, f32) {
        (self.get_viewport_width(), self.get_viewport_height())
    }
}

impl OutputWindow for StageWindow {
    fn set_viewport(&self, width: f32, height: f32) {
        self.set_viewport_width(width);
        self.set_viewport_height(height);
    }

    fn get_viewport(&self) -> (f32, f32) {
        (self.get_viewport_width(), self.get_viewport_height())
    }
}

fn sync_viewport_to_window<W: OutputWindow>(window: &W) {
    let size = window.window().size();
    let scale = window.window().scale_factor();
    if scale > 0.0 && size.width > 0 && size.height > 0 {
        let logical_w = size.width as f32 / scale;
        let logical_h = size.height as f32 / scale;
        let (current_w, current_h) = window.get_viewport();
        if (current_w - logical_w).abs() > 1.0 || (current_h - logical_h).abs() > 1.0 {
            window.set_viewport(logical_w, logical_h);
        }
    }
}

fn apply_geometry<W: OutputWindow>(kind: OutputKind, window: &W, resolution: &Resolution) {
    let target = window.window();
    let (logical_width, logical_height) = monitors::logical_size(kind);

    let Some(monitor) = resolution.monitor.as_ref() else {
        window.set_viewport(logical_width, logical_height);
        target.set_size(WindowSize::Logical(LogicalSize::new(
            logical_width,
            logical_height,
        )));
        return;
    };

    match resolution.mode {
        EffectiveMode::Windowed => {
            window.set_viewport(logical_width, logical_height);
            let cascade = if monitor.is_primary {
                monitors::cascade_offset(kind)
            } else {
                0.0
            };
            let placed = monitors::placement(monitor, logical_width, logical_height, cascade);
            target.set_size(WindowSize::Physical(PhysicalSize::new(
                placed.width,
                placed.height,
            )));
            target.set_position(PhysicalPosition::new(placed.x, placed.y));
        }
        EffectiveMode::Maximized | EffectiveMode::Fullscreen => {
            let scale = if monitor.scale_factor > 0.0 {
                monitor.scale_factor
            } else {
                1.0
            };
            let screen_logical_w = monitor.width as f32 / scale;
            let screen_logical_h = monitor.height as f32 / scale;
            window.set_viewport(screen_logical_w, screen_logical_h);

            target.set_size(WindowSize::Physical(PhysicalSize::new(
                monitor.width,
                monitor.height,
            )));
            target.set_position(PhysicalPosition::new(monitor.x, monitor.y));
        }
    }
}

fn apply_mode<W: ComponentHandle>(window: &W, mode: EffectiveMode) {
    let target = window.window();
    match mode {
        EffectiveMode::Windowed => {}
        EffectiveMode::Maximized => {
            if !target.is_maximized() {
                target.set_maximized(true);
            }
        }
        EffectiveMode::Fullscreen => {
            if !target.is_fullscreen() {
                target.set_fullscreen(true);
            }
        }
    }
}

fn reposition<W: OutputWindow>(kind: OutputKind, window: &W, resolution: &Resolution) {
    let target = window.window();
    let is_fullscreen = target.is_fullscreen();
    let is_maximized = target.is_maximized();
    let should_fullscreen = resolution.mode == EffectiveMode::Fullscreen;
    let should_maximized = resolution.mode == EffectiveMode::Maximized;

    // Se a janela já está no estado desejado (tela cheia ou maximizada), apenas atualiza a geometria sem disparar transições redundantes
    if is_fullscreen && should_fullscreen {
        apply_geometry(kind, window, resolution);
        return;
    }
    if is_maximized && should_maximized {
        apply_geometry(kind, window, resolution);
        return;
    }

    if is_fullscreen && !should_fullscreen {
        target.set_fullscreen(false);
    }
    if is_maximized && !should_maximized {
        target.set_maximized(false);
    }

    apply_geometry(kind, window, resolution);

    if !is_fullscreen && should_fullscreen {
        target.set_fullscreen(true);
    } else if !is_maximized && should_maximized {
        target.set_maximized(true);
    }
}

/// Data por extenso em português do Brasil, sem depender do locale do sistema.
fn format_date_pt_br(date: &DateTime<Local>) -> String {
    format!(
        "{}, {} de {} de {}",
        weekday_pt_br(date.weekday()),
        date.day(),
        month_pt_br(date.month()),
        date.year()
    )
}

const fn weekday_pt_br(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "segunda-feira",
        Weekday::Tue => "terça-feira",
        Weekday::Wed => "quarta-feira",
        Weekday::Thu => "quinta-feira",
        Weekday::Fri => "sexta-feira",
        Weekday::Sat => "sábado",
        Weekday::Sun => "domingo",
    }
}

const fn month_pt_br(month: u32) -> &'static str {
    match month {
        1 => "janeiro",
        2 => "fevereiro",
        3 => "março",
        4 => "abril",
        5 => "maio",
        6 => "junho",
        7 => "julho",
        8 => "agosto",
        9 => "setembro",
        10 => "outubro",
        11 => "novembro",
        _ => "dezembro",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_monitor() -> MonitorInfo {
        MonitorInfo {
            key: "1".into(),
            name: "Built-in".into(),
            label: "Built-in — 1920×1080 (principal)".into(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            scale_factor: 1.0,
            is_primary: true,
            is_builtin: true,
        }
    }

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
        assert!(controller.has_public_timer());
        assert!(!controller.is_visible(OutputKind::Stage));
        assert!(!controller.has_stage_timer());
        assert_eq!(controller.status(OutputKind::Public).text_pt_br(), "Aberta");
        assert_eq!(
            controller.status(OutputKind::Stage).text_pt_br(),
            "Desativada"
        );

        controller.set_enabled(OutputKind::Stage, true);
        assert!(controller.is_visible(OutputKind::Stage));
        assert!(controller.has_stage_timer());

        controller.hide(OutputKind::Public);
        assert!(!controller.is_visible(OutputKind::Public));
        assert!(controller.config().public.enabled);
        assert_eq!(controller.status(OutputKind::Public).text_pt_br(), "Oculta");

        controller.set_monitor(
            OutputKind::Public,
            MonitorSelection::Display {
                id: "missing".into(),
                name: "Missing".into(),
            },
        );
        controller.show_now(OutputKind::Public);
        assert!(controller.is_visible(OutputKind::Public));
        assert!(matches!(
            controller.status(OutputKind::Public),
            OutputStatus::Warning(_)
        ));

        controller.set_enabled(OutputKind::Public, false);
        assert!(!controller.is_visible(OutputKind::Public));
        assert!(!controller.has_public_timer());
        assert!(!controller.config().public.enabled);
        assert_eq!(
            controller.status(OutputKind::Public).text_pt_br(),
            "Desativada"
        );

        controller.set_enabled(OutputKind::Stage, false);
        assert!(!controller.has_stage_timer());
        controller.close_all();
        assert!(!controller.has_public_timer());
        assert!(!controller.has_stage_timer());

        // Verifica que apply_geometry define o viewport para tela cheia e modo janela
        let stage_window = StageWindow::new().unwrap();
        let monitor = MonitorInfo {
            key: "2".into(),
            name: "Retina 4K".into(),
            label: "Retina 4K — 3840×2160".into(),
            x: 0,
            y: 0,
            width: 3840,
            height: 2160,
            scale_factor: 2.0,
            is_primary: false,
            is_builtin: false,
        };

        let res_windowed = Resolution {
            monitor: Some(monitor.clone()),
            mode: EffectiveMode::Windowed,
            warning: None,
        };
        apply_geometry(OutputKind::Stage, &stage_window, &res_windowed);
        assert_eq!(stage_window.get_viewport(), (720.0, 480.0));

        let res_fullscreen = Resolution {
            monitor: Some(monitor),
            mode: EffectiveMode::Fullscreen,
            warning: None,
        };
        apply_geometry(OutputKind::Stage, &stage_window, &res_fullscreen);
        assert_eq!(stage_window.get_viewport(), (1920.0, 1080.0));

        let mut degraded = OutputsController::new(None, Some("Banco indisponível".into()));
        degraded.start();
        assert!(!degraded.is_visible(OutputKind::Public));
        assert_eq!(
            degraded.status(OutputKind::Public).text_pt_br(),
            "Desativada"
        );
    }

    #[test]
    fn missing_saved_monitor_keeps_the_preference_and_warns() {
        let mut controller = OutputsController::new(None, None);
        controller.set_monitors(vec![test_monitor()]);
        let saved = MonitorSelection::Display {
            id: "missing".into(),
            name: "Missing".into(),
        };
        controller.set_monitor(OutputKind::Public, saved.clone());

        assert_eq!(controller.config().public.monitor, saved);
        assert_eq!(controller.monitor_index(OutputKind::Public), 0);
        assert!(matches!(
            controller.status(OutputKind::Public),
            OutputStatus::Warning(_)
        ));
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
    fn apply_config_updates_visible_windows_without_panicking() {
        let mut controller = OutputsController::new(None, None);
        controller.set_monitors(vec![test_monitor()]);
        controller.apply_config_for(OutputKind::Public);
        controller.apply_config_for(OutputKind::Stage);
    }

    #[test]
    fn stage_date_is_in_portuguese() {
        use chrono::TimeZone;
        let date = chrono::Local
            .with_ymd_and_hms(2026, 10, 28, 9, 30, 0)
            .single()
            .unwrap();
        assert_eq!(
            format_date_pt_br(&date),
            "quarta-feira, 28 de outubro de 2026"
        );
    }
}

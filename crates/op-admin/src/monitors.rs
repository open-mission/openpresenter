//! Enumeração de monitores e geometria das janelas de saída.

use display_info::DisplayInfo;
use op_core::{MonitorSelection, OutputKind, OutputMode};

/// Um monitor disponível para posicionar as saídas.
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorInfo {
    /// Identidade estável do monitor (o `id` informado pelo sistema).
    pub key: String,
    /// Nome curto do monitor.
    pub name: String,
    /// Rótulo para exibição: nome, resolução e indicação de principal.
    pub label: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
    pub is_builtin: bool,
}

/// Enumera os monitores conectados em ordem de posição horizontal.
pub fn list() -> Result<Vec<MonitorInfo>, String> {
    let displays = DisplayInfo::all().map_err(|error| error.to_string())?;
    let space = coordinate_space();

    let mut monitors = displays
        .into_iter()
        .map(|display| {
            let DisplayInfo {
                id,
                name,
                friendly_name,
                x,
                y,
                width,
                height,
                scale_factor,
                is_primary,
                is_builtin,
                ..
            } = display;
            let (x, y, width, height) = normalize_rect(x, y, width, height, scale_factor, space);
            let name = if friendly_name.is_empty() {
                name
            } else {
                friendly_name
            };
            let mut label = format!("{name} — {width}×{height}");
            if is_primary {
                label.push_str(" (principal)");
            }
            MonitorInfo {
                key: id.to_string(),
                name,
                label,
                x,
                y,
                width,
                height,
                scale_factor,
                is_primary,
                is_builtin,
            }
        })
        .collect::<Vec<_>>();

    monitors.sort_by_key(|monitor| (monitor.x, monitor.y));
    Ok(monitors)
}

/// Espaço de coordenadas informado pela enumeração do sistema.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum CoordinateSpace {
    Logical,
    Physical,
}

/// O `display-info` devolve coordenadas físicas no Windows e lógicas nas demais plataformas.
pub(crate) fn coordinate_space() -> CoordinateSpace {
    if cfg!(windows) {
        CoordinateSpace::Physical
    } else {
        CoordinateSpace::Logical
    }
}

/// Converte um retângulo para pixels físicos, arredondando para o inteiro mais próximo.
pub(crate) fn normalize_rect(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scale_factor: f32,
    space: CoordinateSpace,
) -> (i32, i32, u32, u32) {
    if space == CoordinateSpace::Physical {
        return (x, y, width, height);
    }

    let scale = valid_scale(scale_factor);
    (
        (x as f32 * scale).round() as i32,
        (y as f32 * scale).round() as i32,
        (width as f32 * scale).round() as u32,
        (height as f32 * scale).round() as u32,
    )
}

/// Trata escalas não finitas, zero ou negativas como `1.0`.
fn valid_scale(scale_factor: f32) -> f32 {
    if scale_factor.is_finite() && scale_factor > 0.0 {
        scale_factor
    } else {
        1.0
    }
}

/// Modo com que a janela realmente abre, depois de resolvidas as preferências.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EffectiveMode {
    Windowed,
    Maximized,
    Fullscreen,
}

/// Aviso produzido ao resolver o monitor de uma saída.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MonitorWarning {
    NoMonitors,
    MissingDisplay { fallback_primary: bool },
}

impl MonitorWarning {
    /// Mensagem em português do Brasil para exibição na interface.
    pub const fn message_pt_br(&self) -> &'static str {
        match self {
            Self::NoMonitors => "Nenhum monitor detectado — usando a posição padrão",
            Self::MissingDisplay {
                fallback_primary: true,
            } => "Monitor indisponível — usando o principal",
            Self::MissingDisplay {
                fallback_primary: false,
            } => "Monitor indisponível — usando o primeiro monitor disponível",
        }
    }
}

/// Monitor e modo efetivos de uma saída.
#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
    pub monitor: Option<MonitorInfo>,
    pub mode: EffectiveMode,
    pub warning: Option<MonitorWarning>,
}

/// Resolve o monitor e o modo efetivo de uma saída.
pub fn resolve(
    kind: OutputKind,
    selection: &MonitorSelection,
    mode: OutputMode,
    monitors: &[MonitorInfo],
) -> Resolution {
    if monitors.is_empty() {
        return Resolution {
            monitor: None,
            mode: EffectiveMode::Windowed,
            warning: Some(MonitorWarning::NoMonitors),
        };
    }

    let (monitor, warning) = match selection {
        MonitorSelection::Auto => {
            let monitor = match kind {
                OutputKind::Public => monitors
                    .iter()
                    .find(|monitor| !monitor.is_primary && !monitor.is_builtin)
                    .or_else(|| monitors.iter().find(|monitor| monitor.is_primary))
                    .unwrap_or(&monitors[0]),
                OutputKind::Stage => {
                    let externals: Vec<&MonitorInfo> = monitors
                        .iter()
                        .filter(|monitor| !monitor.is_primary && !monitor.is_builtin)
                        .collect();
                    if externals.len() >= 2 {
                        externals[1]
                    } else {
                        externals
                            .first()
                            .copied()
                            .or_else(|| monitors.iter().find(|monitor| monitor.is_primary))
                            .unwrap_or(&monitors[0])
                    }
                }
            };
            (monitor, None)
        }
        MonitorSelection::Primary => {
            let monitor = monitors
                .iter()
                .find(|monitor| monitor.is_primary)
                .unwrap_or(&monitors[0]);
            (monitor, None)
        }
        MonitorSelection::Display { id, name } => {
            let found = monitors
                .iter()
                .find(|monitor| monitor.key.as_str() == id.as_str())
                .or_else(|| {
                    monitors
                        .iter()
                        .find(|monitor| monitor.name.as_str() == name.as_str())
                });
            match found {
                Some(monitor) => (monitor, None),
                None => match monitors.iter().find(|monitor| monitor.is_primary) {
                    Some(monitor) => (
                        monitor,
                        Some(MonitorWarning::MissingDisplay {
                            fallback_primary: true,
                        }),
                    ),
                    None => (
                        &monitors[0],
                        Some(MonitorWarning::MissingDisplay {
                            fallback_primary: false,
                        }),
                    ),
                },
            }
        }
    };

    Resolution {
        monitor: Some(monitor.clone()),
        mode: effective_mode(kind, mode, Some(monitor)),
        warning,
    }
}

/// Determina o modo efetivo de uma saída para o monitor escolhido.
pub fn effective_mode(
    kind: OutputKind,
    mode: OutputMode,
    monitor: Option<&MonitorInfo>,
) -> EffectiveMode {
    let Some(monitor) = monitor else {
        return EffectiveMode::Windowed;
    };

    match mode {
        OutputMode::Auto => {
            if kind == OutputKind::Public && !monitor.is_primary {
                EffectiveMode::Fullscreen
            } else {
                EffectiveMode::Windowed
            }
        }
        OutputMode::Windowed => EffectiveMode::Windowed,
        OutputMode::Maximized => EffectiveMode::Maximized,
        OutputMode::Fullscreen => EffectiveMode::Fullscreen,
    }
}

/// Posição e tamanho físicos da janela de uma saída.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPlacement {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Calcula a geometria física da janela dentro do monitor.
pub fn placement(
    monitor: &MonitorInfo,
    logical_width: f32,
    logical_height: f32,
    cascade_logical: f32,
) -> WindowPlacement {
    let scale = valid_scale(monitor.scale_factor);
    let monitor_width = monitor.width.max(1);
    let monitor_height = monitor.height.max(1);

    let width = to_pixels(logical_width * scale).min(monitor_width);
    let height = to_pixels(logical_height * scale).min(monitor_height);
    let cascade = (cascade_logical * scale).round() as i32;

    let monitor_width = monitor_width as i32;
    let monitor_height = monitor_height as i32;
    let centered_x = monitor.x + (monitor_width - width as i32) / 2;
    let centered_y = monitor.y + (monitor_height - height as i32) / 2;

    WindowPlacement {
        x: (centered_x + cascade).clamp(monitor.x, monitor.x + monitor_width - width as i32),
        y: (centered_y + cascade).clamp(monitor.y, monitor.y + monitor_height - height as i32),
        width,
        height,
    }
}

/// Converte uma dimensão lógica em pixels físicos, com mínimo de 1 px.
fn to_pixels(logical: f32) -> u32 {
    (logical.round().max(1.0)) as u32
}

/// Tamanho lógico padrão da janela de cada saída.
pub const fn logical_size(kind: OutputKind) -> (f32, f32) {
    match kind {
        OutputKind::Public => (960.0, 540.0),
        OutputKind::Stage => (720.0, 480.0),
    }
}

/// Deslocamento lógico em cascata usado quando a saída abre no monitor principal.
pub const fn cascade_offset(kind: OutputKind) -> f32 {
    match kind {
        OutputKind::Public => 64.0,
        OutputKind::Stage => 128.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use op_core::{MonitorSelection, OutputKind, OutputMode};

    fn monitor(key: &str, name: &str, x: i32, primary: bool, builtin: bool) -> MonitorInfo {
        MonitorInfo {
            key: key.into(),
            name: name.into(),
            label: format!(
                "{name} — 1920×1080{}",
                if primary { " (principal)" } else { "" }
            ),
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
    fn auto_distributes_public_and_stage_across_multiple_external_monitors() {
        let builtin = monitor("1", "Built-in", 0, true, true);
        let ext1 = monitor("2", "Projetor", 1920, false, false);
        let ext2 = monitor("3", "TV Palco", 3840, false, false);
        let monitors = vec![builtin, ext1, ext2];

        let public_res = resolve(
            OutputKind::Public,
            &MonitorSelection::Auto,
            OutputMode::Auto,
            &monitors,
        );
        let stage_res = resolve(
            OutputKind::Stage,
            &MonitorSelection::Auto,
            OutputMode::Auto,
            &monitors,
        );

        assert_eq!(public_res.monitor.unwrap().key, "2");
        assert_eq!(stage_res.monitor.unwrap().key, "3");
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
            &MonitorSelection::Display {
                id: "2".into(),
                name: "DELL".into(),
            },
            OutputMode::Windowed,
            &monitors,
        );
        assert_eq!(by_id.monitor.unwrap().key, "2");

        let by_name = resolve(
            OutputKind::Public,
            &MonitorSelection::Display {
                id: "missing".into(),
                name: "DELL".into(),
            },
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
            &MonitorSelection::Display {
                id: "missing".into(),
                name: "Missing".into(),
            },
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
            &MonitorSelection::Display {
                id: "missing".into(),
                name: "Missing".into(),
            },
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
        assert_eq!(
            (centered.x, centered.y, centered.width, centered.height),
            (960, 540, 1920, 1080)
        );

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
        assert_eq!(
            (clamped.x, clamped.y, clamped.width, clamped.height),
            (0, 30, 800, 540)
        );
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
}

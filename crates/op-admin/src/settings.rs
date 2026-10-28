//! Estado e callbacks da janela de Configurações.

use std::cell::RefCell;
use std::rc::Rc;

use op_core::{MonitorSelection, OutputKind, OutputMode};
use slint::{ComponentHandle, ModelRc};

use crate::monitors::MonitorInfo;
use crate::outputs::OutputsController;
use crate::{MenuItem, SettingsWindow};

/// Abre a janela de Configurações, criando-a na primeira vez e reutilizando o handle depois.
pub fn open(
    controller: Rc<RefCell<OutputsController>>,
    slot: Rc<RefCell<Option<SettingsWindow>>>,
) -> Result<(), slint::PlatformError> {
    controller.borrow_mut().refresh_monitors();

    {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            let window = SettingsWindow::new()?;
            wire(&window, Rc::clone(&controller));
            *slot = Some(window);
        }
    }

    let slot = slot.borrow();
    let window = slot.as_ref().expect("janela de Configurações criada acima");
    refresh(window, &controller.borrow());
    window.show()
}

/// Esconde a janela de Configurações sem destruir o handle.
pub fn hide(slot: &Rc<RefCell<Option<SettingsWindow>>>) {
    let slot = slot.borrow();
    let Some(window) = slot.as_ref() else {
        return;
    };

    if let Err(error) = window.hide() {
        eprintln!("Não foi possível ocultar a janela de Configurações: {error}");
    }
}

/// Atualiza estados e visibilidade apenas quando a janela está aberta.
pub fn refresh_if_open(slot: &Rc<RefCell<Option<SettingsWindow>>>, controller: &OutputsController) {
    let slot = slot.borrow();
    let Some(window) = slot.as_ref() else {
        return;
    };

    if !window.window().is_visible() {
        return;
    }

    refresh_status(window, controller);
}

/// Atualiza modelos, índices, estados e avisos da janela.
pub fn refresh(window: &SettingsWindow, controller: &OutputsController) {
    let monitors = monitor_items(controller.monitors());
    let modes = mode_items();

    window.set_public_monitor_items(ModelRc::from(monitors.as_slice()));
    window.set_stage_monitor_items(ModelRc::from(monitors.as_slice()));
    window.set_mode_items(ModelRc::from(modes.as_slice()));

    window.set_public_monitor_index(controller.monitor_index(OutputKind::Public));
    window.set_stage_monitor_index(controller.monitor_index(OutputKind::Stage));
    window.set_public_mode_index(mode_index(controller.config().get(OutputKind::Public).mode));
    window.set_stage_mode_index(mode_index(controller.config().get(OutputKind::Stage).mode));
    window.set_public_enabled(controller.config().get(OutputKind::Public).enabled);
    window.set_stage_enabled(controller.config().get(OutputKind::Stage).enabled);
    window.set_single_monitor(controller.monitors().len() == 1);
    window.set_controls_enabled(controller.database_error().is_none());

    refresh_status(window, controller);
}

/// Atualiza somente estados, visibilidade e erros, sem recriar modelos nem fechar dropdowns.
pub fn refresh_status(window: &SettingsWindow, controller: &OutputsController) {
    let public = controller.status(OutputKind::Public);
    let stage = controller.status(OutputKind::Stage);

    window.set_public_status(public.text_pt_br().into());
    window.set_stage_status(stage.text_pt_br().into());
    window.set_public_status_kind(public.kind());
    window.set_stage_status_kind(stage.kind());
    window.set_public_visible(controller.is_visible(OutputKind::Public));
    window.set_stage_visible(controller.is_visible(OutputKind::Stage));
    window.set_monitor_warning(controller.monitor_warning().unwrap_or_default().into());
    window.set_database_error(controller.database_error().unwrap_or_default().into());
    window.set_persistence_error(controller.persistence_error().unwrap_or_default().into());
}

/// Liga os callbacks da janela ao controlador.
fn wire(window: &SettingsWindow, controller: Rc<RefCell<OutputsController>>) {
    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_public_enabled_changed(move |enabled| {
            update(&weak, &controller, |controller| {
                controller.set_enabled(OutputKind::Public, enabled);
            });
        });
    }

    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_stage_enabled_changed(move |enabled| {
            update(&weak, &controller, |controller| {
                controller.set_enabled(OutputKind::Stage, enabled);
            });
        });
    }

    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_public_monitor_selected(move |index| {
            update(&weak, &controller, |controller| {
                let selection = controller.selection_for_index(index);
                controller.set_monitor(OutputKind::Public, selection);
            });
        });
    }

    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_stage_monitor_selected(move |index| {
            update(&weak, &controller, |controller| {
                let selection = controller.selection_for_index(index);
                controller.set_monitor(OutputKind::Stage, selection);
            });
        });
    }

    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_public_mode_selected(move |index| {
            update(&weak, &controller, |controller| {
                controller.set_mode(OutputKind::Public, mode_from_index(index));
            });
        });
    }

    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_stage_mode_selected(move |index| {
            update(&weak, &controller, |controller| {
                controller.set_mode(OutputKind::Stage, mode_from_index(index));
            });
        });
    }

    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_public_visibility_toggled(move || {
            update(&weak, &controller, |controller| {
                if controller.is_visible(OutputKind::Public) {
                    controller.hide(OutputKind::Public);
                } else {
                    controller.show_now(OutputKind::Public);
                }
            });
        });
    }

    {
        let weak = window.as_weak();
        let controller = Rc::clone(&controller);
        window.on_stage_visibility_toggled(move || {
            update(&weak, &controller, |controller| {
                if controller.is_visible(OutputKind::Stage) {
                    controller.hide(OutputKind::Stage);
                } else {
                    controller.show_now(OutputKind::Stage);
                }
            });
        });
    }

    {
        let weak = window.as_weak();
        window.on_close_requested(move || {
            if let Some(window) = weak.upgrade()
                && let Err(error) = window.hide()
            {
                eprintln!("Não foi possível ocultar a janela de Configurações: {error}");
            }
        });
    }
}

/// Executa uma alteração e atualiza a janela depois de liberar o empréstimo do controlador.
fn update(
    weak: &slint::Weak<SettingsWindow>,
    controller: &Rc<RefCell<OutputsController>>,
    action: impl FnOnce(&mut OutputsController),
) {
    {
        let Ok(mut controller) = controller.try_borrow_mut() else {
            // Evento reentrante disparado pelo próprio `refresh`; o valor já veio do controlador.
            return;
        };
        action(&mut controller);
    }

    let Some(window) = weak.upgrade() else {
        return;
    };
    refresh(&window, &controller.borrow());
}

/// Monta os itens do seletor de monitores na ordem esperada pela janela.
fn monitor_items(monitors: &[MonitorInfo]) -> Vec<MenuItem> {
    let mut items = vec![
        menu_item(MonitorSelection::Auto.label_pt_br()),
        menu_item(MonitorSelection::Primary.label_pt_br()),
    ];
    items.extend(monitors.iter().map(|monitor| menu_item(&monitor.label)));
    items
}

/// Monta os itens do seletor de modos na ordem de `OutputMode::ALL`.
fn mode_items() -> Vec<MenuItem> {
    OutputMode::ALL
        .iter()
        .map(|mode| menu_item(mode.label_pt_br()))
        .collect()
}

fn menu_item(text: &str) -> MenuItem {
    MenuItem {
        text: text.into(),
        enabled: true,
        ..Default::default()
    }
}

fn mode_index(mode: OutputMode) -> i32 {
    match mode {
        OutputMode::Auto => 0,
        OutputMode::Windowed => 1,
        OutputMode::Maximized => 2,
        OutputMode::Fullscreen => 3,
    }
}

fn mode_from_index(index: i32) -> OutputMode {
    match index {
        0 => OutputMode::Auto,
        1 => OutputMode::Windowed,
        2 => OutputMode::Maximized,
        3 => OutputMode::Fullscreen,
        _ => OutputMode::Auto,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_index_matches_the_slint_order() {
        assert_eq!(mode_index(OutputMode::Auto), 0);
        assert_eq!(mode_index(OutputMode::Windowed), 1);
        assert_eq!(mode_index(OutputMode::Maximized), 2);
        assert_eq!(mode_index(OutputMode::Fullscreen), 3);
    }

    #[test]
    fn mode_from_index_matches_the_slint_order_and_falls_back_to_auto() {
        for mode in OutputMode::ALL {
            assert_eq!(mode_from_index(mode_index(mode)), mode);
        }
        assert_eq!(mode_from_index(-1), OutputMode::Auto);
        assert_eq!(mode_from_index(4), OutputMode::Auto);
        assert_eq!(mode_from_index(99), OutputMode::Auto);
    }
}

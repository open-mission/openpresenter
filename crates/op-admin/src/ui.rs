use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use slint::{CloseRequestResponse, ComponentHandle, TimerMode};

use crate::outputs::OutputsController;
use crate::session::{LoadedSession, load_session};
use crate::{MainWindow, SettingsWindow, settings};

/// Monta o painel, abre as saídas configuradas e executa o loop de eventos.
pub fn run() -> anyhow::Result<()> {
    let LoadedSession { view, db } = load_session();
    let database_error = if view.db_ready {
        None
    } else {
        Some(view.db_error.clone())
    };
    let controller = Rc::new(RefCell::new(OutputsController::new(db, database_error)));
    let settings_slot = Rc::new(RefCell::new(None::<SettingsWindow>));

    let ui = MainWindow::new()?;

    ui.set_app_name(op_core::APP_NAME.into());
    ui.set_app_tagline(view.tagline.as_str().into());
    ui.set_app_version(view.app_version.as_str().into());
    ui.set_db_path(view.db_path.as_str().into());
    ui.set_db_ready(view.db_ready);
    ui.set_db_error(view.db_error.as_str().into());
    ui.set_schema_version(view.schema_version);
    ui.set_first_run_at(view.first_run_at.as_str().into());
    ui.set_last_run_at(view.last_run_at.as_str().into());
    ui.set_is_first_run(view.is_first_run);
    ui.set_theme_label(view.theme_label.as_str().into());
    ui.set_locale(view.locale.as_str().into());
    ui.set_stack(view.stack.as_str().into());

    if !view.db_ready {
        eprintln!("Banco indisponível ({}): {}", view.db_path, view.db_error);
    }

    ui.show()?;

    controller.borrow_mut().refresh_monitors();
    controller.borrow_mut().start();

    {
        let controller = Rc::clone(&controller);
        let settings_slot = Rc::clone(&settings_slot);
        ui.on_open_settings(move || {
            if let Err(error) = settings::open(Rc::clone(&controller), Rc::clone(&settings_slot)) {
                eprintln!("Não foi possível abrir a janela de Configurações: {error}");
            }
        });
    }

    {
        let controller = Rc::clone(&controller);
        let settings_slot = Rc::clone(&settings_slot);
        ui.window().on_close_requested(move || {
            settings::hide(&settings_slot);
            controller.borrow_mut().close_all();
            if let Err(error) = slint::quit_event_loop() {
                eprintln!("Não foi possível encerrar o loop de eventos: {error}");
            }
            CloseRequestResponse::HideWindow
        });
    }

    // Mantém o estado das Configurações atualizado enquanto o app estiver aberto.
    let status_timer = slint::Timer::default();
    {
        let controller = Rc::clone(&controller);
        let settings_slot = Rc::clone(&settings_slot);
        status_timer.start(TimerMode::Repeated, Duration::from_millis(500), move || {
            let Ok(controller) = controller.try_borrow() else {
                // O controlador está ocupado; o próximo tique reflete o estado.
                return;
            };
            settings::refresh_if_open(&settings_slot, &controller);
        });
    }

    slint::run_event_loop()?;
    drop(status_timer);
    Ok(())
}

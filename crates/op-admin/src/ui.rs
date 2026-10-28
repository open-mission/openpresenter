slint::include_modules!();

use slint::ComponentHandle;

use crate::session::load_session;

/// Monta a janela com os dados da sessão e executa o loop de eventos.
pub fn run() -> anyhow::Result<()> {
    let view = load_session();

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

    ui.run()?;
    Ok(())
}

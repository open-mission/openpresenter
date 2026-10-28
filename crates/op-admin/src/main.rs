slint::include_modules!();

mod commands;
mod monitors;
mod outputs;
mod session;
mod settings;
mod ui;

fn main() -> anyhow::Result<()> {
    ui::run()
}

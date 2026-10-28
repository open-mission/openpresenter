slint::include_modules!();

mod monitors;
mod outputs;
mod session;
mod settings;
mod ui;

fn main() -> anyhow::Result<()> {
    ui::run()
}

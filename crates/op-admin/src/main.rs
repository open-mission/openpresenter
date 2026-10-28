slint::include_modules!();

mod monitors;
// A API restante de saídas será consumida pela janela de Configurações na Task 6.
#[allow(dead_code)]
mod outputs;
mod session;
mod ui;

fn main() -> anyhow::Result<()> {
    ui::run()
}

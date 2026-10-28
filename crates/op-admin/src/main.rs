// A API de monitores passa a ser consumida pelas janelas de saída na Task 5.
#[allow(dead_code)]
mod monitors;
mod session;
mod ui;

fn main() -> anyhow::Result<()> {
    ui::run()
}

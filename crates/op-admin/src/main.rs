mod session;

fn main() {
    let view = session::load_session();
    println!("{} v{}", op_core::APP_NAME, view.app_version);
    println!("{}", view.tagline);
    println!("Stack: {}", view.stack);
    println!("Banco: {}", view.db_path);
    println!("Pronto: {}", view.db_ready);
    println!("Erro: {}", view.db_error);
    println!("Schema: {}", view.schema_version);
    println!("Tema: {} · {}", view.theme_label, view.locale);
    println!("Primeira execução: {}", view.first_run_at);
    println!("Última execução: {}", view.last_run_at);
    println!("Primeira sessão: {}", view.is_first_run);
}

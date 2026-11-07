//! Gerenciamento de ciclo de vida do processo executável do OpenPresenter.

/// Reinicia a aplicação spawando um novo processo independente do executável atual
/// com os mesmos argumentos de linha de comando.
pub fn restart_application() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut cmd = std::process::Command::new(current_exe);
    cmd.args(args);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        const DETACHED_PROCESS: u32 = 0x00000008;
        cmd.creation_flags(CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS);
    }

    cmd.spawn()?;
    Ok(())
}

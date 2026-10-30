//! Módulo de gerenciamento do System Tray (Menu Bar / Barra de Tarefas) e diálogos nativos.

use muda::{Menu, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

/// IDs das opções do menu da bandeja
#[allow(dead_code)]
pub struct TrayMenuIds {
    pub open_admin: muda::MenuId,
    pub open_public: muda::MenuId,
    pub open_stage: muda::MenuId,
    pub change_port: muda::MenuId,
    pub quit: muda::MenuId,
}

/// Estado dos itens do tray
#[allow(dead_code)]
pub struct TrayState {
    pub tray_icon: TrayIcon,
    pub menu_ids: TrayMenuIds,
    pub port: u16,
}

/// Cria o ícone nativo da bandeja com a marca oficial do OpenPresenter.
pub fn create_tray_icon() -> Result<Icon, Box<dyn std::error::Error>> {
    const ICON_BYTES: &[u8] =
        include_bytes!("../../../assets/branding/open-presenter-app-icon-32.png");
    let img = image::load_from_memory(ICON_BYTES)?.into_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    Ok(Icon::from_rgba(rgba, width, height)?)
}

/// Constrói o menu nativo da bandeja do sistema operacional.
pub fn build_tray_menu(port: u16) -> Result<(Menu, TrayMenuIds), Box<dyn std::error::Error>> {
    let menu = Menu::new();

    let open_admin = MenuItem::new("Abrir Painel de Controle", true, None);
    let open_public = MenuItem::new("Abrir Tela Pública (Projetor)", true, None);
    let open_stage = MenuItem::new("Abrir Retorno de Palco (Stage)", true, None);

    let status_label = format!("● Servidor Ativo (Porta {})", port);
    let status_item = MenuItem::new(status_label, false, None);

    let change_port = MenuItem::new("Configurar Porta do Servidor...", true, None);
    let quit = MenuItem::new("Sair do OpenPresenter", true, None);

    let sep1 = PredefinedMenuItem::separator();
    let sep2 = PredefinedMenuItem::separator();

    menu.append(&open_admin)?;
    menu.append(&open_public)?;
    menu.append(&open_stage)?;
    menu.append(&sep1)?;
    menu.append(&status_item)?;
    menu.append(&change_port)?;
    menu.append(&sep2)?;
    menu.append(&quit)?;

    let ids = TrayMenuIds {
        open_admin: open_admin.id().clone(),
        open_public: open_public.id().clone(),
        open_stage: open_stage.id().clone(),
        change_port: change_port.id().clone(),
        quit: quit.id().clone(),
    };

    Ok((menu, ids))
}

/// Inicializa o TrayIcon completo com o menu e ícone da marca.
pub fn init_tray(port: u16) -> Result<TrayState, Box<dyn std::error::Error>> {
    let icon = create_tray_icon()?;
    let (menu, menu_ids) = build_tray_menu(port)?;

    let tooltip = format!("OpenPresenter Minimal — Servidor ativo na porta {}", port);

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(tooltip)
        .with_icon(icon)
        .build()?;

    Ok(TrayState {
        tray_icon,
        menu_ids,
        port,
    })
}

/// Abre um diálogo nativo para entrada de nova porta do servidor.
pub fn prompt_change_port(current_port: u16) -> Option<u16> {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            r#"display dialog "Digite o número da porta para o servidor OpenPresenter (padrão: 18080):" default answer "{}" with title "Configurar Porta do Servidor" buttons {{"Cancelar", "Salvar"}} default button "Salvar""#,
            current_port
        );
        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .ok()?;

        if output.status.success() {
            let out_str = String::from_utf8_lossy(&output.stdout);
            // Saída típica: "button returned:Salvar, text returned:18080"
            if out_str.contains("button returned:Salvar") {
                let pos = out_str.find("text returned:")?;
                let val = out_str[pos + "text returned:".len()..].trim();
                return val
                    .parse::<u16>()
                    .ok()
                    .filter(|&p| (1024..=65535).contains(&p));
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let ps_cmd = format!(
            "[void][System.Reflection.Assembly]::LoadWithPartialName('Microsoft.VisualBasic'); [Microsoft.VisualBasic.Interaction]::InputBox('Digite a nova porta para o servidor OpenPresenter (ex: 18080):', 'Configurar Porta do Servidor', '{}')",
            current_port
        );
        let output = std::process::Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(ps_cmd)
            .output()
            .ok()?;

        if output.status.success() {
            let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Some(p) = out_str
                .parse::<u16>()
                .ok()
                .filter(|&p| (1024..=65535).contains(&p))
            {
                return Some(p);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("zenity")
            .arg("--entry")
            .arg(format!("--entry-text={}", current_port))
            .arg("--text=Digite a nova porta do servidor:")
            .arg("--title=Configurar Porta do Servidor")
            .output()
            .or_else(|_| {
                std::process::Command::new("kdialog")
                    .arg("--inputbox")
                    .arg("Digite a nova porta do servidor:")
                    .arg(current_port.to_string())
                    .output()
            })
            .ok()?;

        if output.status.success() {
            let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Some(p) = out_str
                .parse::<u16>()
                .ok()
                .filter(|&p| (1024..=65535).contains(&p))
            {
                return Some(p);
            }
        }
    }

    None
}

/// Exibe um alerta nativo de confirmação ao usuário.
pub fn show_native_alert(title: &str, message: &str) {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            r#"display alert "{}" message "{}""#,
            title.replace('"', "\\\""),
            message.replace('"', "\\\"")
        );
        let _ = std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .spawn();
    }

    #[cfg(target_os = "windows")]
    {
        let ps_cmd = format!(
            "[void][System.Windows.Forms.MessageBox]::Show('{}', '{}')",
            message.replace('\'', "''"),
            title.replace('\'', "''")
        );
        let _ = std::process::Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(ps_cmd)
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("zenity")
            .arg("--info")
            .arg(format!("--title={}", title))
            .arg(format!("--text={}", message))
            .spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tray_icon_loads_official_branding() {
        let icon = create_tray_icon();
        assert!(
            icon.is_ok(),
            "Ícone do tray deve carregar com sucesso os bytes oficiais"
        );
    }
}

//! Módulo de atualização automática (Auto-Updater) para o OpenPresenter.
//!
//! Realiza checagem de releases no repositório GitHub (open-mission/openpresenter),
//! download do asset correspondente ao sistema operacional/arquitetura e
//! substituição atômica e segura do executável atual.

use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::{Context, Result, anyhow};
use semver::Version;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

/// Repositório canônico no GitHub para checagem de atualizações.
pub const GITHUB_REPO: &str = "open-mission/openpresenter";

/// Representação de um asset de release no GitHub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
    #[serde(default)]
    pub size: u64,
}

/// Representação simplificada de um release do GitHub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
    pub published_at: Option<String>,
    #[serde(default)]
    pub assets: Vec<GithubAsset>,
}

/// Resposta de verificação de atualização enviada para a UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResponse {
    pub update_available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_name: String,
    pub release_notes: String,
    pub html_url: String,
    pub published_at: String,
    pub download_url: Option<String>,
    pub asset_name: Option<String>,
    pub asset_size: u64,
}

/// Estágios do ciclo de vida da atualização.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum UpdateStage {
    Idle,
    Checking,
    Downloading {
        percent: u8,
        downloaded_bytes: u64,
        total_bytes: u64,
    },
    Extracting,
    ReadyToRestart {
        new_version: String,
    },
    Error {
        message: String,
    },
}

/// Estado global em memória do Auto-Updater.
#[derive(Debug, Clone)]
pub struct UpdaterState {
    pub stage: UpdateStage,
    pub last_check: Option<UpdateCheckResponse>,
}

impl Default for UpdaterState {
    fn default() -> Self {
        Self {
            stage: UpdateStage::Idle,
            last_check: None,
        }
    }
}

pub type SharedUpdater = Arc<RwLock<UpdaterState>>;

/// Localiza o asset de build correspondente ao sistema operacional e arquitetura atuais.
pub fn find_matching_asset<'a>(
    assets: &'a [GithubAsset],
    os: &str,
    arch: &str,
) -> Option<&'a GithubAsset> {
    assets.iter().find(|asset| {
        let name = asset.name.to_lowercase();
        match os {
            "windows" => name.contains("windows") && name.ends_with(".zip"),
            "macos" => {
                if arch == "aarch64" {
                    name.contains("macos-arm64")
                        || (name.contains("macos") && name.contains("arm64"))
                } else {
                    name.contains("macos-x64") || (name.contains("macos") && name.contains("x64"))
                }
            }
            "linux" => {
                name.contains("linux") && (name.ends_with(".tar.gz") || name.ends_with(".tgz"))
            }
            _ => false,
        }
    })
}

/// Compara a versão atual contra a tag informada no GitHub.
/// Retorna `true` se a tag for estritamente mais nova que a versão local.
pub fn is_newer_version(current: &str, remote_tag: &str) -> bool {
    let current_clean = current.trim_start_matches('v');
    let remote_clean = remote_tag.trim_start_matches('v');

    match (Version::parse(current_clean), Version::parse(remote_clean)) {
        (Ok(curr), Ok(rem)) => rem > curr,
        _ => {
            // Fallback para comparação léxica básica de versão
            remote_clean != current_clean && !remote_clean.is_empty()
        }
    }
}

/// Consulta o GitHub Releases e retorna as informações de atualização.
pub async fn check_for_updates(
    client: &reqwest::Client,
    current_version: &str,
) -> Result<UpdateCheckResponse> {
    let url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        GITHUB_REPO
    );

    let resp = client
        .get(&url)
        .header(
            "User-Agent",
            format!("OpenPresenter-Updater/{}", current_version),
        )
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .context("Falha ao conectar com o serviço de releases do GitHub")?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Serviço de atualizações do GitHub retornou status {}",
            resp.status()
        ));
    }

    let release: GithubRelease = resp
        .json()
        .await
        .context("Falha ao decodificar informações da release do GitHub")?;

    let update_available = is_newer_version(current_version, &release.tag_name);
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let matching_asset = find_matching_asset(&release.assets, os, arch);

    let (download_url, asset_name, asset_size) = match matching_asset {
        Some(a) => (
            Some(a.browser_download_url.clone()),
            Some(a.name.clone()),
            a.size,
        ),
        None => (None, None, 0),
    };

    Ok(UpdateCheckResponse {
        update_available,
        current_version: current_version.to_string(),
        latest_version: release.tag_name.trim_start_matches('v').to_string(),
        release_name: release.name.unwrap_or_else(|| release.tag_name.clone()),
        release_notes: release.body.unwrap_or_default(),
        html_url: release.html_url,
        published_at: release.published_at.unwrap_or_default(),
        download_url,
        asset_name,
        asset_size,
    })
}

/// Extrai o binário executável a partir dos bytes compactados de um `.zip` ou `.tar.gz`.
pub fn extract_binary_from_archive(bytes: &[u8], asset_name: &str) -> Result<Vec<u8>> {
    let lower_name = asset_name.to_lowercase();

    if lower_name.ends_with(".zip") {
        let cursor = Cursor::new(bytes);
        let mut archive =
            zip::ZipArchive::new(cursor).context("Falha ao abrir arquivo compactado ZIP")?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .context("Falha ao ler item do arquivo ZIP")?;
            let name = file.name().to_string();

            if name.ends_with(".exe") || name.ends_with("open-presenter") {
                let mut buffer = Vec::with_capacity(file.size() as usize);
                file.read_to_end(&mut buffer)?;
                return Ok(buffer);
            }
        }
        Err(anyhow!(
            "Nenhum executável 'open-presenter' foi encontrado no pacote ZIP"
        ))
    } else if lower_name.ends_with(".tar.gz") || lower_name.ends_with(".tgz") {
        let cursor = Cursor::new(bytes);
        let gz = flate2::read::GzDecoder::new(cursor);
        let mut archive = tar::Archive::new(gz);

        for entry in archive
            .entries()
            .context("Falha ao ler entradas do arquivo tar")?
        {
            let mut entry = entry.context("Falha ao processar arquivo do arquivo tar")?;
            let path = entry.path().context("Caminho inválido no arquivo tar")?;
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            if file_name == "open-presenter" || file_name.ends_with(".exe") {
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer)?;
                return Ok(buffer);
            }
        }
        Err(anyhow!(
            "Nenhum executável 'open-presenter' foi encontrado no pacote TAR.GZ"
        ))
    } else {
        Err(anyhow!(
            "Formato de pacote de atualização desconhecido: {}",
            asset_name
        ))
    }
}

/// Realiza a substituição atômica e segura do executável atual pelo novo binário extraído.
pub fn apply_binary_replacement(new_binary_bytes: &[u8]) -> Result<PathBuf> {
    let current_exe =
        std::env::current_exe().context("Não foi possível obter o caminho do executável atual")?;
    let exe_dir = current_exe
        .parent()
        .ok_or_else(|| anyhow!("Não foi possível determinar a pasta do executável"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // Cria arquivo temporário no mesmo diretório para garantir mesmo sistema de arquivos
        let temp_new_path = exe_dir.join(format!(".open-presenter.new.{}", std::process::id()));
        fs::write(&temp_new_path, new_binary_bytes)
            .context("Falha ao salvar novo binário temporário")?;

        // Ajusta permissões de execução (0o755)
        let perms = fs::Permissions::from_mode(0o755);
        let _ = fs::set_permissions(&temp_new_path, perms);

        // Substituição atômica de inode no mesmo filesystem
        fs::rename(&temp_new_path, &current_exe)
            .context("Falha ao substituir executável de forma atômica no Unix")?;
    }

    #[cfg(windows)]
    {
        // No Windows, o arquivo em execução está bloqueado para escrita direta,
        // mas o sistema permite renomeá-lo!
        let old_exe_path = current_exe.with_extension("old.exe");

        // Tenta remover backup anterior se existir
        if old_exe_path.exists() {
            let _ = fs::remove_file(&old_exe_path);
        }

        // Renomeia o executável atual para .old.exe
        fs::rename(&current_exe, &old_exe_path)
            .context("Falha ao renomear executável ativo para .old.exe no Windows")?;

        // Grava o novo executável no caminho original
        if let Err(e) = fs::write(&current_exe, new_binary_bytes) {
            // Em caso de falha, tenta restaurar o arquivo original
            let _ = fs::rename(&old_exe_path, &current_exe);
            return Err(anyhow!("Falha ao gravar novo executável no Windows: {}", e));
        }
    }

    Ok(current_exe)
}

/// Executa o fluxo completo de download e aplicação da atualização em segundo plano.
pub async fn start_background_update(
    client: reqwest::Client,
    updater: SharedUpdater,
    download_url: String,
    asset_name: String,
    new_version: String,
) {
    {
        let mut st = updater.write().await;
        st.stage = UpdateStage::Downloading {
            percent: 0,
            downloaded_bytes: 0,
            total_bytes: 0,
        };
    }

    info!(
        "Iniciando download do pacote de atualização: {}",
        download_url
    );

    let response = match client.get(&download_url).send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                let msg = format!("Falha no download da release (HTTP {})", resp.status());
                error!("{}", msg);
                let mut st = updater.write().await;
                st.stage = UpdateStage::Error { message: msg };
                return;
            }
            resp
        }
        Err(e) => {
            let msg = format!("Erro ao conectar com servidor de download: {}", e);
            error!("{}", msg);
            let mut st = updater.write().await;
            st.stage = UpdateStage::Error { message: msg };
            return;
        }
    };

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut buffer = Vec::with_capacity(if total_size > 0 {
        total_size as usize
    } else {
        10 * 1024 * 1024
    });
    let mut last_percent: u8 = 0;

    let mut response = response;
    loop {
        match response.chunk().await {
            Ok(Some(chunk)) => {
                buffer.extend_from_slice(&chunk);
                downloaded += chunk.len() as u64;

                let percent = if total_size > 0 {
                    ((downloaded as f64 / total_size as f64) * 100.0).min(100.0) as u8
                } else {
                    0
                };

                if percent != last_percent {
                    last_percent = percent;
                    let mut st = updater.write().await;
                    st.stage = UpdateStage::Downloading {
                        percent,
                        downloaded_bytes: downloaded,
                        total_bytes: total_size,
                    };
                }
            }
            Ok(None) => break,
            Err(e) => {
                let msg = format!("Interrupção no download do pacote: {}", e);
                error!("{}", msg);
                let mut st = updater.write().await;
                st.stage = UpdateStage::Error { message: msg };
                return;
            }
        }
    }

    info!(
        "Download concluído com sucesso ({} bytes). Extraindo binário...",
        downloaded
    );
    {
        let mut st = updater.write().await;
        st.stage = UpdateStage::Extracting;
    }

    // Processamento síncrono em thread pool do Tokio
    let asset_name_clone = asset_name.clone();
    let extract_result = tokio::task::spawn_blocking(move || {
        let extracted_bytes = extract_binary_from_archive(&buffer, &asset_name_clone)?;
        apply_binary_replacement(&extracted_bytes)
    })
    .await;

    match extract_result {
        Ok(Ok(target_path)) => {
            info!(
                "Atualização aplicada com sucesso em {:?}! Nova versão {} pronta.",
                target_path, new_version
            );
            let mut st = updater.write().await;
            st.stage = UpdateStage::ReadyToRestart {
                new_version: new_version.clone(),
            };
        }
        Ok(Err(e)) => {
            let msg = format!("Falha ao extrair ou aplicar atualização: {}", e);
            error!("{}", msg);
            let mut st = updater.write().await;
            st.stage = UpdateStage::Error { message: msg };
        }
        Err(e) => {
            let msg = format!("Erro interno ao executar tarefa de atualização: {}", e);
            error!("{}", msg);
            let mut st = updater.write().await;
            st.stage = UpdateStage::Error { message: msg };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer_version() {
        assert!(is_newer_version("0.1.0", "v0.1.1"));
        assert!(is_newer_version("0.1.0", "0.1.1"));
        assert!(is_newer_version("0.1.0", "v0.2.0"));
        assert!(is_newer_version("0.1.0", "v1.0.0"));
        assert!(!is_newer_version("0.1.1", "v0.1.1"));
        assert!(!is_newer_version("0.1.1", "v0.1.0"));
        assert!(!is_newer_version("1.0.0", "v0.9.9"));
    }

    #[test]
    fn test_find_matching_asset() {
        let assets = vec![
            GithubAsset {
                name: "open-presenter-windows-x64.zip".into(),
                browser_download_url: "https://example.com/windows".into(),
                size: 1000,
            },
            GithubAsset {
                name: "open-presenter-macos-arm64.tar.gz".into(),
                browser_download_url: "https://example.com/mac-arm".into(),
                size: 2000,
            },
            GithubAsset {
                name: "open-presenter-macos-x64.tar.gz".into(),
                browser_download_url: "https://example.com/mac-intel".into(),
                size: 2100,
            },
            GithubAsset {
                name: "open-presenter-linux-x64.tar.gz".into(),
                browser_download_url: "https://example.com/linux".into(),
                size: 3000,
            },
        ];

        let win = find_matching_asset(&assets, "windows", "x86_64");
        assert_eq!(
            win.map(|a| a.name.as_str()),
            Some("open-presenter-windows-x64.zip")
        );

        let mac_arm = find_matching_asset(&assets, "macos", "aarch64");
        assert_eq!(
            mac_arm.map(|a| a.name.as_str()),
            Some("open-presenter-macos-arm64.tar.gz")
        );

        let mac_x64 = find_matching_asset(&assets, "macos", "x86_64");
        assert_eq!(
            mac_x64.map(|a| a.name.as_str()),
            Some("open-presenter-macos-x64.tar.gz")
        );

        let lin = find_matching_asset(&assets, "linux", "x86_64");
        assert_eq!(
            lin.map(|a| a.name.as_str()),
            Some("open-presenter-linux-x64.tar.gz")
        );
    }

    #[test]
    fn test_extract_binary_from_zip() {
        use std::io::Write;
        let mut zip_buf = Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut zip_buf);
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);
            zip.start_file("open-presenter.exe", options).unwrap();
            zip.write_all(b"windows_binary_test").unwrap();
            zip.finish().unwrap();
        }

        let zip_bytes = zip_buf.into_inner();
        let extracted = extract_binary_from_archive(&zip_bytes, "open-presenter-windows-x64.zip")
            .expect("Deveria extrair executavel do ZIP");
        assert_eq!(extracted, b"windows_binary_test");
    }

    #[test]
    fn test_extract_binary_from_tar_gz() {
        let mut tar_buf = Vec::new();
        {
            let gz = flate2::write::GzEncoder::new(&mut tar_buf, flate2::Compression::default());
            let mut tar = tar::Builder::new(gz);

            let data = b"unix_binary_test";
            let mut header = tar::Header::new_gnu();
            header.set_path("open-presenter").unwrap();
            header.set_size(data.len() as u64);
            header.set_mode(0o755);
            header.set_cksum();

            tar.append(&header, &data[..]).unwrap();
            tar.finish().unwrap();
        }

        let extracted = extract_binary_from_archive(&tar_buf, "open-presenter-macos-arm64.tar.gz")
            .expect("Deveria extrair executavel do TAR.GZ");
        assert_eq!(extracted, b"unix_binary_test");
    }
}

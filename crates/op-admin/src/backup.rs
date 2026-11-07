//! Módulo de exportação, inspeção e restauração de dados da aplicação (Backup & Migração).
//!
//! Empacota e descompacta arquivos `.opbackup` (formato ZIP com compressão Deflate) contendo:
//! - `manifest.json`: metadados da aplicação, versões e estatísticas quantitativas.
//! - `database/open-presenter.db`: snapshot consistente e atômico do SQLite.
//! - `media/`: imagens, papéis de parede, vídeos, áudios e identidade da igreja.
//! - `bibles/`: módulos bíblicos em SQLite.
//! - `fonts/`: fontes tipográficas customizadas.

use std::fs::{self, File};
use std::io::{self, Read, Seek, Write};
use std::path::{Component, Path, PathBuf};

use chrono::Utc;
use op_core::{
    APP_NAME, BACKUP_FORMAT_VERSION, BackupExportOptions, BackupInspectResult, BackupManifest,
    BackupRestoreResult, BackupStats, VERSION,
};
use op_db::Database;
use tracing::{debug, info, warn};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

/// Sanitiza caminhos de entradas contidas no arquivo ZIP para proteção rigorosa contra Zip Slip.
///
/// Recusa qualquer caminho que contenha componentes reversos (`..`), prefixos de raiz ou que escape
/// do diretório base pretendido.
pub fn sanitize_zip_path(entry_name: &str, base_dir: &Path) -> Result<PathBuf, String> {
    let clean = entry_name.replace('\\', "/");
    if clean.starts_with('/') || clean.contains(':') {
        return Err(format!(
            "Entrada com caminho absoluto ou prefixo inválido rejeitada: {entry_name}"
        ));
    }
    let relative = Path::new(&clean);

    let mut safe_path = PathBuf::new();
    for component in relative.components() {
        match component {
            Component::Normal(c) => safe_path.push(c),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(format!(
                    "Entrada com tentativa de diretório superior ('..') rejeitada: {entry_name}"
                ));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(format!(
                    "Entrada com caminho absoluto rejeitada por segurança: {entry_name}"
                ));
            }
        }
    }

    if safe_path.as_os_str().is_empty() {
        return Err(format!("Entrada com caminho vazio: {entry_name}"));
    }

    let target = base_dir.join(safe_path);
    Ok(target)
}

/// Recursivamente adiciona arquivos de um diretório do sistema de arquivos ao arquivo ZIP.
fn add_directory_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: SimpleFileOptions,
    source_dir: &Path,
    zip_prefix: &str,
    count_counter: &mut usize,
    bytes_counter: &mut u64,
) -> io::Result<()> {
    if !source_dir.exists() || !source_dir.is_dir() {
        return Ok(());
    }

    let read_dir = match fs::read_dir(source_dir) {
        Ok(rd) => rd,
        Err(e) => {
            warn!(
                "Não foi possível ler diretório {}: {}",
                source_dir.display(),
                e
            );
            return Ok(());
        }
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        // Ignora arquivos ocultos temporários (ex: .DS_Store, thumbs.db)
        if file_name.starts_with('.') {
            continue;
        }

        let zip_path = if zip_prefix.is_empty() {
            file_name.to_string()
        } else {
            format!("{zip_prefix}/{file_name}")
        };

        if path.is_dir() {
            add_directory_to_zip(zip, options, &path, &zip_path, count_counter, bytes_counter)?;
        } else if path.is_file() {
            let metadata = match fs::metadata(&path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let mut file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    warn!("Não foi possível abrir arquivo {}: {}", path.display(), e);
                    continue;
                }
            };

            zip.start_file(&zip_path, options)?;
            io::copy(&mut file, zip)?;

            *count_counter += 1;
            *bytes_counter += metadata.len();
            debug!(
                "Adicionado ao backup: {} ({} bytes)",
                zip_path,
                metadata.len()
            );
        }
    }

    Ok(())
}

/// Exporta todo o estado da aplicação para um escritor que implementa `Write + Seek` (ex: arquivo).
pub fn export_backup_package<W: Write + Seek>(
    writer: W,
    data_dir: &Path,
    db: &Database,
    export_opts: &BackupExportOptions,
) -> anyhow::Result<BackupManifest> {
    let mut zip = ZipWriter::new(writer);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut stats = BackupStats {
        songs_count: db.count_songs().unwrap_or(0),
        setlists_count: db.count_setlists().unwrap_or(0),
        team_members_count: db.count_team_members().unwrap_or(0),
        bibles_count: db.count_bibles().unwrap_or(0),
        ..Default::default()
    };

    let mut total_files: usize = 0;
    let mut total_bytes: u64 = 0;

    // 1. Snapshot atômico do banco de dados SQLite
    if export_opts.include_database {
        let temp_dir = tempfile::tempdir()?;
        let temp_db_path = temp_dir.path().join("snapshot.db");
        db.backup_to_file(&temp_db_path)?;

        let mut db_file = File::open(&temp_db_path)?;
        let db_len = db_file.metadata()?.len();

        zip.start_file("database/open-presenter.db", options)?;
        io::copy(&mut db_file, &mut zip)?;

        total_files += 1;
        total_bytes += db_len;
        info!(
            "Snapshot do banco SQLite adicionado com sucesso ({} bytes)",
            db_len
        );
    }

    // 2. Mídias
    let media_dir = data_dir.join("media");
    if export_opts.include_images {
        let mut count = 0;
        let mut bytes = 0;
        add_directory_to_zip(
            &mut zip,
            options,
            &media_dir.join("images"),
            "media/images",
            &mut count,
            &mut bytes,
        )?;
        stats.images_count = count;
        total_files += count;
        total_bytes += bytes;
    }

    if export_opts.include_wallpapers {
        let mut count = 0;
        let mut bytes = 0;
        add_directory_to_zip(
            &mut zip,
            options,
            &media_dir.join("wallpapers"),
            "media/wallpapers",
            &mut count,
            &mut bytes,
        )?;
        stats.wallpapers_count = count;
        total_files += count;
        total_bytes += bytes;
    }

    if export_opts.include_videos {
        let mut count = 0;
        let mut bytes = 0;
        add_directory_to_zip(
            &mut zip,
            options,
            &media_dir.join("videos"),
            "media/videos",
            &mut count,
            &mut bytes,
        )?;
        stats.videos_count = count;
        total_files += count;
        total_bytes += bytes;
    }

    if export_opts.include_audios {
        let mut count = 0;
        let mut bytes = 0;
        add_directory_to_zip(
            &mut zip,
            options,
            &media_dir.join("audios"),
            "media/audios",
            &mut count,
            &mut bytes,
        )?;
        stats.audios_count = count;
        total_files += count;
        total_bytes += bytes;
    }

    if export_opts.include_church_brand {
        let mut count = 0;
        let mut bytes = 0;
        add_directory_to_zip(
            &mut zip,
            options,
            &media_dir.join("church_brand"),
            "media/church_brand",
            &mut count,
            &mut bytes,
        )?;
        stats.church_brand_count = count;
        total_files += count;
        total_bytes += bytes;
    }

    // 3. Bíblias
    if export_opts.include_bibles {
        let mut count = 0;
        let mut bytes = 0;
        add_directory_to_zip(
            &mut zip,
            options,
            &data_dir.join("bibles"),
            "bibles",
            &mut count,
            &mut bytes,
        )?;
        total_files += count;
        total_bytes += bytes;
    }

    // 4. Fontes
    if export_opts.include_fonts {
        let mut count = 0;
        let mut bytes = 0;
        add_directory_to_zip(
            &mut zip,
            options,
            &data_dir.join("fonts"),
            "fonts",
            &mut count,
            &mut bytes,
        )?;
        stats.fonts_count = count;
        total_files += count;
        total_bytes += bytes;
    }

    stats.total_files_count = total_files;
    stats.total_bytes = total_bytes;

    let schema_ver = db.schema_version().unwrap_or(Database::MIGRATIONS_COUNT);
    let manifest = BackupManifest {
        format_version: BACKUP_FORMAT_VERSION,
        app_name: APP_NAME.to_string(),
        app_version: VERSION.to_string(),
        schema_version: schema_ver,
        created_at: Utc::now().to_rfc3339(),
        stats,
    };

    // 5. Adiciona manifest.json na raiz do arquivo ZIP
    let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
    zip.start_file("manifest.json", options)?;
    zip.write_all(&manifest_bytes)?;

    zip.finish()?;
    info!(
        "Backup gerado com sucesso: {} arquivos, {} bytes totais",
        total_files, total_bytes
    );

    Ok(manifest)
}

/// Inspeciona um arquivo ZIP de backup, retornando os metadados do manifesto e o diagnóstico de compatibilidade.
pub fn inspect_backup_package<R: Read + Seek>(
    reader: R,
    filename: &str,
    file_size_bytes: u64,
) -> anyhow::Result<BackupInspectResult> {
    let mut archive = match ZipArchive::new(reader) {
        Ok(a) => a,
        Err(e) => {
            return Ok(BackupInspectResult {
                valid: false,
                is_compatible: false,
                error: Some(format!("Arquivo ZIP inválido ou corrompido: {e}")),
                manifest: None,
                filename: filename.to_string(),
                file_size_bytes,
            });
        }
    };

    // 1. Tenta carregar manifest.json
    let manifest_res: Result<BackupManifest, _> = match archive.by_name("manifest.json") {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                Err(anyhow::anyhow!("Falha ao ler manifest.json: {e}"))
            } else {
                serde_json::from_str(&contents)
                    .map_err(|e| anyhow::anyhow!("manifest.json inválido: {e}"))
            }
        }
        Err(_) => Err(anyhow::anyhow!(
            "Arquivo manifest.json não encontrado no pacote"
        )),
    };

    match manifest_res {
        Ok(manifest) => {
            // Checa compatibilidade
            let max_schema = Database::MIGRATIONS_COUNT;
            let (is_compatible, error) = if manifest.schema_version > max_schema {
                (
                    false,
                    Some(format!(
                        "O backup foi gerado por uma versão mais recente (schema {}) que a suportada por este OpenPresenter (schema {}). Atualize o aplicativo.",
                        manifest.schema_version, max_schema
                    )),
                )
            } else if manifest.format_version > BACKUP_FORMAT_VERSION {
                (
                    false,
                    Some(format!(
                        "Versão de formato do backup ({}) superior à suportada ({}).",
                        manifest.format_version, BACKUP_FORMAT_VERSION
                    )),
                )
            } else {
                (true, None)
            };

            Ok(BackupInspectResult {
                valid: true,
                is_compatible,
                error,
                manifest: Some(manifest),
                filename: filename.to_string(),
                file_size_bytes,
            })
        }
        Err(e) => {
            // Verifica se pelo menos o banco open-presenter.db existe no ZIP
            let has_db = archive.by_name("database/open-presenter.db").is_ok()
                || archive.by_name("open-presenter.db").is_ok();

            if has_db {
                Ok(BackupInspectResult {
                    valid: true,
                    is_compatible: true,
                    error: Some(format!(
                        "Backup legado ou simplificado (sem manifesto formal: {e})"
                    )),
                    manifest: None,
                    filename: filename.to_string(),
                    file_size_bytes,
                })
            } else {
                Ok(BackupInspectResult {
                    valid: false,
                    is_compatible: false,
                    error: Some(format!(
                        "Arquivo não é um pacote de backup válido do OpenPresenter: {e}"
                    )),
                    manifest: None,
                    filename: filename.to_string(),
                    file_size_bytes,
                })
            }
        }
    }
}

/// Restaura o banco de dados e arquivos de mídia a partir de um leitor ZIP.
pub fn restore_backup_package<R: Read + Seek>(
    reader: R,
    data_dir: &Path,
    db: &mut Database,
) -> anyhow::Result<BackupRestoreResult> {
    let mut archive = ZipArchive::new(reader)?;

    // 1. Lê e valida manifest.json caso exista
    let mut manifest: Option<BackupManifest> = None;
    if let Ok(mut manifest_file) = archive.by_name("manifest.json") {
        let mut content = String::new();
        if manifest_file.read_to_string(&mut content).is_ok()
            && let Ok(m) = serde_json::from_str::<BackupManifest>(&content)
        {
            if m.schema_version > Database::MIGRATIONS_COUNT {
                anyhow::bail!(
                    "O backup possui schema ({}) mais recente que o suportado ({}); atualize o OpenPresenter",
                    m.schema_version,
                    Database::MIGRATIONS_COUNT
                );
            }
            manifest = Some(m);
        }
    }

    // 2. Localiza e extrai o banco de dados para arquivo temporário
    let temp_dir = tempfile::tempdir()?;
    let temp_db_path = temp_dir.path().join("restored.db");

    let db_entry_name = if archive.by_name("database/open-presenter.db").is_ok() {
        "database/open-presenter.db"
    } else if archive.by_name("open-presenter.db").is_ok() {
        "open-presenter.db"
    } else {
        anyhow::bail!(
            "Arquivo de banco de dados SQLite não encontrado no pacote de backup (.opbackup / .zip)"
        );
    };

    {
        let mut db_entry = archive.by_name(db_entry_name)?;
        let mut out_file = File::create(&temp_db_path)?;
        io::copy(&mut db_entry, &mut out_file)?;
    }

    // 3. Restaura o SQLite na conexão ativa
    db.restore_from_file(&temp_db_path)?;
    info!("Banco de dados SQLite restaurado com sucesso");

    // 4. Extrai arquivos de mídia, bíblias e fontes com rigorosa proteção contra Zip Slip
    let mut restored_files = 0;
    let archive_len = archive.len();

    for i in 0..archive_len {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        if file.is_dir() || name == "manifest.json" || name == db_entry_name {
            continue;
        }

        // Determina destino seguro
        let dest_path = if name.starts_with("media/")
            || name.starts_with("bibles/")
            || name.starts_with("fonts/")
        {
            match sanitize_zip_path(&name, data_dir) {
                Ok(p) => p,
                Err(err) => {
                    warn!("Ignorando arquivo inseguro no backup: {}", err);
                    continue;
                }
            }
        } else {
            // Ignora entradas desconhecidas
            continue;
        };

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut out = File::create(&dest_path)?;
        io::copy(&mut file, &mut out)?;
        restored_files += 1;
    }

    info!(
        "Restauração concluída com sucesso: {} arquivos extraídos",
        restored_files
    );

    Ok(BackupRestoreResult {
        success: true,
        message: format!(
            "Restauração concluída com sucesso. Banco de dados e {} arquivo(s) restaurados.",
            restored_files
        ),
        restored_files_count: restored_files,
        manifest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use op_core::{SectionType, Song, SongSection};
    use std::io::Cursor;

    #[test]
    fn test_sanitize_zip_path_prevents_directory_traversal() {
        let base = Path::new("/app/data");

        // Casos válidos
        assert_eq!(
            sanitize_zip_path("media/images/foto.jpg", base).unwrap(),
            base.join("media/images/foto.jpg")
        );
        assert_eq!(
            sanitize_zip_path(r"media\wallpapers\dark.svg", base).unwrap(),
            base.join("media/wallpapers/dark.svg")
        );

        // Casos maliciosos (Zip Slip)
        assert!(sanitize_zip_path("../../../etc/passwd", base).is_err());
        assert!(sanitize_zip_path("media/../../secret.txt", base).is_err());
        assert!(sanitize_zip_path("/etc/shadow", base).is_err());
        assert!(sanitize_zip_path("C:\\Windows\\System32\\cmd.exe", base).is_err());
    }

    #[test]
    fn test_export_inspect_and_restore_cycle() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source_data_dir = temp_dir.path().join("source_data");
        let target_data_dir = temp_dir.path().join("target_data");

        // Prepara diretórios de teste
        let source_images = source_data_dir.join("media").join("images");
        let source_bibles = source_data_dir.join("bibles");
        fs::create_dir_all(&source_images).unwrap();
        fs::create_dir_all(&source_bibles).unwrap();

        fs::write(source_images.join("foto_culto.jpg"), b"fake_jpeg_content").unwrap();
        fs::write(source_bibles.join("nvi.sqlite"), b"fake_bible_sqlite").unwrap();

        let db_path = source_data_dir.join("open-presenter.db");
        let source_db = Database::open(&db_path).unwrap();

        let song = Song {
            id: "song-exp-1".into(),
            title: "Grandes Coisas Fez o Senhor".into(),
            artist: "Fernandinho".into(),
            album: None,
            composer: None,
            song_key: Some("C".into()),
            bpm: Some(120),
            lyrics_raw: "Grandes coisas fez o Senhor por nós".into(),
            sections: vec![SongSection {
                section_type: Some(SectionType::Chorus),
                label: "Refrão".into(),
                lines: vec!["Grandes coisas fez o Senhor por nós".into()],
            }],
            tags: vec!["Celebração".into()],
            audio_url: None,
        };
        source_db.save_song(&song, Utc::now()).unwrap();
        source_db
            .set_setting("church_name", "Igreja da Graça")
            .unwrap();

        // 1. Exporta
        let mut zip_buffer = Cursor::new(Vec::new());
        let manifest = export_backup_package(
            &mut zip_buffer,
            &source_data_dir,
            &source_db,
            &BackupExportOptions::default(),
        )
        .expect("Deveria exportar pacote de backup");

        assert_eq!(manifest.stats.songs_count, 1);
        assert_eq!(manifest.stats.images_count, 1);

        let zip_bytes = zip_buffer.into_inner();
        assert!(!zip_bytes.is_empty());

        // 2. Inspeciona
        let inspect_res = inspect_backup_package(
            Cursor::new(&zip_bytes),
            "open-presenter-backup.opbackup",
            zip_bytes.len() as u64,
        )
        .expect("Deveria inspecionar com sucesso");

        assert!(inspect_res.valid);
        assert!(inspect_res.is_compatible);
        assert!(inspect_res.manifest.is_some());
        assert_eq!(inspect_res.manifest.unwrap().stats.songs_count, 1);

        // 3. Restaura em outro ambiente
        fs::create_dir_all(&target_data_dir).unwrap();
        let target_db_path = target_data_dir.join("open-presenter.db");
        let mut target_db = Database::open(&target_db_path).unwrap();

        assert_eq!(target_db.count_songs().unwrap(), 0);

        let restore_res =
            restore_backup_package(Cursor::new(&zip_bytes), &target_data_dir, &mut target_db)
                .expect("Deveria restaurar pacote de backup");

        assert!(restore_res.success);
        assert_eq!(target_db.count_songs().unwrap(), 1);

        let restored_song = target_db.get_song("song-exp-1").unwrap().unwrap();
        assert_eq!(restored_song.title, "Grandes Coisas Fez o Senhor");

        let church_name = target_db.get_setting("church_name").unwrap().unwrap();
        assert_eq!(church_name, "Igreja da Graça");

        // Verifica arquivos físicos restaurados
        let restored_img = target_data_dir
            .join("media")
            .join("images")
            .join("foto_culto.jpg");
        assert!(restored_img.exists());
        assert_eq!(fs::read(&restored_img).unwrap(), b"fake_jpeg_content");

        let restored_bible = target_data_dir.join("bibles").join("nvi.sqlite");
        assert!(restored_bible.exists());
        assert_eq!(fs::read(&restored_bible).unwrap(), b"fake_bible_sqlite");
    }
}

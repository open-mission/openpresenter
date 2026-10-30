use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, SecondsFormat, Utc};
use op_core::{
    AppMeta, BibleBook, BibleSearchResult, BibleVerse, BibleVersionSummary, DEFAULT_LOCALE,
    MediaFolder, MediaItem, MonitorSelection, OutputConfig, OutputKind, OutputMode, OutputsConfig,
    SectionType, SessionInfo, Setlist, SetlistSummary, Song, SongSection, SongSummary, Theme,
};
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};
use thiserror::Error;

/// Erros da camada de banco de dados.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("não foi possível abrir o banco de dados")]
    Open(#[source] rusqlite::Error),
    #[error("falha ao aplicar as migrações do banco de dados")]
    Migration(#[from] rusqlite_migration::Error),
    #[error("falha ao ler o estado do banco de dados")]
    Read(#[source] rusqlite::Error),
    #[error("falha ao gravar o estado do banco de dados")]
    Write(#[source] rusqlite::Error),
    #[error("data inválida no banco de dados: {0}")]
    InvalidTimestamp(String),
}

fn parse_timestamp(value: String) -> Result<DateTime<Utc>, DbError> {
    match DateTime::parse_from_rfc3339(&value) {
        Ok(dt) => Ok(dt.with_timezone(&Utc)),
        Err(_) => Err(DbError::InvalidTimestamp(value)),
    }
}

fn format_timestamp(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// Conexão SQLite já migrada, com o caminho do arquivo para diagnóstico.
pub struct Database {
    conn: Connection,
    path: PathBuf,
}

impl Database {
    /// Abre o banco, aplica os PRAGMAs e roda as migrações pendentes.
    pub fn open(path: &Path) -> Result<Self, DbError> {
        let mut conn = Connection::open(path).map_err(DbError::Open)?;

        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(DbError::Open)?;
        conn.pragma_update(None, "synchronous", "NORMAL")
            .map_err(DbError::Open)?;
        conn.pragma_update(None, "foreign_keys", "ON")
            .map_err(DbError::Open)?;
        conn.busy_timeout(Duration::from_secs(5))
            .map_err(DbError::Open)?;

        let migrations = Migrations::new(vec![
            M::up(include_str!("../migrations/0001_initial.sql")),
            M::up(include_str!("../migrations/0002_output_config.sql")),
            M::up(include_str!("../migrations/0003_songs_fts.sql")),
            M::up(include_str!("../migrations/0004_setlists.sql")),
            M::up(include_str!("../migrations/0005_media_folders.sql")),
            M::up(include_str!("../migrations/0006_app_settings.sql")),
            M::up(include_str!("../migrations/0007_bibles.sql")),
        ]);
        migrations
            .to_latest(&mut conn)
            .map_err(DbError::Migration)?;

        Ok(Self {
            conn,
            path: path.to_path_buf(),
        })
    }

    /// Versão atual do schema, controlada por `PRAGMA user_version`.
    pub fn schema_version(&self) -> Result<u32, DbError> {
        let version: i64 = self
            .conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(DbError::Read)?;
        Ok(version as u32)
    }

    /// Caminho do arquivo de banco aberto.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Lê o estado persistido, se já existir.
    fn load_meta(&self) -> Result<Option<AppMeta>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT app_version, first_run_at, last_run_at, theme, locale, updated_at
                 FROM app_meta WHERE id = 1",
            )
            .map_err(DbError::Read)?;
        let mut rows = stmt.query([]).map_err(DbError::Read)?;

        let Some(row) = rows.next().map_err(DbError::Read)? else {
            return Ok(None);
        };

        let app_version: String = row.get(0).map_err(DbError::Read)?;
        let first_run_at = parse_timestamp(row.get(1).map_err(DbError::Read)?)?;
        let last_run_at = parse_timestamp(row.get(2).map_err(DbError::Read)?)?;
        let theme_value: String = row.get(3).map_err(DbError::Read)?;
        let locale: String = row.get(4).map_err(DbError::Read)?;
        let updated_at = parse_timestamp(row.get(5).map_err(DbError::Read)?)?;

        Ok(Some(AppMeta {
            app_version,
            first_run_at,
            last_run_at,
            // Uma preferência de tema desconhecida não deve impedir a abertura;
            // nesse caso, volta ao padrão do sistema.
            theme: Theme::from_db_str(&theme_value).unwrap_or_default(),
            locale,
            updated_at,
        }))
    }

    /// Registra a sessão atual, preservando a primeira execução e as preferências.
    pub fn begin_session(
        &self,
        app_version: &str,
        now: DateTime<Utc>,
    ) -> Result<SessionInfo, DbError> {
        let previous = self.load_meta()?;
        let previous_last_run_at = previous.as_ref().map(|meta| meta.last_run_at);
        let first_run_at = previous.as_ref().map_or(now, |meta| meta.first_run_at);
        let theme = previous.as_ref().map_or(Theme::System, |meta| meta.theme);
        let locale = previous
            .as_ref()
            .map_or_else(|| DEFAULT_LOCALE.to_string(), |meta| meta.locale.clone());

        let first_run_at_text = format_timestamp(first_run_at);
        let last_run_at_text = format_timestamp(now);
        let updated_at_text = format_timestamp(now);

        let current = AppMeta {
            app_version: app_version.to_string(),
            first_run_at,
            last_run_at: now,
            theme,
            locale,
            updated_at: now,
        };

        self.conn
            .execute(
                "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
                 VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(id) DO UPDATE SET
                     app_version = excluded.app_version,
                     last_run_at = excluded.last_run_at,
                     theme = excluded.theme,
                     locale = excluded.locale,
                     updated_at = excluded.updated_at",
                rusqlite::params![
                    &current.app_version,
                    &first_run_at_text,
                    &last_run_at_text,
                    current.theme.as_str(),
                    &current.locale,
                    &updated_at_text,
                ],
            )
            .map_err(DbError::Write)?;

        Ok(SessionInfo {
            current,
            previous_last_run_at,
        })
    }

    /// Lê a configuração das saídas, preenchendo ausências com os padrões.
    pub fn load_outputs(&self) -> Result<OutputsConfig, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT output, enabled, monitor_key, monitor_name, mode
                 FROM output_config",
            )
            .map_err(DbError::Read)?;
        let mut rows = stmt.query([]).map_err(DbError::Read)?;
        let mut outputs = OutputsConfig::default();

        while let Some(row) = rows.next().map_err(DbError::Read)? {
            let output_value: String = row.get(0).map_err(DbError::Read)?;
            // Linhas com saída desconhecida não devem impedir a abertura.
            let Some(kind) = OutputKind::from_db_str(&output_value) else {
                continue;
            };
            let enabled: i64 = row.get(1).map_err(DbError::Read)?;
            let monitor_key: String = row.get(2).map_err(DbError::Read)?;
            let monitor_name: String = row.get(3).map_err(DbError::Read)?;
            let mode_value: String = row.get(4).map_err(DbError::Read)?;

            let config = outputs.get_mut(kind);
            config.enabled = enabled != 0;
            // Valores inválidos, possíveis apenas por edição manual, caem para Auto.
            config.monitor = MonitorSelection::from_db_key(&monitor_key, monitor_name);
            config.mode = OutputMode::from_db_str(&mode_value).unwrap_or_default();
        }

        Ok(outputs)
    }

    /// Grava a configuração de uma saída, preservando a outra.
    pub fn save_output(&self, config: &OutputConfig, now: DateTime<Utc>) -> Result<(), DbError> {
        let monitor_name = match &config.monitor {
            MonitorSelection::Display { name, .. } => name.as_str(),
            MonitorSelection::Auto | MonitorSelection::Primary => "",
        };
        let enabled = i64::from(config.enabled);

        self.conn
            .execute(
                "INSERT INTO output_config (output, enabled, monitor_key, monitor_name, mode, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(output) DO UPDATE SET
                     enabled = excluded.enabled,
                     monitor_key = excluded.monitor_key,
                     monitor_name = excluded.monitor_name,
                     mode = excluded.mode,
                     updated_at = excluded.updated_at",
                rusqlite::params![
                    config.kind.as_str(),
                    enabled,
                    config.monitor.as_db_key(),
                    monitor_name,
                    config.mode.as_str(),
                    format_timestamp(now),
                ],
            )
            .map_err(DbError::Write)?;

        Ok(())
    }

    /// Salva uma música no banco, inserindo metadados, seções, linhas e atualizando o índice FTS5.
    pub fn save_song(&self, song: &Song, now: DateTime<Utc>) -> Result<(), DbError> {
        let tx = self.conn.unchecked_transaction().map_err(DbError::Write)?;

        let now_str = format_timestamp(now);

        tx.execute(
            "INSERT INTO songs (id, title, artist, album, composer, song_key, bpm, lyrics_raw, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
             ON CONFLICT(id) DO UPDATE SET
                 title = excluded.title,
                 artist = excluded.artist,
                 album = excluded.album,
                 composer = excluded.composer,
                 song_key = excluded.song_key,
                 bpm = excluded.bpm,
                 lyrics_raw = excluded.lyrics_raw,
                 updated_at = excluded.updated_at",
            rusqlite::params![
                song.id,
                song.title,
                song.artist,
                song.album.as_deref().unwrap_or(""),
                song.composer.as_deref().unwrap_or(""),
                song.song_key.as_deref().unwrap_or(""),
                song.bpm,
                song.lyrics_raw,
                now_str,
            ],
        )
        .map_err(DbError::Write)?;

        // Remove seções e letras antigas
        tx.execute(
            "DELETE FROM song_sections WHERE song_id = ?1",
            rusqlite::params![song.id],
        )
        .map_err(DbError::Write)?;
        tx.execute(
            "DELETE FROM song_lyrics WHERE song_id = ?1",
            rusqlite::params![song.id],
        )
        .map_err(DbError::Write)?;

        // Insere seções e linhas
        let mut full_lyrics = String::new();
        for (sec_pos, section) in song.sections.iter().enumerate() {
            let sec_type_str = section.section_type.map(|t| t.as_str());
            tx.execute(
                "INSERT INTO song_sections (song_id, position, section_type, label) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![song.id, sec_pos as i64, sec_type_str, section.label],
            )
            .map_err(DbError::Write)?;

            for (line_pos, line) in section.lines.iter().enumerate() {
                tx.execute(
                    "INSERT INTO song_lyrics (song_id, section_position, line_position, line) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![song.id, sec_pos as i64, line_pos as i64, line],
                )
                .map_err(DbError::Write)?;

                if !full_lyrics.is_empty() {
                    full_lyrics.push(' ');
                }
                full_lyrics.push_str(line);
            }
        }

        if full_lyrics.is_empty() && !song.lyrics_raw.is_empty() {
            full_lyrics = song.lyrics_raw.clone();
        }

        // Atualiza a tabela virtual FTS5
        tx.execute(
            "DELETE FROM songs_fts WHERE song_id = ?1",
            rusqlite::params![song.id],
        )
        .map_err(DbError::Write)?;
        tx.execute(
            "INSERT INTO songs_fts (song_id, title, artist, lyrics) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![song.id, song.title, song.artist, full_lyrics],
        )
        .map_err(DbError::Write)?;

        tx.commit().map_err(DbError::Write)?;
        Ok(())
    }

    /// Busca uma música pelo ID, reconstruindo suas seções e linhas.
    pub fn get_song(&self, id: &str) -> Result<Option<Song>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT title, artist, album, composer, song_key, bpm, lyrics_raw
                 FROM songs WHERE id = ?1",
            )
            .map_err(DbError::Read)?;

        let mut rows = stmt.query(rusqlite::params![id]).map_err(DbError::Read)?;
        let Some(row) = rows.next().map_err(DbError::Read)? else {
            return Ok(None);
        };

        let title: String = row.get(0).map_err(DbError::Read)?;
        let artist: String = row.get(1).map_err(DbError::Read)?;
        let album: Option<String> = row.get(2).map_err(DbError::Read)?;
        let composer: Option<String> = row.get(3).map_err(DbError::Read)?;
        let song_key: Option<String> = row.get(4).map_err(DbError::Read)?;
        let bpm: Option<i32> = row.get(5).map_err(DbError::Read)?;
        let lyrics_raw: String = row.get(6).map_err(DbError::Read)?;

        // Carrega seções
        let mut sec_stmt = self
            .conn
            .prepare(
                "SELECT position, section_type, label FROM song_sections
                 WHERE song_id = ?1 ORDER BY position",
            )
            .map_err(DbError::Read)?;
        let sec_rows = sec_stmt
            .query_map(rusqlite::params![id], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, Option<String>>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .map_err(DbError::Read)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::Read)?;

        let mut sections = Vec::new();
        for (sec_pos, sec_type_opt, label) in sec_rows {
            let section_type = sec_type_opt.and_then(|st| SectionType::from_label(&st));
            let mut line_stmt = self
                .conn
                .prepare(
                    "SELECT line FROM song_lyrics
                     WHERE song_id = ?1 AND section_position = ?2
                     ORDER BY line_position",
                )
                .map_err(DbError::Read)?;
            let lines = line_stmt
                .query_map(rusqlite::params![id, sec_pos], |r| r.get::<_, String>(0))
                .map_err(DbError::Read)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(DbError::Read)?;

            sections.push(SongSection {
                section_type,
                label,
                lines,
            });
        }

        Ok(Some(Song {
            id: id.to_string(),
            title,
            artist,
            album: album.filter(|s| !s.is_empty()),
            composer: composer.filter(|s| !s.is_empty()),
            song_key: song_key.filter(|s| !s.is_empty()),
            bpm,
            sections,
            lyrics_raw,
        }))
    }

    /// Pesquisa músicas usando SQLite FTS5 por título, artista e trechos de letra.
    pub fn search_songs(&self, query: &str) -> Result<Vec<SongSummary>, DbError> {
        let clean = query.trim();
        if clean.is_empty() {
            let mut stmt = self
                .conn
                .prepare(
                    "SELECT id, title, artist, song_key, bpm, composer, album
                     FROM songs ORDER BY title COLLATE NOCASE ASC LIMIT 50",
                )
                .map_err(DbError::Read)?;
            let summaries = stmt
                .query_map([], |r| {
                    Ok(SongSummary {
                        id: r.get(0)?,
                        title: r.get(1)?,
                        artist: r.get(2)?,
                        song_key: r.get(3)?,
                        bpm: r.get(4)?,
                        composer: r.get(5)?,
                        album: r.get(6)?,
                        preview_snippet: None,
                    })
                })
                .map_err(DbError::Read)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(DbError::Read)?;
            return Ok(summaries);
        }

        // Tenta buscar via FTS5 MATCH
        let fts_query = format!("\"{}\"*", clean.replace('"', "\"\""));
        let fts_sql = "SELECT s.id, s.title, s.artist, s.song_key, s.bpm, s.composer, s.album,
                              snippet(songs_fts, 3, '<b>', '</b>', '...', 8)
                       FROM songs_fts f
                       JOIN songs s ON s.id = f.song_id
                       WHERE songs_fts MATCH ?1
                       ORDER BY rank
                       LIMIT 50";

        if let Ok(mut stmt) = self.conn.prepare(fts_sql)
            && let Ok(summaries) = stmt.query_map(rusqlite::params![fts_query], |r| {
                Ok(SongSummary {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    artist: r.get(2)?,
                    song_key: r.get(3)?,
                    bpm: r.get(4)?,
                    composer: r.get(5)?,
                    album: r.get(6)?,
                    preview_snippet: r.get(7)?,
                })
            })
        {
            let list = summaries.filter_map(Result::ok).collect::<Vec<_>>();
            if !list.is_empty() {
                return Ok(list);
            }
        }

        // Fallback para LIKE se FTS5 não trouxer resultados ou termo for parcial
        let pattern = format!(
            "%{}%",
            clean
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_")
        );
        let mut fallback_stmt = self
            .conn
            .prepare(
                "SELECT id, title, artist, song_key, bpm, composer, album
                 FROM songs
                 WHERE title LIKE ?1 ESCAPE '\\'
                    OR artist LIKE ?1 ESCAPE '\\'
                    OR lyrics_raw LIKE ?1 ESCAPE '\\'
                 ORDER BY title COLLATE NOCASE ASC LIMIT 50",
            )
            .map_err(DbError::Read)?;

        let list = fallback_stmt
            .query_map(rusqlite::params![pattern], |r| {
                Ok(SongSummary {
                    id: r.get(0)?,
                    title: r.get(1)?,
                    artist: r.get(2)?,
                    song_key: r.get(3)?,
                    bpm: r.get(4)?,
                    composer: r.get(5)?,
                    album: r.get(6)?,
                    preview_snippet: None,
                })
            })
            .map_err(DbError::Read)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::Read)?;

        Ok(list)
    }

    /// Remove uma música do banco e do índice FTS5.
    pub fn delete_song(&self, id: &str) -> Result<bool, DbError> {
        let count = self
            .conn
            .execute("DELETE FROM songs WHERE id = ?1", rusqlite::params![id])
            .map_err(DbError::Write)?;
        let _ = self.conn.execute(
            "DELETE FROM songs_fts WHERE song_id = ?1",
            rusqlite::params![id],
        );
        Ok(count > 0)
    }

    /// Salva ou atualiza uma Setlist completa com seus itens.
    pub fn save_setlist(&mut self, setlist: &Setlist) -> Result<(), DbError> {
        let tx = self.conn.transaction().map_err(DbError::Write)?;

        let items_json = serde_json::to_string(&setlist.items).unwrap_or_else(|_| "[]".to_string());
        let now_str = format_timestamp(Utc::now());
        let created_at = if setlist.created_at.is_empty() {
            now_str.clone()
        } else {
            setlist.created_at.clone()
        };
        let updated_at = now_str;

        // Se estiver marcando como ativa, desmarca as outras
        if setlist.is_active {
            tx.execute("UPDATE setlists SET is_active = 0", [])
                .map_err(DbError::Write)?;
        }

        tx.execute(
            r#"
            INSERT INTO setlists (
                id, title, description, wallpaper_path, wallpaper_mode,
                items_json, is_active, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                description = excluded.description,
                wallpaper_path = excluded.wallpaper_path,
                wallpaper_mode = excluded.wallpaper_mode,
                items_json = excluded.items_json,
                is_active = excluded.is_active,
                updated_at = excluded.updated_at
            "#,
            rusqlite::params![
                setlist.id,
                setlist.title,
                setlist.description,
                setlist.wallpaper_path,
                setlist.wallpaper_mode,
                items_json,
                if setlist.is_active { 1 } else { 0 },
                created_at,
                updated_at,
            ],
        )
        .map_err(DbError::Write)?;

        tx.commit().map_err(DbError::Write)?;
        Ok(())
    }

    /// Busca uma Setlist pelo seu ID.
    pub fn get_setlist(&self, id: &str) -> Result<Option<Setlist>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, title, description, wallpaper_path, wallpaper_mode,
                       items_json, is_active, created_at, updated_at
                FROM setlists
                WHERE id = ?1
                "#,
            )
            .map_err(DbError::Read)?;

        let mut rows = stmt
            .query_map(rusqlite::params![id], |row| {
                let id: String = row.get(0)?;
                let title: String = row.get(1)?;
                let description: Option<String> = row.get(2)?;
                let wallpaper_path: Option<String> = row.get(3)?;
                let wallpaper_mode: Option<String> = row.get(4)?;
                let items_json: String = row.get(5)?;
                let is_active_int: i32 = row.get(6)?;
                let created_at: String = row.get(7)?;
                let updated_at: String = row.get(8)?;

                let items: Vec<MediaItem> = serde_json::from_str(&items_json).unwrap_or_default();

                Ok(Setlist {
                    id,
                    title,
                    description,
                    wallpaper_path,
                    wallpaper_mode,
                    items,
                    is_active: is_active_int == 1,
                    created_at,
                    updated_at,
                })
            })
            .map_err(DbError::Read)?;

        match rows.next() {
            Some(res) => Ok(Some(res.map_err(DbError::Read)?)),
            None => Ok(None),
        }
    }

    /// Lista todas as Setlists cadastradas ordenadas por ativa e data de atualização.
    pub fn list_setlists(&self) -> Result<Vec<SetlistSummary>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, title, description, wallpaper_path, wallpaper_mode,
                       items_json, is_active, updated_at
                FROM setlists
                ORDER BY is_active DESC, updated_at DESC
                "#,
            )
            .map_err(DbError::Read)?;

        let summaries = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let title: String = row.get(1)?;
                let description: Option<String> = row.get(2)?;
                let wallpaper_path: Option<String> = row.get(3)?;
                let wallpaper_mode: Option<String> = row.get(4)?;
                let items_json: String = row.get(5)?;
                let is_active_int: i32 = row.get(6)?;
                let updated_at: String = row.get(7)?;

                let items: Vec<MediaItem> = serde_json::from_str(&items_json).unwrap_or_default();

                Ok(SetlistSummary {
                    id,
                    title,
                    description,
                    wallpaper_path,
                    wallpaper_mode,
                    item_count: items.len(),
                    is_active: is_active_int == 1,
                    updated_at,
                })
            })
            .map_err(DbError::Read)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::Read)?;

        Ok(summaries)
    }

    /// Remove uma Setlist pelo seu ID.
    pub fn delete_setlist(&mut self, id: &str) -> Result<bool, DbError> {
        let count = self
            .conn
            .execute("DELETE FROM setlists WHERE id = ?1", rusqlite::params![id])
            .map_err(DbError::Write)?;
        Ok(count > 0)
    }

    /// Define uma Setlist como a ativa no momento.
    pub fn set_active_setlist(&mut self, id: &str) -> Result<(), DbError> {
        let tx = self.conn.transaction().map_err(DbError::Write)?;
        tx.execute("UPDATE setlists SET is_active = 0", [])
            .map_err(DbError::Write)?;
        tx.execute(
            "UPDATE setlists SET is_active = 1, updated_at = ?2 WHERE id = ?1",
            rusqlite::params![id, format_timestamp(Utc::now())],
        )
        .map_err(DbError::Write)?;
        tx.commit().map_err(DbError::Write)?;
        Ok(())
    }

    /// Busca a Setlist atualmente ativa.
    pub fn get_active_setlist(&self) -> Result<Option<Setlist>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, title, description, wallpaper_path, wallpaper_mode,
                       items_json, is_active, created_at, updated_at
                FROM setlists
                WHERE is_active = 1
                LIMIT 1
                "#,
            )
            .map_err(DbError::Read)?;

        let mut rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let title: String = row.get(1)?;
                let description: Option<String> = row.get(2)?;
                let wallpaper_path: Option<String> = row.get(3)?;
                let wallpaper_mode: Option<String> = row.get(4)?;
                let items_json: String = row.get(5)?;
                let is_active_int: i32 = row.get(6)?;
                let created_at: String = row.get(7)?;
                let updated_at: String = row.get(8)?;

                let items: Vec<MediaItem> = serde_json::from_str(&items_json).unwrap_or_default();

                Ok(Setlist {
                    id,
                    title,
                    description,
                    wallpaper_path,
                    wallpaper_mode,
                    items,
                    is_active: is_active_int == 1,
                    created_at,
                    updated_at,
                })
            })
            .map_err(DbError::Read)?;

        match rows.next() {
            Some(res) => Ok(Some(res.map_err(DbError::Read)?)),
            None => Ok(None),
        }
    }

    /// Lista todas as pastas de mídias cadastradas.
    pub fn list_media_folders(&self) -> Result<Vec<MediaFolder>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, path, kind, created_at
                FROM media_folders
                ORDER BY name ASC
                "#,
            )
            .map_err(DbError::Read)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(MediaFolder {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    kind: row.get(3)?,
                    created_at: row.get(4)?,
                    subfolders: Vec::new(),
                })
            })
            .map_err(DbError::Read)?;

        let mut folders = Vec::new();
        for r in rows {
            folders.push(r.map_err(DbError::Read)?);
        }
        Ok(folders)
    }

    /// Salva ou atualiza uma pasta de mídias.
    pub fn save_media_folder(&self, folder: &MediaFolder) -> Result<(), DbError> {
        self.conn
            .execute(
                r#"
                INSERT INTO media_folders (id, name, path, kind, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    path = excluded.path,
                    kind = excluded.kind
                "#,
                rusqlite::params![
                    folder.id,
                    folder.name,
                    folder.path,
                    folder.kind,
                    folder.created_at,
                ],
            )
            .map_err(DbError::Write)?;
        Ok(())
    }

    /// Remove uma pasta de mídias pelo ID.
    pub fn delete_media_folder(&self, id: &str) -> Result<bool, DbError> {
        let affected = self
            .conn
            .execute(
                "DELETE FROM media_folders WHERE id = ?1",
                rusqlite::params![id],
            )
            .map_err(DbError::Write)?;
        Ok(affected > 0)
    }

    /// Obtém uma configuração persistida pela chave.
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, DbError> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM app_settings WHERE key = ?1")
            .map_err(DbError::Read)?;
        let mut rows = stmt.query(rusqlite::params![key]).map_err(DbError::Read)?;
        if let Some(row) = rows.next().map_err(DbError::Read)? {
            let val: String = row.get(0).map_err(DbError::Read)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    /// Salva ou atualiza uma configuração pela chave.
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), DbError> {
        let now = format_timestamp(Utc::now());
        self.conn
            .execute(
                r#"
                INSERT INTO app_settings (key, value, updated_at)
                VALUES (?1, ?2, ?3)
                ON CONFLICT(key) DO UPDATE SET
                    value = excluded.value,
                    updated_at = excluded.updated_at
                "#,
                rusqlite::params![key, value, now],
            )
            .map_err(DbError::Write)?;
        Ok(())
    }

    /// Lista todas as configurações salvas.
    pub fn list_settings(&self) -> Result<Vec<(String, String)>, DbError> {
        let mut stmt = self
            .conn
            .prepare("SELECT key, value FROM app_settings ORDER BY key ASC")
            .map_err(DbError::Read)?;
        let rows = stmt
            .query_map([], |row| {
                let k: String = row.get(0)?;
                let v: String = row.get(1)?;
                Ok((k, v))
            })
            .map_err(DbError::Read)?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r.map_err(DbError::Read)?);
        }
        Ok(res)
    }

    /// Lista todas as versões da Bíblia registradas.
    pub fn list_bibles(&self) -> Result<Vec<BibleVersionSummary>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, abbreviation, filename, is_default, created_at, updated_at
                 FROM bibles ORDER BY is_default DESC, name ASC",
            )
            .map_err(DbError::Read)?;

        let rows = stmt
            .query_map([], |row| {
                let is_default_int: i32 = row.get(4)?;
                Ok(BibleVersionSummary {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    abbreviation: row.get(2)?,
                    filename: row.get(3)?,
                    is_default: is_default_int == 1,
                    book_count: 0,
                    verse_count: 0,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })
            .map_err(DbError::Read)?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r.map_err(DbError::Read)?);
        }
        Ok(list)
    }

    /// Obtém uma versão da Bíblia pelo ID.
    pub fn get_bible(&self, id: &str) -> Result<Option<BibleVersionSummary>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, abbreviation, filename, is_default, created_at, updated_at
                 FROM bibles WHERE id = ?1",
            )
            .map_err(DbError::Read)?;

        let mut rows = stmt.query(rusqlite::params![id]).map_err(DbError::Read)?;
        if let Some(row) = rows.next().map_err(DbError::Read)? {
            let is_default_int: i32 = row.get(4).map_err(DbError::Read)?;
            Ok(Some(BibleVersionSummary {
                id: row.get(0).map_err(DbError::Read)?,
                name: row.get(1).map_err(DbError::Read)?,
                abbreviation: row.get(2).map_err(DbError::Read)?,
                filename: row.get(3).map_err(DbError::Read)?,
                is_default: is_default_int == 1,
                book_count: 0,
                verse_count: 0,
                created_at: row.get(5).map_err(DbError::Read)?,
                updated_at: row.get(6).map_err(DbError::Read)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Obtém a versão padrão da Bíblia (ou a primeira existente).
    pub fn get_default_bible(&self) -> Result<Option<BibleVersionSummary>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, abbreviation, filename, is_default, created_at, updated_at
                 FROM bibles ORDER BY is_default DESC, name ASC LIMIT 1",
            )
            .map_err(DbError::Read)?;

        let mut rows = stmt.query([]).map_err(DbError::Read)?;
        if let Some(row) = rows.next().map_err(DbError::Read)? {
            let is_default_int: i32 = row.get(4).map_err(DbError::Read)?;
            Ok(Some(BibleVersionSummary {
                id: row.get(0).map_err(DbError::Read)?,
                name: row.get(1).map_err(DbError::Read)?,
                abbreviation: row.get(2).map_err(DbError::Read)?,
                filename: row.get(3).map_err(DbError::Read)?,
                is_default: is_default_int == 1,
                book_count: 0,
                verse_count: 0,
                created_at: row.get(5).map_err(DbError::Read)?,
                updated_at: row.get(6).map_err(DbError::Read)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Salva ou atualiza uma versão da Bíblia.
    pub fn save_bible(&mut self, bible: &BibleVersionSummary) -> Result<(), DbError> {
        let is_default_int = if bible.is_default { 1 } else { 0 };

        // Se esta versão for padrão, desmarca as outras
        if bible.is_default {
            self.conn
                .execute("UPDATE bibles SET is_default = 0", [])
                .map_err(DbError::Write)?;
        }

        self.conn
            .execute(
                r#"
                INSERT INTO bibles (id, name, abbreviation, filename, is_default, created_at, updated_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    abbreviation = excluded.abbreviation,
                    filename = excluded.filename,
                    is_default = excluded.is_default,
                    updated_at = excluded.updated_at
                "#,
                rusqlite::params![
                    bible.id,
                    bible.name,
                    bible.abbreviation,
                    bible.filename,
                    is_default_int,
                    bible.created_at,
                    bible.updated_at,
                ],
            )
            .map_err(DbError::Write)?;
        Ok(())
    }

    /// Atualiza nome, abreviação e padrão de uma Bíblia.
    pub fn update_bible(
        &mut self,
        id: &str,
        name: &str,
        abbreviation: &str,
        is_default: bool,
    ) -> Result<bool, DbError> {
        let now = format_timestamp(Utc::now());
        if is_default {
            self.conn
                .execute("UPDATE bibles SET is_default = 0", [])
                .map_err(DbError::Write)?;
        }
        let is_default_int = if is_default { 1 } else { 0 };

        let affected = self
            .conn
            .execute(
                "UPDATE bibles SET name = ?1, abbreviation = ?2, is_default = ?3, updated_at = ?4 WHERE id = ?5",
                rusqlite::params![name, abbreviation, is_default_int, now, id],
            )
            .map_err(DbError::Write)?;

        Ok(affected > 0)
    }

    /// Define uma Bíblia como padrão.
    pub fn set_default_bible(&mut self, id: &str) -> Result<bool, DbError> {
        let now = format_timestamp(Utc::now());
        self.conn
            .execute("UPDATE bibles SET is_default = 0", [])
            .map_err(DbError::Write)?;

        let affected = self
            .conn
            .execute(
                "UPDATE bibles SET is_default = 1, updated_at = ?1 WHERE id = ?2",
                rusqlite::params![now, id],
            )
            .map_err(DbError::Write)?;

        Ok(affected > 0)
    }

    /// Remove uma versão da Bíblia do banco.
    pub fn delete_bible(&mut self, id: &str) -> Result<bool, DbError> {
        let affected = self
            .conn
            .execute("DELETE FROM bibles WHERE id = ?1", rusqlite::params![id])
            .map_err(DbError::Write)?;
        Ok(affected > 0)
    }
}

/// Leitor otimizado de um arquivo SQLite individual de Bíblia.
pub struct BibleReader {
    conn: Connection,
    path: PathBuf,
}

impl BibleReader {
    /// Abre uma conexão de leitura ao arquivo SQLite de Bíblia.
    pub fn open(path: &Path) -> Result<Self, DbError> {
        let conn = Connection::open(path).map_err(DbError::Open)?;
        conn.busy_timeout(Duration::from_secs(5))
            .map_err(DbError::Open)?;
        Ok(Self {
            conn,
            path: path.to_path_buf(),
        })
    }

    /// Caminho do arquivo.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Valida se o arquivo SQLite contém as tabelas essenciais (book e verse).
    pub fn validate_schema(&self) -> Result<(), DbError> {
        let has_book: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'book'",
                [],
                |row| row.get(0),
            )
            .map_err(DbError::Read)?;
        let has_verse: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'verse'",
                [],
                |row| row.get(0),
            )
            .map_err(DbError::Read)?;

        if has_book == 0 || has_verse == 0 {
            return Err(DbError::Read(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }

    /// Assegura que o índice em (book_id, chapter) exista para buscas ultrarrápidas.
    pub fn ensure_indexes(&self) -> Result<(), DbError> {
        let _ = self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_verse_book_chapter ON verse (book_id, chapter)",
            [],
        );
        Ok(())
    }

    /// Lê o valor de uma chave na tabela metadata, se existir.
    pub fn get_metadata(&self, key: &str) -> Result<Option<String>, DbError> {
        let has_meta: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'metadata'",
                [],
                |row| row.get(0),
            )
            .map_err(DbError::Read)?;
        if has_meta == 0 {
            return Ok(None);
        }

        let mut stmt = self
            .conn
            .prepare("SELECT value FROM metadata WHERE key = ?1")
            .map_err(DbError::Read)?;
        let mut rows = stmt.query(rusqlite::params![key]).map_err(DbError::Read)?;
        if let Some(row) = rows.next().map_err(DbError::Read)? {
            let val: String = row.get(0).map_err(DbError::Read)?;
            Ok(Some(val.trim_matches('\'').trim_matches('"').to_string()))
        } else {
            Ok(None)
        }
    }

    /// Retorna contagem de (livros, versículos).
    pub fn get_stats(&self) -> Result<(usize, usize), DbError> {
        let book_count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM book", [], |row| row.get(0))
            .map_err(DbError::Read)?;
        let verse_count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM verse", [], |row| row.get(0))
            .map_err(DbError::Read)?;
        Ok((book_count as usize, verse_count as usize))
    }

    /// Lista os livros da Bíblia com metadados canônicos e quantidade de capítulos.
    pub fn get_books(&self) -> Result<Vec<BibleBook>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT b.id, b.book_reference_id, b.testament_reference_id, b.name,
                       COALESCE(MAX(v.chapter), 1) as chapter_count
                FROM book b
                LEFT JOIN verse v ON v.book_id = b.id
                GROUP BY b.id
                ORDER BY b.id ASC
                "#,
            )
            .map_err(DbError::Read)?;

        let rows = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let book_ref: i64 = row.get(1).unwrap_or(id);
                let testament_ref: i64 = row.get(2).unwrap_or(1);
                let name: String = row.get(3)?;
                let chapter_count: i64 = row.get(4)?;
                let abbreviation = op_core::get_book_abbreviation(id).to_string();

                Ok(BibleBook {
                    id,
                    book_reference_id: book_ref,
                    testament_reference_id: testament_ref,
                    name,
                    abbreviation,
                    chapter_count,
                })
            })
            .map_err(DbError::Read)?;

        let mut books = Vec::new();
        for r in rows {
            books.push(r.map_err(DbError::Read)?);
        }
        Ok(books)
    }

    /// Lista os números de capítulos de um determinado livro.
    pub fn get_chapters(&self, book_id: i64) -> Result<Vec<i64>, DbError> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT chapter FROM verse WHERE book_id = ?1 ORDER BY chapter ASC")
            .map_err(DbError::Read)?;
        let rows = stmt
            .query_map(rusqlite::params![book_id], |row| row.get(0))
            .map_err(DbError::Read)?;

        let mut chapters = Vec::new();
        for r in rows {
            chapters.push(r.map_err(DbError::Read)?);
        }
        Ok(chapters)
    }

    /// Retorna todos os versículos de um capítulo.
    pub fn get_verses(&self, book_id: i64, chapter: i64) -> Result<Vec<BibleVerse>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, book_id, chapter, verse, text FROM verse WHERE book_id = ?1 AND chapter = ?2 ORDER BY verse ASC"
            )
            .map_err(DbError::Read)?;

        let rows = stmt
            .query_map(rusqlite::params![book_id, chapter], |row| {
                Ok(BibleVerse {
                    id: row.get(0)?,
                    book_id: row.get(1)?,
                    chapter: row.get(2)?,
                    verse: row.get(3)?,
                    text: row.get(4)?,
                })
            })
            .map_err(DbError::Read)?;

        let mut verses = Vec::new();
        for r in rows {
            verses.push(r.map_err(DbError::Read)?);
        }
        Ok(verses)
    }

    /// Retorna um intervalo de versículos de um capítulo.
    pub fn get_verse_range(
        &self,
        book_id: i64,
        chapter: i64,
        verse_start: i64,
        verse_end: i64,
    ) -> Result<Vec<BibleVerse>, DbError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, book_id, chapter, verse, text FROM verse WHERE book_id = ?1 AND chapter = ?2 AND verse >= ?3 AND verse <= ?4 ORDER BY verse ASC"
            )
            .map_err(DbError::Read)?;

        let rows = stmt
            .query_map(
                rusqlite::params![book_id, chapter, verse_start, verse_end],
                |row| {
                    Ok(BibleVerse {
                        id: row.get(0)?,
                        book_id: row.get(1)?,
                        chapter: row.get(2)?,
                        verse: row.get(3)?,
                        text: row.get(4)?,
                    })
                },
            )
            .map_err(DbError::Read)?;

        let mut verses = Vec::new();
        for r in rows {
            verses.push(r.map_err(DbError::Read)?);
        }
        Ok(verses)
    }

    /// Busca textual livre em versículos (LIKE com suporte a palavra exata).
    ///
    /// Se `exact` for `true`, ou se `query` estiver entre aspas (ex: `"amor"`),
    /// a busca realiza o casamento estrito por palavra inteira isolada por pontuações ou espaços,
    /// impedindo que "amor" corresponda a "amorreus" ou "desamor".
    pub fn search_verses(
        &self,
        query: &str,
        limit: usize,
        exact: bool,
    ) -> Result<Vec<BibleSearchResult>, DbError> {
        let trimmed = query.trim();
        let has_quotes = (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
            || (trimmed.starts_with('“') && trimmed.ends_with('”') && trimmed.len() >= 2);

        let clean_term = if has_quotes {
            trimmed
                .trim_matches('"')
                .trim_matches('“')
                .trim_matches('”')
                .trim()
        } else {
            trimmed
        };

        let is_exact = exact || has_quotes;
        let like_param = format!("%{}%", clean_term);

        // Se a busca for exata, buscamos uma janela maior no banco para permitir o filtro por palavra inteira
        let sql_limit = if is_exact {
            (limit * 5).max(150) as i64
        } else {
            limit as i64
        };

        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT b.id, b.name, v.chapter, v.verse, v.text
                FROM verse v
                JOIN book b ON b.id = v.book_id
                WHERE v.text LIKE ?1
                ORDER BY v.id ASC
                LIMIT ?2
                "#,
            )
            .map_err(DbError::Read)?;

        let rows = stmt
            .query_map(rusqlite::params![like_param, sql_limit], |row| {
                let book_id: i64 = row.get(0)?;
                let book_name: String = row.get(1)?;
                let chapter: i64 = row.get(2)?;
                let verse: i64 = row.get(3)?;
                let text: String = row.get(4)?;
                let book_abbreviation = op_core::get_book_abbreviation(book_id).to_string();

                Ok(BibleSearchResult {
                    book_id,
                    book_name,
                    book_abbreviation,
                    chapter,
                    verse,
                    text,
                })
            })
            .map_err(DbError::Read)?;

        let mut results = Vec::new();
        for r in rows {
            let res = r.map_err(DbError::Read)?;
            if is_exact {
                if matches_whole_word(&res.text, clean_term) {
                    results.push(res);
                    if results.len() >= limit {
                        break;
                    }
                }
            } else {
                results.push(res);
            }
        }

        Ok(results)
    }
}

/// Verifica se `query` ocorre como uma palavra inteira (isolada por caracteres não-alfanuméricos)
/// dentro de `text`. A comparação é case-insensitive e respeita caracteres acentuados Unicode.
pub fn matches_whole_word(text: &str, query: &str) -> bool {
    let q = query.trim();
    if q.is_empty() {
        return false;
    }

    let text_lower: Vec<char> = text.to_lowercase().chars().collect();
    let query_lower: Vec<char> = q.to_lowercase().chars().collect();
    let q_len = query_lower.len();

    if q_len == 0 || text_lower.len() < q_len {
        return false;
    }

    for (i, window) in text_lower.windows(q_len).enumerate() {
        if window == query_lower.as_slice() {
            // Verifica o caractere anterior
            let is_boundary_start = if i == 0 {
                true
            } else {
                !text_lower[i - 1].is_alphanumeric()
            };

            // Verifica o caractere posterior
            let end_idx = i + q_len;
            let is_boundary_end = if end_idx >= text_lower.len() {
                true
            } else {
                !text_lower[end_idx].is_alphanumeric()
            };

            if is_boundary_start && is_boundary_end {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use op_core::{MonitorSelection, OutputConfig, OutputKind, OutputMode, OutputsConfig, Theme};

    fn ts(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc
            .with_ymd_and_hms(year, month, day, hour, minute, 0)
            .single()
            .unwrap()
    }

    #[test]
    fn open_creates_schema_and_applies_pragmas() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("open-presenter.db");
        let db = Database::open(&path).unwrap();

        assert_eq!(db.schema_version().unwrap(), 7);
        let table_count: i64 = db
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'app_meta'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 1);
        let journal_mode: String = db
            .conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .unwrap();
        assert_eq!(journal_mode.to_lowercase(), "wal");
        let foreign_keys: i64 = db
            .conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(foreign_keys, 1);
        let busy_timeout: i64 = db
            .conn
            .query_row("PRAGMA busy_timeout", [], |row| row.get(0))
            .unwrap();
        assert_eq!(busy_timeout, 5000);
        let synchronous: i64 = db
            .conn
            .query_row("PRAGMA synchronous", [], |row| row.get(0))
            .unwrap();
        assert_eq!(synchronous, 1);
    }

    #[test]
    fn open_creates_schema_v7_with_bibles() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

        assert_eq!(db.schema_version().unwrap(), 7);
        let table_count: i64 = db
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('output_config', 'songs', 'songs_fts', 'setlists', 'media_folders', 'app_settings', 'bibles')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 7);
    }

    #[test]
    fn save_and_manage_bibles_works() {
        let dir = tempfile::tempdir().unwrap();
        let mut db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

        assert!(db.list_bibles().unwrap().is_empty());

        let bible1 = BibleVersionSummary {
            id: "bible-ara".into(),
            name: "Almeida Revista e Atualizada".into(),
            abbreviation: "ARA".into(),
            filename: "ARA.sqlite".into(),
            is_default: true,
            book_count: 66,
            verse_count: 31104,
            created_at: "2026-10-29T10:00:00.000Z".into(),
            updated_at: "2026-10-29T10:00:00.000Z".into(),
        };

        db.save_bible(&bible1).unwrap();

        let list = db.list_bibles().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].abbreviation, "ARA");
        assert!(list[0].is_default);

        // Adiciona segunda bíblia
        let bible2 = BibleVersionSummary {
            id: "bible-nvi".into(),
            name: "Nova Versao Internacional".into(),
            abbreviation: "NVI".into(),
            filename: "NVI.sqlite".into(),
            is_default: false,
            book_count: 66,
            verse_count: 31104,
            created_at: "2026-10-29T11:00:00.000Z".into(),
            updated_at: "2026-10-29T11:00:00.000Z".into(),
        };
        db.save_bible(&bible2).unwrap();
        assert_eq!(db.list_bibles().unwrap().len(), 2);

        // Atualiza bible1 (renomear)
        db.update_bible(
            "bible-ara",
            "Almeida Revista Atualizada (Oficial)",
            "ARA-BR",
            false,
        )
        .unwrap();
        let updated = db.get_bible("bible-ara").unwrap().unwrap();
        assert_eq!(updated.name, "Almeida Revista Atualizada (Oficial)");
        assert_eq!(updated.abbreviation, "ARA-BR");

        // Define bible2 como default
        db.set_default_bible("bible-nvi").unwrap();
        let def = db.get_default_bible().unwrap().unwrap();
        assert_eq!(def.id, "bible-nvi");

        // Deleta bible1
        assert!(db.delete_bible("bible-ara").unwrap());
        assert_eq!(db.list_bibles().unwrap().len(), 1);
    }

    #[test]
    fn bible_reader_and_queries_work() {
        let dir = tempfile::tempdir().unwrap();
        let bible_path = dir.path().join("test_bible.sqlite");

        // Cria banco de teste no formato das bíblias
        {
            let conn = rusqlite::Connection::open(&bible_path).unwrap();
            conn.execute_batch(
                r#"
                CREATE TABLE metadata (key VARCHAR(255) PRIMARY KEY, value VARCHAR(255));
                INSERT INTO metadata VALUES ('name', 'Almeida Revista e Atualizada');
                CREATE TABLE book (id INTEGER PRIMARY KEY, book_reference_id INTEGER, testament_reference_id INTEGER, name VARCHAR(50));
                INSERT INTO book VALUES (43, 43, 2, 'João');
                CREATE TABLE verse (id INTEGER PRIMARY KEY, book_id INTEGER, chapter INTEGER, verse INTEGER, text TEXT);
                INSERT INTO verse VALUES (1, 43, 3, 16, 'Porque Deus amou o mundo...');
                INSERT INTO verse VALUES (2, 43, 3, 17, 'Porquanto Deus enviou seu Filho...');
                "#,
            )
            .unwrap();
        }

        let reader = BibleReader::open(&bible_path).unwrap();
        reader.validate_schema().unwrap();
        reader.ensure_indexes().unwrap();

        assert_eq!(
            reader.get_metadata("name").unwrap(),
            Some("Almeida Revista e Atualizada".to_string())
        );

        let (books_count, verses_count) = reader.get_stats().unwrap();
        assert_eq!(books_count, 1);
        assert_eq!(verses_count, 2);

        let books = reader.get_books().unwrap();
        assert_eq!(books.len(), 1);
        assert_eq!(books[0].name, "João");
        assert_eq!(books[0].abbreviation, "Jo");

        let chapters = reader.get_chapters(43).unwrap();
        assert_eq!(chapters, vec![3]);

        let verses = reader.get_verses(43, 3).unwrap();
        assert_eq!(verses.len(), 2);
        assert_eq!(verses[0].verse, 16);

        let search = reader.search_verses("amou o mundo", 10, false).unwrap();
        assert_eq!(search.len(), 1);
        assert_eq!(search[0].chapter, 3);
        assert_eq!(search[0].verse, 16);

        // Teste de busca exata e sintaxe com aspas
        let exact_search = reader.search_verses("\"amou\"", 10, false).unwrap();
        assert_eq!(exact_search.len(), 1);
        assert_eq!(exact_search[0].verse, 16);
    }

    #[test]
    fn test_matches_whole_word_rules() {
        assert!(matches_whole_word("O amor jamais acaba", "amor"));
        assert!(matches_whole_word("amor,", "amor"));
        assert!(matches_whole_word("O amor de Deus", "amor"));
        assert!(matches_whole_word("Amor!", "amor"));
        assert!(matches_whole_word("Deus é amor.", "amor"));

        // Casos que não devem casar
        assert!(!matches_whole_word(
            "E habitou na terra dos amorreus",
            "amor"
        ));
        assert!(!matches_whole_word("Ele é um namorado fiel", "amor"));
        assert!(!matches_whole_word("O desamor corrompeu tudo", "amor"));
        assert!(!matches_whole_word("Sentiu grandes amores", "amor"));
    }

    #[test]
    fn real_sample_bible_sqlite_can_be_read_if_present() {
        let sample_path = std::path::Path::new("/Users/claudio/Downloads/bibles_ARA.sqlite");
        if !sample_path.exists() {
            return;
        }

        let reader = BibleReader::open(sample_path).unwrap();
        reader.validate_schema().unwrap();
        let (books_count, verses_count) = reader.get_stats().unwrap();
        assert_eq!(books_count, 66);
        assert_eq!(verses_count, 31104);

        let books = reader.get_books().unwrap();
        assert_eq!(books.len(), 66);
        assert_eq!(books[0].name, "Gênesis");
        assert_eq!(books[42].name, "João");

        let verses = reader.get_verses(43, 3).unwrap();
        assert_eq!(verses.len(), 36);
        let v16 = verses.iter().find(|v| v.verse == 16).unwrap();
        assert!(v16.text.contains("amou ao mundo"));

        let search = reader.search_verses("amou ao mundo", 5, false).unwrap();
        assert!(!search.is_empty());

        // Busca exata por amorreus não deve vir em busca exata por "amor"
        let amor_exact = reader.search_verses("\"amor\"", 20, true).unwrap();
        for r in &amor_exact {
            assert!(matches_whole_word(&r.text, "amor"));
        }
    }

    #[test]
    fn save_and_manage_app_settings_works() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

        // Inicialmente vazio
        assert_eq!(db.get_setting("public_margins").unwrap(), None);

        // Salva configuração
        db.set_setting(
            "public_margins",
            r#"{"top": 40, "bottom": 40, "horizontal": 60}"#,
        )
        .unwrap();

        let val = db.get_setting("public_margins").unwrap();
        assert!(val.is_some());
        assert!(val.unwrap().contains("top"));

        // Atualiza configuração
        db.set_setting(
            "public_margins",
            r#"{"top": 80, "bottom": 80, "horizontal": 100}"#,
        )
        .unwrap();
        let updated = db.get_setting("public_margins").unwrap().unwrap();
        assert!(updated.contains("80"));

        // Lista configurações
        db.set_setting("church_brand", r#"{"name": "Igreja Central"}"#)
            .unwrap();
        let all = db.list_settings().unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn migrates_v1_database_to_v7_preserving_app_meta() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("open-presenter.db");
        {
            let conn = rusqlite::Connection::open(&path).unwrap();
            conn.execute_batch(include_str!("../migrations/0001_initial.sql"))
                .unwrap();
            conn.pragma_update(None, "user_version", 1).unwrap();
            conn.execute(
                "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
                 VALUES (1, '0.1.0', '2026-10-28T10:00:00.000Z', '2026-10-28T10:00:00.000Z', 'system', 'pt-BR', '2026-10-28T10:00:00.000Z')",
                [],
            )
            .unwrap();
        }

        let db = Database::open(&path).unwrap();
        assert_eq!(db.schema_version().unwrap(), 7);
        let meta = db.load_meta().unwrap().unwrap();
        assert_eq!(meta.first_run_at, ts(2026, 10, 28, 10, 0));
        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM output_config", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn load_outputs_without_rows_returns_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
        assert_eq!(db.load_outputs().unwrap(), OutputsConfig::default());
    }

    #[test]
    fn saves_and_loads_public_and_stage_outputs() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
        let public = OutputConfig {
            kind: OutputKind::Public,
            enabled: false,
            monitor: MonitorSelection::Display {
                id: "7".into(),
                name: "DELL".into(),
            },
            mode: OutputMode::Fullscreen,
        };
        let stage = OutputConfig {
            kind: OutputKind::Stage,
            enabled: true,
            monitor: MonitorSelection::Primary,
            mode: OutputMode::Windowed,
        };

        db.save_output(&public, ts(2026, 10, 28, 11, 0)).unwrap();
        db.save_output(&stage, ts(2026, 10, 28, 11, 1)).unwrap();

        let loaded = db.load_outputs().unwrap();
        assert_eq!(loaded.public, public);
        assert_eq!(loaded.stage, stage);
    }

    #[test]
    fn updating_one_output_preserves_the_other() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
        let mut public = OutputConfig::default_for(OutputKind::Public);
        let stage = OutputConfig::default_for(OutputKind::Stage);
        db.save_output(&public, ts(2026, 10, 28, 11, 0)).unwrap();
        db.save_output(&stage, ts(2026, 10, 28, 11, 0)).unwrap();

        public.mode = OutputMode::Maximized;
        db.save_output(&public, ts(2026, 10, 28, 11, 5)).unwrap();

        let loaded = db.load_outputs().unwrap();
        assert_eq!(loaded.public.mode, OutputMode::Maximized);
        assert_eq!(loaded.stage, stage);
    }

    #[test]
    fn invalid_stored_mode_and_monitor_fall_back_to_auto() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
        db.conn
            .pragma_update(None, "ignore_check_constraints", true)
            .unwrap();
        db.conn
            .execute(
                "INSERT INTO output_config (output, enabled, monitor_key, monitor_name, mode, updated_at)
                 VALUES ('public', 1, 'bogus', '', 'invalid', '2026-10-28T11:00:00.000Z')",
                [],
            )
            .unwrap();

        let loaded = db.load_outputs().unwrap();
        assert_eq!(loaded.public.monitor, MonitorSelection::Auto);
        assert_eq!(loaded.public.mode, OutputMode::Auto);
    }

    #[test]
    fn open_fails_when_parent_directory_is_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing").join("db.sqlite");
        assert!(matches!(Database::open(&path), Err(DbError::Open(_))));
    }

    #[test]
    fn first_session_creates_meta() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
        let now = ts(2026, 10, 28, 10, 0);
        let info = db.begin_session("0.1.0", now).unwrap();

        assert_eq!(info.previous_last_run_at, None);
        assert_eq!(info.current.app_version, "0.1.0");
        assert_eq!(info.current.first_run_at, now);
        assert_eq!(info.current.last_run_at, now);
        assert_eq!(info.current.theme, Theme::System);
        assert_eq!(info.current.locale, "pt-BR");
        assert!(db.load_meta().unwrap().is_some());
    }

    #[test]
    fn second_session_preserves_first_run_and_reports_previous() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("open-presenter.db");
        let first = ts(2026, 10, 28, 10, 0);
        {
            let db = Database::open(&path).unwrap();
            db.begin_session("0.1.0", first).unwrap();
            db.conn
                .execute(
                    "UPDATE app_meta SET theme = 'dark', locale = 'en-US' WHERE id = 1",
                    [],
                )
                .unwrap();
        }
        let second = ts(2026, 10, 29, 18, 30);
        let db = Database::open(&path).unwrap();
        let info = db.begin_session("0.2.0", second).unwrap();

        assert_eq!(info.previous_last_run_at, Some(first));
        assert_eq!(info.current.first_run_at, first);
        assert_eq!(info.current.last_run_at, second);
        assert_eq!(info.current.app_version, "0.2.0");
        assert_eq!(info.current.theme, Theme::Dark);
        assert_eq!(info.current.locale, "en-US");
        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM app_meta", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn invalid_timestamp_is_reported_without_panicking() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();
        db.conn
            .execute(
                "INSERT INTO app_meta (id, app_version, first_run_at, last_run_at, theme, locale, updated_at)
                 VALUES (1, '0.1.0', 'not-a-date', 'not-a-date', 'system', 'pt-BR', 'not-a-date')",
                [],
            )
            .unwrap();
        assert!(matches!(
            db.begin_session("0.1.0", ts(2026, 10, 28, 10, 0)),
            Err(DbError::InvalidTimestamp(_))
        ));
    }

    #[test]
    fn newer_schema_is_reported_as_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("open-presenter.db");
        {
            let conn = rusqlite::Connection::open(&path).unwrap();
            conn.pragma_update(None, "user_version", 99).unwrap();
        }
        assert!(matches!(Database::open(&path), Err(DbError::Migration(_))));
    }

    #[test]
    fn save_song_and_fts_search_works() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

        let song = Song {
            id: "song-123".into(),
            title: "Porque Ele Vive".into(),
            artist: "Harpa Cristã".into(),
            album: None,
            composer: Some("Bill Gaither".into()),
            song_key: Some("G".into()),
            bpm: Some(72),
            sections: vec![
                SongSection {
                    section_type: Some(SectionType::Verse),
                    label: "Verso 1".into(),
                    lines: vec![
                        "Deus enviou Seu Filho amado".into(),
                        "Para morrer em meu lugar".into(),
                    ],
                },
                SongSection {
                    section_type: Some(SectionType::Chorus),
                    label: "Refrão".into(),
                    lines: vec![
                        "Porque Ele vive, posso crer no amanhã".into(),
                        "Porque Ele vive, temor não há".into(),
                    ],
                },
            ],
            lyrics_raw: "## Verso 1\nDeus enviou Seu Filho amado\nPara morrer em meu lugar\n\n## Refrão\nPorque Ele vive, posso crer no amanhã\nPorque Ele vive, temor não há".into(),
        };

        db.save_song(&song, ts(2026, 10, 28, 20, 0)).unwrap();

        // 1. Busca por ID
        let loaded = db.get_song("song-123").unwrap().expect("encontra música");
        assert_eq!(loaded.title, "Porque Ele Vive");
        assert_eq!(loaded.sections.len(), 2);
        assert_eq!(loaded.sections[0].lines.len(), 2);

        // 2. Busca por termo no título
        let by_title = db.search_songs("porque").unwrap();
        assert_eq!(by_title.len(), 1);
        assert_eq!(by_title[0].id, "song-123");

        // 3. Busca FTS5 por termo que só existe no meio da letra ("sepulcro" não existe aqui, mas "morrer" existe)
        let by_lyrics = db.search_songs("morrer").unwrap();
        assert_eq!(by_lyrics.len(), 1);
        assert_eq!(by_lyrics[0].title, "Porque Ele Vive");

        // 4. Busca por artista
        let by_artist = db.search_songs("Harpa").unwrap();
        assert_eq!(by_artist.len(), 1);

        // 5. Exclusão
        assert!(db.delete_song("song-123").unwrap());
        assert!(db.get_song("song-123").unwrap().is_none());
        assert!(db.search_songs("porque").unwrap().is_empty());
    }

    #[test]
    fn save_and_manage_multiple_setlists_works() {
        let dir = tempfile::tempdir().unwrap();
        let mut db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

        let s1 = Setlist {
            id: "set-1".into(),
            title: "Culto de Domingo - Manhã".into(),
            description: Some("Culto com louvores de celebração".into()),
            wallpaper_path: Some("/media/wallpapers/worship.jpg".into()),
            wallpaper_mode: Some("cover".into()),
            items: vec![MediaItem {
                id: "song-1".into(),
                title: "Porque Ele Vive".into(),
                artist_or_ref: None,
                kind: op_core::MediaKind::Song,
                slides: vec![],
                align_h: None,
                align_v: None,
            }],
            is_active: true,
            created_at: "".into(),
            updated_at: "".into(),
        };

        let s2 = Setlist {
            id: "set-2".into(),
            title: "Culto de Domingo - Noite".into(),
            description: Some("Culto evangelístico".into()),
            wallpaper_path: None,
            wallpaper_mode: Some("contain".into()),
            items: vec![],
            is_active: false,
            created_at: "".into(),
            updated_at: "".into(),
        };

        db.save_setlist(&s1).unwrap();
        db.save_setlist(&s2).unwrap();

        let active = db.get_active_setlist().unwrap().unwrap();
        assert_eq!(active.id, "set-1");
        assert_eq!(active.title, "Culto de Domingo - Manhã");
        assert_eq!(active.items.len(), 1);

        let list = db.list_setlists().unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].id, "set-1"); // active first

        // Alterna ativa
        db.set_active_setlist("set-2").unwrap();
        let active2 = db.get_active_setlist().unwrap().unwrap();
        assert_eq!(active2.id, "set-2");

        // Deleta
        assert!(db.delete_setlist("set-1").unwrap());
        assert_eq!(db.list_setlists().unwrap().len(), 1);
    }

    #[test]
    fn save_and_manage_media_folders_works() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("open-presenter.db")).unwrap();

        let f1 = MediaFolder {
            id: "folder-1".into(),
            name: "Imagens do Culto".into(),
            path: "/dummy/path/culto".into(),
            kind: "images".into(),
            created_at: "2026-10-29T10:00:00.000Z".into(),
            subfolders: Vec::new(),
        };

        db.save_media_folder(&f1).unwrap();
        let list = db.list_media_folders().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "folder-1");
        assert_eq!(list[0].name, "Imagens do Culto");

        assert!(db.delete_media_folder("folder-1").unwrap());
        assert_eq!(db.list_media_folders().unwrap().len(), 0);
    }
}

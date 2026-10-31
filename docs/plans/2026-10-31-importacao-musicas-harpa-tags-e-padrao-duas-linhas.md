# Plano de Execução — Importação de Músicas, Harpa Cristã, Tags e Padrão de 2 Linhas por Slide

## Etapas

1. **Camada Core (`op-core`)**:
   - Adicionar `tags: Vec<String>` em `Song` e `SongSummary` com `#[serde(default)]`.
   - Implementar `format_lyrics_lines_per_slide(raw: &str, lines_per_slide: usize) -> String`.
   - Adicionar testes unitários em `op-core` para o particionamento de estrofes em 2 linhas.

2. **Camada de Banco de Dados (`op-db`)**:
   - Criar migração `0009_song_tags.sql`.
   - Atualizar `save_song`, `get_song`, `search_songs` para gravar e recuperar tags.
   - Implementar `list_all_song_tags(&self) -> Result<Vec<String>, DbError>`.
   - Implementar `save_songs_batch(&self, songs: &[Song], now: DateTime<Utc>) -> Result<(), DbError>` para importações em massa de alta performance.
   - Atualizar testes de migração e esquema para versão 9.

3. **Camada de Backend HTTP (`op-admin`)**:
   - Adicionar rotas:
     - `POST /api/songs/import` (importação de arquivos JSON/TXT/MD ou payloads).
     - `POST /api/songs/harpa/install` (instalador da Harpa Cristã).
     - `GET /api/songs/tags` (lista de tags ativas).
   - Ajustar `online_lyrics::fetch_online_song` para respeitar configuração `lines_per_slide` (padrão 2).
   - Adicionar testes de rotas em `tests.rs`.

4. **Camada Frontend (`frontend`)**:
   - Atualizar `types.ts` com `tags?: string[]` e `SongSettings`.
   - Implementar `formatLyricsLinesPerSlide` em `songParser.ts`.
   - Atualizar `SongEditorModal.svelte` com seletor/editor de tags e botão de reformatação de linhas por slide.
   - Atualizar `BottomPanel.svelte` com filtro por tags/grupos e botão de Importar / Instalar Harpa Cristã.
   - Atualizar `SettingsModal.svelte` com seção de Instalação da Harpa Cristã, Importação e configuração de Linhas por Slide.

5. **Portões de Qualidade e Documentação**:
   - Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
   - Executar `npm --prefix frontend run check` e `npm --prefix frontend run build`.
   - Atualizar `CHANGELOG.md` e `docs/README.md`.
   - Atualizar `TODO.md` e commitar.

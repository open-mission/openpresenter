# Spec — Command Palette, Catálogo com SQLite FTS5 e Cadastro de Músicas

- **Status:** aprovada
- **Data:** 2026-10-28
- **Autores:** OpenPresenter Core Team
- **Decisão relacionada:** [ADR 0005](../decisions/0005-command-palette-e-catalogo-musicas-fts5.md)

## 1. Intenção

Implementar um Command Palette completo (`⌘K` / `Ctrl+K`), busca no banco SQLite com FTS5 por título e trechos de letra, busca de letras na internet através dos provedores Holyrics e Vagalume, modal de cadastro e edição de músicas em dois painéis (editor + preview 16:9 em tempo real) e reorganização da sidebar operacional para o conceito de **Setlist** com atalhos rápidos de inclusão.

## 2. Escopo

### Incluído

1. **Camada de Dados e Domínio:**
   - Modelos em `op-core`: `Song`, `SongSection`, `SectionType`, `SongSummary`, `OnlineLyricHit`.
   - Lógica de segmentação de letras:
     - Linhas iniciando com `## <Rótulo>` identificam o tipo e nome da seção (ex: `## Verso 1`, `## Refrão`, `## Ponte`).
     - Linhas em branco (`\n\n`) definem quebra entre slides dentro da seção.
   - Migração `0003_songs_fts.sql` em `op-db`:
     - Tabelas `songs`, `song_sections`, `song_lyrics`.
     - Tabela virtual `songs_fts` usando SQLite FTS5 (`unicode61 remove_diacritics 2`) indexando `title`, `artist` e `lyrics`.
   - Operações em `op-db`: `save_song`, `get_song`, `search_songs`, `delete_song`.
2. **Backend Axum (`op-admin`):**
   - `GET /api/songs?q=...`: busca no banco local via FTS5.
   - `POST /api/songs`: cadastra ou atualiza uma música no banco.
   - `GET /api/songs/online/search?q=...`: pesquisa em provedores online (Holyrics e Vagalume).
   - `GET /api/songs/online/fetch`: baixa e converte a letra do provedor para a estrutura canônica.
   - `POST /api/setlist/add`: adiciona uma música ou mídia à Setlist ativa em memória e emite evento WebSocket.
3. **Frontend Svelte (`frontend/`):**
   - **Command Palette (`CommandPalette.svelte`):**
     - Acessível por `⌘K` (Mac), `Ctrl+K` (Windows/Linux), clique na busca do header ou botão `+` da Setlist.
     - Campo de busca responsivo com busca instantânea no banco local.
     - Exibição de resultados locais com botão "Adicionar à Setlist" e atalho `Shift + Enter`.
     - Opção "Criar nova música manualmente" (abre modal de cadastro).
     - Opção "Buscar na internet" e aba de busca online com provedores (Holyrics, Vagalume).
   - **Modal de Cadastro/Edição de Músicas (`SongEditorModal.svelte`):**
     - Ocupa aproximadamente 95% da tela.
     - Painel esquerdo (Editor): campos de título, artista/cantor, tom musical, BPM, compositor e textarea para letra crua com suporte a `## <Seção>` e separação por linhas vazias.
     - Painel direito (Preview): miniaturas 16:9 em grade atualizadas em tempo real conforme digitação, com cabeçalhos de seção coloridos estilo ProPresenter.
     - Ações de salvar no banco e adicionar à Setlist.
   - **Setlist (Sidebar):**
     - Renomeada de "Biblioteca de Mídias" para "Setlist".
     - Botão `+` no cabeçalho abrindo a Command Palette.
     - Itens da setlist com visual Pro/Holyrics.

## 3. Critérios de Aceite

1. O atalho `⌘K` / `Ctrl+K` e o clique no campo de busca ou no botão `+` abrem a Command Palette.
2. Digitar termos no Command Palette busca no banco SQLite local por título e letra via FTS5.
3. Apertar `Shift + Enter` ou clicar em "Adicionar à Setlist" insere o item imediatamente na lista lateral e sincroniza via WebSocket.
4. É possível clicar em "Buscar na internet" para obter resultados reais do Holyrics e Vagalume e importar a letra diretamente para o editor.
5. O modal de cadastro de músicas preenche 95% da tela e atualiza a grade de slides em 16:9 dinamicamente ao digitar quebras de linha e `## <Rótulos>`.
6. Músicas salvas persistem no banco SQLite local em `open-presenter.db` e podem ser pesquisadas novamente a qualquer momento.
7. Portões de qualidade passam: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `bun run check && bun run build`.

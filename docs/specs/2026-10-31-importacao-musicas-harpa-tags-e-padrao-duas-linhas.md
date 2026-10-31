# Spec — Importação de Músicas (JSON/TXT/MD), Instalação da Harpa Cristã, Tags/Grupos e Padrão de 2 Linhas por Slide

## Intenção

Permitir que igrejas e operadores importem facilmente seus repertórios de louvores a partir de múltiplos formatos (JSON, Text e Markdown), instalem a coletânea completa da Harpa Cristã (540 hinos) com um único clique (usando arquivo local ou download do GitHub), classifiquem e filtrem suas músicas usando metadados de tags/grupos, e garantam que letras importadas ou obtidas da internet sigam o padrão de 2 linhas por slide (configurável).

## Escopo

1. **Importação flexível de músicas:**
   - Suporte a upload / importação de arquivos `.json`, `.txt` e `.md` (individualmente ou em lote).
   - Suporte a múltiplos esquemas de JSON: modelo nativo do OpenPresenter, formato Harpa Cristã / Songbook, e formato genérico de músicas.
   - Suporte a arquivos de texto plano e Markdown com títulos, tags e estrofes.
2. **Instalador nativo da Harpa Cristã (540 hinos):**
   - Disponível no modal de configurações e no painel de músicas.
   - Resolução resiliente: verifica `/Users/claudio/Downloads/harpa-crista.json` -> verifica `assets/songs/harpa-crista.json` (embutido no repositório) -> baixa da URL oficial do GitHub Raw.
   - Importação atômica em transação SQLite com associação automática da tag `Harpa Cristã`.
3. **Tags e Grupos de Músicas:**
   - Tabela relacional SQLite `song_tags (song_id, tag)` via migração `0009_song_tags.sql`.
   - Campos `tags: Vec<String>` em `Song` e `SongSummary`.
   - Adição e remoção de tags no modal de edição de músicas (`SongEditorModal.svelte`).
   - Filtro rápido por tags/grupos no catálogo de músicas (`BottomPanel.svelte`).
4. **Padrão de 2 Linhas por Slide (Configurável):**
   - Configuração `lines_per_slide` em `SongSettings` (padrão: 2 linhas, opções: 1, 2, 3, 4, ou 0 para manter original).
   - Função utilitária no core Rust (`format_lyrics_lines_per_slide`) e no frontend TypeScript (`formatLyricsLinesPerSlide`).
   - Aplicação automática nas letras importadas, nas letras obtidas via web (Holyrics e Vagalume) e botão de ação rápida no editor de músicas.

## Decisões Arquiteturais

1. **Migração 0009:** Criação da tabela `song_tags` com chave primária composta `(song_id, tag)` e índice em `tag`. Isso preserva o histórico de migrações (0001 a 0008) sem alterações destrutivas.
2. **Separação de Camadas:**
   - `op-core`: Adiciona campo `tags: Vec<String>` em `Song` e `SongSummary`, e função pura `format_lyrics_lines_per_slide`. Sem dependência de SQLite ou UI.
   - `op-db`: Migração 0009, persistência e leitura de tags em `save_song`, `get_song`, `search_songs`, método `list_all_song_tags` e inserção em lote `save_songs_batch`.
   - `op-admin`: Rotas `POST /api/songs/import`, `POST /api/songs/harpa/install`, `GET /api/songs/tags`, e ajustes em `fetch_online_lyrics` para respeitar `lines_per_slide`.
   - Frontend (`op-admin` UI): Componentes em `SettingsModal.svelte`, `BottomPanel.svelte` e `SongEditorModal.svelte`.

## Critérios de Aceite

- [x] Migração 0009 aplicada com sucesso e testes de banco atualizados para schema v9.
- [x] Músicas podem conter tags persistidas e consultadas no banco.
- [x] Rota de instalação da Harpa Cristã importa 540 hinos com tag `Harpa Cristã` e estrofes formatadas em 2 linhas.
- [x] Rota de importação aceita arquivos JSON, Text e Markdown individuais ou múltiplos.
- [x] O catálogo de músicas permite filtrar por tags.
- [x] O editor de músicas permite adicionar/editar tags e traz botão para reorganizar em 2 linhas por slide.
- [x] Configuração `lines_per_slide` ajustável nas configurações.
- [x] Todos os testes (`cargo test --workspace`, `npm run check`, `npm run build`) verdes.

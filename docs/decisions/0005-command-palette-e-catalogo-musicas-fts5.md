# ADR 0005 — Command Palette, Catálogo de Músicas com SQLite FTS5 e Busca Online

- **Status:** aceito
- **Data:** 2026-10-28
- **Contexto relacionado:** [ADR 0004](0004-arquitetura-web-axum-svelte.md)

## Contexto

O operador de projeção em igrejas e eventos necessita de agilidade extrema para encontrar louvores, criar músicas de última hora e inseri-las na programação (Setlist) durante o culto. A busca precisa ser instantânea, permitindo localizar músicas não apenas pelo título, mas também por trechos das letras ("letra na ponta da língua"). Caso a música ainda não esteja cadastrada no acervo local, deve ser possível pesquisar em provedores online consagrados (Holyrics e Vagalume) e importar a letra já formatada.

## Decisão

1. **Persistência e Busca com SQLite FTS5:**
   - Adicionar uma nova migração `0003_songs_fts.sql` em `op-db` criando as tabelas `songs`, `song_sections`, `song_lyrics` e a tabela virtual `songs_fts` utilizando o motor FTS5 do SQLite com tokenizador `unicode61 remove_diacritics 2`.
   - Permitir buscas de texto completo de alta velocidade cobrindo título, artista e corpo da letra.
2. **Modelo de Músicas em `op-core`:**
   - Adicionar os tipos canônicos `Song`, `SongSection`, `SectionType` e `SongSummary`.
   - Adicionar parser de letras que reconhece rótulos de seção (`## Verso`, `## Refrão`, `## Ponte`, etc.) e divisão de slides por linhas vazias (`\n\n`), no mesmo padrão do Holyrics e ProPresenter.
3. **Provedores de Busca Online:**
   - Implementar clientes de busca em `op-admin` para Holyrics (com descriptografia do CDN) e Vagalume, acessíveis via endpoint REST `/api/songs/online/search` e `/api/songs/online/fetch`.
4. **Command Palette (`⌘K` / `Ctrl+K`) no Frontend:**
   - Componente modal global acessível pelo teclado (`⌘K` no Mac, `Ctrl+K` no Windows/Linux), pelo campo de busca do cabeçalho ou pelo botão `+` da Setlist.
   - Resultados locais em tempo real com botão "Adicionar à Setlist" e atalho de teclado `Shift + Enter`.
   - Opções diretas para "Criar nova música manualmente" e "Buscar música na internet".
5. **Modal de Cadastro/Edição de Músicas (95% da tela):**
   - Painel esquerdo com campos de metadados (Título, Artista, Tom, BPM) e editor de texto.
   - Painel direito com pré-visualização em tempo real de todos os slides da música gerados com proporção 16:9 e identificação visual de seções (Verso, Refrão, etc.).
6. **Sidebar da Setlist:**
   - Renomear a sidebar de "Biblioteca de Mídias" para "Setlist".
   - Botão `+` para adicionar mídias via Command Palette.

## Consequências

- Busca instantânea e tolerante a acentuação para todo o acervo de letras.
- Integração fluida entre catálogo permanente e setlist temporária da apresentação.
- Importação rápida da internet com um clique.
- Conformidade estrita com o princípio de dependências (`op-admin` → `op-db` → `op-core`) e preservação do histórico de migrações.

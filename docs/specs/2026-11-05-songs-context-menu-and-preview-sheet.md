# Spec — Menus de Contexto na Aba de Músicas e no Sheet de Preview de Letras

## 1. Intenção

Disponibilizar menus de contexto (`ContextMenu`) nativos e responsivos com o clique do botão direito do mouse (`contextmenu`) na aba **Músicas** do painel inferior (`BottomPanel.svelte`), tanto sobre os cards de músicas do catálogo quanto no sheet/gaveta lateral de pré-visualização de letras e seus slides individuais 16:9.

---

## 2. Contexto e Problema

Atualmente, o painel inferior (`BottomPanel.svelte`) já conta com suporte a menus de contexto de alta produtividade nas abas **Fundos** e **Outros** (para papéis de parede, gradientes, avisos, textos e agendas). Entretanto, a aba **Músicas** — o recurso mais utilizado durante cultos e ensaios — não possuía menu de contexto ao clicar com o botão direito:
1. No catálogo de músicas, as ações ficavam restritas aos botões "+ Setlist" e "Ao Vivo". Faltava um meio rápido de editar a música, copiar sua letra integral para a área de transferência ou excluí-la do banco de dados.
2. No sheet/gaveta de visualização detalhada de slides da música (`viewingSongSheet`), o operador não conseguia clicar com o botão direito em um slide específico (ex: Refrão ou Ponte) para dispará-lo imediatamente ao vivo ou copiar seu texto específico sem precisar apresentar toda a música a partir do slide 1.

---

## 3. Escopo e Solução

### 3.1 Backend (`op-admin` e `op-db`)

- Adicionar suporte ao método `DELETE` na rota `/api/songs/{id}` em `crates/op-admin/src/routes.rs`.
- Conectar a rota ao método já existente `op_db::delete_song(&id)`, que remove o registro da tabela `songs` e o índice FTS5 `songs_fts`.
- Registrar a ação no log de auditoria via `log_action("song_deleted", ...)` e notificar os clientes via WebSocket.
- Incluir testes unitários no suite de integração de `op-admin`.

### 3.2 Frontend — AdminView (`AdminView.svelte`)

- Atualizar o manipulador `handlePresentSong` para suportar índice de slide opcional: `handlePresentSong(songId: string, slideIndex = 0)`.
- Adicionar manipulador `handleEditSong(songId: string)` que carrega a música completa via `/api/songs/{id}` e abre o `SongEditorModal` em modo `'song'`.
- Passar os callbacks `onpresentsong` e `onopeneditsong` para `BottomPanel.svelte`.

### 3.3 Frontend — BottomPanel (`BottomPanel.svelte`)

1. **Menu de Contexto no Card da Música (`handleSongContextMenu`):**
   - Cabeçalho com Título, Tom e Artista.
   - **Apresentar Agora (Ao Vivo):** projeta a música a partir do primeiro slide.
   - **Adicionar à Setlist:** insere a música na setlist ativa.
   - **Ver Letra / Slides:** abre ou foca o sheet lateral de visualização.
   - **Editar Música:** abre o modal de edição completo.
   - **Copiar Letra Completa:** copia a letra em texto puro para o clipboard com notificação de sucesso.
   - **Excluir do Catálogo:** solicita confirmação explícita (`confirm(...)`) e dispara `DELETE /api/songs/{id}`, atualizando a listagem e disparando o evento de recarga do catálogo.

2. **Menu de Contexto nos Slides do Sheet de Preview (`handleViewingSongSlideContextMenu`):**
   - Cabeçalho com o identificador e número do slide (ex: `Refrão • Slide 2`).
   - **Apresentar Este Slide Ao Vivo:** projeta a música e seleciona diretamente o slide clicado na tela pública e de palco.
   - **Adicionar Música à Setlist:** adiciona a música inteira à programação ativa.
   - **Copiar Texto Deste Slide:** copia apenas o conteúdo textual do slide para o clipboard.
   - **Editar Música:** abre a música no `SongEditorModal`.

3. **Menu de Contexto no Cabeçalho/Container do Sheet (`handleViewingSongSheetContextMenu`):**
   - Ações agregadas da música em foco: Apresentar do início, Adicionar à setlist, Editar, Copiar letra completa, Excluir do catálogo e Fechar sheet de visualização.

4. **Menu de Contexto no Vazio do Catálogo (`handleSongsEmptyContextMenu`):**
   - Nova Música, Importar Músicas e Recarregar Catálogo.

---

## 4. Critérios de Aceite

1. Clicar com o botão direito em qualquer card de música da lista abre o menu contextual completo com as opções de apresentar, adicionar, visualizar letra, editar, copiar letra e excluir.
2. Clicar com o botão direito em um slide do sheet de preview abre o menu de contexto permitindo apresentar aquele slide específico ao vivo ou copiar seu texto.
3. Clicar com o botão direito no cabeçalho ou área geral do sheet de preview abre o menu de contexto da música em exibição.
4. Ao excluir uma música pelo menu de contexto, um alerta de confirmação é exibido antes de efetivar o `DELETE` no backend.
5. A rota `DELETE /api/songs/{id}` remove a música do SQLite e do FTS5 e retorna status `204 No Content`.
6. Todos os portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test` e `bun run build`) passam com 100% de sucesso.

# Plano de Implementação: Temas Shadcn, Pastas de Mídia, Editor Markdown e Painéis Flexíveis

- **Status:** Em Progresso
- **Data:** 2026-10-29
- **Spec Relacionada:** [`docs/specs/2026-10-29-temas-pastas-midia-editor-markdown-e-paineis.md`](../specs/2026-10-29-temas-pastas-midia-editor-markdown-e-paineis.md)

---

## Fases de Execução

### Fase 1: Domínio e Persistência (`op-core` e `op-db`)
1. **Modelos em `op-core`:**
   - Adicionar `MediaKind::Image` e `MediaKind::Text` se necessário.
   - Adicionar campos opcionais de alinhamento (`align_h`, `align_v`) em `Slide` e `MediaItem`.
   - Adicionar modelo `MediaFolder` (`id`, `name`, `path`, `kind`, `created_at`).
2. **Migração do Banco de Dados (`op-db`):**
   - Criar `migrations/0005_media_folders.sql` com tabela `media_folders`.
   - Atualizar `Migrations::new` em `op-db/src/lib.rs`.
   - Implementar métodos `list_media_folders`, `save_media_folder`, `delete_media_folder`.
   - Atualizar testes do banco de dados (schema v5).

### Fase 2: Backend e Endpoints REST (`op-admin`)
1. **Gestão de Pastas do Sistema e Mídias:**
   - Adicionar subpastas `images`, `videos`, `fonts` em `resolve_media_dir`.
   - Implementar rota `GET /api/system/paths`.
   - Implementar rotas `GET /api/media/folders`, `POST /api/media/folders`, `DELETE /api/media/folders/{id}`.
   - Implementar rota `GET /api/media/images` (varrendo pasta interna e pastas cadastradas).
   - Implementar rota `POST /api/media/images/upload` e `GET /api/media/external-file`.
2. **Atualização de testes do backend.**

### Fase 3: Temas Shadcn & Sidebars Colapsáveis (`frontend`)
1. **Configuração de Cores em `settings.ts`:**
   - Base Color: `Neutral`, `Stone`, `Zinc`, `Mauve`, `Olive`, `Mist`, `Taupe`.
   - Theme Accent: `Neutral`, `Amber`, `Blue`, `Cyan`, `Emerald`, `Fuchsia`, `Green`, `Indigo`, `Lime`, `Orange`, `Pink`, `Purple`, `Rose`, `Violet`, `Yellow`.
   - Sincronização de CSS variables de background e primary no `:root`.
2. **Sidebars Colapsáveis:**
   - Controles de fechar/abrir para Setlist (esquerda) e Previews (direita) em `AdminView.svelte`.

### Fase 4: Drag and Drop na Setlist & Editor Unificado com Typeset Markdown
1. **Drag and Drop na Setlist:**
   - Eventos de arrastar e soltar em `MediaItemRow.svelte` e `AdminView.svelte` com reordenação persistida via WebSocket.
2. **Editor de Apresentações Rápidas e Músicas com Typeset:**
   - Expandir `SongEditorModal.svelte` para suportar modo Texto e modo Música.
   - Adicionar toolbar typeset (Negrito, Itálico, Sublinhado, Highlight).
   - Adicionar seletor de alinhamento horizontal (Esquerda, Centro, Direita, Justificado) e vertical (Topo, Centro, Base).
   - Renderização rica de markdown na prévia, `PublicView` e `StageView`.

### Fase 5: Bottom Panel & Aba de Arquivos e Pastas
1. **Bottom Panel (`BottomPanel.svelte`):**
   - Painel inferior colapsável, redimensionável por arrasto e maximizável.
   - Aba Músicas e Aba Imagens (com visualização e disparo ao vivo).
2. **Aba "Arquivos e Pastas" em `SettingsModal.svelte`:**
   - Visualização dos caminhos do sistema.
   - Gerenciamento de pastas locais importadas com leitura direta sem duplicação.

### Fase 6: Ajustes Refinados de UX, Pastas, Temas e Mídias
1. **File Picker Nativo e Caminhos do Sistema:**
   - Adicionar endpoint `POST /api/system/pick-folder` com diálogo nativo do sistema operacional e `POST /api/system/open-path` para abrir a pasta no Finder.
   - Exibir com destaque o diretório raiz de dados (`data_dir`) com botão de copiar caminho e botão de abrir pasta.
   - Aumentar a largura do `SettingsModal.svelte` (`max-w-4xl` / `max-w-5xl`) e prevenir truncamento de caminhos e textos.
2. **Correção e Aplicação Global de Temas (Shadcn UI):**
   - Mapear variáveis CSS no `@theme` do `app.css` para Tailwind v4 (`--color-primary`, `--color-background`, etc.).
   - Vincular `Button.svelte`, superfícies e anéis às variáveis temáticas.
   - Garantir chamada a `applyBaseColor` e `applyAccentTheme` na montagem do app.
3. **Mídias e Imagens na Setlist:**
   - Botão visível de apresentação rápida em cada miniatura de imagem no `BottomPanel.svelte`.
   - Adição de opções "Adicionar Imagem..." e "Adicionar Pasta de Imagens..." no menu da Setlist e modal de seleção `ImageSelectorModal.svelte`.
   - Sheet / Drawer lateral na aba de Músicas do painel de mídia para visualização instantânea da letra completa com seções.
4. **Atalhos Dedicados Livres de Conflito:**
   - Suporte a `.` (Blackout), `,` (Limpar Texto), `/` ou `Alt+W` (Papel de Parede) e teclas `F1`/`F2`/`F3`, sem conflito com atalhos de seções.

### Fase 7: Portões de Qualidade e Lançamento
1. `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
2. `bun run check` e `bun run build`.
3. Atualização de `CHANGELOG.md` e commits atômicos.

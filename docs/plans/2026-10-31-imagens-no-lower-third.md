# Plano — Exibição e Seleção de Imagens no Lower Third

**Data:** 2026-10-31  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-10-31-imagens-no-lower-third.md`](../specs/2026-10-31-imagens-no-lower-third.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Backend Rust (`op-core`, `op-db`, `op-admin`)
- [x] Adicionar `lower_image_url: Option<String>` com `#[serde(default)]` em `MediaItem` ([`crates/op-core/src/lib.rs`](../../crates/op-core/src/lib.rs)).
- [x] Adicionar `lower_content: Option<String>` com `#[serde(default)]` em `Slide` ([`crates/op-core/src/lib.rs`](../../crates/op-core/src/lib.rs)).
- [x] Atualizar construtores e testes unitários.
- [x] Executar `cargo test -p op-core`.

### Etapa 2: Definições de Tipos e Stores no Frontend
- [x] Em [`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts):
  - Adicionar `lower_image_url?: string` em `MediaItem`.
  - Adicionar `lower_content?: string` em `Slide`.
  - Adicionar `image_display_mode: 'fullscreen' | 'lower_third' | 'hide'` e `image_fit: 'contain' | 'cover'` em `LowerThirdSettings`.
- [x] Em [`frontend/src/lib/displaySettings.ts`](../../frontend/src/lib/displaySettings.ts):
  - Atualizar `DEFAULT_LOWER_THIRD_SETTINGS` com `image_display_mode: 'fullscreen'` e `image_fit: 'contain'`.

### Etapa 3: Renderização de Imagens no `LowerThirdView.svelte`
- [x] Em [`frontend/src/lib/views/LowerThirdView.svelte`](../../frontend/src/lib/views/LowerThirdView.svelte):
  - Detectar se `activeMedia?.kind === 'image'` ou se `currentSlide?.content` é imagem.
  - Resolver a URL da imagem priorizando `activeMedia.lower_image_url || currentSlide.lower_content || currentSlide.content` com `resolveMediaUrl(...)`.
  - Se `lt.image_display_mode === 'hide'`: não renderizar nada.
  - Se `lt.image_display_mode === 'fullscreen'`: renderizar em tela cheia na raiz com animação de fade.
  - Se `lt.image_display_mode === 'lower_third'`: renderizar dentro do card inferior com altura delimitada (banner de transmissão).

### Etapa 4: Configurações de Imagem em `SettingsModal.svelte`
- [x] Adicionar controles na aba "Lower Third" de [`SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - Seletor de Modo de Imagem: `Tela Cheia (Fullscreen)`, `Banner no Terço Inferior` e `Ocultar na Transmissão`.
  - Seletor de Ajuste: `Conter (Proporcional)` vs `Preencher (Cover)`.
  - Botão de simulação `🖼️ Imagem` na prévia 16:9.

### Etapa 5: Vínculo de Imagem Alternativa para Lower Third no `AdminView.svelte`
- [x] No menu de contexto de itens de mídia da setlist ([`AdminView.svelte`](../../frontend/src/lib/views/AdminView.svelte)):
  - Se `item.kind === 'image'`:
    - Adicionar ação "Selecionar Imagem para Lower Third...".
    - Se já tiver `lower_image_url`, adicionar ação "Remover Imagem de Lower Third".
- [x] No componente [`MediaItemRow.svelte`](../../frontend/src/lib/components/ui/MediaItemRow.svelte):
  - Exibir badge sutil `Lower 🖼️` se houver imagem alternativa vinculada.

### Etapa 6: Validação e Portões de Qualidade
- [x] `cd frontend && bun run build`
- [x] `cargo fmt --all`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.

# Plano de Implementação: Separação entre Papel de Parede da Setlist e Fundo de Projeção de Letras

## Etapas

1. **Campos do Modelo em `crates/op-core/src/lib.rs`:**
   - Adicionar `setlist_wallpaper_path`, `setlist_wallpaper_mode`, `lyrics_background_path`, `lyrics_background_mode` em `PresentationState`.
   - Garantir valores padrão em `PresentationState::default()` e compatibilidade serde com `#[serde(default)]`.

2. **Gerenciamento de Estado no Backend (`crates/op-admin/src/state.rs` e `routes.rs`):**
   - Na inicialização de `AppState::new`, carregar `default_lyrics_background` do banco ou usar gradiente esmeralda escuro padrão.
   - Em `select_slide`: alternar `wallpaper_path` para o `lyrics_background_path`.
   - Em `stop_presentation` e remoção da mídia ativa: alternar `wallpaper_path` de volta para `setlist_wallpaper_path`.
   - Em `set_wallpaper`: permitir definir o fundo de letras ou o wallpaper da setlist conforme o contexto.
   - Em `switch_active_setlist`: atualizar `setlist_wallpaper_path` e, se ocioso, `wallpaper_path`.

3. **Ajustes no Frontend (`BottomPanel.svelte`, `AdminView.svelte`, `PublicView.svelte` e `PublicPreviewBox.svelte`):**
   - Na aba "Fundos & Gradientes", rotular claramente os fundos de letras.
   - Garantir que a troca de estado reflita instantaneamente nas telas públicas e de pré-visualização.

4. **Testes Unitários e Validação:**
   - Adicionar teste em `crates/op-admin/src/main.rs` verificando o chaveamento automático de fundos entre repouso (setlist) e ao vivo (letras).
   - Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `bun run build`.

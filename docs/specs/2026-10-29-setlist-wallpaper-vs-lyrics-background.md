# Especificação: Separação entre Papel de Parede da Setlist e Fundo de Projeção de Letras

## Intenção

Permitir que o OpenPresenter diferencie de forma automática o **Papel de Parede da Setlist** (tema do culto / repouso quando nenhuma apresentação está ativa) do **Background de Apresentação** (fundo visual exibido durante a projeção de músicas e passagens bíblicas). Ao iniciar uma apresentação, a tela pública deve projetar o fundo de apresentação; ao cancelar ou parar a apresentação, a tela deve retornar automaticamente ao papel de parede da setlist ativa.

## Escopo

### Incluído
1. **Campos dedicados em `PresentationState`:**
   - `setlist_wallpaper_path` e `setlist_wallpaper_mode`: plano de fundo da setlist ativa (tela de repouso / tema do culto).
   - `lyrics_background_path` e `lyrics_background_mode`: plano de fundo padrão para letras de músicas e textos bíblicos ao vivo.
   - Preservação do campo existente `wallpaper_path`: reflete dinamicamente o fundo ativo no momento (se `active_media_id.is_some()`, reflete o fundo de letras; se `active_media_id.is_none()`, reflete o wallpaper do setlist).
2. **Ciclo de Vida de Apresentação no Backend (`crates/op-admin/src/state.rs`):**
   - Ao iniciar apresentação (`select_slide`): `wallpaper_path` assume o `lyrics_background_path`.
   - Ao parar apresentação (`stop_presentation` / `clear_active_setlist_items`): `wallpaper_path` retorna automaticamente para `setlist_wallpaper_path`.
   - Ao trocar setlist (`switch_active_setlist`): sincroniza `setlist_wallpaper_path`. Se não houver slide ao vivo, atualiza `wallpaper_path`.
   - Ao selecionar fundo na aba de Fundos: atualiza `lyrics_background_path` e persiste em `app_settings` (`default_lyrics_background`). Se houver slide ativo, atualiza em tempo real.
3. **Consistência no Frontend (`PublicView`, `PublicPreviewBox` e `AdminView`):**
   - A tela pública e o preview do painel admin respeitam a transição suave entre o wallpaper de repouso da setlist e o fundo de letras.

## Critérios de Aceite

1. Quando não há slide projetado, a tela pública exibe o wallpaper configurado na setlist ativa.
2. Ao disparar uma música ou passagem bíblica, o fundo muda para o fundo de apresentação/letras.
3. Ao acionar "Parar Apresentação", o fundo retorna imediatamente ao wallpaper da setlist.
4. Ao alterar o fundo na aba "Fundos & Gradientes", o novo fundo é salvo como padrão para as letras sem sobrescrever o wallpaper do tema do culto da setlist.
5. Todos os portões de qualidade (`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e build do frontend) passam com sucesso.

# Plano — Ocultação de Timer ao Terminar com Papel de Parede Limpo, Parada na Apresentação e Reset ao Reiniciar

**Data:** 2026-11-06  
**Spec correspondente:** [`docs/specs/2026-11-06-timer-public-wallpaper-ao-terminar-e-reset-apresentacao.md`](../specs/2026-11-06-timer-public-wallpaper-ao-terminar-e-reset-apresentacao.md)

---

## Etapas de Implementação

### 1. Backend (`crates/op-admin/src/state.rs`)
- [ ] Em `stop_presentation()`:
  - Garantir `state.timer.running = false;` e `state.timer.remaining_seconds = state.timer.duration_seconds;`.
- [ ] Em `select_slide()`:
  - Para `MediaKind::Timer`: garantir `state.timer.allow_overtime = false;`, `state.timer.duration_seconds = dur;`, `state.timer.remaining_seconds = dur;`, `state.timer.running = true;`.
- [ ] Em `quick_present_media_with_slide()`:
  - Para `MediaKind::Timer`: garantir `state.timer.allow_overtime = false;`, `state.timer.duration_seconds = dur;`, `state.timer.remaining_seconds = dur;`, `state.timer.running = true;`.
- [ ] Em `tick_timer()`:
  - Ao atingir `<= 1s` sem `allow_overtime`: cravar `state.timer.remaining_seconds = 0; state.timer.running = false;`.

### 2. Frontend de Projeção Pública (`PublicView.svelte` & `PublicPreviewBox.svelte`)
- [ ] Em `PublicView.svelte`:
  - Condicionar o card de cronômetro com `{#if remainingSecs > 0}`.
  - No bloco `{:else}`, renderizar container transparente vazio (`<div class="w-full flex-1 h-full min-h-0 min-w-0"></div>`), preservando o papel de parede ativo 100% visível.
- [ ] Em `PublicPreviewBox.svelte`:
  - Aplicar o mesmo padrão condicional `{#if remainingSecs > 0}` para ocultar o card de preview do cronômetro quando zerado.

### 3. Frontend do Operador e Monitor de Palco (`AdminView.svelte` & `StageView.svelte`)
- [ ] Em `AdminView.svelte`:
  - Em `presentSlide()`: chamar `setTimer(dur, selectedMedia.title, false)` (sem overtime) e `startTimer()`.
  - Em `handlePresentCustomItem()`: chamar `setTimer(dur, item.title, false)` e `startTimer()`.
  - Em `stopPresentation()`: chamar `wsStopPresentation()`, `pauseTimer()` e `resetTimer()`.
- [ ] Em `StageView.svelte`:
  - No snippet `currentSlideBlock`: renderizar exibição formatada para `activeMedia?.kind === 'timer'` com dígitos destacados, mensagem e badge de status "Tempo Concluído" quando zerado.

### 4. Validação e Qualidade
- [ ] Executar `cargo fmt --all`.
- [ ] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Executar `cargo test --workspace`.
- [ ] Executar `npm --prefix frontend run build`.
- [ ] Atualizar `CHANGELOG.md`, `TODO.md` e `docs/README.md`.
- [ ] Realizar commit seguindo o GitFlow.

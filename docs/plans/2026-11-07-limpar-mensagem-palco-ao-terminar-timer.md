# Plano — Limpeza Automática da Mensagem de Retorno ao Finalizar Timer no Public e Palco

**Data:** 2026-11-07  
**Spec correspondente:** [`docs/specs/2026-11-07-limpar-mensagem-palco-ao-terminar-timer.md`](../specs/2026-11-07-limpar-mensagem-palco-ao-terminar-timer.md)

---

## Etapas de Implementação

### 1. Backend (`crates/op-admin/src/state.rs`)
- [ ] Em `tick_timer()`:
  - Ao atingir `<= 1s` sem `allow_overtime`: cravar `state.timer.remaining_seconds = 0; state.timer.running = false; state.timer.stage_message = None;`.
- [ ] Em `stop_presentation()`:
  - Garantir `state.timer.stage_message = None;`.

### 2. Frontend do Monitor de Retorno/Palco (`StageView.svelte`)
- [ ] Em `StageView.svelte`:
  - No bloco condicional `activeMedia?.kind === 'timer'`: renderizar a mensagem de rodapé apenas se `remainingSecs > 0 && (timer?.stage_message || activeSlide.lower_content)`.

### 3. Testes Unitários e de Integração (`crates/op-admin/src/main.rs`)
- [ ] Em `test_timer_presentation_reset_and_stop_behavior`:
  - Configurar slide de timer com `lower_content: Some("Mensagem de Palco".into())`.
  - Validar que ao selecionar o slide, `pres.timer.stage_message` é populado.
  - Validar que ao atingir o final da contagem (`tick_timer`), `pres.timer.stage_message` é limpo (`None`).
  - Validar que ao chamar `stop_presentation`, `pres.timer.stage_message` é limpo (`None`).

### 4. Validação e Portões de Qualidade
- [ ] Executar `cargo fmt --all`.
- [ ] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Executar `cargo test --workspace`.
- [ ] Executar `npm --prefix frontend run build`.
- [ ] Atualizar `CHANGELOG.md`, `TODO.md` e `docs/README.md`.
- [ ] Efetuar commit no padrão GitFlow (`fix(timer): limpar mensagem de retorno ao finalizar contagem regressiva`).

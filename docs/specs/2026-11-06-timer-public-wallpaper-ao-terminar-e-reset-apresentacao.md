# Spec — Ocultação de Timer ao Terminar com Papel de Parede Limpo, Parada na Apresentação e Reset ao Reiniciar

**Data:** 2026-11-06  
**Status:** Aprovada  
**Contexto:** Ao projetar um item de Cronômetro/Timer na Tela Pública (`/public`), o usuário identificou que quando o timer atinge zero (ou termina), a tela pública deve ocultar a sobreposição do cronômetro e exibir apenas o papel de parede ativo. Além disso, ao cancelar ou parar a apresentação (via botão "Parar", tecla Esc ou deseleção), o timer deve sumir da tela pública, ter sua contagem pausada e restaurada. Ao reiniciar a apresentação do timer, a contagem regressiva deve recomeçar do início (resetada) de forma sincronizada em todas as telas (Pública, Palco e Previews).

---

## 1. Intenção e Objetivos

1. **Papel de Parede Limpo ao Término do Timer na Tela Pública:** Quando o cronômetro do item de apresentação terminar (`remaining_seconds <= 0`), remover imediatamente a caixa/overlay de contagem na tela pública (`PublicView.svelte`) e no preview do operador (`PublicPreviewBox.svelte`), deixando visível unicamente o papel de parede ativo do setlist de forma limpa e profissional.
2. **Cancelamento/Parada de Apresentação:** Ao acionar "Parar apresentação" (`stop_presentation`, botão Parar ou tecla Esc), desativar o item ativo na tela pública, voltar ao papel de parede do setlist e pausar/resetar o timer no backend (`running = false`, `remaining_seconds = duration_seconds`).
3. **Reinício com Reset Automático:** Ao apresentar ou selecionar novamente o item de Timer, resetar e reiniciar atomicamente a contagem a partir da duração nominal configurada (`dur`), sem herdar tempo expirado ou estado de overtime anterior, sincronizando instantaneamente a tela pública, o monitor de retorno (Stage) e o painel do operador.

---

## 2. Escopo

- **Backend (`crates/op-admin/src/state.rs`):**
  - Em `stop_presentation()`: parar o timer (`state.timer.running = false;`) e restaurar o tempo restante para a duração configurada (`state.timer.remaining_seconds = state.timer.duration_seconds;`).
  - Em `select_slide()`: ao apresentar item de timer (`MediaKind::Timer`), assegurar que `state.timer.allow_overtime = false;`, `state.timer.duration_seconds = dur;`, `state.timer.remaining_seconds = dur;`, `state.timer.running = true;`.
  - Em `quick_present_media_with_slide()`: assegurar que `state.timer.allow_overtime = false;`, `state.timer.duration_seconds = dur;`, `state.timer.remaining_seconds = dur;`, `state.timer.running = true;`.
  - Em `tick_timer()`: ao atingir `<= 1s` sem `allow_overtime`, cravar `remaining_seconds = 0; running = false;` e enviar broadcast de atualização.
- **Frontend (`PublicView.svelte` & `PublicPreviewBox.svelte`):**
  - No bloco condicional `activeMedia?.kind === 'timer'`: renderizar a caixa de cronômetro apenas quando `remainingSecs > 0`. Quando `remainingSecs <= 0`, renderizar container vazio transparente, revelando 100% o papel de parede de fundo.
- **Frontend (`StageView.svelte`):**
  - No bloco `currentSlideBlock`: renderizar card visual formatado para `activeMedia?.kind === 'timer'`, com contagem regressiva, mensagem/subtítulo e aviso de "Tempo Concluído" quando `remainingSecs <= 0`.
- **Frontend (`AdminView.svelte`):**
  - Em `presentSlide()` e `handlePresentCustomItem()`: disparar `setTimer(dur, title, false)` (com `allow_overtime = false`) e `startTimer()`.
  - Em `stopPresentation()`: chamar `wsStopPresentation()`, `pauseTimer()` e `resetTimer()`.

---

## 3. Critérios de Aceite

1. **Timer Concluído no Public:**
   - Dado um item de Timer ativo na tela pública, quando a contagem regressiva chegar a zero (`00:00`), a caixa de texto e dígitos do timer deve sumir suavemente da tela pública (`/public`) e do preview (`PublicPreviewBox.svelte`), mantendo o papel de parede ativo visível.
2. **Cancelamento da Apresentação:**
   - Ao clicar no botão "Parar" ou pressionar a tecla `Esc`, a tela pública deixa de exibir o timer imediatamente, o papel de parede do setlist permanece ativo e a contagem do timer é pausada e resetada no backend.
3. **Reinício do Timer:**
   - Ao clicar em "Ao Vivo" ou selecionar o slide do timer novamente (inclusive após ele ter chegado a 0 ou ter sido parado), o timer reinicia do início da duração configurada (ex: 03:00) e a contagem recomeça sincronizada no Public e no Stage.
4. **Qualidade e Estabilidade:**
   - `cargo fmt --all` passa sem erros.
   - `cargo clippy --workspace --all-targets -- -D warnings` passa sem alertas.
   - `cargo test --workspace` passa em todos os testes.
   - `npm --prefix frontend run build` constrói com sucesso sem erros.

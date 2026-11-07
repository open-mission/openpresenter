# Spec — Limpeza Automática da Mensagem de Retorno ao Finalizar Timer no Public e Palco

**Data:** 2026-11-07  
**Status:** Aprovada  
**Contexto:** Ao projetar um item de Cronômetro/Timer na Tela Pública (`/public`), o slide ou item pode conter uma mensagem de retorno / aviso para o palco (`stage_message`, como "O culto começará em instantes. Prepare o seu coração!"). Ao terminar a contagem regressiva (`00:00`), a tela pública oculta a caixa do cronômetro, mas a mensagem de retorno permanecia ativa indefinidamente no monitor de retorno/palco (`StageView`), no painel do operador (`TimerControlPanel`) e nos previews. O objetivo desta mudança é limpar automaticamente a mensagem de retorno (`stage_message = None`) quando a contagem regressiva for concluída ou cancelada, garantindo telas limpas e sincronizadas.

---

## 1. Intenção e Objetivos

1. **Limpeza Automática ao Concluir Contagem:** Quando o timer finalizar sua contagem regressiva (`remaining_seconds <= 1 && !allow_overtime`), além de parar a execução e fixar `remaining_seconds = 0`, limpar automaticamente a mensagem de retorno (`state.timer.stage_message = None`).
2. **Limpeza ao Cancelar/Parar Apresentação:** Ao parar a apresentação ativa (`stop_presentation`, botão Parar ou tecla Esc), além de resetar o timer e restaurar o papel de parede, limpar a mensagem de retorno (`state.timer.stage_message = None`).
3. **Consistência Visual no Monitor de Palco (`StageView.svelte`):** Ao concluir a contagem (`remainingSecs <= 0`), não exibir textos ou mensagens residuais herdadas de `activeSlide.lower_content`, exibindo apenas o indicador "Tempo Concluído" e mantendo a caixa "Aviso para o Palco" limpa.
4. **Atualização em Tempo Real via WebSocket:** Transmitir o novo estado com `timer.stage_message = null` para todos os clientes conectados (Pública, Palco, Remoto e Console do Operador).

---

## 2. Escopo

- **Backend (`crates/op-admin/src/state.rs`):**
  - Em `tick_timer()`: quando `remaining_seconds <= 1 && !state.timer.allow_overtime`, definir `state.timer.stage_message = None;`.
  - Em `stop_presentation()`: definir `state.timer.stage_message = None;`.
- **Frontend (`frontend/src/lib/views/StageView.svelte`):**
  - No bloco de renderização do timer central (`activeMedia?.kind === 'timer'`), condicionar a exibição da mensagem de subtítulo a `remainingSecs > 0 && (timer?.stage_message || activeSlide.lower_content)`.
- **Testes Backend (`crates/op-admin/src/main.rs`):**
  - Atualizar o teste de ciclo de vida do timer para validar que `stage_message` é limpo quando a contagem atinge zero e quando a apresentação é parada.

---

## 3. Critérios de Aceite

1. **Timer Concluído no Public e Palco:**
   - Dado um timer com mensagem de retorno em execução, ao atingir `00:00` (sem overtime), `timer.stage_message` deve se tornar `None`/`null`.
   - O painel do operador (`TimerControlPanel.svelte`) deixa de exibir o badge "Ativo" e a mensagem de retorno.
   - O monitor de retorno (`StageView.svelte`) limpa a caixa "Aviso para o Palco" (exibindo "Nenhum aviso ativo") e oculta o subtítulo no card central.
   - O preview do palco (`StagePreviewBox.svelte`) remove a mensagem ativa de alerta.
2. **Parada da Apresentação:**
   - Ao disparar `stop_presentation()`, qualquer mensagem de retorno residual vinculada ao timer é limpa.
3. **Qualidade e Estabilidade:**
   - `cargo fmt --all` executa sem divergências.
   - `cargo clippy --workspace --all-targets -- -D warnings` passa sem nenhum alerta.
   - `cargo test --workspace` passa com sucesso.
   - `npm --prefix frontend run build` compila sem erros.

# Spec: Correção de Tela Preta no Monitor de Palco (/stage)

## Intenção
Corrigir o erro em tempo de execução que deixava a tela do monitor de palco (`/stage`) completamente preta ao ser acessada diretamente pelo navegador (ex: `http://localhost:18080/stage`), restaurando a renderização segura tanto no modo de descanso (Standby com relógio e cronômetro) quanto nos presets de apresentação.

## Contexto e Causa Raiz
No commit `9a8dc49`, foi introduzido o modo de descanso no `StageView.svelte`. No bloco de marcação HTML do modo Standby, foram utilizadas as referências `isOvertime`, `isWarning` e `timerColorClass`, além da chamada `formatTime(timer.remaining_seconds)` com apenas um argumento. No entanto, no `<script>`, as variáveis reativas haviam sido declaradas como `timerIsOvertime` e `timerIsNearEnd`, fazendo com que o runtime do JavaScript lançasse `ReferenceError: isOvertime is not defined`, abortando a renderização do Svelte e resultando em tela preta.

## Escopo
- `frontend/src/lib/views/StageView.svelte`:
  - Declarar a variável reativa `timerColorClass` correspondente ao status do cronômetro.
  - Ajustar a assinatura de `formatTime(remaining: number, isOvertime = false)` para aceitar parâmetro opcional seguro com default `false`.
  - Atualizar o template do modo Standby para utilizar `timerIsOvertime` e `timerIsNearEnd`.
  - Garantir fallback seguro para `timer` vazio ou nulo.

## Critérios de Aceite
1. Acessar `/stage` sem slides ativos exibe imediatamente o Modo de Descanso com o relógio gigante, a data e o cronômetro centralizado, sem tela preta.
2. Acessar `/stage` com slides ativos exibe o slide atual e o próximo slide normalmente de acordo com o preset selecionado.
3. Não ocorre nenhum erro de JavaScript no console do navegador (`ReferenceError`).
4. Os testes automatizados passam com sucesso.

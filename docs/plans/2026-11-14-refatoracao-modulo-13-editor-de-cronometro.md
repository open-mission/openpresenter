# Plano de Execução — Refatoração do Módulo 13 (Editor de Cronômetros & Timers)

## Fases de Execução

### Fase 1: Fundação de Tipos, Defaults e Controlador
- Criar `frontend/src/lib/components/timer-editor/types.ts`:
  - Tipos e interfaces: `TimerPreset`, `TimerFormData`, `TimerEditorModalProps`, callbacks (`onclose`, `onsave`, `onaddtosetlist`).
- Criar `frontend/src/lib/components/timer-editor/defaults.ts`:
  - Lista `TIMER_PRESETS` (1, 2, 3, 5, 10, 15, 30 min), título padrão e mensagem de apoio padrão.
- Criar `frontend/src/lib/components/timer-editor/controller.svelte.ts`:
  - Runes Svelte 5 (`$state`, `$derived`).
  - Estados: `open`, `isMaximized`, `title`, `minutes`, `seconds`, `subtitle`, `allowOvertime`, `dialogEl`, `triggerEl`.
  - Derivados: `totalDurationSeconds`, `formattedTime`, `isPresetSelected`.
  - Métodos: `syncOpen`, `applyPreset`, `adjustMinutes`, `adjustSeconds`, `buildMediaItem`, `saveOnly`, `saveAndAddToSetlist`, `handleKeydown`, `close`, `restoreFocus`.

### Fase 2: Componentes Auxiliares e Submódulos
- Criar `frontend/src/lib/components/timer-editor/TimerEditorHeader.svelte`:
  - Ícone de ampulheta (`Hourglass`), título dinâmico, badge contextual com piso tipográfico `>= 11px`, botões Maximizar e Fechar com acessibilidade e foco visível.
- Criar `frontend/src/lib/components/timer-editor/TimerDurationSelector.svelte`:
  - Bloco visual de duração: display digital `MM:SS` em destaque, presets rápidos com semântica de rádio acessível (`role="radiogroup"` / `role="radio"`, `aria-checked`), campos numéricos para minutos e segundos com botões de incremento/decremento táteis confortáveis.
- Criar `frontend/src/lib/components/timer-editor/TimerSubtitleField.svelte`:
  - Campo de texto para mensagem de apoio / subtítulo no telão, com rótulo descritivo e texto auxiliar de orientação.
- Criar `frontend/src/lib/components/timer-editor/TimerAdvancedOptions.svelte`:
  - Bloco de opções avançadas (Overtime) com switch acessível (`role="switch"`, `aria-checked`).
- Criar `frontend/src/lib/components/timer-editor/TimerEditorFooter.svelte`:
  - Rodapé com botão Cancelar, Salvar na Biblioteca e Salvar e Adicionar à Setlist (com destaque em cor primária/âmbar e dica do atalho `⌘Enter`).

### Fase 3: Refatoração do Orquestrador `TimerEditorModal.svelte`
- Substituir o conteúdo monolítico pelo novo orquestrador conectando o controlador e os submódulos de `timer-editor/`.
- Garantir `role="dialog"`, `aria-modal="true"`, `aria-labelledby="timer-editor-title"`, `aria-describedby="timer-editor-desc"`, `tabindex="-1"`, trap de foco com `focusTrap.ts` e delegação da tecla `Escape` e atalhos `⌘Enter` / `Ctrl+Enter`.
- Reduzir o orquestrador para menos de 110 linhas.

### Fase 4: Verificação e Portões de Qualidade
- `npm run check` (0 erros e 0 warnings).
- `npm run build` (sucesso limpo).
- `npx impeccable detect frontend/src --json` (0 apontamentos).
- `cargo test -p op-db -p op-admin` (78 testes passando).
- Auditoria e pós-fix do Impeccable Critique (`.impeccable/critique/`).
- Atualização da documentação (`CHANGELOG.md`, `docs/README.md`).

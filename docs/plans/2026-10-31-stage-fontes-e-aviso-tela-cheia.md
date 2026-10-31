# Plano: Fontes Maiores no Monitor de Palco e Alerta em Tela Cheia Temporária

Plano associado à especificação [`docs/specs/2026-10-31-stage-fontes-e-aviso-tela-cheia.md`](../specs/2026-10-31-stage-fontes-e-aviso-tela-cheia.md).

## Etapas de Execução

### Etapa 1: Definição dos Tipos e Valores Padrão (`types.ts` & `displaySettings.ts`)
1. Atualizar `StageDisplaySettings` em `frontend/src/lib/types.ts` com `clock_font_size`, `timer_font_size`, `alert_font_size`, `alert_fullscreen_flash` e `alert_flash_duration_seconds`.
2. Atualizar `DEFAULT_STAGE_SETTINGS` em `frontend/src/lib/displaySettings.ts` com os valores padrão ideais para visualização à distância.

### Etapa 2: Implementação no Monitor de Palco (`StageView.svelte`)
1. Aplicar os tamanhos de fonte definidos em `clockBox`, `timerBox` e `messageBox`.
2. Implementar detecção reativa de nova mensagem com temporizador de overlay em tela cheia e transição suave para o rodapé.
3. Permitir dispensa rápida por clique no overlay de tela cheia.

### Etapa 3: Interface de Configuração (`SettingsModal.svelte`)
1. Adicionar seção de tamanhos de fonte na aba "Palco" com controles deslizantes para Relógio, Timer e Aviso.
2. Adicionar controle de ativação de tela cheia e controle deslizante para duração do alerta.

### Etapa 4: Atualização no Preview (`StagePreviewBox.svelte`)
1. Adaptar `StagePreviewBox.svelte` para espelhar proporcionalmente os tamanhos de fonte configurados.

### Etapa 5: Portões de Qualidade e Documentação
1. Compilar frontend com `bun run build`.
2. Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.
3. Atualizar `docs/README.md`, `CHANGELOG.md` e `TODO.md`.

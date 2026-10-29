# Plano de Implementação: Correção do Modal de Configurações, Proporções no Monitor de Palco e Widget de Timer no Header

**Data:** 29 de outubro de 2026  
**Status:** Em execução  
**Spec de Referência:** [`docs/specs/2026-10-29-fix-settings-stage-proportions-e-header-timer.md`](../specs/2026-10-29-fix-settings-stage-proportions-e-header-timer.md)

---

## Fases de Execução

### Fase 1: Correção do Ciclo de Vida e Botão Salvar em `SettingsModal.svelte`
1. Modificar o `$effect` em `SettingsModal.svelte` para rastrear apenas `open` (com sentinela `wasOpen`) e usar `get(...)` para capturar os snapshots das stores sem criar dependências reativas.
2. Adicionar o botão "Salvar Alterações" no rodapé do modal com confirmação visual temporária ("Alterações salvas com sucesso!").
3. Garantir que as alterações locais permaneçam estáveis sem perda de foco ou reset de aba.

### Fase 2: Proporções dos Blocos no Monitor de Palco (Stage Display)
1. Atualizar a interface `StageDisplaySettings` em `frontend/src/lib/types.ts` com os campos `current_slide_percent`, `next_slide_percent` e `footer_time_percent`.
2. Atualizar os padrões em `frontend/src/lib/displaySettings.ts` (`current_slide_percent: 60`, `next_slide_percent: 30`, `footer_time_percent: 10`).
3. Adicionar na aba "Monitor de Palco (Stage)" de `SettingsModal.svelte` a seção com sliders de ajuste de proporções, amostra visual dinâmica e botão de restauração.
4. Aplicar as proporções configuradas em `frontend/src/lib/views/StageView.svelte` nos 4 presets.

### Fase 3: Widget de Timer no Header do Admin
1. Adicionar o Widget de Cronômetro no cabeçalho de `frontend/src/lib/views/AdminView.svelte` com mostrador digital (`MM:SS` ou `-MM:SS`), status visual (normal, último minuto, overtime) e botões Play/Pause e Reset.
2. Conectar os botões diretamente às funções `startTimer`, `pauseTimer`, `resetTimer` de `ws.ts`.

### Fase 4: Validação e Portões de Qualidade
1. Executar `bun run check` e `bun run build` no frontend.
2. Executar `cargo fmt`, `cargo clippy` e `cargo test --workspace`.
3. Atualizar `CHANGELOG.md` e `docs/README.md`.
4. Commit e envio ao repositório GitHub.

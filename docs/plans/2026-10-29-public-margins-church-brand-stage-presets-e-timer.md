# Plano de Implementação: Margens do Public, Marca da Igreja, Presets de Stage e Timer Global

- **Status:** Em andamento
- **Data:** 2026-10-29
- **Referência:** `docs/specs/2026-10-29-public-margins-church-brand-stage-presets-e-timer.md`

---

## Fases de Execução

### Fase 1: Fundação de Dados e Backend
1. **Migração SQLite `0006_app_settings.sql`:**
   - Criar tabela `app_settings` com schema versão 6.
   - Adicionar métodos em `crates/op-db` para carregar e salvar configurações por categoria (`public`, `church_brand`, `stage`).
2. **Timer State no Backend (`crates/op-core` e `crates/op-admin`):**
   - Definir struct `TimerState` em `op-core` e adicionar ao `PresentationState`.
   - Adicionar mensagens de WebSocket para controle do timer e endpoints REST (`/api/timer/*` e `/api/settings/*`).
   - Implementar loop de contagem assíncrona ou cálculo baseado em timestamp no `op-admin`.

### Fase 2: Configurações de Public Display e Marca da Igreja
1. **Upload e Gestão de Brand da Igreja:**
   - Endpoint `POST /api/church/upload-logo` para salvar logos no diretório de dados do app (`media/church_brand/`).
   - Aba "Marca da Igreja" ou seção em `SettingsModal.svelte`.
2. **Configurações de Margem e Formatação do Public:**
   - Controles de preenchimento máximo (`fill_space`), quebra de linha (`word_wrap`), metadados e margens na aba de Configurações do Public.
   - Renderização no `PublicView.svelte` aplicando as margens, quebra de linha, tamanho de fonte e sobreposição do logotipo da igreja no canto selecionado.

### Fase 3: Presets de Stage Display e Customização de Cores
1. **Aba de Configuração do Stage Display em `SettingsModal.svelte`:**
   - Seletor dos 4 presets (Split 50/50, Modern Grid, Video Focus, Horizontal Stack).
   - Paleta de cores para texto do slide atual, próximo slide, bordas, timer e relógio.
2. **Renderização de Presets em `StageView.svelte` e no Preview do Admin:**
   - Implementar os 4 layouts com aspect-ratio 16:9 responsivo e caixas com bordas estilizadas conforme as referências visuais.
   - Suporte à exibição de Mensagem de Palco (`stage_message`).

### Fase 4: Painel de Controle de Timer e Relógio
1. **Componente de Painel de Timer (`TimerControlPanel.svelte`):**
   - Inserido na barra lateral direita do `AdminView.svelte`, logo abaixo dos previews de Public e Stage.
   - Mostrador digital, botões de ação (Iniciar, Pausar, Reiniciar, Parar) e presets rápidos de tempo.
   - Campo para disparar mensagens instantâneas para o palco.
2. **Efeitos Visuais de Contagem e Overtime:**
   - Efeito pulsante de alerta quando o tempo estiver acabando.
   - Efeito de tempo esgotado / excedido com formatação negativa (`-MM:SS`) em vermelho pulsante vivo.

### Fase 5: Favicon e Portões de Qualidade
1. Atualizar o favicon no `frontend/index.html` e verificar caminhos estáticos.
2. Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
3. Executar `bun run check` e `bun run build`.
4. Atualizar `docs/README.md` e `CHANGELOG.md`.
5. Realizar commits atômicos.

# Especificação: Correção do Modal de Configurações, Proporções no Monitor de Palco e Widget de Timer no Header

**Data:** 29 de outubro de 2026  
**Status:** Aprovado  
**Autores:** Equipe OpenPresenter  

---

## 1. Contexto e Motivação

Durante a operação prática do OpenPresenter em cabines de projeção, três necessidades operacionais foram identificadas:
1. **Estabilidade no Modal de Configurações:** Ao alterar opções (como margens na tela pública ou opções de marca), a visualização resetava abruptamente para a primeira aba ("Aparência"), impedindo a navegação e configuração fluida. Isto ocorria devido a um `$effect` reativo no Svelte 5 rastreando leituras das stores globais e forçando `activeTab = initialTab`. O usuário solicitou a adição de um botão explícito de "Salvar" e a resolução do retorno de tela.
2. **Proporções Customizáveis no Monitor de Palco (Stage Display):** Em palcos com distâncias e telas variadas, os líderes de louvor e pregadores precisam de flexibilidade para definir a proporção de espaço de cada bloco (ex: 60% para o Slide Atual, 30% para o Próximo Slide e 10% para o Relógio/Timer).
3. **Visibilidade Contínua do Cronômetro no Painel do Operador:** O operador necessita monitorar e controlar o tempo (Play, Pause, Reset) diretamente pelo cabeçalho (Header) do Admin, mesmo quando a barra lateral de pré-visualizações estiver recolhida ou com scroll.

---

## 2. Escopo

### No Escopo
- **Modal de Configurações (`SettingsModal.svelte`):**
  - Isolamento do `$effect` de abertura para disparar estritamente na transição `open: false -> true` usando `get(...)` de stores, impedindo loops reativos e resets de aba.
  - Adição do botão **"Salvar Alterações"** no rodapé do modal com confirmação visual temporária ("Alterações salvas com sucesso!").
- **Proporções no Monitor de Palco (`StageDisplaySettings` & `StageView.svelte`):**
  - Novos campos de configuração:
    - `current_slide_percent`: porcentagem do slide atual (padrão 60%, intervalo 30% a 80%).
    - `next_slide_percent`: porcentagem do próximo slide (padrão 30%, intervalo 10% a 50%).
    - `footer_time_percent`: porcentagem do bloco de relógio/timer (padrão 10%, intervalo 8% a 25%).
  - Aplicação dinâmica das proporções nos 4 presets visuais do Stage (`split-50-50`, `modern-grid`, `video-focus`, `horizontal-stack`).
  - Sliders com prévia gráfica interativa e botão "Restaurar Proporções Padrão" na aba Stage de Configurações.
- **Widget de Cronômetro no Header (`AdminView.svelte`):**
  - Mostrador digital de tempo no cabeçalho do painel do operador (`MM:SS` ou `-MM:SS` em overtime).
  - Botões compactos de Play / Pause e Reiniciar (Reset).
  - Alerta visual suave quando o tempo estiver acabando (< 60s) e vermelho pulsante em Overtime.

### Fora do Escopo
- Alterações em drivers de hardware ou captura HDMI física.

---

## 3. Decisões de Arquitetura

1. **Persistência das Proporções:** Integradas a `StageDisplaySettings` na tabela `app_settings` com a chave `stage_display`, mantendo retrocompatibilidade através de valores padrão inteligentes.
2. **Desacoplamento do `$effect` no Svelte 5:** Utilização de `get(store)` para leitura snapshot no momento de abertura, garantindo que mutações subsequentes nas stores não re-disparem o ciclo de inicialização do modal.
3. **Widget de Timer Universal no Header:** Componente integrado no `header` do `AdminView` conectado à store reativa `presentationState`, sincronizado por WebSocket e HTTP fallback.

---

## 4. Critérios de Aceite

- [x] Ao alterar qualquer configuração na aba "Tela Pública", "Monitor de Palco" ou "Marca", a tela não volta para a aba "Aparência" e os sliders funcionam fluidamente.
- [x] O rodapé do modal possui o botão "Salvar Alterações" que persiste as configurações no backend e exibe feedback de sucesso.
- [x] É possível ajustar nos sliders a proporção do Slide Atual, Próximo Slide e Barra de Tempo.
- [x] O monitor de palco (`/stage`) reflete visualmente as proporções configuradas em todos os 4 presets.
- [x] O cabeçalho do AdminView exibe o mostrador do cronômetro com botões de Play/Pause e Reset operacionais em tempo real.
- [x] Todos os testes (`cargo test`, `cargo clippy`, `bun run check`) passam com 100% de sucesso.

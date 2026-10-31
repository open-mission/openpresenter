# Spec: Fontes Maiores no Monitor de Palco e Alerta em Tela Cheia Temporária

## Status
Aprovado / Em implementação.

## Contexto e Motivação
O Monitor de Palco (`/stage`) é visualizado por pregadores, músicos e equipe de louvor a distâncias consideráveis (frequentemente entre 10 e 30 metros).
Identificou-se que:
1. **Legibilidade à distância:** O relógio, o cronômetro e a mensagem de aviso precisam de tamanhos de fonte significativamente maiores e ajustáveis nas configurações para preencher o espaço disponível.
2. **Avisos urgentes passam despercebidos:** Mensagens importantes enviadas pelo operador da cabine (como avisos de trânsito, encerramento de oração ou orientações ao pregador) ficavam restritas ao card do rodapé, podendo ser ignoradas.

## Requisitos e Escopo

### 1. Tamanhos Configuráveis de Fonte para Informações de Tempo e Aviso
- Adicionar os campos na interface `StageDisplaySettings`:
  - `clock_font_size`: tamanho da fonte do Relógio no palco (padrão: `48px`, escala de 24px a 96px).
  - `timer_font_size`: tamanho da fonte do Cronômetro/Timer (padrão: `56px`, escala de 28px a 120px).
  - `alert_font_size`: tamanho da fonte do Aviso para o Palco (padrão: `28px`, escala de 16px a 64px).
- Adicionar controles na aba "Palco" do modal de configurações (`SettingsModal.svelte`):
  - Sliders com indicador numérico em tempo real para Relógio, Timer e Aviso.
  - Botão de restauração rápida para valores recomendados.

### 2. Aviso para o Palco em Tela Cheia Temporária (Flash Pop-in / Overlay)
- Ao receber uma nova mensagem de palco disparada pelo operador:
  - O `StageView.svelte` exibe um overlay em tela cheia de alto impacto visual com fundo escuro translúcido (`bg-black/95 backdrop-blur-xl`), tipografia ampliada e destaque visual.
  - O overlay permanece em tela cheia por uma duração configurável (campo `alert_flash_duration_seconds`, padrão `5s`, configurável de 2 a 30s).
  - Pode ser ativado ou desativado pelo operador através da opção `alert_fullscreen_flash` (padrão: `true`).
  - Permite dispensar imediatamente ao clicar na tela.
  - Após a duração temporária, o overlay se fecha suavemente e a mensagem permanece visível na caixa de aviso do rodapé/lateral enquanto estiver ativa.

### 3. Paridade Visual no Preview do Operador
- Atualizar `StagePreviewBox.svelte` para refletir proporcionalmente as fontes configuradas de Relógio, Timer e Aviso.

### 4. Rótulos Flutuantes em Posicionamento Absoluto (Otimização de Espaço para Letras)
- Reposicionar os cabeçalhos/rótulos dos blocos "Slide Atual" e "A Seguir (Próximo)" com posicionamento absoluto (`position: absolute; top: ...; z-index: 10; pointer-events: none;`) com fundo translúcido sutil.
- Eliminar o consumo vertical rígido de 30px–50px causado por barras de cabeçalho em fluxo estático, liberando 100% da altura dos blocos para o cálculo do `autoScale`, maximizando o corpo e a legibilidade das letras de músicas e passagens bíblicas à distância.

## Critérios de Aceite
1. O relógio e o timer no `/stage` preenchem com maior destaque a tela e respeitam os tamanhos definidos em configurações.
2. O modal de configurações permite ajustar as fontes de Relógio (24–96px), Timer (28–120px) e Aviso (16–64px).
3. Ao disparar uma mensagem de palco, o `/stage` exibe o alerta em tela cheia e, após o tempo definido, retorna ao layout normal mantendo a mensagem no rodapé.
4. É possível desativar o pop-in em tela cheia e alterar o tempo de exibição nas configurações.
5. Os rótulos de identificação de slide ficam sobrepostos em posicionamento absoluto, garantindo área útil máxima de altura para as letras no palco.
6. `cargo fmt`, `cargo clippy`, `cargo test` e `bun run build` passam sem erros ou avisos.

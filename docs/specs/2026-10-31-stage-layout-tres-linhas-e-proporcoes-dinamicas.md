# Spec: Layout em 3 Linhas no Stage e Recálculo Dinâmico de Proporções (Soma 100%)

## Status
Aprovada

## Intenção
Oferecer no Monitor de Palco (`/stage`) a opção de exibir os itens auxiliares (Aviso, Relógio e Cronômetro) horizontalmente abaixo do Próximo Slide, criando uma estrutura de 3 linhas empilhadas de largura total (100% da tela). Permitir ao operador ajustar a altura relativa dessas 3 linhas com rebalanceamento dinâmico automático que mantém a soma das alturas sempre em exatos 100%.

## Escopo
1. **Configuração de Layout**:
   - Adicionar `stage_layout?: 'stacked-rows' | 'side-column'` em `StageDisplaySettings` (padrão: `'stacked-rows'`).
   - Defaults: `current_slide_percent = 50`, `next_slide_percent = 30`, `footer_time_percent = 20` (soma: 100%).
2. **Renderização em 3 Linhas (`stacked-rows`)**:
   - **Linha 1 (Slide Atual)**: Altura `current_slide_percent%`, largura total (100%), autoScale para textos de louvor e versículos bíblicos.
   - **Linha 2 (Próximo Slide)**: Altura `next_slide_percent%`, largura total (100%), indicação "A Seguir".
   - **Linha 3 (Barra de Informações)**: Altura `footer_time_percent%`, largura total (100%), dividida horizontalmente em:
     - Aviso para o Palco (flex 40).
     - Relógio de Parede (flex 30).
     - Cronômetro / Timer com Overtime (flex 30).
3. **Recálculo de Proporções em Tempo Real**:
   - No modal de configurações (`SettingsModal.svelte`), ao arrastar qualquer um dos 3 sliders, o valor restante `100 - novo_valor` é redistribuído proporcionalmente entre as outras duas linhas.
   - Limites mínimos de segurança: Slide Atual >= 20%, Próximo Slide >= 10%, Barra Inferior >= 10%.
   - Indicador visual explícito garantindo soma constante de 100%.
4. **Paridade no Preview (`StagePreviewBox.svelte`)**:
   - Atualização do componente de preview do operador para refletir fielmente o modo de 3 linhas horizontais ou coluna lateral.

## Critérios de Aceite
1. O seletor de layout no Palco permite alternar entre "3 Linhas Horizontais" e "Coluna Lateral Direita".
2. No modo de 3 linhas, o aviso, relógio e timer aparecem abaixo do próximo slide ocupando toda a largura horizontal.
3. Alterar qualquer slider recalcula imediatamente os outros dois de modo que a soma total seja sempre 100%.
4. O preview no painel do operador e nas configurações reflete o layout e proporções configurados.
5. Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

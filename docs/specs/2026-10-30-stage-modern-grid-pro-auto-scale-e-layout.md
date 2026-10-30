# Spec — Modernização do Stage Display: Preset Grid Moderno Pro, Auto-scale e Otimização de Espaço

## Contexto e Motivação

O monitor de retorno de palco (/stage) é essencial para cantores, músicos e pregadores acompanharem o slide atual, o próximo slide, o relógio e os timers em tempo real. Anteriormente, o palco oferecia múltiplos presets visuais que fragmentavam a experiência e utilizavam classes de tamanho de fonte estáticas, resultando em textos curtos que pareciam pequenos ou textos longos que estouravam a caixa ou exigiam rolagem desnecessária. Além disso, caixas com `rounded-2xl` e margens de `gap-4`/`gap-5` desperdiçavam área útil crítica de tela, e a caixa de avisos só aparecia quando havia mensagem ativa, causando saltos repentinos no layout.

## Intenção e Decisões

1. **Preset Único Padronizado — Grid Moderno Pro (`modern-grid`)**:
   - Simplificar as configurações do Palco mantendo apenas o preset **Grid Moderno Pro**.
   - Reduzir complexidade de manutenção e unificar a melhor experiência visual para palco.
2. **Auto-scale Dinâmico de Texto**:
   - O texto dos slides (atual e próximo) preenche automaticamente a área disponível do box.
   - Implementado com algoritmo de busca e medição em ResizeObserver, ajustando o tamanho da fonte (`font-size`) para maximizar o preenchimento sem transbordar.
   - Introduzir o parâmetro configurável `min_font_size` (tamanho mínimo da fonte) em `StageDisplaySettings`, com valor padrão de 24px (configurável na interface).
3. **Maximização do Espaço Útil**:
   - Eliminar os gaps entre as caixas (`gap-0`) e adotar cantos retos (`rounded-none` / `border-radius: 0`).
   - As divisórias entre caixas utilizam bordas simples de alta precisão técnica (`border`), ampliando a área visível do texto em até 15-20%.
4. **Caixa de Mensagens / Avisos Sempre Visível**:
   - A caixa de aviso para o palco permanece fixa na coluna lateral.
   - Quando não há mensagem ativa, exibe um estado de prontidão neutro e discreto ("Nenhum aviso no momento" com ícone suave).
   - Quando uma mensagem é enviada pelo operador, destaca-se com tipografia de alto impacto visual e contraste.

## Critérios de Aceite

- [x] O preset `modern-grid` é o padrão e único exibido nas opções de configuração do Palco.
- [x] Caixas no Stage não têm gaps nem cantos arredondados, preenchendo a tela de ponta a ponta.
- [x] O texto dos slides ajusta dinamicamente seu tamanho de fonte ao tamanho da caixa (auto-scale), respeitando `min_font_size`.
- [x] O campo `min_font_size` pode ser ajustado nas configurações de Palco.
- [x] A caixa de avisos do palco fica permanentemente visível, prevenindo quebras e deslocamentos de layout.
- [x] Todos os testes e checagens de qualidade do workspace passam com sucesso.

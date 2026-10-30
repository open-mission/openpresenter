# Spec: Correção e Aplicação Confiável de Margens na Projeção Pública (Public Screen)

## Intenção
Corrigir o cálculo e aplicação das margens personalizadas (Topo, Base e Laterais) na tela pública (`/public`), garantindo que o container de conteúdo utilize `box-sizing: border-box`, substitua limites rígidos baseados em viewport (`max-w-[96vw]`) por contenção fluida (`w-full`), e assegure que o padding seja aplicado integralmente e sem vazamento em qualquer resolução de projeção.

## Escopo
- Frontend (`PublicView.svelte`):
  - Sanitizar os valores numéricos de `margin_top`, `margin_bottom` e `margin_horizontal`.
  - Aplicar `box-sizing: border-box` no container principal de conteúdo.
  - Substituir `max-w-[96vw]` por contenção no container pai (`w-full`).
  - Utilizar `flex-1 min-h-0 min-w-0` para distribuir o conteúdo entre as margens superior e inferior sem distorcer o alinhamento vertical.

## Decisões
- Preservar a opção `ignore_margins_for_media`: imagens continuam podendo ignorar margens quando configuradas para tela cheia total.
- Compatibilidade total com os valores salvos nas configurações de tela pública (`public_display`).

## Critérios de Aceite
1. Ao ajustar as margens de topo, base ou laterais nas configurações, a área útil de texto na tela pública (`/public`) encolhe e se afasta proporcionalmente das bordas.
2. Não há estouro de layout nem corte indevido de texto à direita ou abaixo.
3. Textos longos respeitam os limites laterais e de topo/base definidos.

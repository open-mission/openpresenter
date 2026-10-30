# Spec — Borda Verde para Item ao Vivo na Barra de Setlist

## Contexto e Motivação

Na barra lateral da Setlist (`SetlistSidebar` / `MediaItemRow`), os itens apresentados ao vivo continham um indicador em pulsação contínua (`animate-ping`), o que gerava distração visual constante para o operador de projeção. Além disso, a diferenciação visual do item em transmissão dependia primariamente de um badge, sem uma delimitação marcante nas bordas do card.

## Intenção e Decisões

1. **Remoção de Indicadores Piscantes**:
   - Remover a bolinha pulsante (`animate-ping` / `animate-pulse`) do item ativo na lista de mídia.
   - Tornar a indicação estável, elegante e sem movimento repetitivo.
2. **Borda Verde Marcante no Item ao Vivo**:
   - Quando o item estiver ao vivo (`isLive = true`), aplicar borda verde esmeralda sólida (`border-emerald-500 ring-1 ring-emerald-500/50`).
   - Aplicar uma tonalidade de fundo sutilmente receptiva (`bg-emerald-950/20`) e barra lateral verde (`bg-emerald-500`), destacando imediatamente para o operador qual louvor ou trecho bíblico está projetado no momento.
   - O badge "Ao Vivo" adota estilo sóbrio e estático com borda verde (`bg-emerald-500/20 text-emerald-400 border-emerald-500/40`), mantendo alta legibilidade.

## Critérios de Aceite

- [x] O indicador piscante (`animate-ping`) foi completamente removido do item ao vivo na setlist.
- [x] O item em transmissão ao vivo exibe uma borda verde nítida (`border-emerald-500`).
- [x] A seleção e a navegação da setlist continuam fluidas e compatíveis com drag-and-drop.
- [x] Testes de workspace e verificações de frontend aprovados.

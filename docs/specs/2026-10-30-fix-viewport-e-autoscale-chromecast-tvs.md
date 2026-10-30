# Spec — Correção de Viewport e Auto-Scale em TVs e Chromecast

- **Status:** aprovada
- **Data:** 2026-10-30
- **Autor:** OpenPresenter Core Team
- **Contexto:** commit `5fd74a6cbd4f9877165ffadb5863fe2b300efc76`

## 1. Intenção

Garantir que a tela pública de projeção (`/public`) e os receptores de transmissão sem fio (Chromecast / Google Cast e Smart TVs HDMI) ocupem 100% da área útil do viewport sem colapso vertical ou distorção no cálculo do auto-scale (`autoScale.ts`).

## 2. Escopo

- Ajustar regras globais de CSS em `frontend/src/app.css` para `html`, `body` e `#app`.
- Ajustar contêiner principal de projeção em `frontend/src/lib/views/PublicView.svelte` para usar `w-full h-full min-w-full min-h-full` em vez de depender exclusivamente de `w-screen h-screen` (que colapsa em certos Web Receivers).
- Ocultar botões flutuantes de interface desnecessários em telas não interativas (Smart TVs e Cast).

## 3. Decisões

1. **Fixação estrita de dimensões no DOM raiz:**
   Definir `width: 100%`, `height: 100%`, `min-height: 100%`, `margin: 0`, `padding: 0` e `overflow: hidden` em `html, body, #app` no CSS global. Isso impede que navegadores embarcados (Chromium no Chromecast) reportem altura 0 para contêineres filhos flexíveis.
2. **Distribuição flexível em PublicView:**
   Em `PublicView.svelte`, o alinhamento vertical (`alignV`) passa a controlar o contêiner interno de texto via classes `justify-start`, `justify-center` ou `justify-end`, mantendo a caixa principal preenchendo 100% da área com auto-scale estável.
3. **Botão de tela cheia:**
   Ocultar o botão flutuante de alternar fullscreen em dispositivos móveis e TVs (`hidden md:block`), exibindo-o apenas sob hover em desktop para evitar poluição visual na transmissão.

## 4. Critérios de Aceite

- [x] Transmissão para Chromecast e navegadores de TV ocupa 100% da tela sem barras pretas laterais ou colapso de altura.
- [x] O algoritmo de `autoScale` calcula a fonte proporcional ao tamanho real da tela.
- [x] Textos centralizados verticalmente permanecem no centro exato da tela.

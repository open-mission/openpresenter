# ADR 0006: Controle de Apresentação Desacoplado, Seções Customizadas e Configurações em Abas

- **Status:** aceito
- **Data:** 2026-10-29
- **Decisores:** OpenPresenter Core Team
- **Substitui:** nenhuma
- **Contexto:** Necessidade de permitir inspeção/seleção de slides sem disparo imediato ao vivo, atalhos semânticos para partes da música e números, configurações completas com abas verticais (Aparência, Seções e Cores, Telas, Atalhos) e aplicação da identidade visual oficial.

---

## 1. Contexto e Motivação

Na versão inicial do painel operacional web, clicar em qualquer slide da mídia o disparava imediatamente para a tela de saída (`display_mode = Live`). Essa abordagem era ágil para testes, mas em ambientes reais de igreja ou eventos ao vivo (estilo ProPresenter e Holyrics), o operador precisa:
1. Selecionar e visualizar um slide internamente no painel sem jogá-lo imediatamente no telão.
2. Iniciar a apresentação explicitamente via botão dedicado ("Apresentar") ou pressionando `Enter` sobre o slide selecionado.
3. Encerrar a projeção imediatamente com `Esc`.
4. Navegar entre os slides com setas (`←` / `→`) ou espaço.
5. Pular instantaneamente para partes da música através de atalhos mnemônicos (ex: `R` para Refrão, `V` para Verso, `P` para Ponte, `I` para Intro, `F` para Final) ou digitando o número do slide seguido de `Enter`.
6. Ter um painel de **Configurações** organizado em abas verticais (inspirado nas preferências do ProPresenter e macOS), onde é possível:
   - Alternar o tema do sistema: Sistema, Escuro (Dark) e Claro (Light).
   - Customizar as cores de destaque e atalhos rápidos das partes da música (Verso, Refrão, Ponte, etc.).
   - Gerenciar as saídas gráficas (Public e Stage) e consultar/configurar todos os atalhos.
7. Substituir marcadores visuais temporários (ex: "OP") pelos arquivos de marca oficiais de `assets/branding/`.

---

## 2. Decisões Arquiteturais

1. **Desacoplamento de Foco e Projeção (Focus vs. Live)**:
   - O painel administrativo mantém dois estados distintos: `focusedSlideIndex` (slide selecionado/focado pelo operador) e `presentationState.active_slide_index` (slide efetivamente projetado ao vivo).
   - O clique simples seleciona o slide (`focusedSlideIndex`).
   - O clique duplo, o pressionamento de `Enter` ou o botão "Apresentar" transmitem o slide focado para o estado ao vivo (`select_slide`).
   - O pressionamento de `Esc` interrompe a apresentação ativa (limpa a projeção ou define `display_mode = ClearText`).

2. **Acesso Rápido a Partes da Música e Teclado Numérico**:
   - Atalhos de letra direta quando o operador não estiver em campos de input:
     - `V`: seleciona/apresenta o primeiro verso da música ativa.
     - `R`: seleciona/apresenta o refrão (ou coro).
     - `P`: seleciona/apresenta a ponte.
     - `I`: seleciona/apresenta a introdução.
     - `F`: seleciona/apresenta o encerramento/final.
   - Digitação de números: se o operador digitar um ou mais dígitos (ex: `2`, `12`), exibe-se um indicador HUD discreto; pressionar `Enter` navega e apresenta o slide ordinal correspondente.

3. **Painel de Configurações em Abas Verticais (`SettingsModal.svelte`)**:
   - Modal em abas verticais à esquerda:
     - **Geral**: opções de exibição e fluxo.
     - **Aparência**: seletor de tema (`system`, `dark`, `light`) com aplicação em tempo real na raiz do DOM (`html.dark` e `data-theme`).
     - **Partes da Música**: configuração por tipo de seção com nome, paleta de cores (Tailwind classes de fundo, texto e badge) e atalho de teclado correspondente.
     - **Telas / Saídas**: status e controle das saídas Public e Stage.
     - **Atalhos**: guia interativo com todos os atalhos de teclado.
   - Persistência das preferências no `localStorage` com valores padrão ricos.

4. **Identidade Visual Oficial**:
   - Uso inalterado dos SVGs canônicos de `assets/branding/`:
     - `open-presenter-app-icon.svg`: ícone da aplicação e favicon.
     - `open-presenter-wordmark-dark.svg`: logotipo e tipografia sobre fundos escuros.
     - `open-presenter-wordmark.svg`: logotipo e tipografia sobre fundos claros.

---

## 3. Consequências

- **Positivas:**
  - O operador ganha total controle sobre o que vai ou não para o telão, eliminando disparos acidentais durante o culto.
  - O fluxo de operação durante o louvor fica extremamente rápido e profissional com teclas de acesso direto (`R`, `V`, `P`, `[num] + Enter`).
  - O usuário pode personalizar as cores das seções conforme as preferências da sua equipe de mídia.
  - A identidade visual do OpenPresenter torna-se 100% consistente em todas as rotas e janelas.

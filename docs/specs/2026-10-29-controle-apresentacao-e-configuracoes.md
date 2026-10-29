# Spec — Controle de Apresentação, Partes da Música e Configurações em Abas

- **Status:** aprovada
- **Data:** 2026-10-29
- **Autores:** OpenPresenter Core Team
- **Decisão relacionada:** [ADR 0006](../decisions/0006-controle-de-apresentacao-secoes-customizadas-e-configuracoes.md)

## 1. Intenção

Proporcionar aos operadores de projeção controle rigoroso sobre a entrada de slides ao vivo (seleção sem disparo automático, botão "Apresentar" e disparo por tecla `Enter`), interrupção imediata por `Esc`, atalhos semânticos para partes da música (`R` para Refrão, `V` para Verso, etc.) e números com `Enter`, janela de configurações com abas verticais para customização de cores, atalhos e tema claro/escuro, e aplicação dos logos oficiais da marca.

## 2. Escopo

### Incluído

1. **Controle de Foco vs. Apresentação ao Vivo**:
   - Clicar em um slide da mídia selecionada destaca-o visualmente como focado (`focusedSlideIndex`), sem emitir disparo WebSocket ao vivo.
   - Adicionar botão **Apresentar** no cabeçalho do painel central de slides.
   - Pressionar a tecla `Enter` ou duplo-clique no slide selecionado inicia/atualiza a projeção ao vivo naquele slide.
   - Pressionar `Esc` encerra a projeção ativa (interrompe o modo Live).
   - As setas de navegação (`←` e `→` ou espaço) avançam e retrocedem entre os slides da mídia.

2. **Acesso Rápido a Partes e Navegação Numérica**:
   - Teclas diretas quando o foco não estiver em caixas de texto:
     - `V`: pula para o primeiro verso da música.
     - `R` ou `C`: pula para o refrão.
     - `P` ou `B`: pula para a ponte.
     - `I`: pula para a introdução.
     - `F` ou `O`: pula para a seção final.
   - Buffer numérico: digitar dígitos (ex: `1`, `3`) exibe um indicador de salto ordinal ("Slide #3"); ao pressionar `Enter`, o slide correspondente é focado e apresentado.

3. **Painel de Configurações em Abas Verticais (`SettingsModal.svelte`)**:
   - Layout de diálogo com sidebar vertical à esquerda com as abas:
     - **Aparência**: escolha entre Sistema (`system`), Escuro (`dark`) e Claro (`light`).
     - **Seções e Cores**: lista de tipos de seções com personalização de cor e atalho de teclado associado.
     - **Saídas**: atalhos para controle e abertura das telas Public e Stage.
     - **Atalhos**: lista descritiva de todos os comandos rápidos do teclado.
   - Botão de abertura de Configurações no cabeçalho operacional (ícone de engrenagem).

4. **Identidade Visual Oficial**:
   - Importação e uso dos arquivos oficiais de `assets/branding/` em `frontend/public/branding/` e componentes de cabeçalho.
   - Substituição do marcador "OP" pelo `open-presenter-app-icon.svg` e inclusão do `open-presenter-wordmark-dark.svg` (ou `open-presenter-wordmark.svg` em tema claro).
   - Definição do favicon para o ícone oficial da marca.

## 3. Critérios de Aceite

1. Clicar em um slide no painel central seleciona o slide sem mudar a projeção do Public ou Stage até que se clique em "Apresentar" ou se aperte `Enter`.
2. Pressionar `Esc` interrompe a apresentação ativa.
3. Teclas de atalho `V`, `R`, `P`, `I`, `F` saltam diretamente para a seção correspondente da música.
4. Digitar um número e pressionar `Enter` leva o operador diretamente àquele slide ordinal.
5. O modal de Configurações possui navegação vertical e permite alternar o tema (Dark / Light / System), customizar cores das seções e configurar seus atalhos.
6. A identidade visual oficial do OpenPresenter está presente no header, favicon e configurações sem distorções ou recolorações indevidas.
7. Todos os quality gates (`cargo fmt`, `cargo clippy`, `cargo test`, `bun run check && bun run build`) são executados e passam com zero falhas.

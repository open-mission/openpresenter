# Spec — Colapso dos Previews de Saída Public e Stage no Painel do Operador

## 1. Intenção

Permitir que o operador recolha ou expanda individualmente as caixas de pré-visualização (16:9) das telas **Public (Projetor)** e **Stage (Retorno Palco)** na barra lateral direita de previews (`AdminView.svelte`), otimizando o espaço vertical conforme a necessidade do culto ou evento.

## 2. Contexto e Problema

Na barra lateral direita do painel do operador (`AdminView.svelte`), encontram-se:
1. "Controle de Saídas" (botões de ação global de projeção)
2. "Public (Projetor)" com preview 16:9
3. "Stage (Retorno Palco)" com preview 16:9
4. "Cronômetro & Palco" (TimerControlPanel)

Anteriormente, os blocos de preview do Public e do Stage eram fixos, ocupando cada um cerca de `260px` de altura vertical na coluna lateral. Em notebooks ou telas menores, o operador precisava rolar a barra lateral frequentemente para acessar o controle de timer e mensagens de palco. Permitir colapsar cada monitor individualmente proporciona total flexibilidade de layout.

## 3. Escopo e Solução

1. **Estados Reativos e Persistência (`AdminView.svelte`):**
   - Criar estados `isPublicPreviewCollapsed` e `isStagePreviewCollapsed`, ambos inicializados como `false` por padrão e sincronizados com `localStorage` (`public_preview_collapsed` e `stage_preview_collapsed`).
   - Criar funções `togglePublicPreviewCollapse()` e `toggleStagePreviewCollapse()`.

2. **Cabeçalhos Interativos:**
   - Tornar os títulos "Public (Projetor)" e "Stage (Retorno Palco)" botões interativos de alternância acompanhados de ícone `ChevronDown` com animação de rotação (-90 graus quando recolhido).
   - Preservar os botões de ação adjacentes (botão de recarregar saída remotamente e badge de status "Conectado" / "Aguardando").

3. **Renderização Condicional dos Previews:**
   - Ocultar o container do `PublicPreviewBox` ou `StagePreviewBox` quando o respectivo monitor estiver colapsado, reduzindo o bloco para uma barra horizontal limpa e compacta.

## 4. Critérios de Aceite

1. Clicar no cabeçalho de "Public (Projetor)" deve alternar o colapso da caixa de preview do projetor.
2. Clicar no cabeçalho de "Stage (Retorno Palco)" deve alternar o colapso da caixa de preview do palco.
3. As preferências devem ser preservadas no `localStorage` após recarregar a página.
4. Os botões de recarregar e os badges de conexão devem continuar visíveis e funcionais mesmo com o preview recolhido.
5. Todos os portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test` e `bun run build`) devem passar com sucesso.

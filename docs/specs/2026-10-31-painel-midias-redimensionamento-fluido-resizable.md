# Spec: Redimensionamento Fluido do Painel de Mídias e Suíte Resizable Shadcn

## Status
Aprovada

## Intenção
Eliminar a latência perceptível no redimensionamento do painel inferior de mídias (`BottomPanel.svelte`). O problema era causado pela classe CSS `transition-[height] duration-150` que interpolava continuamente a altura durante o movimento do cursor, gerando atraso artificial. A spec estabelece a remoção da interpolação durante o arrasto, aprimora o cursor e seleção do body e disponibiliza a suíte nativa de componentes `Resizable` do shadcn-svelte (baseada na biblioteca `paneforge`).

## Escopo
1. **Componentes Resizable Shadcn (`frontend/src/lib/components/ui/resizable`)**:
   - `ResizablePaneGroup.svelte`: invólucro para grupos de painéis horizontais ou verticais.
   - `ResizablePane.svelte`: painel redimensionável com controle de porcentagem mínima e máxima.
   - `ResizableHandle.svelte`: divisor com suporte a handle visual de grip.
   - `index.ts`: exportação unificada no padrão shadcn-svelte.
2. **Fluidez no Painel de Mídias (`BottomPanel.svelte`)**:
   - Desativação de classes de transição CSS (`transition-[height]`) durante o arrasto ativo (`isDragging === false`).
   - Bloqueio de seleção de texto (`document.body.style.userSelect = 'none'`) e aplicação do cursor global `row-resize` durante a captura do mouse.
   - Indicador visual tipo pílula/grip centralizado no divisor superior.
   - Preservação da animação suave ao alternar abas, colapsar ou expandir 100%.

## Critérios de Aceite
1. O redimensionamento do painel inferior acompanha a posição do cursor em tempo real (60/120 fps) sem delay perceptível.
2. A transição de abertura e fechamento permanece suave quando acionada via cliques nos botões de abas ou chevron.
3. Componentes `ui/resizable` integrados ao ecossistema shadcn-svelte.
4. Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

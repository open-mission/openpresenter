# Spec: Altura de 75% na Abertura da Aba de Bíblia

## Status
Aprovada

## Intenção
Aumentar a área de visualização e ergonomia da aba de Bíblia no Painel Inferior (`BottomPanel.svelte`). Quando a aba de Bíblia for aberta estando o painel colapsado (ou ao alternar para a Bíblia com altura padrão reduzida), a altura do painel é automaticamente expandida para 75% da altura útil da janela (`window.innerHeight * 0.75`).

## Escopo
1. **Cálculo da Altura**:
   - Função unificada `selectTab(tab)` em `BottomPanel.svelte`.
   - Ao selecionar a aba `'bible'`, se o painel estiver colapsado ou com altura inferior a 75% da janela, ajustar `panelHeight = Math.round(window.innerHeight * 0.75)`.
   - Garantir que o valor respeite limites seguros (mínimo de 450px e máximo de `window.innerHeight - 80px`).
2. **Preservação de Ajustes Manuais**:
   - Caso o operador já tenha expandido manualmente o painel para uma altura maior, o valor existente é preservado.
   - O redimensionamento por arrasto do separador superior continua 100% funcional.

## Critérios de Aceite
1. Clicar na aba "Bíblia" com o painel fechado abre o painel ocupando 75% da altura da tela.
2. Abrir a Bíblia via Command Palette ou atalho rápido (`requestedTab = 'bible'`) aplica a mesma altura de 75%.
3. As demais abas mantêm o comportamento normal.
4. Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

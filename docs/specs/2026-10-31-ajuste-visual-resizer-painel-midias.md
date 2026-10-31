# Spec: Ajuste Visual e Espaçamento do Resizer do Painel de Mídias

## Status
Aprovada

## Intenção
Ajustar o divisor de redimensionamento superior do painel inferior de mídias (`BottomPanel.svelte`) para eliminar a sobreposição sobre os botões de abas, unificar a cor de fundo com o fundo do painel (`bg-card`) e aprimorar a visibilidade do indicador de arraste (*drag indicator*).

## Escopo
1. **Margem e Desacoplamento dos Botões**:
   - Posicionar o elemento `role="separator"` no topo do painel com altura controlada e margem inferior, em vez de sobrepor a barra de abas com posicionamento absoluto agressivo.
   - Garantir que toda a área dos botões de abas ("Músicas", "Imagens & Pastas", "Fundos & Gradientes", "Bíblia") e controles da janela permaneça 100% clicável sem captura acidental do cursor de redimensionamento.
2. **Harmonização de Fundo**:
   - Trocar a classe de fundo do separador de `bg-border/40` para `bg-card` (mesmo fundo do painel), eliminando o contraste indevido de faixa cinza.
3. **Cor do Indicador de Arraste**:
   - Atualizar a pílula de grip (`h-1 w-9 rounded-full`) de `bg-border/80` para `bg-neutral-400 dark:bg-neutral-500` com destaque em `bg-primary` no hover, foco e arraste ativo.
4. **Critérios de Aceite**:
   - Os botões das abas do painel inferior não sofrem sobreposição visual ou de clique.
   - O separador possui o mesmo fundo do painel.
   - O indicador de arraste é visível e muda para a cor primária ao interagir.
   - Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

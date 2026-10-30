# Spec: Remoção de Botões Duplicados de Sidebars no Header

## Intenção
Eliminar a duplicação dos botões de alternar barras laterais (Setlist e Previews de Saída) do cabeçalho superior do painel de controle do operador, uma vez que cada coluna já possui seus respectivos botões nativos de recolhimento no topo de cada painel e rails colapsados verticais de expansão.

## Escopo
- Frontend (`AdminView.svelte`):
  - Remover o botão de alternância da barra lateral esquerda (`PanelLeftClose` / `PanelLeftOpen`) no início do `<header>`.
  - Remover o botão de alternância da barra lateral direita (`PanelRightClose` / `PanelRightOpen`) no final do `<header>`.
  - Preservar os controles nativos existentes nos próprios painéis e rails laterais colapsados.

## Decisões
- O header superior fica focado na identificação da aplicação, busca global (⌘K), cronômetro, conectividade e acesso rápido às telas e configurações.
- A experiência de abrir e fechar as barras laterais permanece inalterada e intuitiva através dos rails e cabeçalhos dedicados de cada painel.

## Critérios de Aceite
1. O cabeçalho superior não exibe mais os botões de recolher/expandir sidebars.
2. A barra lateral da Setlist pode ser recolhida pelo botão no seu próprio topo e expandida clicando no rail lateral esquerdo.
3. A barra lateral de Previews pode ser recolhida pelo botão no seu próprio topo e expandida clicando no rail lateral direito.

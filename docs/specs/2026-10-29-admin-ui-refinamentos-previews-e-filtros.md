# Especificação: Refinamentos da Interface do Administrador (Previews, Timer e Filtros)

## 1. Intenção

Otimizar o espaço visual e a ergonomia do painel de administração (`AdminView.svelte`), removendo botões redundantes de abertura de janela, permitindo recolher o widget de cronômetro do rodapé para dar mais espaço aos previews de saída, e integrando um botão toggle de filtros por categoria diretamente na barra de busca da Setlist como input group.

## 2. Escopo

1. **Remoção de Botões Redundantes de Abertura nos Previews**:
   - Remover os botões "Abrir Janela do Public" e "Abrir Janela do Stage" que ficavam abaixo das caixas de preview 16:9 na barra lateral direita.
   - O acesso às janelas de saída continua disponível de forma limpa nos botões da barra superior e no modal de compartilhamento via QR Code.
2. **Widget Colapsável de Cronômetro & Palco (`TimerControlPanel`)**:
   - Adicionar estado recolhível (`isCollapsed`) no widget de cronômetro.
   - Cabeçalho interativo com ícone `ChevronDown` e indicação compacta do status do timer quando recolhido.
   - Quando recolhido, o widget encolhe de ~220px para ~36px, liberando espaço vertical substancial para os previews de saída.
3. **Toggle de Filtros por Categoria como Input Group na Busca**:
   - Transformar a barra de busca da Setlist em um input group acoplado.
   - Adicionar botão toggle de filtro (`Filter`) integrado no lado direito do campo de busca.
   - As opções de categoria (Todos, Louvores, Bíblia, Avisos) passam a ser exibidas apenas quando o toggle estiver ativo.
   - Destaque visual no botão quando qualquer filtro estiver ativo.

## 3. Decisões Arquiteturais

1. **Economia de Espaço Vertical e Horizontal**:
   - A remoção de botões redundantes e a capacidade de recolher o timer maximizam o espaço dedicado à visualização dos slides e previews sem perder funcionalidades.
2. **Input Group Ergonômico**:
   - A junção do campo de busca com o botão de filtro reduz a sobrecarga cognitiva da barra lateral e preserva a velocidade de navegação na setlist.

## 4. Critérios de Aceite

- [x] Os botões "Abrir Janela do Public" e "Abrir Janela do Stage" foram removidos da sidebar direita.
- [x] O widget "Cronômetro & Palco" pode ser recolhido e expandido com um clique no cabeçalho.
- [x] O campo de busca da Setlist possui botão integrado de filtro que exibe/oculta as categorias.
- [x] Portões de qualidade passam: `cargo fmt --all`, `cargo clippy`, `cargo test`, `bun run check` e `bun run build`.

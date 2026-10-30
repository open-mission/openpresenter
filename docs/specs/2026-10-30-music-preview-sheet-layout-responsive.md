# Especificação: Layout Responsivo e Correção de Espremimento na Visualização de Músicas

## Contexto e Motivação
Na aba de "Músicas" do painel inferior (`BottomPanel.svelte`), ao clicar em um card para abrir o sheet de visualização/preview de letra (`viewingSong`), o painel de preview ocupava uma largura fixa rígida excessiva (`w-[780px]` a `w-[940px]`), espremendo o catálogo de músicas restante contra o canto esquerdo da tela. Como o catálogo utilizava classes de viewport (`lg:grid-cols-3 xl:grid-cols-4`), os cards eram forçados a se dividir em até 4 colunas em um espaço de apenas 200–300px, quebrando textos, botões e tornando a interface ilegível.

## Escopo
1. **Layout Adaptativo na Aba de Músicas (`BottomPanel.svelte`):**
   - Quando o preview de letra (`viewingSong`) estiver ativo:
     - O catálogo à esquerda atua como barra de seleção lateral compacta (`w-80 sm:w-96 shrink-0`), exibindo as músicas em coluna única (`grid-cols-1`) com cards organizados, botões intactos e destaque visual para a música sob visualização.
     - O painel de visualização de letra ocupa o restante do espaço disponível (`flex-1 min-w-0`), acomodando confortavelmente os slides em proporção 16:9 e o cabeçalho de ações.
   - Quando não houver preview aberto (`viewingSong === null`):
     - O catálogo preenche 100% da largura útil (`flex-1`), com grade fluida e responsiva (`grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5`).
2. **Robustez dos Cards de Música:**
   - Garantir que botões de ação ("+ Setlist", "Ao Vivo") usem `shrink-0` e textos de título/artista adotem truncamento elegante sem quebrar layout sob qualquer resolução de tela.

## Decisões Técnicas
- **Separação Mestre-Detalhe (Master-Detail Layout):** A transição de grade ampla para lista de seleção lateral quando o detalhe é aberto é o padrão consagrado em interfaces modernas de mídia, permitindo que o usuário alterne rapidamente entre músicas mantendo a prévia visível.
- **Fechamento e Atalhos:** O botão de fechar (X) restaura instantaneamente o catálogo para o modo grade ampla.

## Critérios de Aceite
- Ao abrir a prévia de letra de uma música, nenhum card ou botão do catálogo fica espremido ou cortado.
- Os cards na coluna lateral mantêm legibilidade total do título, tom e botões "+ Setlist" e "Ao Vivo".
- A prévia da música preenche o espaço restante de maneira harmoniosa, com slides legíveis em 16:9.
- Ao fechar a prévia, o catálogo retorna suavemente ao modo grade ampla.
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

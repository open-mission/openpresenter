# Spec — Resiliência e Agilidade na Busca de Músicas Online no Command Palette

## Intenção

Corrigir a inconsistência reportada no Command Palette (`CommandPalette.svelte`), onde pesquisar músicas online frequentemente exigia que o operador clicasse repetidamente nos botões "Online" ou "Web" para que a busca realmente fosse disparada e exibisse resultados.

## Problema

1. **Botão de alternância com comportamento confuso no cabeçalho:** O botão no canto superior direito do input do Command Palette exibia o rótulo da aba atual (`Músicas 🎵`, `Bíblia 📖` ou `Web 🌐`) e avançava ciclicamente (`local -> bible -> online`). Quando o operador estava na aba padrão de Músicas e desejava pesquisar na web, ao clicar no botão que exibia "Músicas 🎵", a aba mudava para Bíblia. Era necessário um segundo clique para chegar na Web, e um terceiro clique desarmava a busca web e voltava para Músicas.
2. **Concorrência e requisições duplicadas (*race conditions*):** Ao clicar em "Online" ou selecionar "Buscar na internet", o evento de clique executava `searchOnline(query)` imediatamente, enquanto a reatividade do Svelte 5 disparava o `$effect` com `setTimeout(150)` chamando `searchOnline(query)` uma segunda vez. Como nenhuma chamada possuía controle de cancelamento (`AbortController`) ou identificador sequencial de busca (`currentSearchId`), respostas concorrentes podiam sobrescrever o estado, limpar resultados ou gerar estados inconsistentes de carregamento (`isLoading`).
3. **Seleção padrão inadequada ao digitar título de louvor inédito:** Quando o operador digitava um título de música inexistente no catálogo local (`localResults.length === 0`), a seleção padrão (`selectedIndex = 0`) apontava para "Abrir Bíblia Sagrada". Ao pressionar Enter, em vez de buscar online a música digitada, o modal alternava para a busca bíblica, forçando o operador a procurar e clicar novamente no botão Web.
4. **Perda de foco no campo de texto:** Ao clicar nas abas ou botões de seleção de modo, o foco saía do `<input>`, impedindo a digitação contínua e a submissão via Enter sem que o usuário clicasse novamente no campo ou nos botões.
5. **Falta de disparo imediato via Enter na aba Online:** Quando na aba Online, se nenhum resultado estivesse selecionado ou os resultados estivessem vazios, pressionar Enter não forçava uma re-execução imediata da busca.

## Decisões

1. **Segmented Control direto no cabeçalho:** Substituir o botão rotativo cíclico por um controle segmentado claro com 3 botões dedicados: `[🎵 Músicas]`, `[📖 Bíblia]` e `[🌐 Web]`. Cada botão alterna imediatamente para o modo correspondente em um único clique.
2. **Centralização da alternância com `switchTab`:** Criar a função `switchTab(tab)` que atualiza a aba ativa, cancela timers de debounce pendentes, aborta requisições em trânsito com `AbortController`, sincroniza o estado de busca e preserva/restaura o foco no `<input>`. Se a aba atual já for a selecionada (ex: clicar novamente em "Web"), a busca é re-executada imediatamente.
3. **Controle atômico de busca assíncrona (`AbortController` e `searchId`):** Implementar cancelamento atômico em `searchOnline`, `searchLocal` e `searchBible`. Apenas a resposta da busca mais recente (`searchId === currentSearchId`) tem permissão para alterar o estado de resultados e desativar `isLoading`.
4. **Desduplicação reativa no `$effect`:** Armazenar `lastSearchedTab` e `lastSearchedQuery`. O `$effect` de debounce só dispara se houver alteração real de parâmetros não executada pelo `switchTab`.
5. **Pré-seleção inteligente de "Buscar na internet" no Enter:** Quando o operador digitar um texto e não houver correspondências no catálogo local (`localResults.length === 0`), caso não seja uma referência bíblica, a opção pré-selecionada passa a ser automaticamente a Ação 2 ("Buscar na internet"). Pressionar Enter já dispara a busca web diretamente.
6. **Submissão forçada com Enter na aba Online:** Se o operador estiver na aba Online e pressionar Enter enquanto a lista estiver vazia ou com termo preenchido, a busca online é disparada imediatamente sem aguardar o temporizador de debounce.
7. **Correção de índice em `localResults`:** Corrigir a comparação de `isSelected = selectedIndex === idx + 3` para os itens do banco local, prevenindo seleções duplicadas com a ação rápida 2.

## Critérios de Aceite

- [x] O cabeçalho possui controle segmentado direto com os modos Músicas, Bíblia e Web.
- [x] Clicar em "Web" ou na aba "Online" dispara a pesquisa online imediatamente com um único clique.
- [x] Clicar repetidamente em "Web" ou "Online" força a re-execução da pesquisa sem travar ou alternar para outra aba.
- [x] Digitar uma música inexistente no catálogo local e teclar Enter aciona diretamente a busca online.
- [x] O foco permanece no campo de texto após a troca de abas.
- [x] Não ocorrem requisições redundantes ou sobrescritas por *race conditions* de debounce.
- [x] Portões de qualidade (`cargo fmt`, `clippy`, `test`, `svelte-check`, `vite build`) 100% verdes.

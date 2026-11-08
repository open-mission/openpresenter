# Spec — Hierarquia Visual de Pastas na Navegação da Aba de Mídias (Imagens)

## Intenção

Na aba de Mídias (Imagens), a barra lateral "Navegação de Pastas" exibia subpastas físicas e mapeadas como uma lista plana (flat list) com a mesma indentação, tornando difícil distinguir quando uma pasta está dentro de outra (ex.: `Cultos/2026/Fevereiro`).

Esta alteração transforma a listagem lateral em uma árvore hierárquica genuína (Tree View multinível), onde a estrutura de diretórios aninhados é imediatamente visível através de:
1. Indentação proporcional ao nível de profundidade (`depth`);
2. Linhas guias verticais que conectam visualmente as subpastas à sua pasta mãe;
3. Botões de recolher e expandir (chevrons `▼` / `▶`) individuais para pastas que possuem filhos;
4. Autoexpansão dos ramos ancestrais ao selecionar uma subpasta profunda via árvore ou File Explorer;
5. Distinção clara entre pastas com filhos e pastas folha (sem filhos).

---

## Escopo

- **Frontend (`frontend/src/lib/components/BottomPanel.svelte`)**:
  - Definição do modelo de dados `FolderTreeNode` contendo `name`, `relative_path`, `count`, `depth` e `children: FolderTreeNode[]`.
  - Implementação da função utilitária `buildFolderTree(subfolders: MediaSubfolder[])` que converte a lista de caminhos relativos em uma árvore hierárquica ordenada alfabeticamente.
  - Derivação reativa das árvores `appImageFolderTree` (para a Pasta do App) e para cada pasta mapeada externa em `folders`.
  - Snippet recursivo `renderTreeNode` em Svelte 5 com manipulação de eventos, expansão/recolhimento e linhas conectivas verticais.
  - Efeito reativo para autoexpandir pastas ancestrais quando uma subpasta é ativada.
  - Ajuste ergonômico da largura da barra lateral (`w-56`) para acomodar níveis aninhados com conforto visual.

---

## Decisões Técnicas

1. **Construção da Árvore de Diretórios por Caminho Relativo**:
   - A partir de `relative_path` (ex.: `"Cultos/2026/Fevereiro"`), a função decompõe os segmentos (`split('/')`) e constrói recursivamente os nós na árvore, associando contagens e determinando `depth` (0, 1, 2...).
2. **Snippet Recursivo com Indentação Matemática**:
   - O Svelte 5 permite renderização recursiva limpa através de snippets (`{#snippet renderTreeNode(node, folderId)} ... {@render renderTreeNode(child, folderId)} {/snippet}`).
   - O padding à esquerda é calculado como `node.depth * 14 + 8px`, mantendo alinhamento consistente com o botão expander e ícone da pasta.
3. **Estado de Expansão e Autoexpansão Reativa**:
   - O estado de expansão de cada nó é mantido em um dicionário reativo `expandedSubfolders: Record<string, boolean>`. Por padrão, se não estiver explicitamente recolhido, o nó inicia expandido (`expandedSubfolders[path] ?? true`).
   - Ao alterar `selectedSubfolder`, um `$effect` garante que todos os nós pais do caminho estejam abertos.

---

## Critérios de Aceite

1. **Visibilidade Hierárquica Imediata**:
   - Quando existirem pastas aninhadas (ex.: `Cultos/2026`), a pasta `2026` deve aparecer recuada e visualmente subordinada à pasta `Cultos`.
   - Pastas com subpastas devem exibir chevron para expandir e recolher seus filhos.
2. **Navegação Sincronizada**:
   - Clicar em qualquer nó da árvore deve filtrar os itens para aquele caminho relativo e atualizar a visualização File Explorer e os breadcrumbs.
3. **Compatibilidade com Pastas do App e Pastas Mapeadas**:
   - A hierarquia deve funcionar de forma idêntica tanto para subpastas da Pasta do App quanto para subpastas de diretórios externos mapeados.
4. **Portões de Qualidade**:
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run build` devem passar sem erros nem advertências.

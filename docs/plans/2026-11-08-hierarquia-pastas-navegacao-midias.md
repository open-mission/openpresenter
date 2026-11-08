# Plano — Hierarquia Visual de Pastas na Navegação da Aba de Mídias (Imagens)

Este plano orienta a implementação das mudanças especificadas em [`docs/specs/2026-11-08-hierarquia-pastas-navegacao-midias.md`](../specs/2026-11-08-hierarquia-pastas-navegacao-midias.md).

---

## Etapas de Execução

### Etapa 1: Modelagem e Funções de Montagem de Árvore (`BottomPanel.svelte`)
- [x] Definir a interface `FolderTreeNode` com `name`, `relative_path`, `count`, `depth` e `children`.
- [x] Implementar a função `buildFolderTree(subfolders: MediaSubfolder[])` para agrupar e aninhar recursivamente diretórios com ordenação alfabética.
- [x] Criar a derivada `appImageFolderTree` a partir de `allAppImageSubfolders`.
- [x] Adicionar estado `expandedSubfolders` e função `toggleSubfolderExpand(relPath)`.
- [x] Adicionar efeito `$effect` que autoexpande todos os diretórios pais de `selectedSubfolder`.

### Etapa 2: Implementação do Snippet Recursivo de Renderização da Árvore
- [x] Implementar o snippet `{#snippet renderTreeNode(node, folderId)}`.
- [x] Adicionar botão expander (Chevron) para nós com filhos e espaçador para nós folhas.
- [x] Adicionar recuo visual proporcional (`style="padding-left: {node.depth * 12 + 6}px"`).
- [x] Renderizar linha guia vertical conectando o nó pai a seus filhos.
- [x] Aplicar no bloco da Pasta do App e na seção de Pastas Mapeadas.
- [x] Ajustar largura da barra lateral para `w-56` garantindo legibilidade em múltiplos níveis.

### Etapa 3: Validação de Qualidade e Documentação
- [x] Atualizar `docs/README.md` linkando a nova spec e o novo plano.
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.
- [x] Executar portões de qualidade:
  - `cargo fmt --all`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `npm --prefix frontend run build`

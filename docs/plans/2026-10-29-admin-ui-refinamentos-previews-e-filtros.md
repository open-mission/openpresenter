# Plano de Implementação: Refinamentos da Interface do Administrador

## 1. Etapas

### Etapa 1: Previews na Sidebar Direita (`AdminView.svelte`)
- Remover os botões "Abrir Janela do Public" e "Abrir Janela do Stage" abaixo de `<PublicPreviewBox>` e `<StagePreviewBox>`.

### Etapa 2: Widget Colapsável de Cronômetro (`TimerControlPanel.svelte`)
- Adicionar estado `isCollapsed = $state(false)`.
- Adicionar cabeçalho interativo com `ChevronDown` e indicador compacto quando recolhido.
- Condicionar o corpo do painel ao estado `!isCollapsed`.

### Etapa 3: Input Group com Toggle de Filtro na Setlist (`AdminView.svelte`)
- Adicionar estado `showCategoryFilters = $state(false)`.
- Reestruturar o input de busca como input group contendo o botão de alternância com ícone `Filter`.
- Condicionar a renderização das pílulas de categoria (`Todos`, `Louvores`, `Bíblia`, `Avisos`) a `showCategoryFilters`.

### Etapa 4: Documentação e Portões de Qualidade
- Atualizar `docs/README.md` e `CHANGELOG.md`.
- Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Executar `bun run check` e `bun run build`.

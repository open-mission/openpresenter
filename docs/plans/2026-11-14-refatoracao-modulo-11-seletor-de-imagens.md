# Plano de Execução — Refatoração do Módulo 11 (Seletor de Imagens & Mídias Locais)

## Fases de Execução

### Fase 1: Fundação de Tipos, Defaults e Controlador
- Criar `frontend/src/lib/components/image-selector/types.ts`:
  - Tipos e interfaces: `ImageSelectorMode`, `ImageSelectorProps`, callbacks.
- Criar `frontend/src/lib/components/image-selector/defaults.ts`:
  - Constantes de configuração e textos padrão.
- Criar `frontend/src/lib/components/image-selector/controller.svelte.ts`:
  - Runes Svelte 5 (`$state`, `$derived`).
  - Estados: `open`, `mode`, `images`, `folders`, `selectedFolderFilter`, `searchQuery`, `selectedImageIds`, `selectedFolderId`, `isLoading`, `isMaximized`, `feedbackMessage`, `dialogEl`, `searchInputEl`.
  - Métodos: `syncOpen`, `loadData`, `toggleImageSelection`, `selectAll`, `confirmSingle`, `confirmFolder`, `handleKeydown`, `close`.

### Fase 2: Componentes Auxiliares e Submódulos
- Criar `frontend/src/lib/components/image-selector/ImageSelectorHeader.svelte`:
  - Ícone de imagem, título, subtítulo, botão Maximizar e botão Fechar com `aria-label` e foco visível.
- Criar `frontend/src/lib/components/image-selector/ImageSelectorModeTabs.svelte`:
  - Alternância entre modo "Imagens Individuais" e "Pasta Completa" com `role="tablist"` / `role="tab"` e campo de busca integrado.
- Criar `frontend/src/lib/components/image-selector/ImageSelectorFolderFilter.svelte`:
  - Filtro por pastas com scroll horizontal, contadores, botão "Selecionar Todas" e alvos táteis confortáveis `>= 28px`.
- Criar `frontend/src/lib/components/image-selector/ImageSelectorGrid.svelte`:
  - Grade responsiva com cards acessíveis (`role="checkbox"` / `aria-checked`), preview com lazy loading, checkbox contrastante e legenda com piso tipográfico `>= 11px`.
- Criar `frontend/src/lib/components/image-selector/ImageSelectorFolderList.svelte`:
  - Lista de pastas com semântica de rádio (`role="radiogroup"` / `role="radio"`), contadores e alerta inline acessível (`role="alert"`) substituindo o `alert()`.
- Criar `frontend/src/lib/components/image-selector/ImageSelectorFooter.svelte`:
  - Rodapé com contagem de seleção, atalhos de teclado, botão Cancelar e botão de confirmação.

### Fase 3: Refatoração do Orquestrador `ImageSelectorModal.svelte`
- Substituir o conteúdo monolítico pelo novo orquestrador conectando o controlador e os submódulos da pasta `image-selector/`.
- Garantir `role="dialog"`, `aria-modal="true"`, `aria-labelledby`, `tabindex="-1"`, armadilha de foco via `focusTrap.ts` e delegação da tecla `Escape`.
- Reduzir o orquestrador para menos de 130 linhas.

### Fase 4: Verificação e Portões de Qualidade
- `npm run check` (0 erros e 0 warnings).
- `npm run build` (sucesso).
- `npx impeccable detect frontend/src --json` (0 apontamentos).
- `cargo test -p op-db -p op-admin` (78 testes passando).
- Auditoria e pós-fix do Impeccable Critique (`.impeccable/critique/`).
- Atualização da documentação (`CHANGELOG.md`, `docs/README.md`).

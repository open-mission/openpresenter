# Plano de Execução — Refatoração do Módulo 10 (Seletor de Fundos & Gradientes)

## Fases de Execução

### Fase 1: Fundação de Tipos, Defaults e Controlador
- Criar `frontend/src/lib/components/background-selector/types.ts`:
  - Tipos e interfaces: `BackgroundCategory`, `BackgroundSelectorProps`, callbacks.
- Criar `frontend/src/lib/components/background-selector/defaults.ts`:
  - Constantes de categorias com identificadores, rótulos e metadados.
- Criar `frontend/src/lib/components/background-selector/controller.svelte.ts`:
  - Runes Svelte 5 (`$state`, `$derived`).
  - Estados: `open`, `currentValue`, `selectedCategory`, `searchQuery`, `localWallpapers`, `isLoadingLocal`, `isMaximized`, `dialogEl`, `searchInputEl`.
  - Métodos: `syncOpen`, `loadLocalWallpapers`, `selectPreset`, `selectLocal`, `handleKeydown`, `toggleMaximize`, `close`.

### Fase 2: Componentes Auxiliares e Submódulos
- Criar `frontend/src/lib/components/background-selector/BackgroundHeader.svelte`:
  - Ícone de paleta semântico, título, subtítulo, botão Maximizar e botão Fechar com `aria-label` e foco visível.
- Criar `frontend/src/lib/components/background-selector/BackgroundToolbar.svelte`:
  - Campo de busca com ícone e label acessível + breadcrumb da categoria ativa.
- Criar `frontend/src/lib/components/background-selector/BackgroundSidebar.svelte`:
  - Navegação de categorias com badges de contagem com piso `>= 11px` e semântica acessível.
- Criar `frontend/src/lib/components/background-selector/BackgroundCard.svelte`:
  - Card acessível sem aninhamento inválido de botões, suporte a `Enter`/`Space`, indicador de seleção e prévia 16:9 com texto de exemplo no piso `>= 11px`.
- Criar `frontend/src/lib/components/background-selector/BackgroundGrid.svelte`:
  - Container responsivo da grade de cartões com estado vazio amigável quando a busca não tiver resultados.
- Criar `frontend/src/lib/components/background-selector/BackgroundFooter.svelte`:
  - Rodapé com dica de seleção automática e botão Fechar.

### Fase 3: Refatoração do Orquestrador `BackgroundSelectorModal.svelte`
- Substituir o conteúdo monolítico pelo novo orquestrador conectando o controlador e os submódulos da pasta `background-selector/`.
- Garantir `role="dialog"`, `aria-modal="true"`, `aria-labelledby`, `tabindex="-1"`, trap de foco com `focusTrap.ts` e delegação da tecla `Escape`.
- Reduzir o orquestrador para menos de 120 linhas.

### Fase 4: Verificação e Portões de Qualidade
- `npm run check` (0 erros e 0 warnings).
- `npm run build` (sucesso).
- `npx impeccable detect frontend/src --json` (0 apontamentos).
- `cargo test -p op-db -p op-admin` (78 testes passando).
- Auditoria e pós-fix do Impeccable Critique (`.impeccable/critique/`).
- Atualização da documentação (`CHANGELOG.md`, `docs/README.md`).

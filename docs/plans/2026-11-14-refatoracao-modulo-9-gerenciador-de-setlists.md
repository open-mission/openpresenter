# Plano de Execução — Refatoração do Módulo 9 (Gerenciador de Setlists)

## Fases de Execução

### Fase 1: Fundação de Tipos, Defaults e Controlador
- Criar `frontend/src/lib/components/setlist-editor/types.ts`:
  - Tipos e interfaces: `WallpaperMode`, `SetlistEditorProps`, callbacks de salvamento e fechamento.
- Criar `frontend/src/lib/components/setlist-editor/defaults.ts`:
  - Opções de `WALLPAPER_MODES` com labels e identificadores.
- Criar `frontend/src/lib/components/setlist-editor/controller.svelte.ts`:
  - Runes Svelte 5 (`$state`, `$derived`).
  - Estados: `open`, `initialSetlist`, `title`, `description`, `selectedWallpaper`, `wallpaperMode`, `availableWallpapers`, `isLoading`, `isUploading`, `isBackgroundModalOpen`, `errorMessage`, `isMaximized`, `showDiscardConfirm`.
  - Métodos: `syncOpen`, `getFormSnapshot`, `isDirty`, `requestClose`, `confirmDiscard`, `cancelDiscard`, `loadWallpapers`, `handleFileUpload`, `handleSave`, `handleKeydown`.

### Fase 2: Componentes Auxiliares e Submódulos
- Criar `frontend/src/lib/components/setlist-editor/SetlistHeader.svelte`:
  - Cabeçalho acessível com ícone semântico, título dinâmico (Novo vs Editar), atalho `⌘Enter`, botões Maximizar e Fechar.
- Criar `frontend/src/lib/components/setlist-editor/SetlistMetadata.svelte`:
  - Campos de Título da Setlist e Descrição / Observações com tokens semânticos, `id`/`for` acessíveis e foco inicial no título.
- Criar `frontend/src/lib/components/setlist-editor/SetlistWallpaperSelector.svelte`:
  - Seletor de papel de parede da projeção:
    - Botões de ação ("Explorar Fundos & Gradientes" e "Carregar Imagem").
    - Card de destaque de fundo CSS/gradiente ativo (`resolveBackground`).
    - Grade de papéis de parede com opção "Preto Puro" e imagens com `aria-label` e badge de seleção contrastante.
    - Seletor de modo de ajuste (`Preencher`, `Ajustar`, `Centralizar`, `Repetir`) com semântica de grupo e alvos táteis confortáveis.
- Criar `frontend/src/lib/components/setlist-editor/SetlistConfirmDiscard.svelte`:
  - Modal acessível de confirmação de descarte de alterações (`role="alertdialog"`), com armadilha de foco e retorno ao botão gatilho.

### Fase 3: Refatoração do Orquestrador `SetlistModal.svelte`
- Substituir o conteúdo monolítico pelo novo orquestrador conectando o controlador e os submódulos da pasta `setlist-editor/`.
- Garantir `role="dialog"`, `aria-modal="true"`, `aria-labelledby`, `tabindex="-1"`, `inert` quando o diálogo de confirmação estiver aberto, e armadilha de foco via `focusTrap.ts`.
- Reduzir o orquestrador para menos de 150 linhas.

### Fase 4: Verificação e Portões de Qualidade
- `npm run check` (0 erros e 0 warnings).
- `npm run build` (sucesso).
- `npx impeccable detect frontend/src --json` (0 apontamentos).
- `cargo test -p op-db -p op-admin` (todos os testes passando).
- Auditoria e pós-fix do Impeccable Critique (`.impeccable/critique/`).
- Atualização da documentação (`TODO.md`, `CHANGELOG.md`, `docs/README.md`).

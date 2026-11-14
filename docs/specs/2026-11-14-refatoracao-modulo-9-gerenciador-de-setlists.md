# Especificação — Refatoração do Módulo 9 (Gerenciador de Setlists)

## 1. Contexto e Motivação

O `SetlistModal.svelte` é o componente central para criação e edição de Setlists (listas de programação de cultos e eventos). Além dos metadados básicos (título e descrição/observações), ele configura o papel de parede padrão da projeção pública (`wallpaper_path`) e o modo de exibição (`wallpaper_mode`: `cover`, `contain`, `center`, `repeat`), exibido na tela principal do projetor quando não há slides ao vivo.

Atualmente o componente possui 457 linhas em um arquivo monolítico com pontuação **20 / 40** no Impeccable Critique (`Requer Refatoração Estrutural`).

### Diagnóstico de Problemas (Baseline):
- **[P0] Tema Escuro Rígido (`neutral-*`, `zinc-*`, `bg-black/80`)**: Elementos com fundos e bordas escuras fixas (`bg-neutral-900`, `border-neutral-800`, `bg-neutral-950`, `text-white`), inviabilizando a visualização no Modo Claro (`data-theme="light"`).
- **[P0] Uso de `confirm()` Nativo**: Linha 52 invoca `confirm('Existem alterações não salvas...')`, bloqueando a thread e violando os padrões de acessibilidade WAI-ARIA.
- **[P1] Falhas Graves de WAI-ARIA**: Modal sem `aria-labelledby` e `aria-describedby`; ausência de aprisionamento de foco (`focusTrap`); botões de wallpaper sem descrição acessível; seletor de modo sem semântica adequada.
- **[P1] Violação do Piso Tipográfico**: Texto `text-[9px]` no rótulo de arquivos de papel de parede e falta de hierarquia padronizada.
- **[P1] Monólito sem Modularização**: Lógica de formulário, leitura de base64 (`FileReader`), chamadas de API (`/api/media/wallpapers`, `/api/media/upload`, `/api/setlists`), integração com `BackgroundSelectorModal` e renderização de previews condensados em um único arquivo.
- **[P2] Alvos Interativos Reduzidos**: Botões de modo de ajuste com área tátil apertada e sem estado de foco acessível com alto contraste.
- **[P2] Ausência de Atalhos de Teclado**: Sem suporte a `⌘Enter` / `Ctrl+Enter` para salvar; delegação de `Escape` não hierarquizada com o modal de confirmação.

---

## 2. Metas de Design e Arquitetura

1. **Decomposição Modular**:
   Criar a pasta `frontend/src/lib/components/setlist-editor/` contendo:
   - `types.ts`: interfaces de dados (`SetlistModalProps`, `WallpaperItem`, `WallpaperMode`, etc.).
   - `defaults.ts`: valores padrão e opções de ajuste de wallpaper (`cover`, `contain`, `center`, `repeat`).
   - `controller.svelte.ts`: classe de controle com runes Svelte 5 (`$state`, `$derived`), snapshot de dirty state, upload assíncrono de imagem com feedback visual, persistência via `/api/setlists`, controle de confirmação de descarte e atalhos de teclado (`⌘Enter`, `Escape`).
   - `SetlistHeader.svelte`: cabeçalho acessível com ícone semântico, título dinâmico (Novo vs Editar), atalho `⌘Enter`, botões Maximizar e Fechar.
   - `SetlistMetadata.svelte`: campos de Título (obrigatório) e Descrição/Observações com tokens semânticos e labels associadas (`for`/`id`).
   - `SetlistWallpaperSelector.svelte`: seletor de papel de parede da projeção com:
     - Botões de ação com alvos táteis confortáveis ("Explorar Fundos & Gradientes" e "Carregar Imagem").
     - Card de destaque para fundo procedural/gradiente ativo (`resolveBackground`).
     - Grade responsiva de opções de wallpapers (incluindo "Preto Puro" e imagens com `aria-label` e badge de seleção contrastante).
     - Seletor de modo de ajuste (`Preencher`, `Ajustar`, `Centralizar`, `Repetir`) com semântica de grupo e alvos `>= 28×28px`.
   - `SetlistConfirmDiscard.svelte`: diálogo acessível de confirmação de descarte de alterações (`role="alertdialog"`), com foco inicial seguro e retorno ao botão gatilho.

2. **Adoção Estrita de Tokens Semânticos**:
   - Eliminação de `neutral-*`, `zinc-*`, `bg-black/80`, `text-white`.
   - Adoção de `bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`, `bg-primary`, `text-primary-foreground`.
   - Garantia de contraste WCAG AA tanto no modo escuro quanto no modo claro (`data-theme="light"`).

3. **Acessibilidade WAI-ARIA e Teclado**:
   - Diálogo com `role="dialog"`, `aria-modal="true"`, `aria-labelledby="setlist-dialog-title"`, `aria-describedby="setlist-dialog-desc"`.
   - Gerenciamento de foco via `focusTrap.ts` compartilhado.
   - Foco inicial colocado no campo de Título da Setlist ao abrir o modal.
   - Suporte a `⌘Enter` / `Ctrl+Enter` para salvar diretamente.

---

## 3. Critérios de Aceite e Portões

1. `SetlistModal.svelte` orquestrador limpo (< 150 linhas).
2. Pontuação no Impeccable Critique evoluindo de **20/40** para `>= 36/40` (`Aprovado para Produção`).
3. Zero erros e zero warnings no `npm run check`.
4. Build de produção `npm run build` bem-sucedido.
5. Zero achados no `npx impeccable detect frontend/src --json`.
6. Validação de piso tipográfico `>= 11px` e alvos `>= 28×28px`.

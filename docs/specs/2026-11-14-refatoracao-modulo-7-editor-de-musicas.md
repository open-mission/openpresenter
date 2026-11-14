# Spec — Refatoração do Módulo 7 (Editor de Músicas e Repertório)

## 1. Intenção

Elevar o **Módulo 7 (Editor de Músicas — `SongEditorModal.svelte`)** ao padrão de qualidade e consistência dos módulos já refatorados (`AdminView`, `BottomPanel`, `SettingsModal`, `RemoteView`, `TeamsModal`, `CommandPalette`), conforme estabelecido em [`docs/architecture/frontend-standards.md`](../architecture/frontend-standards.md):

1. **Eliminar Diálogo Nativo:** Substituir a chamada bloqueante `confirm('Existem alterações não salvas...')` por diálogo acessível com `role="alertdialog"`, armadilha de `Tab`, `Escape` e retorno de foco.
2. **Tema Dual Real:** Substituir todas as cores e superfícies fixas (`neutral-950/900/800`, `text-white`, `text-neutral-100`) por tokens semânticos (`bg-background`, `bg-card`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`), garantindo contraste WCAG AA tanto no tema escuro quanto no tema claro real.
3. **Modularização do Monólito:** Decompor `SongEditorModal.svelte` (1.362 linhas) em subcomponentes coesos em `frontend/src/lib/components/song-editor/`:
   - `SongEditorHeader.svelte`: Título do modal, atalhos de salvar, botões de maximizar/restaurar e fechar.
   - `SongEditorMetadata.svelte`: Campos de título, artista/referência, compositor, tom musical, BPM, tags e áudio/playback vinculado com player de pré-escuta.
   - `SongEditorToolbar.svelte`: Barra de ferramentas de formatação (negrito, itálico, sublinhado, destaque, alinhamentos H/V, quebra em 2 linhas, inserção de tags `## Verso`, `## Coro`).
   - `SongEditorLyrics.svelte`: Editor de texto com textarea, contador de linhas/caracteres e atalhos.
   - `SongEditorPreview.svelte`: Painel de visualização em tempo real dos slides 16:9 gerados pelo parser, com cards clicáveis, badges de seção coloridas e alinhamento sincronizado.
   - `SongEditorConfirmDiscard.svelte`: Diálogo acessível de confirmação de descarte de alterações.
4. **Tipografia e Alvos Táteis:** Garantir texto funcional `>= 11px` (ajustando `<kbd>` que estavam em `text-[10px]`) e ampliar botões de alinhamento e tags para área de clique `>= 28×28px`.
5. **Acessibilidade e Foco:** Gestão de foco ao abrir (foco no input de título), armadilha de `Tab` no modal e subdiálogos, pilha de `Escape` (confirmação -> modal) e atalho `⌘S` / `Ctrl+S` sem conflito.

## 2. Critérios de Aceite

1. `npm run check` aprovado com 0 erros e 0 warnings no workspace.
2. `npm run build` concluído com código 0.
3. `npx impeccable detect --json` zerado (`[]`).
4. Zero diálogos nativos `confirm()` no módulo.
5. Modo Claro e Modo Escuro validados visualmente com contraste WCAG AA.
6. Responsividade em `1440×900` e `1024×768` (com rolagem suave nas duas colunas sem esmagamento).
7. Pontuação no Impeccable Critique pós-fix `>= 36 / 40`.

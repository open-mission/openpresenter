# Spec — Maximização vertical da área de edição de letra no Editor de Músicas

## 1. Intenção

Permitir que o operador maximize verticalmente a região de formatação + letra do `SongEditorModal.svelte`, ocultando temporariamente o formulário de metadados. O objetivo é dar mais altura útil ao `textarea` em telas baixas (`1024×768`) e em letras longas, sem perder o acesso rápido aos dados da música.

## 2. Escopo

- `frontend/src/lib/components/song-editor/controller.svelte.ts`: novo estado `isLyricsMaximized` e método `toggleLyricsMaximized()`, com reset ao abrir o editor.
- `frontend/src/lib/components/song-editor/SongEditorToolbar.svelte`: botão acessível de maximizar/restaurar no fim da barra de formatação.
- `frontend/src/lib/components/song-editor/SongEditorMetadata.svelte`: cabeçalho com título do bloco e botão "Recolher".
- `frontend/src/lib/components/SongEditorModal.svelte`: alternância entre formulário completo e barra compacta de sumário (título, artista, tom/BPM) com ação "Editar metadados".
- `frontend/src/lib/components/song-editor/SongEditorLyrics.svelte`: manter o `textarea` flexível (`flex-1 min-h-0`) dentro de um contêiner `overflow-hidden`.

## 3. Decisões

1. **Estado no controlador**: a maximização é estado de UI da sessão de edição, não persistido e não incluído no snapshot de alterações (`isDirty`). Fecha e reabre sempre restaurada.
2. **Dois gatilhos simétricos**: o botão da toolbar alterna o modo, e o botão "Recolher" do cabeçalho de metadados e o botão "Editar metadados" da barra compacta chamam o mesmo `toggleLyricsMaximized()`, evitando caminhos divergentes.
3. **Barra compacta em vez de metadados desmontados sem contexto**: no modo maximizado o operador continua vendo título, artista e tom/BPM, com um único clique para voltar ao formulário.
4. **Sem novos componentes**: a barra compacta é markup local do modal; o cabeçalho de recolhimento pertence ao próprio `SongEditorMetadata`, mantendo a coesão do módulo.
5. **Acessibilidade**: `aria-pressed` no botão de maximização, rótulos explícitos em português e alvos `>= 28px`, com texto funcional `>= 11px`.

## 4. Critérios de Aceite

1. Com o modo maximizado ativo, o formulário de metadados não ocupa espaço e a área de letra preenche a altura livre da coluna esquerda, sem barra de rolagem no contêiner.
2. A barra compacta exibe título, artista (quando houver) e tom/BPM (modo música), além do botão "Editar metadados".
3. Os botões "Maximizar", "Restaurar", "Recolher" e "Editar metadados" alternam o mesmo estado e expõem rótulos/`aria-pressed` corretos.
4. Fechar e reabrir o editor restaura o modo normal.
5. `npm run check` e `npm run build` concluem sem erros; `npx impeccable detect --json` sem novos apontamentos nos arquivos alterados.
6. Layout validado em `1440×900` e `1024×768`, temas `Escuro` e `Claro`.

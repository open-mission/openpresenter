# Plano — Maximização vertical da área de edição de letra

Spec: [`2026-11-14-editor-musicas-maximizar-area-letra.md`](../specs/2026-11-14-editor-musicas-maximizar-area-letra.md)

## Etapa 1 — Estado no controlador

- Adicionar `isLyricsMaximized = $state(false)` e `toggleLyricsMaximized()` em `controller.svelte.ts`.
- Resetar o estado no bloco de abertura de `syncOpen()`.

## Etapa 2 — Toolbar

- Importar `ChevronsDownUp`/`ChevronsUpDown`.
- Adicionar `isLyricsMaximized` e `ontogglelyricsmaximize` às props.
- Inserir botão com `textButtonClass + 'ml-auto'`, `aria-pressed`, `title`/`aria-label` e ícone/rótulo alternando entre "Maximizar" e "Restaurar".

## Etapa 3 — Metadados

- Adicionar `oncollapsemadata` às props.
- Inserir cabeçalho com título por modo e botão "Recolher" (`ChevronUp`) no topo do contêiner.

## Etapa 4 — Modal

- Alternar a classe da coluna esquerda entre `overflow-y-auto` e `overflow-hidden`.
- No modo maximizado, renderizar a barra compacta de sumário com ação "Editar metadados"; no modo normal, renderizar `SongEditorMetadata` com `oncollapsemadata`.
- Encaminhar as novas props para `SongEditorToolbar`.

## Etapa 5 — Letra

- Confirmar `overflow-hidden` no contêiner e `flex-1 min-h-0` no `textarea`.

## Etapa 6 — Verificação

- `cd frontend && npm run check`
- `cd frontend && npm run build`
- `cd frontend && npx impeccable detect --json`
- `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Validação visual em `1440×900` e `1024×768`, temas `Escuro` e `Claro`.

## Documentação

- Linkar spec/plano em `docs/README.md`.
- Registrar a mudança em `CHANGELOG.md` (Unreleased → Adicionado).

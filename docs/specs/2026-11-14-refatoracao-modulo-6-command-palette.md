# Spec — Refatoração do Módulo 6 (Paleta de Comandos ⌘K)

## 1. Intenção

Elevar o **Módulo 6 (Command Palette — Busca Global ⌘K)** ao padrão de qualidade e consistência dos módulos já refatorados (`AdminView`, `BottomPanel`, `SettingsModal`, `RemoteView`, `TeamsModal`), conforme estabelecido em [`docs/architecture/frontend-standards.md`](../architecture/frontend-standards.md):

1. **Tema Dual Real:** Substituir todas as superfícies e tons escuros fixos (`zinc-*`, `bg-black/80`, `text-white`) por tokens semânticos (`bg-background`, `bg-card`, `bg-popover`, `border-border`, `text-foreground`, `text-muted-foreground`), garantindo contraste WCAG AA tanto no tema escuro quanto no claro.
2. **Acessibilidade e Teclado WAI-ARIA:** Eliminar o sequestro forçado da tecla `Tab` para troca de abas (adotando navegação por setas na `tablist` ou atalhos explícitos como `Ctrl+1/2/3`), permitindo que a tecla `Tab` percorra os elementos focáveis normalmente (como `<input>`, `<select>` e botões de ação); implementar semântica de Combobox (`role="combobox"`, `role="listbox"`, `role="option"`, `aria-selected`, `aria-activedescendant`).
3. **Modularização do Monólito:** Decompor `CommandPalette.svelte` (1.311 linhas) em subcomponentes coesos em `frontend/src/lib/components/command-palette/` (`CommandPaletteHeader`, `CommandPaletteTabs`, `CommandPaletteShortcuts`, `LocalSongsResults`, `BibleSearchResults`, `OnlineLyricsResults`).
4. **Resiliência e Cancelamento:** Adotar `AbortController` em buscas assíncronas (Bíblia e Provedores Online) para evitar race conditions em digitação rápida com debounce.
5. **Tipografia e Alvos Táteis:** Garantir piso funcional de 11px em todas as dicas e atalhos `<kbd>`, eliminar classes CSS inexistentes (`py-0.2`) e garantir alvos interativos `>= 24×24px`.

## 2. Contexto e Diagnóstico do Impeccable Critique

A auditoria inicial do Impeccable atribuiu nota **20 / 40 (Requer Refatoração Estrutural)** com `P0: 2`, `P1: 3`:
- **Tema:** Travamento total em `zinc-950/900/800`, gerando quebra visual em Modo Claro.
- **Teclado:** `e.preventDefault()` na tecla `Tab` quebra a navegação por acessibilidade.
- **Monólito:** 1.311 linhas acumulando 3 domínios (músicas locais FTS5, consulta e parser bíblico, busca Holyrics/Vagalume).
- **Tipografia:** Tags `<kbd>` com `text-[9px]` e `text-[10px]`, além de classe inválida `py-0.2`.

## 3. Escopo da Refatoração

### 3.1 Novos Componentes em `frontend/src/lib/components/command-palette/`

| Componente | Responsabilidade |
| --- | --- |
| `CommandPaletteHeader.svelte` | Input de busca com ícone, spinner reativo, botão de limpar e atalho `Esc` visível. |
| `CommandPaletteTabs.svelte` | Barra de abas com `role="tablist"`, navegação por setas/atalhos, chips de contagem e badges de destaque. |
| `LocalSongsResults.svelte` | Listagem de músicas locais com destaque para correspondência de título/letra, ações rápidas (criar nova, Harpa, etc.). |
| `BibleSearchResults.svelte` | Seletor de versão da Bíblia (`<select>` acessível), card de passagem completa e lista de versículos individuais. |
| `OnlineLyricsResults.svelte` | Listagem de resultados online (Holyrics/Vagalume) com estado de loading, feedback de erro com `role="alert"` e botão de importação. |
| `CommandPaletteShortcuts.svelte` | Rodapé com resumo acessível de atalhos (`↵ Projetar`, `⇧↵ Setlist`, `Esc Fechar`) com texto `>= 11px`. |

### 3.2 Orquestrador (`CommandPalette.svelte`)

- Redução para ≤ 300 linhas de orquestração.
- Gestão centralizada do índice selecionado (`selectedIndex`) e despacho de ações (`Enter`, `Shift+Enter`).
- Ciclo de foco preservado: ao abrir, foco automático no input; ao fechar, restauração de foco ao gatilho.

## 4. Critérios de Aceite

1. `npm run check` aprovado com 0 erros e 0 warnings no frontend.
2. `npm run build` concluído com sucesso.
3. `npx impeccable detect --json` zerado (`[]`).
4. Modo Claro e Modo Escuro validados visualmente com tokens semânticos e contraste WCAG AA.
5. Teclado funcional: setas para navegar resultados, `Enter` para projetar, `Shift+Enter` para adicionar à Setlist, `Esc` para fechar e `Tab` livre para percorrer controles sem bloqueio.
6. Texto `>= 11px` e alvos `>= 24×24px`.
7. Pontuação no Impeccable Critique pós-fix `>= 36 / 40`.

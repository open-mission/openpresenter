# Plano — Setlist em Fluxo Contínuo (Estilo ProPresenter) e Customização de Cores e Fundo da Agenda

**Data:** 2026-11-01  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-01-setlist-propresenter-flow-e-agenda-cores-bg.md`](../specs/2026-11-01-setlist-propresenter-flow-e-agenda-cores-bg.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Atualização dos Tipos e Configurações da Agenda
- [x] Em [`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts):
  - [x] Adicionar `schedule_day_color?: string;`, `schedule_time_color?: string;` e `schedule_badge_bg?: boolean;` a `PublicDisplaySettings`.
  - [x] Adicionar `stage_schedule_day_color?: string;`, `stage_schedule_time_color?: string;` e `stage_schedule_badge_bg?: boolean;` a `StageDisplaySettings`.
- [x] Em [`frontend/src/lib/displaySettings.ts`](../../frontend/src/lib/displaySettings.ts):
  - [x] Definir padrões em `DEFAULT_PUBLIC_SETTINGS` e `DEFAULT_STAGE_SETTINGS`.

### Etapa 2: Aplicação das Cores e Fundo na Renderização da Agenda
- [x] Em [`frontend/src/lib/views/PublicView.svelte`](../../frontend/src/lib/views/PublicView.svelte) e [`frontend/src/lib/components/PublicPreviewBox.svelte`](../../frontend/src/lib/components/PublicPreviewBox.svelte):
  - [x] Aplicar `schedule_day_color`, `schedule_time_color` e condicional `schedule_badge_bg !== false`.
- [x] Em [`frontend/src/lib/views/StageView.svelte`](../../frontend/src/lib/views/StageView.svelte) e [`frontend/src/lib/components/StagePreviewBox.svelte`](../../frontend/src/lib/components/StagePreviewBox.svelte):
  - [x] Aplicar cores e fundo nos badges da agenda do palco.
- [x] Em [`frontend/src/lib/components/SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - [x] Adicionar seletores de cor e toggle de fundo dos badges na aba Tela Pública > Agenda e Monitor de Palco > Agenda.

### Etapa 3: Implementação do Modo Fluxo Contínuo (Estilo ProPresenter) no AdminView
- [x] Em [`frontend/src/lib/views/AdminView.svelte`](../../frontend/src/lib/views/AdminView.svelte):
  - [x] Adicionar estado de modo de visualização: `setlistViewMode: 'item' | 'flow'` com persistência em `localStorage`.
  - [x] Adicionar seletor visual na barra de ferramentas do painel central (botões alternadores com ícones de Modo Focado vs Modo Fluxo Contínuo ProPresenter).
  - [x] Criar derivação estruturada `setlistGroups` que agrupa os itens da setlist pelas seções (`kind === 'header'`).
  - [x] Criar estados reativos de colapso (`collapsedSectionIds`, `collapsedItemIds`) e de conclusão (`completedSectionIds`, `completedItemIds`).
  - [x] Renderizar no Modo Fluxo Contínuo:
    - [x] Barra estilizada de cada seção com cor diferenciada, título, contagem de mídias/slides, botões de colapso, marcação de concluído e botão "Apresentar Seção".
    - [x] Seção colapsada ou marcada como concluída exibida em formato compacto sutil.
    - [x] Para cada item não colapsado dentro da seção, renderizar cabeçalho do item e grade de slides 16:9.
    - [x] Suporte a clique para focar e duplo clique (ou Enter) para projetar imediatamente qualquer slide de qualquer item.
  - [x] Implementar a função `presentEntireSection(group)`:
    - [x] Concatena todos os slides de todos os itens da seção em uma super-mídia única e dispara `quickPresentMedia`.
    - [x] Permite avançar e retroceder ininterruptamente por todos os louvores ou imagens daquela seção.

### Etapa 4: Validação dos Portões de Qualidade e Documentação
- [x] Compilar frontend: `cd frontend && bun run build`.
- [x] Executar linters e testes: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- [x] Atualizar [`docs/README.md`](../README.md), [`CHANGELOG.md`](../../CHANGELOG.md) e [`TODO.md`](../../TODO.md).

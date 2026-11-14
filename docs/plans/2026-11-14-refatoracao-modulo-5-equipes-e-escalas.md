# Plano — Refatoração do Módulo 5 (Equipes & Escalas)

**Data:** 2026-11-14
**Spec:** [`docs/specs/2026-11-14-refatoracao-modulo-5-equipes-e-escalas.md`](../specs/2026-11-14-refatoracao-modulo-5-equipes-e-escalas.md)

---

## Etapa 1 — Camada de dados (`frontend/src/lib/teams.ts`)

- [x] Criar `ensureOk(res, fallbackMessage)` privado, lendo texto e extraindo mensagem de JSON.
- [x] Aplicar em `deleteDepartment`, `deleteMember`, `activateRoster`, `deleteRoster`, `removeRosterItem`, `updateRosterItemStatus`.
- [x] Adicionar `signal?: AbortSignal` em `fetchDepartments`, `fetchMembers`, `fetchRosters`, `fetchRoster`.
- [x] Reutilizar `ensureOk` em `saveDepartment`, `saveMember`, `saveRoster`, `addRosterItem`.

## Etapa 2 — Helpers compartilhados (`teams/teamConstants.ts`)

- [x] `PRESET_COLORS` (8 cores) e `PRESET_ICONS` (10 ícones com rótulos pt-BR).
- [x] `getDeptIconComponent`, `getInitials`, `getNextRosterStatus`, `getRosterStatusMeta`.

## Etapa 3 — Primitivos de diálogo

- [x] `TeamsSubDialog.svelte`: backdrop `z-[45]`, `role="dialog"`, `aria-modal`, `aria-labelledby`, armadilha de `Tab`,
      foco inicial no primeiro campo e restauração de foco.
- [x] Reaproveitar `bottom-panel/ConfirmModal.svelte` (z-50) para confirmações destrutivas.

## Etapa 4 — Abas

- [x] `TeamsRosterTab.svelte`: seletor com `aria-label`, ações (nova/editar/escalar/ativar/copiar/excluir),
      resumo de presenças, toggle Flow/Grade com `aria-pressed`, grade de cartões, banner de erro com nova tentativa.
- [x] `TeamsMembersTab.svelte`: busca com limpar, filtro `aria-label`, cards com chips de departamento acessíveis.
- [x] `TeamsDepartmentsTab.svelte`: grid responsivo, contagem real de membros, ações acessíveis.

## Etapa 5 — Formulários

- [x] `RosterFormModal.svelte` (criar/editar, validação e `isSaving`).
- [x] `AssignMemberModal.svelte` (`lead`/`assistant`/`backup`, aviso de duplicidade).
- [x] `MemberFormModal.svelte` (dados, cargo, departamentos, controle remoto, ativo, PIN).
- [x] `DeptFormModal.svelte` (sem `is_active`, cores + 10 ícones).

## Etapa 6 — Orquestrador (`TeamsModal.svelte`)

- [x] Reduzir para ≤ 350 linhas com estado compartilhado e callbacks para as abas.
- [x] Carregar na borda `open: false → true` com `AbortController` + sequência.
- [x] Pilha de `Escape`: confirmação → submodal do topo → fechar modal.
- [x] Abas `role="tablist"` com navegação por setas e painéis preservados (`class:hidden` + `visitedTabs`).
- [x] Reset completo e restauração de foco ao fechar.

## Etapa 7 — Svelte Flow

- [x] `RosterFlowCanvas.svelte`: tema reativo, `patternColor`, `colorMode`, controles/minimapa com tokens.
- [x] `ServiceNode`, `DepartmentNode`, `MemberNode`: tokens semânticos, sem pulso decorativo, 10 ícones, alvos ≥ 24px.

## Etapa 8 — Integração e documentação

- [x] `AdminView.svelte`: `if (isTeamsModalOpen) return;` após a exceção de configurações.
- [x] Atualizar `docs/README.md`, `CHANGELOG.md` e este plano.

## Etapa 9 — Portões de qualidade

- [x] `cd frontend && npm run check` (0 erros nos arquivos do módulo; 17 erros pré-existentes em `ShareOutputsModal.svelte`, `AdminView.svelte` e `ScheduleEditorModal.svelte`).
- [x] `cd frontend && npm run build`.
- [x] `impeccable detect --json frontend/src/lib/components/TeamsModal.svelte frontend/src/lib/components/teams/ frontend/src/lib/teams.ts` → `[]`.
- [x] Inspeção manual em `1440×900` e `1024×768`, temas Claro e Escuro (backend headless real com dados de exemplo; tema trocado via Configurações → Aparência; tablist com setas/`Home`/`End`, pilha de `Escape`, retorno de foco e ausência de overflow do diálogo verificados via CDP).

## Etapa 10 — Rodada cirúrgica de acessibilidade e consistência

- [x] Criar `teams/focusTrap.ts` com `getFocusableElements`, `focusInitialElement` e `trapTabKey` documentados (JSDoc).
- [x] `TeamsModal.svelte`: `bind:this={dialogEl}`, foco inicial na aba ativa via `queueMicrotask` e armadilha de `Tab` mantendo a pilha de `Escape`.
- [x] `TeamsModal.svelte`: banner `role="alert"` com **Tentar novamente** logo abaixo do `TeamsHeader`, visível em todas as abas; arquivo com 348 linhas.
- [x] `TeamsRosterTab.svelte`: remover props `errorMessage`/`onRetry` e o banner duplicado.
- [x] `TeamsSubDialog.svelte`: usar `$props.id()` para `titleId`/`descId` e o helper compartilhado de foco/`Tab`.
- [x] `MemberFormModal.svelte`: `canControlRemote` inicia em `false`; labels de departamento com `min-h-[28px]` e checkbox `w-4 h-4 shrink-0`.
- [x] `cd frontend && npm run check` (0 erros nos arquivos do Módulo 5; 17 erros pré-existentes fora do escopo).
- [x] `cd frontend && npm run build`.
- [x] `impeccable detect --json` nos arquivos do módulo → `[]`.
- [x] Inspeção manual em `1440×900` e `1024×768`, temas Claro e Escuro (offline e backend headless isolado): foco inicial na aba ativa, ciclo de `Tab` contido no diálogo, erro offline nas três abas com `ensureOk` saneando respostas HTML/longas (`max 220` chars) e banner com `max-h-24` + `[overflow-wrap:anywhere]`, pilha de `Escape` com retorno de foco ao botão `Equipe`, `canControlRemote` em `false` e labels de departamento `>= 28px`.

## Etapa 11 — Layout flex do estado vazio em `1024×768` com banner offline

- [x] `TeamsRosterTab.svelte`: contêiner principal com `overflow-y-auto` quando `!roster` e `overflow-hidden` quando há escala (SvelteFlow/Grade intactos).
- [x] `TeamsRosterTab.svelte`: estado vazio com `min-h-full` (sem `h-full`) e `p-4 sm:p-8`; ícone e botão **Criar Primeira Escala** com `shrink-0` e `min-h-[36px]`.
- [x] `TeamsMembersTab.svelte`: estado vazio com `min-h-full` e `p-4 sm:p-8`; contêiner do ícone com `shrink-0`.
- [x] `TeamsDepartmentsTab.svelte`: estado vazio com `min-h-full` e `p-4 sm:p-8`; contêiner do ícone com `shrink-0`.
- [x] `cd frontend && npm run check` (0 erros nos arquivos do Módulo 5; 17 erros pré-existentes fora do escopo).
- [x] `cd frontend && npm run build`.
- [x] `impeccable detect --json src/lib/components/TeamsModal.svelte src/lib/components/teams/ src/lib/teams.ts` → `[]`.
- [x] Inspeção em browser do estado vazio em `1024×768` e `1440×900`, temas Claro e Escuro, com banner offline real (backend derrubado): Escala, Membros e Departamentos centralizados, ícone `56×56` e botão **Criar Primeira Escala** em `36px`; em `1024×420` o painel passa a rolar (`scrollHeight 280 > clientHeight 198`) sem esmagar ícone/botão.

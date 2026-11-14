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

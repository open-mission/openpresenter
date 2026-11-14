# Spec — Refatoração do Módulo 5 (Equipes & Escalas)

## 1. Intenção

Elevar o **Módulo 5 (Equipe de Mídia & Escalas)** ao mesmo padrão dos módulos já refatorados
(`AdminView`, `BottomPanel`, `SettingsModal`, `RemoteView`), conforme
[`docs/architecture/frontend-standards.md`](../architecture/frontend-standards.md):

1. Eliminar as regressões de tema, acessibilidade e tipografia do `TeamsModal.svelte` monolítico (1.788 linhas).
2. Zerar `alert()`/`confirm()` nativos e erros de TypeScript do módulo.
3. Decompor o modal em módulos coesos por domínio, com estado compartilhado explícito.

## 2. Contexto e Problema

O `TeamsModal.svelte` foi construído antes da consolidação dos padrões de interface e acumulou:

- **Tema:** superfícies fixas (`bg-neutral-900`, `bg-neutral-950`, `text-white`, `purple-600`) que quebram no Tema Claro;
  o `RosterFlowCanvas` usava `Background color="#262626"` e controles escuros fixos.
- **Acessibilidade:** modais sem `aria-labelledby`/`aria-modal`, sem armadilha de `Tab` nem retorno de foco;
  abas sem `role="tablist"`/`role="tab"`; botões icônicos sem `aria-label`; alvos de `24×24px` violados (`w-6 h-6`);
  texto de `8–10px` em badges e nós do fluxo.
- **Diálogos nativos:** `alert()` em todas as falhas de rede e `confirm()` em todas as exclusões, o que trava a thread
  e quebra leitores de tela; falhas HTTP silenciadas (`return res.ok`, `catch` sem feedback).
- **Escape:** três handlers independentes (modal principal e submodais) fechavam a pilha inteira de uma vez e vazavam
  para o `AdminView`.
- **Rede:** carregamento disparado por `$effect` sem controle de concorrência/abort, sem `attempted` e sem botão de nova tentativa.
- **Modelo de funções desatualizado:** a UI enviava `role: 'helper'` enquanto o backend usa `'lead' | 'assistant' | 'backup'`;
  o payload de departamento enviava `is_active`, campo inexistente em `UpsertTeamDepartment`.

## 3. Escopo

### 3.1 Camada de dados (`frontend/src/lib/teams.ts`)

- Helper interno `ensureOk(res, fallbackMessage)`: lê o corpo em texto (extraindo `error`/`message` de JSON quando houver),
  lança `Error(...)` em HTTP de erro.
- `deleteDepartment`, `deleteMember`, `activateRoster`, `deleteRoster`, `removeRosterItem`, `updateRosterItemStatus`
  passam a usar `ensureOk`, mantendo o contrato `Promise<boolean>` (`true` no sucesso, `Error` em falha HTTP).
- `signal?: AbortSignal` opcional nos getters `fetchDepartments`, `fetchMembers`, `fetchRosters`, `fetchRoster`.

### 3.2 Novos módulos em `frontend/src/lib/components/teams/`

| Arquivo | Responsabilidade |
| --- | --- |
| `teamConstants.ts` | `PRESET_COLORS`, `PRESET_ICONS` (10 ícones), `getDeptIconComponent`, `getDeptIconLabel`, `getContrastTextColor`, `getInitials`, `getNextRosterStatus`, `getRosterStatusMeta`. |
| `TeamsSubDialog.svelte` | Wrapper acessível de submodal (`role="dialog"`, `aria-modal`, `aria-labelledby`, armadilha de `Tab`, foco inicial e restauração). |
| `TeamsRosterTab.svelte` | Aba Escala: seletor, ações, badge de escala ativa, resumo confirmados/pendentes/recusados, copiar WhatsApp, toggle Flow/Grade, exclusão, erro inline. |
| `TeamsMembersTab.svelte` | Aba Membros: busca com limpar, filtro por departamento, cards responsivos, `departmentMap` via `Map`. |
| `TeamsDepartmentsTab.svelte` | Aba Departamentos: grid responsivo com contagem real de membros. |
| `RosterFormModal.svelte` | Criar/editar escala via `saveRoster(id, payload)`, bloqueio de vazio e de duplo submit. |
| `AssignMemberModal.svelte` | Escalar voluntário com `'lead' | 'assistant' | 'backup'` (Titular/Auxiliar/Apoio), observações e aviso de vínculo duplicado. |
| `MemberFormModal.svelte` | Criar/editar voluntário (dados, cargo, departamentos, controle remoto, ativo, PIN numérico). |
| `DeptFormModal.svelte` | Criar/editar departamento sem `is_active`, com paleta de cores e 10 ícones. |

### 3.3 Orquestrador (`TeamsModal.svelte`, alvo ≤ 350 linhas)

- Carrega dados na borda `open: false → true` (sem depender de `selectedRosterId` em `$effect`), com `AbortController`
  e sequência numérica para descartar respostas antigas.
- Ao fechar, reseta submodais/confirmações e restaura o foco ao elemento anterior.
- Pilha de `Escape` única: fecha primeiro a `ConfirmModal`, depois o submodal do topo, e só então o modal principal.
- Abas WAI-ARIA com `ArrowLeft`/`ArrowRight`/`Home`/`End` e painéis mantidos vivos após a primeira visita.
- `ConfirmModal` (z-50) renderizado depois do modal principal (backdrop z-40) e dos submodais (backdrop z-[45]).

### 3.4 Nós do Svelte Flow

- `ServiceNode`, `DepartmentNode`, `MemberNode` com `bg-card/95 border-border text-foreground`, status legíveis nos dois
  temas, sem `animate-pulse`, 10 ícones via `getDeptIconComponent`, alvos ≥ 24px e texto ≥ 11px.
- `RosterFlowCanvas` reage a `$themeStore` + `matchMedia('(prefers-color-scheme: dark)')`, aplica
  `patternColor={isLightMode ? '#d4d4d8' : '#262626'}`, `colorMode` no `<SvelteFlow>` e estiliza controles/minimapa com tokens CSS.
- Avatares e tiles coloridos escolhem texto preto ou branco por luminância (`getContrastTextColor`), garantindo
  contraste mínimo de 4,5:1 sobre qualquer cor de preset; ícones desconhecidos/herdados são resolvidos por apelido
  (`monitor`, `music`, `audio-lines`) e exibem rótulo em pt-BR.

### 3.5 Integração global

- `AdminView.svelte`: adicionar `if (isTeamsModalOpen) return;` imediatamente após `if (isSettingsOpen) return;`
  no tratamento global de `Escape`, para o módulo gerenciar sua própria pilha.

## 4. Decisões

1. **Um único dono do `Escape`:** o orquestrador centraliza a pilha; submodais não escutam `Escape` para evitar
   fechamento em cascata. A `ConfirmModal` mantém seu próprio handler (já para propagação) e continua no topo.
2. **Estado de formulário vive no próprio modal de formulário:** o orquestrador apenas informa `editing*` e recebe
   `onSaved`, evitando dezenas de variáveis no arquivo principal.
3. **`role_notes` e `status` preservados:** editar escala não altera itens; escalar define `status: 'pending'`.
4. **Sem `is_active` em `UpsertTeamDepartment`:** o backend não possui o campo; a versão anterior gerava erro de tipo.
5. **Filtros de abas preservados:** painéis ficam montados (`class:hidden`) após a primeira visita.

## 5. Critérios de Aceite

1. `cd frontend && npm run check` sem erros nos arquivos do Módulo 5.
2. `cd frontend && npm run build` conclui com código 0.
3. `impeccable detect --json` nos arquivos do módulo retorna `[]`.
4. Zero `alert()`/`confirm()` nativos, zero `neutral-*`/`text-white`/`purple-*` fixos, zero `text-[8px]`/`[9px]`/`[10px]`
   e zero alvo interativo abaixo de `24×24px` no módulo.
5. `Escape` fecha apenas a camada do topo; foco retorna ao gatilho ao fechar.
6. Falhas HTTP exibem banner `role="alert"` com **Tentar novamente** e/ou toast de erro.
7. Layout sem overflow horizontal em `1024×768` e `1440×900`, nos temas Claro e Escuro.

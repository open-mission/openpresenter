# Especificação — Refatoração do Módulo 8 (Cronogramas & Agendas de Culto)

## 1. Contexto e Motivação

O `ScheduleEditorModal.svelte` gerencia a criação e edição da programação semanal e eventos especiais da igreja. Cada evento vira um slide conectado à tela pública e ao monitor de retorno de palco (`/stage`).

Atualmente o componente possui 653 linhas em um arquivo monolítico com pontuação **20 / 40** no Impeccable Critique (`Requer Refatoração Estrutural`).

### Diagnóstico de Problemas (Baseline):
- **[P0] Tema Escuro Rígido (`neutral-*`, `zinc-*`, `bg-black/80`)**: Incompatível com o Modo Claro (`data-theme="light"`).
- **[P0] Uso de `confirm()` Nativo**: Bloqueio síncrono da thread e ausência de acessibilidade na confirmação de descarte de alterações.
- **[P1] Falhas Graves de WAI-ARIA**: Cards de eventos clicáveis definidos com `role="presentation"`, ausência de `aria-labelledby`/`aria-describedby` no modal, botões de ação e reordenação sem rótulos acessíveis contextuais.
- **[P1] Violação do Piso Tipográfico**: Textos em `9px` e `10px` em badges, números de ordenação, campos de formulário e preview de palco.
- **[P1] Monólito sem Modularização**: Lógica de formulário, atalhos, reordenação de eventos, preview de tela pública e palco condensados em um único arquivo.
- **[P2] Alvos Interativos Menores que 24×24px**: Botões de subir, descer e excluir evento com área reduzida.
- **[P2] Atalhos e Plataforma**: Referência apenas a `Ctrl+Enter` sem suporte textual a `⌘Enter` no macOS.

---

## 2. Metas de Design e Arquitetura

1. **Decomposição Modular**:
   Criar a pasta `frontend/src/lib/components/schedule-editor/` contendo:
   - `ScheduleEditorHeader.svelte`: cabeçalho com ícone, título, contagem de eventos, maximizar e fechar com `focusTrap`.
   - `ScheduleEditorMetadata.svelte`: título da programação e categoria/tag.
   - `ScheduleEditorPresets.svelte`: atalhos rápidos (+ Domingo, + Quarta, + Sábado, etc.) com alvos táteis confortáveis.
   - `ScheduleEditorEventsList.svelte`: orquestrador da lista com cabeçalho de seção e botão "+ Novo Evento".
   - `ScheduleEditorEventCard.svelte`: card de edição do evento com WAI-ARIA correto (`role="group"` / `aria-labelledby`), inputs com labels `>= 11px`, botões de reordenação com `aria-label` descritivo e alvos `>= 28×28px`.
   - `ScheduleEditorPreview.svelte`: preview 16:9 da projeção pública e simulação do monitor de retorno de palco (`/stage`), com tipografia no piso mínimo `>= 11px`.
   - `ScheduleEditorConfirmDiscard.svelte`: diálogo acessível de descarte com `role="alertdialog"`, trap de foco e retorno ao botão gatilho.
   - `controller.svelte.ts`: classe de controle encapsulando estado reativo, snapshot de dirty state, validações, reordenação e conversão para `MediaItem`/`Slide`.
   - `types.ts` e `defaults.ts`: tipos e valores padrão da agenda.

2. **Adoção Estrita de Tokens Semânticos**:
   - Substituição de `bg-neutral-900`, `bg-neutral-950`, `border-neutral-800`, `text-white` por `bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`.
   - Badges e acentos de agenda usando `text-amber-700 dark:text-amber-300`, `bg-amber-500/10`, `border-amber-600/30`.

3. **Acessibilidade e Navegação por Teclado**:
   - Trap de foco com `focusTrap.ts` compartilhado.
   - Delegação correta de `Escape` (descarte → modal).
   - Atalhos `⌘Enter` / `Ctrl+Enter` para salvar diretamente.
   - Foco inicial colocado no campo de título da programação ao abrir o modal.

---

## 3. Critérios de Aceite e Portões

1. `ScheduleEditorModal.svelte` orquestrador limpo (< 180 linhas).
2. Pontuação no Impeccable Critique evoluindo de **20/40** para `>= 36/40` (`Aprovado para Produção`).
3. Zero erros e zero warnings no `npm run check`.
4. Build de produção `npm run build` bem-sucedido.
5. Zero achados no `npx impeccable detect frontend/src --json`.
6. Validação ao vivo em `1440×900` e `1024×768` nos modos Claro e Escuro.

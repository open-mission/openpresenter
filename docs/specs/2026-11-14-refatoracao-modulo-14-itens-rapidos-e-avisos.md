# Especificação — Refatoração do Módulo 14 (Itens Rápidos, Divisores & Avisos)

## 1. Contexto e Motivação

O `QuickItemModal.svelte` é o modal responsável pela criação e edição ágil de dois tipos fundamentais de itens na programação e no catálogo:
- **Divisor de Seção (`mode === 'header'`)**: Separador visual dentro da setlist para organizar os blocos do culto (ex.: "Momento de Louvor", "Dízimos e Ofertas", "Palavra Pastoral"). Não gera slides de projeção.
- **Texto Livre / Avisos (`mode === 'announcement'`)**: Criação direta de slides de avisos, comunicados e leituras, onde cada bloco de texto separado por uma linha em branco (`\n\n`) é automaticamente convertido em um slide projetável.

Atualmente o componente possui 225 linhas em um arquivo monolítico com pontuação **20 / 40** no Impeccable Critique (`Requer Refatoração Estrutural`).

### Diagnóstico de Problemas (Baseline):
- **[P0] Uso de `confirm()` Nativo Bloqueante**: Confirmação de descarte de alterações via `window.confirm`, travando a thread da interface e violando as diretrizes de acessibilidade e experiência do usuário.
- **[P0] Tema Escuro Rígido (`neutral-*`, `zinc-*`, `bg-black/80`, `text-white`)**: Classes escuras fixas (`bg-neutral-900`, `border-neutral-800`, `bg-neutral-950`, `text-white`, `text-neutral-400`, `bg-black/80`) que quebram o tema Claro (`data-theme="light"`).
- **[P1] Falhas Graves de WAI-ARIA**: O modal carece de `aria-modal="true"`, `aria-labelledby` e `aria-describedby`; ausência de aprisionamento de foco (`focusTrap`); ausência de restauração do foco ao fechar; uso de `autofocus` com supressão de aviso do linter; botão de fechar sem `aria-label` descritivo.
- **[P1] Quebra do Piso Tipográfico**: A dica de auxílio de divisão de slides utiliza `text-[10px]`, violando o piso mínimo de 11px.
- **[P1] Monólito sem Modularização**: Lógica de dirty checking, confirmação de descarte, parsing de slides, teclado e formulário concentrados em um único arquivo.
- **[P2] Alvos Interativos Reduzidos**: Botões de fechar e maximizar com dimensões reduzidas sem alvos confortáveis `>= 32px`.

---

## 2. Metas de Design e Arquitetura

1. **Decomposição Modular**:
   Criar a pasta `frontend/src/lib/components/quick-item/` contendo:
   - `types.ts`: interfaces de props, callbacks, modos (`header` vs `announcement`) e dados do formulário.
   - `defaults.ts`: títulos, descrições, rótulos e placeholders contextuais para cada modo.
   - `controller.svelte.ts`: classe reativa com runes Svelte 5 (`$state`, `$derived`), snapshot de dirty check, gerenciamento da pilha de descarte (`isDiscardOpen`), parsing de slides por quebra dupla, atalhos de teclado (`Escape`, `⌘Enter` / `Ctrl+Enter`), armadilha de foco e restauração de foco ao fechar.
   - `QuickItemHeader.svelte`: cabeçalho acessível com ícone semântico dinâmico (`Bookmark` para seção, `FileText` para aviso), título, descrição contextual, botões Maximizar e Fechar.
   - `QuickItemForm.svelte`: campo de título/nome de seção e textarea de conteúdo de slides com contador em tempo real e dica acessível `>= 11px`.
   - `QuickItemFooter.svelte`: botões Cancelar e Salvar com destaque visual em tokens semânticos e indicação de atalho `⌘Enter`.
   - `QuickItemConfirmDiscard.svelte`: diálogo acessível de descarte com `role="alertdialog"`, `aria-modal="true"`, foco preso e retorno seguro ao botão gatilho.

2. **Adoção Estrita de Tokens Semânticos**:
   - Eliminação total de `neutral-*`, `zinc-*`, `bg-black/80` e `text-white`.
   - Adoção de `bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`, `bg-primary`, `text-primary-foreground` e acentos semânticos esmeralda/verde.
   - Contraste WCAG AA assegurado nos temas Claro e Escuro.

3. **Acessibilidade WAI-ARIA e Teclado**:
   - Painel do diálogo com `role="dialog"`, `aria-modal="true"`, `aria-labelledby="quick-item-title"`, `aria-describedby="quick-item-desc"`, `tabindex="-1"`.
   - Armadilha de foco via `focusTrap.ts`.
   - Foco inicial seguro no campo de título ao abrir.
   - Fechamento com tecla `Escape` respeitando a confirmação de descarte.
   - Atalhos `⌘Enter` / `Ctrl+Enter` para salvar.
   - Diálogo de descarte com `role="alertdialog"`.

---

## 3. Critérios de Aceite e Portões

1. `QuickItemModal.svelte` orquestrador limpo (< 115 linhas).
2. Pontuação no Impeccable Critique evoluindo de **20/40** para `>= 36/40` (`Aprovado para Produção`).
3. Zero erros e zero warnings no `npm run check`.
4. Build de produção `npm run build` bem-sucedido.
5. Zero achados no `npx impeccable detect frontend/src --json`.
6. 78/78 testes passando no `cargo test`.

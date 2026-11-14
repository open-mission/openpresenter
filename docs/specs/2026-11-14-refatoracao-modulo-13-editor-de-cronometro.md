# Especificação — Refatoração do Módulo 13 (Editor de Cronômetros & Timers)

## 1. Contexto e Motivação

O `TimerEditorModal.svelte` é o modal responsável pela criação e edição de cronômetros e contagens regressivas projetadas na tela pública e no monitor de palco:
- **Contagem Regressiva**: Definição de duração em minutos e segundos com display digital em tempo real.
- **Presets Rápidos**: Botões pré-configurados de 1 min, 2 min, 3 min, 5 min, 10 min, 15 min e 30 min.
- **Mensagem de Apoio / Subtítulo**: Texto complementar exibido logo abaixo dos dígitos na tela pública (ex.: "O culto começará em instantes. Prepare o seu coração!").
- **Overtime**: Configuração de tempo extra com contagem negativa após o zero.
- **Ações de Salvamento**: Salvar na biblioteca de mídias ou salvar e já adicionar diretamente à setlist ativa.

Atualmente o componente possui 303 linhas em um arquivo monolítico com pontuação **20 / 40** no Impeccable Critique (`Requer Refatoração Estrutural`).

### Diagnóstico de Problemas (Baseline):
- **[P0] Tema Escuro Rígido (`neutral-*`, `zinc-*`, `bg-black/80`, `text-white`)**: Uso massivo de classes escuras hardcoded (`bg-neutral-900`, `border-neutral-800`, `bg-neutral-950`, `text-white`, `text-neutral-400`, `bg-neutral-800`), quebrando a adaptação ao Modo Claro (`data-theme="light"`).
- **[P1] Falhas Graves de WAI-ARIA**: Ausência de `role="dialog"`, `aria-modal="true"`, `aria-labelledby` e `aria-describedby` no painel; ausência de aprisionamento de foco (`focusTrap`); ausência de foco inicial automático no título; ausência de listener para tecla `Escape`; presets sem semântica acessível (`role="radiogroup"` / `role="radio"`); switch de overtime sem `role="switch"` e sem estado anunciado para leitores de tela.
- **[P1] Quebra do Piso Tipográfico**: Badge do cabeçalho usa `text-[10px]`, violando o piso mínimo `>= 11px`.
- **[P1] Monólito sem Modularização**: Lógica de cálculo de duração, conversão de tempo, presets, sincronização de estado na abertura, montagem de `MediaItem`/`Slide` e atalhos em arquivo único.
- **[P2] Alvos Interativos Reduzidos e Atalhos**: Presets de tempo com área de clique reduzida (`px-2.5 py-1 text-xs`), ausência de atalho multiplataforma `⌘Enter` / `Ctrl+Enter` para salvar e inputs de tempo sem botões de ajuste rápido.

---

## 2. Metas de Design e Arquitetura

1. **Decomposição Modular**:
   Criar a pasta `frontend/src/lib/components/timer-editor/` contendo:
   - `types.ts`: interfaces de props, callbacks, presets e dados do formulário de timer.
   - `defaults.ts`: presets padrão de duração (1, 2, 3, 5, 10, 15, 30 min), textos e valores iniciais padrão.
   - `controller.svelte.ts`: classe reativa com runes Svelte 5 (`$state`, `$derived`), sincronização de estado com `initialItem`, cálculo de duração total em segundos, formatação `MM:SS`, montagem de `MediaItem`, atalhos de teclado (`Escape`, `⌘Enter` / `Ctrl+Enter`), armadilha de foco e restauração de foco ao fechar.
   - `TimerEditorHeader.svelte`: cabeçalho acessível com ícone semântico de ampulheta, título dinâmico (Novo vs Editar), badge descritiva com piso tipográfico `>= 11px`, botão Maximizar/Restaurar e botão Fechar.
   - `TimerDurationSelector.svelte`: seletor de duração com display digital de alto contraste, presets com semântica de rádio (`role="radiogroup"` / `role="radio"`, `aria-checked`), inputs de minutos e segundos com labels associadas e botões rápidos de ajuste.
   - `TimerSubtitleField.svelte`: campo de texto do subtítulo/mensagem de apoio projetada com texto auxiliar e contadores.
   - `TimerAdvancedOptions.svelte`: controle de Overtime com switch acessível (`role="switch"`, `aria-checked`).
   - `TimerEditorFooter.svelte`: rodapé com botão Cancelar, Salvar na Biblioteca e Salvar e Adicionar à Setlist com destaque visual e dica do atalho `⌘Enter`.

2. **Adoção Estrita de Tokens Semânticos**:
   - Eliminação total de `neutral-*`, `zinc-*`, `bg-black/80` e `text-white`.
   - Adoção de `bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`, `bg-primary`, `text-primary-foreground` e acentos semânticos âmbar/amarelo para contagem regressiva.
   - Contraste WCAG AA assegurado nos temas Claro e Escuro.

3. **Acessibilidade WAI-ARIA e Teclado**:
   - Painel do diálogo com `role="dialog"`, `aria-modal="true"`, `aria-labelledby="timer-editor-title"`, `aria-describedby="timer-editor-desc"`, `tabindex="-1"`.
   - Armadilha de foco via `focusTrap.ts`.
   - Foco inicial seguro no campo de título ao abrir.
   - Fechamento imediato com tecla `Escape`.
   - Atalhos `⌘Enter` / `Ctrl+Enter` para salvar e fechar.
   - Presets com semântica acessível `role="radiogroup"` / `role="radio"`.
   - Switch de overtime com `role="switch"` e `aria-checked`.

---

## 3. Critérios de Aceite e Portões

1. `TimerEditorModal.svelte` orquestrador limpo (< 120 linhas).
2. Pontuação no Impeccable Critique evoluindo de **20/40** para `>= 36/40` (`Aprovado para Produção`).
3. Zero erros e zero warnings no `npm run check`.
4. Build de produção `npm run build` bem-sucedido.
5. Zero achados no `npx impeccable detect frontend/src --json`.
6. 78/78 testes passando no `cargo test`.

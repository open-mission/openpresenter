# Padrões e Regras de Implementação Frontend

Estas diretrizes consolidam as lições verificadas nas auditorias e refatorações do **Módulo 1** (`AdminView.svelte` e componentes centrais) e do **Módulo 2** (`BottomPanel.svelte`, `BiblePanel.svelte`, `bottom-panel/*` e stores de suporte). Devem ser seguidas em toda criação ou refatoração de interface no diretório `frontend/`.

Para a estrutura geral e fluxo de dependências, consulte também a [Visão da arquitetura](overview.md).

---

## 1. Tema Dual (`Escuro` / `Claro`) e Contraste WCAG AA

1. **Tokens semânticos obrigatórios**: usar apenas tokens declarados em [`frontend/src/app.css`](../../frontend/src/app.css) (`bg-background`, `bg-card`, `bg-popover`, `bg-muted`, `bg-secondary`, `text-foreground`, `text-muted-foreground`, `border-border`).
2. **Proibido hardcode de superfície ou cor neutra**: nunca usar `bg-neutral-900`, `bg-neutral-950`, `bg-zinc-*` ou `bg-black` em painéis, cartões, menus de contexto, badges ou rodapés da mesa de operação.
   - *Única exceção*: o canvas 16:9 de saída projetada (`PublicView` / `StageView` / mini-preview de saída), quando o preto representa a saída real do projetor.
3. **Sem classes Tailwind inexistentes**: não escrever utilitários inventados como `hover:bg-neutral-850` ou `py-0.2`; eles são descartados silenciosamente no build.
4. **Alertas e ações destrutivas com contraste real**: não usar `text-destructive` diretamente sobre fundos translúcidos (`bg-destructive/10` ou `bg-red-500/10`), pois o contraste cai abaixo de 4.5:1 em ambos os temas. Adotar `text-red-700 dark:text-red-300 border-red-500/35 bg-red-500/10` (como em [`ConfirmModal.svelte`](../../frontend/src/lib/components/bottom-panel/ConfirmModal.svelte)).
5. **Validação real do Modo Claro**: [`frontend/src/app.css`](../../frontend/src/app.css) define apenas o `:root` escuro; os tokens do Modo Claro são aplicados dinamicamente por `setTheme` em [`frontend/src/lib/settings.ts`](../../frontend/src/lib/settings.ts). Nunca testar Modo Claro alterando classe DOM manualmente — alternar sempre pela UI (`Configurações -> Claro`) ou via `setTheme('light')`.

---

## 2. Ciclo de Vida, Estado Persistente e Rede Offline-First

1. **Preservar estado ao recolher ou alternar abas**: nunca desmontar painéis com estado ativo (player de áudio, capítulo bíblico, filtros, formulários) com `{#if !isCollapsed}` ou `{:else if activeTab === ...}`. Adotar mapa de inicialização sob demanda (`visitedTabs`) combinado com `class:hidden` para manter o DOM vivo sem disparar scans ou fetches no startup (padrão de [`BottomPanel.svelte`](../../frontend/src/lib/components/BottomPanel.svelte)).
2. **Estado global fora da árvore visual**: controles contínuos (como áudio em [`audioPlayer.ts`](../../frontend/src/lib/audioPlayer.ts) ou coleções em [`customItems.ts`](../../frontend/src/lib/customItems.ts)) devem residir em stores dedicadas para não serem interrompidos por troca de aba ou colapso do painel.
3. **Zero loops de retry em `$effect`**: todo carregamento automático disparado por `$effect` (ex.: `!loaded && !loading`) deve verificar também uma flag `attempted` (ex.: `foldersAttempted`) setada antes da requisição. Sem ela, respostas `404`/`500` do backend provocam enxurradas infinitas de requisições; novas tentativas após erro devem ocorrer apenas por ação explícita do operador.
4. **Limpeza simétrica de listeners globais**: listeners em `window` ou `document` registrados dentro de componentes devem retornar função de remoção em `$effect` ou usar `onDestroy`.
5. **Tratamento explícito de erros HTTP**: nunca silenciar falhas com `.catch(() => {})` nem exibir páginas HTML brutas de erro na UI. Usar [`readHttpError`](../../frontend/src/lib/media.ts#L60) e renderizar alerta `role="alert"` com botão de tentar novamente.
6. **Contadores e mídia sob demanda**: inicializar badges de abas não visitadas com `null` (ocultando o contador em vez de mostrar `0` enganoso antes da carga). Em listagens de vídeo, evitar pré-carregamento pesado contínuo; respeitar `@media (prefers-reduced-motion: reduce)` já centralizado em [`frontend/src/app.css`](../../frontend/src/app.css).

---

## 3. Segurança Ao Vivo, Atalhos Globais, Foco e WAI-ARIA

1. **Proteger atalhos globais de mesa ao vivo**: listeners globais de teclado (`Enter`, `Space`, `Escape`, setas) não podem disparar ações de projeção (como `stopPresentation()` ou troca de slide) quando o foco estiver em `<input>`, `<textarea>`, `<select>`, `[contenteditable]`, `<button>`, `<a>`, `[role="button"]` **ou** enquanto existir modal (`[aria-modal="true"]`), menu ou popover aberto.
2. **Proibido aninhar controles interativos**: nunca colocar `<button>`, `<input>` ou `<audio controls>` dentro de outro `<button>` ou `<div role="button">`. Estruturar cartões como `<article>` (ou container neutro) com `<button aria-pressed>` principal para seleção e botões irmãos para ações secundárias.
3. **Proibido `alert()` e `confirm()` nativos**: diálogos nativos travam a thread e quebram o foco/leitor de tela. Usar [`ConfirmModal.svelte`](../../frontend/src/lib/components/bottom-panel/ConfirmModal.svelte) (ou modal equivalente com `role="alertdialog"`/`role="dialog"`, `aria-modal="true"`, armadilha de `Tab`, `Escape` bloqueado durante processamento e retorno de foco ao gatilho).
4. **Overlays, popovers e foco de retorno**: todo popover, menu de contexto ou gaveta deve fechar com `Escape` e clique no backdrop externo, devolvendo foco ao botão que o abriu. Ao usar wrappers visuais ou componentes como [`Button.svelte`](../../frontend/src/lib/components/ui/Button.svelte), garantir referência direta ao `<button>` HTML nativo (via `id` ou ref) para que `.focus()` funcione.
5. **Semântica estrita de `tablist` e divisórias**:
   - `role="tablist"` deve conter exclusivamente filhos `role="tab"` com `aria-selected`, `aria-controls` e navegação por `ArrowLeft`/`ArrowRight`/`Home`/`End`; botões utilitários (minimizar, maximizar, adicionar) ficam fora do `tablist`.
   - Splitters redimensionáveis devem usar `PointerEvent` (`setPointerCapture`) e `role="separator"` com suporte a setas do teclado, `aria-orientation`, `aria-valuemin`/`max`/`now` e `aria-disabled` quando bloqueado/maximizado.
6. **Prevenção contra disparo acidental no ar**: seleções que alteram imediatamente a tela pública a partir de listas densas (como fundos de letras ou itens rápidos) devem exigir duplo clique ou botão explícito, nunca um clique simples acidental.
7. **Sem placeholders falsos ou IDs frágeis**: nunca exibir datas ou textos fixos inventados quando não houver dado real; para foco/rolagem por elemento de lista, usar IDs compostos estáveis (ID do item + índice) em vez de índice isolado.

---

## 4. Responsividade (`1024×768` e `1440×900`), Alvos e Tipografia

1. **Piso de área clicável (`>= 24×24px`)**: nenhum botão, aba, chip ou ícone interativo pode medir menos que `24×24px` (preferir `>= 28–36px` para controles frequentes de culto ao vivo). Em linhas flex densas, aplicar `shrink-0` nos botões de ícone para que títulos longos não os esmaguem abaixo de 24px.
2. **Scroll vertical sem esmagamento em contêineres baixos**: em áreas roláveis (`overflow-y-auto`) com conteúdo centralizado (`items-center justify-center`), envolver o conteúdo em um wrapper `min-h-full` ou evitar `justify-center` direto na raiz do scroll para que botões inferiores (como `"Tentar novamente"`) nunca sejam cortados fora da área visível.
3. **Piso tipográfico operacional (`>= 11px`)**: todo texto funcional de operação (rótulos, badges, metadados, dicas, alertas) deve ter no mínimo `11px` (`text-[11px]` ou `text-xs`), reservando tamanhos inferiores apenas ao escalonamento proporcional interno de mini-previews 16:9. Usar linguagem clara para o operador em `pt-BR` sem jargão técnico cru (ex.: `"Exata"` no lugar de `"[Ab]"`).
4. **Grades fluidas e alturas limitadas ao viewport**: substituir grids fixos de muitas colunas (`lg:grid-cols-4`) e alturas rígidas (`75vh`, `88%` sem teto dinâmico) por `grid-cols-[repeat(auto-fill,minmax(...))]` e tetos calculados contra `viewportHeight`, preservando espaço utilizável no palco principal mesmo em `1024×768`.

---

## 5. Modularização de Componentes Grandes

Componentes acima de ~600–800 linhas que misturam múltiplas abas ou fluxos independentes devem ser divididos em módulos coesos por domínio (seguindo o padrão de `frontend/src/lib/components/bottom-panel/`), mantendo estado compartilhado em stores tipadas e helpers puros sem duplicação.

---

## 6. Checklist Obrigatório Antes de Concluir um Módulo Frontend

1. `cd frontend && npm run check` sem nenhum erro novo nos arquivos criados ou editados (registrando eventuais erros preexistentes fora do escopo).
2. `cd frontend && npm run build` concluído com código `0`.
3. `npx impeccable detect --json` executado nos arquivos/diretórios do módulo sem regressões estruturais.
4. Inspeção visual e funcional em navegador nas resoluções **`1440×900`** e **`1024×768`**, nos temas **`Escuro`** e **`Claro`** reais (alternados via configurações do app):
   - zero erro de console ou loop de rede (`404`/`500` repetido);
   - zero texto funcional `< 11px` e zero alvo interativo `< 24×24px`;
   - navegação por teclado (`Tab`, `Escape`, setas em `tablist`/listas) e preservação de estado ao alternar/recolher painéis;
   - quando o backend Rust não estiver ativo no preview estático, registrar explicitamente quais endpoints/fluxos ficaram limitados pelo preview sem servidor.

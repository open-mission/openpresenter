# Spec: Impeccable Audit & Craft Fixes no Painel do Operador (Admin)

## Intenção
Implementar integralmente as correções de craft, acessibilidade, theming semântico, responsividade e integridade identificadas na auditoria técnica Impeccable no Painel do Operador (`AdminView.svelte`, `BottomPanel.svelte`, `CommandPalette.svelte`, `SettingsModal.svelte` e `app.css`), elevando o Audit Health Score da interface de 12/20 (Acceptable) para 19-20/20 (Excellent).

## Escopo
1. **Acessibilidade (A11y & ARIA):**
   - Adicionar atributos `aria-label`, `aria-expanded` e `aria-haspopup` em todos os botões de ação e ícones no cabeçalho e barras de ferramentas do operador.
   - Implementar o padrão WAI-ARIA completo de abas no painel inferior (`BottomPanel.svelte`): `role="tablist"`, `role="tab"`, `aria-selected`, `aria-controls` e `role="tabpanel"`.
   - Adicionar suporte a navegação por teclado (`role="separator"`, `tabindex="0"`, `aria-orientation="horizontal"`) no handle de redimensionamento do painel inferior.
   - Adicionar classes `focus-visible:ring-2 focus-visible:ring-primary focus-visible:outline-none focus-visible:ring-offset-1` em elementos interativos.
2. **Theming e Design Tokens:**
   - Eliminar mais de 100 classes literais de cor fixa (`neutral-800`, `neutral-900`, `zinc-800`), substituindo-as pelos tokens semânticos do Tailwind (`bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`), garantindo suporte perfeito aos temas customizados e temas claros.
   - Corrigir o anti-pattern `[gray-on-color]` no `CommandPalette.svelte`, garantindo texto contrastante puro (`text-white`) sobre fundos coloridos de destaque (`bg-emerald-600`, `bg-amber-600`, `bg-sky-600`).
3. **Motion e Sensibilidade a Movimento:**
   - Adicionar suporte a `@media (prefers-reduced-motion: reduce)` em `app.css` para suavizar e respeitar preferências de acessibilidade vestibular do sistema operacional.
   - Substituir o anti-pattern `[bounce-easing]` (`animate-bounce`) no `SettingsModal.svelte` por uma desaceleração e pulso suaves de alta elegância.
4. **Responsividade e Alvos de Toque:**
   - Assegurar área de toque mínima confortável (`min-h-[32px] min-w-[32px]` / `min-h-[36px]`) em controles rápidos.
   - Suporte inteligente a viewports menores em `AdminView.svelte`.

## Critérios de Aceite
1. O detector `impeccable detect` roda em `frontend/src/` com zero (0) anti-patterns restantes.
2. Todos os botões do cabeçalho e controles do operador contêm `aria-label` descritivos e anéis de foco visíveis (`focus-visible`).
3. As abas do `BottomPanel.svelte` possuem semântica acessível (`tablist`, `tab`, `tabpanel`) e o divisor suporta interação por teclado.
4. O CSS global respeita `prefers-reduced-motion: reduce`.
5. Os portões de qualidade Rust e Frontend passam integralmente (`cargo fmt`, `cargo clippy`, `cargo test`, `npm check` e `npm build`).

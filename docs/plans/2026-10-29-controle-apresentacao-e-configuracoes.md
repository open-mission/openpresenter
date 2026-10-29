# Plano — Controle de Apresentação, Partes da Música e Configurações

- **Status:** em andamento
- **Data:** 2026-10-29
- **Autores:** OpenPresenter Core Team
- **Spec:** [Spec Controle de Apresentação e Configurações](../specs/2026-10-29-controle-apresentacao-e-configuracoes.md)

## 1. Etapas de Execução

1. **Assets e Identidade Visual:**
   - Copiar os arquivos canônicos de `assets/branding/` para `frontend/public/branding/`.
   - Atualizar `frontend/index.html` para usar `/branding/open-presenter-app-icon.svg` como favicon.
   - Atualizar os cabeçalhos de `AdminView.svelte`, `PublicView.svelte` e `StageView.svelte` para usar o ícone oficial e o wordmark.

2. **Gerenciamento de Preferências e Cores de Seções:**
   - Criar módulo `frontend/src/lib/settings.ts` com gerenciamento de tema (Dark, Light, System) e configurações de seções (nome, cor, atalho).
   - Integrar persistência no `localStorage`.

3. **Componente de Configurações com Abas Verticais:**
   - Criar `frontend/src/lib/components/SettingsModal.svelte` com abas verticais:
     - Aparência (Tema Dark / Light / System)
     - Seções e Cores (Customização visual e atalhos de cada tipo de parte)
     - Telas / Saídas (Ativação e abertura das telas gráficas)
     - Atalhos (Guia de referência operacional)
   - Adicionar botão de configurações (engrenagem) no cabeçalho do `AdminView.svelte`.

4. **Lógica de Foco vs. Apresentação (Live):**
   - No `AdminView.svelte`, desacoplar a seleção interna do disparo ao vivo:
     - Criar `focusedSlideIndex`.
     - Clicar em um slide atualiza apenas o foco.
     - Botão "Apresentar" no cabeçalho dos slides dispara o slide focado para a projeção.
     - Tecla `Enter` ou duplo clique dispara o slide selecionado.
     - Tecla `Esc` encerra o modo de apresentação ao vivo.
     - Setas `←` e `→` ou espaço navegam pelos slides.

5. **Atalhos Rápidos de Seções e Salto Numérico:**
   - Implementar buffer numérico com exibição HUD flutuante ("Slide #N") ao digitar dígitos e disparar ao pressionar `Enter`.
   - Interceptar atalhos configuráveis de seções (`V`, `R`, `P`, `I`, `F`) para navegar diretamente até a parte da música e colocar ao vivo se a apresentação estiver ativa.

6. **Validação, Portões de Qualidade e Commits:**
   - Executar `cargo fmt --all`.
   - Executar `cargo clippy --workspace --all-targets -- -D warnings`.
   - Executar `cargo test --workspace`.
   - Executar `cd frontend && bun run check && bun run build`.
   - Atualizar `CHANGELOG.md` e índices de documentação.
   - Realizar commits atômicos organizados.

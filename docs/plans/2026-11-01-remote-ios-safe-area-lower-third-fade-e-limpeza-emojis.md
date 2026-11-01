# Plano — Remote iOS Safe-Area & Divisores, Fade no Lower Third e Limpeza de Emojis

**Data:** 2026-11-01  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-01-remote-ios-safe-area-lower-third-fade-e-limpeza-emojis.md`](../specs/2026-11-01-remote-ios-safe-area-lower-third-fade-e-limpeza-emojis.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Finalização das Configurações no SettingsModal
- [x] Em [`frontend/src/lib/components/SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - [x] Adicionar controle de fade suave para imagens e papel de parede na aba Tela Pública > Margens & Geral (`image_fade_enabled` e `image_fade_duration_ms`).
  - [x] Na aba Monitor de Palco > Agenda, adicionar inputs numéricos acoplados e equalizar sliders de Dia da Semana, Horário e Detalhes.
  - [x] Na aba Monitor de Palco > Layout & Tempo, adicionar switches para ocultar segundos (`clock_show_seconds`) e data (`clock_show_date`), além de inputs numéricos acoplados aos tamanhos de fonte de Relógio, Timer e Aviso.
  - [x] Ampliar largura dos containers de abas do Palco para `max-w-4xl` e assegurar transição e cliques fluidos para "Layout & Tempo".

### Etapa 2: Ajustes no Remote (/remote)
- [x] Em [`frontend/src/lib/views/RemoteView.svelte`](../../frontend/src/lib/views/RemoteView.svelte):
  - [x] Remover o texto "OpenPresenter" no header, preservando o ícone oficial e o badge de status.
  - [x] Corrigir padding superior para safe-area no iOS com `max(calc(env(safe-area-inset-top, 0px) + 8px), 14px)`.
  - [x] Renderizar itens do tipo `header` como separadores visuais elegantes (sem número sequencial e sem "0 slides").
  - [x] Reindexar visualmente os números das mídias reais desconsiderando os divisores de seção.

### Etapa 3: Fade Suave no Lower Third (OBS / Transmissão)
- [x] Em [`frontend/src/lib/views/LowerThirdView.svelte`](../../frontend/src/lib/views/LowerThirdView.svelte):
  - [x] Importar `fade` de `svelte/transition`.
  - [x] Aplicar `transition:fade` no contêiner do box do Lower Third (`{#if isVisible && currentSlide}`) para entrada e saída suaves ao iniciar, encerrar, limpar ou aplicar blackout.
  - [x] Adicionar transição suave de fade entre slides via `{#key currentSlide.id || currentSlide.content}`.

### Etapa 4: Limpeza Global de Emojis
- [x] Substituir emojis por texto ou ícones SVG Lucide:
  - [x] [`frontend/src/lib/views/LowerThirdView.svelte`](../../frontend/src/lib/views/LowerThirdView.svelte): referências e badges.
  - [x] [`frontend/src/lib/components/SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte): abas, preview do Lower Third e botões de modo.
  - [x] [`frontend/src/lib/components/BottomPanel.svelte`](../../frontend/src/lib/components/BottomPanel.svelte): abas da biblioteca Outros.
  - [x] [`frontend/src/lib/components/ui/MediaItemRow.svelte`](../../frontend/src/lib/components/ui/MediaItemRow.svelte): badge de Lower Third.
  - [x] [`frontend/src/lib/views/AdminView.svelte`](../../frontend/src/lib/views/AdminView.svelte): botões de ação e status.
  - [x] [`frontend/src/lib/views/StageView.svelte`](../../frontend/src/lib/views/StageView.svelte): aviso do operador.
  - [x] [`frontend/src/lib/views/RemoteView.svelte`](../../frontend/src/lib/views/RemoteView.svelte): status da projeção.

### Etapa 5: Validação de Qualidade e Documentação
- [x] Compilar frontend: `cd frontend && bun run build`.
- [x] Executar linters e testes: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.

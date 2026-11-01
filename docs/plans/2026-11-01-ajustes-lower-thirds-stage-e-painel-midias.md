# Plano — Ajustes de Lower Thirds, Stage View, Modal de Configurações e Painel de Mídias

**Data:** 2026-11-01  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-01-ajustes-lower-thirds-stage-e-painel-midias.md`](../specs/2026-11-01-ajustes-lower-thirds-stage-e-painel-midias.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Correção do SettingsModal (Layout & Tempo do Stage + Largura da Modal)
- [x] Em [`frontend/src/lib/components/SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - [x] Adicionar `MessageSquare` na importação de `lucide-svelte` para corrigir a falha ao abrir a sub-aba "Layout & Tempo".
  - [x] Aumentar a largura do container da modal para `w-[95vw] max-w-6xl 2xl:max-w-7xl h-[90vh] max-h-[850px]` e relaxar containers internos para melhor aproveitamento da tela.

### Etapa 2: Aprimoramentos do Lower Third
- [x] Em [`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts) e [`frontend/src/lib/displaySettings.ts`](../../frontend/src/lib/displaySettings.ts):
  - [x] Adicionar `scripture_font_size`, `box_bg_type`, `box_bg_image_url`, `box_border_radius`, `box_edge_fade`, `box_edge_fade_amount` a `LowerThirdSettings`.
- [x] Em [`frontend/src/lib/views/LowerThirdView.svelte`](../../frontend/src/lib/views/LowerThirdView.svelte):
  - [x] Usar `scripture_font_size` quando `isScripture` for verdadeiro.
  - [x] Corrigir alinhamento da Caixa Flutuante (`floating`) respeitando `lt.text_align` (`left`, `center`, `right`).
  - [x] Suavizar Gradiente Central (`gradient`) com fade radial e máscara suave sem cortes retos.
  - [x] Suporte a imagem de fundo na caixa com bordas arredondadas e efeito de fade nas bordas (`box_edge_fade`).
- [x] Em [`frontend/src/lib/components/SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - [x] Adicionar controles para tamanho de fonte bíblica no Lower Third e controles de fundo com imagem (upload/seleção de mídia, border-radius e fade nas bordas).

### Etapa 3: Stage View com Imagem em Tela Cheia
- [x] Em [`frontend/src/lib/views/StageView.svelte`](../../frontend/src/lib/views/StageView.svelte) e [`frontend/src/lib/components/StagePreviewBox.svelte`](../../frontend/src/lib/components/StagePreviewBox.svelte):
  - [x] Quando `isCurrentImage` for verdadeiro, ocultar o bloco "A Seguir" e expandir o bloco do Slide Atual para ocupar todo o espaço útil acima do rodapé de relógio/timer/aviso.
  - [x] Remover limite rígido `max-h-[50vh]` na imagem, permitindo ajuste proporcional suave com `object-contain`.

### Etapa 4: BottomPanel — Aba Outros e Borda de Seleção de Músicas
- [x] Em [`frontend/src/lib/components/BottomPanel.svelte`](../../frontend/src/lib/components/BottomPanel.svelte):
  - [x] Na aba "Outros": remover auto-seleção inicial (`viewingCustomItem = null`), renderizar grade completa de cartões quando fechado e split view quando um item for clicado (com botão de fechar X).
  - [x] Na aba de Músicas e de Outros: adicionar espaçamento interno adequado (`p-1.5`) nos containers de rolagem para que as bordas e anéis de foco/seleção dos cartões não fiquem cortados.

### Etapa 5: Validação e Portões de Qualidade
- [x] `cd frontend && bun run build`
- [x] `cargo fmt --all`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.

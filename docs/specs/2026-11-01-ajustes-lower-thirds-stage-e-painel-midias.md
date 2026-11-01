# Spec — Ajustes de Lower Thirds, Stage View, Modal de Configurações e Painel de Mídias

**Data:** 2026-11-01  
**Status:** Aprovada  
**Autor:** Antigravity  
**Contexto:** Solicitação de aprimoramentos de layout, usabilidade e renderização no Lower Third (OBS), Monitor de Palco (Stage), SettingsModal e BottomPanel (mídias).

---

## 1. Intenção e Motivação

Atender a um conjunto de melhorias de interface e fidelidade de exibição relatadas pelo usuário:
1. **Lower Thirds (OBS):**
   - Suporte a configuração de tamanho de fonte independente para versículos bíblicos (`scripture_font_size`), permitindo tamanhos menores e mais confortáveis para leituras bíblicas longas sem afetar letras de louvor.
   - Correção do alinhamento da **Caixa Flutuante** (`floating`): atualmente forçava alinhamento centralizado independente da configuração selecionada; deve respeitar `text_align` (`left`, `center`, `right`) tanto no posicionamento quanto no texto.
   - Suavização do **Gradiente Central** (`gradient`): evitar bordas cortadas secas usando gradiente radial mais suave e máscara com fade suave nas bordas (`mask-image`).
   - Suporte a **Imagem como Fundo da Caixa**: permitir utilizar uma imagem (PNG/JPG com transparência ou foto) como plano de fundo da caixa do Lower Third, com suporte a cantos arredondados (`border-radius`) e efeito de fade suave nas bordas (feather/máscara).
2. **Monitor de Palco (Stage View):**
   - Ao apresentar uma mídia do tipo imagem (ou slide contendo imagem), a imagem deve expandir para preencher 100% da área superior (unificando o espaço do Slide Atual e do A Seguir), mantendo apenas o rodapé com relógio, cronômetro e aviso.
3. **Modal de Configurações (SettingsModal):**
   - Aumento da largura da modal (`w-[95vw] max-w-6xl 2xl:max-w-7xl h-[90vh] max-h-[850px]`) para melhor aproveitamento em telas modernas.
   - Correção de erro na aba Stage: importação do ícone `MessageSquare` que causava falha ao tentar acessar a sub-aba "Layout & Tempo".
4. **Painel Inferior de Mídias (BottomPanel):**
   - Na aba "Outros" (Agenda, Textos, Avisos, Seções), o sheet lateral de preview dos slides não deve iniciar aberto; deve abrir apenas quando um item for clicado (igual ao comportamento da aba de Músicas).
   - Na aba de Músicas (e Outros), ajuste de espaçamento interno (`p-1.5`) para evitar que a borda/anel de seleção do item fique cortada nos limites da rolagem.

---

## 2. Escopo

- [`frontend/src/lib/types.ts`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/types.ts): Extensão de `LowerThirdSettings` com `scripture_font_size`, `box_bg_type`, `box_bg_image_url`, `box_border_radius`, `box_edge_fade`, `box_edge_fade_amount`.
- [`frontend/src/lib/displaySettings.ts`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/displaySettings.ts): Valores padrão e sincronização de `DEFAULT_LOWER_THIRD_SETTINGS`.
- [`frontend/src/lib/views/LowerThirdView.svelte`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/views/LowerThirdView.svelte): Aplicação do alinhamento configurado na caixa flutuante, renderização de fundo por imagem com arredondamento e fade nas bordas, e gradiente central sem corte.
- [`frontend/src/lib/views/StageView.svelte`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/views/StageView.svelte) e [`frontend/src/lib/components/StagePreviewBox.svelte`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/components/StagePreviewBox.svelte): Expansão da imagem para ocupar todo o espaço útil acima do rodapé.
- [`frontend/src/lib/components/SettingsModal.svelte`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/components/SettingsModal.svelte): Largura ampliada da modal, importação de `MessageSquare`, novos controles de fonte bíblica e fundo de imagem no Lower Third.
- [`frontend/src/lib/components/BottomPanel.svelte`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/components/BottomPanel.svelte): Fechamento inicial do sheet na aba Outros, exibição condicional e padding anti-clipping de seleção nas músicas e outros itens.

---

## 3. Critérios de Aceite

1. **Lower Thirds — Versículos Bíblicos:**
   - Sliders independentes em Settings para Louvor (`font_size`) e Bíblia (`scripture_font_size`).
   - Ao projetar versículo bíblico, o Lower Third aplica `scripture_font_size`.
2. **Lower Thirds — Caixa Flutuante:**
   - Respeita `text_align: 'left'` (alinhado à esquerda), `'center'` (centralizado) e `'right'` (alinhado à direita), tanto na caixa quanto no texto.
3. **Lower Thirds — Gradiente Central e Fundo de Imagem:**
   - Gradiente central não apresenta cortes secos retangulares nas bordas.
   - Possibilidade de selecionar imagem como fundo do Lower Third, com suporte a `border-radius` ajustável e efeito de fade suave nas bordas.
4. **Stage View com Imagem:**
   - Quando `isCurrentImage` for verdadeiro, o bloco "A Seguir" é ocultado e o bloco da imagem ocupa 100% da área acima do rodapé.
5. **SettingsModal:**
   - Modal mais larga aproveitando a janela.
   - Sub-aba "Layout & Tempo" do Stage abre normalmente sem travamento.
6. **BottomPanel:**
   - Aba Outros inicia com a grade de cartões completa; clicar em um item abre o painel lateral com botão de fechar (X).
   - Bordas e anéis de seleção dos cartões de música e outros itens não sofrem cortes.
7. **Qualidade:**
   - Build do frontend (`bun run build`) e gates do Rust (`cargo fmt`, `clippy -D warnings`, `cargo test`) passam com 100% de sucesso.

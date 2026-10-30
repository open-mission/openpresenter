# Spec — Configurações Desacopladas de Louvores e Textos Bíblicos

- **Status:** aprovada
- **Data:** 2026-10-30
- **Autor:** OpenPresenter Core Team
- **Contexto:** commit `0da4508d0ff2674efcfeb84343c6b1744934f3fb`

## 1. Intenção

Permitir que operadores configurem a tipografia, alinhamentos, limites de tamanho de fonte e cores de forma totalmente independente para letras de louvor (músicas) e versículos bíblicos, tanto na projeção pública quanto no monitor de retorno de palco. Além disso, aprimorar o contraste visual das chaves de alternância (toggles/switches).

## 2. Escopo

- Extensão do schema de configurações em `frontend/src/lib/types.ts` (`PublicDisplaySettings` e `StageDisplaySettings`).
- Valores padrão retrocompatíveis em `frontend/src/lib/displaySettings.ts`.
- Reorganização do modal de configurações (`SettingsModal.svelte`) em sub-seções estruturadas para Projeção Pública e Monitor de Palco.
- Aplicação das configurações desacopladas nas visões `PublicView.svelte`, `StageView.svelte` e no preview `StagePreviewBox.svelte`.
- Alternador de prévia (Louvor vs. Bíblia) na aba de Fundos de Letras & CSS customizado.

## 3. Decisões

1. **Campos dedicados para Escrituras:**
   - Projeção Pública: `scripture_align_h`, `scripture_align_v`, `scripture_min_font_size`, `scripture_max_font_size`, `scripture_fill_space`.
   - Projeção Pública (Letras de Louvor): `lyrics_align_h`, `lyrics_align_v`, `lyrics_min_font_size`, `lyrics_max_font_size`, `fill_space`.
   - Monitor de Palco: `scripture_text_color`, `scripture_border_color`, `scripture_min_font_size` independentes de `current_text_color` e `current_border_color`.
2. **Sub-navegação no Modal de Configurações:**
   - Na aba **Projeção Pública**, sub-abas: `Letras de Louvor`, `Textos Bíblicos` e `Margens & Geral`.
   - Na aba **Monitor de Palco**, sub-abas: `Letras no Palco`, `Textos Bíblicos no Palco` e `Layout & Tempo`.
3. **Prévia Dinâmica 16:9 em Fundos de Letras:**
   - Adicionado alternador em tempo real `[🎵 Letra de Louvor]` vs `[📖 Texto Bíblico]` para validar instantaneamente o contraste do CSS e cor de texto em ambos os formatos.
4. **Correção de Contraste em Toggles:**
   - Chaves de alternância ativas com indicador verde esmeralda de alto contraste (`bg-emerald-600`), eliminando o bug de renderização branca ilegível.

## 4. Critérios de Aceite

- [x] Configurar alinhamento à esquerda para Bíblia não afeta o alinhamento centralizado dos louvores.
- [x] Tamanho de fonte máximo e mínimo para versículos bíblicos opera independentemente das músicas.
- [x] Cores de texto e borda no Monitor de Palco para Bíblia respeitam a preferência salva.
- [x] Toggles e switches possuem estado ligado/desligado nitidamente contrastante.

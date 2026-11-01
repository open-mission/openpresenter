# Spec — Customizações e Configurações Visuais da Agenda (Public e Stage)

**Data:** 2026-10-31  
**Status:** Aprovado  
**Autor:** Antigravity / OpenPresenter Team  
**Plano correspondente:** [`docs/plans/2026-10-31-configuracoes-agenda-public-e-stage.md`](../plans/2026-10-31-configuracoes-agenda-public-e-stage.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Intenção e Problema

O tipo de mídia Agenda (`MediaKind::Schedule`) introduzido na versão 0.4.0 permite exibir programações de cultos semanais e eventos festivos futuros. No entanto, sua apresentação visual na Tela Pública (`/public`) e no Monitor de Palco (`/stage`) utilizava valores e estilos estáticos rígidos (container fixo com fundo preto 65%, padding fixo e tamanhos de fonte predeterminados).

Diferentes igrejas e ambientes possuem necessidades visuais distintas:
1. Algumas igrejas utilizam papéis de parede artísticos ricos ou vídeos de fundo e desejam remover o fundo do container da agenda ou controlar sua opacidade para máxima integração estética.
2. Telões de diferentes resoluções exigem controle da largura máxima do container (% da tela) e do espaçamento interno (padding compacto vs espaçoso).
3. Operadores necessitam ajustar a tipografia de forma granular: tamanho do título (autoScale min/max), tamanho das badges de dia da semana e horário, e tamanho dos detalhes/locais.
4. No Monitor de Palco (`/stage`), os cantores e pregadores necessitam de calibração independente de fontes para o evento atual e o próximo evento da agenda, permitindo leitura confortável a distâncias de 10 a 30 metros.

---

## 2. Escopo

### Dentro do escopo
- Extensão de `PublicDisplaySettings` ([`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts)) com configurações dedicadas para Agenda:
  - `schedule_box_background`: Ativar/desativar fundo translúcido do container (padrão: `true`).
  - `schedule_bg_opacity`: Opacidade do fundo do container de 0% a 100% (padrão: `65`).
  - `schedule_max_width_percent`: Largura máxima do container de 50% a 100% (padrão: `80`).
  - `schedule_padding`: Espaçamento interno `'compact' | 'normal' | 'spacious'` (padrão: `'normal'`).
  - `schedule_title_min_font_size` e `schedule_title_max_font_size`: Limites do auto-scale do título (padrão: `32` a `110`).
  - `schedule_day_font_size`: Tamanho da badge de dia da semana (padrão: `20`).
  - `schedule_time_font_size`: Tamanho da badge de horário (padrão: `18`).
  - `schedule_details_font_size`: Tamanho do texto de detalhes/local (padrão: `20`).
  - `schedule_show_badges`: Exibir ou ocultar badges de dia e horário (padrão: `true`).
  - `schedule_show_footer`: Exibir ou ocultar o identificador/título da agenda no rodapé (padrão: `true`).
  - `schedule_align_h`: Alinhamento horizontal do conteúdo `'left' | 'center' | 'right'` (padrão: `'center'`).
- Extensão de `StageDisplaySettings` ([`frontend/src/lib/types.ts`](../../frontend/src/lib/types.ts)) para calibração de textos da Agenda no Palco:
  - `stage_schedule_title_min_font_size` e `stage_schedule_title_max_font_size`: Auto-scale do título no Slide Atual (padrão: `28` a `180`).
  - `stage_schedule_day_font_size`: Tamanho da badge de dia no Slide Atual (padrão: `18`).
  - `stage_schedule_time_font_size`: Tamanho da badge de horário no Slide Atual (padrão: `16`).
  - `stage_schedule_details_font_size`: Tamanho dos detalhes no Slide Atual (padrão: `18`).
  - `stage_schedule_next_min_font_size` e `stage_schedule_next_max_font_size`: Auto-scale do título no Próximo Slide (A Seguir) (padrão: `18` a `120`).
- Atualização da aba "Projeção Pública" no [`SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte) adicionando a subseção `Agenda / Programação` com controles interativos e prévia 16:9 em tempo real com modo `[📅 Agenda]`.
- Atualização da aba "Monitor de Palco" no [`SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte) adicionando a subseção `Agenda` para ajuste dos tamanhos de fontes do palco.
- Aplicação das configurações em [`PublicView.svelte`](../../frontend/src/lib/views/PublicView.svelte), [`PublicPreviewBox.svelte`](../../frontend/src/lib/components/PublicPreviewBox.svelte), [`StageView.svelte`](../../frontend/src/lib/views/StageView.svelte) e [`StagePreviewBox.svelte`](../../frontend/src/lib/components/StagePreviewBox.svelte).

### Fora do escopo
- Alteração no modelo de dados do evento da agenda (`ScheduleEvent`).

---

## 3. Decisões Arquiteturais e de Interface

1. **Subseções Dedicadas nos Modais de Configurações:**
   - Na aba "Projeção Pública", a barra de seções passa a contar com 4 abas equilibradas: `[🎵 Letras de Louvor]`, `[📖 Textos Bíblicos]`, `[📅 Agenda]`, `[🎛️ Margens & Geral]`.
   - Na aba "Monitor de Palco", da mesma forma: `[🎵 Letras de Louvor]`, `[📖 Textos Bíblicos]`, `[📅 Agenda]`, `[🎛️ Layout & Tempo]`.
2. **Container Flexível com Suporte a Transparência:**
   - Se `schedule_box_background === false`, o container remove cor de fundo, blur e bordas, aplicando apenas drop-shadow de texto para contraste sobre papéis de parede escuros.
   - A opacidade é dinamicamente interpolada via `rgba(0, 0, 0, opacity / 100)`.
3. **Paridade Fiel entre Telas Reais e Caixas de Pré-visualização:**
   - Todos os cálculos e classes são compartilhados entre `PublicView` e `PublicPreviewBox`, e entre `StageView` e `StagePreviewBox`.

---

## 4. Critérios de Aceite

1. O operador pode acessar Configurações > Projeção Pública > Agenda e ajustar:
   - Exibir/ocultar fundo do container e alterar opacidade;
   - Regular a largura máxima do container de 50% a 100%;
   - Selecionar o espaçamento interno (compacto, normal, espaçoso);
   - Regular individualmente o tamanho mínimo/máximo do título, dia, hora e detalhes;
   - Ocultar badges ou rodapé se desejado;
   - Visualizar tudo em tempo real na prévia 16:9 selecionando o modo `[📅 Agenda]`.
2. O operador pode acessar Configurações > Monitor de Palco > Agenda e ajustar:
   - Tamanhos de títulos, badges e detalhes do slide atual e do próximo slide.
3. As alterações refletem imediatamente na Projeção Pública (`/public`) e no Monitor de Palco (`/stage`) sem necessidade de recarregar a página.
4. `cargo fmt`, `cargo clippy`, `cargo test` e compilação do frontend executam com zero erros.

# Spec — Fontes da Agenda, Relógio sem Segundos/Data no Palco, Fade de Imagens e Abas Agrupadas

**Data:** 2026-11-01  
**Status:** Aprovado  
**Escopo:** `frontend` (`types.ts`, `displaySettings.ts`, `SettingsModal.svelte`, `PublicView.svelte`, `StageView.svelte`, `StagePreviewBox.svelte`, `PublicPreviewBox.svelte`) e documentação  
**Plano correspondente:** [`docs/plans/2026-11-01-agenda-fontes-customizadas-e-stage-relogio-segundos-data.md`](../plans/2026-11-01-agenda-fontes-customizadas-e-stage-relogio-segundos-data.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Intenção e Motivação

1. **Equalização e Fontes Customizadas na Agenda (Tela Pública e Palco):**
   - Na aba de configurações da Agenda na Projeção Pública, os controles de tamanho de fonte do Dia da Semana (`schedule_day_font_size`) e do Horário (`schedule_time_font_size`) possuíam limites máximos diferentes (`max="36"` vs `max="32"`). Como ambos são dispostos lado a lado na mesma linha na tela pública, essa discrepância gerava assimetria e impedia igualar tamanhos ou aumentar o texto em telões de alta resolução.
   - O usuário necessita da capacidade de definir um tamanho customizado de fonte através de entrada numérica direta (`<input type="number">`), permitindo valores que ultrapassem o limite tradicional dos sliders, com ajuste dinâmico do slider para acompanhar o valor customizado.

2. **Controle de Exibição do Relógio no Palco (Segundos e Data):**
   - Nas configurações do Monitor de Palco (`/stage`), adicionar opções para ocultar os segundos (exibindo formato limpo `HH:MM` em vez de `HH:MM:SS`) e para ocultar a data (dia da semana e data por extenso).
   - Essas configurações devem ser respeitadas tanto no monitor de palco em execução normal (caixa de horário do rodapé) quanto na tela de descanso/espera do palco (relógio central imponente quando nenhuma mídia está ativa).

3. **Fade Customizável para Imagens e Papel de Parede:**
   - Ao apresentar uma imagem e ao retornar para o papel de parede (ou alternar entre imagens na projeção pública), aplicar uma transição suave com efeito de fade (cross-fade / opacidade).
   - Disponibilizar configurações na aba Projeção Pública para ativar/desativar o fade (`image_fade_enabled`) e ajustar a duração em milissegundos (`image_fade_duration_ms`), com slider e campo numérico.

4. **Agrupamento Temático das Abas de Configuração (`SettingsModal.svelte`):**
   - Organizar as 14 abas verticais da barra lateral de configurações em categorias lógicas claras (ex.: **Telas & Projeção**, **Conteúdo & Mídias**, **Geral & Sistema**), facilitando a localização de recursos e aprimorando a ergonomia do operador.

---

## 2. Decisões Arquiteturais e de Interface

1. **Novas Propriedades em `StageDisplaySettings`:**
   - `clock_show_seconds?: boolean;` (padrão: `true` — quando `false`, exibe apenas `HH:MM`).
   - `clock_show_date?: boolean;` (padrão: `true` — quando `false`, oculta a linha de data).

2. **Novas Propriedades em `PublicDisplaySettings`:**
   - `image_fade_enabled?: boolean;` (padrão: `true`).
   - `image_fade_duration_ms?: number;` (padrão: `500` ms, ajustável entre 50ms e 3000ms).

3. **Equalização da Tipografia da Agenda (`PublicDisplaySettings` e `StageDisplaySettings`):**
   - Valores padrão harmonizados: `schedule_day_font_size: 20` e `schedule_time_font_size: 20`.
   - Sliders de dia e horário com o mesmo range amplo: `min="12"` e `max={Math.max(80, value)}`.
   - Adição de `<input type="number">` compacto com estilização Shadcn ao lado do slider para entrada direta de valores em pixels (ex: 12 a 200px), permitindo escalar o texto sem restrições.

4. **Formatação do Horário e Exibição de Data em `StageView.svelte` e `StagePreviewBox.svelte`:**
   - `updateClock` gera `currentTime` formatado condicionalmente: se `clock_show_seconds !== false`, inclui segundos (`HH:MM:SS`); se `false`, gera `HH:MM`.
   - Exibição de `currentDate` envolta em `{#if stageSettings.clock_show_date !== false}` tanto no rodapé do palco quanto na tela de descanso.

5. **Transição de Fade em `PublicView.svelte` e `PublicPreviewBox.svelte`:**
   - Uso de `transition:fade` do Svelte no container da imagem com duração parametrizada por `image_fade_duration_ms`.
   - Ao desativar ou colocar duração 0, a transição é imediata sem animação.
   - Na volta para o papel de parede, a imagem faz fade out suave revelando o fundo existente sem corte abrupto.

6. **Grupos da Barra Lateral do `SettingsModal.svelte`:**
   - **Telas & Projeção:** Tela Pública, Monitor de Palco, Lower Third (OBS), Fundos de Letras & CSS, Telas e Saídas de Vídeo.
   - **Conteúdo & Mídias:** Bíblias Sagradas, Músicas & Harpa, Partes e Cores, Arquivos e Pastas.
   - **Geral & Sistema:** Aparência & Cores, Marca da Igreja, Atalhos Rápidos, Histórico & Logs, Sobre & Updates.

---

## 3. Critérios de Aceite

1. **Agenda Pública e Palco:**
   - Sliders de dia e horário possuem o mesmo limite máximo (padrão 80px, expansível dinamicamente).
   - Há campo numérico interativo ao lado de cada slider permitindo digitar valores customizados.
   - Digitar um valor customizado atualiza o estado imediatamente e ajusta a prévia e a tela pública.
2. **Relógio do Palco:**
   - Opções "Exibir Segundos no Horário" e "Exibir Data Abaixo do Horário" disponíveis em `SettingsModal.svelte` sob o Monitor de Palco.
   - Ao desativar segundos, tanto o rodapé do palco quanto a tela de descanso exibem apenas horas e minutos (`HH:MM`).
   - Ao desativar a data, a data por extenso desaparece tanto no rodapé do palco quanto na tela de descanso.
3. **Fade de Imagens e Papel de Parede:**
   - Chave seletora e controle de duração de fade em milissegundos presentes nas configurações da Projeção Pública.
   - Apresentar imagem e voltar para papel de parede produz fade suave e elegante.
4. **Agrupamento de Abas:**
   - A barra lateral de configurações exibe os grupos categorizados com cabeçalhos sutis e navegação fluida.
5. **Qualidade:**
   - `bun run build` no frontend sem erros de compilação ou de tipos.
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` aprovados com 100% de sucesso.

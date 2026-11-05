# Spec — Tamanho do Aviso no Monitor de Palco em Modo de Espera e Configuração Dedicada

## 1. Intenção

Permitir que mensagens e avisos enviados pela equipe para o Monitor de Palco (`/stage`) sejam perfeitamente legíveis à distância quando a projeção estiver em modo de descanso/espera (*standby* — sem slide ativo), além de fornecer controle dedicado e transparente nas configurações para ajustar o tamanho da fonte desse aviso.

## 2. Contexto e Problema

No monitor de retorno de palco (`/stage`), quando nenhuma mídia ou slide está sendo apresentado, a tela exibe o horário de parede em formato grande, a data e, caso haja uma mensagem de palco ativa (`stageMessage`), um banner centralizado de alerta no topo.

Anteriormente:
1. O banner de aviso no modo de espera utilizava classes estáticas do Tailwind (`text-base sm:text-lg md:text-xl font-bold`), resultando em uma fonte de apenas `20px` em monitores desktop/TV. A essa escala, o texto ficava desproporcionalmente pequeno frente ao relógio de 140px/170px e ilegível a distâncias típicas de palco (5 a 15 metros).
2. O elemento ignorava completamente as configurações de tamanho de fonte (`alert_font_size`), impedindo o operador de ajustar o tamanho do texto através do painel de configurações.
3. O container possuía largura máxima restrita (`max-w-3xl` / 768px) e ícone pequeno fixo em 20px, quebrando textos mais longos de forma desnecessária em telas 1080p ou 4K.

## 3. Escopo e Solução

1. **Configuração Dedicada e Valor Padrão Apropriado:**
   - Adicionar o campo opcional `standby_alert_font_size` em `StageDisplaySettings` (`types.ts`), com valor padrão de `36px` em `DEFAULT_STAGE_SETTINGS` (`displaySettings.ts`).
   - Caso `standby_alert_font_size` não esteja definido, utilizar como fallback o valor configurado em `alert_font_size` ou `36px`.

2. **Interface de Configurações (`SettingsModal.svelte`):**
   - Na aba *Monitor de Palco (Stage)* > *Layout & Tempo*, dentro da seção *Tamanhos das Fontes de Palco*, disponibilizar 4 controles completos em grade responsiva (com `<input type="number">` e slider deslizante):
     1. **Timer / Cronômetro** (`timer_font_size`, padrão `56px`, min 14, max 200)
     2. **Relógio Oficial** (`clock_font_size`, padrão `48px`, min 14, max 200)
     3. **Aviso na Apresentação** (`alert_font_size`, padrão `28px`, min 12, max 150) — Mensagens no rodapé do grid durante os slides.
     4. **Aviso na Espera (Standby)** (`standby_alert_font_size`, padrão `36px`, min 14, max 150) — Banner de destaque na tela de descanso/espera.
   - Atualizar a ação "Restaurar Padrão" para também restaurar `standby_alert_font_size`.

3. **Renderização no Monitor de Palco (`StageView.svelte`):**
   - Atualizar o banner em modo descanso (`!activeSlide`) para aplicar dinamicamente o tamanho configurado via `style="font-size: {standbyAlertSize}px; line-height: 1.2;"`.
   - Dimensionar proporcionalmente o ícone `<AlertTriangle>` de acordo com a fonte configurada (`Math.max(22, Math.min(48, Math.round(standbyAlertSize * 0.9)))`).
   - Ampliar a largura máxima para `max-w-4xl lg:max-w-5xl`, com espaçamento e padding generosos (`p-4 sm:p-5 md:p-6 rounded-2xl`).

4. **Pré-visualização do Operador (`StagePreviewBox.svelte`):**
   - Exibir o banner de aviso proporcionalmente no modo de descanso caso haja `stage_message` ativo, escalonado com base em `standby_alert_font_size`.

## 4. Critérios de Aceite

1. Quando houver mensagem de palco ativa e nenhum slide estiver em exibição, o texto no `/stage` deve ser exibido com destaque e tamanho padrão de `36px` (ou tamanho configurado).
2. Alterar o valor de "Aviso na Espera (Standby)" nas configurações deve refletir imediatamente na visualização `/stage` em tempo real sem necessidade de recarregar a página.
3. Clicar em "Restaurar Padrão" deve resetar todos os tamanhos, incluindo o aviso de espera para `36px`.
4. Todos os portões de qualidade (`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e compilação do frontend) devem ser atendidos sem erros ou advertências.

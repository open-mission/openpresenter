# Spec — Separação da Mensagem de Retorno em Seção Própria na Barra Lateral

## 1. Intenção

Separar o controle de "Mensagem no Retorno (Stage)" do painel do cronômetro, transformando-o em uma seção própria e independente na barra lateral direita de previews (`TimerControlPanel.svelte`), mantendo total consistência visual com as seções de Controle de Saídas, Public, Stage e Cronômetro.

## 2. Contexto e Problema

Anteriormente, o envio de avisos instantâneos para o monitor de palco encontrava-se embutido no rodapé do painel do cronômetro (`TimerControlPanel.svelte`) sob o título "Cronômetro & Palco". Isso gerava dois problemas:
1. Ao recolher o painel do cronômetro para economizar espaço vertical, a caixa de mensagens de palco ficava inacessível.
2. A interface acumulava muitas responsabilidades em um único bloco (mostrador digital, título do timer, presets de minutos e envio de mensagens), dificultando a visualização rápida e o envio ágil de avisos ao vivo durante o culto.

## 3. Escopo e Solução

1. **Seção Autônoma "Cronômetro":**
   - Focada exclusivamente no mostrador do cronômetro, botões de ação (play, pause, reset, zerar), título da contagem com chips rápidos, presets de tempo e relógio oficial de parede.
   - Estado de colapso individual `isTimerCollapsed` com persistência em `localStorage` (`timer_section_collapsed`).

2. **Seção Autônoma "Mensagem no Retorno":**
   - Cabeçalho padronizado com ícone `MessageSquare` em tom esmeralda, título "Mensagem no Retorno" e `ChevronDown` para recolher/expandir.
   - Estado de colapso individual `isMessageCollapsed` com persistência em `localStorage` (`stage_message_collapsed`).
   - Exibição de badge "Ativo" e botão "Limpar" no cabeçalho quando houver mensagem ativa, permitindo limpar o aviso mesmo com a seção recolhida.
   - Quando expandida, exibe o banner da mensagem atual, campo de texto com botão de envio e chips de mensagens rápidas frequentes (*5 min restantes*, *Atenção ao tempo*, *Próximo louvor*, *Encerrar*).

## 4. Critérios de Aceite

1. O controle de mensagem de palco deve ser uma seção própria com seu próprio cabeçalho colapsável.
2. O cronômetro e a mensagem de palco podem ser recolhidos ou expandidos de maneira independente.
3. O status de colapso de ambas as seções deve ser preservado no `localStorage`.
4. Todos os portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test` e `bun run build`) devem passar sem erros.

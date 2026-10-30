# Especificação: Título Configurável para o Cronômetro no Stage Display

## Contexto e Motivação
No painel do operador, o controle do cronômetro ("Cronômetro & Palco") utilizava rótulos automáticos e estáticos como "10 min" ou "Contagem". Durante o culto ou evento, a equipe de mídia precisa indicar claramente aos ministrantes e pregadores qual atividade está sendo cronometrada no monitor de retorno de palco (por exemplo: "Pregação", "Momento de Louvor", "Oração", "Avisos", "Ceia").

## Escopo
- Adicionar campo de entrada editável de título/rótulo do cronômetro no painel `TimerControlPanel.svelte`.
- Oferecer atalhos rápidos (chips) com títulos comuns de cultos ("Pregação", "Louvor", "Oração", "Avisos", "Palavra", "Ceia") para seleção instantânea com um clique.
- Manter o título customizado ativo ao selecionar presets de minutos (5m, 10m, 15m, etc.) e ao adicionar minutos (+1m, +5m).
- Exibir o título atualizado no cabeçalho do bloco de cronômetro no monitor de palco (`StageView.svelte`), no modo standby e no preview do operador (`StagePreviewBox.svelte`).

## Decisões Técnicas
- O estado de timer do backend (`TimerState.label`) já suporta `Option<String>` e é sincronizado via WebSocket em tempo real através da mensagem `SetTimer`.
- As alterações no campo de título no painel do operador disparam a atualização com debounce suave para sincronização imediata com todos os clientes conectados.

## Critérios de Aceite
- O operador pode digitar qualquer título para o cronômetro ou clicar em um chip de sugestão rápida.
- O título definido é exibido imediatamente no bloco do cronômetro no monitor `/stage` e no preview do operador.
- O título persiste ao acionar presets de minutos e controles de tempo.
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

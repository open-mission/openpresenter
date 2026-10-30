# Especificação: Exibição do Timer no Header Somente Quando em Execução

## Contexto e Motivação
No cabeçalho principal do painel do operador (`AdminView.svelte`), o widget de cronômetro permanecia visível permanentemente, inclusive com estado "Pausado" (`isTimerRunning: false`), ocupando espaço visual no cabeçalho mesmo quando o operador não estava utilizando o recurso de cronômetro no momento.

## Escopo
- No componente `AdminView.svelte`, condicionar a renderização do widget do cronômetro no cabeçalho à condição `isTimerRunning`.
- Quando o timer estiver parado/pausado, o widget é ocultado do cabeçalho, deixando o cabeçalho mais limpo e focado.
- Quando o timer for iniciado (pelo painel lateral "Cronômetro & Palco" ou atalhos), o widget aparece no cabeçalho exibindo a contagem e os controles rápidos de pausa e reinício.

## Decisões Técnicas
- A visibilidade é controlada pela expressão reativa `{#if isTimerRunning}` ao redor do container do widget no cabeçalho.
- O painel lateral na barra direita continua permitindo configurar, iniciar e zerar o timer a qualquer momento.

## Critérios de Aceite
- Quando o cronômetro estiver pausado ou zerado, ele não é exibido no cabeçalho superior.
- Quando o cronômetro estiver rodando, ele é exibido normalmente com as cores e badges de aviso/overtime.
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

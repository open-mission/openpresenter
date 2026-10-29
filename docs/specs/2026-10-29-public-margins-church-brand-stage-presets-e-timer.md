# Especificação: Margens do Public, Identidade da Igreja, Presets de Stage e Timer Global

- **Status:** Aprovado
- **Data:** 2026-10-29
- **Autor:** Antigravity

---

## 1. Intenção

Prover personalização visual avançada para a tela de projeção pública (Public) e tela de retorno (Stage), incluindo controle de aproveitamento espacial da letra (preenchimento máximo / auto-fit, controle de quebra de linha e visibilidade de metadados), margens configuráveis e identidade visual da igreja (upload e posicionamento de logos). Adicionar presets de layout e customização de cores para o Stage Display com base em padrões de excelência de transmissão e projeção eclesiástica. Implementar um Timer Global e Relógio sincronizados entre o operador e o palco, com efeitos de contagem regressiva, alerta de tempo esgotando e indicação clara de tempo excedido (overtime com exibição negativa pulsante em vermelho). Garantir a aplicação consistente do favicon da marca do programa.

---

## 2. Escopo

### No escopo

1. **Ajuste Espacial e Margens da Tela Pública (Public Display):**
   - Opção para que o texto preencha o máximo de espaço no slide (`fill_space` / `auto_fit`).
   - Opção de controle de quebra de linha (`word_wrap`: permitir quebra automática ou manter versos em linha única `nowrap`).
   - Controle de visibilidade de metadados da música (`show_metadata`: exibir ou ocultar título e artista/compositor no rodapé).
   - Edição de margens da projeção pública: Margem Superior, Margem Inferior e Margem Lateral (Horizontal).

2. **Identidade Visual da Igreja (Church Branding):**
   - Configuração de marca da igreja com upload e seleção de variações de logotipo:
     - Logo Principal
     - Logo Dark (para fundos escuros)
     - Logo Light (para fundos claros)
     - Logo Completo (com tipografia/slogan)
   - Exibição do logo da igreja na tela pública:
     - Ativação/desativação da exibição do logo
     - Escolha da variante de logo
     - Posicionamento em 4 cantos: Superior Esquerdo (`top-left`), Superior Direito (`top-right`), Inferior Esquerdo (`bottom-left`), Inferior Direito (`bottom-right`)
     - Controle de opacidade (20% a 100%) e escala de tamanho.

3. **Aba de Configuração do Stage Display com Presets e Cores:**
   - Nova aba dedicada ao Stage nas Configurações.
   - Presets de layout baseados nas referências visuais:
     - **Split 50/50:** caixas de Current Slide e Next Slide lado a lado com bordas arredondadas e rótulo no rodapé da borda; Time Remaining e Relógio na base.
     - **Modern Grid (Stage Pro):** Current Slide em destaque à esquerda superior, Next Slide à esquerda inferior, e coluna lateral direita com Mensagem de Palco, Relógio e Timer.
     - **Foco Vídeo / Contagem:** Live Preview central com borda em destaque e Video Countdown grande na base.
     - **Horizontal Stack:** Barra superior com mensagem, cronômetro e relógio; faixa central com slide atual em tipografia massiva; faixa inferior com próximo slide.
   - Customização de cores:
     - Cor do texto do Slide Atual (`current_text_color`)
     - Cor do texto do Próximo Slide (`next_text_color`)
     - Cor de borda do Slide Atual (`current_border_color`)
     - Cor de borda do Próximo Slide (`next_border_color`)
     - Cor do Timer (`timer_color`) e Relógio (`clock_color`).

4. **Timer Global, Relógio e Painel de Controle:**
   - Estado de timer compartilhado sincronizado em tempo real via WebSocket e REST:
     - `running` (ativo/pausado), `duration_seconds`, `remaining_seconds`, `label`, `stage_message`.
   - Efeitos visuais para contagem regressiva:
     - Alerta visual suave quando o tempo estiver próximo do fim (ex: últimos 60 segundos).
     - Quando o tempo esgotar (`remaining_seconds <= 0`), transição automática para contagem negativa em vermelho pulsante (Overtime, ex: `-02:30`), conforme o padrão ProPresenter.
   - Painel de controle dedicado na barra lateral direita do painel Admin (abaixo dos previews de monitores):
     - Mostrador digital com contagem regressiva em tempo real e relógio.
     - Botões de Iniciar, Pausar, Reiniciar (Reset) e Zerar (Stop).
     - Presets rápidos de tempo: 5 min, 10 min, 15 min, 20 min, 30 min, 45 min, 60 min e ajuste manual.
     - Campo de envio instantâneo de Mensagem de Palco (`Stage Message`).

5. **Favicon da Aplicação:**
   - Vinculação correta do favicon oficial com o logotipo do programa no cabeçalho HTML da aplicação.

### Fora do escopo
- Edição de layouts por arrasto livre de nós vetoriais em tela cheia (canvas wysiwyg livre).
- Sistema de mensageria bidirecional chat de palco (apenas mensagens do operador para o retorno de palco).

---

## 3. Decisões de Design e Arquitetura

1. **Persistência de Configurações no SQLite (Schema v6):**
   - Criação da migração `0006_app_settings.sql` com tabela de chave-valor `app_settings` para persistência robusta das preferências de Public, Marca da Igreja e Stage Display no servidor backend.
2. **Timer State Centralizado no Backend:**
   - O `TimerState` é mantido pelo servidor Axum e sincronizado com os clientes via WebSocket (`WsMessage::StateUpdated` e rotas REST dedicadas), garantindo que todos os monitores de palco e operadores vejam exatamente a mesma contagem de tempo sem desvios de relógio local.
3. **Cálculo de Overtime Negativo:**
   - Quando `remaining_seconds < 0`, a formatação exibe o sinal de menos seguido de minutos e segundos (ex: `-01:45`), disparando animações CSS de alerta (`animate-pulse text-red-500 font-bold`).

---

## 4. Critérios de Aceite

1. O operador consegue configurar preenchimento máximo, quebra de linha, margens e exibição de metadados para a tela pública nas configurações.
2. É possível realizar upload e configurar a posição e opacidade do logotipo da igreja na tela de projeção.
3. O operador pode alternar entre os 4 presets de layout do Stage e customizar as cores dos textos e bordas.
4. O painel de controle do timer permite iniciar, pausar, resetar e aplicar presets de tempo com 1 clique, refletindo no monitor de retorno em tempo real.
5. Quando o timer chega a zero, a contagem passa para negativa em vermelho pulsante indicando tempo excedido.
6. O favicon do programa carrega adequadamente no navegador.
7. Todos os testes em `cargo test --workspace` e `bun run check` passam sem regressões.

# Especificação: Integração com Google Cast (Chromecast) para Telas Pública e Retorno de Palco

## Intenção

Permitir que igrejas e eventos transmitam sem fio as telas de saída do OpenPresenter — tanto a **Tela Pública** (`/public`) quanto o **Monitor de Retorno de Palco** (`/stage`) — para televisores e projetores conectados a dispositivos **Google Cast**, **Chromecast** ou **Android TV / Google TV** conectados na mesma rede local (Wi-Fi), dispensando a necessidade de cabeamento HDMI longo e complexo.

## Escopo

- **Dentro do escopo:**
  - Carregamento sob demanda do Google Cast Web Sender SDK no frontend.
  - Controle centralizado de sessão Cast (`frontend/src/lib/cast.ts`) com estado reativo da transmissão.
  - Suporte ao Receiver Web que abre diretamente a URL `/public` ou `/stage` na TV via Chromium do Chromecast.
  - Nova interface de transmissão Cast integrada ao modal de compartilhamento de telas ([`ShareOutputsModal.svelte`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/components/ShareOutputsModal.svelte)).
  - Atalho de transmissão Cast no menu de telas do cabeçalho ([`AdminView.svelte`](file:///Users/claudio/Projects/open-presenter-minimal/frontend/src/lib/views/AdminView.svelte)).
  - Guia operacional e passo a passo de contingência para espelhamento de aba nativo do Chrome / Edge sem depender de conexão com internet externa.
  - Registro de decisão arquitetural (ADR) documentando a escolha de Web Receiver com WebSocket de latência quase zero (< 30ms) em detrimento de streaming de vídeo HLS.

- **Fora do escopo:**
  - Transmissão remota pela internet pública (o Cast opera exclusivamente em rede local Wi-Fi / LAN).
  - Codificação de vídeo HLS em tempo real no backend Rust (descartada devido ao atraso de 2 a 6 segundos e alto custo computacional).

## Decisões Técnicas

1. **Abordagem Web Receiver com WebSocket local:**
   - O Chromecast executa seu navegador Chromium embutido carregando a aplicação SPA do OpenPresenter.
   - A conexão WebSocket local (`ws://<IP_LOCAL>:18080/ws`) entrega trocas de slides instantâneas (< 30ms), sincronização milimétrica do cronômetro no palco e renderização de fontes vetoriais nítidas em 1080p sem compressão de vídeo.
2. **Carregamento Condicional do SDK:**
   - O script `cast_sender.js` é carregado de forma assíncrona sob demanda para que o aplicativo continue funcionando normalmente em ambientes 100% offline ou sem acesso aos servidores da Google.
3. **App ID Padrão e Customizável:**
   - Uso por padrão do App ID comunitário `5CB45E5A` (namespace `urn:x-cast:com.url.cast`), com envio do payload de redirecionamento `{"type": "loc", "url": "<URL_LOCAL>"}`.
   - Opção para o usuário inserir seu próprio App ID caso possua registro no Google Cast Developer Console.
4. **Fallback Nativo sem Conexão Externa:**
   - Instruções claras integradas à UI para espelhamento de aba via WebRTC nativo do Chrome (`Transmitir aba`), operando sem internet.

## Critérios de Aceite

1. O modal de compartilhamento de saídas apresenta uma aba dedicada para Chromecast / Google Cast.
2. É possível disparar a transmissão tanto para a Tela Pública (`/public`) quanto para o Monitor de Retorno de Palco (`/stage`).
3. O diálogo nativo do Google Cast abre no Chrome/Edge listando os Chromecasts detectados na rede local.
4. Ao conectar a um Chromecast, o status da sessão é exibido na interface do operador com o nome do dispositivo receptor e botão para interromper a transmissão.
5. A troca de slides no painel do operador atualiza imediatamente o Chromecast conectado via WebSocket.
6. Todos os testes automatizados da suíte (`cargo test --workspace`) e verificações de linter (`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all`) passam sem avisos ou erros.

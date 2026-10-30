# ADR 0013 — Integração com Google Cast: Web Receiver via WebSocket vs. Transmissão de Vídeo HLS

## Contexto

O OpenPresenter necessita de uma solução sem fio para projetar as telas de saída (Tela Pública `/public` e Monitor de Retorno de Palco `/stage`) em aparelhos de TV e projetores equipados com Google Cast / Chromecast na rede Wi-Fi local, eliminando custos e limitações físicas de cabeamento HDMI longo.

Foi levantada a possibilidade técnica de utilizar **HLS (HTTP Live Streaming)** para enviar a projeção ao player de mídia padrão do Chromecast.

Contudo, transmissões em cultos religiosos e eventos ao vivo exigem requisitos estritos de:
1. **Baixa latência imediata**: Ao avançar para a próxima estrofe de um louvor, a letra deve ser projetada instantaneamente para a congregação e músicos, sem atraso perceptível.
2. **Exatidão do cronômetro de palco**: O pregador e a equipe de louvor precisam de contagem de segundos em tempo real sem defasagem em relação ao painel do operador.
3. **Eficiência computacional**: O OpenPresenter deve rodar confortavelmente em notebooks comuns de operadores de igreja, sem exigir codificação contínua e intensiva de vídeo H.264/HEVC.

## Decisão

Adota-se a arquitetura de **Web Receiver** nativo em detrimento do streaming de vídeo HLS:

1. **Chromium Embutido do Chromecast:** O Google Cast executa seu ambiente de navegador web Chromium para carregar diretamente a aplicação cliente do OpenPresenter (`http://<IP_LOCAL>:18080/public` ou `/stage`).
2. **WebSocket em Tempo Real:** O Chromecast conecta-se diretamente ao canal WebSocket (`/ws`) do servidor local Axum (`op-admin`), recebendo eventos de troca de slides e tiques de timer com latência inferior a 30 milissegundos.
3. **Qualidade Vetorial Nítida:** A renderização é realizada diretamente na GPU do Chromecast com fontes CSS vetoriais nativas em 1080p, sem artefatos de compressão de vídeo em volta de textos brancos com fundo preto.
4. **Custo de CPU Zero no Servidor:** O computador do operador não realiza codificação de vídeo ou captura de tela via FFmpeg, preservando recursos do sistema.
5. **App ID Padrão com Fallback:** Utilização do App ID comunitário `5CB45E5A` para lançamento da URL, com suporte a App ID personalizado e instruções visuais para transmissão nativa de aba via WebRTC do Google Chrome.

## Consequências

- **Positivas:**
  - Latência praticamente nula (< 30ms), essencial para o ritmo de louvor ao vivo.
  - Sincronia perfeita do cronômetro de palco com o painel do operador.
  - Zero dependências de binários pesados de vídeo (como FFmpeg) no executável do OpenPresenter.
  - O Chromecast figura como um cliente conectado regular no painel de monitoramento do operador.
- **Atenção:**
  - O Chromecast e o computador do operador devem estar conectados na mesma rede local/Wi-Fi permitindo tráfego direto entre dispositivos.

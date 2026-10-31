# Especificação: Exposição e Detecção de IP na Rede Local (LAN) para Servidor e Vite

## Intenção

Permitir que o OpenPresenter seja facilmente acessível por qualquer dispositivo conectado à rede local Wi-Fi / cabeada (como celulares no Controle Remoto `/remote`, televisores/Chromecasts na Tela Pública `/public`, notebooks no OBS `/lower-third` e tablets no Monitor de Palco `/stage`), exibindo com clareza o IP de rede da máquina servidora no terminal e na interface, além de habilitar a exposição em rede do servidor Vite em modo de desenvolvimento (`host: true`).

## Escopo

- **Dentro do escopo:**
  1. **Exibição Clara do IP de Rede Local no Terminal (`crates/op-admin/src/main.rs`):**
     - O servidor Axum já efetua o bind em `0.0.0.0` (todas as interfaces de rede), porém o banner inicial impresso no terminal exibia apenas `http://localhost:<PORT>`.
     - Atualizar o banner de inicialização para exibir tanto os endereços locais (`localhost`) quanto os endereços diretos na rede local (`http://<IP_LOCAL>:<PORTA>`), discriminando os links para Painel Admin, Tela Pública, Palco, Controle Remoto, Lower Third e WebSocket.
     - Suporte à variável de ambiente `HOST` caso o operador queira forçar uma interface específica (padrão: `0.0.0.0`).
  2. **Resiliência na Descoberta do IP Local sem Internet (`crates/op-admin/src/routes.rs`):**
     - Aprimorar `get_local_ip()` para sondar múltiplos alvos (`8.8.8.8`, `1.1.1.1`, gateways privados `192.168.1.1`, `192.168.0.1`, `10.0.0.1`, `172.16.0.1` e multicast `224.0.0.1`), garantindo que o IP local seja identificado mesmo em roteadores de igreja sem conexão ativa com a internet pública.
  3. **Exposição na Rede Local no Modo Desenvolvimento (`frontend/vite.config.ts`):**
     - Configurar `server.host: true` (bind em `0.0.0.0`), permitindo que a CLI do Vite imprima e atenda conexões pelo IP da rede (`Network: http://192.168.x.x:5173/`).
     - Adicionar proxy para `/media` apontando para o backend Axum na porta 3000.
  4. **Fidelidade de IP na Interface Web (`ShareOutputsModal.svelte` e `LowerThirdView.svelte`):**
     - Ajustar `ShareOutputsModal.svelte` para priorizar o hostname da conexão caso `network-info` retorne `127.0.0.1`.
     - Dinamizar a URL do OBS em `LowerThirdView.svelte` utilizando a origem real da página.

- **Fora do escopo:**
  - Tunelamento externo pela internet pública (ngrok/Cloudflare Tunnels).

## Critérios de Aceite

1. Ao iniciar o servidor Axum, o banner do terminal exibe claramente o IP de rede local (`http://<IP_LOCAL>:<PORTA>`) para todas as saídas.
2. Em ambientes offline (sem acesso externo à internet), `get_local_ip()` obtém o IP privado da interface de rede local.
3. No frontend (`vite.config.ts`), a opção `server.host: true` permite expor o Vite na rede local com suporte a proxy de `/media`.
4. Os testes automatizados (`cargo test --workspace`), linter (`cargo clippy --workspace --all-targets -- -D warnings`) e formatação (`cargo fmt --all`) passam sem erros.

# Especificação — Detecção Multirrede (LAN, Tailscale e Externo) para Compartilhamento e /remote

## 1. Contexto e Motivação

Durante testes em ambiente real (servidor local da igreja), foi constatado que o OpenPresenter exibia o link do `/remote` com o endereço IP público/externo em vez do IP local da rede interna. Ao escanear o QR Code pelo celular conectado ao Wi-Fi da igreja, a página ficava em carregamento infinito sem conseguir conectar.

### 1.1 Diagnóstico da Causa Raiz
1. **Sonda UDP no Backend (`routes.rs:get_local_ip`)**: A função atual utilizava uma lista estática de sondas UDP iniciando por `"8.8.8.8:80"`. Quando o servidor da igreja possui rota padrão ou interface WAN conectada diretamente à internet (ex: modem bridge/PPPoE, link dedicado, interface 4G/5G ou servidor em nuvem), a tabela de rotas do SO seleciona a interface com o IP externo público.
2. **Falta de Discriminação de Sub-redes Privadas (RFC 1918)**: O filtro anterior apenas descartava `127.x.x.x` e `0.0.0.0`, aceitando qualquer IP público como "IP local".
3. **Ausência de NAT Loopback (Hairpin NAT)**: Em redes locais, roteadores residenciais ou de operadoras descartam pacotes de dispositivos internos (Wi-Fi) direcionados para o seu próprio IP público de WAN na porta 18080.
4. **Falta de Suporte a Tailscale**: Igrejas que utilizam Tailscale para operar de qualquer lugar (ex: casa ou dados móveis 4G/5G) não tinham como gerar links e QR Codes com o IP da Tailnet (`100.64.0.0/10`).

---

## 2. Metas e Requisitos

1. **Backend Rust (`crates/op-admin`)**:
   - Varrer todas as interfaces físicas e virtuais ativas do sistema utilizando `sysinfo::Networks`.
   - Classificar cada IP em:
     - `lan`: IPs privados RFC 1918 (`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`).
     - `tailscale`: IPs CGNAT na faixa oficial `100.64.0.0/10` (`100.64.x.x` a `100.127.x.x`) ou interfaces nomeadas `tailscale*`/`utun*`.
     - `external`: IPs públicos roteáveis na internet.
     - `localhost`: `127.0.0.1`.
   - Priorizar estritamente o IP de **Rede Local (LAN)** como o padrão primário (`local_ip`), garantindo que dispositivos conectados ao Wi-Fi da igreja sempre recebam um IP acessível sem risco de bloqueio por NAT.
   - Expandir a rota `GET /api/system/network-info` retrocompativelmente com a lista de interfaces (`interfaces`), `lan_ip`, `tailscale_ip` e `external_ip`.

2. **Frontend Svelte 5 (`ShareOutputsModal.svelte` e `share-outputs/*`)**:
   - Adicionar seletor visual e intuitivo de rede acima do QR Code:
     - 🟢 **Wi-Fi / Rede Local** (Padrão recomendado).
     - 🔵 **Tailscale VPN** (Aparece automaticamente se o servidor tiver Tailscale ativo).
     - 🌐 **IP Externo** (Com aviso contextual sobre necessidade de porta 18080 aberta no roteador).
     - 💻 **Host do Navegador** (Usado pelo operador para acessar o Admin).
   - O QR Code, o texto do link e o botão de copiar devem atualizar imediatamente ao trocar a rede.
   - Mensagens contextuais claras para cada tipo de rede selecionada.
   - Persistência da preferência de rede no `localStorage`.

---

## 3. Decisões de Implementação no Frontend

1. **Fonte da verdade**: `NetworkInfoResponse.interfaces` é opcional para retrocompatibilidade; quando ausente ou vazio, o modal continua usando as URLs de topo (`public_url`, `stage_url`, `remote_url`, `lower_third_url`) e nenhum seletor é exibido.
2. **Uma opção por tipo de rede**: o seletor mostra no máximo quatro chips (Rede Local, Tailscale, IP Externo, Este computador), reaproveitando a primeira interface de cada `kind` na ordem já priorizada pelo backend (LAN > Tailscale > Externo > local). Isso evita poluir o modal quando a máquina possui várias placas LAN.
3. **Seleção padrão**: o estado inicial é a sentinela `default`, resolvida para a primeira opção disponível (sempre a LAN quando existir). A escolha é persistida em `localStorage` na chave `openpresenter_selected_network`; um id que não existe mais no sistema é descartado automaticamente, voltando para `default`.
4. **Reatividade**: `activeUrl` é derivado da interface escolhida e do tipo de saída ativo; trocar de chip atualiza imediatamente o QR Code, o link exibido, o botão de copiar e o rodapé (`IP da máquina`).
5. **Acessibilidade e tema**: `role="radiogroup"` com `aria-labelledby`, chips `role="radio"` com `aria-checked`, foco itinerante (`tabindex` 0 apenas no selecionado), navegação por `ArrowLeft`/`ArrowRight`/`ArrowUp`/`ArrowDown`/`Home`/`End`, alvos de 34px, foco visível via `ring-ring` e apenas tokens semânticos (`bg-primary`, `bg-muted`, `border-border`, `text-muted-foreground`).
6. **Dicas contextuais**: texto abaixo do seletor em região `aria-live="polite"` explicando o requisito de cada rede (mesmo Wi-Fi, mesma Tailnet, porta 18080 no roteador ou uso apenas neste computador).
7. **Host do Navegador**: o tipo `localhost` permanece suportado no modelo e renderiza chip próprio se o backend um dia o devolver, mas o frontend não o sintetiza. A descoberta atual filtra loopback e, quando o Admin é aberto por `localhost`/`127.0.0.1` fora do próprio servidor, o fallback já reescreve as URLs para o hostname do navegador, evitando uma opção redundante.

---

## 4. Garantia de Qualidade

- 100% de testes passando no backend (`cargo test`).
- 0 erros no typecheck (`npm run check`) e build limpo (`npm run build`).
- Conformidade WCAG AA nos temas Claro e Escuro.
- `npx impeccable detect src --json` sem ocorrências.



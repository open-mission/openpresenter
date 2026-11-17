# Especificação — Correção do IP Exibido no QR Code do Remote em Docker

## 1. Contexto e Motivação

No servidor da igreja o OpenPresenter roda em Docker. O IP exibido no modal de
QR Code do `/remote` era `http://172.19.0.2:18080/remote`, mas o operador
acessa o painel por `http://192.168.0.231:18080`.

### 1.1 Diagnóstico da Causa Raiz

1. `GET /api/system/network-info` dependia apenas de `sysinfo::Networks`.
2. Dentro do container, a única interface IPv4 relevante é `eth0` com o IP
   interno do bridge do Docker (`172.19.0.2`), classificado como `lan` por estar
   na faixa RFC 1918 (`172.16.0.0/12`).
3. O frontend monta o QR Code e os links a partir de `local_ip`/`remote_url`
   devolvidos pela API, portanto exibia um endereço inalcançável pela rede
   local.
4. O endereço correto já chega ao backend em toda requisição: o header `Host`
   enviado pelo navegador do operador (`192.168.0.231:18080`).

## 2. Intenção

Fazer o endpoint `/api/system/network-info` priorizar o host pelo qual a
requisição chegou, removendo o IP interno do container quando houver um host de
acesso válido. Assim o QR Code e os links de saída passam a apontar para o
endereço realmente alcançável pela rede local, sem exigir configuração manual.

## 3. Escopo

### 3.1 Dentro do escopo

- `crates/op-admin/src/routes.rs`: extração do host de acesso, detecção de IP de
  bridge Docker e ajuste da lista de interfaces.
- `crates/op-admin/src/main.rs`: testes unitários e de integração cobrindo o
  comportamento.
- Documentação (spec, plano, índice e changelog).

### 3.2 Fora do escopo

- Alterações no frontend: o contrato de `NetworkInfoResponse` não muda.
- Configuração de proxy reverso, portas ou TLS.
- Suporte a host IPv6.
- Mudanças de schema ou migrações.

## 4. Comportamento Esperado

### 4.1 Extração do host de acesso

`extract_access_host(headers)` lê o header `Host`, remove a porta e devolve o
host apenas quando ele é utilizável pela rede local. É rejeitado:

- host vazio;
- `localhost` (comparação case-insensitive);
- endereços iniciados por `127.`, `169.254.` ou `0.`;
- qualquer valor contendo `[` ou `:` (IPv6 não suportado).

Sem header `Host` válido o comportamento anterior é preservado.

### 4.2 Detecção de bridge Docker

`is_docker_bridge_ip(addr)` considera bridge Docker os IPv4 com primeiro octeto
`172` e segundo octeto entre `17` e `31` (inclusive), cobrindo o intervalo
padrão de redes de containers.

### 4.3 Ajuste das interfaces

`apply_access_host(interfaces, access_host, port)`:

1. Sem host de acesso, devolve a lista original sem alterações.
2. Com host de acesso, remove interfaces cujo IP seja bridge Docker e remove
   duplicatas do próprio host.
3. Insere na posição 0 uma interface sintética `kind = "lan"`, com
   `id = "access:{host}"`, `name = "access"` e rótulo
   `Rede Local (acesso atual: {host})`.
4. Reordena a lista priorizando o host de acesso.

### 4.4 Resposta do endpoint

`get_network_info` passa a receber o `HeaderMap` da requisição:

- `local_ip` é o host de acesso quando disponível; caso contrário mantém a
  prioridade LAN > Tailscale > Externo.
- `public_url`, `stage_url`, `remote_url` e `lower_third_url` usam esse
  `local_ip`.
- `lan_ip`, `tailscale_ip` e `external_ip` continuam derivados das interfaces já
  ajustadas.

## 5. Critérios de Aceite

1. Requisição com `Host: 192.168.0.231:18080` devolve `local_ip =
   192.168.0.231` e `remote_url = http://192.168.0.231:18080/remote`.
2. A primeira interface `lan` é o host de acesso e o IP `172.19.0.2` não
   aparece na resposta.
3. Requisição sem header `Host` mantém o comportamento anterior (teste de
   integração existente continua passando).
4. Hosts `localhost`, `127.0.0.1` e IPv6 são ignorados.
5. `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e
   `cargo test --workspace` passam sem avisos.

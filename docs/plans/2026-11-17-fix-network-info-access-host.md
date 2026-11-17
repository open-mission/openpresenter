# Plano — Correção do IP Exibido no QR Code do Remote em Docker

Spec: [2026-11-17-fix-network-info-access-host.md](../specs/2026-11-17-fix-network-info-access-host.md)

## 1. Preparação

- [x] Criar a branch `fix/network-info-access-host-docker` a partir de `main`.
- [x] Registrar spec e plano em `docs/` e linkar no índice.

## 2. Backend (`crates/op-admin/src/routes.rs`)

1. [x] Importar `axum::http::HeaderMap`.
2. [x] Implementar `extract_access_host(&HeaderMap) -> Option<String>` com
   rejeição de vazio, `localhost`, loopback, link-local, `0.*` e IPv6.
3. [x] Implementar `is_docker_bridge_ip(Ipv4Addr) -> bool` para
   `172.17.0.0/16`–`172.31.0.0/16`.
4. [x] Implementar `apply_access_host(Vec<NetworkInterfaceInfo>, Option<&str>, u16)`
   removendo bridge Docker e duplicatas, inserindo a interface `access:{host}`
   na posição 0 e reordenando.
5. [x] Alterar `get_network_info` para receber `HeaderMap`, aplicar o host de
   acesso e derivar `local_ip` e as URLs de topo a partir dele.

## 3. Testes (`crates/op-admin/src/main.rs`)

- [x] Unitário `extract_access_host`: host com porta, `localhost`, loopback,
  IPv6 e ausência de header.
- [x] Unitário `is_docker_bridge_ip`: `172.19.0.2`, `172.16.0.1` e
  `192.168.0.231`.
- [x] Unitário `apply_access_host`: remove `172.19.0.2`, insere o host de acesso
  como primeira interface LAN e monta `remote_url`.
- [x] Integração `GET /api/system/network-info` com `Host:
  192.168.0.231:18080`.
- [x] Manter o teste de integração existente sem header `Host`.

## 4. Documentação

- [x] Linkar spec e plano em `docs/README.md`.
- [x] Adicionar entrada de correção em `CHANGELOG.md` (`[Unreleased]`,
  categoria `Corrigido`, sem bump de versão).

## 5. Portões e entrega

1. [x] `cargo fmt --all`
2. [x] `cargo clippy --workspace --all-targets -- -D warnings`
3. [x] `cargo test --workspace`
4. [x] Commits atômicos em Conventional Commits na branch de correção, sem
   push, merge, tag ou deploy.

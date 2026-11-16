# Plano de Execução — Detecção Multirrede (LAN, Tailscale e Externo) para /remote

Este plano orienta a implementação da especificação [`docs/specs/2026-11-16-deteccao-multirrede-lan-tailscale-externo.md`](../specs/2026-11-16-deteccao-multirrede-lan-tailscale-externo.md).

## Fases de Execução

### Fase 1: Backend Rust (`crates/op-admin`)
- [x] Definir `NetworkInterfaceInfo` e expandir `NetworkInfoResponse` em `crates/op-admin/src/routes.rs`.
- [x] Implementar `discover_network_interfaces()` usando `sysinfo::Networks::new_with_refreshed_list()`.
- [x] Implementar classificação de sub-redes (RFC 1918 para LAN, `100.64.0.0/10` para Tailscale, WAN para externo).
- [x] Garantir que o `local_ip` padrão priorize sempre a LAN privada, eliminando o problema de selecionar IP externo por engano.
- [x] Adicionar testes unitários em `crates/op-admin/src/main.rs` validando a detecção e o endpoint `GET /api/system/network-info`.
- [x] Validar com `cargo test -p op-admin`.

### Fase 2: Tipos e Controller no Frontend (`frontend/src/lib/components/share-outputs/`)
- [x] Atualizar `NetworkInfoResponse` e adicionar `NetworkInterfaceInfo` em `frontend/src/lib/types.ts` e `frontend/src/lib/components/share-outputs/types.ts`.
- [x] Criar `share-outputs/network.ts` com deduplicação por tipo, resolução da interface ativa, dicas contextuais e persistência em `localStorage`.
- [x] Atualizar `ShareOutputsController` com suporte à seleção de interface ativa (`selectedInterfaceId`), gerando `activeUrl` reativamente com base na interface escolhida.
- [x] Adicionar persistência da escolha de interface via `localStorage` (`openpresenter_selected_network`), descartando ids que não existem mais.
- [x] Refletir o IP da interface escolhida no rodapé (`IP da máquina`).

### Fase 3: Interface do Modal (`ShareOutputsQrView.svelte` & `ShareOutputsModal.svelte`)
- [x] Inserir seletor de rede acessível (chips com ícones: Wifi para LAN, Shield/VPN para Tailscale, Globe para Externo).
- [x] Atualizar dinamicamente o QR Code e o botão de copiar com base no IP selecionado.
- [x] Exibir dicas contextuais acessíveis conforme a interface (ex: "Conecte no mesmo Wi-Fi", "Necessário Tailscale ativo no celular").
- [x] Validar acessibilidade WAI-ARIA, foco e tema Claro/Escuro.

### Fase 4: Portões de Qualidade & Verificação
- [x] `cargo test -p op-db -p op-admin` (todos passando).
- [x] `cd frontend && npm run check` (0 erros e 0 warnings).
- [x] `cd frontend && npm run build` (sucesso).
- [x] `npx impeccable detect src --json` (0 problemas).
- [x] Registro das entregas no `TODO.md` e `CHANGELOG.md`.

# Plano — Telemetria de Hardware & Footer de Monitoramento do Sistema (CPU, Memória RAM e GPU)

**Data:** 2026-11-05  
**Status:** Concluído  
**Spec correspondente:** [`docs/specs/2026-11-05-footer-telemetria-sistema-cpu-ram-gpu.md`](../specs/2026-11-05-footer-telemetria-sistema-cpu-ram-gpu.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Etapas de Execução

### Etapa 1: Camada de Domínio (`crates/op-core`)
- [x] Criar `crates/op-core/src/system.rs` com struct `SystemMetrics`.
- [x] Re-exportar em `crates/op-core/src/lib.rs`.
- [x] Implementar testes unitários de serialização em `crates/op-core`.

### Etapa 2: Coleta de Telemetria e Endpoint REST (`crates/op-admin`)
- [x] Configurar dependência `sysinfo` com features `system` e `gpu` em `crates/op-admin/Cargo.toml`.
- [x] Criar módulo `crates/op-admin/src/metrics.rs` com struct `MetricsCollector`.
- [x] Adicionar instância de `MetricsCollector` em `AppState`.
- [x] Registrar rota `GET /api/system/metrics` em `crates/op-admin/src/routes.rs`.
- [x] Atualizar especificação OpenAPI em `crates/op-admin/src/api_docs.rs`.
- [x] Implementar testes de integração em `crates/op-admin`.

### Etapa 3: Interface do Usuário (Frontend Svelte 5)
- [x] Adicionar tipagem `SystemMetrics` em `frontend/src/lib/types.ts`.
- [x] Criar componente `frontend/src/lib/components/SystemMetricsFooter.svelte`:
  - [x] Chips visuais de CPU, Memória RAM e GPU com cores dinâmicas de carga.
  - [x] Indicador de conexões ativas e status do servidor.
  - [x] Popover detalhado de diagnósticos de hardware e uptime da aplicação.
  - [x] Colapso/expansão com persistência de preferência em `localStorage`.
- [x] Integrar footer em `frontend/src/lib/views/AdminView.svelte`.

### Etapa 4: Validação dos Portões de Qualidade e Documentação
- [x] Executar `cargo fmt --all`.
- [x] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Executar `cargo test --workspace`.
- [x] Executar `cd frontend && npm run build`.
- [x] Atualizar `docs/README.md`, `CHANGELOG.md` e `TODO.md`.

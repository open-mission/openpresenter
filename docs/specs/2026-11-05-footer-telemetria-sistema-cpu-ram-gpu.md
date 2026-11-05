# Spec — Telemetria de Hardware & Footer de Monitoramento do Sistema (CPU, Memória RAM e GPU)

## 1. Intenção

Fornecer aos operadores do OpenPresenter monitoramento visual em tempo real do uso de CPU, memória RAM e GPU — tanto do processo da aplicação quanto da máquina anfitriã — através de uma barra de status (*footer*) elegante, discreta e não intrusiva no painel de administração, garantindo previsibilidade e estabilidade de performance durante cultos e transmissões ao vivo.

---

## 2. Contexto e Problema

1. **Visibilidade Crítica em Apresentações ao Vivo:** Durante cultos e eventos com reprodução de vídeos, múltiplos monitores conectados (Projetor Público, Retorno de Palco, Lower Third OBS e Controles Remotos WebSockets), o consumo de recursos computacionais pode variar drasticamente.
2. **Prevenção de Quedas e Engasgos:** Sem um indicador nativo na interface, o operador não consegue diagnosticar antecipadamente se o computador está sofrendo com sobrecarga de CPU, esgotamento de memória RAM ou sobreaquecimento/estrangulamento de GPU antes que travamentos visíveis ocorram na tela do projetor.
3. **Ergonomia e Simplicidade:** O monitoramento precisa ser acessível em uma única olhada no rodapé da janela principal, sem poluir a área de trabalho dos slides e sem exigir que o operador abra o Gerenciador de Tarefas do Windows ou o Monitor de Atividades do macOS.

---

## 3. Escopo e Solução

### 3.1 Camada de Domínio (`crates/op-core`)
- Módulo `system.rs` com a struct `SystemMetrics`:
  - `process_memory_bytes` / `process_memory_mb`: memória física (RSS) consumida pelo processo do OpenPresenter.
  - `process_cpu_percent`: percentual de uso de CPU do processo da aplicação.
  - `system_memory_total_bytes` / `system_memory_total_mb`: memória RAM total do computador.
  - `system_memory_used_bytes` / `system_memory_used_mb`: memória RAM atualmente ocupada por todos os processos.
  - `system_memory_percent`: percentual de ocupação da memória RAM do sistema.
  - `system_cpu_percent`: carga global de CPU de todos os núcleos da máquina.
  - `gpu_model`: nome/modelo da placa de vídeo detectada (ex: *Apple M1*, *NVIDIA GeForce RTX 3060*, *Intel Iris Xe*).
  - `gpu_usage_percent`: percentual de processamento da GPU em tempo real (quando reportado pelo driver/sistema operacional).
  - `gpu_memory_used_mb` / `gpu_memory_total_mb`: VRAM dedicada utilizada e total (quando aplicável).
  - `uptime_seconds`: tempo de execução contínua do servidor OpenPresenter.
  - `timestamp`: timestamp Unix da leitura da telemetria.

### 3.2 Camada de Backend e Coleta de Métricas (`crates/op-admin`)
- Integração da crate `sysinfo` (com features `system` e `gpu`) no `crates/op-admin/Cargo.toml`.
- Módulo `crates/op-admin/src/metrics.rs`:
  - Estrutura `MetricsCollector` com `sysinfo::System` e `sysinfo::Gpus` para preservação do estado entre medições (cálculo de deltas de tempo para CPU e GPU).
  - Métodos thread-safe protegidos por `tokio::sync::Mutex` em `AppState`.
- Endpoint REST `GET /api/system/metrics`:
  - Retorna o JSON de `SystemMetrics`.
  - Documentado na especificação OpenAPI 3.0.3 servida em `/api/openapi.json` e visualizável no Scalar (`/docs`) e Swagger UI (`/swagger`).
- Testes unitários cobrindo a coleta e a resposta do endpoint.

### 3.3 Interface do Usuário (Frontend Svelte 5)
- Tipagem em `frontend/src/lib/types.ts` (`SystemMetrics`).
- Componente `frontend/src/lib/components/SystemMetricsFooter.svelte`:
  - Barra de status fixada no rodapé de `frontend/src/lib/views/AdminView.svelte`.
  - **Indicadores Rápidos (Chips Minimalistas):**
    - **CPU:** Ícone `Cpu`, porcentagem do processo OpenPresenter e global da máquina. Cor dinâmica (verde < 60%, âmbar 60-85%, vermelho > 85%).
    - **Memória RAM:** Ícone `HardDrive` ou `Activity`, consumo do app em MB e da máquina em GB/%.
    - **GPU:** Ícone `Monitor` / `Layers`, modelo da placa e percentual de carga gráfica.
    - **Conexões / Rede:** Quantidade de telas/remotos conectados via WebSocket e status do servidor.
    - **Versão:** Tag discreta da versão do OpenPresenter.
  - **Popover Detalhado ao Clicar:**
    - Diálogo suspenso com breakdown detalhado (número de núcleos de CPU, VRAM, tempo de atividade / uptime e sugestões de saúde da máquina).
  - **Controle de Visualização e Persistência:**
    - Botão sutil para colapsar/expandir o footer ou recolher para modo compacto, persistido em `localStorage` (`system_footer_collapsed`).
  - Intervalo de atualização eficiente a cada 2,5 segundos, com cancelamento automático quando a janela perde foco ou o componente é desmontado para não onerar o navegador.

---

## 4. Critérios de Aceite

1. O endpoint `GET /api/system/metrics` responde com código 200 e dados válidos de CPU, RAM e GPU da máquina local.
2. A especificação OpenAPI (`/api/openapi.json`, `/docs` e `/swagger`) reflete o novo endpoint de telemetria.
3. O rodapé no `AdminView.svelte` renderiza os chips de CPU, Memória e GPU sem interferir na altura das janelas de slides e setlist.
4. O consumo de CPU adicional causado pela coleta de métricas permanece inferior a 0.1%.
5. Clicar no footer abre o painel explicativo detalhado de hardware e sistema.
6. A preferência de colapso do footer é respeitada e restaurada via `localStorage`.
7. Todos os portões de qualidade passam sem ressalvas:
   - `cargo fmt --all`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cd frontend && npm run build`

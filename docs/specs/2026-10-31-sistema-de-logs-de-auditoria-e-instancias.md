# Spec — Sistema de Logs de Atividades do Programa e Rastreamento por Instância

## Contexto e Motivação

O OpenPresenter Minimal suporta múltiplos clientes simultâneos conectados à mesma rede local: operadores no painel administrativo desktop, telões públicos em projetores ou Smart TVs, monitores de palco de músicos/pastores e controle remoto em smartphones.

Com múltiplas instâncias operando simultaneamente, era necessário registrar um histórico auditável das ações executadas no sistema, identificando com precisão **qual ação foi realizada**, **quando ocorreu** e **quem realizou** (qual instância, aparelho, navegador e endereço IP).

## Intenção e Decisões

1. **Estrutura no Banco de Dados (`op-db`) — Migração 0008**:
   - Criação da tabela `action_logs`:
     - `id`: INTEGER PRIMARY KEY AUTOINCREMENT
     - `timestamp`: TEXT NOT NULL (data/hora no formato ISO 8601)
     - `action`: TEXT NOT NULL (identificador semântico em inglês, ex: `presentation_start`, `slide_change`, `song_saved`, `connection_joined`)
     - `category`: TEXT NOT NULL (`presentation`, `bibles`, `songs`, `setlists`, `timer`, `connections`, `settings`)
     - `details`: TEXT NOT NULL (descrição em português, ex: `Apresentou verso 2 de 'Porque Ele Vive'`, `Conectou Controle Remoto (iPhone)`)
     - `instance_id`: TEXT (identificador estável da instância cliente)
     - `client_type`: TEXT (`admin`, `remote`, `public`, `stage`, `system`)
     - `device_info`: TEXT (`Chrome / macOS`, `Safari / iOS`, `Firefox / Windows`, etc.)
     - `ip_address`: TEXT (endereço IP de origem)
   - Índices em `timestamp DESC` e `category`.
   - Atualização da versão do schema para `8`.

2. **Tipos no Núcleo (`op-core`)**:
   - `ActionLog` e `CreateActionLog` serializáveis via `serde`.
   - `ClientConnection` expandido com campos opcionais `instance_id` e `device_info`.
   - Mensagem WebSocket `WsMessage::ActionLogged { log: ActionLog }`.

3. **Captura e Rastreamento de Instância (`op-admin`)**:
   - Ao conectar via WebSocket ou enviar requisição HTTP, o frontend transmite `instance_id` (gerado uma vez e persistido no `localStorage`) e `device_info` (resolvido a partir de `navigator.userAgent`).
   - Métodos auxiliares em `AppState` para registrar eventos automaticamente nas rotas de apresentação, catálogo, setlists e timer.
   - Endpoint `GET /api/logs` com suporte a `limit`, `offset` e filtro opcional por `category`.
   - Endpoint `DELETE /api/logs` para limpeza de histórico quando solicitado pelo operador.

4. **Interface do Usuário (`frontend`)**:
   - Nova aba **Histórico & Logs** no modal de Configurações (`SettingsModal.svelte`), com cards detalhados por ação, filtros por categoria, badges coloridas, dispositivo de origem, horário e botão para limpar histórico.

## Critérios de Aceite

- [x] Migração `0008_action_logs.sql` cria a tabela e índices corretamente, avançando o schema para a versão 8.
- [x] Disparos de apresentação, mudanças de slide, edições de música e conexões são registrados no banco de dados com IP, tipo de cliente e aparelho.
- [x] Endpoints `/api/logs` (GET e DELETE) funcionam com testes unitários no Rust.
- [x] Aba de Logs no modal de Configurações lista os registros em tempo real e permite filtragem.
- [x] Portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test` e `npm run check`) 100% aprovados.

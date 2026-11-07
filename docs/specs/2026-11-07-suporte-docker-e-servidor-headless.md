# Spec — Suporte a Docker e Execução em Servidor Headless sem System Tray

**Data:** 2026-11-07  
**Status:** Aprovada  
**Contexto:** O OpenPresenter foi desenhado prioritariamente como aplicação desktop executada em computadores locais com bandeja do sistema (System Tray / Menu Bar via bibliotecas `tao`, `muda` e `tray-icon`). No entanto, igrejas e organizações necessitam executar o OpenPresenter em servidores Linux locais (ex: mini-PCs dedicados, servidores em rack, VPS ou contêineres Docker em rede local). Nesses ambientes, não há servidor gráfico (X11/Wayland) nem bandeja do sistema. Compilar ou executar a aplicação com dependências de GUI obrigatórias em servidores ou contêineres acarreta dependências pesadas de bibliotecas C de interface gráfica (GTK-3, libappindicator, etc.), imagens Docker desnecessariamente volumosas e possíveis falhas de runtime na inicialização de loops de eventos gráficos.

---

## 1. Intenção e Objetivos

1. **Desacoplamento Opcional do System Tray via Feature Flag:**
   - Tornar o suporte ao System Tray (`tao`, `muda`, `tray-icon`, `open`) opcional no crate `op-admin`, mantido habilitado por padrão (`default = ["tray"]`) para compilações desktop normais, mas desativável via `--no-default-features` para servidores e contêineres Docker.
2. **Execução Headless Nativa e Desempenho em Servidor:**
   - Quando compilado sem a feature `tray`, o binário deve funcionar como um servidor web Axum puro, ultraleve, sem vincular qualquer biblioteca de interface gráfica (GTK, X11, AppIndicator).
3. **Tratamento Gracioso de Sinais Unix (SIGTERM e SIGINT):**
   - O runtime assíncrono Tokio deve capturar tanto `SIGINT` (Ctrl+C) quanto `SIGTERM` (sinal padrão enviado por `docker stop` ou `systemd`), garantindo término imediato e gracioso sem bloqueio de 10 segundos no Docker, preservando a integridade das transações do SQLite.
4. **Empacotamento Oficial em Contêiner Docker Multi-Stage:**
   - Criação de `Dockerfile` multi-stage otimizado (compilação do frontend Svelte em Node/Alpine, compilação do binário Rust headless em Debian/Rust Slim e imagem final mínima e segura em Debian Bookworm Slim).
   - Criação de `docker-compose.yml` para implantação com 1 comando, incluindo persistência de dados em volume nomeado (`/data`), mapeamento da porta `18080`, política de restart e healthcheck HTTP (`GET /api/status`).
5. **Configuração Flexível por Variáveis de Ambiente:**
   - Garantir que `PORT`, `HOST`, `OPENPRESENTER_DATA_DIR` e `OPENPRESENTER_HEADLESS` funcionem perfeitamente no ambiente conteinerizado com defaults seguros (`HOST=0.0.0.0`, `PORT=18080`, `OPENPRESENTER_DATA_DIR=/data`).
6. **Segurança do Contêiner:**
   - Execução sob usuário não-root (`openpresenter:openpresenter`, UID 1000) e diretório de dados com permissões devidamente atribuídas.

---

## 2. Escopo

- **crates/op-admin/Cargo.toml:**
  - Adição da feature `tray = ["dep:muda", "dep:open", "dep:tao", "dep:tray-icon"]` com `default = ["tray"]`.
  - Configuração de dependências opcionais: `muda`, `open`, `tao`, `tray-icon`.
- **crates/op-admin/src/process.rs:**
  - Extração da função `restart_application()` do módulo `tray.rs` para permitir que ela permaneça acessível às rotas de sistema (`routes.rs`) tanto em modo desktop quanto em modo headless.
- **crates/op-admin/src/main.rs:**
  - Condicionalização do módulo `tray` (`#[cfg(feature = "tray")]`).
  - Implementação de encerramento gracioso com suporte a `SIGTERM` e `SIGINT` em Unix.
  - Ramificação limpa: quando `tray` está desabilitado, entra imediatamente em modo servidor headless aguardando sinal de encerramento.
- **Dockerfile:**
  - Build multi-stage: `frontend-builder`, `backend-builder` (com `--no-default-features`) e runtime `runner` (Debian Bookworm Slim com `ca-certificates`, `tzdata`, `curl`).
- **docker-compose.yml:**
  - Configuração de serviço com volume persistente para `/data` e mapeamento de porta `18080:18080`.
- **.dockerignore:**
  - Exclusão de arquivos temporários, `target/`, `node_modules/`, repositório `.git` e bancos de dados locais.
- **docs/:**
  - Criação da spec, plano, ADR 0015, atualização de `docs/README.md`, `docs/decisions/README.md` e `docs/architecture/overview.md`.
- **README.md e CHANGELOG.md:**
  - Documentação do modo Docker e registro no changelog.

---

## 3. Critérios de Aceite

1. **Compilação sem Tray Funcional:**
   - `cargo build --package op-admin --no-default-features` compila com sucesso sem exigir qualquer dependência de GTK ou X11.
   - `cargo test --package op-admin --no-default-features` executa todos os testes aplicáveis com sucesso.
2. **Compilação Padrão Desktop Preservada:**
   - `cargo build --package op-admin` e `cargo test --workspace` continuam compilando e passando todos os testes com a bandeja nativa habilitada.
3. **Encerramento Gracioso em Unix:**
   - O servidor responde a sinais `SIGTERM` e `SIGINT`, encerrando sem erros e liberando conexões do banco de dados SQLite.
4. **Imagem Docker Válida e Autocontida:**
   - O `Dockerfile` produz uma imagem funcional que expõe os endpoints REST e WebSocket na porta configurada, servindo o frontend SPA embutido.
   - `docker-compose.yml` mapeia a porta `18080:18080` e persiste o diretório `/data`.
5. **Portões de Qualidade:**
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run build` passam sem avisos ou erros.

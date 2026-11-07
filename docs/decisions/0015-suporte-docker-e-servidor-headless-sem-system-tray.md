# ADR 0015 — Suporte a Docker e Servidor Headless sem System Tray

## Contexto

O OpenPresenter nasceu como uma aplicação de projeção local, desktop-first, onde o binário principal (`open-presenter`) gerencia tanto o servidor web (Axum + Tokio) quanto a integração com a bandeja do sistema (System Tray / Menu Bar) por meio das crates `tao`, `muda` e `tray-icon`.

Com a expansão de uso para igrejas com infraestruturas de rede dedicadas (mini-PCs, servidores Linux em rack, VPS ou contêineres Docker em rede local), surgiu a necessidade de executar o OpenPresenter em ambientes puramente headless (sem monitores conectados, servidores gráficos X11/Wayland ou gerenciadores de janelas).

Nesses ambientes de servidor e contêiner:
1. As crates `tao`, `tray-icon` e `muda` dependem de bibliotecas C pesadas do subsistema gráfico do Linux (tais como `libgtk-3-dev`, `libayatana-appindicator3-dev`, `libx11-dev`).
2. Tentar compilar o projeto em contêineres Docker sem essas bibliotecas provoca erros de build; instalá-las infla a imagem Docker com centenas de megabytes de pacotes desnecessários para um servidor web.
3. Além disso, contêineres Docker recebem o sinal `SIGTERM` no comando `docker stop`. Se o processo não tratar `SIGTERM`, o encerramento trava por 10 segundos até o timeout forçado do Docker (`SIGKILL`), o que pode interromper operações de banco de dados SQLite WAL.

## Decisão

1. **Feature Flag `tray` Opcional e Habilitada por Padrão (`default = ["tray"]`):**
   - As dependências gráficas (`tao`, `muda`, `tray-icon`, `open`) tornam-se opcionais no `Cargo.toml` de `op-admin`, agrupadas sob a feature `tray`.
   - Compilações desktop convencionais (`cargo build`, empacotamento oficial de Windows, macOS e Linux desktop) continuam habilitando `tray` por padrão, garantindo experiência inalterada para usuários de desktop.
   - Compilações voltadas a contêineres ou servidores utilizam `cargo build --package op-admin --no-default-features`, gerando um binário Rust 100% livre de dependências de GTK, X11 ou qualquer biblioteca de GUI.

2. **Isolamento de Funções de Processo (`crates/op-admin/src/process.rs`):**
   - A rotina de reinicialização de processo `restart_application()` é isolada em módulo próprio, sem dependência de crates gráficas, permitindo que a rota HTTP `POST /api/system/restart` continue funcionando em modo headless e desktop.

3. **Encerramento Gracioso Multi-Sinal (SIGINT e SIGTERM):**
   - Implementação de escuta combinada de sinais no Unix (`tokio::signal::unix::{signal, SignalKind}`): ao receber `SIGTERM` (padrão do `docker stop`) ou `SIGINT` (Ctrl+C), o servidor realiza shutdown gracioso imediato do listener Axum e fecha as conexões do SQLite com segurança.

4. **Empacotamento em Imagem Docker Oficial Multi-Stage:**
   - **Estágio Frontend:** `node:22-alpine` para gerar os estáticos compilados do Svelte (`npm run build`).
   - **Estágio Backend:** `rust:slim-bookworm` (Rust estável) para compilar o binário em modo release com `--no-default-features`.
   - **Estágio Runtime:** `debian:bookworm-slim` enxuta, com usuário não-root `openpresenter` (UID 1000), volume persistente `/data`, healthcheck HTTP nativo e inicialização imediata.
   - Fornecimento de arquivo `docker-compose.yml` para facilitar a implantação em servidores domésticos e paroquiais.

## Consequências

- **Positivas:**
  - Imagem Docker ultraleve e eficiente, compilada rapidamente sem necessidade de dependências do GTK/X11.
  - Segurança aprimorada no contêiner com usuário não-root e volume dedicado `/data` para o banco SQLite e arquivos de mídia.
  - Parada imediata e consistente em contêineres com tratamento nativo de `SIGTERM`.
  - Zero impacto ou quebra para os usuários de desktop: o comportamento padrão com bandeja do sistema e abertura do navegador permanece exatamente o mesmo.
- **Atenção:**
  - Em contêineres Docker, o System Tray e caixas de diálogo nativas do sistema operacional (como `zenity` ou `osascript`) não são exibidos; toda administração é feita via painel web HTTP / WebSocket.

# ==============================================================================
# OpenPresenter — Dockerfile Multi-Stage (Servidor Headless)
# ==============================================================================

# ------------------------------------------------------------------------------
# Estágio 1: Compilação do Frontend SPA (Svelte 5 + TypeScript + Vite)
# ------------------------------------------------------------------------------
FROM node:22-alpine AS frontend-builder
WORKDIR /app/frontend

# Instala dependências aproveitando cache de camadas do Docker
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

# Copia o código-fonte do frontend e compila a distribuição de produção
COPY frontend/ ./
RUN npm run build

# ------------------------------------------------------------------------------
# Estágio 2: Compilação do Servidor Rust Headless (Axum + Rusqlite)
# ------------------------------------------------------------------------------
FROM rust:slim-bookworm AS backend-builder
WORKDIR /app

# Instala dependências essenciais de build
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copia descritores de workspace do Cargo
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY assets ./assets

# Copia os arquivos compilados do frontend para que o rust-embed os incorpore no binário
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

# Compila o binário open-presenter em modo release sem dependências gráficas/GUI (sem System Tray)
RUN cargo build --release --locked --no-default-features --package op-admin

# ------------------------------------------------------------------------------
# Estágio 3: Imagem Final de Execução (Runtime Minimal e Seguro)
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runner

LABEL org.opencontainers.image.title="OpenPresenter" \
      org.opencontainers.image.description="Software open source para projeção em igrejas e eventos" \
      org.opencontainers.image.licenses="GPL-3.0-only" \
      org.opencontainers.image.source="https://github.com/open-mission/openpresenter"

# Instala certificados TLS, fuso horário e curl para verificação de saúde (healthcheck)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Cria usuário não-root (UID/GID 1000) e diretório de dados persistente
RUN groupadd -g 1000 openpresenter && \
    useradd -u 1000 -g openpresenter -s /bin/sh -m openpresenter && \
    mkdir -p /data && \
    chown -R openpresenter:openpresenter /data

# Copia o binário executável autocontido gerado no estágio anterior
COPY --from=backend-builder --chown=openpresenter:openpresenter /app/target/release/open-presenter /usr/local/bin/open-presenter

USER openpresenter
WORKDIR /data

# Variáveis de ambiente padrão para execução em contêiner
ENV PORT=18080 \
    HOST=0.0.0.0 \
    OPENPRESENTER_DATA_DIR=/data \
    OPENPRESENTER_HEADLESS=1 \
    RUST_LOG=open_presenter=info,tower_http=info

# Porta padrão de escuta HTTP/WebSocket
EXPOSE 18080

# Volume persistente para o banco SQLite (open-presenter.db) e mídias (/data/media)
VOLUME ["/data"]

# Verificação contínua de integridade do servidor
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:18080/api/status || exit 1

ENTRYPOINT ["/usr/local/bin/open-presenter"]

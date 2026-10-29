# ADR 0004 — Arquitetura Web com Rust/Axum e Svelte/shadcn-svelte

- **Status:** aceito
- **Data:** 2026-10-28
- **Substitui:** a decisão de interface desktop (Slint e componentes locais) do [ADR 0001](0001-workspace-rusqlite-slint.md); adapta o modelo multi-telas do [ADR 0003](0003-saidas-multitela.md) para saídas web (`/public` e `/stage`).

## Contexto

O OpenPresenter Minimal tem como propósito a projeção de louvores, escrituras, avisos e mídias em igrejas e eventos, tendo como referências de usabilidade o ProPresenter e o Holyrics. A abordagem inicial com interface gráfica de desktop nativa (Slint com componentes Material vendorizados) impunha barreiras arquiteturais significativas:
1. Restrição de uso apenas na máquina local com monitor conectado diretamente, impedindo operação distribuída via rede local (Wi-Fi, tablets, segundo operador ou múltiplos displays em rede).
2. Complexidade de renderização e customização avançada de temas, layouts e tipografia dinâmica.
3. Necessidade de build scripts complexos (`build.rs`) e dependências binárias adicionais.

O ecossistema web moderno oferece excelente maturidade para interfaces ricas, reativas e responsivas (Svelte 5 + TypeScript + Vite + Tailwind CSS + shadcn-svelte), enquanto Rust + Axum oferece excelente performance, segurança de tipos, controle estrito de concorrência e WebSocket nativo com latência desprezível.

## Decisão

1. **Backend em Rust (`crates/op-admin`):**
   - Servidor HTTP e WebSocket construído com **Axum** e **Tokio**.
   - **Estado global em memória (`AppState`):** gerencia o item de mídia ativo, slide atual, slides da programação, estado de blackout/limpar tela e saídas ativas.
   - **Comunicação em tempo real:** canal `tokio::sync::broadcast` transmitindo mensagens JSON via WebSocket (`/ws`) para sincronizar o painel do operador (Admin) e as telas de projeção (`Public` e `Stage`).
   - **Persistência SQLite:** reutiliza `op-db` com PRAGMAs WAL, integridade referencial e migrações numeradas.
   - **Entrega de estáticos:** o Axum serve os assets compilados do frontend Svelte para acesso unificado em `http://localhost:3000`.

2. **Frontend em Svelte (`frontend/`):**
   - **Stack:** Svelte 5, TypeScript, Vite, Tailwind CSS e componentes no estilo **shadcn-svelte** (com `clsx`, `tailwind-merge` e `lucide-svelte`).
   - **Visões do sistema:**
     - `/` (ou `/admin`): Painel do operador com layout de 3 seções:
       - **Sidebar de mídias:** catálogo de itens de mídia (músicas, escrituras, comunicados) com busca e seleção.
       - **Painel central:** visualização dos slides da mídia selecionada com navegação rápida e disparo "Ao vivo".
       - **Painel de saídas e preview:** previews do Public e Stage com proporção 16:9, controles de Blackout / Limpar texto e botões para iniciar/abrir as telas de projeção.
     - `/public`: Tela de Projeção Pública limpa, sem controles visíveis, proporção 16:9 responsiva, pronta para projetores ou captura de tela em OBS.
     - `/stage`: Tela de Retorno para Palco e Apresentador, exibindo relógio digital em tempo real, data, slide atual e prévia do próximo slide.

3. **Limpeza do projeto:**
   - Remoção completa de arquivos Slint (`crates/op-admin/ui/`), scripts de compilação legados (`crates/op-admin/build.rs`) e diretórios de componentes vendorizados (`vendor/`).

## Consequências

- **Acessibilidade:** o painel e as saídas podem ser acessados em qualquer navegador moderno local ou na rede.
- **Produtividade visual:** estilos e componentes altamente customizáveis com Tailwind CSS e shadcn-svelte.
- **Desacoplamento e testabilidade:** domínio puro em `op-core`, persistência pura em `op-db`, e `op-admin` atuando como serviço robusto com APIs REST e WebSocket tipadas.
- **Portabilidade:** elimina dependências nativas de janela e driver de vídeo do binário Rust, facilitando compilação em qualquer sistema operacional.

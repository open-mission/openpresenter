# Decisões arquiteturais (ADRs)

Cada decisão arquitetural duradoura é registrada em um arquivo `NNNN-slug.md`, numerado em sequência.

## Índice

- [ADR 0001 — Workspace, rusqlite e Slint Material vendorizado](0001-workspace-rusqlite-slint.md)
- [ADR 0002 — Licença GPL-3.0-only e política de marca](0002-licenca-gpl-3.0.md)
- [ADR 0003 — Saídas Public e Stage em janelas independentes](0003-saidas-multitela.md)
- [ADR 0004 — Arquitetura Web com Rust/Axum e Svelte/shadcn-svelte](0004-arquitetura-web-axum-svelte.md)
- [ADR 0005 — Command Palette, Catálogo com SQLite FTS5 e Busca Online](0005-command-palette-e-catalogo-musicas-fts5.md)
- [ADR 0006 — Controle de Apresentação Desacoplado, Seções Customizadas e Configurações em Abas](0006-controle-de-apresentacao-secoes-customizadas-e-configuracoes.md)
- [ADR 0007 — Múltiplas Setlists, Context Menu e Armazenamento de Mídias](0007-multiplas-setlists-context-menu-e-armazenamento-de-midias.md)
- [ADR 0008 — Temas Shadcn, Pastas de Mídia Externas, Editor Markdown e Layout Flexível](0008-temas-shadcn-pastas-midia-externas-editor-markdown-e-layout-flexivel.md)
- [ADR 0009 — Presets de Stage Display, Identidade da Igreja e Timer Global com Overtime](0009-stage-presets-church-branding-e-timer-global.md)

## Formato

- **Contexto** — a situação e as forças em jogo.
- **Decisão** — o que foi escolhido.
- **Alternativas consideradas** — o que foi descartado e por quê.
- **Consequências** — o que a decisão habilita, custa ou restringe.

## Regras

- ADRs aceitos não são editados.
- Uma mudança de rumo cria um novo ADR que substitui o anterior, com referência explícita.
- O número é sequencial e não é reutilizado.

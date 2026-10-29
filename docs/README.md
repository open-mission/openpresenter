# Documentação do OpenPresenter Minimal

Índice de toda a documentação do projeto.

## Visão e fundação

- [README do projeto](../README.md)
- [Spec — Fundação técnica v0.1.0](specs/2026-10-28-fundacao-tecnica.md)
- [Plano — Fundação técnica v0.1.0](plans/2026-10-28-fundacao-tecnica.md)

## Saídas multi-tela (v0.2.0)

- [Spec — Saídas Public e Stage](specs/2026-10-28-multi-telas.md)
- [Spec — Ajuste de tela cheia e conteúdo responsivo](specs/2026-10-28-ajuste-fullscreen-e-conteudo-responsivo.md)
- [Plano — Saídas Public e Stage](plans/2026-10-28-multi-telas.md)

## Arquitetura Web — Rust/Axum + Svelte/shadcn-svelte (v0.3.0)

- [Spec — Transição para Arquitetura Web](specs/2026-10-28-arquitetura-web-axum-svelte.md)
- [Plano — Transição para Arquitetura Web](plans/2026-10-28-arquitetura-web-axum-svelte.md)
- [ADR 0004 — Arquitetura Web com Rust/Axum e Svelte/shadcn-svelte](decisions/0004-arquitetura-web-axum-svelte.md)

## Catálogo de Músicas e Command Palette (v0.4.0)

- [Spec — Command Palette, Catálogo com SQLite FTS5 e Cadastro de Músicas](specs/2026-10-28-command-palette-e-cadastro-musicas.md)
- [Plano — Command Palette, Catálogo com SQLite FTS5 e Cadastro de Músicas](plans/2026-10-28-command-palette-e-cadastro-musicas.md)
- [ADR 0005 — Command Palette, Catálogo com SQLite FTS5 e Busca Online](decisions/0005-command-palette-e-catalogo-musicas-fts5.md)

## Controle de Apresentação e Configurações (v0.5.0)

- [Spec — Controle de Apresentação, Partes da Música e Configurações em Abas](specs/2026-10-29-controle-apresentacao-e-configuracoes.md)
- [Plano — Controle de Apresentação, Partes da Música e Configurações](plans/2026-10-29-controle-apresentacao-e-configuracoes.md)
- [ADR 0006 — Controle de Apresentação Desacoplado, Seções Customizadas e Configurações em Abas](decisions/0006-controle-de-apresentacao-secoes-customizadas-e-configuracoes.md)

## Múltiplas Setlists, Context Menu e Papel de Parede (v0.6.0)

- [Spec — Múltiplas Setlists, Context Menu, Papel de Parede e Armazenamento de Mídias](specs/2026-10-29-setlists-context-menu-e-wallpapers.md)
- [Plano — Múltiplas Setlists, Context Menu, Papel de Parede e Armazenamento de Mídias](plans/2026-10-29-setlists-context-menu-e-wallpapers.md)
- [ADR 0007 — Múltiplas Setlists, Context Menu e Armazenamento de Mídias](decisions/0007-multiplas-setlists-context-menu-e-armazenamento-de-midias.md)
- [ADR 0008 — Temas Shadcn, Pastas de Mídia Externas, Editor Markdown e Layout Flexível](decisions/0008-temas-shadcn-pastas-midia-externas-editor-markdown-e-layout-flexivel.md)
- [ADR 0009 — Presets de Stage Display, Identidade da Igreja e Timer Global com Overtime](decisions/0009-stage-presets-church-branding-e-timer-global.md)

## Temas Shadcn, Pastas de Mídia, Editor Markdown e Painéis Flexíveis (v0.7.0)

- [Spec — Temas Shadcn, Pastas de Mídia Externas, Editor Markdown e Painéis Flexíveis](specs/2026-10-29-temas-pastas-midia-editor-markdown-e-paineis.md)
- [Plano — Temas Shadcn, Pastas de Mídia, Editor Markdown e Painéis Flexíveis](plans/2026-10-29-temas-pastas-midia-editor-markdown-e-paineis.md)
- [ADR 0008 — Temas Shadcn, Pastas de Mídia Externas, Editor Markdown e Layout Flexível](decisions/0008-temas-shadcn-pastas-midia-externas-editor-markdown-e-layout-flexivel.md)

## Margens do Public, Marca da Igreja, Presets de Stage e Timer Global (v0.8.0)

- [Spec — Margens do Public, Marca da Igreja, Presets de Stage e Timer Global](specs/2026-10-29-public-margins-church-brand-stage-presets-e-timer.md)
- [Plano — Margens do Public, Marca da Igreja, Presets de Stage e Timer Global](plans/2026-10-29-public-margins-church-brand-stage-presets-e-timer.md)

## Estabilidade de Configurações, Proporções no Stage e Widget de Timer no Header

- [Spec — Correção do Modal de Configurações, Proporções no Monitor de Palco e Widget de Timer no Header](specs/2026-10-29-fix-settings-stage-proportions-e-header-timer.md)
- [Plano — Correção do Modal de Configurações, Proporções no Monitor de Palco e Widget de Timer no Header](plans/2026-10-29-fix-settings-stage-proportions-e-header-timer.md)

## Ajustes de Imagens, QR Code, Timer e Otimização do Stage

- [Spec — Ajustes de Imagens, QR Code de Saídas, Timer sem Overtime Indesejado e Otimização do Monitor de Palco](specs/2026-10-29-image-display-qrcode-timer-stage-media.md)
- [Plano — Ajustes de Imagens, QR Code, Timer e Otimização do Stage](plans/2026-10-29-image-display-qrcode-timer-stage-media.md)

## Arquitetura e decisões

- [Visão da arquitetura](architecture/overview.md)
- [Índice de decisões (ADRs)](decisions/README.md)
- [ADR 0001 — Workspace, rusqlite e Slint Material](decisions/0001-workspace-rusqlite-slint.md)
- [ADR 0002 — Licença GPL-3.0-only e política de marca](decisions/0002-licenca-gpl-3.0.md)
- [ADR 0003 — Saídas Public e Stage em janelas independentes](decisions/0003-saidas-multitela.md)
- [ADR 0004 — Arquitetura Web com Rust/Axum e Svelte/shadcn-svelte](decisions/0004-arquitetura-web-axum-svelte.md)
- [ADR 0005 — Command Palette, Catálogo com SQLite FTS5 e Busca Online](decisions/0005-command-palette-e-catalogo-musicas-fts5.md)
- [ADR 0006 — Controle de Apresentação Desacoplado, Seções Customizadas e Configurações em Abas](decisions/0006-controle-de-apresentacao-secoes-customizadas-e-configuracoes.md)
- [ADR 0007 — Múltiplas Setlists, Context Menu e Armazenamento de Mídias](decisions/0007-multiplas-setlists-context-menu-e-armazenamento-de-midias.md)
- [ADR 0008 — Temas Shadcn, Pastas de Mídia Externas, Editor Markdown e Layout Flexível](decisions/0008-temas-shadcn-pastas-midia-externas-editor-markdown-e-layout-flexivel.md)
- [ADR 0009 — Presets de Stage Display, Identidade da Igreja e Timer Global com Overtime](decisions/0009-stage-presets-church-branding-e-timer-global.md)

## Marca e design

- [Branding](../assets/branding/README.md)

## Governança e licença

- [Instruções para agentes — AGENTS.md](../AGENTS.md)
- [Instruções para agentes — GEMINI.md](../GEMINI.md)
- [Política de marca](../TRADEMARK.md)
- [Licença GPL-3.0-only](../LICENSE)
- [Changelog](../CHANGELOG.md)

## Regras de documentação

1. Toda funcionalidade ou mudança relevante começa por uma spec em `docs/specs/`.
2. Mudanças com várias etapas têm um plano em `docs/plans/`, linkado à spec correspondente.
3. Decisões arquiteturais duradouras viram ADR em `docs/decisions/`; ADRs aceitos são imutáveis.
4. `docs/architecture/overview.md` acompanha a estrutura real do código.
5. `CHANGELOG.md` registra toda mudança visível ao usuário.
6. Este índice deve linkar todos os documentos.

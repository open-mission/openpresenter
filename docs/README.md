# Documentação do OpenPresenter Minimal

Índice de toda a documentação do projeto.

## Visão e fundação

- [README do projeto](../README.md)
- [Spec — Fundação técnica v0.1.0](specs/2026-10-28-fundacao-tecnica.md)
- [Plano — Fundação técnica v0.1.0](plans/2026-10-28-fundacao-tecnica.md)

## Saídas multi-tela (v0.2.0)

- [Spec — Saídas Public e Stage](specs/2026-10-28-multi-telas.md)
- [Plano — Saídas Public e Stage](plans/2026-10-28-multi-telas.md)

## Arquitetura e decisões

- [Visão da arquitetura](architecture/overview.md)
- [Índice de decisões (ADRs)](decisions/README.md)
- [ADR 0001 — Workspace, rusqlite e Slint Material](decisions/0001-workspace-rusqlite-slint.md)
- [ADR 0002 — Licença GPL-3.0-only e política de marca](decisions/0002-licenca-gpl-3.0.md)
- [ADR 0003 — Saídas Public e Stage em janelas independentes](decisions/0003-saidas-multitela.md)

## Marca e dependências

- [Branding](../assets/branding/README.md)
- [Material vendorizado](../vendor/README.md)

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

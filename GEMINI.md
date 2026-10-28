# Instruções para agentes de IA

Estas instruções são permanentes e se aplicam a todas as sessões.

O índice da documentação está em [`docs/README.md`](docs/README.md). Antes de implementar, leia a spec e o plano relevantes em `docs/specs/` e `docs/plans/`.

## Portões de qualidade

Antes de declarar uma tarefa concluída, execute:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Regras

1. Antes de implementar qualquer funcionalidade ou mudança relevante, criar ou atualizar uma spec em `docs/specs/` com intenção, escopo, decisões e critérios de aceite; para mudanças com várias etapas, criar o plano correspondente em `docs/plans/`.
2. Manter `docs/README.md` atualizado e linkando todos os documentos.
3. Registrar decisões arquiteturais duradouras em `docs/decisions/` como ADR; ADRs aceitos não são editados, são substituídos por outro.
4. Atualizar `docs/architecture/overview.md` quando a estrutura, o fluxo ou as dependências mudarem.
5. Atualizar `CHANGELOG.md` para toda mudança visível ao usuário e seguir SemVer.
6. Nunca editar uma migração já aplicada; criar uma nova migração numerada. Versão do app e versão do schema são independentes.
7. Antes de declarar uma tarefa concluída, executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.
8. Escrever documentação e textos de interface em português do Brasil; código e identificadores em inglês.
9. Respeitar a direção de dependências: `op-admin` → `op-db` → `op-core`; `op-core` não conhece UI nem SQLite.
10. Usar os assets de `assets/branding/` sem alteração; novas peças de marca precisam ser adicionadas ao README de branding.
11. Não versionar banco local, arquivos `-wal`/`-shm`, segredos ou dados de usuário.
12. Manter `AGENTS.md` e `GEMINI.md` sincronizados; em caso de divergência, `AGENTS.md` é a fonte canônica.
13. Preservar a licença GPL-3.0-only e a política de marca: forks não podem usar o nome ou a identidade visual do OpenPresenter como se fossem oficiais.

## Sincronização

`AGENTS.md` é a fonte canônica. `GEMINI.md` deve permanecer sincronizado; em caso de divergência, prevalece `AGENTS.md`.

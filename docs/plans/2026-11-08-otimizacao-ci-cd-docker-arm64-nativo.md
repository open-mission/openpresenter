# Plano de Implementação — Otimização do Pipeline Docker com Compilação Paralela Nativa ARM64 e AMD64

**Data:** 2026-11-08  
**Status:** Em execução  
**Spec de referência:** [`docs/specs/2026-11-08-otimizacao-ci-cd-docker-arm64-nativo.md`](../specs/2026-11-08-otimizacao-ci-cd-docker-arm64-nativo.md)

---

## 1. Etapas de Execução

### Etapa 1: Documentação e Índices
- [x] Criar a spec em `docs/specs/2026-11-08-otimizacao-ci-cd-docker-arm64-nativo.md`.
- [x] Criar o plano em `docs/plans/2026-11-08-otimizacao-ci-cd-docker-arm64-nativo.md`.
- [x] Atualizar `docs/README.md` com a nova spec e plano.
- [x] Adicionar tarefa no `TODO.md` sob `Doing`.

### Etapa 2: Refatoração do Workflow GitHub Actions
- [x] Atualizar `.github/workflows/docker-publish.yml`:
  - Configuração do job `build` com matrix (`ubuntu-latest` para `linux/amd64` e `ubuntu-24.04-arm` para `linux/arm64`).
  - Remoção de QEMU.
  - Setup do Docker Buildx com autenticação condicional no Docker Hub e GHCR.
  - Push por digest com escopos de cache isolados por plataforma (`type=gha,scope=build-${{ matrix.platform-id }}`).
  - Exportação e upload dos artefatos de digest.
  - Configuração do job `merge` com download de artefatos de digest, extração de metadados OCI e execução de `docker buildx imagetools create` para GHCR e Docker Hub.
  - Adição de step de inspeção com `docker buildx imagetools inspect`.

### Etapa 3: Validação, Portões de Qualidade e Registro
- [x] Atualizar `CHANGELOG.md` na seção `[Unreleased]`.
- [x] Executar portões de qualidade: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- [x] Atualizar `TODO.md` sob `Done` com commit e data.

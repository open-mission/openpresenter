# Plano de Implementação — Pipeline CI/CD de Publicação Automática de Imagens Docker

**Data:** 2026-11-07  
**Status:** Em execução  
**Spec de referência:** [`docs/specs/2026-11-07-ci-cd-publicacao-docker-hub.md`](../specs/2026-11-07-ci-cd-publicacao-docker-hub.md)

---

## 1. Etapas de Execução

### Etapa 1: Documentação e Índices
- [x] Criar a spec em `docs/specs/2026-11-07-ci-cd-publicacao-docker-hub.md`.
- [x] Criar o plano em `docs/plans/2026-11-07-ci-cd-publicacao-docker-hub.md`.
- [x] Atualizar `docs/README.md` com a nova spec e plano.
- [x] Adicionar tarefa no `TODO.md` sob `Doing`.

### Etapa 2: Implementação do Workflow GitHub Actions
- [x] Criar o arquivo `.github/workflows/docker-publish.yml`:
  - Configuração de triggers (`push tags v*`, `workflow_dispatch`).
  - Permissões (`packages: write`, `contents: read`).
  - Configuração de QEMU e Buildx.
  - Autenticação condicional no Docker Hub e GitHub Container Registry.
  - Extração de tags SemVer e labels OCI.
  - Build multi-plataforma (`linux/amd64,linux/arm64`) e push com cache GHA.

### Etapa 3: Instruções de Uso e Validação
- [x] Atualizar o `README.md` com instruções detalhadas para configurar os segredos `DOCKERHUB_USERNAME` e `DOCKERHUB_TOKEN` no GitHub.
- [x] Atualizar o `CHANGELOG.md` na seção `[Unreleased]`.
- [x] Executar os portões de qualidade (`cargo fmt --all`, `cargo clippy`, `cargo test`).
- [ ] Atualizar `TODO.md` e realizar commits no padrão GitFlow.

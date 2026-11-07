# Spec — Pipeline CI/CD de Publicação Automática de Imagens Docker (Docker Hub & GHCR)

**Data:** 2026-11-07  
**Status:** Aprovada  
**Contexto:** Com a introdução do `Dockerfile` multi-stage e do modo headless no OpenPresenter, é necessário disponibilizar um pipeline de automação (CI/CD) no GitHub Actions para compilar imagens multi-arquitetura (`linux/amd64` e `linux/arm64`) e publicá-las automaticamente no Docker Hub e no GitHub Container Registry (`ghcr.io`) sempre que uma nova tag de versão for criada ou sob demanda manual.

---

## 1. Intenção e Objetivos

1. **Automação Completa via GitHub Actions:**
   - Criar o workflow `.github/workflows/docker-publish.yml` acionado por criação de tags (`v*`) e por disparo manual (`workflow_dispatch`).
2. **Suporte Multi-Plataforma Nativo (`linux/amd64` e `linux/arm64`):**
   - Utilizar Docker Buildx e QEMU para compilar e empacotar imagens para servidores x86_64 tradicionais e processadores ARM64 (AWS Graviton, Oracle Cloud Ampere, Raspberry Pi 4/5 e Apple Silicon), publicando manifestos unificados.
3. **Publicação Dupla (Docker Hub e GitHub Packages / GHCR):**
   - Publicar no Docker Hub (`<usuario>/openpresenter`) autenticado via segredos `DOCKERHUB_USERNAME` e `DOCKERHUB_TOKEN`.
   - Publicar adicionalmente no GitHub Container Registry (`ghcr.io/<org>/openpresenter`) aproveitando as permissões nativas de `GITHUB_TOKEN`.
4. **Versionamento Semântico e Tags Automáticas:**
   - Extrair metadados e tags OCI padronizados via `docker/metadata-action`:
     - Tag exata (ex: `0.5.1`)
     - Tag menor (ex: `0.5`)
     - Tag `latest` para a versão estável
     - Tags manuais customizadas via `workflow_dispatch`.
5. **Otimização de Cache de Build:**
   - Empregar cache nativo do GitHub Actions (`type=gha`) para acelerar compilações incrementais de Rust e Node.js.

---

## 2. Escopo

- **.github/workflows/docker-publish.yml:**
  - Definição do job `docker` com `checkout`, `setup-qemu`, `setup-buildx`, `login-action` (Docker Hub e GHCR), `metadata-action` e `build-push-action`.
- **docs/:**
  - Criação de especificação (`docs/specs/2026-11-07-ci-cd-publicacao-docker-hub.md`) e plano (`docs/plans/2026-11-07-ci-cd-publicacao-docker-hub.md`).
  - Atualização do índice em `docs/README.md`.
- **README.md e CHANGELOG.md:**
  - Documentação da configuração dos segredos do repositório (`DOCKERHUB_USERNAME` e `DOCKERHUB_TOKEN`) e registro no changelog.
- **TODO.md:**
  - Registro de progresso e commit no GitFlow.

---

## 3. Critérios de Aceite

1. **Workflow Válido e Declarativo:**
   - O arquivo YAML em `.github/workflows/docker-publish.yml` possui sintaxe válida de GitHub Actions.
2. **Compatibilidade Multi-Arch:**
   - O pipeline está configurado para as plataformas `linux/amd64` e `linux/arm64`.
3. **Resiliência e Segurança de Credenciais:**
   - O login no Docker Hub é executado apenas quando `DOCKERHUB_USERNAME` estiver definido nos segredos, evitando quebras desnecessárias em forks.
   - Segredos nunca são expostos em logs.
4. **Portões de Qualidade:**
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` permanecem 100% aprovados.

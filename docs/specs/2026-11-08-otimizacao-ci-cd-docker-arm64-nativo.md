# Spec — Otimização do Pipeline Docker com Compilação Paralela Nativa ARM64 e AMD64

**Data:** 2026-11-08  
**Status:** Aprovada  
**Contexto:** O pipeline de publicação de imagens Docker multi-arquitetura (`.github/workflows/docker-publish.yml`) executava a compilação de ambas as arquiteturas (`linux/amd64` e `linux/arm64`) em um único runner `ubuntu-latest` x86_64 utilizando emulação de software via QEMU. Devido à natureza intensiva de CPU e compilação multithreaded de Rust com LLVM em modo release (`cargo build --release` com 311 dependências), a compilação ARM64 sob emulação QEMU em runner de 2 vCPUs ultrapassou 1h30m de execução. Esta especificação define a transição para compilação nativa paralela utilizando os novos runners ARM64 do GitHub Actions e montagem de manifesto multi-arch via Docker Buildx Imagetools.

---

## 1. Intenção e Objetivos

1. **Eliminar Gargalo de Emulação de Software (QEMU):**
   - Substituir a compilação ARM64 emulada por compilação nativa em hardware ARM64 dedicado (`runs-on: ubuntu-24.04-arm`), que dispõe de 4 vCPUs e 16 GB de RAM gratuitos para repositórios públicos.
2. **Execução Paralela Distribuída (Matrix Strategy):**
   - Executar os builds das plataformas `linux/amd64` (em `ubuntu-latest`) e `linux/arm64` (em `ubuntu-24.04-arm`) simultaneamente e de forma independente.
3. **Isolamento de Cache de Build por Arquitetura:**
   - Configurar escopos independentes de cache do GitHub Actions (`type=gha,scope=build-${{ matrix.platform }}`), evitando contaminação cruzada de artefatos entre arquiteturas.
4. **Montagem Atômica de Manifestos OCI Multi-Arch (Imagetools Create):**
   - Publicar cada plataforma por digest imutável (`push-by-digest=true`).
   - Coletar os digests através de artefatos temporários e uni-los em um job de merge leve (`merge`) utilizando `docker buildx imagetools create`.
5. **Redução Drástica do Tempo Total de Build:**
   - Reduzir o tempo de execução do pipeline de > 90 minutos para aproximadamente 4 a 6 minutos.
6. **Preservação de Compatibilidade e Resiliência de Registries:**
   - Manter a publicação dual e transparente no GitHub Container Registry (`ghcr.io`) e no Docker Hub (quando as credenciais estiverem configuradas).

---

## 2. Escopo

- **`.github/workflows/docker-publish.yml`:**
  - Substituição do job monolítico `docker` por dois jobs encadeados:
    1. `build`: matriz com `linux/amd64` (`ubuntu-latest`) e `linux/arm64` (`ubuntu-24.04-arm`), push por digest e exportação de artefatos de digest.
    2. `merge`: download de artefatos, extração de metadados SemVer e montagem dos manifestos multi-plataforma via `docker buildx imagetools create` no GHCR e Docker Hub.
- **`docs/`:**
  - Criação desta spec (`docs/specs/2026-11-08-otimizacao-ci-cd-docker-arm64-nativo.md`).
  - Criação do plano de implementação (`docs/plans/2026-11-08-otimizacao-ci-cd-docker-arm64-nativo.md`).
  - Atualização do índice em `docs/README.md`.
- **`TODO.md` e `CHANGELOG.md`:**
  - Registro de tarefas no `TODO.md` e atualização do `CHANGELOG.md` na seção `[Unreleased]`.

---

## 3. Critérios de Aceite

1. **Ausência de Emulação QEMU:**
   - O pipeline não depende mais de `docker/setup-qemu-action`, eliminando todo o overhead de emulação.
2. **Sintaxe Válida e Declarativa:**
   - O arquivo YAML é validado e estruturado conforme o padrão oficial do Docker para GitHub Actions.
3. **Execução Nativa em ARM64:**
   - A plataforma `linux/arm64` roda nativamente no runner `ubuntu-24.04-arm`.
4. **Manifesto Multi-Arquitetura Íntegro:**
   - A inspeção do manifesto (`imagetools inspect`) comprova a existência de ambas as arquiteturas vinculadas à mesma tag (ex: `latest`, `v0.6.0`).
5. **Portões de Qualidade:**
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` permanecem 100% aprovados.

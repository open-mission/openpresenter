# Plano — Exportação e Importação Completa de Dados da Aplicação (Backup & Migração)

**Data:** 2026-11-06  
**Status:** Em execução  
**Spec relacionada:** [Spec — Exportação e Importação Completa de Dados](docs/specs/2026-11-06-exportar-e-importar-dados-backup.md)  
**Abordagem:** Desenvolvimento incremental passo a passo (um item de cada vez), com validação contínua e garantia dos portões de qualidade.

---

## Fases de Execução

### Item 1: Métodos de Backup e Restore no Banco de Dados SQLite (`op-db`)
- [x] Implementar `Database::backup_to_file(&self, target_path: &Path) -> Result<(), DbError>` usando `rusqlite::backup::Backup`.
- [x] Implementar `Database::restore_from_file(&mut self, source_path: &Path) -> Result<(), DbError>` com checagem de integridade (`PRAGMA integrity_check`) e execução de migrações automáticas.
- [x] Adicionar testes unitários em `op-db` validando backup e restore atômico entre arquivos.
- [x] Rodar `cargo test -p op-db` e atualizar documentação/TODO.

### Item 2: Modelagem de Dados no Domínio e Módulo de Empacotamento ZIP (`op-core` e `op-admin`)
- [x] Criar estruturas `BackupManifest`, `BackupStats`, `BackupInspectResult`, `BackupRestoreResult` em `crates/op-core/src/lib.rs`.
- [x] Criar módulo `crates/op-admin/src/backup.rs` responsável por:
  - Gerar o manifesto `manifest.json`.
  - Criar o pacote compactado `.opbackup` (ZIP com compressão Deflate).
  - Incluir banco snapshot e pastas de arquivos (`media/`, `bibles/`, `fonts/`).
  - Inspecionar pacote ZIP existente e extrair metadados para validação.
  - Extrair e restaurar arquivos com proteção rigorosa contra Zip Slip.
- [x] Adicionar testes unitários cobrindo empacotamento, integridade e sanitização de caminhos.

### Item 3: Endpoints REST na API Axum (`op-admin`)
- [x] Adicionar rota `GET /api/backup/export` com streaming do arquivo `.opbackup` e opções de inclusão via query params.
- [x] Adicionar rota `POST /api/backup/inspect` para upload temporário e retorno das estatísticas do backup antes da confirmação.
- [x] Adicionar rota `POST /api/backup/import` para restauração do banco e arquivos com recarregamento em memória do `AppState` e notificação WebSocket.
- [x] Atualizar documentação da OpenAPI / Swagger offline em `crates/op-admin/src/api_docs.rs`.
- [x] Adicionar testes de integração das rotas.

### Item 4: Interface do Usuário no Painel de Configurações (`frontend`)
- [x] Adicionar seção "Backup & Migração" na aba "Arquivos & Mídias" do `SettingsModal.svelte`.
- [x] Implementar Card de Exportação com botão de download e opções de seleção.
- [x] Implementar Card de Importação com seletor de arquivo `.opbackup`/`.zip`.
- [x] Implementar diálogo/modal de inspeção prévia exibindo estatísticas (total de músicas, setlists, imagens, etc.) e confirmação de restauração.
- [x] Feedback visual de progresso, notificação de sucesso e atualização da página.
- [x] Executar `npm --prefix frontend run build`.

### Item 5: Documentação Final, Overview da Arquitetura e CHANGELOG
- [ ] Atualizar `docs/architecture/overview.md` detalhando o fluxo de exportação e restauração.
- [ ] Atualizar `CHANGELOG.md` com as novas capacidades de migração e backup.
- [ ] Concluir o item em `TODO.md` com commits e referências organizadas.
- [ ] Executar todos os portões de qualidade: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.

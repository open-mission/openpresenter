# Spec — Exportação e Importação Completa de Dados da Aplicação (Backup & Migração)

**Data:** 2026-11-06  
**Status:** Aprovada  
**Contexto:** Usuários do OpenPresenter necessitam transferir instalações completas entre computadores (ex: máquina pessoal do operador para o notebook oficial da igreja, migrações de sistema operacional ou recuperação de desastres). Isso requer empacotar de forma integral e consistente tanto os dados relacionais do SQLite (`open-presenter.db`) quanto os arquivos físicos de mídia importados pelo usuário (imagens, vídeos, áudios, wallpapers customizados, marca da igreja, bíblias e fontes).

---

## 1. Intenção e Objetivos

1. **Backup Completo e Portátil (.opbackup / .zip):**
   - Permitir a exportação de todos os dados do OpenPresenter em um único arquivo compactado com compressão Deflate, contendo um manifesto padronizado (`manifest.json`), uma cópia snapshot consistente do banco SQLite (`database/open-presenter.db`) e os diretórios de mídia (`media/images`, `media/wallpapers`, `media/videos`, `media/audios`, `media/church_brand`), bíblias (`bibles`) e fontes (`fonts`).
2. **Snapshot Atômico do Banco de Dados SQLite sem Corrupção:**
   - Empregar a API de Online Backup do SQLite (`rusqlite::backup::Backup`) para exportar e restaurar o banco de dados em tempo de execução sem dependência ou travamento de arquivos `-wal` e `-shm`, garantindo integridade ACID e validação preventiva com `PRAGMA integrity_check`.
3. **Segurança e Proteção contra Zip Slip:**
   - Durante a importação, sanitizar rigorosamente todos os caminhos dos arquivos do arquivo compactado contra ataques de *directory traversal* (Zip Slip), recusando entradas relativas maliciosas (`..`) ou caminhos absolutos.
4. **Inspeção Prévia e Restauração Segura:**
   - Disponibilizar rota de inspeção preliminar (`POST /api/backup/inspect`) que valida o arquivo, verifica compatibilidade de versões e schemas, e lista as estatísticas do pacote (número de músicas, setlists, membros, imagens, vídeos, etc.) antes da confirmação do usuário.
   - Fornecer rota de importação (`POST /api/backup/import`) que aplica o snapshot ao banco SQLite, extrai os arquivos físicos, sincroniza o estado da memória (`AppState`, setlist ativa, saídas) e transmite sinal via WebSocket para atualização imediata dos clientes conectados.
5. **Interface de Usuário Amigável em Configurações:**
   - Adicionar seção "Backup & Migração" na aba "Arquivos & Mídias" do modal de configurações (`SettingsModal.svelte`), com cards claros para Exportar Dados (com download de 1 clique) e Importar Dados (com seletor de arquivo, resumo estatístico do pacote e confirmação preventiva).

---

## 2. Escopo

- **op-db:**
  - Método `Database::backup_to_file(&self, target_path: &Path) -> Result<(), DbError>` usando `rusqlite::backup::Backup`.
  - Método `Database::restore_from_file(&mut self, source_path: &Path) -> Result<(), DbError>` com validação de `PRAGMA integrity_check`, verificação de versão de schema e aplicação de migrações automáticas.
- **op-core:**
  - Tipos e estruturas de dados de backup: `BackupManifest`, `BackupStats`, `BackupInspectResult` e `BackupRestoreResult`.
- **op-admin:**
  - Módulo `src/backup.rs` com utilitários para criar o arquivo compactado (`export_backup_package`) e inspecionar/restaurar (`inspect_backup_package`, `restore_backup_package`).
  - Rotas REST:
    - `GET /api/backup/export` (stream do arquivo `.opbackup`).
    - `POST /api/backup/inspect` (análise sem efeitos colaterais).
    - `POST /api/backup/import` (restauração segura com atualização de estado).
- **frontend:**
  - Interface na aba "Arquivos & Mídias" em `SettingsModal.svelte`:
    - Card de Exportação com opções de seleção de conteúdo e botão de download direto.
    - Card de Importação com seletor de arquivo `.opbackup` ou `.zip`, modal de confirmação com resumo das estatísticas e barra de progresso.

---

## 3. Critérios de Aceite

1. **Atomicidade e Consistência do Banco:**
   - O backup do banco SQLite é gerado sem erros mesmo com a aplicação em execução ativa.
   - A restauração substitui o banco em memória/disco via SQLite Backup API e executa as migrações caso o backup seja de versão anterior.
   - Backups com SQLite corrompido ou com schema superior ao do binário atual são rejeitados com mensagem explicativa em pt-BR.
2. **Empacotamento de Mídias e Arquivos:**
   - Todas as imagens, vídeos, áudios, papéis de parede, logotipo da igreja, bíblias e fontes contidos no diretório de dados são exportados e preservados na restauração.
   - Links relativos de mídias (`/api/media/files/...`) permanecem 100% funcionais após a restauração.
3. **Segurança:**
   - Arquivos contendo caminhos com `..` ou barras iniciais no ZIP são rejeitados ou sanitizados com segurança.
4. **Portões de Qualidade:**
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run build` passam com sucesso absoluto.

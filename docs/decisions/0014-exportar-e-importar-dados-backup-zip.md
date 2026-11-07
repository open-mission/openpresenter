# ADR 0014 — Exportação e Importação de Dados: Pacote ZIP com SQLite Online Backup API

## Contexto

O OpenPresenter armazena dados relacionais estruturados em um banco de dados SQLite (`open-presenter.db`), configurado em modo Write-Ahead Logging (`WAL`), e arquivos de mídia desestruturados no sistema de arquivos local (`<data_dir>/media/images`, `videos`, `audios`, `wallpapers`, `church_brand`), módulos bíblicos (`bibles/`) e fontes customizadas (`fonts/`).

Para permitir que usuários migrem facilmente uma instalação existente para outro computador ou realizem cópias de segurança (backups), é necessário:
1. Copiar o banco SQLite sem risco de inconsistência, pois arquivos em modo WAL podem ter transações pendentes no arquivo `-wal` e `-shm`, e uma simples cópia de arquivo bruto no sistema operacional (especialmente no Windows) pode falhar devido a bloqueios de arquivo (file locks).
2. Empacotar os arquivos de mídia de modo compacto e padronizado, sem exigir utilitários externos ou bibliotecas com dependências pesadas de C.
3. Garantir segurança absoluta contra vulnerabilidades de descompactação maliciosa (*Zip Slip* / Directory Traversal).
4. Suportar restauração atômica na aplicação em execução com migração automática caso o backup provenha de uma versão anterior.

## Decisão

1. **SQLite Online Backup API (`rusqlite::backup::Backup`):**
   - Para exportação, o banco é gravado através da API de backup online do SQLite, criando um arquivo `.db` consistente, totalmente comitado e autocontido (sem dependência de `-wal` ou `-shm`), mesmo com o servidor e conexões ativas.
   - Para restauração, o arquivo importado é primeiro validado com `PRAGMA integrity_check` e sua versão de schema checada; a seguir, a conexão ativa do `Database` tem suas páginas substituídas via SQLite Backup API e as migrações mais recentes são aplicadas automaticamente (`migrations.to_latest`).
2. **Formato do Pacote: Arquivo ZIP (.opbackup) com Compressão Deflate:**
   - Adota-se o formato ZIP padronizado com compressão Deflate utilizando a crate `zip` já presente no workspace. O arquivo recebe a extensão amigável `.opbackup` (ou `.zip`).
   - A raiz do pacote contém:
     - `manifest.json`: Metadados da versão do app, versão do schema, timestamp de criação e contagens de entidades (músicas, setlists, membros, imagens, etc.).
     - `database/open-presenter.db`: Arquivo SQLite integral.
     - `media/`: Diretórios de imagens, wallpapers, vídeos, áudios e identidade da igreja.
     - `bibles/`: Bíblias SQLite adicionadas.
     - `fonts/`: Arquivos de fonte instalados.
3. **Defesa em Profundidade contra Zip Slip:**
   - Todo arquivo descompactado tem seu caminho canônico sanitizado; caminhos contendo componentes reversos (`..`), caminhos absolutos ou tentativas de escape do diretório base são estritamente rejeitados com erro.
4. **Fluxo em Duas Etapas com Inspeção Prévia:**
   - Disponibilização de endpoint de inspeção (`POST /api/backup/inspect`) para validar integridade e exibir um resumo quantitativo ao usuário na interface antes de prosseguir com a substituição do banco e arquivos.

## Consequências

- **Positivas:**
  - Zero risco de corrupção ou travamento de arquivo no Windows, Linux e macOS graças à API nativa de backup do SQLite.
  - Compatibilidade reversa: backups gerados em versões anteriores do OpenPresenter são automaticamente migrados para o schema atual ao serem importados.
  - O arquivo gerado é portátil, pode ser armazenado em pendrives, nuvem ou compartilhado entre membros da equipe da igreja.
  - Formato aberto e inspecionável: caso necessário, o arquivo `.opbackup` pode ser aberto com qualquer descompactador ZIP padrão.
- **Atenção:**
  - Ao restaurar um backup, os dados locais ativos do banco são substituídos pelo conteúdo do backup. Por isso, a interface deve exibir um alerta preventivo e detalhes da inspeção antes da execução definitiva.

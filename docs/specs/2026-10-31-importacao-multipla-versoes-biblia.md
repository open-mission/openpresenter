# Spec: Importação de Múltiplas Versões da Bíblia de uma Vez

## Status
Aprovada

## Intenção
Permitir ao operador selecionar e importar múltiplos arquivos de versões da Bíblia (`.sqlite`, `.db`, `.sqlite3`) simultaneamente através do diálogo nativo do sistema de arquivos, processando a leitura, validação e persistência sequencialmente ("um a um") com feedback de progresso em tempo real e notificações Sonner.

## Escopo
1. **Seleção Múltipla de Arquivos**:
   - Adicionar o atributo `multiple` e extensão `.sqlite3` ao input de arquivos em `SettingsModal.svelte`: `<input type="file" multiple accept=".sqlite,.db,.sqlite3" />`.
   - Ajustar texto de botões e descrições para indicar suporte a uma ou múltiplas versões ("Importar Bíblias (.sqlite)").

2. **Fila de Importação Sequencial ("um a um")**:
   - Loop assíncrono sequencial (`for ... of`) que processa cada arquivo individualmente:
     - Leitura do arquivo em Base64.
     - Disparo da requisição `POST /api/bibles/import`.
     - Atualização do estado de progresso (`current`, `total`, `currentFileName`, `successCount`, `failedCount`).
     - Emissão de feedback com toast Sonner para cada Bíblia importada com sucesso.
     - Em caso de falha em um arquivo específico (ex: schema inválido ou corrompido), registrar o erro sem interromper a importação dos demais arquivos da fila.
   - Sincronização automática com a store reativa de bíblias (`loadBibles()` e `refreshBibles()`).

3. **Suporte a Múltiplos Caminhos Locais**:
   - No formulário "Por Caminho...", permitir que o operador cole múltiplos caminhos absolutos separados por vírgula, ponto-e-vírgula ou quebras de linha, processando cada um sequencialmente.

4. **Interface de Progresso em Tempo Real**:
   - Exibição de card estilizado com barra de progresso percentual, contagem de arquivos e nome do arquivo atualmente em processamento.
   - Resumo consolidado ao término do lote.

5. **Garantia de Identificadores Únicos no Backend (`op-admin`)**:
   - Utilização de contador atômico sequencial `BIBLE_IMPORT_COUNTER` no gerador de ID da versão (`bible-{timestamp}-{counter}`), impedindo colisões de nomes de arquivos físicos e registros no SQLite durante requisições sequenciais quase simultâneas.

## Critérios de Aceite
1. O seletor de arquivos de Bíblia aceita seleção múltipla via Shift ou Ctrl/Cmd.
2. Os arquivos selecionados são importados de forma sequencial com exibição de barra de progresso.
3. Versões válidas são registradas no banco e listadas imediatamente na tabela de versões instaladas e no seletor do `BiblePanel`.
4. Se um dos arquivos for inválido, os demais continuam sendo processados e os erros são reportados com clareza.
5. Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

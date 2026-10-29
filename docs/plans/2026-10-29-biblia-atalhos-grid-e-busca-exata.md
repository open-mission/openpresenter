# Plano de Execução: Bíblia — Atalhos Globais, Visualização em Grade, Ativação de Versão e Busca Exata

## Etapas de Execução

### Etapa 1: Backend Rust (`op-core`, `op-db`, `op-admin`)
1. **`crates/op-core/src/bible.rs`:**
   - Adicionar campo opcional `version: Option<String>` na struct `ParsedReference`.
   - Implementar detecção de versão bíblica no final dos tokens em `parse_bible_reference` (ex: `Salmos 91 ARA`, `João 3:16 NVI`).
   - Adicionar testes unitários para formatos com versão.
2. **`crates/op-db/src/lib.rs`:**
   - Implementar função de verificação de palavra exata com limites alfanuméricos unicode (`matches_whole_word`).
   - Atualizar `search_verses` para aceitar `exact: bool`, suportando busca exata e sintaxe com aspas `"..."`.
   - Adicionar testes unitários garantindo que `"amor"` não encontre `"amorreus"`.
3. **`crates/op-admin/src/routes.rs`:**
   - Atualizar a query struct `BibleSearchQuery` para suportar `exact: Option<bool>`.
   - Tratar termos com aspas automaticamente como busca exata.

### Etapa 2: Parser Frontend e Types
1. **`frontend/src/lib/types.ts`:**
   - Adicionar `version?: string` em `ParsedReference`.
2. **`frontend/src/lib/bibleParser.ts`:**
   - Atualizar `parseBibleReference` para extrair versão bíblica ao final de referências (ex: `Salmos 91 ARA`).

### Etapa 3: Command Palette e Atalhos de Teclado
1. **`frontend/src/lib/components/CommandPalette.svelte`:**
   - Adicionar ação rápida "Abrir Bíblia Sagrada" com ícone `BookOpen` e atalho `⌘B` / `F4`.
   - Expor callback `onopenbibletab?: () => void`.
2. **`frontend/src/lib/views/AdminView.svelte`:**
   - Adicionar atalho global de teclado `(Ctrl+B / ⌘B)` e `F4` para abrir a aba Bíblia no `BottomPanel`.
   - Conectar callback `onopenbibletab` do `CommandPalette`.
   - Ajustar atalhos de saída para não colidirem com o novo atalho de Bíblia.

### Etapa 4: Painel da Bíblia (`BiblePanel.svelte`)
1. **Grade de livros com siglas:**
   - Renderizar em grade (`grid grid-cols-3 sm:grid-cols-4 gap-1 p-1`) quando `bookViewMode === 'abbreviations'`.
2. **Largura da coluna de capítulos:**
   - Aumentar largura padrão de 140px para 170px e mínimo de 110px.
3. **Ativação automática de versão:**
   - Quando `parsed.version` for fornecida e existir entre as bíblias instaladas, trocar `selectedBibleId` imediatamente.
4. **Input group e busca de palavra exata:**
   - Adicionar botão toggle de palavra exata (`[Ab]`) acoplado no campo de busca.
   - Suportar busca com aspas `"amor"`.
   - Enviar parâmetro `exact=true` na API.

### Etapa 5: Validação e Portões de Qualidade
1. Executar `cargo fmt --all`.
2. Executar `cargo clippy --workspace --all-targets -- -D warnings`.
3. Executar `cargo test --workspace`.
4. Executar `bun run build`.
5. Atualizar `CHANGELOG.md` e `docs/README.md`.

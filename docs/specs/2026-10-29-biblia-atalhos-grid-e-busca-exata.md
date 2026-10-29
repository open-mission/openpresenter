# Especificação Técnica: Bíblia — Atalhos Globais, Visualização em Grade, Ativação de Versão e Busca Exata

## 1. Intenção e Motivação

Aprimorar a experiência operacional do módulo de Bíblias Sagradas com foco em agilidade no culto ao vivo:
1. Permitir que o operador abra a aba de Bíblia instantaneamente via teclado (`Ctrl+B` / `⌘B` ou `F4`) e pelo `CommandPalette` (⌘K).
2. Facilitar a seleção de livros na aba de Bíblia quando as abreviações/siglas estiverem ativadas, exibindo-os em formato de grade (grid) em vez de lista vertical.
3. Aumentar a largura do painel de capítulos para navegação mais espaçosa e confortável.
4. Suportar indicação de versão bíblica diretamente no campo de busca (ex: `Salmos 91 ARA` ou `João 3:16 NVI`), ativando a versão instalada imediatamente.
5. Adicionar opção de busca textual exata (whole word) para evitar correspondências parciais indesejadas (ex: `amor` não trazer `Amorreus`), acessível via botão toggle no input group e sintaxe com aspas (`"amor"`).

---

## 2. Escopo

### Backend (`op-core` e `op-db` e `op-admin`)
- Atualizar `ParsedReference` para incluir campo opcional `version: Option<String>`.
- Atualizar parser de referências bíblicas (`parse_bible_reference`) para extrair a sigla/versão bíblica caso informada (ex: `Salmos 91 ARA`, `Jo 3:16 NVI`, `1 Co 13:4-7 arc`).
- Atualizar `search_verses` em `op-db` para aceitar parâmetro `exact: bool` e sintaxe de aspas `"..."`, aplicando filtro de palavra inteira (whole-word boundary) com preservação de pontuações e acentos.
- Atualizar endpoint `GET /api/bibles/:id/search` em `op-admin` para receber query param `exact: Option<bool>` e considerar sintaxe de aspas.

### Frontend
- Atualizar `bibleParser.ts` e tipo `ParsedReference` com suporte a `version`.
- `CommandPalette.svelte`:
  - Adicionar comando dedicado "Abrir Bíblia Sagrada" com atalho `⌘B` / `F4` e ícone `BookOpen`.
  - Prop `onopenbibletab?: () => void` para abrir e focar a aba de Bíblia diretamente.
- `AdminView.svelte`:
  - Atalho global de teclado `(Ctrl+B / ⌘B)` e `F4` para alternar/abrir a aba da Bíblia.
  - Ajuste de atalhos de saída para garantir compatibilidade sem interferências.
  - Ao receber referência bíblica com versão, ativar a versão correspondente na projeção ou na interface.
- `BiblePanel.svelte`:
  - Modo grade (grid) para a primeira coluna quando o modo de abreviações (`bookViewMode === 'abbreviations'`) estiver ativo.
  - Aumento da largura padrão e responsiva da coluna de capítulos (`col2Width`).
  - Input group na busca de versículos com botão toggle de "Palavra exata" (`[Ab]`), com suporte automático a aspas (`"amor"`).
  - Reconhecimento dinâmico de versões bíblicas digitadas na busca, ativando a versão imediatamente no seletor caso instalada.

---

## 3. Decisões Técnicas

1. **Correspondência de Palavra Exata (Whole Word):**
   - Utilização de algoritmo em Rust que identifica limites de palavras alfanuméricas (`!c.is_alphanumeric()`) nas extremidades do termo procurado, garantindo compatibilidade total com caracteres acentuados da língua portuguesa (ex: `amor` não casa com `amorreus`, mas casa com `amor,`, `amor.`, `o amor de Deus`).
   - O SQLite pré-filtra os registros que contêm a substring (`LIKE '%termo%'`) e o Rust filtra por palavra inteira com custo computacional desprezível.
2. **Sintaxe com Aspas e Botão Toggle:**
   - O usuário pode ativar a busca exata tanto clicando no botão do input group quanto digitando o termo entre aspas (`"amor"`). Ambas as abordagens ativam a busca estrita.
3. **Detecção de Versão na Referência:**
   - O parser identifica tokens alfabéticos finais após os versículos ou capítulos como sigla da versão bíblica (ex: `ARA`, `NVI`, `ARC`, `NVT`).
   - Se a sigla coincidir com alguma bíblia instalada, a versão é ativada instantaneamente sem exigir recarga ou clique no dropdown.
4. **Modo Grid para Siglas de Livros:**
   - Com as siglas ativadas, a primeira coluna organiza os livros em botões quadrados/chips compactos em grade (`grid-cols-4` ou `grid-cols-3 sm:grid-cols-4`), permitindo visualizar quase todos os livros sem necessidade de rolagem excessiva.

---

## 4. Critérios de Aceite

1. Digitar `Salmos 91 ARA` no campo de busca da Bíblia ativa a versão ARA (se instalada) e abre Salmos 91:1.
2. Digitar `"amor"` ou ativar o botão toggle de busca exata encontra versículos com a palavra "amor" isolada, e não "amorreus".
3. Alternar para modo "Siglas" na aba de Bíblia exibe os livros em grade compacta.
4. O painel de capítulos possui largura mais espaçosa e botões legíveis.
5. Pressionar `⌘B` / `Ctrl+B` ou `F4` abre a aba de Bíblia imediatamente.
6. O `CommandPalette` inclui a ação rápida "Abrir Bíblia Sagrada".
7. `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `bun run build` passam sem erros.

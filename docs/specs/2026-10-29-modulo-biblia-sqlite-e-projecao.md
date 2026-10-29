# Spec — Módulo de Bíblias SQLite, Navegação em 3 Colunas, Parser de Referências e Projeção

## Intenção

Prover suporte nativo completo à exibição e projeção de textos bíblicos no OpenPresenter a partir de arquivos SQLite padronizados de Bíblias. O operador deve poder gerenciar versões bíblicas (importar, renomear versão/sigla e definir padrão), navegar agilmente pelos livros em 3 colunas redimensionáveis (Livros com visualização em nome ou abreviação, Capítulos e Versículos em slides de 2 colunas), buscar versículos por texto livre ou por referências inteligentes (ex: `joao 3.16`, `Salmos 91`), apresentar versículos instantaneamente com duplo clique ou atalhos (`Shift+Enter`), navegar entre versículos com setas direcionais, selecionar múltiplos versículos para inclusão em setlists, e visualizar referências e versículos com tipografia e diagramação otimizadas no monitor de palco (`/stage`) e projeção pública (`/public`).

Além disso, esta spec inclui a correção da prévia de proporções do palco nas configurações, garantindo que o diagrama dinâmico reflita fielmente o preset ativo (`split-50-50`, `modern-grid`, `video-focus`, `horizontal-stack`).

---

## Escopo

### Incluído

1. **Armazenamento e Gerenciamento de Bíblias (Backend & Configurações):**
   - Criação da pasta de dados de bíblias `<data_dir>/bibles/` no diretório local do sistema.
   - Migração `0007_bibles.sql` no banco `open-presenter.db` com tabela de metadados das versões importadas (`id`, `name`, `abbreviation`, `filename`, `is_default`, `created_at`, `updated_at`).
   - Leitura e validação de arquivos SQLite contendo as tabelas `book (id, book_reference_id, testament_reference_id, name)` e `verse (id, book_id, chapter, verse, text)` (conforme padrão `bibles_ARA.sqlite`).
   - Criação automática de índices de performance (`idx_verse_book_chapter`) nos arquivos SQLite importados.
   - Endpoints REST para listar, importar (upload de arquivo ou caminho local), atualizar (nome e sigla/versão), definir padrão e excluir bíblias.
   - Nova aba "Bíblias" no modal de Configurações do operador (`SettingsModal.svelte`) com listagem, upload de SQLite, edição de nome/sigla e seleção da versão padrão.

2. **Parser e Busca de Referências Bíblicas:**
   - Parser resiliente em `op-core` para o português do Brasil capaz de interpretar:
     - Livros por nome completo ou abreviação canônica (ex: `Gênesis` ou `Gn`, `João` ou `Jo`, `1 Coríntios` ou `1Co`, `Salmos` ou `Sl`).
     - Tratamento transparente de acentuação, maiúsculas/minúsculas e pontuações variadas (`.` ou `:` ou espaço ou vírgula, ex: `joao 3.16`, `joão 3:16`, `jo 3 16`, `salmos 91`).
     - Intervalos de versículos (ex: `joao 3:16-18`).
     - Foco automático no versículo 1 ao informar apenas livro e capítulo (ex: `Salmos 91` foca no verso 1).
   - Busca textual de termos e palavras através dos versículos da bíblia ativa (`v.text LIKE %query%`).
   - Integração do parser na barra do painel de bíblia e na `CommandPalette`.

3. **Painel de Bíblias no Painel Inferior (Mídias):**
   - Nova aba "Bíblia" no `BottomPanel.svelte`.
   - Layout de **3 colunas redimensionáveis**:
     1. **Coluna 1 (Livros):** Lista dos 66 livros da Bíblia, com filtro por testamento (Todos, AT, NT), busca rápida e alternância entre exibição por **Nomes completos** ou **Abreviações canônicas** (ex: `Gn`, `Ex`, `Lv`).
     2. **Coluna 2 (Capítulos):** Grade de botões numéricos com todos os capítulos do livro selecionado.
     3. **Coluna 3 (Versículos em Slides):** Exibição dos versículos em grade de slides (duas colunas), exibindo referência e texto formatado.
   - Redimensionamento suave das colunas via divisores (splitters / drag handles).
   - Seletor de versão bíblica ativa (dropdown de versões instaladas).

4. **Apresentação e Interações:**
   - Duplo clique em um slide de versículo: inicia a projeção imediata (`DisplayMode::Live`) posicionada exatamente no versículo acionado.
   - Navegação com teclado: setas `Left` e `Right` (ou PgUp/PgDn) avançam ou recuam versículos durante a projeção.
   - Duplo clique em outro slide: salta instantaneamente para aquele versículo na tela pública e no retorno.
   - Tecla `Enter` na busca/parser: seleciona e posiciona o livro/capítulo/versículo no painel.
   - Tecla `Shift+Enter` na busca/parser: posiciona e projeta ao vivo imediatamente.
   - Modo de seleção múltipla: clique com `Shift` seleciona intervalos de versículos; surge botão de ação e item de menu de contexto para "Adicionar à Setlist".

5. **Ajustes no Monitor de Palco (`/stage`) e Projeção Pública (`/public`):**
   - Exibição destacada da referência bíblica completa (ex: `João 3:16 ARA`) como cabeçalho ou badge nos slides bíblicos.
   - No Stage Display, exibição nítida do versículo atual e da prévia do próximo versículo com a referência (ex: `João 3:17`), auxiliando a leitura do pregador.

6. **Correção do Preview de Proporções do Stage:**
   - Correção na aba de Monitor de Palco do modal de Configurações: a simulação de distribuição de espaço agora se adapta dinamicamente ao layout do preset selecionado (`split-50-50`, `modern-grid`, `video-focus`, `horizontal-stack`), refletindo as porcentagens e cores configuradas em tempo real.

### Fora de escopo

- Download automático de bíblias de repositórios remotos proprietários (o app permite importação de arquivos SQLite locais fornecidos pelo usuário).
- Comparação simultânea de múltiplas versões lado a lado no mesmo slide (pode ser objeto de evolução futura).

---

## Decisões Técnicas

1. **Armazenamento e Separação de Bíblias:**
   - As bíblias são mantidas em arquivos SQLite individuais na pasta `<data_dir>/bibles/{id}.sqlite`, desacopladas do banco principal do app `open-presenter.db`.
   - O banco principal registra apenas o catálogo de versões instaladas (`bibles`), sua sigla, nome e indicação de versão padrão.
   - Essa abordagem evita o inchaço do banco do app e permite adicionar ou remover facilmente múltiplas traduções (ARA, NVI, ACF, NVT, KJV, etc.).

2. **Direção de Dependências:**
   - `op-core`: Implementa os tipos de dados bíblicos (`BibleBook`, `BibleVerse`, `ParsedReference`), a tabela de abreviações canônicas em português e a lógica pura de parsing de referências.
   - `op-db`: Gerencia o catálogo no banco principal e o acesso otimizado de leitura aos arquivos SQLite individuais de cada Bíblia.
   - `op-admin`: Expõe as rotas REST para arquivos, uploads e consultas, e gerencia a apresentação rápida.
   - `frontend`: Componentes de interface em Svelte com TypeScript e Tailwind CSS.

---

## Critérios de Aceite

1. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo fmt --all` passam com zero erros e warnings.
2. `bun run check` no frontend executa com zero erros e zero warnings.
3. Importação de um arquivo SQLite (como `bibles_ARA.sqlite`) copia o arquivo para `<data_dir>/bibles/`, detecta metadados e cadastra no banco.
4. Na aba de Configurações > Bíblias, é possível renomear nome e sigla da versão e alternar a bíblia padrão.
5. No painel de mídias, a aba "Bíblia" apresenta as 3 colunas redimensionáveis (Livros, Capítulos, Versículos em slides de 2 colunas).
6. Alternância entre nomes de livros completos e abreviações canônicas funciona instantaneamente.
7. Duplo clique em um versículo inicia a apresentação imediata; setas `Left` / `Right` navegam entre versículos.
8. Duplo clique em outro versículo salta imediatamente para ele.
9. Shift+click permite selecionar versículos e adicioná-los à setlist ativa.
10. Digitar `joao 3.16` ou referências similares foca no texto e `Shift+Enter` projeta imediatamente.
11. Digitar `Salmos 91` foca no capítulo com verso 1 selecionado.
12. O monitor de palco exibe a referência bíblica destacada e a prévia do próximo versículo.
13. Na configuração do Stage, o preview dinâmico de distribuição reflete o preset ativo selecionado.

# ADR 0010 — Módulo de Bíblias em Arquivos SQLite Desacoplados, Parser de Referências em Português e Exibição de Escrituras no Palco

## Contexto

A projeção de textos bíblicos é uma necessidade central para igrejas e eventos litúrgicos. As versões bíblicas em português (como ARA, NVI, ARC, NVT) possuem dezenas de milhares de versículos estruturados em 66 livros e milhares de capítulos. Além disso:

1. Os operadores e pregadores utilizam com frequência referências com notações variadas (`joao 3.16`, `jo 3 16`, `joão 3:16`, `Salmos 91`, `1 co 13:4-7`), demandando um parser de referências ágil e tolerante a acentuação e pontuação.
2. É essencial uma navegação rápida e visual com 3 colunas (Livros com opção de nomes ou siglas, Capítulos em botões e Versículos em formato de slides), além de suporte a atalhos de projeção imediata (`Shift+Enter` e duplo clique) e navegação com setas do teclado.
3. Cada tradução da Bíblia possui seu próprio ciclo e volume (geralmente arquivos SQLite independentes contendo tabelas `book` e `verse`).
4. Os monitores de retorno no palco necessitam de destaque especial para a referência do versículo em leitura e a prévia do próximo versículo para o pregador.

## Decisão

1. **Arquivos SQLite Desacoplados:**
   As bíblias importadas são armazenadas como arquivos SQLite individuais na pasta `<data_dir>/bibles/{id}.sqlite`, enquanto o banco principal da aplicação `open-presenter.db` mantém apenas o catálogo das versões instaladas (migração `0007_bibles.sql`), incluindo sua identificação, sigla (ex: ARA, NVI), nome oficial e status de versão padrão. Isso evita inflar o banco operacional e facilita backups e inclusão de novas traduções.

2. **Parser Puro de Referências em `op-core`:**
   A lógica de mapeamento canônico dos 66 livros e o algoritmo de interpretação de referências textuais reside no `op-core`, garantindo independência de banco de dados ou UI, e facilitando testes unitários determinísticos para todos os livros da Bíblia e suas variações ortográficas.

3. **Navegação em 3 Colunas e Apresentação em Slides:**
   A interface fornece um painel de 3 colunas redimensionáveis com splitters:
   - Coluna de Livros (comutável entre nomes completos e abreviações canônicas).
   - Coluna de Capítulos (grade numérica).
   - Coluna de Versículos em formato de slides de 2 colunas, permitindo visualização prévia da divisão de slides que será projetada.
   Duplo clique em um versículo realiza o disparo imediato da projeção, e `Shift+click` viabiliza a seleção de versículos individuais ou intervalos para adição na setlist ativa.

4. **Correção do Diagrama Proporcional de Palco:**
   O diagrama de distribuição de espaço nas configurações de Stage agora é dinâmico e respeita o preset ativo (`split-50-50`, `modern-grid`, `video-focus`, `horizontal-stack`), refletindo imediatamente os percentuais e as cores customizadas.

## Consequências

- **Positivas:**
  - Performance extrema: buscas e navegação direta em SQLite local com índices sob medida (`idx_verse_book_chapter`).
  - Flexibilidade para o operador adicionar, renomear siglas e alternar traduções bíblicas sem reiniciar a aplicação.
  - Experiência de uso fluida para operadores de projeção com suporte a busca instantânea, navegação com setas direcionais e atalhos com `Shift+Enter`.
  - Melhor legibilidade no monitor de palco para oradores e pastores.
- **Negativas / Cuidados:**
  - Importação de arquivos SQLite externos requer validação de integridade estrutural (tabelas e colunas esperadas) e tratamento de erros amigável caso o arquivo esteja corrompido ou em formato incompatível.

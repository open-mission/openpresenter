# Especificação: Busca de Textos Bíblicos por Referência e Palavras no Command Palette Global

## Contexto e Motivação
O Command Palette global (acionado por `Ctrl+K` ou `Cmd+K`) é a ferramenta central de busca rápida do operador. Anteriormente, ele permitia buscar apenas músicas locais e músicas online, com apenas uma detecção básica de referência de versículo único. O operador precisava navegar até a aba "Bíblia" do painel inferior para buscar passagens completas ou pesquisar versículos bíblicos por palavras-chave (ex: "amor de Deus", "princípio criou Deus", "pastor é meu pastor").

## Escopo
- Adicionar a aba dedicada **Bíblia Sagrada** no Command Palette global com navegação por teclado (`Tab`, setas, atalhos).
- **Busca por Referência Bíblica:** Suporte a qualquer referência como "João 3:16", "Sl 23:1-4", "Romanos 8", "Gênesis 1", com carregamento dos versículos correspondentes da passagem.
- **Busca por Palavras-chave:** Quando a busca não for uma referência, consultar a API de busca bíblica por texto (`/api/bibles/{id}/search?q=...`), listando os versículos encontrados com referência, livro, capítulo, número do versículo e texto.
- **Ações Imediatas:**
  - **Apresentar ao Vivo:** Dispara projeção imediata do versículo ou passagem nas saídas pública e de palco.
  - **Adicionar à Setlist:** Insere o item bíblico formatado diretamente na setlist ativa.
  - **Abrir no Painel da Bíblia:** Navega até o livro/capítulo/versículo no painel de Bíblias.
- **Seletor de Versão:** Possibilidade de alternar entre as versões instaladas (ARA, NVI, etc.) diretamente no Command Palette.

## Decisões Técnicas
- Utiliza os endpoints existentes `/api/bibles`, `/api/bibles/{id}/books/{bookId}/chapters/{chapter}/verses` e `/api/bibles/{id}/search`.
- Busca com debounce de 150ms para fluidez de digitação.
- Atalhos de teclado familiares: `Enter` para apresentar ao vivo ou abrir, e `Shift+Enter` para adicionar diretamente à setlist ativa.

## Critérios de Aceite
- O operador pode abrir o Command Palette (`Cmd+K` / `Ctrl+K`), selecionar a aba Bíblia e buscar passagens por referência (ex: `João 3:16`) ou por palavras (ex: `luz do mundo`).
- Os versículos são listados com opções para apresentar ao vivo e adicionar à setlist.
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

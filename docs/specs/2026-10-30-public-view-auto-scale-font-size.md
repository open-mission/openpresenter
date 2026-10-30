# Spec — Auto-scale de Fontes e Controle de Tamanho na Tela Pública (/public)

## Contexto e Motivação

Na projeção pública (/public), os slides de músicas e passagens bíblicas anteriormente utilizavam classes estáticas de tamanho de fonte (`text-4xl` a `text-8xl`), dependendo apenas de uma flag booleana `fill_space`. Quando uma estrofe possuía poucas palavras, a fonte não aproveitava todo o impacto visual do telão. Por outro lado, quando o operador desativava a quebra de linha (`word_wrap = false`), versos longos eram truncados com reticências (`...` via `text-ellipsis`), prejudicando a leitura da congregação.

## Intenção e Decisões

1. **Auto-scale Dinâmico na Projeção Pública**:
   - Integrar a ação `autoScale` no elemento de texto principal da tela `/public`.
   - O algoritmo calcula em tempo real o maior tamanho de fonte possível dentro da área visível (respeitando margens seguras e caixa de leitura).
2. **Eliminação de Reticências no Modo sem Quebra de Linha**:
   - Quando `word_wrap` estiver desativado, o texto adota `whitespace-nowrap` sem `text-ellipsis`.
   - O auto-scale reduz suavemente o tamanho da fonte até que todos os versos caibam na largura horizontal sem quebrar linhas e sem adicionar `...`.
3. **Controle de Tamanho Mínimo e Máximo nas Configurações**:
   - Adicionar os campos `min_font_size` (padrão 24px) e `max_font_size` (padrão 128px / 8rem) em `PublicDisplaySettings`.
   - Disponibilizar controles na aba "Público (Projetor)" das Configurações, permitindo ajustar o limite superior de impacto (alvo de 8rem) e o limite inferior para evitar texto ilegível.

## Critérios de Aceite

- [x] O texto da tela `/public` dimensiona-se automaticamente (auto-scale) para preencher a tela com base no conteúdo e resolução.
- [x] Com a quebra de linha desativada (`word_wrap: false`), as linhas longas não recebem `...` (reticências) e diminuem a fonte para caber.
- [x] As configurações de tela pública incluem ajuste de tamanho mínimo e máximo/alvo de fonte.
- [x] As configurações persistem no backend e sincronizam em tempo real.
- [x] Quality gates executados e aprovados (`cargo test`, `npm run check`, `npm run build`).

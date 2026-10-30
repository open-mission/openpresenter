# Especificação: Refinamento Visual da Referência Bíblica na Projeção Pública (/public)

## Contexto e Motivação
Na tela pública (`/public`) e no preview do operador (`PublicPreviewBox.svelte`), a exibição de versículos bíblicos apresentava incongruências de design reportadas pelo usuário:
1. **Duplicação de Versão:** A sigla da versão aparecia duplicada (ex.: "📖 João 3:1 ARA ARA"), pois a abreviação estava sendo embutida tanto no rótulo do slide quanto no metadado da mídia.
2. **Posicionamento:** A referência ficava empilhada no meio do bloco de texto logo abaixo do versículo em vez de ancorada de forma elegante no rodapé da projeção.
3. **Estilo e Cores:** A presença do emoji (`📖`) e o uso de tipografia amarela chamativa (`text-amber-300`) contrastavam de forma desarmônica com o restante do design limpo e minimalista da projeção.

## Escopo
1. **Eliminação de Duplicidade:**
   - Em `BiblePanel.svelte`, padronizar o rótulo do slide para conter apenas a referência do versículo (ex.: `João 3:1`), mantendo a sigla da tradução em `artist_or_ref`.
   - Na renderização da tela pública e previews, sanitizar e desmembrar a referência e versão caso a versão já conste no final do rótulo.
2. **Posicionamento e Alinhamento:**
   - Ancorar a barra de referência no rodapé do quadro (`mt-auto`), utilizando distribuição horizontal (`justify-between`), com a referência bíblica à esquerda e a insígnia da tradução (ex.: `ARA`, `NVI`) à direita.
3. **Remoção de Emoji e Harmonização de Cores:**
   - Remover o emoji de livro (`📖`) de todas as telas públicas e monitores de retorno de palco.
   - Substituir a cor amarela por tipografia branca/neutra harmoniosa (`text-white font-bold drop-shadow`), integrando-se perfeitamente aos temas e fundos de projeção.

## Decisões Técnicas
- **Função Utilitária de Sanitização:** Criação de `formatScriptureReference(label, version)` para garantir retrocompatibilidade com qualquer setlist ou histórico que já possua a sigla embutida no label.
- **Hierarquia Visual:** O texto do versículo permanece no centro da tela com auto-scale, enquanto a referência fica discretamente ancorada na base do container.

## Critérios de Aceite
- Na tela `/public` e no preview, versículos bíblicos exibem a referência limpa (ex.: "João 3:1") sem duplicação de versão (ex.: não exibe "ARA ARA").
- O emoji `📖` não é exibido na projeção pública nem no monitor de palco.
- A referência bíblica é posicionada no rodapé da área de texto com layout `justify-between` (referência de um lado, versão do outro).
- A cor da referência é consistente e elegante (branca com drop shadow, sem amarelo estridente).
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

# Spec — Correção de Destaque Ativo de Versículos em Múltiplas Instâncias do Painel Admin

## Contexto e Motivação

Quando duas ou mais instâncias do painel administrativo estão conectadas simultaneamente (por exemplo, operador principal no computador e co-operador em um notebook ou tablet), ocorria uma inconsistência visual no painel de Bíblias (`BiblePanel.svelte`).

Se a máquina A estivesse apresentando um versículo específico (por exemplo, 1 Tessalonicenses 5:7), ao abrir a aba da Bíblia na máquina B em outro capítulo (por exemplo, 1 Tessalonicenses 1) ou outro livro (por exemplo, Gênesis 1), o versículo 7 daquele capítulo não relacionado ficava incorretamente destacado em verde com o selo "Ao Vivo".

### Causa Raiz

Em `frontend/src/lib/components/BiblePanel.svelte`:
1. `isScriptureLive` era avaliado apenas como:
   ```ts
   const isScriptureLive = $derived(
     $presentationState.active_media_id?.startsWith('scripture-')
   );
   ```
   Isso retornava `true` para qualquer projeção bíblica ativa no sistema, independentemente de qual livro ou capítulo estivesse sendo exibido localmente.
2. Na renderização do grid de versículos:
   ```svelte
   {@const isLive = isScriptureLive && $presentationState.active_slide_index !== null && verses[$presentationState.active_slide_index]?.verse === v.verse}
   ```
   Como o índice do slide ativo era 6 (versículo 7), a expressão avaliava `verses[6]?.verse === 7`, que coincidia com o versículo 7 de qualquer capítulo aberto em outra máquina, tornando-o verde indevidamente.

## Intenção e Decisões

1. **Validação Específica de Livro e Capítulo em `isScriptureLive`**:
   - `isScriptureLive` deve checar não apenas o prefixo `scripture-`, mas verificar se a mídia atualmente ao vivo pertence exatamente ao livro (`selectedBookId`) e capítulo (`selectedChapter`) atualmente carregados no painel.
   - Suportar tanto mídias avulsas rápidas (`quick_media` com ID `scripture-${selectedBibleId}-${selectedBookId}-${selectedChapter}`) quanto trechos bíblicos adicionados à Setlist (verificando o título da mídia e o rótulo dos slides contra `${currentBook.name} ${selectedChapter}` e `${currentBook.abbreviation} ${selectedChapter}`).

2. **Destaque Seguro do Versículo Individual (`isVerseLive`)**:
   - O versículo só pode ser considerado `isLive` se `isScriptureLive` for verdadeiro para o capítulo atual e o versículo corresponder com precisão ao slide ativo (`activeSlide.label` ou índice dentro do conjunto de versículos do capítulo validado).

3. **Independência de Instâncias**:
   - Uma máquina que esteja navegando em qualquer outro livro ou capítulo não terá nenhum versículo colorido de verde nem marcado como "Ao Vivo", a menos que navegue exatamente para o livro e capítulo que estão no ar.

## Critérios de Aceite

- [x] Quando 1Ts 5:7 estiver sendo apresentado, navegar para 1Ts 1 (ou qualquer outro capítulo/livro) não deixa o versículo 7 (nem qualquer outro) verde.
- [x] Ao navegar para o capítulo que realmente está sendo apresentado (1Ts 5), apenas o versículo 7 fica destacado em verde com a badge "Ao Vivo".
- [x] Suporta passagens projetadas via duplo clique na Bíblia, via Command Palette e via Setlist.
- [x] Execução limpa dos testes Rust (`cargo test --workspace`) e checagem TypeScript/Svelte (`npm --prefix frontend run check`).

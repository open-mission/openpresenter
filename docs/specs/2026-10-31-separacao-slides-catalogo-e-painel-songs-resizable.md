# Spec: Separação Correta de Slides e Redimensionamento com Resizable Shadcn no Catálogo de Músicas

## Status
Aprovada

## Intenção
Corrigir a renderização e separação de slides na visualização detalhada de músicas no painel inferior (`BottomPanel.svelte`), assegurando que estrofes separadas por linhas em branco (`\n\n`) gerem slides individuais em proporção 16:9 com disposição responsiva em múltiplas colunas (2 ou 3 colunas com wrap), e possibilitar o redimensionamento fluido entre a lista do catálogo e o painel de slides utilizando os componentes `Resizable` do shadcn-svelte (`paneforge`).

## Escopo
1. **Separação Correta dos Slides (`viewingSongSlides`)**:
   - Deixar de iterar unicamente sobre `song.sections` (que agrupa todas as estrofes contíguas em uma única seção sem respeitar quebras de slide por linha em branco).
   - Implementar cálculo derivado `viewingSongSlides` utilizando `parseRawLyrics(viewingSong.lyrics_raw)`, gerando individualmente cada slide a partir das quebras de estrofe (`\n\n` e `##`).
   - Adicionar tag de identificação da seção com cor temática (`getSectionColor`), numeração ordinal e proporção 16:9.

2. **Grade Responsiva com Wrap (2 a 3 Colunas)**:
   - Atualizar o layout dos cards de slides para grade responsiva fluida: `grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3`.
   - Limitar o texto em `line-clamp-6` para evitar estiramento vertical dos cards e garantir que caibam confortavelmente dentro da proporção de aspecto 16:9.

3. **Divisão Redimensionável com Shadcn UI (`Resizable`)**:
   - Envolver a lista de músicas do catálogo e o painel de prévia de slides em `<ResizablePaneGroup direction="horizontal">`.
   - Painel esquerdo (`ResizablePane` com lista de músicas): `defaultSize={35}`, `minSize={20}`, `maxSize={60}`.
   - Divisor central: `<ResizableHandle withHandle class="hover:bg-primary transition-colors cursor-col-resize" />`.
   - Painel direito (`ResizablePane` com prévia dos slides): `defaultSize={65}`, `minSize={40}`.
   - Quando nenhuma música estiver sendo visualizada (`viewingSong === null`), a grade de catálogo expande naturalmente para preencher 100% do espaço horizontal.

## Critérios de Aceite
1. Letras com estrofes separadas por linhas em branco são decompostas em slides individuais 16:9, eliminando o card único acumulado.
2. A área de prévia de slides distribui os cards em 2 ou 3 colunas quando houver largura disponível.
3. O operador consegue redimensionar a largura entre a lista de músicas e os slides arrastando o handle vertical do shadcn.
4. Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

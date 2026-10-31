# Spec: Fundo Vermelho para o Botão de Parar Apresentação no Header do Controle Remoto

## Status
Aprovada

## Intenção
Destacar visualmente o botão de parar apresentação no cabeçalho do Controle Remoto Mobile (`/remote`), aplicando um fundo vermelho distintivo (`bg-red-600`) com ícone preenchido e rótulo adaptativo, tornando a ação de interromper a projeção instantaneamente intuitiva e segura para o operador no smartphone ou tablet.

## Escopo
1. **Estilização Visual do Botão no Header**:
   - Modificar o botão de parada (`handleStop`) em `frontend/src/lib/views/RemoteView.svelte`.
   - Substituir a cor neutra escura anterior (`bg-neutral-800 text-neutral-400`) por um padrão de alta visibilidade e intenção semântica destrutiva: fundo vermelho `bg-red-600 hover:bg-red-500`, texto/ícone em branco `text-white`, borda sutil `border-red-500/40` e sombra de contraste `shadow-md shadow-red-950/50`.
   - Preencher o ícone do quadrado (`<Square size={13} class="fill-current" />`) para reforçar o símbolo convencional de parada (stop).
   - Adicionar o texto "Parar" em telas a partir de 370px (`<span class="hidden min-[370px]:inline">Parar</span>`), mantendo o design compacto e sem quebra em telas ultraestreitas.
2. **Critérios de Aceite**:
   - O botão de parar no Header do Remote apresenta fundo vermelho nítido e ícone de stop destacado.
   - O clique no botão continua disparando `stopPresentation()` normalmente com feedback tátil e sonner toast.
   - A barra superior preserva alinhamento perfeito em smartphones sem estourar o cabeçalho.
   - Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

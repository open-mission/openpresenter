# Spec — Correção de Sentido na Navegação por Teclas de Seta e Sincronização de Slides em Tempo Real

## Contexto e Motivação

Durante a navegação por slides via teclado (especialmente ao apresentar versículos bíblicos ou letras de louvor), ao pressionar a tecla de seta para a direita (`ArrowRight`, `PageDown` ou `Espaço`), o slide ativo frequentemente não avançava para o próximo, mas saltava para trás (por exemplo, voltando para o início ou para o slide 2).

### Causas Raízes Identificadas

1. **Descompasso entre Foco Local (`focusedSlideIndex`) e Slide ao Vivo (`active_slide_index`)**:
   - `nextSlide()` e `prevSlide()` em `AdminView.svelte` avançavam ou retrocediam unicamente a partir de `focusedSlideIndex`, que era inicializado em `0`.
   - Ao disparar um versículo bíblico ad-hoc (por exemplo, versículo 7, índice 6), `handlePresentScripture` não atribuía o índice do slide a `focusedSlideIndex`.
   - Devido ao ciclo assíncrono de rede do WebSocket, `focusedSlideIndex` permanecia em `0`.
   - Ao teclar `ArrowRight`, `nextSlide()` calculava `0 + 1 = 1` e invocava `selectSlide(item.id, 1)` (versículo 2), retrocedendo o versículo de 7 para 2.

2. **Ausência de Sincronização Bidirecional entre Múltiplos Aparelhos**:
   - Se uma segunda máquina ou o controle remoto mobile `/remote` alterasse o slide ativo, o `$effect` em `AdminView.svelte` só verificava `if (selectedMediaId !== prevMediaId)`.
   - Como o ID da mídia não havia mudado, `focusedSlideIndex` não era atualizado para refletir o slide atualizado via WebSocket.
   - Pressionar qualquer tecla de navegação em uma máquina conectada causava um salto imediato para um índice defasado.

## Intenção e Decisões

1. **Navegação com Base no Slide ao Vivo Atual**:
   - Quando a mídia em foco estiver ao vivo (`isSelectedMediaLive`), `nextSlide()` e `prevSlide()` devem calcular o próximo slide a partir do índice real transmitido pelo backend (`$presentationState.active_slide_index ?? focusedSlideIndex`).
   - Se a apresentação estiver no slide 6, a seta direita navegará infalivelmente para o slide 7 (`6 + 1 = 7`), e a seta esquerda para o slide 5 (`6 - 1 = 5`).

2. **Sincronização Reativa Contínua em Tempo Real**:
   - O `$effect` de sincronização de estado deve monitorar mudanças tanto em `active_media_id` quanto em `active_slide_index`.
   - Ao receber atualização de slide ao vivo de qualquer dispositivo conectado, `focusedSlideIndex` é sincronizado instantaneamente.
   - Se uma nova mídia entrar ao vivo no sistema, a visão do operador acompanha e seleciona a mídia ativa.

3. **Inicialização Consistente nos Disparos de Mídia**:
   - `handlePresentScripture` deve inicializar explicitamente `focusedSlideIndex = slideIndex` no momento do disparo.
   - `handlePresentSong` e `handlePresentImage` devem inicializar `focusedSlideIndex = 0`.

4. **Auto-Scroll no Painel de Slides**:
   - Garantir que o card do slide focado e ativo permaneça visível na rolagem do painel central.

## Critérios de Aceite

- [x] Ao apresentar um versículo bíblico (ex: 1Ts 5:7) e pressionar `ArrowRight`, o sistema avança para o versículo 8 (índice 7), sem jamais retroceder.
- [x] Ao pressionar `ArrowLeft`, o sistema retrocede para o versículo anterior de forma consistente.
- [x] Quando outro aparelho conectado (ou controle remoto mobile) avançar o slide, o operador no painel admin sincroniza seu índice e, ao usar as setas, continua a partir do slide correto.
- [x] Testes de workspace Rust (`cargo test --workspace`) e checagem TypeScript/Svelte (`npm --prefix frontend run check`) 100% aprovados.

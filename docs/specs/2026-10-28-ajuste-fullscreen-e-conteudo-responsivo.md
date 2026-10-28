# Spec — Ajuste de tela cheia independente e conteúdo responsivo nas saídas

- **Status:** aprovado
- **Data:** 2026-10-28
- **Versão do app:** 0.2.0
- **Versão do schema:** 2 (inalterado)
- **Substitui:** nada; estende e corrige a [spec de multi-telas](2026-10-28-multi-telas.md)

## 1. Intenção

Permitir que as duas saídas de projeção (**Public** e **Stage**) funcionem em modo **Tela cheia** simultaneamente em suas respectivas telas sem conflito ou cancelamento mútuo, e garantir que o conteúdo visual de ambas as saídas seja responsivo e preencha a tela adequadamente em qualquer resolução, evitando que o conteúdo fique reduzido a um bloco pequeno e fixo no centro da tela.

## 2. Problemas identificados

1. **Cancelamento mútuo de tela cheia**:
   - Ao alterar qualquer configuração de uma saída (ex: alternar Stage para Tela cheia), `OutputsController::apply_config()` executava `reposition()` em **todas** as saídas abertas.
   - A função `reposition()` chamava incondicionalmente `target.set_fullscreen(false)` antes de qualquer ação.
   - Como resultado, se a saída Public já estivesse em tela cheia e o usuário colocasse Stage em tela cheia, o Public era forçado a sair de tela cheia. No macOS / AppKit, disparar saída e entrada de tela cheia em múltiplas janelas em sequência causa transições conflitantes e descarte de eventos pelo gerenciador de janelas.
   - A alteração de uma saída deve reposicionar exclusivamente a saída que foi modificada (`apply_config_for(kind)`), e `reposition()` deve ser ciente do estado atual (`is_fullscreen()`, `is_maximized()`), nunca chamando `set_fullscreen(false)` se a janela já está em tela cheia e deve permanecer nela.

2. **Conteúdo estático não preenchendo a tela em tela cheia**:
   - Em `crates/op-admin/ui/outputs.slint`, tanto o `PublicWindow` quanto o `StageWindow` utilizavam tamanhos absolutos e fixos (ex: logotipo de 300px × 98px, texto de 18px no Public; relógio de 88px no Stage).
   - Em telas cheias de alta resolução (1080p, 1440p, 4K ou Retina), o fundo escuro preenchia 100% da tela, mas os elementos gráficos e tipográficos ficavam reduzidos a uma pequena área centralizada, com enorme vazio ao redor.
   - Telas de projeção e retorno de palco exigem elementos em escala proporcional e legíveis a longa distância (músicos e pregadores a 10–30 metros de distância).

## 3. Escopo da solução

1. **Controlador de saídas (`crates/op-admin/src/outputs.rs`)**:
   - Implementar `apply_config_for(&mut self, kind: OutputKind)` para reaplicar geometria e modo estritamente à saída especificada.
   - Modificar `set_mode`, `set_monitor`, `show_public_now` e `show_stage_now` para usarem `apply_config_for(kind)`.
   - Tornar `reposition()` idempotente:
     - Consultar `target.is_fullscreen()` e `target.is_maximized()`.
     - Só sair de tela cheia se `is_fullscreen && !should_fullscreen`.
     - Só sair de maximizado se `is_maximized && !should_maximized`.
     - Só entrar em tela cheia se `!is_fullscreen && should_fullscreen`.
     - Só entrar em maximizado se `!is_maximized && should_maximized`.
     - Não chamar métodos do sistema operacional quando o estado da janela já corresponder ao modo desejado.
   - Proteger `apply_mode()` contra chamadas repetidas a `set_fullscreen(true)`.

2. **Interface das saídas (`crates/op-admin/ui/outputs.slint`)**:
   - `PublicWindow`:
     - Calcular fator de escala proporcional `scale-factor` a partir de `root.width` e `root.height`.
     - Escalar logotipo, espaçamentos e tipografia proporcionalmente para preencher harmonicamente a tela em telas cheias de qualquer resolução.
   - `StageWindow`:
     - Layout vertical distribuído (`alignment: space-between`): cabeçalho/data no topo, relógio de alta visibilidade no centro ocupando espaço de destaque, e status do evento/apresentação na base.
     - Tipografia do relógio e rótulos proporcionais às dimensões da janela.

## 4. Critérios de aceite

1. Ambas as saídas (`Public` e `Stage`) podem ter o modo `Tela cheia` configurado simultaneamente sem que uma feche ou cancele o modo da outra.
2. Em modo `Tela cheia`, o conteúdo do Public (marca, badge e texto) expande proporcionalmente à resolução da tela.
3. Em modo `Tela cheia`, o retorno de palco (`Stage`) distribui cabeçalho, relógio e rodapé, com o relógio ocupando posição central de destaque e tamanho visível à distância.
4. Em janelas menores ou redimensionadas, o conteúdo se adapta proporcionalmente sem estourar os limites da janela.
5. Todos os portões de qualidade (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`) passam sem erros ou warnings.

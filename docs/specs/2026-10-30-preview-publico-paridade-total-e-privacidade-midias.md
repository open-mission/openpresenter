# Spec — Paridade 100% no Preview do Admin, Ajustes Bíblicos e Privacidade de Imagens

- **Status:** aprovada
- **Data:** 2026-10-30
- **Autor:** OpenPresenter Core Team
- **Contexto:** commit `95ee5ca39f86f14229de0297bb1a310da0afa85f`

## 1. Intenção

Proporcionar paridade visual absoluta (100% fiel) entre o preview da tela pública no painel do operador (`PublicPreviewBox.svelte`) e a projeção real (`/public`), além de refinar a apresentação da referência bíblica, introduzir opções avançadas de encaixe de papel de parede e salvaguardar a privacidade de arquivos de mídia do operador durante cultos ao vivo.

## 2. Escopo

- Reestruturação de `PublicPreviewBox.svelte` para renderização em canvas virtual Full HD (1920x1080) escalado dinamicamente via `transform: scale()`.
- Suporte a posicionamento configurável da referência bíblica: **No Rodapé** (com barra elegante) ou **Abaixo do Texto** (acompanhando versículo centralizado).
- Adição de controle deslizante para tamanho da fonte da referência bíblica no rodapé (16px a 64px, padrão 28px).
- Adição de opções de encaixe de wallpaper (`wallpaper_fit`): `contain`, `cover` e `fill`, além de margens seguras contra overscan de televisores HDMI/Chromecast (`wallpaper_safe_margins`).
- Implementação de filtro de privacidade na galeria de imagens (`BottomPanel.svelte`), ocultando por padrão arquivos que não pertençam a pastas importadas explicitamente pelo usuário (`is_external: true`).
- Correção de geometria e alinhamento dos botões interruptores (switches) no padrão Tailwind UI.
- Priorização de arquivos estáticos compilados em disco (`frontend/dist`) no servidor backend Rust (`crates/op-admin/src/static_files.rs`).

## 3. Decisões

1. **Canvas Virtual HD 1080p Escalado:**
   Em vez de aproximar tamanhos com multiplicações manuais e fontes fixas, `PublicPreviewBox.svelte` adota um contêiner interno com dimensões nativas `1920x1080px`, aplicando `transform: scale(containerWidth / 1920)`. Assim, quebras de linha, margens percentuais, auto-scale e proporções são idênticas ao projetor público real.
2. **Posição da Referência Bíblica:**
   O usuário pode optar por exibir a referência bíblica no rodapé da projeção (`footer`) ou centralizada logo abaixo do versículo bíblico (`below-text`).
3. **Privacidade de Mídias:**
   Para evitar que imagens pessoais ou residuais do sistema operacional apareçam na tela do operador durante transmissões ao vivo, a visualização padrão restringe-se a pastas mapeadas explicitamente. Um botão de alternância com persistência no LocalStorage permite revelar todas as pastas quando necessário.
4. **Geometria dos Switches (Tailwind UI Standard):**
   Utilização de contêiner relativo com translação exata do círculo (`translate-x-5` quando ativo), garantindo que o thumb permaneça perfeitamente contido dentro da cápsula arredondada sem vazar pelas bordas.

## 4. Critérios de Aceite

- [x] O preview do painel administrativo reflete 100% da tipografia, quebra de linha e escala da tela `/public`.
- [x] A referência bíblica pode ser alternada entre rodapé e abaixo do versículo, com tamanho de fonte customizável.
- [x] Imagens do sistema não são exibidas na galeria de mídias a menos que o operador ative o botão de exibição completa.
- [x] Todos os switches no modal de configurações possuem alinhamento geométrico perfeito e contraste visível.
- [x] Arquivos estáticos em `frontend/dist` são servidos preferencialmente no backend.

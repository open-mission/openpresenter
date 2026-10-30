# Spec — Apresentação Rápida da Bíblia sem Poluir a Setlist e Menu de Contexto

## Contexto e Motivação

Anteriormente, ao dar duplo clique em um versículo na aba de Bíblias, o sistema realizava um `quick_present_media` que inseria automaticamente o capítulo inteiro na Setlist ativa. Isso poluía a lista de reprodução do culto com itens indesejados quando o operador desejava apenas projetar uma referência bíblica pontual ao vivo. Além disso, não havia um menu de contexto (botão direito) nos versículos para adicionar trechos selecionados ou o capítulo completo à Setlist.

## Intenção e Decisões

1. **Apresentação Rápida Independente (Quick Media sem Inserção na Setlist)**:
   - O `quick_present_media_with_slide` no backend e no frontend não adiciona itens em `media_items` (Setlist ativa).
   - O item apresentado sob demanda passa a ser armazenado no campo `quick_media: Option<MediaItem>` de `PresentationState`, sendo sincronizado em tempo real com as saídas `/public`, `/stage` e com a tela do operador.
   - Ao parar a apresentação ou ao selecionar um item da Setlist, `quick_media` é descarregado normalmente.
2. **Duplo Clique no Versículo Apenas Apresenta**:
   - Duplo clique em um versículo projeta instantaneamente a passagem na tela pública e de retorno, sem adicionar nada à Setlist.
3. **Menu de Contexto nos Versículos da Bíblia**:
   - Clique com o botão direito em qualquer versículo abre o menu de contexto flutuante com as opções:
     - **Adicionar à Setlist**: adiciona os versículos selecionados (ou o versículo clicado) como item da Setlist ativa.
     - **Adicionar Capítulo Inteiro à Setlist**: cria um item da Setlist contendo todo o capítulo atual.
     - **Apresentar ao Vivo**: projeta o versículo imediatamente.
     - **Copiar Versículo(s)**: copia o texto formatado com referência para a área de transferência.

## Critérios de Aceite

- [x] O duplo clique em um versículo bíblico projeta o versículo ao vivo sem adicionar o capítulo à Setlist.
- [x] A tela pública e o palco exibem o versículo e navegam entre os versículos normalmente.
- [x] O clique com o botão direito em um versículo exibe menu de contexto com opção clara de adicionar à Setlist.
- [x] Testes de workspace Rust e testes de frontend aprovados.

# Especificação — Itens e Abas de Áudio, Vídeo e Projeção de Timer na Tela Pública

**Data:** 2026-11-05  
**Status:** Aprovada  
**Versão Alvo:** 0.4.5  
**Autor:** Antigravity (Pair Programming com Usuário)

---

## 1. Intenção e Contexto

No ambiente de apresentações para igrejas e eventos, o operador multimídia precisa frequentemente:
1. **Reproduzir Áudio:** Executar trilhas sonoras de fundo, instrumentais, prelúdios, avisos gravados ou efeitos sonoros sem depender de um reprodutor externo fora do OpenPresenter.
2. **Apresentar Vídeos:** Disparar vídeos promocionais, testemunhos, clipes musicais e vinhetas institucionais diretamente na tela pública (projetor/LED) com controle integrado de reprodução, pausa, reinício e volume.
3. **Exibir Timer na Tela Pública:** Apresentar contagens regressivas elegantes (ex.: *"O culto começa em 05:00"*, momentos de oração, avisos temporizados) na tela pública, de forma totalmente sincronizada com o monitor de palco (*Stage Display*), garantindo que tanto a congregação quanto os ministros vejam o mesmo tempo em tempo real.

Esta especificação define os modelos de domínio, rotas de backend, interfaces de usuário no painel inferior (`BottomPanel`), renderização na tela pública (`PublicView`), sincronização no palco (`StageView`) e controles de operação no `AdminView`.

---

## 2. Escopo

### 2.1 Backend (`op-core`, `op-db`, `op-admin`)
- Expansão do enum `MediaKind` em `op-core` com as novas variantes:
  - `MediaKind::Audio` (rótulo: *"Áudio"*)
  - `MediaKind::Video` (rótulo: *"Vídeo"*)
  - `MediaKind::Timer` (rótulo: *"Cronômetro"* / *"Timer"*)
- Novas rotas REST e escaneamento de arquivos em `op-admin`:
  - `GET /api/media/audios` e `POST /api/media/audios/upload`
  - `GET /api/media/videos` e `POST /api/media/videos/upload`
  - Suporte a streaming com headers de range para arquivos de mídia (`.mp4`, `.webm`, `.mov`, `.mp3`, `.wav`, etc.).
  - Mapeamento automático sob as pastas internas `<data_dir>/media/audios` e `<data_dir>/media/videos`, além de suporte a pastas externas cadastradas.
- Documentação OpenAPI 3.0.3 e Scalar atualizada para todas as novas rotas.

### 2.2 Frontend — Aba de Áudios (`BottomPanel.svelte`)
- Nova aba **"Áudios"** na barra de navegação de mídias com ícone `Volume2`.
- Listagem de faixas de áudio com filtro por nome e exibição de tamanho e metadados.
- Reprodutor embutido no console do operador com Play, Pause, barra de progresso (seek), tempo decorrido / restante, controle de volume e botão de Loop.
- Ações rápidas:
  - *"Tocar / Pausar"* (pré-escuta ou execução direta).
  - *"Adicionar à Setlist"* (criação de `MediaItem` do tipo `audio`).
  - Menu de contexto (botão direito) para ações rápidas.

### 2.3 Frontend — Aba de Vídeos (`BottomPanel.svelte`)
- Nova aba **"Vídeos"** na barra de navegação com ícone `Film`.
- Grade responsiva com cartões de vídeo, nome, tamanho e pasta de origem.
- Botões de upload e recarga rápida.
- Ações rápidas:
  - *"Apresentar ao Vivo"* (disparo imediato para a tela pública).
  - *"Adicionar à Setlist"* (criação de `MediaItem` do tipo `video`).
  - Menu de contexto para pré-visualização e gerenciamento.

### 2.4 Frontend — Projeção de Vídeo (`PublicView.svelte` e `AdminView.svelte`)
- Na tela pública (`PublicView`), quando a mídia ativa for do tipo `video`:
  - Renderização fluida de elemento `<video>` em tela cheia com ajuste proporcional (`object-fit: contain` ou `cover`).
  - Respeito aos modos de exibição (Blackout oculta/pausa o vídeo; ClearText preserva o vídeo).
- No preview do operador (`AdminView`), espelhamento sincronizado com controles dedicados de reprodução (Play, Pause, Reiniciar, Barra de Progresso, Volume e Mudo).

### 2.5 Frontend — Item Timer na Tela Pública e Sincronização com o Palco
- Criação de Timer pelo menu *"+ Adicionar ▾"* da aba *Outros* ou modal dedicado:
  - Título/Rótulo (ex.: *"Contagem Regressiva"*, *"Início do Culto"*).
  - Duração em minutos e segundos (com presets: 1m, 2m, 3m, 5m, 10m, 15m, 30m).
  - Mensagem explicativa opcional (ex.: *"Prepare o seu coração para adorar"*).
  - Permissão de overtime e tempo de alerta visual.
- Quando apresentado ao vivo:
  - O timer global do aplicativo (`$presentationState.timer`) é acionado e iniciado automaticamente com a duração e rótulo configurados.
  - A tela de palco (`StageView`) sincroniza instantaneamente, exibindo o mesmo tempo restante para os ministros.
  - A tela pública (`PublicView`) renderiza um layout moderno e imersivo de contagem regressiva em tela cheia:
    - Tipografia grande e nítida com dígitos `MM:SS`.
    - Card em *glassmorphism* sobre o papel de parede ativo.
    - Animação de transição e alerta cromático (< 60s âmbar, zero/overtime aviso de início).
- No painel do operador (`AdminView`), controles rápidos de Pausar, Continuar, Reiniciar e adicionar tempo (+1 min).

---

## 3. Decisões Arquiteturais

1. **Reutilização do `MediaItem`:** Manter a estrutura unificada de `MediaItem` para que Áudios, Vídeos e Timers possam ser organizados em qualquer Setlist, reordenados por drag-and-drop, exportados e disparados pelo mesmo fluxo de apresentação.
2. **Sincronização Unidirecional de Timer via WebSocket:** O disparo de um `MediaItem` do tipo `Timer` emite a mensagem de sincronização do cronômetro para o backend (`set_timer` / `start_timer`), propagando o `TimerState` para todos os clientes conectados (Admin, Public, Stage e Remote).
3. **Reprodução de Áudio Local no Operador:** A saída de áudio dos cultos normalmente é conectada ao computador do operador (mesa de som). Por isso, o reprodutor de áudio roda no contexto do Admin, com opção de disparo a partir da setlist ou da aba de Áudios.
4. **Streaming de Vídeo:** Vídeos são servidos com suporte adequado a Range HTTP (206) e URLs relativas tratadas por `resolveMediaUrl`, garantindo compatibilidade multiplataforma.

---

## 4. Critérios de Aceite

1. **Aba Áudios:** O operador pode acessar a aba "Áudios" no `BottomPanel`, ver arquivos de áudio locais, reproduzir com Play/Pause/Seek/Volume e adicionar à setlist.
2. **Aba Vídeos:** O operador pode acessar a aba "Vídeos", visualizar a lista/grade de vídeos, disparar diretamente para a tela pública ou adicionar à setlist ativa.
3. **Apresentação de Vídeo:** Ao colocar o item de vídeo no ar, a tela pública (`/public`) exibe o vídeo em reprodução; ao acionar Blackout ou Parar, o vídeo é pausado/ocultado.
4. **Item Timer:** O operador pode criar um item de Timer com duração definida, adicioná-lo à setlist e apresentá-lo.
5. **Sincronização Public + Stage:** Ao apresentar o item de Timer, a tela pública exibe a contagem regressiva em destaque e o monitor de palco (`/stage`) exibe simultaneamente o mesmo tempo sincronizado.
6. **Portões de Qualidade:** `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run build` passam com 0 erros.

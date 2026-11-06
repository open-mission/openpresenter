# Spec — Ajustes de Timer da Setlist, Customização de Aparência, Áudio no Stage/Public com Waveforms e Pastas Mapeadas para Áudio e Vídeo

## 1. Intenção

Atender ao feedback operacional sobre os novos itens e abas de Áudio, Vídeo e Cronômetro (Timer):
1. **Sincronização Fiel do Timer da Setlist:** Ao disparar ou focar um item de Timer presente na setlist (ou na biblioteca Outros), o aplicativo deve apresentar os dados exatos daquele item (duração programada, título do culto/momento e mensagem personalizada), e não um valor residual esquecido no painel lateral de controle do operador.
2. **Configuração de Aparência do Timer na Tela Pública:** Disponibilizar na aba de Aparência/Tela Pública das Configurações (`SettingsModal.svelte`) opções completas para personalizar a exibição do cronômetro na tela pública: ativação/desativação da caixa de fundo (*box background* em *glassmorphism*), controle de opacidade do fundo, ajuste de tamanho de fonte dos dígitos numéricos, tamanho do título e tamanho da mensagem/subtítulo.
3. **Comportamento Padrão e Visual de Áudio (Public vs Stage):**
   - Na **Projeção Pública**, por padrão, a reprodução de um item de áudio não deve projetar nenhum elemento visual, mantendo a tela limpa com o papel de parede ativo. Uma nova opção configurável permitirá exibir uma arte elegante com visualizador de ondas sonoras (*waveforms*) animadas, metadados da faixa e barra de tempo decorrido.
   - No **Monitor de Palco (Stage View)**, exibir o status de reprodução do áudio ativo com tempo atual (`MM:SS`) e tempo restante (`-MM:SS`), proporcionando total ciência da duração da trilha para a liderança e equipe do altar.
4. **Mapeamento de Pastas Externas para Vídeos e Áudios:** Permitir aos operadores mapear pastas locais do computador (via botão "Pastas" nas abas de Vídeos e Áudios e na aba Pastas das Configurações), navegando e filtrando pastas sem necessidade de importar/duplicar arquivos pesados na pasta do app.

---

## 2. Escopo

### Backend (`op-core` e `op-admin`)
- Em `crates/op-core/src/lib.rs`:
  - Adicionar `AudioPlaybackState` (`is_playing`, `current_time_seconds`, `duration_seconds`, `title`) em `PresentationState`.
  - Adicionar mensagem WebSocket `WsMessage::UpdateAudioPlayback { is_playing, current_time_seconds, duration_seconds, title }`.
  - Adicionar mensagem WebSocket `WsMessage::ClearAudioPlayback`.
  - Adicionar suporte a deserialização tolerante de `SetTimer` tanto com campos em `payload` quanto campos planos no topo.
- Em `crates/op-admin/src/state.rs`:
  - Em `select_slide`, se a mídia for do tipo `MediaKind::Timer`, definir atomicamente `timer.duration_seconds`, `timer.remaining_seconds`, `timer.label`, `timer.stage_message` e `timer.running = true`. Manter `wallpaper_path` com o papel de parede do setlist (não substituir por fundo de letras).
  - Implementar métodos `update_audio_playback` e `clear_audio_playback` para sincronização em broadcast.
- Em `crates/op-admin/src/ws.rs`:
  - Tratar mensagens `UpdateAudioPlayback` e `ClearAudioPlayback`.
- Em `crates/op-admin/src/routes.rs`:
  - Registrar endpoints auxiliares `/api/presentation/audio-playback` e `/api/presentation/audio-playback/clear`.

### Frontend
- Em `frontend/src/lib/types.ts` e `frontend/src/lib/displaySettings.ts`:
  - Adicionar campos de configuração de aparência do timer: `timer_box_background`, `timer_bg_opacity`, `timer_digits_font_size`, `timer_title_font_size`, `timer_message_font_size`, `timer_text_color`.
  - Adicionar campo de áudio na tela pública: `audio_show_waveform_on_public` (default `false`).
- Em `frontend/src/lib/components/SettingsModal.svelte`:
  - Adicionar seção "Cronômetro / Timer na Tela Pública" e "Áudio na Tela Pública" com sliders, inputs numéricos e switches.
  - Atualizar a aba "Pastas de Mídia" esclarecendo que pastas mapeadas indexam Imagens, Vídeos e Áudios.
- Em `frontend/src/lib/views/PublicView.svelte` e `frontend/src/lib/components/PublicPreviewBox.svelte`:
  - Aplicar configurações de tamanho de fonte, opacidade e ativação da caixa de fundo no card do timer.
  - Para itens de áudio: por padrão, não sobrepor visual (fundo limpo com papel de parede). Se `audio_show_waveform_on_public` estiver ativo, renderizar arte estilizada com barras de *waveforms* animadas e tempos de áudio.
- Em `frontend/src/lib/views/StageView.svelte`:
  - Exibir indicador de áudio ativo no palco com título da faixa, tempo atual (`MM:SS`) e tempo restante (`-MM:SS`).
- Em `frontend/src/lib/components/BottomPanel.svelte`:
  - Na Aba Vídeos e na Aba Áudios: incluir botão "Pastas" (`FolderPlus`) e seletor/chips de pastas mapeadas.
- Em `frontend/src/lib/views/AdminView.svelte`:
  - Ao disparar slide de Timer via setlist ou double-click, sincronizar o timer com a duração e título do item.
  - Ao reproduzir áudios, sincronizar o progresso de áudio com o servidor via WebSocket.

---

## 3. Critérios de Aceite

1. Disparar um item de timer da setlist de 3 minutos altera a contagem regressiva para 03:00 e inicia o cronômetro sincronizado no Public e no Stage, substituindo qualquer timer residual anterior.
2. Nas configurações de Tela Pública, desativar a caixa de fundo remove a borda e o fundo escuro do timer; alterar os tamanhos de fonte reflete instantaneamente no Public e no PublicPreviewBox.
3. Ao tocar ou apresentar um áudio, a tela pública mantém o papel de parede ativo sem sobreposições por padrão. Se a opção de *waveforms* for ligada nas configurações, exibe card visual com ondas animadas.
4. No Stage View, é exibido o tempo atual e o tempo restante do áudio enquanto estiver em reprodução.
5. Nas abas de Vídeos e Áudios, o botão "Pastas" permite abrir a configuração de pastas e o filtro por pastas exibe os arquivos contidos em pastas externas mapeadas no sistema.
6. Todos os portões de qualidade (`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run build`) passam com zero erros e zero avisos.

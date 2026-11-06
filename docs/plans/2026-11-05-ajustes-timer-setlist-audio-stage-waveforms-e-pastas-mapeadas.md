# Plano — Ajustes de Timer da Setlist, Customização de Aparência, Áudio no Stage/Public com Waveforms e Pastas Mapeadas para Áudio e Vídeo

## Etapas de Implementação

### Etapa 1: Backend (`op-core` e `op-admin`)
- [ ] Modelar `AudioPlaybackState` e adicionar `audio: Option<AudioPlaybackState>` em `PresentationState` em `crates/op-core/src/lib.rs`.
- [ ] Adicionar mensagens `UpdateAudioPlayback` e `ClearAudioPlayback` em `WsMessage` em `crates/op-core/src/lib.rs`.
- [ ] Em `crates/op-admin/src/state.rs`:
  - Atualizar `select_slide`: se a mídia ativa for `MediaKind::Timer`, definir a duração correta a partir de `slide.content`, atualizar rótulo e mensagem e iniciar o timer. Manter o papel de parede do setlist.
  - Implementar métodos `update_audio_playback` e `clear_audio_playback`.
- [ ] Em `crates/op-admin/src/ws.rs`:
  - Tratar mensagens de atualização e limpeza de áudio no loop do socket.
- [ ] Em `crates/op-admin/src/routes.rs`:
  - Adicionar rotas REST auxiliares para playback de áudio se necessário.
- [ ] Validar testes automatizados com `cargo test --workspace`.

### Etapa 2: Tipos e Configurações de Aparência do Timer e Waveforms de Áudio
- [ ] Em `frontend/src/lib/types.ts`:
  - Estender `PublicDisplaySettings` com `timer_box_background`, `timer_bg_opacity`, `timer_digits_font_size`, `timer_title_font_size`, `timer_message_font_size`, `timer_text_color`, `audio_show_waveform_on_public`.
  - Estender `PresentationState` com `audio?: AudioPlaybackState | null`.
- [ ] Em `frontend/src/lib/displaySettings.ts`:
  - Adicionar valores padrão para as novas configurações.
- [ ] Em `frontend/src/lib/components/SettingsModal.svelte`:
  - Implementar seção de customização de aparência do Timer na Tela Pública (switch de caixa de fundo, slider de opacidade, slider/inputs para tamanho de fonte dos dígitos, título e mensagem).
  - Implementar switch para ativação de Waveforms de Áudio na Tela Pública.
  - Atualizar textos na aba Pastas para explicitar suporte a Imagens, Vídeos e Áudios.

### Etapa 3: Sincronização do Timer e Visualização na Tela Pública e Previews
- [ ] Em `frontend/src/lib/views/AdminView.svelte`:
  - Garantir que qualquer seleção/disparo de slide de item Timer (via duplo clique, atalhos, botão "Ao Vivo" do item) configure e inicie o timer com a duração e título do item.
- [ ] Em `frontend/src/lib/views/PublicView.svelte` e `frontend/src/lib/components/PublicPreviewBox.svelte`:
  - Aplicar as configurações de aparência do timer (caixa de fundo, opacidade, tamanhos de fonte dos dígitos e textos).
  - Em itens de áudio: por padrão, não sobrepor visual à tela pública (manter papel de parede limpo). Se `audio_show_waveform_on_public` estiver ativo, exibir card estilizado com visualizador de ondas (*waveforms*) animadas, título e tempo.

### Etapa 4: Exibição de Áudio no Stage View (`StageView.svelte`)
- [ ] Em `frontend/src/lib/views/StageView.svelte`:
  - Quando houver áudio ativo/em reprodução (`$presentationState.audio`), exibir caixa/chip de status no palco com título da faixa, tempo atual e tempo restante formatados.
- [ ] Em `frontend/src/lib/views/AdminView.svelte` e `BottomPanel.svelte`:
  - Emitir atualizações de tempo de áudio via WebSocket para manter o Stage sincronizado.

### Etapa 5: Filtro e Botão de Pastas nas Abas de Vídeos e Áudios
- [ ] Em `frontend/src/lib/components/BottomPanel.svelte`:
  - Na Aba Vídeos: adicionar botão "Pastas" (`FolderPlus`) chamando `onopenfolderconfig` e barra/chips de filtro por pasta mapeada.
  - Na Aba Áudios: adicionar botão "Pastas" (`FolderPlus`) chamando `onopenfolderconfig` e barra/chips de filtro por pasta mapeada.

### Etapa 6: Portões de Qualidade e Finalização
- [ ] Executar `cargo fmt --all`.
- [ ] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Executar `cargo test --workspace`.
- [ ] Executar `npm --prefix frontend run build`.
- [ ] Atualizar `CHANGELOG.md` e `TODO.md`.
- [ ] Commit e push conforme GitFlow.

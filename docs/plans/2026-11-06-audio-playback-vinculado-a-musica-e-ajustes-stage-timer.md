# Plano — Playback de Áudio Vinculado a Músicas, Widget de Player Superior/Remoto e Ajustes no Monitor de Palco (Áudio e Timer)

**Data:** 2026-11-06  
**Spec correspondente:** [`docs/specs/2026-11-06-audio-playback-vinculado-a-musica-e-ajustes-stage-timer.md`](../specs/2026-11-06-audio-playback-vinculado-a-musica-e-ajustes-stage-timer.md)

---

## Etapas de Implementação

### 1. Banco de Dados e Modelagem de Domínio
- [ ] Criar migração SQLite `crates/op-db/migrations/0013_song_audio_url.sql`:
  - `ALTER TABLE songs ADD COLUMN audio_url TEXT;`
- [ ] Em `crates/op-core/src/lib.rs`:
  - Adicionar `#[serde(default)] pub audio_url: Option<String>` em `Song`, `SongSummary` e `MediaItem`.
  - Em `Song::into_media_item`: propagar `audio_url: self.audio_url`.
  - Adicionar variante `ControlAudioPlayback` ao enum `WsMessage`.
- [ ] Em `crates/op-db/src/lib.rs`:
  - Incluir migração `0013_song_audio_url.sql` em `Migrations::new`.
  - Atualizar `save_song` e `save_songs_batch` com a coluna `audio_url`.
  - Atualizar `get_song` para ler `audio_url`.
  - Atualizar asserts de schema version de 12 para 13 nos testes unitários de migração.

### 2. Backend Axum & WebSocket
- [ ] Em `crates/op-admin/src/ws.rs`:
  - Tratar `WsMessage::ControlAudioPlayback` fazendo broadcast para todos os clientes conectados.

### 3. Frontend — Módulo Global de Áudio (`audioPlayer.ts`)
- [ ] Criar ou estruturar `frontend/src/lib/audioPlayer.ts`:
  - Gerenciador singleton com elemento HTMLAudioElement para o host desktop.
  - Sincronização periódica (1s) via `syncAudioWs` (`update_audio_playback` e `clear_audio_playback`).
  - Funções exportadas: `playTrack`, `togglePlay`, `pause`, `stop`, `seek`, `setVolume`, `toggleMute`.
  - Handler de WebSocket para comandos remotos `control_audio_playback`.

### 4. Frontend — Ajustes no Monitor de Palco (`StageView.svelte`)
- [ ] Corrigir `isCurrentImage` e `isNextImage` para ignorar mídias de `audio`, `video` e `timer`.
- [ ] Reordenar a renderização do bloco central de slide para priorizar `activeMedia?.kind === 'audio'`.
- [ ] Ajustar `isStandbyMode`: incluir `activeMedia?.kind === 'timer'` para manter tela de descanso normal com o relógio gigante e o cronômetro centralizado.

### 5. Frontend — Vinculação no Cadastro de Músicas (`SongEditorModal.svelte`)
- [ ] Adicionar seletor de "Áudio / Playback Vinculado" com lista de áudios disponíveis em `/api/media/audios`.
- [ ] Adicionar botão de pré-escuta (play/pause) no modal de edição.
- [ ] Salvar `audio_url` no objeto `Song` e repassar para `MediaItem`.

### 6. Frontend — Widget de Player Superior no Operador (`AdminView.svelte`)
- [ ] No `<header>` do `AdminView.svelte`, ao lado do botão de busca (⌘K):
  - Renderizar widget de player quando a música selecionada/ao vivo possuir `audio_url` ou quando houver áudio tocando.
  - Exibir Play/Pause, Stop, título da faixa, barra de progresso e timecode.

### 7. Frontend — Controle Remoto de Playback (`RemoteView.svelte`)
- [ ] Na aba Setlist do `RemoteView.svelte`:
  - Exibir badge ou indicador de playback nos itens vinculados.
  - Quando a música estiver selecionada ou ao vivo, exibir card de controle com Play/Pause/Stop enviando mensagem de controle via WebSocket para o computador da igreja.

### 8. Validação e Qualidade
- [ ] `cargo fmt --all`.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] `cargo test --workspace`.
- [ ] `npm --prefix frontend run build`.
- [ ] Atualizar `CHANGELOG.md`, `TODO.md` e `docs/README.md`.
- [ ] Commit seguindo o GitFlow.

# Plano de Implementação — Itens e Abas de Áudio, Vídeo e Timer Público

**Data:** 2026-11-05  
**Spec Correspondente:** [`docs/specs/2026-11-05-audio-video-e-timer-publico.md`](../specs/2026-11-05-audio-video-e-timer-publico.md)  
**Status:** Em Execução  

---

## Etapas de Implementação

### Etapa 1: Backend Domain & Modelos (`op-core`, `op-admin`)
- [ ] Adicionar variantes `Audio`, `Video` e `Timer` ao enum `MediaKind` em `crates/op-core/src/lib.rs`.
- [ ] Atualizar métodos auxiliares `label_pt_br()` e testes de serialização em `op-core`.
- [ ] Criar structs `AudioMediaItem` e `VideoMediaItem` em `crates/op-admin/src/routes.rs`.
- [ ] Adicionar funções de escaneamento de arquivos de áudio (`.mp3`, `.wav`, `.ogg`, `.aac`, `.m4a`, `.flac`) e vídeo (`.mp4`, `.webm`, `.mov`, `.mkv`) em `crates/op-admin/src/routes.rs`.
- [ ] Implementar rotas `/api/media/audios`, `/api/media/audios/upload`, `/api/media/videos`, `/api/media/videos/upload`.
- [ ] Assegurar suporte a Range HTTP e subpastas conhecidas (`audios`, `videos`) em `serve_media_file`.
- [ ] Documentar novas rotas no OpenAPI 3.0.3 (`api_docs.rs`).

### Etapa 2: Tipos Frontend e Gerenciamento de Mídias
- [ ] Atualizar tipos TypeScript em `frontend/src/lib/types.ts`:
  - Atualizar `MediaKind = ... | 'audio' | 'video' | 'timer'`.
  - Definir interfaces `AudioMediaItem` e `VideoMediaItem`.
- [ ] Criar helpers de API em `frontend/src/lib/media.ts` (ou módulo dedicado):
  - `fetchAudioItems()`, `fetchVideoItems()`, upload de áudio e vídeo.
- [ ] Atualizar `customItems.ts` com suporte nativo à criação e persistência de itens do tipo `timer`.

### Etapa 3: Abas de Áudio e Vídeo no `BottomPanel.svelte`
- [ ] Expandir abas do painel inferior para incluir `audios` e `videos` com ícones `Volume2` e `Film`.
- [ ] Implementar a aba **Áudios**:
  - Busca, upload e lista de faixas de áudio.
  - Reprodutor de áudio integrado (Play/Pause, barra de progresso seek, tempo decorrido/total, volume, loop).
  - Botão "Adicionar à Setlist" (cria `MediaItem` com `kind: 'audio'`).
- [ ] Implementar a aba **Vídeos**:
  - Busca, upload e grade de vídeos com cartões visuais.
  - Botão "Apresentar ao Vivo" (quick present).
  - Botão "Adicionar à Setlist" (cria `MediaItem` com `kind: 'video'`).

### Etapa 4: Projeção de Vídeo e Áudio (`PublicView.svelte`, `AdminView.svelte`)
- [ ] Em `PublicView.svelte`:
  - Renderizar `<video>` quando `activeMedia?.kind === 'video'` com reprodução fluida, autoplay, loop configurável e respeito ao Blackout/ClearText.
- [ ] Em `AdminView.svelte`:
  - Espelhar o vídeo no monitor de Preview Public com barra de controles multimídia (Play/Pause/Seek/Mute).
  - Barra de controle de áudio ativa para faixas em execução.

### Etapa 5: Item Timer na Tela Pública e Sincronização com o Palco
- [ ] Modal de criação/edição de Timer (no menu "+ Adicionar ▾" / "Outros"):
  - Título, duração (com presets rápidos 1m..30m), mensagem de apoio e alerta visual.
- [ ] Disparo e Apresentação:
  - Ao apresentar item do tipo `timer`: sincronizar duração e rótulo com `$presentationState.timer` e disparar cronômetro.
- [ ] Em `PublicView.svelte`:
  - Renderizar tela de Contagem Regressiva imersiva com dígitos grandes `MM:SS`, card translúcido e animação sobre o papel de parede ativo.
- [ ] Em `StageView.svelte`:
  - Confirmar sincronização perfeita em tempo real com o mesmo timer.
- [ ] Em `AdminView.svelte`:
  - Controles rápidos de pausa, reinício e +1 min no painel e nos previews.

### Etapa 6: Portões de Qualidade e Documentação
- [ ] Executar `cargo fmt --all`.
- [ ] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Executar `cargo test --workspace`.
- [ ] Executar `npm --prefix frontend run build`.
- [ ] Atualizar `CHANGELOG.md`, `TODO.md` e `docs/README.md`.
- [ ] Commit e encerramento seguro.

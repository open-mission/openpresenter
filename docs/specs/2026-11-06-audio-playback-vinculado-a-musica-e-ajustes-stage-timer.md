# Spec — Playback de Áudio Vinculado a Músicas, Widget de Player Superior/Remoto e Ajustes no Monitor de Palco (Áudio e Timer)

**Data:** 2026-11-06  
**Status:** Aprovada  
**Contexto:** No monitor de palco (`/stage`), um item de áudio ativo estava sendo incorretamente interpretado como imagem devido a uma checagem abrangente de prefixos de URL, renderizando uma tag `<img>` com falha de carregamento em vez dos tempos de reprodução (decorrido e restante). Além disso, quando um timer é apresentado na tela pública, o monitor de palco deve manter sua tela de descanso normal com o relógio gigante e o timer centralizado, em vez do layout de slides. Por fim, cultos frequentemente utilizam trilhas de playback para acompanhar as músicas cantadas; portanto, é necessário permitir vincular um arquivo de áudio a uma música (no cadastro/edição) e exibir um widget de player compacto ao lado do campo de busca superior no console do operador (`AdminView`) e na setlist do remoto mobile (`RemoteView`), possibilitando disparar o playback no sistema de som principal enquanto as letras dos slides continuam sendo apresentadas normalmente nas telas.

---

## 1. Intenção e Objetivos

1. **Correção do Monitor de Palco para Itens de Áudio (`/stage`):**
   - Garantir que itens de áudio (`activeMedia.kind === 'audio'`) nunca sejam interpretados como imagens na tela de palco.
   - Exibir na área principal ou no banner superior o ícone de áudio, título da faixa, timecode decorrido (`MM:SS`) e tempo restante (`-MM:SS`).
2. **Tela de Descanso Normal no Palco com Timer em Execução:**
   - Quando um item de Cronômetro/Timer estiver sendo apresentado no Telão Público (`activeMedia.kind === 'timer'`), o monitor de palco (`StageView.svelte`) deve permanecer no modo de descanso/standby tradicional: relógio gigante no centro, data, eventuais alertas/mensagens no topo e o bloco do cronômetro sincronizado com contagem regressiva abaixo do relógio.
3. **Vinculação de Playback de Áudio a Músicas:**
   - Adicionar o campo `audio_url` no modelo `Song` do catálogo e no `MediaItem` da setlist.
   - Criar a migração SQLite `0013_song_audio_url.sql`.
   - No `SongEditorModal.svelte`, incluir seletor/upload de áudio vinculado com botão de pré-escuta rápida.
4. **Widget de Player no Topo do AdminView e no Remoto Mobile:**
   - No cabeçalho superior do operador (`AdminView.svelte`), ao lado do campo de busca global (⌘K), exibir um widget de player quando a música selecionada ou ativa possuir áudio vinculado (ou sempre que um áudio estiver em reprodução).
   - O widget deve conter botões de Play/Pause, Stop, tempo decorrido / duração total, barra de progresso (seek) e controle de volume.
   - Na setlist do `/remote`, exibir badge de playback nos itens vinculados e um card de controle com Play/Pause e barra de tempo para que o operador remoto possa disparar o playback no computador principal da igreja.
   - Comunicação e controle via WebSocket com a nova mensagem `ControlAudioPlayback`.

---

## 2. Escopo

- **Banco de Dados e Core:**
  - Migração `crates/op-db/migrations/0013_song_audio_url.sql`.
  - Atualização do schema version para 13 em `crates/op-db/src/lib.rs`.
  - Campos `audio_url: Option<String>` em `Song`, `SongSummary` e `MediaItem` em `crates/op-core/src/lib.rs`.
  - Mensagem WebSocket `WsMessage::ControlAudioPlayback` em `crates/op-core/src/lib.rs`.
  - Persistência e leitura de `audio_url` em `save_song`, `save_songs_batch` e `get_song` em `crates/op-db/src/lib.rs`.
- **Backend Axum (`crates/op-admin`):**
  - Broadcast de `ControlAudioPlayback` no WebSocket para sincronização remota entre clientes.
- **Frontend:**
  - Módulo centralizado de áudio (`frontend/src/lib/audioPlayer.ts`) gerenciando reprodução de áudio, seek, volume e envio WebSocket de `UpdateAudioPlayback`.
  - `StageView.svelte`:
    - Correção de `isCurrentImage` para excluir mídias de áudio/vídeo/timer.
    - Condição `isStandbyMode` estendida para `activeMedia?.kind === 'timer'`, mantendo relógio e timer centralizados.
  - `SongEditorModal.svelte`: campo de seleção de playback com listagem de áudios locais e pré-visualização sonora.
  - `AdminView.svelte`: widget de player no cabeçalho superior ao lado da barra de busca.
  - `RemoteView.svelte`: indicador de playback e card de controle de reprodução na aba Setlist.

---

## 3. Critérios de Aceite

1. **Monitor de Palco com Áudio:**
   - Ao apresentar uma faixa de áudio avulsa ou da setlist, o monitor de palco (`/stage`) não renderiza mais elemento `<img>` com erro; exibe os dados da faixa e os cronômetros decorrido (`MM:SS`) e restante (`-MM:SS`).
2. **Monitor de Palco com Timer:**
   - Ao projetar um item de timer no Public, o monitor de palco exibe o relógio central em tamanho grande, a data e o cronômetro sincronizado no formato de descanso, sem renderizar a grade de slides.
3. **Música com Áudio Vinculado:**
   - É possível selecionar um arquivo de áudio para uma música no `SongEditorModal` e salvar no banco de dados.
   - Ao selecionar ou apresentar a música no operador, o widget de player aparece ao lado da busca superior.
   - Clicar em Play no widget toca a música no computador principal e sincroniza os tempos com o monitor de palco em tempo real.
   - O operador consegue navegar nos slides de letras da música normalmente enquanto a trilha de playback toca.
4. **Remoto Mobile (`/remote`):**
   - Ao acessar uma música com playback na setlist do `/remote`, o operador remoto pode dar Play/Pause/Stop, acionando a execução no computador principal da igreja.
5. **Portões de Qualidade:**
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run build` passam com 100% de sucesso.

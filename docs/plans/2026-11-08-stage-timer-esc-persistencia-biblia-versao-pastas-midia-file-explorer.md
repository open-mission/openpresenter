# Plano — Desacoplamento de Parada da Apresentação do Timer de Palco, Correção de Troca de Versão da Bíblia e Gerenciamento de Pastas com File Explorer em Mídias

Este plano orienta a implementação das mudanças especificadas em [`docs/specs/2026-11-08-stage-timer-esc-persistencia-biblia-versao-pastas-midia-file-explorer.md`](../specs/2026-11-08-stage-timer-esc-persistencia-biblia-versao-pastas-midia-file-explorer.md).

---

## Etapas de Execução

### Etapa 1: Backend — Desacoplar Parada de Apresentação do Timer (`op-admin`)
- [x] No arquivo `crates/op-admin/src/state.rs`:
  - No método `stop_presentation(&self)`, remover as linhas que resetam o timer (`state.timer.running = false; state.timer.remaining_seconds = state.timer.duration_seconds; state.timer.stage_message = None;`).
  - Manter a restauração do papel de parede do setlist e a limpeza dos campos de slide/mídia ao vivo.
- [x] No arquivo `crates/op-admin/src/main.rs`:
  - Atualizar os testes `stop_presentation_and_set_wallpaper_work` e `test_timer_presentation_reset_and_stop_behavior` para verificar que `stop_presentation` preserva o timer ativo e sua contagem contínua.
  - Adicionar teste específico garantindo que chamar `stop_presentation` repetidas vezes não afeta o timer ativo.

### Etapa 2: Backend — Endpoints de Pastas Físicas e Upload com Subpastas (`op-admin`)
- [x] No arquivo `crates/op-admin/src/routes.rs`:
  - Implementar estrutura `CreateDirectoryPayload` contendo `kind`, `folder_id`, `parent_subfolder` e `name`.
  - Criar rota `POST /api/media/folders/create-dir` para criação física de diretórios sanitizados em `<data_dir>/media/{kind}/{parent_subfolder}/{name}` ou em pastas mapeadas.
  - Criar rota `GET /api/media/app-subfolders` para listar todas as subpastas físicas presentes em `<data_dir>/media/images` e `<data_dir>/media/videos`, incluindo subpastas vazias com contagem 0.
  - Atualizar `POST /api/media/images/upload` e `POST /api/media/videos/upload` para aceitar `subfolder` opcional e salvar o arquivo no diretório relativo correspondente.
- [x] Registrar as novas rotas e atualizar `crates/op-admin/src/api_docs.rs` se aplicável.
- [x] Adicionar testes automatizados no `crates/op-admin/src/main.rs` para criação de diretório físico, listagem de subpastas vazias e upload em subpastas.

### Etapa 3: Frontend — Desacoplar Timer no Escape e Apresentação (`AdminView.svelte`)
- [x] No arquivo `frontend/src/lib/views/AdminView.svelte`:
  - No método `stopPresentation()`, remover as chamadas `pauseTimer()` e `resetTimer()`.
  - Garantir que a tecla `Escape` encerre apenas a projeção de telas públicas e mantenha o cronômetro do palco rodando.

### Etapa 4: Frontend — Correção da Troca de Versão da Bíblia (`BiblePanel.svelte`)
- [x] No arquivo `frontend/src/lib/components/BiblePanel.svelte`:
  - Ajustar o `<select>` de versão para extrair `(e.currentTarget as HTMLSelectElement).value` no evento `onchange` e chamar `handleVersionChange(newId)`.
  - Atualizar a chave do loop `{#each verses as v (...)}` para `${selectedBibleId}-${v.book_id}-${v.chapter}-${v.verse}`.
  - Atualizar o efeito de `targetPassage` para incluir `targetPassage.version` na verificação de chave.
  - Em `jumpToPassage` e no parser de referências, garantir o uso imediato do ID da versão encontrada para recarregar livros, capítulos e versículos daquela passagem na nova versão.

### Etapa 5: Frontend — Criação de Pastas e File Explorer nas Abas de Mídias (`BottomPanel.svelte` e `media.ts`)
- [x] No arquivo `frontend/src/lib/media.ts`:
  - Adicionar função `createMediaDirectory(kind, folderId, name, parentSubfolder)`.
  - Adicionar função `fetchAppSubfolders(kind)`.
- [x] No arquivo `frontend/src/lib/components/BottomPanel.svelte`:
  - Adicionar estado e modal de criação de pasta ("Nova Pasta").
  - Adicionar botão "+ Pasta" na barra de ações das abas de Imagens e Vídeos.
  - Na aba de Imagens: integrar cartões de pasta estilo File Explorer na grade de conteúdo, permitindo clique para entrar, voltar com breadcrumbs e criar subpastas no nível atual.
  - Na aba de Vídeos: replicar a mesma navegação por árvore/pastas estilo File Explorer, permitindo entrar em subpastas de vídeos e exibir vídeos da pasta ativa.
  - Ajustar importação de imagens e vídeos para enviar `subfolder: selectedSubfolder` quando o usuário estiver dentro de uma subpasta.

### Etapa 6: Validação de Qualidade e Documentação
- [x] Atualizar `docs/README.md` linkando a nova spec e o novo plano.
- [x] Atualizar `CHANGELOG.md` e `TODO.md`.
- [x] Executar portões de qualidade:
  - `cargo fmt --all`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `npm --prefix frontend run build`

# Plano de Execução: Refinamento de UX/UI e Correção de Bugs do Controle Remoto Mobile (/remote)

## 1. Fase 1: Utilitários e Helpers
- [ ] Criar/atualizar `frontend/src/lib/remoteData.ts`:
  - Adicionar suporte a feedback háptico (`triggerHaptic`).
  - Adicionar helper universal para extração e parsing de slides de músicas (`extractSongSlides(song: SongSummary | MediaItem)`).
  - Validar helpers existentes de formatação de tempo e autorização.

## 2. Fase 2: Componentes Modais Dedicados
- [ ] Criar `frontend/src/lib/components/remote/RemoteSetlistSheet.svelte`:
  - Sheet acessível para seleção de setlists salvas.
- [ ] Criar `frontend/src/lib/components/remote/RemoteBibleSheet.svelte`:
  - Sheet acessível para seleção de versão bíblica ativa.
- [ ] Criar `frontend/src/lib/components/remote/RemotePinDialog.svelte`:
  - Diálogo acessível para autenticação de PIN do operador.
- [ ] Criar `frontend/src/lib/components/remote/RemoteStopDialog.svelte`:
  - Confirmação acessível de parada com `role="alertdialog"`.

## 3. Fase 3: Correções Funcionais nos Subcomponentes
- [ ] Atualizar `frontend/src/lib/components/remote/RemoteSongsTab.svelte`:
  - Corrigir parsing de estrofes utilizando `extractSongSlides` com suporte a `lyrics_raw` e `sections`.
  - Exibir `preview_snippet` nos resultados de busca com destaque.
  - Aplicar feedback háptico no disparo e seleção de estrofe.
- [ ] Atualizar `frontend/src/lib/components/remote/RemoteBibleTab.svelte`:
  - Garantir navegação estável entre Versículos, Livros e Capítulos.
  - Aplicar feedback háptico no envio de versículos.
- [ ] Atualizar `frontend/src/lib/components/remote/RemoteLiveControlCard.svelte`:
  - Implementar transição entre itens da setlist ativa no avanço/recuo de slides.
  - Exibir indicação visual do próximo item quando estiver no último slide da mídia atual.
  - Aplicar feedback háptico nos botões de slide, colapso e modo seguro.
- [ ] Atualizar `frontend/src/lib/components/remote/RemoteHeader.svelte`:
  - Adicionar feedback háptico nos botões de controle de tela pública (Preto, Limpar, Parar).

## 4. Fase 4: Orquestração e Modularização de `RemoteView.svelte`
- [ ] Criar `frontend/src/lib/components/remote/controller.svelte.ts`:
  - Encapsular estado reativo da sessão, listas, bíblia, equipe e WebSocket.
  - Corrigir rota de versículos para incluir `/verses`.
  - Corrigir despachos ao vivo para usar `quickPresentMedia` e `selectSlide`.
- [ ] Refatorar `frontend/src/lib/views/RemoteView.svelte`:
  - Reduzir o monólito de 1.150 linhas para ~180 linhas limpas, delegando responsabilidades aos submódulos.

## 5. Fase 5: Validação, Testes e Portões de Qualidade
- [ ] Executar `cd frontend && npm run check` (assegurar 0 erros e 0 warnings).
- [ ] Executar `cd frontend && npm run build` (garantir build de produção impecável).
- [ ] Executar `cargo test -p op-db -p op-admin` (78/78 testes passando).
- [ ] Executar `npx impeccable detect frontend/src --json` (0 antipadrões).
- [ ] Testar ao vivo no navegador com `openbot_browser` em viewport móvel (`390×844`).
- [ ] Gerar snapshot pós-fix do Impeccable Critique.
- [ ] Atualizar `CHANGELOG.md` e `docs/README.md`.
- [ ] Commit com trailer `Co-authored-by: OpenBot <335604065+openbotgit[bot]@users.noreply.github.com>`.
- [ ] Publicar resultado no canal compartilhado via `openbot.channel_result`.

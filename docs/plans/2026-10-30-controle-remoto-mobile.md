# Plano de Execução — Controle Remoto Mobile (/remote)

- **Status:** Em andamento
- **Spec associada:** [`docs/specs/2026-10-30-controle-remoto-mobile.md`](../specs/2026-10-30-controle-remoto-mobile.md)
- **Data:** 2026-10-30

## Etapas

1. **Backend Rust:**
   - [x] Atualizar `NetworkInfoResponse` em `crates/op-admin/src/routes.rs` para incluir `remote_url`.
   - [x] Atualizar banner do console em `crates/op-admin/src/main.rs` para exibir a URL do Controle Remoto.
   - [x] Adicionar item de menu "Abrir Controle Remoto (Mobile)" no System Tray em `crates/op-admin/src/tray.rs` e no loop de eventos do `main.rs`.
   - [x] Assegurar que `crates/op-admin/src/ws.rs` mapeie `client_type == "remote"` para rota default `/remote`.
2. **Frontend Tipos e Roteamento:**
   - [x] Atualizar `NetworkInfoResponse` em `frontend/src/lib/types.ts` com `remote_url: string`.
   - [x] Atualizar `frontend/src/App.svelte` para reconhecer rota `/remote` e carregar `RemoteView.svelte`.
   - [x] Atualizar `frontend/src/lib/ws.ts` para suportar `clientType: 'remote'`.
3. **Desenvolvimento da Interface Mobile (`RemoteView.svelte`):**
   - [x] Cabeçalho com indicador de status de conexão WebSocket, nome do app e ações de emergência (Blackout, Limpar, Parar).
   - [x] Card persistente com slide atualmente ao vivo (título, índice, prévia e botões gigantes de Anterior / Próximo para o polegar).
   - [x] Barra de navegação inferior com 3 abas ergonômicas (Setlist, Músicas, Bíblia).
   - [x] Implementação da aba **Setlist**: listagem de itens da setlist ativa, seleção de slide com um toque.
   - [x] Implementação da aba **Músicas**: busca reativa com debounce, catálogo, prévia de estrofes e projeção direta.
   - [x] Implementação da aba **Bíblia**: seletor de versão bíblica, livros, capítulos e versículos com toque para projeção ao vivo imediata.
4. **Compartilhamento e QR Code:**
   - [x] Atualizar `frontend/src/lib/components/ShareOutputsModal.svelte` adicionando a aba "Controle Remoto" com QR Code para leitura rápida via câmera do celular.
   - [x] Atualizar cabeçalho do operador em `frontend/src/lib/views/AdminView.svelte` para oferecer acesso rápido ao modal com a aba de Controle Remoto.
5. **Validação e Portões de Qualidade:**
   - [x] `cargo fmt --all -- --check`
   - [x] `cargo clippy --workspace --all-targets -- -D warnings`
   - [x] `cargo test --workspace`
   - [x] `npm run check` & `npm run build`
6. **Atualização de Documentação e TODO:**
   - [x] Atualizar `docs/README.md`.
   - [x] Atualizar `TODO.md` e mover Item 1 para `Doing` -> `Done` com commit e timestamp.
   - [x] Atualizar `CHANGELOG.md`.

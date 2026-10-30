# Plano de Execução — Saída Lower Third Transparente para OBS (/lower-third)

- **Status:** Em andamento
- **Spec associada:** [`docs/specs/2026-10-30-saida-lower-third-obs.md`](../specs/2026-10-30-saida-lower-third-obs.md)
- **Data:** 2026-10-30

## Etapas

1. **Backend Rust:**
   - [x] Atualizar `NetworkInfoResponse` em `crates/op-admin/src/routes.rs` para incluir `lower_third_url`.
   - [x] Atualizar banner de console em `crates/op-admin/src/main.rs` com URL do Lower Third.
   - [x] Adicionar atalho no menu do System Tray em `crates/op-admin/src/tray.rs` e no loop de eventos do `main.rs`.
   - [x] Suportar identificação WebSocket de cliente `lower-third` em `crates/op-admin/src/ws.rs`.
2. **Tipos e Gerenciador de Configurações no Frontend:**
   - [x] Definir interface `LowerThirdSettings` em `frontend/src/lib/types.ts`.
   - [x] Criar store e valores padrão em `frontend/src/lib/displaySettings.ts` com sincronização e persistência.
   - [x] Atualizar `NetworkInfoResponse` no frontend.
3. **Desenvolvimento da Visualização `/lower-third` (`LowerThirdView.svelte`):**
   - [x] Fundo 100% transparente compatível com OBS Studio Browser Source.
   - [x] Posicionamento no terço inferior da tela.
   - [x] Suporte a estilos de caixa: Gradiente Central, Barra Inferior, Caixa Flutuante e Minimalista.
   - [x] Tipografia com sombras configuráveis (`drop-shadow` / `text-shadow`) para máxima legibilidade.
   - [x] Animações de transição suave entre slides, blackout e cleartext.
   - [x] Badge elegante de referência bíblica ou título de música.
4. **Aba de Configurações do Lower Third (`SettingsModal.svelte`):**
   - [x] Adicionar aba "Lower Third (OBS)" na barra lateral de configurações.
   - [x] Controles de estilo, cores, gradiente, tamanho de fonte, força de sombra e animação.
   - [x] Prévia em tempo real 16:9 com simulação de transmissão de vídeo de fundo.
5. **Integração no Modal de Saídas e Roteamento:**
   - [x] Atualizar `frontend/src/App.svelte` para rotear `/lower-third` (e alias `/lower` e `/obs`).
   - [x] Atualizar `frontend/src/app.css` para permitir fundo transparente na rota `/lower-third`.
   - [x] Atualizar `ShareOutputsModal.svelte` com aba dedicada para OBS Studio contendo URL, instruções e botão de cópia.
6. **Portões de Qualidade e Atualização de Documentação:**
   - [x] `cargo fmt --all -- --check`
   - [x] `cargo clippy --workspace --all-targets -- -D warnings`
   - [x] `cargo test --workspace`
   - [x] `npm run check` & `npm run build`
   - [x] Atualizar `docs/README.md`.
   - [x] Atualizar `TODO.md` movendo o item para `Done` com commit e timestamp.
   - [x] Atualizar `CHANGELOG.md`.

# Plano — Correções no Lower Third e PWA Remote iOS

## Etapas de Execução

1. **Manifesto PWA do Controle Remoto:**
   - [x] Criar `frontend/public/manifest-remote.webmanifest` com `start_url: "/remote"` e `scope: "/remote"`.
   - [x] Atualizar `frontend/src/App.svelte` para alternar dinamicamente o link do manifesto e título do iOS, além de redirecionar PWA standalone mobile na raiz `/` para `/remote`.

2. **Ajustes de Resolução e Renderização no Lower Third:**
   - [x] Atualizar `frontend/src/lib/views/LowerThirdView.svelte`:
     - Importar `mediaItemsList` e `activeSetlist`.
     - Implementar resolução resiliente de `activeMedia` (quick_media, mediaItemsList, activeSetlist, fetch fallback).
     - Atribuir classes `is-lower-third` em `document.documentElement` e `document.body`.
     - Suportar prévia interativa com simulação de câmera e estado limpo informativo.
   - [x] Atualizar `frontend/src/app.css` para aplicar `background-color: transparent !important` com as classes `is-lower-third`.

3. **Portões de Qualidade e Documentação:**
   - [x] Executar `npm run check` e `npm run build` no frontend.
   - [x] Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.
   - [x] Atualizar `docs/README.md`, `CHANGELOG.md` e `TODO.md`.
   - [x] Commitar as alterações de forma organizada.

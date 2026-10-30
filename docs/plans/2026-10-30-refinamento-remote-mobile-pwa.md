# Plano — Refinamento do Controle Remoto Mobile e Suporte a PWA

## Etapas de Execução

1. **Configuração de PWA e Assets:**
   - [x] Gerar ícones PWA nos tamanhos 192x192 e 512x512 a partir do branding oficial.
   - [x] Criar `frontend/public/manifest.webmanifest` com configurações standalone, ícones, tema e atalhos.
   - [x] Criar `frontend/public/sw.js` com service worker para registro PWA e cache básico.
   - [x] Atualizar `frontend/index.html` com `viewport-fit=cover`, meta tags PWA para iOS e manifesto linkado.
   - [x] Registrar o Service Worker em `frontend/src/main.ts`.

2. **Correção do Carregamento de Setlist:**
   - [x] Modificar `loadActiveSetlist()` em `RemoteView.svelte` para buscar detalhes em `/api/setlists/{id}`.
   - [x] Garantir que o array de itens seja sempre acessível com fallback seguro para evitar exceções de renderização.

3. **Refatoração da Interface do Remoto (`RemoteView.svelte`):**
   - [x] **Header:** Aumentar dimensões de toque, incluir padding de safe-area superior e integrar o logotipo oficial do OpenPresenter.
   - [x] **Card de Controle ao Vivo:** Adicionar botão de toggle de colapso (*collapse/expand*) e renderização compacta quando recolhido.
   - [x] **Bíblia:**
     - Implementar sub-abas: **Livros**, **Capítulos** e **Versículos**.
     - Livros em grade (grid com 3 colunas e touch targets confortáveis), com filtro por Testamento (Todos, AT, NT).
     - Capítulos em grade de botões quadrados (5 a 6 colunas com mínimo de 44px).
     - Versículos em lista com texto completo e destaque de versículo ativo ao vivo.
     - *Input Group* na busca e seleção de versão.
     - Drawer/Bottom Sheet Shadcn com lista de versões da Bíblia e seleção interativa.
   - [x] **Músicas:**
     - Adicionar estado ativo (*active state*) com borda verde esmeralda e badge nas estrofes em exibição ao vivo e na listagem geral.
   - [x] **Bottom Navigation Bar:**
     - Aplicar padding inferior da área segura (`safe-area-inset-bottom`) e aumentar dimensões táteis dos botões.

4. **Verificação dos Portões de Qualidade:**
   - [x] Executar `cd frontend && npm run check`.
   - [x] Executar `cd frontend && npm run build`.
   - [x] Executar `cargo fmt --all -- --check`.
   - [x] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
   - [x] Executar `cargo test --workspace`.
   - [x] Atualizar `docs/README.md`, `CHANGELOG.md` e `TODO.md`.
   - [ ] Commitar as alterações.

# Especificação — Correções no Lower Third e PWA Remote iOS

**Data:** 2026-10-30  
**Status:** Aprovado  
**Escopo:** `frontend/src/lib/views/LowerThirdView.svelte`, `frontend/src/App.svelte`, `frontend/public/manifest-remote.webmanifest`, `frontend/src/app.css`

---

## 1. Contexto e Problemas Identificados

1. **Lower Third (`/lower-third`) sem exibição de dados e tela branca no navegador:**
   - Em `LowerThirdView.svelte`, a resolução de mídia ativa (`activeMedia`) observava estritamente `$presentationState.quick_media`. Ao projetar músicas regulares da biblioteca ou de setlists, `quick_media` é nulo, fazendo com que `currentSlide` fosse nulo e nenhum dado fosse renderizado no DOM.
   - Quando aberto em um navegador convencional (ex: Chrome ou Safari) para conferência ou teste, uma página sem fundo explícito é desenhada pelo canvas padrão do navegador em branco. Sem slides renderizados, a tela ficava totalmente branca sem qualquer indicação de status.
   - A transparência para o OBS dependia exclusivamente da pseudo-classe CSS `:has(.lower-third-root)`, que pode não ser interpretada por versões mais antigas do Chromium Embedded Framework (CEF).

2. **PWA no iOS redirecionando para a raiz `/` em vez de `/remote`:**
   - O arquivo `manifest.webmanifest` definia `"start_url": "/"` e `"scope": "/"`. Ao usar a ação "Adicionar à Tela de Início" do Safari no iPhone a partir da rota `/remote`, o iOS gravava o webapp apontando para a raiz `/` (painel desktop de administração).

---

## 2. Decisões de Projeto

1. **Resolução de Mídia no Lower Third:**
   - Importar `mediaItemsList` e `activeSetlist` no `LowerThirdView.svelte`, replicando a estratégia do `StageView` e `PublicView`.
   - Adicionar busca em cascata: `quick_media` → `$mediaItemsList` → `$activeSetlist.items` → fetch assíncrono sob demanda `/api/songs/:id` como fallback caso o cliente conecte isolado.
   - Tratar a referência e rótulos bíblicos (`currentSlide.label` prioritário para versículos como "João 3:16").

2. **Transparência e Experiência no Navegador:**
   - Injetar classes explícitas `html.is-lower-third` e `body.is-lower-third` no ciclo de vida de `LowerThirdView`.
   - Adicionar suporte a simulação de câmera de vídeo no navegador com parâmetro `?preview=1` ou botão interativo discreto para testes no desktop.
   - Exibir um estado informativo quando não houver slide em exibição no navegador (oculto no OBS ou quando um slide estiver ao vivo), eliminando a tela branca vazia.

3. **Manifesto Dedicado para o Controle Remoto:**
   - Criar `frontend/public/manifest-remote.webmanifest` com `"start_url": "/remote"` e `"scope": "/remote"`.
   - Alternar dinamicamente o link `<link rel="manifest">` e a tag `<meta name="apple-mobile-web-app-title">` em `App.svelte` de acordo com a rota ativa.
   - Adicionar redirecionamento protetivo em `App.svelte`: se aberto em modo standalone em dispositivo móvel na raiz `/`, redirecionar para `/remote`.

---

## 3. Critérios de Aceite

- [x] Ao projetar qualquer música ou versículo bíblico no painel, a rota `/lower-third` renderiza o texto, gradiente/barra e referência corretamente.
- [x] A transparência para OBS Studio permanece nativa e sem bordas indesejadas.
- [x] No navegador comum, o operador tem visibilidade do status do Lower Third e pode alternar a simulação de prévia de câmera.
- [x] Ao adicionar o PWA à tela de início no iOS Safari em `/remote`, o atalho abre diretamente em `/remote`.
- [x] Portões de qualidade (`cargo fmt`, `clippy`, `cargo test`, `npm run check`, `npm run build`) 100% aprovados.

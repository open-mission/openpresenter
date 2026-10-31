# Plano: Correção de Estilização, CORS e Resolução de Mídia no Google Cast (Web Receiver)

Plano de implementação associado à spec [`docs/specs/2026-10-31-google-cast-web-receiver-cors-e-midia.md`](../specs/2026-10-31-google-cast-web-receiver-cors-e-midia.md).

## Etapas de Implementação

### Etapa 1: Backend Rust (`crates/op-admin`) — CORS e Rota de Mídias
1. **Middleware de CORS e Injeção de Cabeçalhos Globais:**
   - Em `crates/op-admin/src/main.rs`, criar middleware que injeta `Access-Control-Allow-Origin: *`, `Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS, HEAD` e `Access-Control-Allow-Headers: *` em todas as respostas HTTP cujo cabeçalho ainda não contenha essas diretivas.
2. **Cabeçalhos Explícitos em Arquivos Estáticos (`crates/op-admin/src/static_files.rs`):**
   - Em `serve_local_file`, `serve_embedded_file` e `fallback_html_response`, adicionar os cabeçalhos CORS explícitos para garantir que consultas via `curl -I` sem o header `Origin` retornem `access-control-allow-origin: *`.
3. **Mapeamento de `/media/{*subpath}` e CORS em Mídias (`crates/op-admin/src/routes.rs`):**
   - Adicionar a rota `/media/{*subpath}` apontando para `serve_media_file`.
   - Em `serve_media_file` e `serve_external_file`, adicionar cabeçalhos CORS irrestritos na resposta e habilitar resolução de arquivos no diretório raiz e em subpastas conhecidas (`wallpapers`, `images`, `church_brand`).
4. **Testes de Integração:**
   - Adicionar testes automatizados validando o retorno de `access-control-allow-origin: *` em rotas estáticas (`/assets/...`, `/public`) e na nova rota `/media/...`.

### Etapa 2: Frontend — Base URL, Query Params e Estilos Críticos
1. **Configuração de `<base href="/" />` e Estilos no `frontend/index.html`:**
   - Adicionar `<base href="/" />` no `<head>` logo após `<meta charset="UTF-8" />`.
   - Adicionar estilos críticos inline em `body` e `#app` (`background-color: #000; color: #fff; width: 100vw; height: 100vh; overflow: hidden; margin: 0;`).
2. **Suporte a Query Params de Visualização no `frontend/src/App.svelte`:**
   - Suportar `?view=public`, `?view=stage`, `?view=remote` e `?view=lower-third` no seletor de rotas para compatibilidade total com o carregamento em webviews/iframes.
3. **Estilos Críticos Inline no `frontend/src/lib/views/PublicView.svelte`:**
   - Incluir estilos críticos no container raiz `<main>` de `PublicView.svelte` (`background-color: #000; color: #fff; width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; overflow: hidden;`).

### Etapa 3: Normalização de URLs de Mídia e Background (`cast.ts` e `PublicView.svelte`)
1. **Funções `getServerOrigin()` e `normalizeMediaUrl()` em `frontend/src/lib/cast.ts`:**
   - Implementar e exportar `getServerOrigin` e `normalizeMediaUrl(url: string | null | undefined): string`.
   - Atualizar `resolveOutputUrl(target)` para utilizar a origem correta do servidor e cache da rede local.
2. **Aplicação de URLs Normalizadas em `PublicView.svelte`:**
   - Importar `normalizeMediaUrl` e aplicar em planos de fundo (`resolvedBg.url` para imagens e vídeos), logotipo da igreja (`activeLogoUrl`), imagens de slide direto (`activeSlide.content`) e ícones de branding no estado de espera.

### Etapa 4: Validação e Portões de Qualidade
1. Executar `cargo fmt --all`.
2. Executar `cargo clippy --workspace --all-targets -- -D warnings`.
3. Executar `cargo test --workspace`.
4. Executar build do frontend com `bun run build` ou `npm run build` na pasta `frontend`.
5. Atualizar documentação (`docs/README.md`, `CHANGELOG.md` e `TODO.md`).

# Especificação: Correção de Estilização, CORS e Resolução de Mídia no Google Cast (Web Receiver)

## Intenção

Garantir que a transmissão sem fio via Google Cast (Chromecast, Android TV e Google TV) exiba a projeção pública e o retorno de palco com fidelidade visual idêntica à visualização direta no navegador, renderizando estilos Tailwind CSS, tipografia, fontes vetoriais, fundos com cores/gradientes e mídias locais (imagens, vídeos e wallpapers) sem bloqueios por CORS, erros de roteamento ou problemas com caminhos relativos no sandbox isolado do Web Receiver.

## Escopo

- **Dentro do escopo:**
  1. **Permissões de CORS Irrestritas para Assets Estáticos e Mídia (`crates/op-admin`):**
     - Configuração de cabeçalhos CORS explícitos (`Access-Control-Allow-Origin: *`, `Access-Control-Allow-Methods: GET, HEAD, OPTIONS`, `Access-Control-Allow-Headers: *`) em todos os manipuladores de arquivos estáticos (`static_files.rs`), arquivos de mídia (`serve_media_file`, `serve_external_file`) e via middleware global de resposta no Axum (`main.rs`).
     - Suporte explícito à rota `/media/{*subpath}` com resolução inteligente em subpastas de mídia (`media/`, `media/wallpapers/`, `media/images/`, `media/church_brand/`).
     - Garantia de que requisições inspecionadas via `curl -I http://<IP_LOCAL>:<PORTA>/assets/...` ou `/media/...` retornem o cabeçalho `access-control-allow-origin: *` mesmo sem envio do cabeçalho `Origin`.
  2. **Caminhos Absolutos de Mídia e Background (`frontend/src/lib/cast.ts`, `frontend/src/lib/backgrounds.ts` e `frontend/src/lib/views/PublicView.svelte`):**
     - Implementação da função canônica `resolveMediaUrl(url: string | null | undefined): string` convertendo caminhos relativos (`/media/...`, `./backgrounds/...`, `media/...`, `/branding/...`) em URLs absolutas utilizando a origem real do servidor local (`${window.location.origin}/...`).
     - Função `resolveCssUrls(style: string)` para converter referências relativas `url(...)` dentro de estilos CSS em URLs absolutas.
     - Integração de `resolveMediaUrl` em `PublicView.svelte` para planos de fundo (`resolvedBg.url`), logotipo institucional (`activeLogoUrl`), imagens de slide direto (`activeSlide.content`) e imagens institucionais em repouso.
     - Resiliência no carregamento assíncrono do Google Cast SDK (`loadCastSdk`), implementando sondagem contínua (polling) em vez de timeout estático rígido e limpeza de cache de falha para permitir novas tentativas sem travar a interface.
  3. **Resolução de Base URL e Suporte a Query Params no Receiver (`frontend/index.html` e `frontend/src/App.svelte`):**
     - Inclusão da tag `<base href="/" />` no `<head>` do `index.html` para direcionamento canônico de recursos estáticos empacotados pelo Vite.
     - Suporte ao parâmetro de consulta `?view=public`, `?view=stage` e `?view=remote` no roteador do `App.svelte`, garantindo carregamento direto mesmo quando receptores web ou iframes sanitizarem a barra de caminho.
  4. **Estilos Críticos Inline de Fallback (`frontend/src/lib/views/PublicView.svelte` e `frontend/index.html`):**
     - Aplicação de estilos inline no container raiz `<main>` de `PublicView.svelte` (`background-color: #000; color: #fff; width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; overflow: hidden;`), bem como no `body` e `#app` de `index.html`, prevenindo telas brancas ou desalinhadas durante a inicialização do WebView no Chromecast.

- **Fora do escopo:**
  - Alteração no protocolo de mensagens Cast (`urn:x-cast:com.url.cast`).
  - Streaming de vídeo HLS ou WebRTC em tempo real no servidor backend.

## Decisões Técnicas

1. **Middleware de CORS no Axum + Headers Manuais em Arquivos Estáticos:**
   - O `tower_http::cors::CorsLayer` opera segundo a especificação W3C, injetando cabeçalhos apenas quando o cabeçalho `Origin` está presente na requisição. Como ferramentas como `curl -I` e alguns componentes de webviews/fontes do Chromecast solicitam recursos sem `Origin`, adiciona-se um middleware global no Axum que assegura `Access-Control-Allow-Origin: *` em todas as respostas HTTP, além de headers explícitos na montagem de `Response<Body>` em `static_files.rs` e nas rotas de mídia de `routes.rs`.
2. **Rota Unificada `/media/{*subpath}` com Busca Multi-diretório:**
   - O backend passa a mapear tanto `/api/media/files/{*subpath}` quanto `/media/{*subpath}`, consultando sequencialmente o diretório base de mídias e suas subpastas conhecidas (`wallpapers`, `images`, `church_brand`), evitando erros 404 caso o cliente solicite com ou sem prefixo de pasta.
3. **Normalização Absoluta de URLs com Origem Resiliente:**
   - Em `frontend/src/lib/cast.ts`, a função `getServerOrigin()` prioriza parâmetros de URL (`?server=`, `?origin=`), a origem de rede obtida da API `/api/system/network-info`, e a origem atual do navegador (`window.location.origin`), garantindo resolução robusta para qualquer dispositivo na rede local.
4. **Proteção Contra FOUC (Flash of Unstyled Content) no Web Receiver:**
   - A adição de estilos críticos inline no `index.html` e no container principal de `PublicView.svelte` garante fundo preto puro (#000000) e centralização antes mesmo do parse completo do CSS do Tailwind pelo Chromium do Chromecast.

## Critérios de Aceite

1. Ao inspecionar o servidor via `curl -I http://<IP_LOCAL>:<PORTA>/assets/...`, os headers de resposta retornam `access-control-allow-origin: *`.
2. Requisições para `/media/{*subpath}` retornam o arquivo com tipo MIME adequado e cabeçalho `access-control-allow-origin: *`.
3. Ao iniciar a sessão Cast no painel do operador, a TV/Chromecast renderiza o fundo preto, imagens/vídeos de wallpaper e tipografia com cores e proporções idênticas à rota `/?view=public` ou `/public`.
4. URLs de mídia em `PublicView.svelte` são normalizadas com a origem HTTP absoluta do servidor.
5. O carregamento de `/?view=public` renderiza a Tela Pública corretamente.
6. A suíte de testes do Rust (`cargo test --workspace`), verificação de linter (`cargo clippy --workspace --all-targets -- -D warnings`) e formatação (`cargo fmt --all`) passam sem erros.

# Changelog

Todas as mudanças relevantes deste projeto são registradas neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e o projeto adere a [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Adicionado
- **Nova Aba "Outros" no Painel de Mídias para Agenda, Textos Livres, Avisos e Seções:**
  - Adição da aba "Outros" (`id="tab-others"`) na barra de navegação do painel inferior (`BottomPanel.svelte`) com badge de quantidade de itens cadastrados.
  - Catálogo persistente via módulo `customItems.ts` integrado ao banco de dados SQLite (`app_settings` com chave `custom_media_items`) e sincronização reativa via WebSocket em tempo real.
  - Seeding inicial com modelos pré-configurados úteis para o culto (Programação Semanal com horários e cultos, Boas-Vindas aos Visitantes e Avisos Gerais).
  - Barra de ferramentas com busca textual instantânea, filtros por categoria (`Todos`, `📅 Agenda`, `📝 Textos`, `📢 Avisos`, `🔖 Seções`), botão recarregar, importação direta de itens da setlist ativa e atalhos rápidos `+ Agenda`, `+ Texto` e `+ Aviso`.
  - Layout dividido em painel resizable:
    - Lista de cards com badges coloridos de tipo, indicador de slides, resumo/detalhes e botões de ação rápida (`+ Setlist`, `Ao Vivo`, `Editar` e `Excluir`).
    - Pré-visualização detalhada com cabeçalho de ações e grade responsiva de miniaturas 16:9 estilizadas para cada tipo de conteúdo (card de agenda com badges de dia/hora ou texto formatado).
  - Ações rápidas: clique duplo no card ou no slide para apresentação imediata ao vivo via `quickPresentMedia`, e adição à setlist ativa com feedback visual via Sonner.
- **Configurações Visuais e Tipográficas da Agenda (Projeção Pública e Monitor de Palco):**
  - Controles visuais dedicados para o tipo de mídia Agenda (`MediaKind::Schedule`) em `SettingsModal.svelte` nas abas "Projeção Pública" e "Monitor de Palco".
  - **Projeção Pública (`/public` e preview do operador):**
    - Opção de exibir ou ocultar o container com fundo translúcido (`schedule_box_background`).
    - Ajuste de opacidade do fundo do card de 10% a 100% (`schedule_bg_opacity`).
    - Largura máxima do container de 50% a 100% (`schedule_max_width_percent`).
    - Espaçamento interno com modos Compacto, Normal e Amplo (`schedule_padding`).
    - Alinhamento horizontal do conteúdo: Esquerda, Centro e Direita (`schedule_align_h`).
    - Configuração fina da tipografia com auto-scale: título mín/máx (18-64px / 48-160px), badges de dia da semana (12-36px), horário (12-32px) e texto de detalhes (12-40px).
    - Opções para exibir ou ocultar os distintivos (badges) de dia/horário e o rodapé com o título geral da agenda.
  - **Monitor de Palco (`/stage` e preview):**
    - Ajuste das fontes do Slide Atual da Agenda: auto-scale do título mín/máx (20-60px / 60-240px), badges de dia da semana (12-36px), horário (12-32px) e texto de detalhes (12-32px).
    - Ajuste do bloco "A Seguir" para o próximo evento da agenda com auto-scale do título mín/máx (12-48px / 40-160px).
  - **Simulação em Tempo Real:**
    - Alternador de prévia `[📅 Agenda]` adicionado à visualização 16:9 na aba "Fundos de Letras & CSS" para teste de contraste em tempo real com os presets de plano de fundo.
    - Prévia interativa em tempo real com `StagePreviewBox` na aba do Monitor de Palco refletindo instantaneamente as alterações de fontes para o evento atual e o próximo evento.

## [0.4.0] - 2026-10-31

### Adicionado
- **Exibição e Seleção de Imagens no Lower Third para Transmissão OBS (`/lower-third`):**
  - Renderização gráfica de imagens no terço inferior da transmissão (`LowerThirdView.svelte`), substituindo a exibição de texto bruto do caminho do arquivo.
  - Opções configuráveis em `LowerThirdSettings` e na aba "Lower Third" do `SettingsModal.svelte`:
    - `image_display_mode`: `'fullscreen'` (imagem em tela cheia cobrindo a transmissão), `'lower_third'` (banner ajustado dentro da tarja do terço inferior) ou `'hide'` (ocultar imagem na saída do OBS mantendo o vídeo limpo).
    - `image_fit`: ajuste de preenchimento `'contain'` (preservar proporções sem corte) ou `'cover'` (preenchimento completo da área).
    - Simulação visual interativa de imagem na prévia 16:9 das configurações.
  - Suporte à imagem alternativa dedicada para o Lower Third via campo `lower_image_url` em `MediaItem` e `lower_content` em `Slide` no backend Rust (`op-core`, `op-db`, `op-admin`) e frontend Svelte.
  - Ações no menu de contexto da setlist no painel principal (`AdminView.svelte`) para vincular/alterar e desvincular uma imagem específica para o Lower Third via seletor modal com prévia, e badges indicadores `Lower 🖼️` em `MediaItemRow.svelte` e no cabeçalho do slide.
- **Tipo de Mídia Agenda (Schedule) para Programação de Cultos e Festividades:**
  - Novo tipo de mídia no backend (`MediaKind::Schedule`, `"schedule"`, rótulo `"Agenda"`) e frontend (`ScheduleEvent`), persistido nas setlists em memória e no banco SQLite.
  - Modal dedicado de criação e edição (`ScheduleEditorModal.svelte`) com atalhos de um clique para dias da semana (`+ Domingo`, `+ Quarta`, `+ Sábado`, etc.) e eventos festivos futuros com data livre, horários e locais/detalhes.
  - Opção "Agenda / Programação..." adicionada ao menu `+` da setlist no painel principal e ao menu de contexto de área vazia e reedição de itens.
  - Renderização imersiva na Tela Pública (`PublicView.svelte`) e no preview do operador (`PublicPreviewBox.svelte`) com card translúcido sobre o papel de parede selecionado, badges destacados para dia e horário com ícone e auto-scaling inteligente no título do evento.
  - Sincronização sequencial com o Monitor de Palco (`StageView.svelte`) e preview (`StagePreviewBox.svelte`): cada evento compõe um slide da agenda, exibindo o dia e culto atual no bloco Slide Atual e antecipando o dia e horário seguintes no bloco A Seguir.
- **Aviso para o Palco em Tela Cheia Temporária (Flash Pop-in):**
  - Ao receber uma nova mensagem do operador, o Monitor de Palco (`/stage`) exibe o aviso em tela cheia com fundo translúcido escuro (`bg-black/95 backdrop-blur-xl`), tipografia ampliada de alto impacto e badge de alerta pulsante.
  - Temporizador automático configurável (`alert_flash_duration_seconds`, padrão: 5 segundos) com transição suave e recolhimento para a miniatura/caixa de aviso do rodapé ou coluna lateral.
  - Opção nas configurações para ativar/desativar o pop-in em tela cheia (`alert_fullscreen_flash`) e fechar imediatamente por clique ou tecla `Escape`.
- **Seletor de Quantidade de Músicas no Painel de Catálogo (50, 100, 150, 200, Todas):**
  - Controle integrado na barra de ações da aba de Músicas (`BottomPanel.svelte`) permitindo filtrar e paginar dinamicamente a quantidade de músicas exibidas com opções de `50`, `100`, `150`, `200` ou `Todas`.
  - Parâmetro `limit` suportado na API backend `GET /api/songs?limit=...`, truncando resultados com segurança após aplicação de filtros de busca e tags.
  - Persistência automática da preferência do operador no `localStorage` (`open_presenter_song_limit`).
- **Importação de Músicas (JSON, TXT, MD), Instalação da Harpa Cristã, Tags e Padrão de 2 Linhas por Slide:**
  - Importação flexível de arquivos de músicas nos formatos JSON (estruturado ou Harpa Cristã), texto puro (.txt) e Markdown (.md), preservando seções ou dividindo estrofes no padrão configurado.
  - Instalação oficial com 1 clique da Harpa Cristã completa (540 hinos tradicionais) a partir do arquivo embutido `assets/songs/harpa-crista.json` com fallback para download resiliente via GitHub raw, formatada no padrão configurado e categorizada automaticamente com tags `#harpa`, `#hinos`, `#tradicional`.
  - Sistema de tags e grupos para músicas com persistência relacional SQLite (migração `0009_song_tags.sql`), agregação de tags, listagem e barra de filtros interativa no catálogo (`Todas`, `#tag`).
  - Configuração de padrão de linhas por slide (padrão: 2 linhas por slide) customizável nas configurações (`song_settings`), aplicado automaticamente a músicas importadas e capturadas online na web (Holyrics/Vagalume).
  - Botão dedicado "2 Linhas/Slide" no editor de músicas (`SongEditorModal.svelte`) para reformatar letras instantaneamente mantendo partes estruturais (Verso, Refrão, etc.).
  - Nova aba "Músicas & Harpa" no modal de configurações com cartões informativos para instalação da Harpa Cristã, seletor de linhas por slide e importador de arquivos com atribuição de tags.
- **Sistema de Notificações Toast com Sonner para Ações e Eventos em Tempo Real:**
  - Integração do componente Sonner via `svelte-sonner` adaptado com estilização Shadcn/Tailwind UI, posicionado no canto superior direito no painel administrativo e topo central no controle remoto mobile.
  - Notificações instantâneas e não invasivas para início e parada de apresentações, salvamento e atualização de músicas no catálogo, alteração de modos de exibição (blackout/ocultar texto) e conexões de novos aparelhos na rede local.
  - Sincronização automática com eventos WebSocket (`WsMessage::ActionLogged`), informando operadores de desktop quando uma ação foi realizada remotamente via celular/tablet.
  - Exclusão estrita de toasts nas telas públicas e de transmissão (`/public`, `/stage` e `/lower-third`), preservando a projeção e a saída de vídeo 100% limpas.
- **Opção de Reiniciar a Aplicação e o Servidor (Interface Web e System Tray):**
  - Implementação de função multiplataforma segura para spawn desanexado do novo executável mantendo argumentos da linha de comando original (`restart_application`).
  - Adição da opção "Reiniciar OpenPresenter" no menu nativo da bandeja do sistema operacional (System Tray), permitindo reinicialização ágil pelo operador na barra de tarefas / menu bar.
  - Endpoints HTTP `POST /api/system/restart` e `POST /api/updater/restart` com agendamento desacoplado, rastreabilidade nos logs auditáveis e entrega limpa da resposta HTTP antes da finalização graciosa do processo anterior.
  - Botão de ação "Reiniciar Aplicativo Agora" no modal de configurações ao concluir a instalação de uma nova versão via Auto-Updater (`ReadyToRestart`) e cartão "Reiniciar Servidor e Aplicação" na aba Sobre com overlay de transição e polling automático para recarregar a página assim que o servidor voltar a responder.
- **Sistema de Logs de Auditoria e Identificação de Dispositivos Conectados:**
  - Criação da tabela persistente `action_logs` via migração SQLite `0008_action_logs.sql` registrando ações de apresentação, setlists, catálogo de músicas, cronômetros, configurações e auditoria de conexões.
  - Identificador único persistente de instância (`instance_id`) e extração automática de metadados do navegador/dispositivo (`device_info`), permitindo rastreabilidade precisa de qual operador realizou cada ação em rede local.
  - Nova aba "Histórico e Logs de Auditoria" na modal de configurações com filtragem dinâmica por categoria, paginação, limpeza e sincronização em tempo real via evento WebSocket `action_logged`.
  - Exibição refinada no dropdown de status de rede do cabeçalho, mostrando tipo de aparelho com ícones dedicados (computador, smartphone, tablet), sistema operacional/navegador, IP, rota e atalho direto para os logs.

### Melhorado
- **Resiliência e Diagnóstico do Google Cast Sender SDK (`index.html`, `cast.ts`, `CastControl.svelte`):**
  - Registro precoce de `window.__onGCastApiAvailable` e injeção assíncrona do SDK no `<head>` de `index.html`, eliminando a race condition de injeção dinâmica sob demanda e permitindo inicialização imediata do Cast Framework.
  - Implementação do diagnóstico de ambiente (`getCastDiagnostic`) para detectar acessos via IP de rede local (`http://192.168.x.x`), em que o Google Chrome desativa o Cast Sender por política de Contexto Seguro (que exige `localhost` ou HTTPS), além de orientar usuários em navegadores incompatíveis (Safari/Firefox).
  - Preservação de mensagens de erro detalhadas em `startCasting`, evitando sobrescrever causas específicas com a mensagem genérica de indisponibilidade.
  - Aprimoramento do banner de aviso do Cast com atalhos rápidos de 1 clique para abrir `/public` e `/stage` em nova aba e instruções claras para transmissão nativa de aba via menu do Google Chrome.
- **Rótulos Flutuantes em Posicionamento Absoluto no Monitor de Palco (`StageView.svelte`):**
  - Reposicionamento dos cabeçalhos dos blocos "Slide Atual" e "A Seguir (Próximo)" com estilo flutuante em posicionamento absoluto (`absolute top-2 left-3 right-3`) e fundo translúcido sutil.
  - Eliminação do consumo vertical rígido de 30px–50px causado por barras estáticas de cabeçalho, liberando 100% da altura e largura útil dos blocos para o algoritmo de `autoScale`, permitindo que as letras de louvor e versículos bíblicos alcancem tamanhos muito maiores e mais legíveis.
  - Atualização correspondente no preview de palco (`StagePreviewBox.svelte`).
- **Fontes Maiores e Controles de Tamanho no Monitor de Palco (`/stage` e `SettingsModal.svelte`):**
  - Aumento significativo dos tamanhos de fonte do Relógio, Timer/Cronômetro e Aviso para o Palco, garantindo que preencham melhor o espaço útil da tela e sejam perfeitamente legíveis a 10–30 metros de distância.
  - Novos controles deslizantes na aba "Palco" das configurações com ajustes numéricos em tempo real para Relógio (`clock_font_size`: 24px a 96px, padrão 48px), Timer (`timer_font_size`: 28px a 120px, padrão 56px) e Aviso (`alert_font_size`: 16px a 64px, padrão 28px).
  - Paridade visual proporcional no preview do operador (`StagePreviewBox.svelte`).
- **Simplificação e Agrupamento das Saídas no Cabeçalho do Operador (`AdminView.svelte`):**
  - Substituição da fileira de 5 botões individuais (`Iniciar Public`, `Iniciar Stage`, `Cast`, `QR Code`, `Remoto`) por uma Central de Saídas Única com o menu dropdown `Saídas`.
  - Agrupamento semântico claro no menu suspenso em duas seções: "Telas do Computador" (Projeção Pública `/public`, Retorno de Palco `/stage` e Lower Third `/lower-third` para OBS) e "Sem Fio & Dispositivos" (Google Cast, Controle Remoto Mobile `/remote` e QR Codes de rede).
  - Indicador visual animado de conexão ativa no botão `Saídas` quando houver transmissão Google Cast em andamento (`castStore.isConnected`).
  - Redução drástica da densidade visual do cabeçalho, deixando apenas os controles essenciais no canto direito (`Online`, `Saídas`, `Configurações` e badge de atualização).
- **Exibição e Descoberta de IP na Rede Local (LAN) para Servidor e Vite:**
  - Atualização do banner de inicialização no terminal (`main.rs`) exibindo tanto o acesso local (`http://localhost:<PORTA>`) quanto o endereço na rede local (`http://<IP_LOCAL>:<PORTA>`), além das URLs de todas as telas (Admin, Pública, Palco, Controle Remoto, Lower Third e WebSocket).
  - Suporte à variável de ambiente `HOST` no servidor Axum (padrão: `0.0.0.0`), permitindo especificar a interface de escuta.
  - Sonda multi-alvo em `get_local_ip()` (`routes.rs`) cobrindo conexões sem internet externa (roteadores offline com sub-redes `192.168.x`, `10.x`, `172.x` e multicast `224.0.0.1`).
  - Configuração `server.host: true` e proxy para `/media` no `vite.config.ts`, permitindo expor o frontend em desenvolvimento na rede local para testes em smartphones, tablets e Chromecasts.
- **Remoção do Limite Fixo de 50 Músicas e Sincronização Dinâmica do Catálogo:**
  - Remoção do teto hardcoded de `LIMIT 50` em `search_songs` (`op-db`), permitindo exibir o catálogo integral com todos os louvores e os 540 hinos da Harpa Cristã tanto na listagem geral quanto em filtros por tag (`#harpa`).
  - Elevação do limite de busca FTS5 e LIKE para até 1.000 resultados, viabilizando pesquisas abrangentes em grandes acervos musicais.
  - Sincronização reativa instantânea através do evento `openpresenter:songs-changed` e no retorno à aba de músicas (`activeTab === 'songs'`), recarregando o catálogo automaticamente após salvar, editar ou instalar hinos nas configurações.
- **Resiliência e Agilidade na Busca de Músicas Online no Command Palette:**
  - Substituição do botão rotativo cíclico no cabeçalho do Command Palette por um controle segmentado direto com 3 modos dedicados (`[🎵 Músicas]`, `[📖 Bíblia]`, `[🌐 Web]`), permitindo alternar e pesquisar online com um único clique sem requerer cliques sucessivos.
  - Implementação de cancelamento atômico de buscas assíncronas com `AbortController` e identificador sequencial `searchId`, eliminando conflitos de concorrência e requisições duplicadas entre cliques diretos e o temporizador de debounce reativo.
  - Pré-seleção inteligente da Ação 2 ("Buscar na internet") ao digitar um título de louvor inexistente no catálogo local (`localResults.length === 0`), permitindo disparar a busca web diretamente com a tecla Enter sem desviar para a busca bíblica.
  - Preservação contínua de foco no campo de texto (`inputElement.focus()`) ao trocar de abas, contagem de resultados na aba Online e re-execução imediata ao pressionar Enter na aba Web.
- **Separação Correta de Slides e Redimensionamento com Resizable Shadcn no Catálogo de Músicas:**
  - Decomposição fidedigna das letras de músicas usando `parseRawLyrics(song.lyrics_raw)`, gerando slides 16:9 individuais para cada estrofe delimitada por linhas em branco (`\n\n`), evitando o problema onde dezenas de estrofes eram condensadas em um único card gigante.
  - Grade responsiva com quebra automática (*wrap*) em 1, 2, 3 ou 4 colunas (`grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-3`), adaptando o layout dos slides proporcionalmente à largura útil disponível.
  - Integração da suíte Shadcn `ResizablePaneGroup`, `ResizablePane` e `ResizableHandle` no painel de músicas (`BottomPanel.svelte`), permitindo ao operador redimensionar horizontalmente a proporção entre a lista do catálogo e o painel de pré-visualização de slides com alça de arraste visual (*drag handle*).
- **Ajuste Visual e Espaçamento do Resizer do Painel de Mídias:**
  - Desacoplamento do separador de redimensionamento do painel inferior de mídias (`BottomPanel.svelte`), posicionando-o no fluxo superior com margem dedicada para não sobrepor os botões de seleção de abas ("Músicas", "Imagens", etc.).
  - Unificação da cor de fundo do separador com o fundo do painel (`bg-card`), eliminando contraste indevido de faixa cinza.
  - Atualização da cor do indicador de arraste (*drag indicator*) para tom neutro com maior visibilidade (`bg-neutral-400 dark:bg-neutral-500`) e realce na cor primária durante interações e arraste ativo.
- **Importação de Múltiplas Versões da Bíblia Simultaneamente ("Um a Um"):**
  - Habilitação de seleção múltipla de arquivos (`multiple`) no diálogo nativo do sistema operacional com suporte às extensões `.sqlite`, `.db` e `.sqlite3`.
  - Processamento em fila sequencial ("um a um") com leitura assíncrona em Base64, requisição individual à API e notificação toast Sonner para cada versão instalada.
  - Card de progresso visual com indicador percentual, contagem de versões concluídas e nome do arquivo em processamento.
  - Suporte a múltiplos caminhos de arquivo separados por vírgula ou nova linha na importação manual por caminho ("Por Caminho...").
  - Adição de contador atômico sequencial no gerador de ID da versão (`BIBLE_IMPORT_COUNTER`) no backend `op-admin`, prevenindo colisões em uploads quase instantâneos.
- **Destaque Visual com Fundo Vermelho no Botão de Parar Apresentação no Header do Remote:**
  - Aplicação de fundo vermelho de alta visibilidade (`bg-red-600 hover:bg-red-500 text-white`) no botão de parar apresentação no cabeçalho do Controle Remoto Mobile (`/remote`), com ícone de quadrado preenchido e rótulo adaptativo ("Parar"), tornando a interrupção da projeção imediata e intuitiva em telas touch.
- **Layout de 3 Linhas no Monitor de Palco e Recálculo Automático de Proporções (Soma 100%):**
  - Implementação do layout de 3 linhas horizontais empilhadas (`stacked-rows`) no Monitor de Palco (`/stage`), posicionando Aviso para o Palco, Relógio e Cronômetro em uma barra horizontal inferior abaixo do Próximo Slide, garantindo 100% da largura útil da tela para os textos do slide atual e do próximo slide.
  - Seletor de layout ("3 Linhas Horizontais" vs "Coluna Lateral Direita") nas configurações com persistência local e no backend.
  - Algoritmo de rebalanceamento proporcional automático nos controles deslizantes de altura (`current_slide_percent`, `next_slide_percent` e `footer_time_percent`), mantendo a soma das três áreas invariavelmente em 100% com indicador visual em tempo real.
  - Atualização completa do componente de preview (`StagePreviewBox.svelte`) para refletir com paridade visual o modo de layout selecionado e suas respectivas proporções.
- **Redimensionamento Fluido do Painel de Mídias e Suíte Resizable Shadcn:**
  - Eliminação da latência e atraso perceptível no redimensionamento manual do painel inferior de mídias (`BottomPanel.svelte`) através da desativação condicional da transição CSS (`transition-[height]`) durante o arrasto, captura global do cursor `row-resize` e bloqueio de seleção acidental de texto.
  - Adição de indicador visual centralizado de grip bar estilizado no separador superior do painel inferior.
  - Introdução da biblioteca de componentes `ui/resizable` no padrão shadcn-svelte (`ResizablePaneGroup`, `ResizablePane`, `ResizableHandle`) baseada em `paneforge`.
- **Altura Otimizada ao Abrir a Aba de Bíblia:**
  - Expansão automática do painel inferior para 75% da altura útil da janela (`Math.round(window.innerHeight * 0.75)`) ao selecionar a aba "Bíblia" com o painel fechado ou com altura padrão reduzida.
  - Proporciona espaço ergonômico e amplo para consulta de livros, capítulos, versículos e buscas textuais, preservando a barra superior e o preview do slide ativo.

### Corrigido
- **Resolução Absoluta de URLs de Mídia e Polling Resiliente do SDK no Google Cast:**
  - Implementação da função canônica `resolveMediaUrl(url: string | null | undefined): string` no frontend (`cast.ts` e `backgrounds.ts`), garantindo que planos de fundo (imagens e vídeos), logotipos da igreja e slides de imagem direta recebam URLs absolutas apontando para o servidor local (`${window.location.origin}/...` ou parâmetro `?server=...`).
  - Adição da função `resolveCssUrls(style: string)` em `backgrounds.ts`, normalizando referências `url(...)` relativas embutidas em gradientes e propriedades CSS inline para URLs absolutas antes da aplicação no DOM.
  - Atualização dos componentes `PublicView.svelte` e `PublicPreviewBox.svelte` para utilizar consistentemente `resolveMediaUrl` em elementos `<video>`, `<img>` e em estilos `background-image`.
  - Reformulação do carregador do SDK do Google Cast (`loadCastSdk` em `cast.ts`) com sondagem assíncrona periódica a cada 250ms (com limite de 7s) e limpeza de cache de promessa em caso de falha, eliminando o bloqueio permanente que gerava a mensagem de indisponibilidade indevida ("Google Cast SDK indisponível no momento").
- **Estilização, CORS Irrestrito e Resolução de Mídia no Google Cast (Web Receiver):**
  - Configuração de cabeçalhos CORS explícitos (`Access-Control-Allow-Origin: *`, `Access-Control-Allow-Methods: GET, HEAD, OPTIONS`, `Access-Control-Allow-Headers: *`) em todos os manipuladores de arquivos estáticos (`static_files.rs`), arquivos de mídia (`routes.rs`) e middleware global no Axum (`main.rs`), garantindo que requisições de CSS, fontes e mídias sem cabeçalho `Origin` (como em inspeções com `curl -I`) recebam liberação irrestrita de CORS.
  - Adição da rota unificada `/media/{*subpath}` com resolução sequencial resiliente em subpastas conhecidas (`media/`, `media/wallpapers/`, `media/images/`, `media/church_brand/`).
  - Tag canônica `<base href="/" />` no `<head>` de `index.html` e suporte direto a query params (`?view=public`, `?view=stage`, `?view=remote`) no roteador do `App.svelte` para compatibilidade total com webviews e iframes do Chromecast.
  - Estilos críticos inline de fallback no container raiz `<main>` de `PublicView.svelte` e no `body`/`#app` de `index.html`, prevenindo flashes de tela branca e garantindo fundo preto e centralização imediata durante o carregamento inicial no receptor.
- **Sincronização de versículos bíblicos ao vivo no painel admin em múltiplas instâncias:**
  - Correção na validação de passagem ativa em `BiblePanel.svelte`, assegurando que a marcação em verde e o selo "Ao Vivo" sejam exibidos estritamente quando o livro e capítulo atualmente abertos corresponderem à passagem em projeção.
  - Eliminação de falso-positivo onde instâncias secundárias exibiam o mesmo número de versículo em verde em capítulos não relacionados (ex: 1Ts 1:7 aparecendo verde quando 1Ts 5:7 estava sendo apresentado).
- **Sentido na navegação de slides via teclado e sincronização em tempo real:**
  - Correção nas funções `nextSlide()` e `prevSlide()` em `AdminView.svelte`, assegurando que o cálculo do próximo slide utilize o índice ativo em tempo real (`active_slide_index`) quando a mídia estiver ao vivo, evitando retrocessos involuntários ao pressionar `ArrowRight`, `PageDown` ou `Espaço`.
  - Inicialização explícita de `focusedSlideIndex` ao disparar versículos bíblicos (`handlePresentScripture`), músicas e imagens.
  - Sincronização contínua do foco do operador quando outro aparelho conectado (ou controle remoto mobile) alterar o slide ativo.

## [0.3.1] - 2026-10-30

### Corrigido
- **Resolução de Mídia e Renderização na Saída Lower Third (`/lower-third`):**
  - Correção na resolução de slide ativo em `LowerThirdView`, expandindo a busca para `$mediaItemsList`, `$activeSetlist` e fallback assíncrono sob demanda `/api/songs/:id`, garantindo que músicas da biblioteca/setlist e textos bíblicos sejam projetados normalmente.
  - Correção da exibição no navegador convencional: inclusão de classes explícitas `is-lower-third` no `<html>` e `<body>`, eliminação da tela branca vazia com introdução de tela de status elegante quando não há slides em exibição e botão de alternar simulação de vídeo/transparência alfa para testes no desktop.
- **PWA no iOS apontando para `/remote`:**
  - Criação de manifesto dedicado `manifest-remote.webmanifest` com `start_url: "/remote"` e `scope: "/remote"`.
  - Alternância dinâmica do manifesto e tags de título do iOS (`apple-mobile-web-app-title`) em `App.svelte` de acordo com a rota ativa.
  - Redirecionamento automático de dispositivos móveis em modo standalone (PWA instalado) que abrirem na raiz `/` diretamente para o controle remoto `/remote`.

## [0.3.0] - 2026-10-30

### Adicionado
- **Ícone Nativo do Executável do OpenPresenter (Multiplataforma):**
  - No macOS, geração do arquivo de ícones oficial `.icns` com resolução Retina de 16x16 até 1024x1024 (`assets/branding/open-presenter-app-icon.icns`).
  - Criação de script de empacotamento (`scripts/package-macos.sh`) gerando o bundle nativo `OpenPresenter.app` com `Info.plist` e recursos de ícone completos.
  - Atribuição do ícone oficial da marca diretamente no arquivo executável Mach-O (`open-presenter`) via atributos estendidos do Finder (`com.apple.FinderInfo`), garantindo que o arquivo avulso exiba o ícone no Finder e desktop.
  - Configuração em tempo de execução no `main.rs` para registrar o ícone oficial no Cocoa `NSApplication`, assegurando exibição na Dock do macOS e no alternador de tarefas (Cmd+Tab).
  - No Windows, incorporação do ícone nos recursos PE do executável `.exe` via `winresource` apontando para `open-presenter-app-icon.ico`.
- **Suporte a Progressive Web App (PWA) e Instalação Standalone:**
  - Criação do manifesto web (`manifest.webmanifest`) com ícones oficiais de alta resolução (192x192 e 512x512), tema escuro e atalhos rápidos para o Painel, Remoto, Telão e Palco.
  - Implementação de Service Worker (`sw.js`) permitindo a instalação do OpenPresenter como aplicativo autônomo (standalone) no desktop e em smartphones (Android e iOS).
  - Suporte completo à área segura de dispositivos Apple (`viewport-fit=cover`, `safe-area-inset-top`, `safe-area-inset-bottom`) e meta tags de app em tela cheia.
- **Refinamento Ergonômico do Controle Remoto Mobile (`/remote`):**
  - **Navegação Bíblica com Sub-Abas em Grade:** Divisão em **Livros** (grade com 3 colunas e filtro AT/NT), **Capítulos** (grade touch com botões de 44px) e **Versículos** (leitura integral, navegação rápida e projeção direta com um toque).
  - **Input Group e Drawer Shadcn:** Seletor de versão bíblica elegante deslizando de baixo para cima (Drawer) integrado em grupo com o campo de busca.
  - **Card de Controle ao Vivo Retrátil (Collapsible):** Opção de recolher/expandir o card do slide atual, liberando 80% do espaço vertical da tela para leitura da Bíblia e seleção de músicas.
  - **Estados Ativos (Active States):** Indicação visual de "Ao Vivo" nas estrofes das músicas e nos versículos que estão projetados no telão público em tempo real.
  - **Header Otimizado para Touch com Branding Oficial:** Logotipo oficial SVG do OpenPresenter Minimal e botões de emergência com áreas de toque ampliadas.
  - **Correção da Setlist:** Carregamento detalhado via `/api/setlists/:id` com suporte a alternar entre múltiplos cultos cadastrados.
- **Saída Lower Third Transparente para Transmissão / OBS Studio (`/lower-third`):**
  - Nova tela dedicada e otimizada para software de transmissão ao vivo (OBS Studio, vMix, Wirecast) com canal alfa nativo e transparência total de fundo.
  - Projeção de letras de louvor e passagens bíblicas ancoradas no terço inferior da tela, sem obstruir o pastor, pregador ou ministro no enquadramento da câmera.
  - 4 temas visuais selecionáveis:
    - **Gradiente Central:** Máscara radial elíptica que escurece suavemente sob o texto e se dissipa nas extremidades para uma estética refinada de transmissão broadcast.
    - **Barra Inferior:** Faixa horizontal contínua de ponta a ponta com borda acentuada no estilo jornalístico/TV.
    - **Caixa Flutuante:** Pílula arredondada com efeito *glassmorphism* e desfoque de fundo (*backdrop blur*).
    - **Minimalista:** Apenas tipografia com contorno e sombras fortes para contraste máximo sem caixas visíveis.
  - Opções completas de personalização na nova aba **Transmissão (OBS / vMix)** nas Configurações:
    - Ativação/desativação independente da transmissão com um clique.
    - Personalização da cor de fundo (RGBA), opacidade (10% a 100%), distância da margem inferior e largura máxima.
    - Tipografia flexível: tamanho da fonte (20px a 56px), peso (Normal a Extra Negrito) e alinhamento (Esquerda, Centro, Direita).
    - Níveis de sombra de texto para contraste sobre cenas claras ou escuras (Nenhuma, Suave, Média, Forte).
    - Animações de transição configuráveis: Fade suave, Deslize vertical (Slide), Zoom leve ou Instantânea.
    - Badge opcional para identificação de referência bíblica ou título/autor da música.
    - Prévia interativa em proporção 16:9 com simulação de imagem de câmera de palco e alternador entre louvor e bíblia.
  - Integração no modal de compartilhamento (`ShareOutputsModal.svelte`) com instruções detalhadas para Browser Source do OBS Studio.
  - Atalho no menu do System Tray ("Abrir Lower Third (OBS / Live)") e exibição da rota no banner do console.
- **Controle Remoto Mobile para Smartphones (`/remote`):**
  - Nova interface web dedicada e otimizada para smartphones (mobile-first e touch ergonomics), permitindo aos operadores, pregadores e líderes de louvor controlarem o culto diretamente do celular via rede local Wi-Fi.
  - Navegação inferior fixa com 3 abas principais:
    - **Setlist:** Visualização da ordem de culto em tempo real com indicador visual do item ativo, expansão de slides e avanço imediato.
    - **Músicas:** Catálogo e busca instantânea de letras, prévia de estrofes e projeção rápida com um toque.
    - **Bíblia:** Seletor de traduções bíblicas ativas, livros e capítulos, busca rápida por referência ou palavras e projeção ao vivo de versículos.
  - Card de controle persistente no topo com visualização do slide ativo e botões grandes de Anterior e Próximo para o polegar.
  - Botões de emergência rápida: Blackout total, Limpar Texto e Parar Apresentação.
  - Aba dedicada no modal de compartilhamento (`ShareOutputsModal.svelte`) com QR Code para leitura rápida com a câmera de qualquer smartphone conectado ao Wi-Fi da igreja.
  - Atalho no menu do System Tray ("Abrir Controle Remoto (Mobile)") e exibição da rota no banner de inicialização.
- **Paridade Visual 100% no Preview da Projeção Pública do Painel Admin (`PublicPreviewBox.svelte`):**
  - Implementação de renderização virtual com resolução base HD 1080p (1920x1080) escalada dinamicamente via `transform: scale()` para o tamanho exato da coluna de preview.
  - O auto-scale (`autoScale.ts`), as quebras de linha de texto, proporções de margens, tipografia, logotipo da igreja e posicionamento de referência bíblica agora são 100% idênticos à tela pública real do projetor (`/public`).
- **Opções Avançadas de Referência Bíblica e Encaixe de Fundo na Projeção Pública:**
  - Adicionada opção de posicionamento da referência bíblica: **No Rodapé** (barra inferior com divisor elegante) ou **Abaixo do Texto** (acompanhando o versículo centralizado).
  - Controle deslizante para personalizar o tamanho da fonte da referência bíblica no rodapé (16px a 64px, padrão 28px).
  - Opção de Encaixe do Papel de Parede na Projeção (`wallpaper_fit`): `contain` (sem cortes em TV/Chromecast), `cover` (preenchimento completo) e `fill` (esticar sem bordas).
  - Margens seguras aplicáveis ao papel de parede (`wallpaper_safe_margins`) para proteger contra overscan de televisores HDMI e Chromecast.
- **Privacidade e Filtro de Imagens Importadas vs. Sistema na Galeria (`BottomPanel.svelte`):**
  - Por padrão, a galeria de imagens exibe apenas arquivos de pastas importadas/mapeadas (`is_external: true`), impedindo que fotos pessoais ou arquivos residuais do sistema apareçam acidentalmente em cultos.
  - Adicionado botão de alternância de privacidade na barra de ferramentas ("Pastas Importadas" vs. "Todas + Sistema"), persistido no armazenamento local do navegador.
- **Separação de Ajustes para Louvores e Textos Bíblicos na Projeção Pública e Retorno de Palco:**
  - Na aba **Projeção Pública**, inclusão de navegação por sub-seções independentes: `Letras de Louvor`, `Textos Bíblicos` e `Margens & Geral`. Permite configurar alinhamentos, limites de tamanho de fonte e aproveitamento de largura de tela de forma desacoplada para músicas e versículos.
  - Na aba **Monitor de Palco**, divisão em 3 seções: `Letras de Louvor no Palco`, `Textos Bíblicos no Palco` e `Layout & Tempo`, com personalização dedicada de cores (texto e moldura do versículo) e tamanho mínimo de fonte para garantir legibilidade no púlpito.
  - Na aba **Fundos de Letras & CSS**, inclusão de alternador na prévia 16:9 em tempo real entre `[🎵 Letra de Louvor]` e `[📖 Texto Bíblico]`, permitindo validar o contraste e o estilo em ambos os tipos de conteúdo.

### Corrigido
- **Busca Bíblica no Controle Remoto Mobile:**
  - Correção da rota de busca textual FTS para `/api/bibles/:id/search` e inclusão de parser de referências bíblicas instantâneas (ex: "Jo 3:16", "Sl 23") via `parseBibleReference`, permitindo navegar e projetar passagens digitadas diretamente na barra de busca.
- **Divisão de Estrofes e Estrutura de Músicas no Remoto:**
  - Substituição da lógica anterior pelo parser canônico `parseRawLyrics`, respeitando blocos separados por quebras de linha duplas (`\n\n`) e cabeçalhos de seção (`##`), dividindo músicas longas em estrofes adequadas para projeção e toque touch.
- **Projeção Contínua e Indicador de Slides na Bíblia:**
  - Correção do indicador de slides estático (`1/1`) na Bíblia: ao apresentar versículos pelo celular, todos os versículos do capítulo são carregados como slides do `MediaItem`, permitindo ao operador ou pregador avançar e recuar versículos com os botões Anterior/Próximo e acompanhar a contagem real (`Slide X/Total`).
- **Atualização do Catálogo de Músicas no Remoto:**
  - Adicionado botão de recarga instantânea (`RefreshCw`) ao lado do campo de busca de músicas.
- **Alinhamento e Geometria dos Toggles e Switches de Configuração:**
  - Substituição da geometria anterior com posicionamentos absolutos manuais por modelo robusto baseado no padrão Tailwind UI (`border-2 border-transparent` e translação `translate-x-5`), corrigindo o botão circular que vazava para fora da pílula verde.
  - Correção do contraste nas chaves de alternância que ficavam totalmente brancas quando ativas (`peer-checked:bg-primary`).
- **Priorização de Arquivos Estáticos Compilados do Frontend em Disco:**
  - Em `op-admin`, priorização dos arquivos estáticos locais em `frontend/dist` antes dos assets embutidos em tempo de compilação, permitindo que alterações recentes de interface, CSS e Chromecast sejam servidas imediatamente aos clientes sem necessidade de recompilar o binário Rust.
- **Centralização Vertical e Auto-scale de Versículos Bíblicos na Projeção Pública (`/public`):**
  - Correção da distribuição vertical de versículos bíblicos na tela pública, centralizando o texto harmoniosamente no viewport e garantindo o aproveitamento total de largura (`w-full`) para que a fonte expanda de maneira mais legível.
- **Contraste e Visibilidade em Toggles e Switches de Configuração:**
  - Substituição da cor primária monocromática nas chaves de alternância ativas (`role="switch"`) por fundo esmeralda de alto contraste (`bg-emerald-600`) e borda nítida quando inativo, eliminando a renderização totalmente branca que impedia identificar se as opções estavam ligadas ou desligadas.
- **Integração com Google Cast (Chromecast) para Telas Pública e Retorno de Palco:**
  - Carregamento sob demanda do Google Cast Web Sender SDK (`cast_sender.js`) com gerenciador centralizado em `frontend/src/lib/cast.ts`.
  - Transmissão sem fio com latência quase zero (< 30ms) utilizando Web Receiver Chromium e sincronização via WebSocket direto na rede local.
  - Suporte completo à transmissão tanto da Tela Pública (`/public`) quanto do Monitor de Retorno de Palco (`/stage`).
  - Nova aba "Chromecast" dentro do modal de compartilhamento de saídas na rede local (`ShareOutputsModal.svelte`) com componente dedicado `CastControl.svelte`.
  - Atalho de transmissão Cast com indicador de status no cabeçalho do painel do operador (`AdminView.svelte`).
  - Guia de contingência e orientações passo a passo para espelhamento nativo de aba via Chrome / Edge em ambientes 100% offline.
  - Registro de decisão arquitetural (ADR 0013) documentando a superioridade do Web Receiver com WebSocket local em relação a streams de vídeo HLS para projeção de cultos ao vivo.

## [0.2.0] - 2026-10-30

### Adicionado
- **Refinamento de Craft, Acessibilidade (A11y) e Design Tokens no Painel do Operador (Impeccable Audit):**
  - Eliminação de todos os anti-patterns de interface detectados pelo Impeccable (remoção de `gray-on-color` e substituição de `animate-bounce` por transições intencionais e suaves).
  - Suporte completo e nativo a `prefers-reduced-motion` no CSS global (`frontend/src/app.css`), honrando preferências de acessibilidade do usuário contra movimento excessivo.
  - Implementação completa do padrão WAI-ARIA em abas (`role="tablist"`, `role="tab"`, `aria-controls`, `aria-selected`, `role="tabpanel"`) e no redimensionador superior acessível por teclado com setas (`role="separator"`, `aria-orientation`) em `BottomPanel.svelte`.
  - Enriquecimento semântico com `aria-label`, `aria-expanded`, `aria-haspopup` e anéis de foco visíveis (`focus-visible:ring-2`) em todos os botões de ação, menus e modais do operador em `AdminView.svelte`.
  - Padronização de tokens de design semânticos no lugar de classes neutras estáticas (`bg-card`, `border-border`, `bg-muted`, `text-foreground`, `text-muted-foreground`).
  - Atualização do componente base `Button.svelte` para suportar e propagar atributos ARIA (`aria-label`, `aria-expanded`, `aria-haspopup`, `aria-controls`) de primeira classe.
  - Atualização do ecossistema de skills Impeccable para a versão mais recente (`v4.8.0`).
- **Painel Flutuante de Conexões WebSocket Ativas no Cabeçalho:**
  - O indicador "Online" no topo da interface agora é um botão interativo com contador de clientes conectados.
  - Ao clicar, exibe um dropdown moderno listando todas as conexões WebSocket ativas em tempo real, informando o tipo do cliente (Painel Operador, Projetor Público, Retorno de Palco), endereço IP do dispositivo, rota aberta (`/`, `/public`, `/stage`) e horário da conexão.
  - Adicionado endpoint `GET /api/connections` e sincronização instantânea via evento `ConnectionsUpdated`.
- **Busca Bíblica Integrada por Referência e Palavras no Command Palette Global:**
  - Adicionada aba dedicada **Bíblia Sagrada** no Command Palette global (`Ctrl+K` / `Cmd+K`), com navegação por teclado rápida e atalho `Tab`.
  - Suporte à busca de passagens bíblicas por referência (ex: `João 3:16`, `Sl 23:1-4`, `Romanos 8`), listando instantaneamente os versículos solicitados com opção de projetar passagem completa ou versículos individuais.
  - Suporte à busca textual por palavras-chave (ex: `luz do mundo`, `princípio`), consultando a API de pesquisa bíblica e listando resultados com livro, capítulo, versículo e texto.
  - Ações imediatas por teclado e mouse: `Enter` projeta o versículo ou passagem ao vivo, e `Shift+Enter` adiciona diretamente à setlist ativa.
  - Seletor rápido de versão bíblica instalada (ARA, NVI, etc.) integrado no cabeçalho do Command Palette.
- **Título Configurável para o Cronômetro no Stage Display:**
  - Adicionado campo de título editável e chips rápidos de sugestão ("Pregação", "Louvor", "Oração", "Avisos", "Palavra", "Ceia") no painel do cronômetro do operador.
  - O título definido é exibido em destaque no cabeçalho do bloco do cronômetro no monitor de palco (`/stage`), no modo standby e no preview do operador.
  - Presets de minutos e botões de incremento preservam o título customizado definido pelo operador.
- **Importação de Fundos de Imagens e Vídeos na Aba de Mídias > Fundos:**
  - Adicionado botão "Importar Fundo" na barra de ferramentas da aba de Fundos com suporte a imagens e clipes de vídeo (.mp4, .webm, .mov).
  - O backend passa a reconhecer e listar arquivos de vídeo na pasta de papéis de parede locais com streaming e detecção de MIME apropriada.
  - O sistema de resolução de planos de fundo identifica arquivos de vídeo e ativa reprodução em loop contínuo e silencioso (`autoplay`, `loop`, `muted`, `playsinline`) na tela pública (`/public`) e no preview do operador.
  - A galeria da aba de Fundos e o modal seletor exibem cartões interativos de vídeo com prévia em movimento e badge identificador `<Film /> Vídeo`.
- **Customização de Fundos de Letras, Estilos CSS e Cores de Texto nas Configurações:**
  - Adicionada a aba dedicada "Fundos de Letras & CSS" nas Configurações do OpenPresenter.
  - Permite configurar o CSS da propriedade `background` (gradientes lineares, radiais, mesh ou cores sólidas), classe de animação CSS e a cor específica do texto (`textColor`) para cada plano de fundo.
  - Simulador de Projeção 16:9 em tempo real dentro das Configurações, exibindo tipografia de letra e rodapé bíblico harmonizados na cor configurada para o fundo.
  - Criação de novos fundos customizados ilimitados com nome, estilos e cores próprias.
  - Seletor de cores hexadecimal com paleta rápida de cores de alto contraste recomendadas (Branco Puro, Off-White, Dourado/Ouro, Âmbar, Ciano Gelo, Menta Verde, Rosa Claro).
  - Ações para definir um fundo como padrão para letras de louvor, restaurar presets nativos ou excluir presets customizados.
  - A tela pública (`/public`) e o preview do operador renderizam instantaneamente o texto e rodapés utilizando a cor configurada para o fundo ativo com sincronização em tempo real via WebSocket.
- **Suporte a Sub-pastas na Aba de Imagens e Varredura Hierárquica de Mídias:**
  - O sistema de varredura de pastas de mídia locais agora percorre diretórios recursivamente, descobrindo automaticamente todas as sub-pastas e seus arquivos de imagem.
  - Na aba de Imagens do painel inferior, adicionada navegação expansível na árvore lateral de pastas, cards navegáveis de sub-pastas no topo da galeria e navegação por breadcrumbs para entrar e sair de sub-diretórios.
  - Imagens dentro de sub-pastas exibem identificador discreto de sua localização e podem ser apresentadas ao vivo ou adicionadas à setlist normalmente.
  - O modal de inserção de imagens para setlist passa a buscar e indicar sub-pastas na listagem de arquivos.
- **Auto-scale Dinâmico e Controle de Tamanho de Fonte na Tela Pública (/public):**
  - Implementado auto-scale responsivo para textos de músicas e versículos na projeção pública, ajustando dinamicamente o tamanho da fonte para preencher a tela de forma harmoniosa.
  - Quando a quebra de linha estiver desativada (`word_wrap: false`), eliminadas as reticências (`...`); o auto-scale reduz suavemente o tamanho da fonte para que todas as linhas caibam inteiramente na largura da tela.
  - Adicionadas configurações de **Tamanho Máximo da Fonte** (alvo de impacto para frases curtas, padrão 128px / 8rem) e **Tamanho Mínimo da Fonte** (padrão 24px) na aba de Projeção Pública.
- **Preset Grid Moderno Pro Padronizado, Auto-scale e Maximização de Espaço no Monitor de Palco (/stage):**
  - O monitor de palco e seu preview foram simplificados e unificados no preset **Grid Moderno Pro**, garantindo a melhor experiência visual para palco.
  - Implementado sistema de auto-scale dinâmico com medição e ajuste em tempo real (ResizeObserver e busca binária de font-size), fazendo com que o texto dos slides preencha todo o espaço do quadro sem transbordar.
  - Adicionada configuração de **Tamanho Mínimo da Fonte** (`min_font_size`, padrão 24px) nas Configurações de Palco para garantir legibilidade ideal mesmo em textos extensos.
  - Removidos gaps e cantos arredondados (`rounded-none`, `gap-0`, `p-0`) entre as caixas do palco para maximizar a área útil de visualização.
  - A caixa de mensagens e avisos para o palco agora permanece sempre visível na coluna lateral com estado neutro quando inativa, eliminando saltos bruscos de layout quando mensagens entram ou saem do ar.
- **Modo de Descanso (Standby) com Relógio e Cronômetro Gigantes no Monitor de Palco:**
  - Quando não há apresentação ativa (repouso com papel de parede da setlist no público), o monitor de palco (`/stage`) e seu preview exibem automaticamente um relógio em tipografia gigante, data por extenso e o cronômetro/timer centralizado com alertas de aviso e overtime.
  - Ao iniciar qualquer slide de música ou bíblia, a tela comuta instantaneamente para o preset configurado de slides.
- **Sincronização em Tempo Real de Configurações e Recarga Remota das Telas:**
  - Alterações salvas nas configurações agora disparam transmissão imediata via WebSocket (`WsMessage::SettingUpdated`), atualizando as telas `/public` e `/stage` instantaneamente sem necessidade de recarregar a página (F5).
  - Adicionados botões de ação rápida nos cabeçalhos de preview do painel do operador para recarga remota imediata das saídas de projeção e retorno de palco.
- **Ajuste de Margem para Logotipo da Igreja na Tela Pública:**
  - Adicionadas configurações deslizantes de margem horizontal e margem vertical na aba "Marca da Igreja" das Configurações.
  - O logotipo sobreposto na tela pública (`/public`) e no preview do operador agora respeita o distanciamento exato em pixels configurado para os quatro cantos da tela.
- **Ícone Oficial no System Tray e Executável Nativo:**
  - O ícone do System Tray agora carrega o bitmap da marca oficial derivado de `open-presenter-app-icon.svg`, garantindo traçado nítido e proporções perfeitas na bandeja do sistema.
  - Adicionado suporte a `winresource` via `build.rs` no Windows para embutir o arquivo `.ico` e metadados oficiais diretamente no executável `open-presenter.exe`.
- **Modo Compacto e Otimização Visual dos Botões da Interface:**
  - Os botões da barra de navegação de slides (Apresentar, Anterior, Próximo e Parar), da barra de saídas e do cabeçalho rápido operam por padrão em modo compacto (somente ícones) para economizar espaço de tela útil.
  - O botão "Apresentar" agora adota variante sóbria estável em repouso e variante pulsante apenas quando a apresentação estiver transmitindo ao vivo.
  - A opção na aba "Aparência & Cores" das Configurações permite ativar os rótulos de texto (labels) a qualquer momento.

### Alterado
- **Limpeza do Cabeçalho da Barra Lateral da Setlist:**
  - Removido o badge numérico com a contagem de itens ("X itens") do cabeçalho da setlist para evitar poluição visual.
  - Removido o botão de apagar todos os itens da setlist com ícone de borracha (`Eraser`) do cabeçalho, evitando limpezas acidentais durante o culto (a gestão de itens permanece nos menus contextuais e ações específicas).
- **Exibição do Cronômetro no Cabeçalho Somente Quando em Execução:**
  - O widget de cronômetro no cabeçalho do painel do operador agora é renderizado exclusivamente quando o timer estiver em execução (`isTimerRunning`), eliminando poluição visual quando pausado ou zerado.
- **Bíblia: Apresentação Rápida via Duplo Clique e Menu de Contexto Completo:**
  - O duplo clique em qualquer versículo bíblico agora ativa a projeção efêmera (`quick_media`) diretamente nas telas pública e de retorno de palco, sem poluir nem adicionar o item à setlist ativa.
  - Adicionado menu de contexto com botão direito nos versículos bíblicos com opções para "Adicionar à Setlist (selecionados)", "Adicionar Capítulo Completo à Setlist", "Apresentar ao Vivo" e "Copiar Texto com Referência".
  - Removido o efeito pulsante (`animate-pulse`) do badge "Ao Vivo" no card de versículo, mantendo um design estável e sem distrações visuais.
- **Barra de Setlist: Destaque de Item ao Vivo com Borda Verde:**
  - Removido o indicador pulsante/piscante (`animate-ping`) do item ativo para eliminar distrações visuais na barra lateral de setlist.
  - O item em apresentação ao vivo agora é destacado com uma borda verde nítida (`border-emerald-500 ring-1 ring-emerald-500/50`), fundo sutil esmeralda e indicador lateral verde sólido.
- **Modo de Descanso no Monitor de Palco (/stage):**
  - O bloco do cronômetro agora é exibido exclusivamente quando o timer estiver em execução (`running`) ou em overtime, evitando poluição visual quando inativo.
  - Removidos os badges de "Modo de Descanso (Standby)" e texto de orientação do rodapé, deixando o relógio gigante e a data centralizados de forma ultra limpa e minimalista.
- **System Tray e Abertura do Navegador:**
  - O ícone do System Tray agora utiliza a versão oficial com fundo preto e traçado branco (`open-presenter-icon-dark-32.png`), integrando-se perfeitamente com temas escuros da barra de tarefas e menu bar.
  - O navegador agora abre automaticamente apenas uma única vez na inicialização da aplicação, eliminando a reabertura indesejada da aba do painel em interações com a bandeja do sistema. A reabertura sob demanda continua disponível a qualquer momento pelo menu nativo ("Abrir Painel de Controle").
- **Interface do Operador:**
  - Removidos os botões duplicados de alternância de sidebars (Setlist e Previews) do cabeçalho superior; o controle de recolher e expandir agora fica centralizado exclusivamente nos botões nativos dos cabeçalhos dos painéis e nos rails laterais colapsados.
  - Removido o subtítulo "Projeção para igrejas e eventos" do cabeçalho superior para deixar a barra de ferramentas mais limpa e compacta.

### Corrigido
- **Ícone e Transparência do System Tray (macOS Menu Bar e Bandeja do Sistema):**
  - Corrigido o visual do ícone da aplicação no System Tray, eliminando o bloco quadrado escuro e os cantos brancos opacos na barra de menus.
  - Implementado novo asset bitmap transparente da marca (`open-presenter-icon-tray-32.png` e `@2x` 64x64) com fundo 100% transparente e silhueta preenchida com botão play vazado em espaço negativo.
  - Ativado o modo template nativo no macOS (`with_icon_templated`), permitindo que o sistema operacional aplique automaticamente a cor apropriada (branca ou preta) e gerencie estados de seleção de acordo com a aparência do sistema (modo claro ou escuro).
- **Design da Referência Bíblica na Projeção Pública (/public) e Monitores de Retorno:**
  - Eliminada a duplicação de versão (ex.: "ARA ARA") na barra de referência bíblica através de sanitização automática de rótulo e metadado de versão.
  - Removido o emoji `📖` e a tipografia amarela estridente, adotando visual branco refinado com drop-shadow integrado ao tema da projeção.
  - A barra de referência bíblica agora fica ancorada elegantemente no rodapé da área de texto (`mt-auto`), com distribuição horizontal (`justify-between`) entre a referência (à esquerda) e a sigla da tradução (à direita).
- **Espremimento no Catálogo de Músicas com Prévia Aberta:**
  - Corrigido o espremimento e quebra dos cards de música quando o painel de visualização/preview de letra está aberto.
  - Implementado layout adaptativo mestre-detalhe: o catálogo de músicas à esquerda se ajusta em coluna única com largura estável (`w-80 sm:w-96`), botões protegidos contra quebra e destaque para a música ativa, enquanto o painel de prévia preenche fluentemente o espaço restante com slides em proporção 16:9.
- **Monitor de Palco (/stage):**
  - Corrigido erro de execução JavaScript no modo Standby (`ReferenceError`) provocado por identificadores incorretos de timer e overtime, eliminando a tela preta e restaurando a exibição do relógio e cronômetro gigantes.
- **Aplicação e Contenção de Margens na Projeção Pública:**
  - O container de projeção pública (`/public`) agora adota `box-sizing: border-box` e contenção fluida `w-full` (eliminando o limite rígido de `max-w-[96vw]`), garantindo que as margens superior, inferior e laterais sejam fielmente respeitadas sem vazamentos ou cortes de texto.
- **Separação de Papel de Parede da Setlist vs. Fundo das Letras:**
  - O papel de parede selecionado na setlist agora é mantido na tela pública durante os momentos de repouso (sem apresentação ativa).
  - Ao iniciar a projeção de uma música ou versículo bíblico, o projetor comuta automaticamente para o fundo padrão configurado para letras, restaurando o contraste visual do texto.
  - Ao encerrar a apresentação (Esc / Parar Projeção / Limpar), a tela pública retorna automaticamente ao papel de parede da setlist.

## [0.1.0] - 2026-10-29

### Adicionado

- **Sistema de Atualização Automática Completa (Auto-Updater Multiplataforma):**
  - Módulo backend [`crates/op-admin/src/updater.rs`] com verificação contínua e sob demanda de novas releases via API oficial do GitHub (`open-mission/openpresenter`).
  - Comparação semântica de versões (`semver`) garantindo que apenas releases estritamente mais recentes disparem alertas.
  - Mapeamento inteligente de assets de download conforme arquitetura e SO (`.zip` no Windows, `.tar.gz` no macOS e Linux).
  - Download em background com relatório de progresso percentual via endpoint `/api/updater/status` e `/api/updater/install`.
  - Substituição atômica e segura do executável (`std::env::current_exe()`), contornando locks de arquivo em execução no Windows via renomeação temporária `.old.exe` e mantendo permissões executáveis Unix (`0o755`).
  - 100% de preservação de dados: o banco SQLite (`open-presenter.db`), mídias e bíblias continuam intactos no diretório de dados do usuário.
  - Interface no Painel de Controle e no System Tray com changelog detalhado da nova versão e botão de atualização em 1 clique.
- **Executável Nativo Autocontido com System Tray (Windows, macOS e Linux):**
  - Distribuição do OpenPresenter como binário executável único, pronto para uso com duplo clique sem necessidade de instalação de Node.js, Bun ou ferramentas de terminal.
  - Ícone nativo na bandeja do sistema operacional (System Tray / Menu Bar) com menu de contexto rápido para abrir o Painel de Controle, a Tela Pública (Projetor), o Retorno de Palco (Stage), configurar a porta do servidor e encerrar o aplicativo com segurança.
  - Abertura automática do navegador padrão na tela do operador ao iniciar a aplicação.
  - Suporte à flag `--headless` e variável `OPENPRESENTER_HEADLESS=1` para execução em servidores sem interface gráfica ou contêineres.
- **Porta Padrão Segura (18080) e Configuração Interativa:**
  - Alteração da porta padrão do servidor para `18080`, eliminando conflitos frequentes com outros serviços web e ferramentas de desenvolvimento locais (`3000`, `8080`, `8000`).
  - Diálogo nativo interativo para alterar a porta do servidor diretamente pelo menu do System Tray, com persistência automática no banco SQLite (`app_settings.server_port`) e precedência sobre variável de ambiente `PORT`.
- **Assets de Frontend Embutidos no Binário (`rust-embed`):**
  - Incorporação completa da aplicação SPA Svelte compilada (`frontend/dist`) no interior do executável Rust em tempo de compilação, permitindo transporte do binário sem pastas adicionais.
  - Estratégia híbrida com fallback em tempo de desenvolvimento para recarga rápida do disco.
- **Automação de CI/CD e Publicação de Releases Multiplataforma (GitHub Actions):**
  - Workflow `.github/workflows/release.yml` com compilação automatizada em matriz para Windows x64 (`.zip`), macOS Apple Silicon e Intel (`.tar.gz`) e Linux x64 (`.tar.gz`).
  - Step final com `softprops/action-gh-release@v2` para publicação de release oficial anexando todos os arquivos de build gerados.
- **Comando no Command Palette e Atalhos de Teclado para Abrir Bíblia Sagrada:**
  - Novo comando rápido "Abrir Bíblia Sagrada" (`BookOpen`) nas ações do `CommandPalette` (⌘K).
  - Atalho global de teclado `⌘B` / `Ctrl+B` e tecla de função `F4` no painel administrativo (`AdminView`) para expandir o painel inferior e focar na aba de Bíblia instantaneamente.
- **Visualização dos Livros Bíblicos em Grade (Grid) no Modo de Siglas:**
  - Ao alternar para o modo de siglas canônicas (`bookViewMode === 'abbreviations'`), os livros da Bíblia são dispostos em formato de grade compacta (`grid-cols-3 sm:grid-cols-4`) com botões táteis e tooltips informativos, facilitando a escolha rápida sem rolagem vertical extensa.
- **Suporte a Versão Bíblica na Busca com Ativação Automática:**
  - O parser de referências bíblicas (`parseBibleReference` e `parse_bible_reference`) agora reconhece a sigla da versão ao final da passagem (ex: `Salmos 91 ARA`, `João 3:16 NVI`, `1 Co 13:4-7 arc`).
  - Digitar uma passagem com versão instalada ativa a versão correspondente na hora no seletor de versões e carrega os versículos da tradução informada. Digitar a sigla da versão isoladamente também ativa a versão imediatamente.
- **Busca de Versículos por Palavra Exata (Whole Word) com Input Group e Aspas:**
  - Botão toggle `[Ab]` integrado na barra de busca em formato de input group para alternar entre busca por substring (`LIKE '%amor%'`) e palavra inteira isolada (`matches_whole_word`).
  - Suporte automático a sintaxe de busca com aspas (ex: `"amor"`), garantindo que termos como "amor" não tragam falsos positivos como "amorreus" ou "desamor".
- **Coluna de Capítulos Ampliada e Grade Readequada:**
  - Aumento da largura padrão da segunda coluna de 140px para 180px no `BiblePanel`, com grade expandida (`grid-cols-4 sm:grid-cols-5`) e botões numéricos mais confortáveis para clique rápido em cultos ao vivo.
- **Ação Rápida de Limpeza de Itens da Setlist Ativa:**
  - Novo endpoint HTTP `POST /api/setlists/items/clear` para remover todos os itens (músicas, versículos, avisos e imagens) da setlist ativa com um único clique.
  - Finalização segura da apresentação caso um item removido estivesse em exibição ao vivo, sincronizando o novo estado via WebSocket.
  - Botão de ação rápida com ícone de borracha (`Eraser`) diretamente no cabeçalho do painel de Setlist no painel operacional (`AdminView`).
  - Opção "Limpar Itens da Setlist" no dropdown de gerenciamento de setlists, com confirmação prévia para evitar acionamentos acidentais.
- **Componente Dedicado de Preview Fiel da Projeção Pública (`PublicPreviewBox`):**
  - Espelhamento idêntico 16:9 da tela pública no painel operacional (`AdminView`), respeitando papel de parede (cores gradientes CSS ou imagens em modos `cover`, `contain`, `repeat`, `center`), margens proporcionais, modos de quebra (`word_wrap`), preenchimento (`fill_space`), logo da igreja com posicionamento e opacidade, blackout e clear text.
- **Configuração de Escurecimento Geral de Fundo e Placa de Leitura nos Textos:**
  - Opção `darken_background` (Padrão: desligado / `false`): película translúcida escura sobre todo o plano de fundo apenas quando houver slide ativo.
  - Opção `text_box_background` (Padrão: desligado / `false`): placa translúcida com cantos arredondados posicionada diretamente atrás do texto e versículos bíblicos para contraste seletivo sem escurecer a imagem inteira.
  - Controles e textos explicativos profissionais na aba "Tela Pública" das configurações.

### Alterado

- **Widget de Cronômetro & Palco Colapsável na Coluna Lateral de Saídas:**
  - O painel de controle de cronômetro e mensagens de palco (`TimerControlPanel`) no rodapé da coluna de saídas agora conta com opção de recolher/expandir com chevron animado, exibindo um cabeçalho compacto com relógio e tempo restante quando colapsado para maximizar a área de trabalho.
- **Barra de Busca da Setlist com Input Group e Filtros Retráteis:**
  - O campo de pesquisa da Setlist (`AdminView.svelte`) foi integrado a um botão toggle de filtros em formato de input group (`Filter`), mantendo as pílulas de categorias (Todos, Louvores, Bíblia, Avisos) recolhidas por padrão e reveladas apenas sob demanda.
- **Ajustes de Proporções e Tipografia no Preset Grid Moderno do Monitor de Palco (`/stage`):**
  - Equiparação da escala tipográfica do próximo slide ao slide atual (`text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black`), eliminando a diferença excessiva de tamanho de fonte.
  - Distribuição equilibrada da coluna lateral em repouso: 50% para Horário e 50% para Cronômetro.
  - Distribuição com aviso de palco ativo: 60% para o card de Aviso com tipografia gigante (`text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-black`), 20% para Horário e 20% para Cronômetro.
  - Atualização do componente `StagePreviewBox` para refletir fielmente as mesmas proporções de 50%/50% e 60%/20%/20%.

- **Ampliação da Referência Bíblica na Projeção Pública:**
  - Aumento significativo da escala tipográfica das referências bíblicas na tela pública (`PublicView.svelte`) e no preview operacional (de `text-sm` para `text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-black`), com badge da versão bíblica em alto contraste para legibilidade nítida à distância em telões e projetores de alta resolução.
- **Remoção do Backdrop Escuro Fixo Padrão na Tela Pública:**
  - O papel de parede agora mantém sua luminosidade e cores originais por padrão, ativando escurecimento somente se explicitamente habilitado nas preferências pelo operador.
- **Preview Dinâmico do Stage nas Configurações:**
  - Integração do componente real `StagePreviewBox` na área de simulação de retorno do modal de configurações, refletindo instantaneamente as alterações de presets, cores e sliders de proporções (atual, próximo, rodapé) em tempo real.

- **Módulo Completo de Bíblias Sagradas (SQLite) e Projeção de Escrituras:**
  - Suporte nativo a versões da Bíblia Sagrada em formato SQLite (`.sqlite`, ex: Almeida Revista e Atualizada - ARA, NVI, ARC, NVT), com armazenamento e cópia automática para `<data_dir>/bibles/`.
  - Migração de schema v7 (`0007_bibles.sql`) adicionando tabela `bibles` com rastreamento de versões, abreviações canônicas e definição de versão padrão.
  - Leitor de SQLite otimizado (`BibleReader`) com criação automática de índices (`idx_verse_book_chap`), validação de integridade e consultas de livros, capítulos, versículos e busca textual.
  - Aba de configurações "Bíblias Sagradas" em `SettingsModal` para importação de arquivos `.sqlite`, importação por caminho local, definição de versão padrão, exclusão e edição/renomeação de nomes e siglas canônicas.
  - Nova aba "Bíblia" no painel inferior (`BottomPanel` / `BiblePanel`) com layout de três colunas redimensionáveis:
    - Coluna 1: Livros da Bíblia (com filtro Antigo/Novo Testamento, busca rápida e alternância entre nomes completos e abreviações canônicas).
    - Coluna 2: Lista de capítulos do livro selecionado.
    - Coluna 3: Grade de slides de versículos em 2 colunas com tipografia clara, identificação visual e indicação de status Ao Vivo.
  - Interação e navegação ao vivo de versículos:
    - Duplo clique em qualquer versículo projeta instantaneamente o texto na tela pública e no monitor de palco (`quickPresentMedia`).
    - Navegação com setas do teclado (`ArrowLeft` e `ArrowRight`) para avançar ou recuar versículos consecutivamente.
    - Duplo clique em outro slide navega automaticamente a projeção para o novo versículo.
  - Seleção de múltiplos versículos com `Shift + Clique` para adição de versículos individuais ou intervalos inteiros à setlist ativa com contexto/ação dedicada.
  - Parser inteligente de referências bíblicas em português (`parseBibleReference` e `parse_bible_reference`):
    - Reconhecimento automático de formatos variados: `joao 3.16`, `joão 3:16`, `jo 3 16`, `1 co 13:4-7`, `Salmos 91`, `sl 91`.
    - Suporte a disambiguação canônica refinada (ex: `Jó` vs `João`).
    - Quando apenas livro e capítulo são informados (ex: `Salmos 91`), o texto abre focando diretamente no verso 1.
    - Integração no `CommandPalette` (⌘K): digitação de passagem bíblica exibe card dedicado de ação imediata; pressionar `Enter` navega até a passagem e `Shift + Enter` projeta o versículo instantaneamente ao vivo na tela.
  - Suporte aprimorado no Monitor de Palco (`/stage`) e no telão público (`/public`):
    - Emblema/badge em destaque para escrituras bíblicas (`📖 {livro} {cap}:{verso} ({versão})`) no slide atual e prévia clara do próximo versículo.
    - Metadados e referência bíblica destacados na projeção pública.

### Removido

- **Botões Redundantes "Abrir Janela" nos Previews da Barra Lateral:**
  - Remoção dos botões "Abrir Janela do Public" e "Abrir Janela do Stage" que ficavam abaixo das caixas de preview 16:9 na barra lateral de saídas (`AdminView.svelte`), mantendo a abertura de telas e janelas centralizada na barra superior do operador (header) e no modal de conexões/QR Code.

### Corrigido
- **Monitor de Palco (/stage):**
  - Corrigido erro de execução JavaScript no modo Standby (`ReferenceError`) provocado por identificadores incorretos de timer e overtime, eliminando a tela preta e restaurando a exibição do relógio e cronômetro gigantes.

- **Sincronização Reativa Automática das Versões da Bíblia ao Fechar Configurações:**
  - Criação da store reativa `biblesStore` e do utilitário `refreshBibles()`, garantindo que importações, renomeações, exclusões e alterações de versão padrão feitas no modal de configurações atualizem imediatamente o seletor do painel de Bíblias sem necessidade de recarregar a página.
- **Suporte a Upload de Arquivos SQLite Grandes de Bíblias (Body Limit no Axum):**
  - Configuração de `DefaultBodyLimit::max(128MB)` nas rotas HTTP do backend Axum, eliminando o erro `Failed to buffer the request body: length limit exceeded` ao importar bancos SQLite de Bíblias (geralmente entre 3MB e 10MB) via upload de arquivo no navegador.
- **Correção da Simulação Gráfica dos Presets do Monitor de Palco nas Configurações:**
  - A amostra interativa de distribuição de espaço em `SettingsModal` agora reflete fielmente o layout do preset ativo selecionado (`split-50-50`, `modern-grid`, `video-focus` e `horizontal-stack`).
  - Aplicação visual das proporções configuradas (atual, próximo, rodapé) e das cores de borda e texto em tempo real na prévia do modal.

- **Compartilhamento de Saídas com QR Code e URLs de Rede Local:**
  - Novo modal `ShareOutputsModal` acessível diretamente pelo cabeçalho do operador via botão "QR Code".
  - Geração dinâmica de QR Code escaneável com o IP da máquina na rede local para conexão imediata de smartphones e tablets de músicos, pregadores e equipe técnica.
  - Abas dedicadas para o Monitor de Palco (`/stage`) e Projeção Pública (`/public`), com botão de cópia de link e abertura no navegador.
  - Novo endpoint `GET /api/system/network-info` no backend Rust com descoberta automática de IP local via sockets de rota.
- **Ajustes de Projeção de Imagem (Edge-to-Edge e Encaixe Customizável):**
  - Opção para ignorar margens de texto para imagens e vídeos (`ignore_margins_for_media`), permitindo projeção 16:9 de ponta a ponta sem bordas pretas adicionais.
  - Seletor de modo de encaixe de imagens (`contain`, `cover`, `fill`, `center`) configurável nas preferências globais e em barra de ferramentas rápida na aba de imagens.
- **Apresentação Rápida de Mídias Avulsas e Modal de Prévia:**
  - Botão "Apresentar" na galeria projeta a imagem instantaneamente ao vivo (`POST /api/presentation/quick-media`) sem poluir a setlist salva do culto.
  - Novo modal `ImagePreviewModal` com botão de visualização (`Eye`), exibindo a imagem em alta resolução, cálculo automático de resolução e proporção (16:9, 4:3, etc.), dados de origem e ações rápidas.
  - Botão dedicado `+ Setlist` para inclusão intencional de imagens no repertório.
- **Reprodução Fiel de Mídia e Tipografia Gigante no Monitor de Palco (`/stage`):**
  - O palco agora renderiza a imagem propriamente dita (`<img>`), eliminando a exibição incorreta do caminho de arquivo em texto puro.
  - Exibição de miniatura gráfica para o próximo slide caso também seja uma imagem.
  - Aumento expressivo das escalas tipográficas dos slides (atual e próximo) e redução de paddings no preset vertical (`modern-grid`) e demais layouts para visibilidade nítida a longa distância (10 a 20 metros).
- **Preview Fidedigno do Stage no Painel Administrativo:**
  - Novo componente `StagePreviewBox` que reproduz fielmente na barra lateral do operador o preset ativo, cores de texto e borda, proporções configuradas, relógio, contagem do timer e mensagens de palco.
- **Controle Preciso de Parada do Timer em 00:00:**
  - O cronômetro regressivo agora para automaticamente ao atingir zero (`running = false; remaining_seconds = 0`), evitando contagem negativa não solicitada.
  - Opção `allow_overtime` para permitir overtime contínuo sob demanda do operador.
- **Widget de Cronômetro Persistente no Header do Operador:**
  - Mostrador digital integrado ao cabeçalho principal do painel administrativo (`AdminView`), exibindo a contagem regressiva em tempo real com tipografia tabular mono.
  - Indicadores visuais de status (ativo em cor primária, alerta em amarelo nos 60s finais e contagem negativa em vermelho pulsante durante overtime).
  - Ações rápidas de Play/Pause e Reset (`RotateCcw`) diretamente no header, permitindo controle imediato do tempo mesmo com o painel lateral de previews recolhido.
- **Distribuição Dinâmica de Espaço no Monitor de Palco (`Stage Display`):**
  - Configuração de proporções percentuais entre o Slide Atual, Próximo Slide e Rodapé de Tempo/Relógio na aba de Configurações de Stage.
  - Prévia gráfica interativa com simulação da distribuição de tela e botão "Restaurar Proporções" (padrão 60% atual / 30% próximo / 10% rodapé).
  - Aplicação responsiva das proporções personalizadas nos 4 presets de layout do palco (`split-50-50`, `modern-grid`, `video-focus` e `horizontal-stack`).

### Modificado

- **Estabilização do Modal de Configurações e Salvamento Centralizado:**
  - Resolução do problema onde alterações na aba de Tela Pública ou outras opções causavam o reset automático da aba de navegação.
  - Leitura segura em snapshot das stores no momento de abertura do modal e adição de botão explícito "Salvar Alterações" no rodapé com feedback visual de confirmação.

- **Timer Global Sincronizado, Avisos de Palco e Overtime ProPresenter:**
  - Motor de cronômetro e timer regressivo executando em background no backend Rust via task Tokio assíncrona, com broadcast de ticks de 1s via WebSocket para todas as conexões (Admin, Public, Stage).
  - Alerta visual no último minuto (< 60s) e contagem negativa contínua em Overtime (`-01:30`, `-02:30`) em vermelho pulsante quando o tempo expira.
  - Painel de controle integrado na barra lateral direita do operador (`TimerControlPanel`), com mostrador digital, controles de Play/Pause/Reset/Stop, presets rápidos (`5m`, `10m`, `15m`, `+1m`, `+5m`) e campo para envio de mensagens e alertas de palco (`stage_message`).
- **Layouts e Presets do Monitor de Palco (`/stage`) com Cores Customizáveis:**
  - Quatro presets estruturais configuráveis:
    - `split-50-50` (Divisão Equilibrada): Caixas de slide atual e próximo lado a lado com etiquetas em destaque e rodapé de tempo.
    - `modern-grid` (Grid Moderno Pro): Slide atual em destaque no topo, próximo abaixo e coluna lateral para aviso de palco, relógio de parede e timer regressivo proeminente.
    - `video-focus` (Foco em Projeção): Centro expandido para leitura e barra inferior compacta de contadores.
    - `horizontal-stack` (Faixas Horizontais): Header superior com relógio/timer, centro em tipografia máxima e próximo slide em faixa na base.
  - Personalização de cores com seletores para slide atual (texto e borda), próximo slide (texto e borda), relógio e timer.
- **Configurações da Projeção Pública (`/public`):**
  - Opção para maximizar a escala tipográfica das letras na tela pública (`fill_space`).
  - Opção para controle de quebra de linhas longas (`word_wrap`), permitindo manter cada verso em linha única sem quebra (`whitespace-nowrap`).
  - Controle de exibição de metadados (`show_metadata`: nome do louvor e cantor/compositor).
  - Edição de margens personalizadas do slide público (topo, base e laterais) com prévia gráfica de safe area no modal de configurações.
- **Identidade Visual e Marca da Igreja (`Church Brand`):**
  - Upload e armazenamento local no servidor dos logotipos da igreja (`logo_primary`, `logo_dark`, `logo_light`, `logo_full`) servidos em `/media/church_brand/`.
  - Sobreposição de logotipo configurável no slide público com suporte a 4 cantos (`top-left`, `top-right`, `bottom-left`, `bottom-right`), escala de tamanho e opacidade.
- **Persistência de Preferências com Nova Tabela de Configurações:**
  - Migração `0006_app_settings.sql` (schema v6) com métodos `get_setting`, `set_setting` e `list_settings` no SQLite para salvar opções de exibição, marca e stage de forma persistente.
- **Favicon Oficial do Aplicativo:**
  - Vinculação do ícone minimalista oficial do OpenPresenter como favicon SVG e apple-touch-icon no `index.html`.

- **Cancelamento Completo de Apresentação e Limpeza de "Ao Vivo":**
  - Novo comando `StopPresentation` via WebSocket e endpoint `POST /api/presentation/stop`.
  - Pressionar a tecla `Esc` ou clicar em "Parar" desativa instantaneamente a apresentação ao vivo, zerando `active_media_id` e `active_slide_index`.
  - Remoção imediata do badge pulsante "AO VIVO" nos slides e limpeza de projeção de imagens e textos nas saídas públicas e nos previews de tela do operador.
- **Tree Navigation Vertical e Breadcrumbs nas Abas de Mídia:**
  - Coluna lateral de navegação em árvore vertical para a aba de Imagens (Todas, Pasta do App, e Pastas Mapeadas expansíveis) e para a aba de Fundos.
  - Barra de Breadcrumbs dinâmica no topo do painel inferior para navegação hierárquica e retorno em um clique.
- **Aba de Fundos com Gradientes, Mesh e Animações CSS:**
  - Nova aba "Fundos & Gradientes" no painel inferior com biblioteca visual para projeção em cultos e eventos: Gradientes Lineares escuros, Mesh Gradients multicamadas e Animações CSS suaves (`@keyframes` de aurora boreal, nebulosa e fluxo cósmico).
  - Aplicação instantânea ao vivo no projetor (`/public`) com 1 clique (mesmo sem apresentação ativa ou durante slides) via endpoint `POST /api/presentation/wallpaper` e WebSocket.
  - Modal seletor visual `BackgroundSelectorModal` integrado também ao modal de edição de Setlist (`SetlistModal`).
- **Sheet de Letras de Música Ampliado em 2 Colunas:**
  - Abertura direta do Sheet ao clicar em qualquer parte do cartão da música no catálogo.
  - Largura expandida do Sheet lateral e exibição dos slides formatados em grade de 2 colunas no aspecto 16:9.
- **Seletor Gráfico Nativo de Pastas (File Picker):**
  - Diálogo gráfico nativo do sistema operacional (`osascript` no macOS com fallback web via `input[webkitdirectory]`) para seleção direta de diretórios de mídia em Configurações.
  - Endpoint `POST /api/system/pick-folder` no backend Rust para integração com o Finder.
- **Exibição e Abertura do Diretório de Dados do Sistema:**
  - Card em destaque na aba "Arquivos e Pastas" exibindo o caminho absoluto da pasta raiz de dados do app (`data_dir`), com botões para "Copiar Caminho" e "Abrir no Finder".
  - Endpoint `POST /api/system/open-path` para revelação direta de diretórios e arquivos no gerenciador do sistema operacional.
- **Inserção de Imagens e Pastas de Imagens na Setlist:**
  - Novo modal `ImageSelectorModal` acessível pelo menu suspenso `+` e menu de contexto da Setlist.
  - Suporte à seleção múltipla de imagens avulsas ou de uma pasta inteira (interna ou mapeada), criando apresentações ordenadas de slides visuais.
- **Ações Rápidas de Apresentação em Imagens:**
  - Botão visível de "Apresentar" (Play) e "+ Setlist" em cada miniatura da galeria do painel inferior de mídia, além do suporte a duplo clique.
- **Sheet Lateral de Visualização de Letras de Música:**
  - Gaveta lateral (Sheet) deslizante na aba de Músicas do painel de mídia para conferência rápida e leitura da letra completa estruturada por seções musicais, com ações de projetar ao vivo ou adicionar à setlist.
- **Atalhos Dedicados Livres de Conflito:**
  - Atalhos globais sem colisão com partes musicais: `.` / `Alt+B` / `F1` para Blackout, `,` / `Alt+C` / `F2` para Limpar Texto, e `/` / `Alt+W` / `F3` para Papel de Parede.
  - Novo botão de ação rápida "Fundo" (Papel de Parede) na barra de controle de saídas.

### Modificado

- **Aplicação Completa de Temas e Cores Shadcn UI:**
  - Mapeamento explícito no `@theme` do Tailwind v4 de tokens semânticos (`--color-background`, `--color-primary`, `--color-border`, etc.) apontando para as variáveis do `:root`.
  - Padrão do sistema configurado como `neutral` com suporte fiel aos modos escuro e claro.
  - Componentes `Button` e `Badge` atualizados para renderizar as variáveis do tema e destaque ativas.
- **Layout e Responsividade do Modal de Configurações:**
  - Largura expandida para `max-w-4xl xl:max-w-5xl`, prevenindo colisão de textos com badges e aplicando truncamento e quebra de linha segura em caminhos longos de diretórios.

## [0.7.0] - 2026-10-29

### Adicionado

- **Cores e Temas no Padrão Shadcn UI:**
  - Configuração independente de **Base Color** (Neutral, Stone, Zinc, Mauve, Olive, Mist, Taupe) para tonalidade de fundo, cartões, bordas e textos neutros.
  - Configuração de **Theme / Accent Color** (Neutral, Amber, Blue, Cyan, Emerald, Fuchsia, Green, Indigo, Lime, Orange, Pink, Purple, Rose, Violet, Yellow) para destaques operacionais, botões primários e anéis de foco.
  - Injeção dinâmica de variáveis CSS no `:root` com persistência em `localStorage`.
- **Sidebars Colapsáveis (Setlist e Previews):**
  - Possibilidade de recolher a barra lateral da Setlist (esquerda) e o painel de Previews do Public/Stage (direita) com botões dedicados no cabeçalho e nas próprias barras laterais.
  - Modos rail estreito colapsado com rótulos verticais e botões de restauração em um único clique, liberando largura total para a grade central de slides.
- **Reorganização de Itens por Arrastar e Soltar (Drag and Drop):**
  - Reordenação intuitiva de itens na Setlist arrastando os manipuladores `GripVertical`.
  - Indicadores visuais de drop zone e sincronização automática via WebSocket e persistência no SQLite.
- **Editor Unificado para Músicas e Textos Livres / Avisos:**
  - Layout modal ocupando 95% da tela com editor de texto à esquerda e prévia 16:9 de slides à direita.
  - Criação rápida de apresentações de avisos, anúncios ou orações sem necessidade de metadados musicais.
- **Suporte a Markdown Typeset e Alinhamento nos Slides:**
  - Barra de ferramentas typeset com formatação de **negrito** (`**`), *itálico* (`*`), <u>sublinhado</u> (`<u>` / `__`) e 🟨 **realce/highlight** (`==` / `<mark>`).
  - Alinhamento horizontal (Esquerda, Centro, Direita, Justificado) e vertical (Topo, Centro, Base) dos slides.
  - Renderização consistente da formatação na tela operacional, na tela pública e no retorno de palco.
- **Painel Inferior Colapsável e Redimensionável (Bottom Panel):**
  - Aba "Músicas": catálogo completo do banco com busca instantânea e ações de apresentação imediata ou adição à setlist.
  - Aba "Imagens": galeria de mídias com visualização em grade, upload para a pasta interna do app e projeção direta.
  - Borda superior redimensionável por arrasto, botão de recolher em barra discreta de ~36px e botão de maximização em 100%.
- **Aba "Arquivos e Pastas" nas Configurações:**
  - Exibição transparente das pastas do sistema do app (`data_dir`, banco de dados, `media/images`, `media/videos`, `media/wallpapers`, `fonts`).
  - Mapeamento de pastas externas de imagens do usuário no disco sem duplicação de arquivos (zero-copy), servidas com segurança pela API REST.
  - Nova tabela SQLite `media_folders` (migração `0005_media_folders.sql`) com schema versão 5.

### Adicionado

- **Menus de Contexto na Setlist:**
  - Clique direito em itens da setlist com ações de Fixar no topo, Editar (música, seção ou texto), Renomear e Remover da Setlist (com interrupção automática da projeção se o item estiver ao vivo).
  - Clique direito em área vazia da sidebar Setlist para adicionar rapidamente Música, Seção ou Texto Livre / Aviso.
  - Menu suspenso integrado no botão `+` do cabeçalho da Setlist para criação rápida de Música, Seção e Texto Livre.
- **Múltiplas Setlists:**
  - Nova migração `0004_setlists.sql` no SQLite com tabela `setlists` para persistência de múltiplas programações com Título, Descrição, Papel de Parede, Modo de Ajuste e itens ordenados.
  - Seletor de Setlist com menu dropdown no topo da barra lateral para alternar cultos/eventos, criar novas programações ou gerenciar a atual.
  - Modal de gerenciamento de Setlist (`SetlistModal`) com seleção de wallpapers e upload local.
- **Papel de Parede da Projeção e Modos de Ajuste:**
  - Projeção do papel de parede da setlist ativa na tela pública (`/public`) quando em repouso ou em modo `cleartext`.
  - Suporte a 4 modos de ajuste: Preencher (`cover`), Ajustar (`contain`), Centralizar (`center`) e Repetir (`repeat`).
  - Configuração do modo padrão de ajuste na aba de Telas e Saídas do painel de Configurações.
- **Armazenamento de Mídias em Pasta do Sistema:**
  - Subpasta `<data_dir>/media/wallpapers/` criada e gerenciada diretamente no sistema de arquivos, incluindo papéis de parede padrão na inicialização.
  - Endpoints REST para listagem de papéis de parede (`/api/media/wallpapers`), upload (`/api/media/upload`) e entrega estática (`/api/media/files/*`).
- **Correção Visual do Header:**
  - Eliminação da duplicação do logotipo no cabeçalho e exibição do ícone oficial em proporção nítida acompanhado de tipografia elegante.
- **Cores Neutras do Shadcn-UI:**
  - Adoção da paleta Neutra (`neutral`) como padrão do sistema.
  - Suporte à customização de famílias neutras (Neutral, Zinc, Slate, Stone, Gray) na aba de Aparência das Configurações.


## [0.5.0] - 2026-10-29

### Adicionado

- **Controle de Apresentação e Foco de Slides:**
  - O clique simples em um slide no painel central agora seleciona e foca o slide sem colocá-lo imediatamente no ar (prevenindo disparos acidentais ao vivo).
  - Adicionado botão dedicado **Apresentar (Enter)** com ícone Play e suporte a duplo clique no slide.
  - Tecla `Enter` no slide focado dispara a projeção imediatamente.
  - Tecla `Escape` (`Esc`) encerra a apresentação imediatamente e oculta o texto projetado (`cleartext`).
  - Navegação contínua por teclado (`←`, `→` ou `Espaço`) respeitando o estado ao vivo da projeção.
- **Atalhos Rápidos de Seções e Buffer Numérico:**
  - Atalhos de letras individuais para saltar instantaneamente entre partes da música: `V` para Verso, `R` para Refrão, `P` para Ponte, `I` para Intro, `F` para Final, etc.
  - Ao pressionar a letra repetidamente, a navegação cicla sequencialmente entre todos os slides daquela seção (ex: Verso 1 → Verso 2 → Verso 3 → Verso 1).
  - Buffer numérico com feedback visual flutuante (HUD): digitar o número do slide (ex: `2`) e pressionar `Enter` projeta diretamente o slide ordinal indicado.
- **Painel de Configurações com Abas Verticais (`SettingsModal`):**
  - Modal de configurações com navegação vertical lateral, inspirado no padrão macOS e ProPresenter.
  - **Aparência:** alternância em tempo real entre temas Sistema, Escuro (Dark) e Claro (Light), com persistência local.
  - **Partes e Cores:** personalização completa das seções de música, com customização do nome, atalho de teclado e paleta de cores (Esmeralda, Azul, Âmbar, Ciano, Teal, Roxo, Rosa, Índigo) com reflexo imediato nos cartões da grade central.
  - **Telas e Saídas:** monitoramento de status e atalhos diretos para abrir as janelas Public e Stage.
  - **Atalhos Rápidos:** guia interativo e documentado de atalhos de teclado do operador.
  - **Sobre o App:** informações de versão, arquitetura e licença do OpenPresenter.
- **Identidade Visual Oficial:**
  - Aplicação dos logotipos e ícones oficiais vetoriais (`open-presenter-app-icon.svg`, `open-presenter-wordmark-dark.svg`, `favicon.svg`) no cabeçalho do painel do operador, na tela de projeção pública (`/public`) e na tela de retorno (`/stage`).


## [0.4.0] - 2026-10-28

### Adicionado

- **Command Palette Global (`⌘K` / `Ctrl+K`):**
  - Acessível via atalho global de teclado, clique na barra de busca superior ou no botão `+` da Setlist.
  - Busca instantânea em tempo real no banco de dados local com **SQLite FTS5** (`unicode61 remove_diacritics 2`) pesquisando simultaneamente por título e letra completa, com geração de snippets contextualizados.
  - Ação rápida para adicionar qualquer música encontrada diretamente à Setlist ativa com o atalho `Shift + Enter` ou botão dedicado, disparando sincronização WebSocket em tempo real.
  - Ações rápidas de criação manual e busca na web integradas na mesma paleta.
- **Busca de Músicas Online (Holyrics e Vagalume):**
  - Módulo `online_lyrics` em Rust com suporte nativo aos provedores Holyrics e Vagalume.
  - Implementação de handshake, token HMAC-SHA512 + MD5 e descompressão gzip (`.txt.gz`) para o CDN do Holyrics.
  - Importação de letras da internet diretamente para o formulário de cadastro com metadados pré-preenchidos (Título, Artista, Tom, BPM e Compositor).
- **Modal de Cadastro e Edição de Músicas:**
  - Interface ampla ocupando 95% da tela (`95vw x 95vh`) com layout dividido em dois painéis.
  - **Painel esquerdo:** formulário de metadados completos e editor de texto da letra com suporte à sintaxe padrão do Holyrics (`## Verso 1`, `## Refrão`, `## Ponte`, quebra de slides por linha em branco) e botões rápidos para inserção de partes.
  - **Painel direito:** visualização em tempo real dos slides gerados como cartões 16:9 (`aspect-video`), com tags de seções coloridas inspiradas no ProPresenter e numeração sequencial.
  - Opção para adicionar à Setlist automaticamente ao salvar.
- **Renomeação da barra lateral para Setlist:**
  - Painel lateral renomeado de "Biblioteca de Mídias" para "Setlist".
  - Botão `+` no cabeçalho da Setlist para abertura rápida da paleta de comandos.
- **Migração do Banco de Dados v3:**
  - Migração `0003_songs_fts.sql` criando as tabelas relacionais `songs`, `song_sections`, `song_lyrics` e a tabela virtual `songs_fts` via SQLite FTS5.
  - Funções de repositório `save_song`, `get_song`, `search_songs` e `delete_song` com suporte a busca fonética/sem acentos e fallback resiliente.

## [0.3.0] - 2026-10-28

### Adicionado

- **Arquitetura Web completa:** transição para backend em Rust + Axum e frontend em Svelte 5 + TypeScript + Vite.
- **Painel do Operador (Admin) via navegador:**
  - Layout de 3 colunas inspirado em ProPresenter e Holyrics.
  - **Sidebar de mídias:** catálogo de itens de mídia (louvores, textos bíblicos, avisos) com busca, filtros por categoria e itens no estilo shadcn-svelte (`MediaItemRow`).
  - **Painel central:** grade de slides da mídia selecionada com proporção 16:9, numeração de estrofes/versos, indicador visual de slide ao vivo e navegação rápida por teclado (`←`, `→`, `Espaço`).
  - **Painel de saídas e preview:** previews do **Public** e do **Stage** com aspect ratio 16:9 estrito, botões de ação para abrir as telas dedicadas em novas janelas do navegador e controles rápidos de projeção (**Ao Vivo**, **Blackout (F1)**, **Limpar Texto (F2)**).
- **Tela de Projeção Pública (`/public`):**
  - Rota dedicada e limpa para projetores e captura em OBS, com proporção 16:9, fundo escuro, tipografia de alto contraste e suporte a tela cheia nativa (`F`).
  - Atualização em tempo real via WebSocket a partir dos disparos do operador.
- **Tela de Retorno de Palco (`/stage`):**
  - Rota dedicada para monitor de retorno com relógio digital em tempo real (HH:MM:SS), data por extenso em português, slide atual destacado e pré-visualização do próximo slide a ser cantado/lido.
- **Backend Rust + Axum (`op-admin`):**
  - Servidor HTTP com rotas REST (`/api/status`, `/api/media`, `/api/presentation/live`, etc.).
  - Servidor WebSocket bidirecional em `/ws` com sincronização inicial e broadcast de eventos em tempo real (`tokio::sync::broadcast`).
  - Estado global em memória (`AppState`) thread-safe.
  - Serviço de entrega de estáticos compilados do frontend (`frontend/dist`) com fallback SPA.
- **Limpeza do projeto:** remoção definitiva dos arquivos de UI desktop legados (Slint, componentes vendorizados e scripts de build).

## [0.2.0] - 2026-10-28

### Adicionado

- Saídas **Public** e **Stage** como janelas independentes, com placeholders de marca; o Stage inclui relógio e data por extenso.
- Janela de **Configurações** independente, com a aba **Saídas**.
- Ativação, escolha de monitor e escolha de modo de exibição por saída, com aplicação imediata.
- Restauração das saídas habilitadas na inicialização, cada uma no monitor salvo.
- Migração v2 com a tabela `output_config` e as operações `load_outputs`/`save_output`.
- Estados degradados para banco indisponível, falha ao salvar e falha ao abrir saídas.

## [0.1.0] - 2026-10-28

### Adicionado

- Workspace Rust com `op-core`, `op-db` e `op-admin`.
- Banco SQLite com PRAGMAs WAL, integridade referencial e migrações numeradas.
- Registro de metadados da aplicação e controle de primeira e última execução em `app_meta`.

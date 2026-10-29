# Changelog

Todas as mudanças relevantes deste projeto são registradas neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e o projeto adere a [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Adicionado

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

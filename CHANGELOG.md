# Changelog

Todas as mudanças relevantes deste projeto são registradas neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e o projeto adere a [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Adicionado
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
- **Interface do Operador:**
  - Removidos os botões duplicados de alternância de sidebars (Setlist e Previews) do cabeçalho superior; o controle de recolher e expandir agora fica centralizado exclusivamente nos botões nativos dos cabeçalhos dos painéis e nos rails laterais colapsados.
  - Removido o subtítulo "Projeção para igrejas e eventos" do cabeçalho superior para deixar a barra de ferramentas mais limpa e compacta.

### Corrigido
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

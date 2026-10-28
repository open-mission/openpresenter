# Spec — Painel Principal Operacional e Command Palette

- **Status:** aprovado
- **Data:** 2026-10-28
- **Versão do app:** 0.3.0
- **Versão do schema:** 2 (inalterado)
- **Substitui:** nada; estende a interface administrativa estabelecida na fundação e no multi-telas

## 1. Intenção

Transformar a janela principal do OpenPresenter de um painel informativo estático em um ambiente de trabalho operacional real para projeção em cultos e eventos. A interface passa a ser composta por uma barra lateral de recursos/mídias (Sidebar), uma coluna de Programação do evento (ordem de culto) e uma coluna de Preview e controle dos slides das mídias.

Além disso, introduzir a **Command Palette** acionada globalmente por `⌘K` / `Ctrl+K`, servindo como a central de busca rápida e comandos do aplicativo (músicas, referências bíblicas e ações de sistema), com estrutura de dados extensível em Rust e Slint para adição incremental de provedores de conteúdo.

Por fim, garantir que a janela principal abra preenchendo todo o espaço disponível da tela (maximizada no monitor principal) e com separação clara das janelas na inicialização, evitando sobreposições com as saídas em tela cheia.

## 2. Problemas e Requisitos

1. **Painel principal meramente informativo**:
   - A tela inicial exibia apenas cartões estáticos sobre o banco SQLite e metadados de sessão.
   - O operador necessita de um fluxo visual de trabalho: selecionar mídias, organizar a ordem do evento e visualizar previamente o que será projetado.

2. **Necessidade de busca ágil e comandos rápidos (Command Palette)**:
   - Em cultos ao vivo, o operador precisa encontrar músicas por nome ou trecho de letra, localizar passagens bíblicas ou acionar comandos sem navegar por menus profundos.
   - O atalho universal `⌘K` (macOS) / `Ctrl+K` (Windows/Linux) deve abrir uma caixa de busca modal sobreposta, com navegação rápida por teclado e arquitetura desacoplada e extensível.

3. **Janela principal não maximizada e conflito de sobreposição na inicialização**:
   - A janela principal abria em tamanho reduzido (960×640) centralizada na tela.
   - Na inicialização, quando as saídas estavam ativas em tela cheia, a janela administrativa abria sobreposta ou disputava foco na mesma tela da saída de palco. A janela administrativa deve abrir maximizada no monitor principal, mantendo as saídas nos seus respectivos monitores.

## 3. Escopo da Solução

### 3.1. Novo Layout da Janela Principal (`MainWindow`)

1. **Barra Superior (Header / Toolbar)**:
   - Identidade visual (ícone e wordmark OpenPresenter).
   - Barra de acionamento rápido da Command Palette (`⌘K / Ctrl+K Busca rápida de músicas, bíblia e comandos`).
   - Badges de estado das saídas (Public e Stage: Aberta, Oculta ou Desativada).
   - Botão de Menu (`☰`) para acesso às Configurações e opções gerais.

2. **Sidebar Lateral (Mídias e Recursos)**:
   - Painel lateral expansível e organizado por categorias de recursos:
     - 🎵 **Músicas**: acervo de louvores.
     - 📖 **Bíblia**: livros, capítulos e versões.
     - 🖼️ **Mídias**: imagens de fundo e vídeos.
     - 📢 **Avisos**: comunicados e minutas da igreja.
   - Lista rápida de itens disponíveis para inclusão na programação.
   - Botão de ação para criar ou importar mídia.

3. **Painel de Programação (Ordem do Evento)**:
   - Título da programação (ex: "Culto de Domingo").
   - Lista sequencial dos itens do evento com tipologia (Louvor, Palavra, Vídeo, Avisos), título e número de telas/slides.
   - Destaque claro do item atualmente selecionado.
   - Controles de reordenação (Adicionar item, Mover para cima/baixo, Remover).

4. **Painel de Preview e Controle de Projeção**:
   - Exibição do item em foco na programação com lista/grade de telas/slides.
   - Miniaturas dos slides com conteúdo tipográfico e fundo.
   - Indicação de slide ativo "Ao Vivo" (projetando no Public) e "Em Espera" (preview selecionado).
   - Barra de controle de saída no rodapé do preview:
     - **Ao Vivo**: envia a mídia selecionada para o público.
     - **Limpar Tela (Black)**: escurece a tela pública imediatamente.
     - **Ocultar Texto**: preserva o fundo removendo apenas o texto.
     - **Logotipo**: projeta o logotipo padrão na tela.

5. **Rodapé Discreto (Status Bar)**:
   - Conexão do banco local, versão do app e indicação das telas ativas.

### 3.2. Command Palette (`Cmd+K` / `Ctrl+K`)

1. **Ativação e Atalhos**:
   - `Cmd+K` no macOS e `Ctrl+K` no Windows/Linux em qualquer lugar da janela administrativa.
   - Clique na barra de busca do header.
   - Tecla `Escape` ou clique no backdrop para fechar.
   - Setas `↑` e `↓` para navegar entre os resultados; `Enter` para executar a ação selecionada.

2. **Interface Visual**:
   - Modal centralizado com fundo escurecido (backdrop).
   - Caixa de busca com ícone de lupa, foco automático imediato e botão para limpar.
   - Filtros de categoria: "Todos", "Músicas", "Bíblia", "Comandos".
   - Lista de resultados categorizados com ícone, título, subtítulo explicativo e atalho/tipo.

3. **Arquitetura Extensível no Rust (`op-admin`)**:
   - Módulo `commands.rs`:
     - Estrutura `CommandItem`: id, título, categoria, descrição, atalho opcional, ação.
     - Provedores de busca (`CommandRegistry`): comandos nativos do sistema (abrir configurações, alternar telas, limpar tela), acervo de músicas e referências bíblicas.
     - Modelo reativo para Slint (`CommandEntry`).

### 3.3. Janela Maximizada e Separação na Inicialização

1. **Abertura Maximizada da Janela Administrativa**:
   - `MainWindow` inicializa com `maximized: true` ou `ui.window().set_maximized(true)`.
2. **Separação de Monitores**:
   - O painel administrativo é fixado no monitor primário (`is_primary: true`).
   - As saídas públicas e de palco são posicionadas em seus monitores dedicados.
   - Caso uma saída esteja configurada para o monitor primário em modo tela cheia, a inicialização coordena a ordem de apresentação para não ocultar a interface operacional do operador.

## 4. Decisões Arquiteturais

1. **Desacoplamento entre visual e fontes de dados**:
   - A interface do Slint expõe modelos de lista para a Programação, Mídias e Command Palette.
   - O backend em Rust fornece dados iniciais estruturados para validação visual, sem exigir ainda tabelas de músicas ou banco de Bíblias (que virão em specs posteriores).
2. **Slint FocusScope e KeyEvents**:
   - O atalho global `⌘K` / `Ctrl+K` é capturado no `FocusScope` raiz da janela principal usando `event.modifiers.meta || event.modifiers.control` e `event.text == "k"`.
3. **Preservação das Configurações**:
   - A janela de Configurações existente e o menu continuam totalmente funcionais e integrados aos novos painéis.

## 5. Critérios de Aceite

1. A janela principal abre limpa e maximizada, sem os cartões estáticos legados, apresentando a estrutura: Header, Sidebar de mídias, Painel de Programação, Painel de Preview e Status bar.
2. Pressionar `⌘K` (macOS) ou `Ctrl+K` (outros sistemas) ou clicar na busca do header abre a Command Palette centralizada.
3. A Command Palette permite filtrar por texto, navegar com as setas do teclado e fechar com `Escape` ou clique fora.
4. Os comandos do sistema na Command Palette (ex: Abrir Configurações) funcionam ao pressionar `Enter` ou clicar.
5. Os painéis de Programação e Preview apresentam itens estruturados e botões de controle de projeção (Ao Vivo, Limpar Tela, Ocultar Texto, Logotipo).
6. O painel administrativo e as saídas não entram em conflito de sobreposição na inicialização.
7. Todos os portões de qualidade (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`) passam com 100% de sucesso.

# Spec — Desacoplamento de Parada da Apresentação do Timer de Palco, Correção de Troca de Versão da Bíblia e Gerenciamento de Pastas com File Explorer em Mídias

## Intenção

Este documento especifica a resolução de três pontos essenciais levantados em operação real de culto no OpenPresenter:
1. **Desacoplamento entre Parada de Apresentação (ESC / Botão Parar) e Cronômetros no Palco (Stage) e Itens de Timer**:
   - Durante a ministração (ex.: pregação com cronômetro rodando no palco), o operador frequentemente projeta versículos ou letras e, em seguida, pressiona a tecla `Escape` (ESC) para interromper a exibição na tela pública.
   - Pressionar ESC ou clicar em "Parar Apresentação" deve apenas cessar a projeção na tela pública (restaurando o papel de parede ou repouso da saída pública).
   - O cronômetro em execução no palco (Stage Display) e no painel do operador **não deve ser pausado, parado nem resetado**.
   - Para Itens de Cronômetro (Timer Items adicionados na setlist ou biblioteca): mesmo que a projeção na tela pública seja finalizada com ESC, o cronômetro deve continuar rodando normalmente no monitor de retorno do palco, até que o operador decida explicitamente pausar ou resetar através dos controles dedicados no painel de controle do timer (`TimerControlPanel`).
2. **Correção na Troca de Versão da Bíblia na Aba da Bíblia**:
   - Na aba da Bíblia, a troca da versão selecionada (via seletor suspenso `<select>`, atalho ou parser de referências) informava a nova sigla, porém o texto dos versículos permanecia exibindo a versão anterior devido a problemas de sincronização no evento de alteração e reutilização indevida de chaves de iteração de versículos no Svelte.
   - A seleção de versão deve carregar e refletir imediatamente o texto correspondente da versão escolhida nos cartões de versículos.
3. **Criação de Pastas e Visualização no Estilo File Explorer nas Abas de Mídias (Imagens e Vídeos)**:
   - Permitir a criação de novas pastas (botão "+ Pasta" / "Nova Pasta") nas abas de Imagens e Vídeos.
   - Quando criada dentro da "Pasta do App" (ou subpastas internas/externas), a pasta física correspondente deve ser criada de fato no sistema de arquivos local (`std::fs::create_dir_all`).
   - Na área principal de visualização de itens, apresentar as pastas como em um navegador de arquivos (File Explorer): cartões de pastas clicáveis com ícone, nome e contador de itens, breadcrumbs no topo para subir/navegar entre níveis e listagem das mídias daquela pasta.

---

## Escopo

- **op-admin (Backend Rust)**:
  - Em `crates/op-admin/src/state.rs`: remover a interrupção/reset forçado do `state.timer` no método `stop_presentation(&self)`.
  - Em `crates/op-admin/src/routes.rs`:
    - Adicionar endpoint `POST /api/media/folders/create-dir` para criar fisicamente diretórios locais na pasta do app ou em pastas mapeadas.
    - Adicionar endpoint `GET /api/media/app-subfolders` para descobrir e listar todas as subpastas físicas presentes em `<data_dir>/media/images` e `<data_dir>/media/videos` (inclusive pastas vazias recém-criadas).
    - Atualizar `POST /api/media/images/upload` e `POST /api/media/videos/upload` para suportar o salvamento de arquivos dentro de subpastas especificadas.
  - Atualizar testes de unidade em `crates/op-admin/src/main.rs`.
- **frontend (Svelte 5)**:
  - Em `frontend/src/lib/views/AdminView.svelte`:
    - Remover chamadas a `pauseTimer()` e `resetTimer()` no método `stopPresentation()`.
  - Em `frontend/src/lib/components/BiblePanel.svelte`:
    - Corrigir o manipulador de troca de versão do `<select>` para capturar o valor atualizado via evento `onchange` e disparar a recarga imediata de livros, capítulos e versículos.
    - Alterar a chave do loop `{#each verses as v (...)}` para `${selectedBibleId}-${v.book_id}-${v.chapter}-${v.verse}`, forçando a re-renderização completa do conteúdo dos cartões ao alternar de versão.
    - Incluir a versão na chave de verificação de `targetPassage` para garantir disparo ao trocar versão pela Command Palette ou referências.
  - Em `frontend/src/lib/media.ts` e `frontend/src/lib/components/BottomPanel.svelte`:
    - Implementar funções de criação de diretório físico e busca de subpastas do app.
    - Adicionar botão "+ Pasta" na barra de ações das abas de Imagens e de Vídeos com modal/prompt para digitação do nome.
    - Exibir pastas no estilo File Explorer diretamente na área de conteúdo de Imagens e Vídeos com navegação fluida por clique e breadcrumbs integrados.
    - Salvar novas mídias importadas na subpasta atualmente aberta no File Explorer.

---

## Decisões Técnicas

1. **Autonomia Completa do Cronômetro Global e do Stage**:
   - O ciclo de vida do timer é independente do ciclo de vida da projeção de slides. A ação de parar a apresentação (`stop_presentation`) limpa apenas `active_media_id`, `active_slide_index`, `quick_media` e áudio, mantendo `timer.running`, `timer.remaining_seconds` e `timer.stage_message` intactos.
   - O timer só altera seu estado de execução por comando explícito de `/api/timer/start`, `/api/timer/pause`, `/api/timer/reset` ou seleção de um novo item de timer.
2. **Criação Segura de Diretórios com Proteção contra Path Traversal**:
   - Nomes de pastas são sanitizados (remoção de caracteres perigosos, proibição de `..`, `/` e `\`), garantindo que diretórios sejam criados estritamente sob as raízes permitidas (`<data_dir>/media/{images,videos}` ou a raiz validada de uma pasta externa).
3. **Chaves Unívocas de Versículos no Svelte**:
   - Ao compor a chave do bloco `{#each}` com `${selectedBibleId}-${v.book_id}-${v.chapter}-${v.verse}`, eliminamos qualquer risco de o motor de reconciliação de DOM do Svelte reutilizar nós existentes que tenham o mesmo `v.id` entre diferentes arquivos SQLite de bíblias.
4. **Navegação Uniforme no Estilo File Explorer em Imagens e Vídeos**:
   - Unificar a experiência de navegação em pastas para Imagens e Vídeos: ao entrar em uma pasta, o usuário vê primeiro os cartões de subpastas e, logo em seguida, os arquivos de mídia contidos naquele nível, com trilha de navegação (breadcrumbs) interativa.

---

## Critérios de Aceite

1. **Timer Preservado ao Usar ESC e Parar Apresentação**:
   - Iniciar um timer de 10 minutos (pelo painel ou projetando um item de timer).
   - Apresentar um versículo ou música na tela pública.
   - Pressionar ESC (inclusive repetidamente): a apresentação pública deve cessar, mas o timer deve continuar contando ininterruptamente no Stage e no painel do operador.
2. **Troca de Versão da Bíblia Imediata e Fiel**:
   - Na aba da Bíblia, navegar até um versículo conhecido (ex.: João 1:1 ou Gênesis 1:1).
   - Alterar a versão no seletor para outra versão instalada (ex.: de ARA para AS ou NTLH).
   - O texto exibido em todos os cartões deve mudar imediatamente para o texto da nova versão, sem exigir recarga da página.
3. **Criação de Pastas e File Explorer em Mídias**:
   - Na aba de Imagens ou Vídeos, clicar no botão "+ Pasta", digitar um nome (ex.: "Cultos Especiais") e confirmar.
   - A pasta física deve ser criada no disco no diretório correspondente da aplicação.
   - A pasta deve aparecer imediatamente na lista e na grade principal como um cartão de pasta.
   - Ao clicar na pasta, o usuário navega para dentro dela, os breadcrumbs atualizam (ex.: `Mídias > Imagens > Pasta do App > Cultos Especiais`), e o usuário pode criar subpastas ou importar mídias diretamente nela.
4. **Qualidade e Testes**:
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run build` devem passar sem erros nem advertências.

# TO-DO



## Coisas pendentes que foi encontrada para melhorar ou corrigir



## Backlog
- [ ] Fix: Eu percebi que quando temos duas instancias conectadas no painel admin, ele se perde um pouco, por exemplo: Se em uma maquina eu estou com a aba da biblia aberta em 1Ts 5.7 (apresentando), se abro outra instancia em outra maquina e for por exemplo 1Ts 1.7 ele fica verde como se estivesse sendo apresentado o versiculo, mas na verdade é 1ts 5.7 acredito que isso aconteça pelo fato do numero de versículo for o mesmo.
- [ ] QUando estamos em versículo por exemplo e uso as setas de navegação left e right do teclado, ele nao está avançando, mas sim voltando ou algo do tipo, em meu teste eu apertava a tecla right e ele ia para trás ao invés de ir para o proximo. Revise isso por favor, isso pode acontecer quando tem mais de aparelho conectado no painel.
- [ ] Feat: Desenvolver sistema de logs para manter salvos as acoes do programa e quem vez isso no sentido de instancia
- [ ] Improve: no componente que mostra "Online" e vemos os clientes conectados, ele fica no Header. Add a opção de visualizar o Navegador ou Aparelho que está conectado.
- [ ] Improve: Add a opcao de reiniciar a aplicacao ou o servidor ao ser atualizado. e add isso no trail menu.
- [ ] Feat: Add suport ao Sonner/toast para avisar quando as cores acontece: Apresentacao é iniciada, parada. Nova Musica Salva. Editada, Novo Aparelho Conectado entre outros.
- [ ] Improve: Quando clicamos na aba de Biblia e ela está fechada, aumentar a altura que ela está abrindo, talvez podemos colocar 75% da altura.
- [ ] Improve/fix: Quando redmensionamos o painel de midias ele parece que tem um delay, veja se dá para usar o nativo do shadnui para ficar mais fluido (https://www.shadcn-svelte.com/docs/components/resizable)
- [ ] Improve: Add a opcao de mostrar no Stage os itens (aviso,  clock e timer) embaixo do Proximo Slide, ficando tres linhas assim podemos aproveitar melhor o espaço. Ajustar para podermos controlar a altura dessas linhas, e as porcentagens serem atualizadas, por exemplo se temos 3 linhas uma preenche 30%, outra 30% entao a ultima só pode preencer até 40%, ou seja as alturas da linha precisa ser somadas até 100% para recalcular as alturas.


## Doing



## Done (com o commit e data e hora de conclusão)

- [x] Lançamento da versão 0.3.1: unificação de versão em todo o workspace (Cargo.toml, package.json, workflows, componentes e testes), consolidação das correções no Lower Third e PWA Remote no iOS, fechamento da release 0.3.1 no CHANGELOG.md, spec docs/specs/2026-10-30-release-0-3-1.md e publicação da tag v0.3.1. (commit 11cf751 - 2026-10-30 18:17:12 -03:00)

- [x] Correções no Lower Third (/lower-third) e suporte a PWA Remote no iOS: resolução resiliente de activeMedia e slides cobrindo músicas da biblioteca/setlist e textos bíblicos com fallback sob demanda; eliminação de tela branca no navegador convencional com introdução de tela de status/boas-vindas para operadores e alternância de simulação de câmera/transparência alfa; classes explícitas is-lower-third no html/body; e criação do manifesto dedicado manifest-remote.webmanifest com start_url: /remote e scope: /remote, alternância dinâmica de manifesto e tags de título do iOS e redirecionamento de PWA mobile standalone da raiz / para /remote. (commit 3fb26f9 - 2026-10-30 18:12:27 -03:00)

- [x] Preparar a versão 0.3.0 para lançamento e ícone oficial no executável: unificação de versão em todo o workspace (Cargo.toml, package.json, workflows, componentes e testes), geração de ícone nativo .icns e PNG 1024x1024 a partir da marca oficial, script de empacotamento macOS (scripts/package-macos.sh) gerando OpenPresenter.app e atribuindo atributos de ícone do Finder ao executável Mach-O, injeção em runtime via Cocoa (Dock/App Switcher), fechamento da release 0.3.0 no CHANGELOG.md e spec docs/specs/2026-10-30-release-0-3-0.md. (commit 4bdf3bb - 2026-10-30 17:29:48 -03:00)

- [x] Correções no Controle Remoto Mobile (/remote): busca bíblica funcional no endpoint /api/bibles/:id/search com suporte a referências rápidas ("Jo 3:16", "Sl 23") via parseBibleReference; divisão estrutural de estrofes de músicas respeitando quebras duplas \n\n com parseRawLyrics; navegação contínua de versículos na Bíblia projetando o capítulo com todos os versículos para habilitar botões Anterior/Próximo e contagem correta de slides; e botão de recarregar catálogo de músicas. (commit e61c7db - 2026-10-30 17:03:29 -03:00)

- [x] Refinamento do Controle Remoto Mobile (/remote) e Suporte a PWA: navegação bíblica com sub-abas (Livros em grade, Capítulos em grade touch de 44px, Versículos com leitura integral), Input Group e Drawer Shadcn para seleção de versão bíblica, correção do carregamento da Setlist (/api/setlists/:id), card de controle ao vivo retrátil (collapsible), active states em músicas e versículos projetados ao vivo, safe-area no iOS e Android (top e bottom na navigation bar), header com logotipo oficial do OpenPresenter Minimal, e suporte completo a PWA (manifest.webmanifest, service worker, ícones 192 e 512, favicons e meta tags de app standalone instalável). (commit 8b4d8e6 - 2026-10-30 16:31:19 -03:00)

- [x] Adicionar saída Lower Third transparente (/lower-third) para OBS Studio e transmissão: HTML com fundo transparente (alfa nativo), texto no terço inferior, gradiente/barra customizável, sombras de alto contraste, animações de transição configuráveis (fade, slide, zoom), aba de configurações completa com prévia interativa 16:9, atalho no tray e instruções de Browser Source. (commit fc6a647 - 2026-10-30 15:57:41 -03:00)

- [x] Implementação do Controle Remoto Mobile (/remote) permitindo apresentar setlist, músicas e Bíblia pelo celular: interface mobile-first responsiva com bottom navigation bar em 3 abas, card persistente de controle ao vivo com botões ergonômicos de anterior/próximo para o polegar, ações rápidas de blackout/limpar/parar, busca instantânea, sincronização WebSocket em tempo real, rota no system tray e aba com QR Code no modal de compartilhamento. (commit 52d45eb - 2026-10-30 15:39:03 -03:00)

- [x] Paridade 100% no preview do operador com canvas virtual HD 1080p escalado (idêntico à tela pública real), personalização de posicionamento da referência bíblica (rodapé vs. abaixo do texto) com tamanho de fonte customizável, opções de encaixe do papel de parede (contain, cover, fill) com proteção de margens seguras contra overscan, alinhamento dos switches Tailwind UI e filtro de privacidade na galeria de imagens para proteger arquivos pessoais. (commit 95ee5ca - 2026-10-30 15:01:21 -03:00)

- [x] Separação de configurações dedicadas para letras de louvor e textos bíblicos na Projeção Pública e Monitor de Palco (sub-seções no modal de configurações, alinhamentos, tamanhos de fonte e preenchimento de tela independentes, cores dedicadas no stage) e correção do contraste dos interruptores (switches) de configurações. (commit 0da4508 - 2026-10-30 13:49:51 -03:00)

- [x] Correção de colapso de viewport e auto-scale em receptores Google Cast / Chromecast e Smart TVs HDMI, assegurando dimensões 100% na raiz do DOM e distribuição vertical correta. (commit 5fd74a6 - 2026-10-30 12:59:24 -03:00)

- [x] Integração completa com Google Cast (Chromecast): transmissão sem fio com latência quase zero (&lt;30ms) via Web Receiver Chromium e WebSocket local para Telas Pública (/public) e Monitor de Palco (/stage), modal dedicado de Cast e atalho com status no header do painel. (commit 526a3dc - 2026-10-30 04:39:25 -03:00)

- [x] Especificação técnica, plano de execução e Registro de Decisão Arquitetural (ADR 0013) para suporte a Google Cast com Web Receiver em relação a streams de vídeo HLS. (commit b54a64a - 2026-10-30 04:34:44 -03:00)

- [x] Preparar a versão 0.2.0 para lançamento: unificação de versão (Cargo.toml, package.json, workflows), spec de release e atualização do CHANGELOG.md (2026-10-30 04:25:00 -03:00)

- [x] Auditoria técnica Impeccable no painel do operador: correção de acessibilidade WAI-ARIA, suporte a prefers-reduced-motion, padronização de design tokens semânticos e atualização da skill para v4.8.0. (commit 26dd358 - 2026-10-30 04:13:07 -03:00)

- [x] o icone do trail está estranho ![](orca-paste-1793330003849-ead85ae4-f46f-4d15-aefe-326d48b20352.png) (commit 2cdd5ff - 2026-10-30 02:35:37 -03:00)

- [x] Design Feedback /: Remover badge "X itens" e botão de apagar do cabeçalho da Setlist; exibir dropdown flutuante de conexões WebSocket ativas com IP e rota ao clicar em "Online". (commit 7d3e4fa - 2026-10-30 02:10:22 -03:00)

- [x] Add no command pallete global do app a opcao de pesquisar textos biblico por referencia ou palavras igual acontece no painel da biblias. (commit 6f2498a - 2026-10-30 01:53:27 -03:00)

- [x] Design Feedback /: div "Cronômetro &amp; Palco" - Add a opcao de dar um titulo para o cronometro que irá aparecer no stage. (commit 9044ed4 - 2026-10-30 01:41:35 -03:00)

- [x] Design Feedback /: div "Cronômetro: Cronômetro (Pausado)" no header - Isso aqui só deve aparecer se o timer estiver rodando. (commit 09003ca - 2026-10-30 01:36:35 -03:00)

- [x] Add a opcao de importar fundos de imagens e videos na aba de midias &gt; fundos. (commit a97efc5 - 2026-10-30 01:32:56 -03:00)

- [x] Nos fundos de letras precisamos implementar um sistema que tenhamos um css nas configuracoes para ajustar tanto o fundo como a cor de texto para cada fundo. Isso pode ser util se quisermos entrarmos novos fundos futuramente. (commit d7cb27f - 2026-10-30 01:24:16 -03:00)

- [x] Design Feedback /public: div "📖 João 3:1 ARA ARA" - mover para o rodapé em space-between, remover duplicidade de ARA, remover emoji e tipografia amarela (commit fea7e6a - 2026-10-30 01:05:46 -03:00)

- [x] Quando na aba de musicas o sheet de preview está aberto.  o item de musica fica espremido. ![](orca-paste-1793329505020-6cf2d68b-36b1-487c-92ad-aeaff9836e82.png) (commit dde80a9 - 2026-10-30 01:00:21 -03:00)

- [x] Na aba de imagens ele nao aceita a sub-pastas. Se eu importo uma pasta ele nao mostra as subpastas e os items. (commit b0ee635 - 2026-10-30 00:56:03 -03:00)

- [x] Na aba de Biblias quando clico duas vezes em um versiculo ele está add no setlist o capitulo. PRecisamos corrigir isso. A ideia é que se eu clicar duas vezes ele apresenta mais nao add no setlist. Se eu clicar com o botao direito mostrar o contexto menu para adicionar no setlist. (commit 852b76d - 2026-10-30 00:44:00 -03:00)

- [x] No setlist Bar quando um item está ao vivo ele tem um indicativo que fica piscando. Vamos remover isso e add a borda verde para quando estamos apresentando um item. (commit ba84d09 - 2026-10-30 00:32:55 -03:00)

- [x] Na Tela Public, add o Auto Scale nas fonts, add a opcao de font-size minimo algo proximo a font-size: 8rem, mas podendo ser ajustado nas configuracoes. Quando a quebra de linha está desatvado está add ... mas a ideia é que reduza o tamanho da fonte para caber. (commit c510264 - 2026-10-30 00:30:14 -03:00)

- [x] Ajustar O stage: Vamos simplificar. A principio vamos manter apenas o preset Grid Moderno Pro. Ajustar o texto para preenhcer o texto fazendo auto-scale para sempre caber no box. Add a opcao de Font-size minimo nas configuracoes. Para ganharmos espaco podemos remover os gaps das boxes e tirar o border-radius. Fazer o Boz de Messagem sempre visivel no Stage. (commit dd36342 - 2026-10-30 00:26:18 -03:00)

- [x] Refinar visual do modo Standby no Stage: cronômetro aparece somente se estiver rodando, remoção do badge superior e do rodapé (commit 7c71974 - 2026-10-29 23:46:05 -03:00)

- [x] Quando eu fecho a aba do painel ele fica abrindo de novo, normalmente se eu já consigo acessar pelo trail, ele nao precisa ficar reabrindo toda hora, abrir automaticamente somente quando abrir a aplicacao. O icone do trail está fundo branco. Ajuste para a versao do icone preto. (commit 6d89743 - 2026-10-29 23:38:48 -03:00)

- [x] Ao acessar [http://localhost:18080/stage](http://localhost:18080/stage) a tela está preta e nao mostra nada, mesmo que no preview pareça mostrar (commit afa1e3c - 2026-10-29 23:14:46 -03:00)

- [x] Ajustar modo compacto de botões (somente ícones) na barra de navegação dos slides e cabeçalho, com suporte a toggle nas configurações de aparência (prints orca-paste) (commit a4f9bbd - 2026-10-29 22:52:19 -03:00)

- [x] Add a descanso mode quando nao tem nada apresentando mostrar o relogio e/ou timer na tela stage. Ele normalmente aparece quando o papel de parede do setlist é ativado. (commit 9a8dc49 - 2026-10-29 22:19:02 -03:00)
- [x] No header do Painel temos o botao para fechar o sidebars porem podemos remove-los pois temos eles nos paines abaixo. Entao fica duplicado. (commit 4af462c - 2026-10-29 22:15:30 -03:00)
- [x] Nas configuracoes temoa a opcao de margem nas definicoes da tela public, porem me parece que nao está funcionando em si, pois nao está aplicando a margem no publico. (commit 02fee4d - 2026-10-29 22:12:23 -03:00)
- [x] Toda vez que uma configuracao for aplicada nao deveria atualizar as telas public e stage? Pois toda vez que tenho que recarregar manualmente, afinal nao temos a opca de recarregar as telas no painel. (commit 18d92fc - 2026-10-29 22:08:52 -03:00)
- [x] Nas configuracoes em Marca da Igreja, temos a opcao de add o logo da igreja, precisamos add a opcao de margem para ajustar melhor a posicao (commit 4c9b712 - 2026-10-29 22:02:13 -03:00)
- [x] No trail criado na aplicacao na maquina nao tem o icone do programa e nem no executavel, ajustar. (commit a75121c - 2026-10-29 21:58:43 -03:00)
- [x] Na secão de previews dos slides pode usar icones apenas para ganhar espço, entao. mas add opcao na configuracoes para exibir labels no botao pode add na aba de aparencia. (commit 6e1a1f6 - 2026-10-29 21:54:23 -03:00)
- [x] No Header remover o texto "Projeção para igrejas e eventos" (commit 80e20ef - 2026-10-29 21:49:56 -03:00)
- [x] Fix: Quando selecionamos o plano de fundo no setlist ele deve ser usado quando nao tem nenhuma apresentacao sendo apresentada, mais quando iniciamos uma apresentacao deve ser substituito o fundo para o padrao selecionado. Entao temos o papel de parede do setlist que normalmente é o tema do culto ou nome do culto, e o background fundo que é apresentado no fundo das letras. E quando cancelamos a apresentacao ele deve voltar ao plano de fundo do setlist. (commit 7c2bc21 - 2026-10-29 21:48:48 -03:00)

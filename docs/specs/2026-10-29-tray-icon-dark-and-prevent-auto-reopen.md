# Spec: Ícone Escuro no System Tray e Controle de Reabertura do Navegador

## Intenção
Ajustar o ícone da bandeja do sistema (System Tray) para utilizar a versão com fundo preto e traçado branco da marca oficial (`open-presenter-icon-dark`), integrando-se perfeitamente com barras de tarefas e menu bars escuras, e garantir que a aba do painel no navegador seja aberta automaticamente apenas na inicialização da aplicação, sem ficar reabrindo indesejadamente em eventos do mouse na bandeja.

## Contexto e Decisões
1. **Comportamento do Navegador:**
   - Na inicialização (`Event::NewEvents(StartCause::Init)`), a aplicação abre o navegador uma única vez na URL do servidor (`http://localhost:PORT`).
   - Anteriormente, o manipulador de `Event::UserEvent(UserEvent::Tray(_tray_event))` acionava `open::that` em qualquer evento de interação com o ícone da bandeja (incluindo passagem de mouse e abertura/fechamento do menu nativo), causando reabertura constante da aba do navegador.
   - O disparador indiscriminado em `UserEvent::Tray` foi neutralizado. A reabertura sob demanda do painel permanece disponível a qualquer momento pelo menu nativo do tray ("Abrir Painel de Controle").
2. **Ícone do Tray:**
   - O arquivo `assets/branding/open-presenter-icon-dark-32.png` foi gerado a partir do SVG oficial `open-presenter-icon-dark.svg` (fundo preto `#111318` com traçado e play brancos `#FFFFFF`).
   - O `tray.rs` agora carrega `open-presenter-icon-dark-32.png`, eliminando o bloco branco no tray.
   - O `assets/branding/README.md` foi atualizado conforme as diretrizes de governança de marca.

## Critérios de Aceite
1. Ao iniciar a aplicação, a aba do painel abre normalmente no navegador padrão.
2. Ao fechar a aba do painel, passar o mouse ou interagir com o ícone do System Tray não reabre o navegador sozinho.
3. Ao clicar na opção "Abrir Painel de Controle" no menu do tray, a URL do painel abre no navegador sob demanda.
4. O ícone exibido no System Tray utiliza o asset com fundo preto e traçado branco.
5. Todos os testes unitários do workspace passam sem regressão.

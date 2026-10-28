# ADR 0003 — Saídas Public e Stage em janelas independentes

- **Status:** aceito
- **Data:** 2026-10-28

## Contexto

A v0.1.0 entregou um único painel administrativo, o banco SQLite local e a identidade visual do OpenPresenter. A v0.2.0 precisa abrir duas saídas independentes — **Public**, visível ao público, e **Stage**, retorno para o palco e o apresentador — cada uma em um monitor e modo configuráveis, restauradas na inicialização.

O cenário principal é um computador com monitor externo: o painel fica na tela do computador e o Public no monitor externo. O Stage é opcional. A aplicação precisa continuar funcionando com um único monitor, e falhas de banco, de enumeração de monitores ou de criação de janela não podem impedir o painel de abrir. O Slint 1.18 não expõe enumeração de monitores, e a janela de Configurações deve poder ficar aberta enquanto as saídas são ajustadas, inclusive em outro monitor.

## Decisão

- Abrir Public e Stage como **janelas Slint independentes**, mantidas por handles fortes no controlador, com um único loop de eventos.
- Enumerar monitores com a crate **`display-info` 0.5**, usada apenas em `op-admin`, e normalizar as coordenadas para **pixels físicos** antes de chamar a API do Slint. Tamanhos de janela e cascata permanecem lógicos e são convertidos pela escala do monitor de destino.
- Persistir a configuração no SQLite com o **schema v2**, que adiciona a tabela `output_config` sem alterar `app_meta`. A ausência de linha significa “usar o padrão da saída”.
- Oferecer uma **janela de Configurações independente**, aberta pelo menu interno **☰ → Configurações…**, com a aba **Saídas**. Ela pode ser fechada sem afetar as saídas.
- **Public ativo por padrão** (monitor `Auto`, modo `Auto`); **Stage opcional e desativado por padrão** (monitor `Primary`, modo `Windowed`).
- **Fechar uma saída pelo X apenas a oculta** nesta sessão; `enabled` continua verdadeiro e a saída volta na próxima inicialização. **Desativar** fecha a janela e persiste `enabled = false`.
- Restaurar as saídas habilitadas na inicialização, no monitor salvo. Se o monitor não estiver disponível, usar o principal com aviso, sem apagar a preferência.

## Alternativas consideradas

- **Modal de configuração sobre o painel:** menos código de ciclo de vida, mas cobre o painel, não pode ser movido para outro monitor e fica pesado quando novas abas forem adicionadas.
- **Arquivo JSON em vez de SQLite:** simples, mas cria uma segunda fonte de verdade e exige tratamento próprio de corrupção, escaping e concorrência.
- **`winit` diretamente:** o Slint já usa winit internamente; criar um segundo event loop no mesmo processo é frágil e conflita com o loop do Slint.
- **Uma única janela com duas superfícies:** não permite colocar Public e Stage em monitores diferentes nem manter o painel administrativo visível ao mesmo tempo.

## Consequências

- O aplicativo passa a rodar **um único loop de eventos com múltiplas janelas**; o ciclo de vida precisa manter handles fortes e distinguir ocultar de fechar. Fechar o painel encerra o loop e todas as saídas.
- As **coordenadas variam por plataforma**: macOS e Linux (X11 e Wayland) informam valores lógicos e o Windows informa físicos. A normalização fica isolada em funções puras e testáveis; no Wayland o compositor pode ignorar a posição solicitada.
- A **identidade do monitor é best-effort**. O `id` nativo é a identidade primária e o nome salvo é o fallback; a estabilidade varia por plataforma e o handle do Windows pode mudar com porta ou configuração. A preferência original é preservada.
- **Não há hot-plug** nesta versão: a lista de monitores é atualizada na inicialização e ao abrir as Configurações.
- As saídas exibem **placeholders de marca** até existir um modelo de apresentação; não há conteúdo real, controle de projeção, NDI/OBS ou atalhos globais.
- Falha de banco impede a abertura das saídas e desabilita os controles; falha de gravação mostra uma faixa de erro separada e mantém a mudança em memória.
- Uma dependência adicional, `display-info`, passa a integrar o build de `op-admin` em todas as plataformas suportadas.

# Especificação: Limpeza da Barra Lateral da Setlist e Dropdown de Conexões WebSocket

## Contexto e Motivação
No painel do operador (`/`), a barra lateral da Setlist exibia um badge redundante informando a quantidade de itens (ex: "3 itens") e um botão rápido com ícone de borracha para limpar toda a setlist ativa. Esse botão gerava risco de perda acidental da ordem de culto montada, enquanto a contagem de itens poluía visualmente o cabeçalho.
Além disso, o badge "Online" no topo da interface informava apenas o estado de conexão binário do WebSocket local, sem permitir ao operador diagnosticar quais clientes (projeção pública, retorno de palco, outros operadores) estavam conectados, em quais IPs e em quais rotas.

## Escopo
1. **Limpeza da Barra Lateral da Setlist (`AdminView.svelte`):**
   - Remover o badge com a contagem de itens (`span.inline-flex.items-center` / `Badge {$mediaItemsList.length} itens`).
   - Remover o botão com ícone de borracha (`Eraser`) que limpava todos os itens da setlist ativa diretamente no cabeçalho.
2. **Rastreamento de Conexões WebSocket no Backend:**
   - Adicionar modelo `ClientConnection` em `op-core` contendo `id`, `ip`, `route`, `client_type` e `connected_at`.
   - Adicionar mensagem `WsMessage::ConnectionsUpdated { connections }` e campo opcional `route` em `WsMessage::Identify`.
   - Gerenciar conexões ativas no `AppState` (`Arc<RwLock<Vec<ClientConnection>>>`).
   - Extrair IP do cliente via `ConnectInfo<SocketAddr>` e headers (`x-forwarded-for`, `x-real-ip`).
   - Criar endpoint REST `GET /api/connections` para consulta imediata das sessões conectadas.
   - Atualizar a lista e transmitir broadcast em tempo real sempre que um cliente se conecta, identifica ou desconecta.
3. **Dropdown de Conexões no Badge Online do Cabeçalho (`AdminView.svelte`):**
   - Transformar o indicador "Online" em um botão interativo com contador discreto de conexões ativas.
   - Ao clicar, exibir dropdown flutuante moderno exibindo a lista de conexões ativas com ícone da rota, nome descritivo (ex: "Painel Operador", "Projetor Público", "Retorno de Palco"), endereço IP e horário de conexão.

## Decisões Técnicas
- A extração de IP no Axum utiliza `Option<ConnectInfo<SocketAddr>>` e verificação de cabeçalhos de proxy reverso (`X-Forwarded-For`), evitando falhas em testes unitários.
- A rota é enviada pelo frontend na mensagem de identificação (`identify`) e deduzida no handshake.
- O dropdown pode ser fechado clicando no badge, clicando fora (backdrop) ou pressionando a tecla `Esc`.

## Critérios de Aceite
- O cabeçalho da barra de setlist não exibe o badge "X itens" nem o botão de apagar (`Eraser`).
- Clicar no badge "Online" no cabeçalho abre um dropdown com a lista de clientes WebSocket conectados, seus IPs e respectivas rotas.
- Testes e portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

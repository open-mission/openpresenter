# Spec — Controle Remoto Mobile (/remote)

- **Status:** aprovada
- **Data:** 2026-10-30
- **Autor:** OpenPresenter Core Team
- **Contexto:** Item 1 do backlog no TODO.md

## 1. Intenção

Disponibilizar uma interface web dedicada para smartphones (`/remote`) otimizada para toque (mobile-first), permitindo que operadores, pregadores e líderes de louvor controlem as apresentações (Setlist, Músicas e Bíblias) remotamente via rede local Wi-Fi sem precisar operar diretamente o computador do projetor.

## 2. Escopo

- Nova rota SPA `/remote` em `frontend/src/App.svelte` e visualização `frontend/src/lib/views/RemoteView.svelte`.
- Roteamento e identificação WebSocket de cliente `remote` em `crates/op-admin/src/ws.rs` e `frontend/src/lib/ws.ts`.
- Exposição da URL remota e QR Code em `NetworkInfoResponse` (`/api/system/network-info`) e no modal de compartilhamento (`ShareOutputsModal.svelte`).
- Adição da opção de abrir Controle Remoto no System Tray (`crates/op-admin/src/tray.rs`) e no banner inicial de inicialização (`crates/op-admin/src/main.rs`).
- Interface responsiva com 3 abas principais:
  1. **Setlist:** Visualização de itens da setlist ativa, seleção de slides e avanço.
  2. **Músicas:** Catálogo, busca instantânea e projeção direta de letras por estrofe.
  3. **Bíblia:** Seletor de versões, livros, capítulos, versículos, busca por referência e projeção rápida ao vivo.
- Barra superior de status ao vivo com botões de emergência (**Blackout**, **Limpar Texto**, **Parar**) e controles ergonômicos de **Próximo** e **Anterior**.

## 3. Decisões

1. **Mobile-First & Touch Ergonomics (Design Impeccable Shape):**
   - Altura de toque mínima recomendada de 48px para botões primários.
   - Navegação inferior fixa (Bottom Navigation Bar) com 3 abas bem espaçadas e ícones representativos (`ListMusic` para Setlist, `Music` para Músicas, `BookOpen` para Bíblia).
   - Botão de avanço rápido ("Próximo Slide") posicionado de forma prioritária e acessível com o polegar.
2. **Sincronização Bidirecional em Tempo Real:**
   - O controle remoto conecta ao WebSocket `/ws` como `client_type: "remote"` e `route: "/remote"`.
   - Mudanças feitas no computador refletem instantaneamente no celular; comandos acionados no celular atualizam a projeção em menos de 15ms.
3. **Desacoplamento e Independência:**
   - O celular não armazena dados em banco local; consome diretamente os endpoints REST e WebSocket do servidor Rust em execução na máquina de projeção.
4. **Compartilhamento Descomplicado via QR Code:**
   - Adicionada aba "Controle Remoto" no modal de saídas de rede (`ShareOutputsModal.svelte`), gerando QR Code direto com IP da rede Wi-Fi da igreja para leitura instantânea com a câmera do smartphone.

## 4. Critérios de Aceite

- [x] Acessar `http://<ip-do-servidor>:18080/remote` carrega a interface mobile responsiva do Controle Remoto.
- [x] Aba **Setlist** lista os itens da ordem de culto ativa e permite alternar slides ao vivo com um toque.
- [x] Aba **Músicas** permite buscar músicas por título/letra e projetá-las diretamente.
- [x] Aba **Bíblia** permite navegar por livro/capítulo/versículo ou buscar referência e projetar ao vivo.
- [x] Botões de **Próximo**, **Anterior**, **Blackout**, **Limpar** e **Parar** funcionam perfeitamente em tempo real.
- [x] Conexão do celular aparece no dropdown de conexões WebSocket com rota `/remote` e IP do dispositivo.
- [x] Modal de compartilhamento exibe aba com QR Code para o Controle Remoto.
- [x] Menu do System Tray inclui atalho "Abrir Controle Remoto (Mobile)".

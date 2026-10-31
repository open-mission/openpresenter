# Spec: Simplificação e Agrupamento das Saídas no Cabeçalho

## Status
Aprovado / Em implementação.

## Contexto e Motivação
No cabeçalho operacional (`AdminView.svelte`), o lado direito continha uma sequência de 7 botões distintos:
1. `Online [N]` (Pill com status WebSocket e dropdown de conexões)
2. `Configurações` (Botão com engrenagem)
3. `Iniciar Public` (Botão com abertura de janela pop-up)
4. `Iniciar Stage` (Botão com abertura de janela pop-up)
5. `Cast` (Botão abrindo modal de saídas na aba Cast)
6. `QR Code` (Botão abrindo modal de saídas na aba Palco)
7. `Remoto` (Botão abrindo modal de saídas na aba Remoto)

Essa proliferação de botões causava:
- **Poluição visual e sobrecarga cognitiva:** A barra superior ficava densa com múltiplos ícones e textos truncados ou ocultados em resoluções menores (`hidden md:inline`, `hidden lg:inline`).
- **Redundância:** Três botões (`Cast`, `QR Code`, `Remoto`) acionavam o mesmo modal (`ShareOutputsModal`) apenas alternando a aba inicial.
- **Falta de escalabilidade:** A adição de novas saídas (como `/lower-third`) continuaria inflando o cabeçalho.

## Decisão de Design (Opção 1 Selecionada)
Criar uma **Central de Saídas Única** acessível através de um menu dropdown elegante **"Saídas"** (com ícone `MonitorPlay` e `ChevronDown`).
O lado direito do cabeçalho passa a contar estritamente com:
1. `Online [N]` (Pill de status e auditoria)
2. `[ 🖥️ Saídas ▾ ]` (Menu Dropdown "Saídas")
3. `[ ⚙️ Configurações ]` (Botão de configurações com engrenagem e badge de atualização se disponível)

## Escopo do Menu Dropdown "Saídas"
O dropdown apresenta os itens organizados em duas seções semânticas:
1. **Telas de Projeção (Janelas do Navegador):**
   - 🖥️ **Tela Pública (Projetor)** (`/public`) — abre a janela de projeção principal (`openPublicWindow()`).
   - 📺 **Monitor de Palco (Stage)** (`/stage`) — abre a janela de retorno para músicos e pregadores (`openStageWindow()`).
   - 🎥 **Lower Third (OBS Studio)** (`/lower-third`) — abre a janela com transparência alfa para software de transmissão (`openLowerThirdWindow()`).
2. **Conexões Sem Fio & Compartilhamento:**
   - 📡 **Transmitir via Google Cast** — abre o `ShareOutputsModal` na aba `cast` (com destaque verde e pulso se o Cast já estiver conectado).
   - 📱 **Controle Remoto Mobile** — abre o `ShareOutputsModal` na aba `remote` com QR Code para celular.
   - 🔗 **Compartilhar Saídas na Rede** — abre o `ShareOutputsModal` na aba `stage` com todos os links e QR Codes da rede local.

## Critérios de Aceite
1. O cabeçalho exibe apenas `Online`, `Saídas` e `Configurações` no lado direito, reduzindo drasticamente o número de botões.
2. O botão `Saídas` exibe indicador visual verde se o Google Cast estiver com sessão ativa (`$castStore.isConnected`).
3. Ao clicar em `Saídas`, abre-se o dropdown com as opções divididas em seções ("Telas Locais" e "Transmissão & Conexões"), com títulos claros, ícones e descrições.
4. Clicar em qualquer opção fecha o dropdown e executa a respectiva ação (abrir janela ou abrir o modal na aba correspondente).
5. O dropdown fecha ao pressionar a tecla `Escape` ou ao clicar fora (backdrop invisível).
6. A suite de testes (`cargo fmt`, `cargo clippy`, `cargo test` e `bun run build`) permanece 100% verde.

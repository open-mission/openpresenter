# Especificação — Remote iOS Safe-Area & Divisores, Fade no Lower Third e Limpeza de Emojis

**Data:** 2026-11-01  
**Status:** Aprovado  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Contexto e Motivação

Com as recentes melhorias de experiência visual no OpenPresenter, foram identificados pontos de polimento tanto no controle remoto móvel (`/remote`) quanto na saída de transmissão Lower Third e na consistência visual de toda a aplicação:
1. **Controle Remoto Móvel (`/remote`):**
   - O cabeçalho continha o texto por extenso "OpenPresenter", ocupando espaço valioso em telas estreitas de smartphones.
   - Dispositivos iOS (iPhone com Dynamic Island, entalhe/notch ou bordas arredondadas) apresentavam corte superior devido a safe-area insuficiente.
   - Na lista da setlist ativa, divisores de seção (`kind: 'header'`) eram exibidos como cartões apresentáveis comuns com badge numérico e "0 slides", em vez de atuarem como separadores visuais como ocorre no painel do operador.
2. **Saída Lower Third (OBS / Transmissão):**
   - O aparecimento e encerramento da projeção no Lower Third ocorriam instantaneamente sem desvanecimento suave do box e do conteúdo.
3. **Identidade Visual e Consistência (Remoção de Emojis):**
   - Vários elementos de UI continham emojis (🎵, 📖, 📅, 🖼️, 🚫, ⚠️) em botões, abas e identificadores de mídia, gerando inconsistência visual com a identidade minimalista e profissional do OpenPresenter.
4. **Finalização dos Ajustes das Configurações (`SettingsModal.svelte`):**
   - Incorporar controles de fade de imagens e papel de parede na Tela Pública, inputs numéricos acoplados na Agenda do Palco, switches de segundos e data no relógio do palco e garantir navegação fluida em "Layout & Tempo".

---

## 2. Escopo

### No Escopo
- **/remote:**
  - Remoção do texto do logo "OpenPresenter" no topo, mantendo o ícone oficial da marca e o badge de status (Remoto / Offline).
  - Safe-area no iOS: `padding-top: max(calc(env(safe-area-inset-top, 0px) + 8px), 14px)`.
  - Tratamento de itens de seção (`kind === 'header'`): renderização como divisores horizontais não clicáveis, com ícone `Bookmark`, alinhados ao estilo de `MediaItemRow.svelte`.
  - Reindexação numérica da lista para numerar apenas itens apresentáveis reais.
- **Lower Third (`LowerThirdView.svelte`):**
  - Aplicação de transições `transition:fade` nativas do Svelte no container do Lower Third para transição suave de entrada e saída ao iniciar, encerrar, limpar ou aplicar blackout.
  - Transição de chaveamento de slide suave.
- **Remoção de Emojis:**
  - Substituição de emojis por ícones SVG do Lucide ou texto limpo em `LowerThirdView.svelte`, `SettingsModal.svelte`, `BottomPanel.svelte`, `MediaItemRow.svelte`, `AdminView.svelte`, `StageView.svelte` e `RemoteView.svelte`.
- **Configurações (`SettingsModal.svelte`):**
  - Adição de configuração de transição de imagens com fade suave (liga/desliga e duração em ms).
  - Inputs numéricos acoplados para tamanhos de fonte na Agenda do Palco.
  - Opções para ocultar segundos e data no relógio do monitor de palco.
  - Garantia de acesso e usabilidade na sub-aba "Layout & Tempo" do Palco com container alargado `max-w-4xl`.

---

## 3. Critérios de Aceite

1. No `/remote`, o cabeçalho mostra apenas o ícone SVG e o badge de conexão, sem cortar em iPhones nem sobrepor a Dynamic Island.
2. Na setlist do `/remote`, divisores de seção aparecem como linhas separadoras com título em caixa alta, sem números ou "0 slides".
3. No `/lower-third`, o box e textos surgem e somem com fade suave ao vivo, sem cortes abruptos.
4. As menções a "Louvor", "Bíblia", "Agenda", "Tela Cheia", etc., utilizam ícones do Lucide ou texto limpo, sem emojis.
5. Em `SettingsModal.svelte`, todas as abas e sub-abas (incluindo "Layout & Tempo") são acessíveis, com sliders e inputs numéricos sincronizados.
6. Todos os portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test`, `bun run build`) passam sem erros ou warnings.

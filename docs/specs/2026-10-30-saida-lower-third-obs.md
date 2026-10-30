# Spec — Saída Lower Third Transparente para OBS e Transmissão (/lower-third)

- **Status:** aprovada
- **Data:** 2026-10-30
- **Autor:** OpenPresenter Core Team
- **Contexto:** Item 2 do backlog no TODO.md

## 1. Intenção

Disponibilizar uma saída visual dedicada (`/lower-third`) com fundo 100% transparente para ser capturada como Browser Source (Navegador) no OBS Studio, vMix ou outros softwares de streaming, projetando letras de louvor e versículos bíblicos no terço inferior da tela sobreposta à câmera ao vivo, com gradiente suave central, sombra para alto contraste e animações fluidas de transição.

## 2. Escopo

- Nova rota SPA `/lower-third` em `frontend/src/App.svelte` e visualização dedicada `frontend/src/lib/views/LowerThirdView.svelte`.
- Fundo transparente no DOM raiz para captura com canal alfa pelo OBS Studio.
- Roteamento e identificação de cliente WebSocket `lower-third` em `crates/op-admin/src/ws.rs`.
- Adição da URL em `NetworkInfoResponse` (`crates/op-admin/src/routes.rs` e `frontend/src/lib/types.ts`).
- Nova aba no modal de configurações (`SettingsModal.svelte`): "Lower Third (OBS)", permitindo customizar:
  - Estilo visual (Gradiente central, Barra inferior, Caixa flutuante ou Minimalista).
  - Vínculo opcional da cor de gradiente ao fundo selecionado no público ou paleta customizada.
  - Opacidade de fundo, tamanho de fonte, peso da tipografia e alinhamento do texto.
  - Força da sombra do texto (nenhuma, suave, média, forte) para contraste contra qualquer iluminação de câmera.
  - Animação de transição (fade suave, deslize vertical, zoom).
  - Prévia em tempo real 16:9 sobre vídeo simulado.
- Adição de aba informativa "Lower Third" no modal de compartilhamento (`ShareOutputsModal.svelte`) com instruções detalhadas para o OBS Studio.
- Atalho "Abrir Lower Third (OBS / Transmissão)" no System Tray e exibição no banner do terminal.

## 3. Decisões

1. **Transparência Alfa Nativa:**
   - Na rota `/lower-third`, `app.css` e o contêiner raiz desativam a cor de fundo padrão (`background: transparent !important`), permitindo integração perfeita com navegadores OBS sem necessidade de Chroma Key.
2. **Gradiente Central e Contraste Adaptativo:**
   - Um gradiente translúcido central (`radial-gradient` ou `linear-gradient`) proporciona contraste suave diretamente sob o texto sem bloquear o enquadramento do pastor ou ministro de louvor.
   - Sombra projetada (`drop-shadow` / `text-shadow`) multicamada para garantir leitura nítida mesmo sobre roupas brancas ou fundos claros iluminados.
3. **Animações de Entrada e Saída:**
   - Transições de troca de slides e modos (Live, Blackout, ClearText) utilizam animações CSS aceleradas por GPU (`transition: opacity 250ms cubic-bezier(0.16, 1, 0.3, 1), transform 250ms`).
4. **Sincronização em Tempo Real:**
   - Toda alteração nas configurações de Lower Third é sincronizada instantaneamente via WebSocket (`setting_updated`), atualizando o layout no OBS ao vivo sem descontinuidade na transmissão.

## 4. Critérios de Aceite

- [x] Acessar `http://localhost:18080/lower-third` exibe os slides ao vivo posicionados no terço inferior com fundo transparente.
- [x] A cor de fundo ou gradiente pode ser personalizada ou sincronizada com o tema ativo.
- [x] Opções de tamanho de fonte, força de sombra e animação funcionam e refletem em tempo real.
- [x] Transições entre slides respeitam animação configurada (fade / slide).
- [x] Modal de configurações possui aba dedicada "Lower Third (OBS)" com prévia em tempo real.
- [x] Modal de compartilhamento e System Tray possuem atalhos e instruções claras para o OBS Studio.

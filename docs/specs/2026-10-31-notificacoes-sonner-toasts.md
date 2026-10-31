# Spec: Sistema de Notificações Toast com Sonner

## Status
Aprovada

## Intenção
Integrar o sistema de notificações Sonner / Toast ao OpenPresenter para fornecer feedback visual instantâneo e amigável aos operadores (tanto no painel administrativo desktop quanto no controle remoto mobile) sobre eventos centrais da aplicação: início e parada de apresentações, salvamento e edição de músicas, conexão de novos aparelhos, e alterações de estado.

## Escopo
1. **Biblioteca e Componente**:
   - Integração da biblioteca oficial `svelte-sonner` compatível com Svelte 5.
   - Componente `frontend/src/lib/components/ui/Sonner.svelte` estilizado com as variáveis de tema do aplicativo e suporte a modo escuro/claro.
   - Utilitário `frontend/src/lib/toast.ts` exportando funções auxiliares de notificação (`notifySuccess`, `notifyError`, `notifyInfo`, `notifyWarning`, `notifyActionLog`).
2. **Eventos Cobertos**:
   - Apresentação iniciada ou slide selecionado.
   - Apresentação parada.
   - Música salva ou editada com sucesso no catálogo.
   - Novo aparelho ou controle remoto conectado na rede local (ex: "Novo dispositivo conectado: iPhone / Safari (192.168.1.15)").
   - Aparelho desconectado.
   - Modo de exibição alternado (Blackout / Limpar Texto).
3. **Escopo de Visualização**:
   - As notificações são exibidas no **Painel de Controle** (`AdminView`) e no **Controle Remoto Mobile** (`RemoteView`).
   - Telas de projeção (`PublicView`, `StageView`, `LowerThirdView`) **nunca** exibem toasts, garantindo que a transmissão e a projeção na igreja permaneçam 100% limpas.

## Decisões Técnicas
- **Integração com `WsMessage::ActionLogged`**:
  - Aproveitar o canal de logs e eventos em tempo real (`actionLogsStore`) para que o operador do desktop seja informado quando alguém no controle remoto mobile acionar um slide, parar a apresentação ou quando um novo voluntário conectar seu aparelho.
  - Comparar `log.instance_id` com o ID da instância local (`getInstanceId()`) para personalizar mensagens originadas de outros aparelhos (ex: "Controle Remoto iniciou apresentação").
- **Deduplicação**:
  - Evitar toasts duplicados para ações locais que já possuam feedback específico (como o salvamento de música no editor).

## Critérios de Aceite
1. O componente Sonner é montado em `AdminView` e `RemoteView`.
2. As telas públicas (`/public`, `/stage`, `/lower-third`) permanecem sem toasts.
3. Iniciar apresentação, parar apresentação, salvar música e conectar novo aparelho geram toasts informativos.
4. Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% aprovados.

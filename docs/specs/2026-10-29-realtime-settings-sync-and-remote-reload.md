# Spec: Sincronização em Tempo Real de Configurações e Recarga Remota das Telas Public e Stage

## Intenção
Garantir que qualquer alteração de configurações (margens, logotipo, tipografia, cores do stage display, aproveitamento de espaço) efetuada pelo operador seja refletida instantaneamente nas telas remotas `/public` e `/stage` via WebSocket, sem necessidade de intervenção ou recarga manual (F5). Além disso, disponibilizar botões de recarga remota no painel de controle para forçar a atualização das telas quando necessário.

## Escopo
- Backend (`op-core`): Adicionar variantes `SettingUpdated { key: String, value: Option<String> }` e `ReloadClients { target: Option<String> }` no enum `WsMessage`.
- Backend (`op-admin`):
  - No handler `save_app_setting`, realizar broadcast imediato de `WsMessage::SettingUpdated`.
  - No loop de WebSocket, repassar mensagens `ReloadClients` para os demais clientes.
  - Criar endpoint `POST /api/outputs/reload` para permitir disparo via HTTP se desejado.
- Frontend (`ws.ts` e `displaySettings.ts`):
  - Tratar mensagens `setting_updated` atualizando as stores reativas (`publicSettingsStore`, `churchBrandStore`, `stageSettingsStore`) e seus caches locais.
  - Tratar mensagens `reload_clients` recarregando a página na tela de destino (`/public` ou `/stage`).
- Frontend (`AdminView.svelte`):
  - Adicionar botões de ação rápida de recarga (`RefreshCw`) nos cabeçalhos de preview de "Public (Projetor)" e "Stage (Retorno Palco)".
  - Adicionar comandos na Command Palette (⌘K) para recarregar as telas.

## Critérios de Aceite
1. Ao alterar qualquer configuração no modal de configurações (ex: margem do logotipo ou cor do stage), as telas `/public` e `/stage` atualizam sua exibição instantaneamente sem recarregar a página.
2. Ao clicar no botão de recarga remota do Public ou Stage no painel do operador, a tela correspondente recarrega imediatamente.
3. Testes do workspace e linters executam sem erros.

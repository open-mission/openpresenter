# Especificação: Limpeza de Itens da Setlist Ativa

## 1. Intenção

Permitir que o operador limpe rapidamente todos os itens (músicas, versículos bíblicos, avisos e imagens) da setlist ativa com uma única ação, sem precisar remover item por item manualmente e sem excluir a setlist em si.

## 2. Escopo

1. **Backend Rust (`op-admin`)**:
   - Método `clear_active_setlist_items` em `AppState`:
     - Limpa os itens em memória (`media_items.clear()`).
     - Se houver mídia da setlist ativa sendo apresentada, redefine para modo `ClearText` e desativa o slide ativo.
     - Persiste a setlist atualizada com itens vazios no banco de dados SQLite (`sync_active_setlist_items`).
     - Emite `WsMessage::SyncState` e `WsMessage::SetlistUpdated` para sincronizar instantaneamente todas as interfaces conectadas.
   - Endpoint HTTP `POST /api/setlists/items/clear`.
2. **Frontend (`AdminView.svelte`, `ws.ts`)**:
   - Função utilitária `clearSetlistItems()` em `frontend/src/lib/ws.ts`.
   - Botão de ação rápida "Limpar Setlist" na barra de ações da Setlist (ao lado do contador de itens e botão de adicionar).
   - Opção "Limpar Itens da Setlist" no dropdown de gerenciamento de setlists (`isSetlistSelectorOpen`).
   - Confirmação do usuário (`confirm(...)`) antes de executar para evitar perda acidental de repertório.
   - Desativação visual quando a setlist já estiver vazia (`$mediaItemsList.length === 0`).
   - Limpeza do item selecionado (`selectedMediaId = ''`).

## 3. Decisões Arquiteturais

1. **Preservação dos Metadados da Setlist**:
   - A limpeza remove os itens filhos (`items: []`), mas preserva o ID, título, descrição, papel de parede e status de ativação da setlist.
2. **Segurança Operacional**:
   - Exige confirmação prévia para prevenir acionamentos acidentais durante transmissões ao vivo.
3. **Sincronização em Tempo Real**:
   - Disparo simultâneo de eventos de WebSocket (`SyncState` e `SetlistUpdated`) garantindo que todos os operadores vejam a lista limpa instantaneamente.

## 4. Critérios de Aceite

- [x] Endpoint `POST /api/setlists/items/clear` responde com sucesso e limpa os itens da setlist ativa.
- [x] O banco de dados SQLite armazena a setlist sem itens.
- [x] A apresentação ativa é finalizada de forma segura caso estivesse reproduzindo um item removido.
- [x] Na interface de administração, há opção visível e intuitiva no painel de Setlist.
- [x] A ação solicita confirmação do usuário antes de prosseguir.
- [x] Portões de qualidade passam: `cargo fmt --all`, `cargo clippy`, `cargo test` e `bun run check` + `bun run build`.

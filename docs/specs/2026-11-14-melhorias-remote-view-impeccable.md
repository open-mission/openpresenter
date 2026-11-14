# Especificação Técnica: Refinamento de UX/UI e Correção de Bugs do Controle Remoto Mobile (/remote)

## 1. Visão Geral e Contexto
O Controle Remoto Mobile (`/remote`) do OpenPresenter é a ferramenta de missão crítica utilizada por operadores e líderes de ministério da igreja em smartphones e tablets para conduzir o culto, avançar slides de louvor e liturgia, projetar versículos bíblicos de surpresa e coordenar a equipe em tempo real.

A auditoria Impeccable detectou problemas funcionais críticos (P0) que inviabilizavam a exibição de versículos bíblicos e slides de músicas, rota de API desatualizada, bloqueio de avanço entre itens da setlist e carência de feedback háptico táctil no aparelho celular.

---

## 2. Diagnóstico e Bugs Mapeados (P0 e P1)

### 2.1 [P0] Endpoint Bíblico Incorreto (`/verses`)
- **Causa Raiz**: `RemoteView.svelte` chamava `/api/bibles/${bibleId}/books/${bookId}/chapters/${chapter}`, enquanto a rota no backend Axum é `/api/bibles/{id}/books/{book_id}/chapters/{chapter}/verses`.
- **Impacto**: O Axum redirecionava rotas não mapeadas para o fallback estático da SPA (`index.html`), resultando em erro de parsing JSON (`Unexpected token '<', "<!doctype "`) e impossibilidade de consultar ou projetar a Bíblia.
- **Correção**: Corrigir a URL da API para incluir `/verses` e adicionar tratamento de erro robusto.

### 2.2 [P0] Mapeamento de Letras de Louvores (`lyrics_raw` / `sections`)
- **Causa Raiz**: `RemoteSongsTab.svelte` e `RemoteView.svelte` buscavam a propriedade `song.lyrics`, inexistente nas respostas da API de músicas (que retorna `song.lyrics_raw` e `song.sections`).
- **Impacto**: Todas as músicas do catálogo e da Harpa Cristã exibiam `"Esta música não possui slides configurados"`.
- **Correção**: Implementar parsing robusto que priorize `song.slides`, `song.sections` e `song.lyrics_raw`, garantindo que toda música exiba suas estrofes com etiquetas corretas (Verso, Refrão, etc.).

### 2.3 [P0] Chamada a Rota Inexistente de Projeção Rápida
- **Causa Raiz**: O código tentava fazer POST em `/api/presentation/quick` com `{ media, slide_index }`, enquanto o Axum expõe `/api/presentation/quick-media` com `{ item, slide_index }`.
- **Correção**: Integrar diretamente `quickPresentMedia` e `selectSlide` de `$lib/ws`, assegurando broadcast imediato por WebSocket com fallback HTTP padronizado.

### 2.4 [P1] Navegação Fluida entre Itens da Ordem do Culto
- **Causa Raiz**: O operador ficava travado no último slide da música no card de controle ao vivo.
- **Correção**: Permitir que "Próximo" no último slide de um item avance para o primeiro slide do próximo item da setlist ativa; e "Anterior" no primeiro slide retroceda para o último slide do item anterior, com indicação contextual do próximo item.

### 2.5 [P1] Feedback Háptico e Ergonomia Mobile
- **Implementação**: Utilitário seguro `triggerHaptic(type: 'light' | 'medium' | 'warning')` usando `navigator.vibrate`, ativado no toque de slides, avanço/recuo e controles de saída (Preto/Limpar/Parar).
- **Áreas de toque**: Garantir alvos confortáveis (`>= 44×44px` ou `>= 48×48px`), respeitando safe areas iOS/Android.

### 2.6 [P1] Arquitetura Modularizada do `/remote`
- **Decomposição**: Reduzir `RemoteView.svelte` de 1.150 linhas para um orquestrador limpo, extraindo:
  - `controller.svelte.ts`: Gerenciamento reativo de estado, dados e chamadas WS/API.
  - `RemoteSetlistSheet.svelte`: Seletor de setlists em sheet acessível.
  - `RemoteBibleSheet.svelte`: Seletor de versão bíblica.
  - `RemotePinDialog.svelte`: Desbloqueio seguro de operador com PIN.
  - `RemoteStopDialog.svelte`: Confirmação acessível de parada com `role="alertdialog"`.

---

## 3. Critérios de Aceite
1. **Bíblia Funcional**: Leitura de versículos de qualquer livro e capítulo sem erros de JSON; projeção de versículos rápida e precisa.
2. **Catálogo de Louvores**: Exibição completa de estrofes de músicas locais e da Harpa Cristã; projeção ao vivo via 1 Toque ou 2 Toques.
3. **Setlist Contínua**: Avanço e recuo contínuo entre itens da setlist pelo card principal sem interrupções.
4. **Ergonomia e Haptics**: Vibração tátil sutil em dispositivos móveis compatíveis sem falhas em navegadores desktop.
5. **Portões de Qualidade**: `npm run check` (0 erros), `npm run build` (sucesso), `cargo test -p op-db -p op-admin` (78/78 testes passando), `impeccable detect` (0 antipadrões).

# Especificação — Aba "Outros" no Painel de Mídias (Agenda, Textos e Avisos)

**Data:** 2026-11-01  
**Status:** Aprovado  
**Autores:** OpenPresenter Core Team  
**Plano correspondente:** [`docs/plans/2026-11-01-aba-outros-painel-de-midias.md`](../plans/2026-11-01-aba-outros-painel-de-midias.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Contexto e Motivação

O painel inferior de mídias (`BottomPanel.svelte`) é o catálogo central onde os operadores acessam recursos para compor o culto:
- Músicas (`songs`)
- Imagens (`images`)
- Fundos & Gradientes (`backgrounds`)
- Bíblia (`bible`)

Com a introdução dos tipos de mídia Agenda (`MediaKind::Schedule`), Textos Livres (`MediaKind::Text`), Avisos (`MediaKind::Announcement`) e Seções/Divisores (`MediaKind::Header`), o operador precisava criá-los manualmente na setlist a cada culto ou reeditá-los sem um repositório centralizado de consulta e reutilização.

Esta especificação define a nova aba **"Outros"** no painel de mídias inferior, oferecendo um catálogo organizado para itens de agenda, textos, avisos e futuros tipos de mídia, com busca, filtros por categoria, pré-visualização de slides em 16:9, adição rápida à setlist ativa e projeção imediata ao vivo.

---

## 2. Escopo

### 2.1 Em Escopo
1. **Catálogo Persistente de Itens Customizados:**
   - Módulo `frontend/src/lib/customItems.ts` com store reativa `customItemsStore`.
   - Persistência no banco SQLite através do endpoint de configurações `/api/settings/custom_media_items` e sincronização automática via WebSocket (`WsMessage::SettingUpdated`).
   - Seeding inteligente com modelos úteis de culto caso o catálogo esteja vazio (ex.: Programação Semanal com dias e horários, Boas-Vindas aos Visitantes e Avisos Gerais).
2. **Integração no Painel de Mídias (`BottomPanel.svelte`):**
   - Nova aba "Outros" na barra superior do painel com ícone de recursos (`Boxes` / `Layers`).
   - Barra de busca textual e filtros de chips por tipo: `Todos`, `📅 Agenda`, `📝 Textos`, `📢 Avisos`, `🔖 Seções`.
   - Botões de criação direta na barra: `+ Agenda`, `+ Texto`, `+ Aviso`.
   - Botão para sincronizar/importar itens avulsos da setlist ativa para o catálogo.
3. **Interface Dividida em Lista e Prévia 16:9:**
   - Lista de cards ergonômicos à esquerda: título, badge do tipo, quantidade de slides, resumo dos dados, e botões `+ Setlist`, `Ao Vivo`, `Editar` e `Excluir`.
   - Painel de visualização à direita: cabeçalho com ações e grade responsiva de slides 16:9 com cards estilizados para cada tipo (badges e horários para agenda; tipografia centrada para textos).
4. **Integração no `AdminView.svelte`:**
   - Salvamento automático de novas agendas, textos e avisos criados na setlist também no catálogo de itens customizados.
   - Conexão dos callbacks de ação do `BottomPanel`: adicionar à setlist, projetar diretamente com `quickPresentMedia` e disparar os modais de edição.

### 2.2 Fora de Escopo
- Alterações em schemas de migração do banco SQLite (utilização da tabela existente `app_settings` com JSON serializado).
- Criação de novos tipos de mídia no backend além dos já suportados por `MediaKind`.

---

## 3. Decisões de Arquitetura

1. **Armazenamento via `app_settings`:** A chave `custom_media_items` é salva no banco SQLite através da infraestrutura de `app_settings`, reaproveitando o broadcast de WebSocket em tempo real e a persistência ACID sem requerer migração de schema.
2. **Reuso de Modais Existentes:** As ações de criação e edição da aba "Outros" utilizam os mesmos modais testados e consolidados (`ScheduleEditorModal.svelte`, `SongEditorModal.svelte` em modo texto e `QuickItemModal.svelte`).
3. **Projeção Imediata Desacoplada da Setlist:** Operadores podem apresentar um item ou slide diretamente da aba "Outros" sem necessidade de adicioná-lo à setlist antes, usando `quickPresentMedia`.

---

## 4. Critérios de Aceite

1. O operador visualiza a aba **Outros** no painel inferior de mídias (`BottomPanel.svelte`).
2. A lista exibe itens de agenda, textos, avisos e divisores, permitindo pesquisa instantânea e filtro por categoria.
3. Ao clicar em um item, o painel lateral exibe a prévia de todos os slides no formato 16:9.
4. É possível adicionar o item à setlist ativa com 1 clique (`+ Setlist`).
5. É possível apresentar o item ou qualquer slide individual imediatamente ao vivo (`Ao Vivo`).
6. Botões `+ Agenda`, `+ Texto` e `+ Aviso` abrem seus respectivos modais e salvam os itens no catálogo.
7. Todos os portões de qualidade passam: `bun run build`, `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.

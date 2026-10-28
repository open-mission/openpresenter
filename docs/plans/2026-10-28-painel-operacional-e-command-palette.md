# Plano — Painel Principal Operacional e Command Palette

- **Status:** concluído
- **Data:** 2026-10-28
- **Spec associada:** [`docs/specs/2026-10-28-painel-operacional-e-command-palette.md`](../specs/2026-10-28-painel-operacional-e-command-palette.md)
- **Versão do app:** 0.3.0
- **Versão do schema:** 2 (inalterado)

## 1. Visão Geral

Este plano organiza a transformação da janela administrativa em um painel operacional completo para cultos e eventos com três áreas de trabalho (Sidebar de Mídias, Programação do Evento e Preview de Mídias), implementação da Command Palette com atalhos de teclado (`⌘K` / `Ctrl+K`), abertura maximizada da janela principal e isolamento correto das janelas na inicialização.

---

## 2. Tarefas de Implementação

### Tarefa 1: Arquitetura da Command Palette no Rust (`crates/op-admin/src/commands.rs`)
- [x] Definir a enum de categorias: `All`, `Songs`, `Bible`, `Commands`.
- [x] Definir a struct `CommandItem`: id, title, category, description, shortcut, action callback.
- [x] Implementar `CommandRegistry` para catalogar comandos do sistema (abrir configurações, alternar telas, limpar tela) e itens de exemplo para Músicas e Bíblia.
- [x] Implementar busca e filtragem por texto e por categoria.
- [x] Criar testes unitários para o registro e busca de comandos.

### Tarefa 2: Componentes Slint da Command Palette (`crates/op-admin/ui/command_palette.slint`)
- [x] Criar componente modal `CommandPalette` com backdrop escurecido, caixa de busca, input com auto-focus e ícone.
- [x] Abas de filtros ("Todos", "Músicas", "Bíblia", "Comandos").
- [x] Lista de resultados rolável com ícone, título, categoria e atalho.
- [x] Destaque do item selecionado e suporte a navegação por teclado (Enter, Escape, Setas).

### Tarefa 3: Redesenho do Painel Principal (`crates/op-admin/ui/main.slint`)
- [x] Remover cartões legados de metadados de sessão/banco da tela principal.
- [x] Header moderno:
  - Marca e wordmark OpenPresenter.
  - Botão de busca rápida que abre a Command Palette com indicação do atalho (`⌘K`).
  - Badges de estado das saídas Public e Stage.
  - Botão de Menu (`☰`).
- [x] Layout de 3 colunas:
  1. **Sidebar Lateral (Esquerda)**: Mídias da programação (Músicas, Bíblia, Mídias, Avisos) e lista de recursos com botão "+ Nova Mídia".
  2. **Painel de Programação (Centro)**: Ordem do culto/evento com itens organizados, numeração, seleção e botões de reordenação.
  3. **Painel de Preview das Mídias (Direita)**: Visualização dos slides do item selecionado, telas em miniatura, distinção entre "Ao Vivo" e "Preview", e barra de controle de projeção (Ao Vivo, Limpar Tela, Ocultar Texto, Logotipo).
- [x] Rodapé com status discreto (conexão SQLite, versão).
- [x] Integração do `CommandPalette` sobreposto à janela principal.
- [x] Captura de atalhos globais de teclado no `FocusScope` raiz (`⌘K` / `Ctrl+K` para abrir; `Esc` para fechar).

### Tarefa 4: Conexão no Rust (`crates/op-admin/src/ui.rs` e `crates/op-admin/src/main.rs`)
- [x] Conectar os callbacks da interface: busca na command palette, execução de comando, seleção de item na programação, alternância de slide no preview, botões de ação (Ao Vivo, Limpar Tela).
- [x] Fazer a janela principal abrir preenchendo o espaço disponível da tela (`maximized: true`).
- [x] Ajustar a inicialização em `ui.rs` para que o painel administrativo e as saídas (Public e Stage) abram coordenadamente em monitores separados, sem sobreposição indesejada.

### Tarefa 5: Atualização da Documentação e Portões de Qualidade
- [x] Atualizar `docs/README.md` linkando a nova spec e este plano.
- [x] Atualizar `CHANGELOG.md` com as novas funcionalidades sob a versão [Unreleased] / 0.3.0.
- [x] Executar `cargo fmt --all`.
- [x] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Executar `cargo test --workspace`.

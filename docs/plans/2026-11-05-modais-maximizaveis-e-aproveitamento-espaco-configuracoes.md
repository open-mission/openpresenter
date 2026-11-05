# Plano: Modais Maximizáveis e Aproveitamento de 100% do Espaço em Configurações

**Data:** 2026-11-05  
**Spec correspondente:** [`docs/specs/2026-11-05-modais-maximizaveis-e-aproveitamento-espaco-configuracoes.md`](../specs/2026-11-05-modais-maximizaveis-e-aproveitamento-espaco-configuracoes.md)

---

## Etapas de Execução

### Etapa 1: Atualização do `SettingsModal.svelte`
- [x] Adicionar imports `Maximize2` e `Minimize2` de `lucide-svelte`.
- [x] Criar estado reativo `let isMaximized = $state(false);`.
- [x] Ajustar backdrop e container do modal com classes condicionais (`w-full h-full rounded-none` quando maximizado vs `w-[95vw] max-w-6xl rounded-xl` quando padrão).
- [x] Adicionar botão de maximizar / restaurar no cabeçalho antes do botão de fechar.
- [x] Reformular seções com largura restritiva:
  - Aba Sobre (`about`): transformar em layout de grid responsivo `w-full max-w-5xl` aproveitando a largura total para Updates e API Docs.
  - Aba Atalhos (`shortcuts`): distribuir atalhos em 2 colunas responsivas `w-full max-w-5xl`.
  - Abas Aparência, Notificações DND, Rede e Pastas: flexibilizar limitadores de largura para `w-full max-w-4xl`.

### Etapa 2: Atualização dos Modais de Criação e Edição
- [x] **`SongEditorModal.svelte`**:
  - Adicionar `isMaximized` com `Maximize2` / `Minimize2`.
  - Expandir container para `w-screen h-screen` preenchendo a tela toda.
- [x] **`ScheduleEditorModal.svelte`**:
  - Adicionar `isMaximized` com `Maximize2` / `Minimize2`.
  - Expandir lista de eventos, formulário e preview para altura e largura total.
- [x] **`QuickItemModal.svelte`**:
  - Adicionar `isMaximized` com `Maximize2` / `Minimize2`.
  - Expandir textarea de edição de slides/avisos em tela cheia.
- [x] **`TeamsModal.svelte`**:
  - Adicionar `isMaximized` com `Maximize2` / `Minimize2`.
  - Permitir que o Svelte Flow e a grade de cartões utilizem 100% do display.
- [x] **`SetlistModal.svelte`**:
  - Adicionar `isMaximized` com `Maximize2` / `Minimize2`.
- [x] **`BackgroundSelectorModal.svelte` & `ImageSelectorModal.svelte`**:
  - Adicionar `isMaximized` com `Maximize2` / `Minimize2` para visualização expandida de fundos e imagens.

### Etapa 3: Build, Documentação e Validação
- [x] Executar `npm run build` no frontend.
- [x] Atualizar `docs/README.md` e `CHANGELOG.md`.
- [x] Executar portões de qualidade Rust (`cargo fmt`, `cargo clippy`, `cargo test`).

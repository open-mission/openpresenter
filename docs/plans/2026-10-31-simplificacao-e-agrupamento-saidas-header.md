# Plano: Simplificação e Agrupamento das Saídas no Cabeçalho

Plano associado à especificação [`docs/specs/2026-10-31-simplificacao-e-agrupamento-saidas-header.md`](../specs/2026-10-31-simplificacao-e-agrupamento-saidas-header.md).

## Etapas de Execução

### Etapa 1: Estado e Lógica no Frontend (`AdminView.svelte`)
1. Adicionar o estado reativo `isOutputsMenuOpen = $state(false);` em `AdminView.svelte`.
2. Adicionar a função `openLowerThirdWindow()` para complementar `openPublicWindow()` e `openStageWindow()`.
3. Adicionar o tratamento de fechamento em `handleKeydown` (Escape) e fechar o menu ao clicar fora ou ao selecionar qualquer ação.
4. Importar o ícone `Video` de `lucide-svelte` para a opção Lower Third (OBS).

### Etapa 2: Construção da Interface do Dropdown "Saídas"
1. Substituir os 5 botões individuais (`Iniciar Public`, `Iniciar Stage`, `Cast`, `QR Code`, `Remoto`) pelo botão único de menu `Saídas`.
2. Adicionar indicador de estado conectado para o Google Cast no botão de gatilho.
3. Renderizar o painel suspenso do dropdown com as duas seções:
   - Seção 1: Telas de Projeção (Pública, Palco, OBS) com ícones e links externos.
   - Seção 2: Conexões Sem Fio & Compartilhamento (Google Cast, Controle Remoto Mobile, QR Codes).

### Etapa 3: Verificação e Portões de Qualidade
1. Testar compilação do frontend com `bun run build`.
2. Executar `cargo fmt --all`.
3. Executar `cargo clippy --workspace --all-targets -- -D warnings`.
4. Executar `cargo test --workspace`.
5. Atualizar `docs/README.md`, `CHANGELOG.md` e `TODO.md`.

# Especificação — Refatoração do Módulo 11 (Seletor de Imagens & Mídias Locais)

## 1. Contexto e Motivação

O `ImageSelectorModal.svelte` é o componente responsável pela seleção e importação de imagens avulsas ou pastas inteiras de fotos para a Setlist de apresentação ou como mídia dedicada para o Lower Third (saída OBS/transmissão).

Atualmente o componente possui 470 linhas em um arquivo monolítico com pontuação **20 / 40** no Impeccable Critique (`Requer Refatoração Estrutural`).

### Diagnóstico de Problemas (Baseline):
- **[P0] Uso de `alert()` Nativo (Linha 162)**: Chamada a `alert('Esta pasta não possui imagens.')`, bloqueando a interface e quebrando acessibilidade.
- **[P0] Tema e Backdrop Escuro Fixo (`bg-black/80`)**: Ausência de token semântico adaptativo (`bg-background/80`).
- **[P1] Falhas Graves de WAI-ARIA**: O diálogo não possui `role="dialog"`, `aria-modal="true"`, `aria-labelledby`, `aria-describedby` nem `tabindex="-1"`; não há armadilha de foco (`focusTrap`); não há suporte a fechamento via tecla `Escape`; cards de imagens e pastas são implementados com `div` sem atributos acessíveis (`role="checkbox"` / `role="radio"`, `aria-checked`, `tabindex="0"`).
- **[P1] Violação Severa do Piso Tipográfico**: Fontes com `text-[9px]` e `text-[10px]` em legendas e caminhos de pastas.
- **[P1] Monólito sem Modularização**: Lógica de busca, filtros de pasta, seleção de itens, montagem de itens de mídia (`MediaItem`/`Slide`), requisições de API (`/api/media/images`, `/api/media/folders`) condensadas em um único componente.
- **[P2] Alvos Interativos Reduzidos**: Filtros de pasta com altura inferior a 24px e falta de atalho `⌘Enter` / `Ctrl+Enter` para adicionar/confirmar.

---

## 2. Metas de Design e Arquitetura

1. **Decomposição Modular**:
   Criar a pasta `frontend/src/lib/components/image-selector/` contendo:
   - `types.ts`: interfaces de props, modos de seleção e callbacks.
   - `defaults.ts`: constantes e rótulos padrão.
   - `controller.svelte.ts`: classe de controle com runes Svelte 5 (`$state`, `$derived`), busca, filtro por pastas, seleção múltipla com `Set`, conversão para `MediaItem`, carregamento de API, tratamento de foco e teclado (`⌘Enter`, `Escape`).
   - `ImageSelectorHeader.svelte`: cabeçalho acessível com ícone de imagem, título dinâmico, subtítulo, botões Maximizar e Fechar.
   - `ImageSelectorModeTabs.svelte`: abas para alternância entre modo Imagens Individuais e Pasta Completa (quando habilitado).
   - `ImageSelectorFolderFilter.svelte`: barra de filtro por pastas com scroll horizontal e botão de Selecionar/Desmarcar Todas, com alvos táteis confortáveis `>= 28px`.
   - `ImageSelectorGrid.svelte`: grade responsiva de imagens com cards acessíveis (`role="checkbox"`), checkbox visual contrastante, legenda no piso `>= 11px` e suporte a clique duplo.
   - `ImageSelectorFolderList.svelte`: seletor de pastas completas acessível (`role="radiogroup"` / `role="radio"`), contadores de imagens e feedback inline acessível quando a pasta estiver vazia (substituindo o `alert()`).
   - `ImageSelectorFooter.svelte`: rodapé com contadores contextuais de seleção, botão Cancelar e botão de confirmação com suporte a `⌘Enter`.

2. **Adoção Estrita de Tokens Semânticos**:
   - Uso de `bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`, `bg-primary`, `text-primary-foreground`.
   - Contraste WCAG AA assegurado nos modos Claro e Escuro.

3. **Acessibilidade WAI-ARIA e Teclado**:
   - Diálogo com `role="dialog"`, `aria-modal="true"`, `aria-labelledby="image-selector-title"`, `aria-describedby="image-selector-desc"`.
   - Armadilha de foco via `focusTrap.ts`.
   - Foco inicial colocado no input de busca ou na primeira imagem.
   - Suporte a `⌘Enter` / `Ctrl+Enter` para confirmar a inserção.
   - Fechamento imediato com tecla `Escape`.
   - Cards acessíveis via teclado (`Enter` e `Space` para marcar/desmarcar).

---

## 3. Critérios de Aceite e Portões

1. `ImageSelectorModal.svelte` orquestrador limpo (< 130 linhas).
2. Pontuação no Impeccable Critique evoluindo de **20/40** para `>= 36/40` (`Aprovado para Produção`).
3. Zero erros e zero warnings no `npm run check`.
4. Build de produção `npm run build` bem-sucedido.
5. Zero achados no `npx impeccable detect frontend/src --json`.
6. 78/78 testes passando no `cargo test`.

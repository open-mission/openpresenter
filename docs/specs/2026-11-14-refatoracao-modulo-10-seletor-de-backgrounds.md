# Especificação — Refatoração do Módulo 10 (Seletor de Fundos & Gradientes)

## 1. Contexto e Motivação

O `BackgroundSelectorModal.svelte` gerencia a exploração e seleção de papéis de parede e planos de fundo visuais para a projeção pública. Ele apresenta presets procedurais CSS (gradientes lineares, mesh gradients e gradientes animados), além de mídias locais (imagens e vídeos).

Atualmente o componente possui 395 linhas em um arquivo monolítico com pontuação **21 / 40** no Impeccable Critique (`Requer Refatoração Estrutural`).

### Diagnóstico de Problemas (Baseline):
- **[P0] Backdrop Escuro Fixo (`bg-black/80`)**: Ausência de token semântico adaptativo (`bg-background/80`).
- **[P1] Falhas Graves de WAI-ARIA**: O diálogo não declara `role="dialog"`, `aria-modal="true"`, `aria-labelledby`, `aria-describedby` nem `tabindex="-1"`; não há armadilha de foco (`focusTrap`); não há suporte a tecla `Escape`; os cards utilizam `<div role="button">` contendo um `<Button>` aninhado dentro, o que viola as diretrizes WAI-ARIA de botões aninhados.
- **[P1] Violação Severa do Piso Tipográfico**: Uso generalizado de `text-[9px]` e `text-[10px]` em badges de contagem de categorias, badge animado, badge de vídeo, texto de exemplo de letra, descrições e botões de ação.
- **[P1] Monólito sem Modularização**: Lógica de busca textual, filtragem de categorias, chamada à API `/api/media/wallpapers`, sidebar de navegação e renderização de cards procedurais e de vídeo acoplados.
- **[P2] Alvos Interativos Reduzidos**: Botões de ação com apenas 24px de altura (`h-6`), botões de fechar/maximizar compactos demais.

---

## 2. Metas de Design e Arquitetura

1. **Decomposição Modular**:
   Criar a pasta `frontend/src/lib/components/background-selector/` contendo:
   - `types.ts`: interfaces de props, categorias e itens de background.
   - `defaults.ts`: definições de categorias de fundo, rótulos e ícones.
   - `controller.svelte.ts`: classe reativa com runes Svelte 5 (`$state`, `$derived`), busca, filtro por categorias, contadores dinâmicos, carregamento assíncrono de mídias locais e gerenciamento de foco/Escape.
   - `BackgroundHeader.svelte`: cabeçalho acessível com ícone semântico de paleta, título, descrição, botões Maximizar e Fechar com alvos confortáveis.
   - `BackgroundToolbar.svelte`: campo de pesquisa com ícone e breadcrumb da categoria ativa.
   - `BackgroundSidebar.svelte`: navegação vertical de categorias com `role="tablist"` / `role="tab"` acessível, badges de contagem no piso mínimo `>= 11px`.
   - `BackgroundCard.svelte`: cartão unificado para presets CSS e papéis de parede locais (vídeo/imagem), eliminando aninhamento inválido de botões, com `aria-label` descritivo, preview 16:9, indicador visual de seleção e botão de ação `>= 28px`.
   - `BackgroundGrid.svelte`: grade responsiva para exibição dos cards com estado vazio acessível quando a busca não retornar resultados.
   - `BackgroundFooter.svelte`: rodapé informativo com dica de seleção automática e botão Fechar.

2. **Adoção Estrita de Tokens Semânticos**:
   - Uso de `bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`, `bg-primary`, `text-primary-foreground`.
   - Garantia de contraste WCAG AA tanto no tema escuro quanto no claro.

3. **Acessibilidade WAI-ARIA e Teclado**:
   - Diálogo com `role="dialog"`, `aria-modal="true"`, `aria-labelledby="bg-selector-title"`, `aria-describedby="bg-selector-desc"`.
   - Armadilha de foco com `focusTrap.ts`.
   - Foco inicial colocado no input de busca ao abrir.
   - Fechamento imediato com tecla `Escape`.
   - Cards acessíveis via teclado (`Enter` e `Space` para selecionar).
   - Piso tipográfico `>= 11px` e alvos interativos `>= 28×28px`.

---

## 3. Critérios de Aceite e Portões

1. `BackgroundSelectorModal.svelte` orquestrador limpo (< 120 linhas).
2. Pontuação no Impeccable Critique evoluindo de **21/40** para `>= 36/40` (`Aprovado para Produção`).
3. Zero erros e zero warnings no `npm run check`.
4. Build de produção `npm run build` bem-sucedido.
5. Zero achados no `npx impeccable detect frontend/src --json`.
6. 78/78 testes passando no `cargo test`.

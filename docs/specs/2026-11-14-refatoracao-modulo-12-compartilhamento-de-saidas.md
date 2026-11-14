# Especificação — Refatoração do Módulo 12 (Compartilhamento e Streaming de Saídas)

## 1. Contexto e Motivação

O `ShareOutputsModal.svelte` é o modal responsável por gerar links, QR Codes e gerenciar a transmissão sem fio (Google Cast / Chromecast) das saídas do OpenPresenter:
- **Palco (`/stage`)**: Monitor de retorno para pregadores e equipe de louvor.
- **Pública (`/public`)**: Tela principal de projeção para telões e monitores auxiliares.
- **Remoto (`/remote`)**: Painel de controle móvel para tablets e smartphones.
- **OBS / Lower Third (`/lower-third`)**: Saída com transparência alfa para software de transmissão (OBS Studio, vMix).
- **Cast (`Google Cast`)**: Descoberta e envio direto para dispositivos Chromecast/Google TV.

Atualmente o componente possui 313 linhas em um arquivo monolítico com pontuação **20 / 40** no Impeccable Critique (`Requer Refatoração Estrutural`).

### Diagnóstico de Problemas (Baseline):
- **[P0] Tema Escuro Rígido (`neutral-*`, `zinc-*`, `bg-black/80`)**: Elementos com fundos e textos pretos fixos (`bg-neutral-900`, `border-neutral-800`, `bg-neutral-950`, `text-white`), ignorando o Modo Claro (`data-theme="light"`).
- **[P1] Falhas Graves de WAI-ARIA**: O backdrop externo tem `role="dialog"` em vez da caixa de conteúdo; ausência de `aria-describedby` e `tabindex="-1"`; não há armadilha de foco (`focusTrap`); seletor de abas sem `role="tablist"` / `role="tab"` e sem suporte a setas do teclado; notificação de cópia sem suporte a leitor de telas (`aria-live="polite"`).
- **[P1] Falta de Hierarquia e Piso Tipográfico Apertado**: Uso indiscriminado de `text-[11px]` sem escalonamento tipográfico adequado.
- **[P1] Monólito sem Modularização**: Lógica de geração de QR Code, requisições de rede, detecção e substituição de IP local, controle de área de transferência e integração com `CastControl` em um único arquivo.
- **[P2] Alvos Interativos Reduzidos**: Abas com altura inferior a 28px (`py-1.5`) e link de rodapé compacto.

---

## 2. Metas de Design e Arquitetura

1. **Decomposição Modular**:
   Criar a pasta `frontend/src/lib/components/share-outputs/` contendo:
   - `types.ts`: interfaces de props, abas e informações de rede.
   - `defaults.ts`: definições das 5 abas (Palco, Pública, Remoto, OBS, Cast) com ícones, descrições contextuais e títulos.
   - `controller.svelte.ts`: classe reativa com runes Svelte 5 (`$state`, `$derived`), requisição assíncrona para `/api/system/network-info`, correção inteligente de IP local, geração reativa de QR Code via biblioteca `qrcode`, cópia para clipboard com debounce de estado e gerenciamento de foco/teclado (`Escape`).
   - `ShareOutputsHeader.svelte`: cabeçalho acessível com ícone dinâmico (Cast vs QrCode), título, descrição e botão Fechar.
   - `ShareOutputsTabs.svelte`: barra de navegação com `role="tablist"`, suporte a setas (`ArrowLeft`/`ArrowRight`), `role="tab"` com `aria-selected` e alvos táteis confortáveis `>= 32px`.
   - `ShareOutputsQrView.svelte`: container de visualização com QR Code responsivo (com fundo branco puro para leitura óptica garantida de câmeras), instruções, caixa de URL com botão Copiar acessível e live region.
   - `ShareOutputsFooter.svelte`: rodapé semântico com indicação de IP da máquina e botão "Abrir link" ou indicador de Google Cast SDK.

2. **Adoção Estrita de Tokens Semânticos**:
   - Eliminação de `neutral-*`, `zinc-*`, `bg-black/80` e `text-white`.
   - Adoção de `bg-card`, `bg-background`, `bg-muted`, `border-border`, `text-foreground`, `text-muted-foreground`, `bg-primary`, `text-primary-foreground`.
   - Contraste WCAG AA assegurado nos temas Claro e Escuro.

3. **Acessibilidade WAI-ARIA e Teclado**:
   - Painel do diálogo com `role="dialog"`, `aria-modal="true"`, `aria-labelledby="share-outputs-title"`, `aria-describedby="share-outputs-desc"`, `tabindex="-1"`.
   - Armadilha de foco via `focusTrap.ts`.
   - Foco inicial seguro ao abrir o modal.
   - Fechamento imediato com tecla `Escape`.
   - Anúncio acessível de cópia via `aria-live="polite"`.

---

## 3. Critérios de Aceite e Portões

1. `ShareOutputsModal.svelte` orquestrador limpo (< 100 linhas).
2. Pontuação no Impeccable Critique evoluindo de **20/40** para `>= 36/40` (`Aprovado para Produção`).
3. Zero erros e zero warnings no `npm run check`.
4. Build de produção `npm run build` bem-sucedido.
5. Zero achados no `npx impeccable detect frontend/src --json`.
6. 78/78 testes passando no `cargo test`.

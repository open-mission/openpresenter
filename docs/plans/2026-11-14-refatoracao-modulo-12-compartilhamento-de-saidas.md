# Plano de Execução — Refatoração do Módulo 12 (Compartilhamento e Streaming de Saídas)

## Fases de Execução

### Fase 1: Fundação de Tipos, Defaults e Controlador
- Criar `frontend/src/lib/components/share-outputs/types.ts`:
  - Tipos e interfaces: `ShareOutputTab`, `ShareOutputsModalProps`, callbacks.
- Criar `frontend/src/lib/components/share-outputs/defaults.ts`:
  - Metadados das 5 abas (`stage`, `public`, `remote`, `lower`, `cast`), ícones, rótulos e instruções contextuais.
- Criar `frontend/src/lib/components/share-outputs/controller.svelte.ts`:
  - Runes Svelte 5 (`$state`, `$derived`).
  - Estados: `open`, `activeTab`, `networkInfo`, `isLoading`, `qrCodeDataUrl`, `isCopied`, `dialogEl`.
  - Métodos: `syncOpen`, `loadNetworkInfo`, `generateQrCode`, `copyUrl`, `openExternal`, `handleKeydown`, `close`.

### Fase 2: Componentes Auxiliares e Submódulos
- Criar `frontend/src/lib/components/share-outputs/ShareOutputsHeader.svelte`:
  - Ícone dinâmico (Cast vs QrCode), título, subtítulo e botão Fechar com `aria-label` e foco visível.
- Criar `frontend/src/lib/components/share-outputs/ShareOutputsTabs.svelte`:
  - Navegação de abas com `role="tablist"` / `role="tab"`, navegação por setas (`ArrowLeft`/`ArrowRight`), `aria-selected` e alvos táteis confortáveis `>= 32px`.
- Criar `frontend/src/lib/components/share-outputs/ShareOutputsQrView.svelte`:
  - Exibição de QR Code de alta resolução com fundo branco puro, instrução com ícone de Wi-Fi, caixa de link truncada e botão de copiar com feedback sonoro/visual acessível (`aria-live="polite"`).
- Criar `frontend/src/lib/components/share-outputs/ShareOutputsFooter.svelte`:
  - Rodapé semântico com IP local do servidor e botão "Abrir link" com ícone de link externo ou indicação de Google Cast SDK.

### Fase 3: Refatoração do Orquestrador `ShareOutputsModal.svelte`
- Substituir o conteúdo monolítico pelo novo orquestrador conectando o controlador e os submódulos da pasta `share-outputs/`.
- Delegar a aba `cast` para o subcomponente existente `CastControl.svelte`.
- Garantir `role="dialog"`, `aria-modal="true"`, `aria-labelledby`, `aria-describedby`, `tabindex="-1"`, trap de foco com `focusTrap.ts` e delegação da tecla `Escape`.
- Reduzir o orquestrador para menos de 100 linhas.

### Fase 4: Verificação e Portões de Qualidade
- `npm run check` (0 erros e 0 warnings).
- `npm run build` (sucesso).
- `npx impeccable detect frontend/src --json` (0 apontamentos).
- `cargo test -p op-db -p op-admin` (78 testes passando).
- Auditoria e pós-fix do Impeccable Critique (`.impeccable/critique/`).
- Atualização da documentação (`CHANGELOG.md`, `docs/README.md`).

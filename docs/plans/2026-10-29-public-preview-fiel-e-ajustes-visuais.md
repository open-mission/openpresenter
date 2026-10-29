# Plano de Implementação: Preview Fiel da Tela Pública e Ajustes Visuais

## 1. Etapas

### Etapa 1: Tipagem e Defaults
- Adicionar `darken_background?: boolean` e `text_box_background?: boolean` na interface `PublicDisplaySettings` em `frontend/src/lib/types.ts`.
- Definir valores padrão como `false` em `DEFAULT_PUBLIC_SETTINGS` em `frontend/src/lib/displaySettings.ts`.

### Etapa 2: Atualização de `PublicView.svelte`
- Tornar o overlay escuro condicional a `publicSettings.darken_background`.
- Aplicar estilo de placa translúcida (`text_box_background`) ao redor do container de texto quando a opção estiver ativada.
- Ampliar a referência bíblica para tipografia de grande porte com badge da versão da Bíblia.

### Etapa 3: Criação de `PublicPreviewBox.svelte`
- Criar `frontend/src/lib/components/PublicPreviewBox.svelte` com aspect ratio 16:9, suporte a:
  - Fundo (gradiente CSS ou imagem com modo de preenchimento).
  - Overlay escuro condicional (`darken_background`).
  - Placa de texto condicional (`text_box_background`).
  - Margens configuradas.
  - Alinhamento horizontal e vertical do slide.
  - Quebra de linha (`word_wrap`) e tamanho (`fill_space`).
  - Referência bíblica ampliada.
  - Logo da igreja (`churchBrand`) com posicionamento e opacidade.
  - Exibição de blackout e cleartext.

### Etapa 4: Integração no Painel Admin (`AdminView.svelte`)
- Substituir o bloco estático de preview do Public por `<PublicPreviewBox>`.

### Etapa 5: Atualização de `SettingsModal.svelte` e `StagePreviewBox.svelte`
- Adicionar os controles de `darken_background` e `text_box_background` na aba da Tela Pública com textos explicativos aprimorados.
- Garantir atualização reativa imediata via spread nos campos de palco e tela pública.
- Aprimorar o `StagePreviewBox` para aplicar as proporções em todos os presets e integrar na modal de configurações.

### Etapa 6: Documentação e Portões de Qualidade
- Atualizar `docs/README.md` e `CHANGELOG.md`.
- Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Executar `bun run check` e `bun run build`.

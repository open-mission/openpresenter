# Plano de Implementação: Ajustes de Imagens, QR Code, Timer e Otimização do Stage

- **Data:** 29/10/2026
- **Status:** Em Andamento
- **Spec Correspondente:** [`docs/specs/2026-10-29-image-display-qrcode-timer-stage-media.md`](../specs/2026-10-29-image-display-qrcode-timer-stage-media.md)

---

## Etapas de Execução

### Etapa 1: Backend Rust (`op-core` e `op-admin`)
1. **`TimerState` com controle de overtime:**
   - Adicionar `allow_overtime: bool` (padrão `false`) em `crates/op-core/src/lib.rs`.
   - Ajustar `tick_timer()` em `crates/op-admin/src/state.rs`: ao atingir `<= 1s`, se `!allow_overtime`, cravar `remaining_seconds = 0` e `running = false`.
   - Adicionar endpoint ou parâmetro para alternar `allow_overtime`.
2. **Descoberta de Rede e URLs de Saída (`/api/system/network-info`):**
   - Criar função utilitária `get_local_ip()` em `crates/op-admin`.
   - Criar rota `GET /api/system/network-info` que retorna `{ local_ip, port, public_url, stage_url }`.
3. **Apresentação Rápida de Mídia em Memória (`/api/presentation/quick-media`):**
   - Adicionar método em `AppState` para registrar mídia temporária e selecioná-la imediatamente para exibição sem salvar no banco de dados SQLite da setlist.

### Etapa 2: Frontend - Imagens, Margens e Encaixe
1. **Tipos e Stores (`types.ts`, `displaySettings.ts`):**
   - Estender `PublicDisplaySettings` com `image_fit: 'contain' | 'cover' | 'fill' | 'center'` e `ignore_margins_for_media: boolean`.
2. **`PublicView.svelte`:**
   - Se a mídia for imagem, zerar margens de texto quando `ignore_margins_for_media` for `true`.
   - Aplicar o modo de encaixe correspondente (`object-contain`, `object-cover`, `object-fill`, etc.).
3. **`SettingsModal.svelte`:**
   - Adicionar controles na aba "Tela Pública" para definir o encaixe padrão de imagens e o toggle de ignorar margens.
4. **`BottomPanel.svelte`:**
   - Adicionar barra de ferramentas rápida na aba de Imagens com controles de Encaixe (`contain`, `cover`, `fill`) e Margens (Ativar/Desativar).

### Etapa 3: Frontend - QR Code e Modal de Compartilhamento
1. **`ShareOutputsModal.svelte`:**
   - Modal com abas "Tela Pública (/public)" e "Monitor de Palco (/stage)".
   - Renderização de QR Code nítido via biblioteca `qrcode`.
   - URL completa com IP de rede local e botão "Copiar URL" com feedback visual.
2. **Botão no Header (`AdminView.svelte`):**
   - Botão com ícone `QrCode` ao lado dos botões Iniciar Public e Stage.

### Etapa 4: Frontend - Apresentação Avulsa e Preview de Imagem
1. **`ImagePreviewModal.svelte`:**
   - Modal de alta resolução exibindo imagem, resolução calculada, proporção, formato, tamanho e pasta.
   - Botões "Apresentar ao Vivo", "Adicionar à Setlist" e "Copiar Link".
2. **Galeria de Imagens em `BottomPanel.svelte`:**
   - Botão de Apresentar (Play) chamando `/api/presentation/quick-media` (sem adicionar à setlist).
   - Botão de "+ Setlist" para inclusão manual.
   - Botão de "Preview" (Eye) abrindo o `ImagePreviewModal`.

### Etapa 5: Frontend - Stage Display com Imagens e Fontes Ampliadas
1. **`StageView.svelte`:**
   - Renderizar `<img>` nítido com rótulo quando a mídia for do tipo imagem.
   - Exibir miniatura do próximo item se também for imagem.
   - Reduzir paddings de `p-6` para `p-3 sm:p-4` no layout vertical (`modern-grid`) e outros presets.
   - Aumentar expressivamente as escalas de fonte do slide atual e do próximo slide.
2. **Preview Fidedigno do Stage no Painel Admin (`AdminView.svelte` / `StagePreviewBox.svelte`):**
   - Substituir o mock estático do preview de palco por um componente dinâmico que reflita o layout do preset ativo (`split-50-50`, `modern-grid`, `video-focus`, `horizontal-stack`), as proporções, cores de texto e borda, o timer real e o modo imagem.

### Etapa 6: Verificação, Testes e Documentação
1. Executar `bun run check` e `bun run build`.
2. Executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.
3. Atualizar `docs/README.md` e `CHANGELOG.md`.
4. Git commit e push.

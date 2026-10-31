# Spec — Exibição e Seleção de Imagens no Lower Third

**Data:** 2026-10-31  
**Status:** Aprovado  
**Autor:** Antigravity / OpenPresenter Team  
**Plano correspondente:** [`docs/plans/2026-10-31-imagens-no-lower-third.md`](../plans/2026-10-31-imagens-no-lower-third.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Intenção e Problema

Ao projetar uma imagem (aviso, cartaz ou foto) no OpenPresenter, a saída pública (`/public`) exibe a imagem perfeitamente. Contudo, na saída de transmissão para o OBS Studio (`/lower-third`), a imagem não é renderizada graficamente: a tela exibe apenas o caminho bruto do arquivo (ex.: `/media/uuid-arquivo.jpg` ou `/api/media/...`), como se fosse um texto simples.

Além disso, em transmissões ao vivo de cultos e eventos com câmeras no OBS:
1. Certas imagens (ex.: cartazes de avisos gerais ou chamadas de início/fim de culto) devem poder ser exibidas em **tela cheia** (`fullscreen`) cobrindo a transmissão.
2. Outras vezes, a igreja deseja que a imagem seja apresentada como um **banner ajustado no terço inferior** (`lower_third`), sem cobrir o rosto do pregador ou a transmissão da câmera.
3. Há casos em que a arte do telão é vertical ou muito pesada, e o operador deseja vincular uma **imagem alternativa específica para o Lower Third** (ex.: uma faixa horizontal 1920x250).

Esta especificação define o suporte completo a imagens na saída de Lower Third e as opções de controle correspondentes.

---

## 2. Escopo

### Dentro do escopo
- Renderização visual de imagens (`<img>`) em [`LowerThirdView.svelte`](../../frontend/src/lib/views/LowerThirdView.svelte) quando a mídia ativa for do tipo imagem (`kind === 'image'`) ou o slide contiver URL de imagem.
- Opções configuráveis em [`LowerThirdSettings`](../../frontend/src/lib/types.ts) e aba "Lower Third" de [`SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte):
  - `image_display_mode`: `'fullscreen'` (tela cheia cobrindo o OBS), `'lower_third'` (banner ajustado no terço inferior) ou `'hide'` (ocultar na transmissão).
  - `image_fit`: `'contain'` (preservar proporções) ou `'cover'` (preenchimento completo).
- Campo `lower_image_url` em `MediaItem` e `lower_content` em `Slide` no backend (`crates/op-core`) e frontend (`frontend/src/lib/types.ts`).
- Opção no menu de contexto dos itens da setlist em [`AdminView.svelte`](../../frontend/src/lib/views/AdminView.svelte) para vincular/remover uma imagem alternativa dedicada para o Lower Third.
- Atualização da prévia interativa 16:9 em [`SettingsModal.svelte`](../../frontend/src/lib/components/SettingsModal.svelte) suportando simulação de imagem.

### Fora do escopo
- Edição ou corte manual de imagens dentro do app (cropper integrado).

---

## 3. Decisões Arquiteturais e Modelo de Dados

### 3.1 Configurações de Lower Third (`LowerThirdSettings`)
```typescript
export interface LowerThirdSettings {
  // ... campos existentes ...
  image_display_mode: 'fullscreen' | 'lower_third' | 'hide';
  image_fit: 'contain' | 'cover';
}
```

- **`fullscreen`:** A imagem é renderizada com resolução total, centralizada na tela do Lower Third com fundo transparente ou escurecido suave, ideal para cartazes de abertura, encerramento ou avisos solenes.
- **`lower_third`:** A imagem é renderizada dentro do card/barra do terço inferior com altura proporcional (ex.: max-height 120px a 160px), agindo como uma tarja/banner televisivo.
- **`hide`:** A imagem é ignorada na saída do OBS, mantendo a transmissão do vídeo limpa enquanto a imagem é exibida no telão da igreja.

### 3.2 Imagem Alternativa para o Lower Third (`lower_image_url`)
Em `MediaItem`:
- `lower_image_url: Option<String>` (com `#[serde(default)]`).
- Se `lower_image_url` estiver definido no item, o Lower Third prioriza essa imagem em vez da imagem principal da Projeção Pública.

---

## 4. Critérios de Aceite

1. Ao projetar uma imagem na setlist, a saída `/lower-third` nunca deve exibir o caminho do arquivo como texto.
2. No modo `fullscreen`, a imagem é renderizada em tela cheia na saída `/lower-third`.
3. No modo `lower_third`, a imagem é renderizada harmoniosamente como banner na área do terço inferior.
4. No modo `hide`, nada é exibido no `/lower-third` durante a projeção da imagem.
5. O operador pode clicar com o botão direito em uma imagem na setlist e escolher "Selecionar Imagem para Lower Third...", vinculando uma arte horizontal alternativa.
6. A prévia do Lower Third em Configurações permite alternar e visualizar o comportamento de imagem.
7. Todos os testes passam em `cargo test --workspace` e compilação do frontend sem erros.

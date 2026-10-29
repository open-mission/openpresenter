# Especificação Técnica: Ajustes de Imagens, QR Code de Saídas, Timer sem Overtime Indesejado e Otimização do Monitor de Palco

- **Data:** 29/10/2026
- **Autor:** Antigravity / DeepMind Pair Programming
- **Status:** Aprovada
- **Contexto:** Solicitação de refinamentos operacionais essenciais para cultos e eventos ao vivo.

---

## 1. Intenção e Problema

Durante a operação prática do OpenPresenter em eventos e cultos ao vivo, foram identificados seis pontos fundamentais de aprimoramento:

1. **Margens e Encaixe de Imagens na Projeção Pública (`/public`):**
   - Ao projetar uma imagem em 16:9, margens configuradas para textos eram aplicadas ao redor da imagem, gerando bordas pretas indesejadas mesmo em telas 16:9.
   - Necessidade de alternar rapidamente se imagens respeitam ou ignoram margens (Edge-to-Edge) e definir o modo de encaixe (`contain`, `cover`, `fill`, `center`), tanto nas configurações globais quanto em controles rápidos na aba de mídias.

2. **Compartilhamento de Saídas via QR Code e URLs de Rede (`/public` e `/stage`):**
   - Operadores, músicos e equipe técnica precisam abrir o monitor de palco em tablets e celulares na rede local da igreja sem precisar digitar IPs e portas manualmente.
   - Necessidade de um modal de compartilhamento com QR Code escaneável, URL direta de rede local e botão de cópia rápida.

3. **Comportamento do Cronômetro / Timer Global ao Chegar a 00:00:**
   - O timer não parava ao atingir zero, continuando a rodar indefinidamente em valores negativos.
   - O comportamento padrão para contagens regressivas deve ser parar imediatamente em 00:00 (`running = false`), mantendo o recurso de contagem negativa apenas se o operador ativar explicitamente a opção `allow_overtime`.

4. **Apresentação Rápida de Imagens sem Poluição da Setlist & Modal de Prévia:**
   - Clicar em "Apresentar" em uma imagem da galeria forçava a inclusão automática da imagem na setlist do culto. O operador precisa poder projetar uma imagem avulsa instantaneamente sem adicioná-la à setlist salva.
   - Adição de botão de preview com modal dedicado exibindo a imagem ampliada e seus metadados (resolução, proporção, formato, tamanho e pasta).

5. **Exibição Correta de Imagens no Monitor de Palco (`Stage Display`):**
   - O monitor de palco exibia o caminho textual do arquivo (ex: `/media/images/banner.jpg`) em vez de renderizar a imagem visual para os integrantes do palco.
   - O layout do Stage deve detectar quando a mídia for do tipo imagem e renderizar a imagem propriamente dita, com identificação clara e miniatura do próximo item.

6. **Tipografia Gigante e Otimização de Espaço no Monitor de Palco:**
   - Em distâncias de 10 a 20 metros, os textos no palco (especialmente no preset vertical onde o próximo fica embaixo) estavam pequenos com paddings excessivos.
   - Reduzir paddings desnecessários e maximizar a escala das fontes para leitura confortável à distância.

7. **Fidelidade do Preview do Stage no Painel Administrativo (`AdminView.svelte`):**
   - A miniatura de preview do Stage na barra lateral direita do operador continha um mock estático com textos cortados, não refletindo o preset ativo (`split-50-50`, `modern-grid`, etc.), nem as cores configuradas, proporções, mensagens de palco ou projeção de imagens.
   - O preview do Stage deve ser sincronizado com as configurações reais de `stageSettingsStore` e o estado ao vivo da apresentação.

---

## 2. Escopo

### Backend Rust (`op-core`, `op-admin`):
- `TimerState`: adicionar campo `allow_overtime: bool` (padrão `false`).
- `tick_timer()`: se `remaining_seconds <= 1` e `!allow_overtime`, cravar em 0 e pausar (`running = false`).
- Endpoint `GET /api/system/network-info`: detectar IP local ativo da máquina na rede via consulta de rota de socket local UDP e retornar porta e URLs completas para `/public` e `/stage`.
- Endpoint `POST /api/presentation/quick-media`: registrar mídia rápida em memória e apresentá-la ao vivo sem persistir na tabela de setlist do SQLite.

### Frontend Svelte:
- `PublicDisplaySettings`: adicionar `image_fit` (`'contain' | 'cover' | 'fill' | 'center'`) e `ignore_margins_for_media: boolean`.
- `PublicView.svelte`: se `activeMedia?.kind === 'image'`, obedecer `ignore_margins_for_media` e aplicar a classe de encaixe correspondente (`object-contain`, `object-cover`, etc.).
- `ShareOutputsModal.svelte`: modal de compartilhamento com QR Code via biblioteca SVG nativa `qrcode`, abas Public e Stage, exibição do link de rede local e botão de cópia.
- Botão "Compartilhar Saídas" no cabeçalho do operador (`AdminView.svelte`).
- `BottomPanel.svelte`:
  - Botão de "Apresentar Rápido" (não adiciona à setlist).
  - Botão "+ Setlist" explícito.
  - Botão de "Preview / Visualizar" abrindo `ImagePreviewModal.svelte`.
  - Barra de controle de encaixe e margens de imagem na aba de Imagens.
- `ImagePreviewModal.svelte`: modal com visualização ampliada, metadados (dimensões, aspecto, peso, pasta) e botões de ação ("Apresentar", "Adicionar à Setlist", "Copiar Link").
- `StageView.svelte`:
  - Renderizar `<img>` com `object-contain` quando `activeMedia?.kind === 'image'` e miniaturas no próximo slide.
  - Otimização do espaçamento (redução de paddings `p-6` para `p-3`) e aumento expressivo das fontes no slide atual e próximo.

---

## 3. Critérios de Aceite

1. **Apresentação de Imagens 16:9:**
   - Com `ignore_margins_for_media = true` (padrão), imagens 16:9 preenchem a tela pública sem bordas pretas adicionais.
   - O operador pode alternar na aba de imagens entre Conter, Preencher (Cover) e Esticar, e ligar/desligar margens.
2. **QR Code e URLs de Rede:**
   - O modal de compartilhamento abre pelo header e exibe QR Code escaneável com o IP local da máquina (ex: `http://192.168.x.x:8080/stage`).
   - Botão "Copiar URL" copia para a área de transferência com feedback.
3. **Timer no Zero:**
   - Ao atingir `00:00`, o timer para automaticamente a contagem por padrão.
   - Caso `allow_overtime` seja habilitado pelo operador, a contagem continua negativa em vermelho pulsante.
4. **Mídia Avulsa & Modal de Preview:**
   - Ao clicar em "Apresentar" na galeria, a imagem vai para o telão e stage instantaneamente sem ser adicionada à lista da setlist.
   - O botão de Preview abre o modal com imagem grande e metadados.
5. **Stage com Imagens e Texto Ampliado:**
   - O Stage exibe a imagem renderizada com rótulo, sem exibir caminhos de arquivo em texto puro.
   - O texto dos slides (atual e próximo) no layout vertical está legível de longe com padding reduzido.
6. **Preview Fidedigno do Stage no Painel do Operador:**
   - O preview de retorno na sidebar direita do Admin reproduz o preset, as proporções, cores e status do Stage real.
7. **Qualidade:**
   - `bun run check && bun run build` sem erros.
   - `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` 100% aprovados.

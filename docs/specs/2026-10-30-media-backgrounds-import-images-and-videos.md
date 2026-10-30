# Especificação: Importação de Fundos de Imagens e Vídeos na Aba de Mídias > Fundos

## Contexto e Motivação
Na projeção de cultos e eventos, equipes de mídia frequentemente necessitam projetar vídeos em loop contínuo (motion backgrounds) ou imagens temáticas específicas como plano de fundo de letras de louvor ou da tela de descanso. Anteriormente, a aba de Mídias > Fundos exibia apenas presets nativos e arquivos pré-existentes na pasta `wallpapers`, sem um botão de importação direta de arquivos e sem suporte a arquivos de vídeo (.mp4, .webm, .mov) para planos de fundo.

## Escopo
1. **Suporte a Vídeos em Planos de Fundo (`op-admin` & `backgrounds.ts`):**
   - Estender `WallpaperItem` no backend com `is_video: bool`.
   - Atualizar `list_wallpapers` para varrer e listar extensões de vídeo (`.mp4`, `.webm`, `.mov`, `.mkv`) além de imagens (`.jpg`, `.jpeg`, `.png`, `.webp`, `.svg`).
   - Atualizar `resolveBackground` em `backgrounds.ts` para identificar extensões de vídeo e expor `isVideo: true`.
2. **Reprodução Contínua na Projeção e Preview (`PublicView.svelte` & `PublicPreviewBox.svelte`):**
   - Quando o fundo ativo for um vídeo (`resolvedBg.isVideo`), renderizar a tag `<video>` com reprodução contínua em loop, sem áudio (`muted`), `autoplay` e `playsinline`.
3. **Interface de Importação na Aba de Mídias > Fundos (`BottomPanel.svelte`):**
   - Adicionar botão **Importar Fundo** com ícone de upload na barra superior da aba Fundos.
   - Suporte a seleção de arquivos de imagens e vídeos (`image/*`, `.mp4`, `.webm`, `.mov`).
   - Leitura e envio para `/api/media/upload`, persistindo na pasta de papéis de parede/fundos locais e recarregando a listagem dinamicamente.
   - Exibir badge indicador `<Film /> Vídeo` e mini-player com loop nos cartões de vídeo da galeria.
   - Refletir compatibilidade no modal seletor de fundos (`BackgroundSelectorModal.svelte`).

## Decisões Técnicas
- **Formato de Transmissão:** O endpoint `/api/media/upload` existente aceita upload em base64 e já possui limite de payload configurado para 128 MB via `DefaultBodyLimit::max(128 * 1024 * 1024)`.
- **Streaming de Mídia:** O endpoint `/api/media/files/{subpath}` utiliza `mime_guess` que automaticamente fornece `video/mp4`, `video/webm`, etc. com headers de cache adequados.
- **Desempenho no Navegador:** Tags `<video>` de background no operador e na tela pública utilizam propriedades `muted`, `loop`, `playsinline` e `object-cover` para aceleração por hardware nativa sem sobrecarga de CPU.

## Critérios de Aceite
- O operador pode clicar em "Importar Fundo" na aba Fundos e enviar tanto imagens quanto vídeos (.mp4, .webm, .mov).
- O arquivo importado surge imediatamente na galeria de fundos.
- Ao clicar em "Apresentar" ou "Usar nas Letras" para um fundo de vídeo, o vídeo roda em loop na tela `/public` e no preview do operador sob o texto da projeção.
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

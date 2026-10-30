# Especificação: Customização de CSS e Cor de Texto por Fundo de Letras nas Configurações

## Contexto e Motivação
Na projeção de músicas e versículos bíblicos, diferentes planos de fundo (gradientes escuros, fundos cósmicos, tons dourados ou temas mais claros) exigem contraste adequado para garantir legibilidade impecável para o público. Atualmente, o texto da projeção pública adota cor branca fixa e não havia um sistema nas configurações que permitisse ao operador ajustar o CSS do fundo ou associar uma cor de texto específica para cada fundo, nem cadastrar novos fundos com estilos e cores próprias.

## Escopo
1. **Modelo e Armazenamento de Estilos de Fundo (`backgrounds.ts` e `displaySettings.ts`):**
   - Estender `BackgroundPreset` e criar `BackgroundCustomConfig` com campos para `textColor` (cor do texto), `style` (CSS do plano de fundo), `name`, `animationClass` e indicador de customização `isCustom`.
   - Adicionar store reativa `backgroundStylesStore` e persistência via `app_settings` (`background_styles_config`), com sincronização em tempo real via WebSocket (`handleSettingBroadcast`).
   - Atualizar `resolveBackground` para extrair e fornecer o `textColor` configurado para o fundo ativo.
2. **Aba de Configuração "Fundos de Letras & CSS" (`SettingsModal.svelte`):**
   - Adicionar aba dedicada com navegação por categorias (Gradientes, Mesh, Animados, Customizados).
   - Painel de edição com simulação da projeção 16:9 em tempo real.
   - Ajuste de **CSS do Plano de Fundo** (estilo `background: ...`).
   - Ajuste de **Cor do Texto** com seletor de cores, código hexadecimal e paleta de atalhos rápidos (Branco, Dourado, Menta, Azul Gelo, etc.).
   - Ação de **+ Novo Fundo Customizado** para criar presets personalizados ilimitados.
   - Ações de **Definir como Fundo Padrão de Letras**, **Restaurar Padrão** e **Excluir Fundo Customizado**.
3. **Aplicação na Projeção Pública (`PublicView.svelte` e `PublicPreviewBox.svelte`):**
   - O texto das letras e escrituras utiliza a cor configurada para o fundo ativo (`style="color: {activeTextColor};"`).
   - A barra de referência bíblica no rodapé harmoniza com a mesma cor do texto.

## Decisões Técnicas
- **Persistência Dinâmica Zero-Migração:** As customizações e novos fundos são salvos em JSON na tabela `app_settings` sob a chave `background_styles_config`. Não requer migração no schema SQLite.
- **Sincronização em Tempo Real:** Toda alteração é transmitida via `WsMessage::SettingUpdated` e refletida instantaneamente na tela `/public` sem necessidade de recarregar o navegador.

## Critérios de Aceite
- O operador pode acessar a aba "Fundos de Letras & CSS" nas Configurações, visualizar os fundos existentes e ajustar o CSS e a cor de texto de qualquer fundo.
- O operador pode cadastrar novos fundos customizados com nome, CSS e cor de texto próprios.
- Na tela `/public` e no preview do operador, o texto projeta respeitando a cor configurada para o fundo em exibição.
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

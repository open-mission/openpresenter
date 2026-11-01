# Spec — Context Menu e Exclusão de Papéis de Parede Locais no Painel de Fundos

**Data:** 2026-11-01  
**Status:** Aprovado  
**Escopo:** `op-admin`, `frontend` (`BottomPanel.svelte`, `routes.rs`), documentação  
**Plano correspondente:** [`docs/plans/2026-11-01-context-menu-e-exclusao-de-fundos.md`](../plans/2026-11-01-context-menu-e-exclusao-de-fundos.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Contexto e Motivação

Na aba "Fundos & Gradientes" do painel inferior de mídias (`BottomPanel.svelte`), o operador pode navegar entre presets CSS (gradientes, mesh, animados) e papéis de parede locais importados (imagens e vídeos salvos em `media/wallpapers`).

No entanto:
1. **Impossibilidade de Exclusão:** Os usuários conseguiam importar papéis de parede locais via botão "Importar Fundo" (`POST /api/media/upload`), mas não havia nenhuma rota ou mecanismo na interface para excluir ou remover arquivos de papéis de parede locais indesejados ou antigos.
2. **Ausência de Menu de Contexto:** Ao clicar com o botão direito nos cartões de fundos locais, nos presets CSS ou na grade da aba, o navegador abria o menu padrão do browser, sem oferecer opções úteis contextuais para o operador de multimídia da igreja.

---

## 2. Escopo

### 2.1 Em Escopo

1. **Endpoint Backend para Remoção de Papéis de Parede (`crates/op-admin`):**
   - Rota `DELETE /api/media/wallpapers/{filename}` em `routes.rs`.
   - Sanitização de caminho com `Path::file_name()` para prevenir path traversal.
   - Proteção de papéis de parede padrão do sistema (`dark-ambient.svg`, `worship-glow.svg`, `cross-subtle.svg`), impedindo sua exclusão.
   - Remoção assíncrona do arquivo em disco via `tokio::fs::remove_file`.
   - Se o papel de parede excluído estiver ativo na projeção atual (`wallpaper_path` ou `lyrics_background_path`), redefinir de forma segura para o fundo padrão do sistema e notificar as telas conectadas via WebSocket (`WsMessage::StateUpdated`).
   - Retorno de status `204 No Content` em caso de sucesso, `404 Not Found` se não existir, `403 Forbidden` se for padrão do sistema, e `400 Bad Request` em caso de nome inválido.
   - Teste unitário e de integração para o novo endpoint.

2. **Menu de Contexto e Ação de Exclusão no Frontend (`BottomPanel.svelte`):**
   - **Menu de Contexto nos Papéis de Parede Locais (`handleLocalWallpaperContextMenu`):**
     - Rótulo com o nome do arquivo.
     - ▶️ *Usar como Fundo das Letras (Ao Vivo)* (`applyBackground(wall.url)`).
     - 🖥️ *Definir como Papel de Parede da Tela de Repouso* (`setPresentationWallpaper(wall.url)`).
     - 📋 *Copiar URL do Arquivo*.
     - 🗑️ *Excluir Papel de Parede Local* (se `!wall.is_default`) com diálogo de confirmação amigável e recarregamento automático da lista via `loadWallpapers()`.
   - **Botão de Exclusão Rápida no Cartão:**
     - Ícone de lixeira no rodapé de cada cartão de papel de parede local (quando não for default), para remoção com 1 clique confirmada.
   - **Menu de Contexto nos Presets CSS (`handlePresetContextMenu`):**
     - Rótulo com o nome do preset.
     - ▶️ *Usar como Fundo das Letras (Ao Vivo)*.
     - 🖥️ *Definir como Papel de Parede de Repouso*.
     - 📋 *Copiar Código CSS do Fundo*.
   - **Menu de Contexto na Área Vazia da Grade de Fundos (`handleBackgroundsEmptyContextMenu`):**
     - 📥 *Importar Novo Fundo (Imagem / Vídeo)...*
     - 🔄 *Recarregar Fundos Locais*
     - ❌ *Remover Fundo Ativo das Letras* (limpar fundo para preto/transparente).

### 2.2 Fora de Escopo
- Alterações em presets estáticos pré-compilados de gradientes CSS (apenas fundos locais armazenados em disco podem ser deletados).

---

## 3. Critérios de Aceite

1. `DELETE /api/media/wallpapers/{filename}` remove o arquivo local da pasta `media/wallpapers/` com sucesso e retorna `204 No Content`.
2. Papéis de parede padrão do sistema são protegidos e retornam `403 Forbidden` caso haja tentativa de exclusão.
3. Se o papel de parede excluído estiver em uso na projeção, o sistema comuta automaticamente para o papel de parede padrão sem interrupção de serviço.
4. Clicar com o botão direito em um cartão de papel de parede local abre o menu de contexto com a opção de excluir e definir como fundo.
5. Clicar com o botão direito em presets CSS ou na área vazia da aba de fundos abre seus respectivos menus de contexto sem exibir o menu do navegador.
6. A lista de papéis de parede locais atualiza imediatamente após a exclusão com notificação toast de sucesso.
7. Todos os portões de qualidade passam: `bun run build`, `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.

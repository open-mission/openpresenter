# Especificação — Inserção no Cursor e Menus de Contexto no Editor de Músicas e Aba Outros

**Data:** 2026-11-01  
**Status:** Aprovado  
**Autores:** OpenPresenter Core Team  
**Plano correspondente:** [`docs/plans/2026-11-01-context-menu-editor-musicas-e-aba-outros.md`](../plans/2026-11-01-context-menu-editor-musicas-e-aba-outros.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Contexto e Motivação

1. **Inserção no Editor de Músicas e Textos (`SongEditorModal.svelte`):**
   - Ao clicar nos atalhos de seção (`[Verso 1]`, `[Refrão]`, etc.), a tag era anexada sempre ao final do texto via concatenação (`lyricsRaw += textToInsert`), ignorando onde o operador posicionava o cursor ou selecionava texto no `textarea`.
   - A barra horizontal de botões de atalhos consumia espaço vertical permanente sobre a área de digitação.
   - O operador desejava uma experiência mais ergonômica: poder clicar com o botão direito nos cards de prévia de slides em 16:9 para atribuir ou alterar a seção daquele slide (ex.: definir como Refrão, Ponte, Verso), além de inserir no cursor no texto.

2. **Aba "Outros" no Painel Inferior (`BottomPanel.svelte`):**
   - Ao clicar com o botão direito nos itens ou na área da nova aba "Outros", o navegador exibia o menu de contexto nativo do browser (Voltar, Recarregar, Inspecionar elemento), em vez do menu de contexto customizado da aplicação.

---

## 2. Escopo

### 2.1 Em Escopo
1. **Correção da Inserção no Cursor (`SongEditorModal.svelte`):**
   - Inserir a tag de seção (`## NomeDaSecao`) exatamente na posição do cursor (`selectionStart`/`selectionEnd`) do `textarea`.
   - Ajustar quebras de linha limpas antes e depois da tag.
   - Posicionar o cursor imediatamente após a tag inserida com foco devolvido ao editor.
2. **Menu de Contexto nos Slides de Prévia do Editor:**
   - Adicionar menu de contexto via [`ContextMenu.svelte`](../../frontend/src/lib/components/ContextMenu.svelte) ao clicar com o botão direito em qualquer card de slide na prévia do editor.
   - Opções para definir/alterar a seção daquele slide:
     - 🏷️ `Verso 1`, `Verso 2`, `Verso 3`, `Pré-Refrão`, `Refrão`, `Ponte`, `Intro`, `Instrumental`, `Final` (modo música).
     - 🏷️ `Boas-vindas`, `Avisos`, `Campanha`, `Informações`, `Encerramento` (modo texto).
   - Opções de duplicar slide e excluir slide.
   - Remover a barra estática volumosa de botões sobre o editor, mantendo um botão compacto suspenso `[+ Seção ▾]` na barra de ferramentas superior para conveniência.
3. **Menu de Contexto Customizado na Aba "Outros" (`BottomPanel.svelte`):**
   - Substituir o menu padrão do navegador em cards da lista de itens com opções:
     - ▶️ Apresentar ao Vivo
     - ➕ Adicionar à Setlist
     - ✏️ Editar Item
     - 📋 Duplicar Item no Catálogo
     - 🗑️ Excluir Item do Catálogo
   - Menu de contexto na prévia 16:9 dos slides:
     - ▶️ Apresentar Este Slide ao Vivo
     - ➕ Adicionar Item à Setlist
     - ✏️ Editar Item
   - Menu de contexto em área vazia da aba:
     - 📅 Nova Agenda...
     - 📝 Novo Texto...
     - 📢 Novo Aviso...
     - 🔖 Nova Seção / Divisor...
     - 🔄 Recarregar Catálogo
     - 📥 Importar Itens da Setlist

### 2.2 Fora de Escopo
- Mudanças no parser sintático em `songParser.ts` (já suporta as tags `## Nome`).

---

## 3. Critérios de Aceite

1. Clicar em um atalho de inserção insere a seção na posição exata do cursor no editor de texto, e não no final.
2. Clicar com o botão direito em um slide da prévia no editor abre o menu de contexto com opções para alterar a seção daquele slide, duplicar ou excluir.
3. A barra permanente de botões de seção no editor de música é removida/substituída por interface limpa com menu de contexto e seletor compacto na barra de ferramentas.
4. Clicar com o botão direito nos itens ou na área da aba "Outros" abre o menu de contexto customizado, sem abrir o menu nativo do navegador.
5. Todos os portões de qualidade passam: `bun run build`, `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.

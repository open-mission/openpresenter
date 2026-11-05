# Spec — Refinamentos de Interface: Colapso do Controle de Saídas, Dropdown Adicionar em Outros e Ajustes no Command Palette

## 1. Intenção

Aprimorar a ergonomia, o aproveitamento de espaço e a clareza visual da interface principal do operador (`AdminView.svelte`, `BottomPanel.svelte` e `CommandPalette.svelte`), atendendo aos feedbacks de design:
1. Permitir recolher/expandir a seção "Controle de Saídas" na barra lateral de previews.
2. Agrupar os botões de criação da aba Outros (`+ Agenda`, `+ Texto`, `+ Aviso` e importação de setlist) em um único botão elegante `+ Adicionar ▾` com menu suspenso (dropdown).
3. Exibir apenas a abreviação da versão da Bíblia (ex: `ARA`, `AS`, `NTLH`) no seletor do Command Palette, reduzindo a largura excessiva.
4. Eliminar a quebra de linha no botão da aba de Bíblia no Command Palette com `whitespace-nowrap` e rótulo conciso.

## 2. Contexto e Problemas Identificados

1. **Seção "Controle de Saídas":** A barra lateral de previews do operador exibe os botões de controle de saída (Ao Vivo, Parar, Blackout, Limpar, Fundo). Quando o operador deseja focar apenas nos previews de tela (Public e Stage) e no timer, esses botões ocupavam espaço vertical fixo sem opção de colapso individual da seção.
2. **Barra de Ações da Aba Outros:** Havia 4 a 5 botões individuais dispostos horizontalmente (`Recarregar`, `Importar da Setlist`, `+ Agenda`, `+ Texto`, `+ Aviso`), tornando a barra poluída e propensa a quebras em resoluções menores.
3. **Seletor de Versão Bíblica no Command Palette:** O `<select>` exibia o nome completo da versão (`ARA - Almeida Revista e Atualizada`), consumindo cerca de `280px` de largura.
4. **Quebra de Linha na Aba Bíblia do Command Palette:** Devido à largura excessiva do seletor da Bíblia e à falta de `whitespace-nowrap`, o botão "Bíblia Sagrada" quebrava em duas linhas ("Bíblia\nSagrada"), desalinhando a barra de abas e aumentando a altura do botão para `37px`.

## 3. Escopo e Soluções

1. **Colapso da Seção "Controle de Saídas" (`AdminView.svelte`):**
   - Adicionar estado reativo `isOutputControlsCollapsed` com persistência em `localStorage` (`output_controls_collapsed`).
   - Tornar o cabeçalho interativo com ícone `ChevronDown` animado (rotacionando 90 graus quando recolhido) e tooltip explicativo.
   - Renderizar o grid de botões condicionalmente com animação suave de transição.

2. **Menu Dropdown "Adicionar" na Aba Outros (`BottomPanel.svelte`):**
   - Substituir os botões isolados `+ Agenda`, `+ Texto` e `+ Aviso` por um botão único `+ Adicionar ▾` (`Button`).
   - Ao clicar, abrir menu dropdown suspenso contendo as opções:
     - **Nova Programação (Agenda)** com ícone `CalendarDays`
     - **Novo Texto Livre** com ícone `FileText`
     - **Novo Aviso / Comunicado** com ícone `Bookmark`
     - **Novo Divisor de Seção** com ícone `Heading`
     - Divisor visual
     - **Importar da Setlist Ativa** com ícone `Upload` (com tooltip explicativo de que salva itens criados na setlist para o catálogo persistente).
   - Fechamento automático ao clicar fora ou ao selecionar uma opção.

3. **Abreviação no Seletor de Bíblias (`CommandPalette.svelte`):**
   - Alterar o texto das opções do `<select>` para `{b.abbreviation || b.name}`, mantendo `title="{b.abbreviation} - {b.name}"` para visualização acessível ao passar o cursor.
   - Reduzir drasticamente a largura do select de ~280px para ~60-70px.

4. **Ajuste de Quebra de Linha no Botão da Bíblia (`CommandPalette.svelte`):**
   - Aplicar `whitespace-nowrap shrink-0` no botão da aba de Bíblia e nos demais botões de abas.
   - Ajustar o rótulo para `Bíblia` (acompanhado da contagem quando houver busca ativa), padronizando com `Músicas` e `Online`.

## 4. Critérios de Aceite

1. Clicar no cabeçalho ou ícone de "Controle de Saídas" deve recolher e expandir o grid de botões de controle de saída, lembrando a preferência do operador.
2. Na aba Outros do painel inferior, os botões de criação devem estar consolidados em um botão `+ Adicionar ▾` que abre menu dropdown com todas as opções de criação e importação.
3. No Command Palette (Ctrl+K / Cmd+K), o seletor de versão da Bíblia deve exibir a sigla/abreviação concisa (ex: `ARA`).
4. A aba de Bíblia no Command Palette deve permanecer estritamente em uma única linha sem quebras ou aumento vertical indevido.
5. Todos os testes e portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test`, `bun run build`) devem passar com sucesso.

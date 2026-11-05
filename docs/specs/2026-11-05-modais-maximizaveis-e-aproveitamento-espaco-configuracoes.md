# Especificação: Modais Maximizáveis e Aproveitamento de 100% do Espaço em Configurações

**Data:** 2026-11-05  
**Status:** Aprovado  
**Escopo:** Frontend (`frontend/src/lib/components/*`)

---

## 1. Contexto e Motivação

Durante a operação de cultos e eventos ao vivo, operadores de projeção e voluntários de mídia frequentemente necessitam de mais espaço útil de tela para:
1. Compor letras longas de músicas e apresentações de texto no `SongEditorModal`.
2. Visualizar escalas complexas de múltiplos voluntários e departamentos no `TeamsModal` (tanto no fluxo interativo Svelte Flow quanto na grade de cartões).
3. Montar cronogramas extensos de cultos no `ScheduleEditorModal`.
4. Digitar avisos e comunicados no `QuickItemModal`.
5. Visualizar grades ricas de papéis de parede e galerias de imagens em `BackgroundSelectorModal` e `ImageSelectorModal`.
6. Configurar telas, saídas, diagnósticos e atalhos no `SettingsModal`.

Adicionalmente, na tela de configurações (`SettingsModal`), diversas seções (em especial a aba "Sobre & Updates", "Atalhos Rápidos", "Aparência", etc.) utilizavam limitadores rígidos de largura (`max-w-xl`, `max-w-2xl`), deixando vastas áreas vazias e subutilizadas à direita quando visualizadas em telas médias, grandes ou ultrawide.

---

## 2. Objetivos e Requisitos

1. **Botão de Maximizar / Restaurar (Fullscreen Toggle):**
   - Disponibilizar nos cabeçalhos de todos os modais principais (`SettingsModal`, `TeamsModal`, `SongEditorModal`, `ScheduleEditorModal`, `QuickItemModal`, `SetlistModal`, `BackgroundSelectorModal`, `ImageSelectorModal`) um botão de alternância com ícones `Maximize2` e `Minimize2` da biblioteca `lucide-svelte`.
   - Tooltips amigáveis: *"Maximizar"* e *"Restaurar tamanho"*.
   - Transição visual suave sem travamentos ou quebra de layout.

2. **Comportamento em Modo Maximizado:**
   - O backdrop remove espaçamentos externos (`p-0` ao invés de `p-3 sm:p-6`).
   - O container do modal passa a ocupar `w-full h-full max-w-none max-h-none rounded-none border-0` (ou `w-screen h-screen`), preenchendo a janela do navegador.
   - O layout interno (colunas de edição, listas de slides, previews, canvas de fluxo e abas) se expande de ponta a ponta.

3. **Aproveitamento de Espaço no Modal de Configurações (`SettingsModal`):**
   - Remover as restrições artificiais `max-w-xl` e `max-w-2xl` nas seções de configuração.
   - **Aba Sobre & Updates:** Reestruturar os blocos em um layout responsivo de múltiplas colunas (`grid grid-cols-1 lg:grid-cols-2 gap-6`) para Updates do Sistema e Documentação de APIs Offline (Scalar/Swagger UI), banner centralizado e utilitários em largura total (`w-full max-w-5xl`).
   - **Aba Atalhos Rápidos:** Reorganizar a tabela de atalhos operacionais em grid de 2 colunas (`grid grid-cols-1 md:grid-cols-2 gap-4`), permitindo consulta rápida lado a lado sem rolagem excessiva.
   - **Demais Abas:** Permitir que as opções usem `w-full max-w-4xl` ou `w-full max-w-5xl`, preservando alinhamento estético elegante e legibilidade.

---

## 3. Critérios de Aceite

- [x] Todos os modais chave possuem o botão de maximizar / restaurar no cabeçalho antes do botão fechar (`X`).
- [x] O estado maximizado expande o modal para 100% da viewport (`100vw` / `100vh`).
- [x] O botão de alternância inverte o ícone (`Maximize2` $\leftrightarrow$ `Minimize2`) e o título.
- [x] A aba "Sobre & Updates" no `SettingsModal` aproveita horizontalmente o espaço com layout moderno e harmonioso.
- [x] A aba "Atalhos Rápidos" exibe as teclas de atalho distribuídas de forma equilibrada.
- [x] Nenhuma regressão nas funcionalidades existentes (salvamento, atalhos de teclado Esc, responsividade mobile).
- [x] `npm run build` compila sem erros e o bundle é atualizado.

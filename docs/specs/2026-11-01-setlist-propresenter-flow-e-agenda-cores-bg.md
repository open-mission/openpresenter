# Especificação — Setlist em Fluxo Contínuo (Estilo ProPresenter) e Customização de Cores e Fundo da Agenda

**Data:** 2026-11-01  
**Status:** Aprovado  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Contexto e Motivação

1. **Ajustes de Customização na Agenda:**
   - Na projeção pública e no monitor de retorno, o usuário necessita personalizar independentemente as **cores do dia da semana** e do **horário**, além de escolher se deseja **exibir ou ocultar o fundo (background)** das pílulas de destaque e do container de conteúdo.

2. **Visualização da Setlist em Fluxo Contínuo (Estilo ProPresenter / Show Flow):**
   - Atualmente, a setlist no painel administrativo opera exclusivamente no modo focado: uma barra lateral à esquerda exibe os itens cadastrados e a coluna central exibe apenas os slides do item selecionado.
   - Usuários acostumados a ferramentas profissionais como ProPresenter demandam visualizar **toda a ordem do culto de forma contínua** na tela central, com seções estruturadas, todos os itens e seus slides visíveis sequencialmente.
   - Principais necessidades:
     - Alternância configurável entre **Modo Focado (Item)** e **Modo Fluxo Contínuo (ProPresenter)**.
     - Agrupamento natural dos itens por seções delimitadoras (`kind === 'header'`).
     - Capacidade de **colapsar/expandir** seções e itens individuais.
     - Recurso de **Marcar como Concluído / Apresentado** para seções e itens, que recolhe o elemento para formato compacto atenuado, limpando a área de trabalho visual do operador.
     - Recurso de **Apresentar Seção Completa**, agregando todos os slides de todas as mídias daquela seção em uma única apresentação sequencial sem interrupções (ex: bloco ininterrupto de louvores ou carrossel de imagens de dízimos/ofertas).

---

## 2. Escopo

### No Escopo
- **Customizações da Agenda:**
  - Tipos e configurações: `schedule_day_color`, `schedule_time_color`, `schedule_badge_bg` em `PublicDisplaySettings` e `StageDisplaySettings`.
  - Renderização na Tela Pública (`PublicView.svelte`) e preview (`PublicPreviewBox.svelte`).
  - Renderização no Monitor de Palco (`StageView.svelte`) e preview (`StagePreviewBox.svelte`).
  - Controles visuais no `SettingsModal.svelte` nas seções de Agenda (Public e Stage).
- **Setlist em Fluxo Contínuo (`AdminView.svelte`):**
  - Toggle de alternância de visualização: Modo Focado vs Modo Fluxo Contínuo ProPresenter, com persistência em `localStorage`.
  - No Modo Contínuo:
    - Agrupamento sequencial automático dos itens pelas seções de cabeçalho (`kind === 'header'`).
    - Exibição de barra de seção estilizada com título, contagem de mídias/slides, botão de colapso, botão de concluir e botão "Apresentar Seção".
    - Exibição de cada item de mídia da seção com cabeçalho próprio e grade de slides correspondente (16:9).
    - Estado de colapso independente (`collapsedSectionIds`, `collapsedItemIds`).
    - Estado de marcação de conclusão (`completedSectionIds`, `completedItemIds`), colapsando automaticamente o elemento com estilo atenuado.
    - Suporte a apresentação direta ao vivo de qualquer slide com um clique/duplo clique.
  - **Apresentar Seção Contínua:**
    - Botão "Apresentar Seção" sintetiza uma super-mídia contendo todos os slides concatenados de todos os itens da seção e dispara `quickPresentMedia`, permitindo navegação fluida por todo o bloco do culto.

---

## 3. Critérios de Aceite

1. Nas configurações da Agenda (Public e Stage), o usuário pode escolher a cor do dia da semana, do horário e ativar/desativar o fundo dos badges e do box.
2. Na Tela Pública e no Palco, as cores e a visibilidade do fundo da agenda são respeitadas com tipografia legível.
3. No painel administrativo, o operador pode alternar com um clique entre o Modo Focado e o Modo Fluxo Contínuo (ProPresenter).
4. No Modo Contínuo, todas as seções e seus respectivos itens/slides aparecem verticalmente encadeados.
5. Seções e itens podem ser colapsados e expandidos manualmente ou marcados como concluídos (recolhendo para formato compacto).
6. O botão "Apresentar Seção" une todos os slides daquela seção e permite navegar continuamente entre todas as músicas ou imagens do bloco.
7. Todos os portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test`, `bun run build`) são aprovados sem erros.

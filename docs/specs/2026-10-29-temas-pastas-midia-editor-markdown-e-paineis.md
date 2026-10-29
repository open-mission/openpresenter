# Especificação: Temas Shadcn, Pastas de Mídia Externas, Editor Markdown e Painéis Flexíveis

- **Status:** Aprovado
- **Data:** 2026-10-29
- **Autor:** Antigravity

---

## 1. Intenção

Expandir o ambiente de apresentação e operação do OpenPresenter Minimal com personalização visual completa baseada nas especificações de temas do Shadcn UI (Base Colors e Accent Themes), suporte a reordenação por arrastar e soltar (drag and drop) na Setlist, colapsamento de sidebars operacionais, um painel inferior (Bottom Panel) redimensionável com abas para catálogo de músicas e mídias, gestão de pastas de arquivos locais do sistema e externas, e um editor unificado de músicas e textos/apresentações rápidas com suporte a Markdown, typeset (negrito, itálico, sublinhado, highlight) e alinhamento horizontal/vertical.

---

## 2. Escopo

### No escopo
1. **Configuração de Temas e Cores:**
   - Seleção de **Base Color** (`Neutral`, `Stone`, `Zinc`, `Mauve`, `Olive`, `Mist`, `Taupe`), alterando as variáveis de superfície, cartões e bordas.
   - Seleção de **Theme / Cor de Destaque** (`Neutral`, `Amber`, `Blue`, `Cyan`, `Emerald`, `Fuchsia`, `Green`, `Indigo`, `Lime`, `Orange`, `Pink`, `Purple`, `Rose`, `Violet`, `Yellow`), alterando `--primary`, anéis e detalhes de destaque.
   - Interface idêntica aos seletores de componentes Shadcn UI com swatch de cores.
2. **Sidebars Colapsáveis:**
   - Opção de colapsar / fechar o painel lateral da Setlist (esquerda) e o painel de Previews (direita).
   - Botões de alternância no header e nas bordas das sidebars.
3. **Reordenação Drag and Drop na Setlist:**
   - Capacidade de arrastar e soltar itens da setlist para reorganizar a ordem de apresentação.
   - Sincronização persistente com o backend via WebSocket/REST.
4. **Editor Unificado de Música e Texto com Markdown & Typeset:**
   - Modal em 95% da tela para criação e edição tanto de músicas quanto de textos livres/apresentações rápidas.
   - Barra de ferramentas (Typeset): negrito (`**`), itálico (`*`), sublinhado (`<u>`), highlight (`==` ou `<mark>`).
   - Alinhamento horizontal (Esquerda, Centro, Direita, Justificado) e vertical (Superior, Centro, Inferior).
   - Renderização rica e formatada nos slides de prévia, na tela Public e na tela Stage.
5. **Painel Inferior Colapsável e Redimensionável (Bottom Panel):**
   - Barra inferior acoplada com abas: "Músicas" e "Imagens".
   - Três estados: Minimizado (barra de ~36px), Aberto (com arrasto para redimensionamento de altura) e Maximizado (100% de altura).
   - Aba Músicas: busca rápida no banco SQLite e botão para adicionar à setlist ou apresentar.
   - Aba Imagens: galeria de miniaturas da pasta do app e de pastas externas importadas.
6. **Aba de Configurações: Arquivos e Pastas & Pastas de Mídia:**
   - Exibição destacada da pasta raiz de dados do aplicativo (`data_dir`) e caminhos derivados (`db`, `media/images`, `media/videos`, `media/wallpapers`, `fonts`) com opções de copiar caminho e abrir no Finder/Explorador nativo.
   - Cadastro de pastas locais externas do usuário via File Picker nativo do sistema operacional (`/api/system/pick-folder` ou seletor gráfico de diretórios) além de campo de digitação manual.
   - Redimensionamento e largura aprimorada da janela de configurações para evitar truncamento ou vazamento de caminhos e textos.
   - Endpoints REST para servir e listar imagens e gerenciar as pastas mapeadas.
7. **Atalhos Operacionais Dedicados Livres de Conflito:**
   - Atalhos dedicados para Blackout (`.` ou `Alt+B`), Limpar Texto (`F2`, `,` ou `Alt+C`) e Papel de Parede (`F3`, `/` ou `Alt+W`) que não conflitam com teclas de navegação rápida de partes da música (V, R, P, I, F).
8. **Integração de Imagens na Setlist:**
   - Possibilidade de adicionar imagens avulsas ou pastas completas de imagens diretamente à Setlist através do menu `+` e menu de contexto.
9. **Apresentação Rápida de Imagens e Sheet de Letra no Painel Inferior:**
   - Botão visível de apresentação rápida direta em cada cartão de imagem do painel inferior.
   - Gaveta lateral (Sheet) na aba Músicas para visualização instantânea da letra formatada completa e metadados antes de projetar ou adicionar à setlist.

### Fora do escopo
- Edição destrutiva de arquivos em pastas externas do usuário.
- Transcodificação de vídeo em tempo real no servidor backend.
- Autenticação multiusuário remota.

---

## 3. Decisões de Design e Arquitetura

1. **Separação de Base Color e Accent Theme:**
   - A *Base Color* determina a paleta neutra de fundo e cartões no modo escuro/claro.
   - O *Theme* injeta a cor de destaque primária no CSS `:root`, conferindo identidade visual flexível sem alterar a lógica dos componentes.
2. **Editor de Apresentações Rápidas:**
   - Em vez de um componente isolado e limitado, o editor de textos reaproveita a estrutura do editor de músicas em tela cheia (95%), permitindo separar slides por quebras de linha e formatar texto com precisão.
3. **Pastas Externas Locais (Zero-Copy):**
   - Pastas adicionadas pelo usuário têm seus metadados persistidos na tabela `media_folders` (SQLite). Os arquivos de imagem continuam no diretório de origem do sistema operacional, sendo lidos pelo backend e servidos com sanitização de caminho para segurança.

---

## 4. Critérios de Aceite

1. O usuário consegue alterar a *Base Color* e o *Theme* nas configurações, e as cores da interface refletem a escolha imediatamente.
2. A Setlist e o painel de Previews podem ser fechados e abertos através de botões nas sidebars e/ou header.
3. Os itens da Setlist podem ser arrastados e reordenados via mouse/drag-and-drop.
4. O editor de texto/música suporta tags de markdown (negrito, itálico, sublinhado, highlight) e alinhamentos vertical e horizontal, refletidos na prévia e nas saídas.
5. O painel inferior pode ser expandido, colapsado e redimensionado via arrasto no topo.
6. A aba "Arquivos e Pastas" exibe os caminhos do sistema e permite adicionar pastas externas cujas imagens aparecem na galeria de imagens para projeção.
7. Todos os testes em `cargo test --workspace` e `bun run check` passam sem regressões.

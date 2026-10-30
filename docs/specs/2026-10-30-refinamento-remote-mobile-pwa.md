# Spec — Refinamento do Controle Remoto Mobile e Suporte a PWA

## Contexto e Motivação

Após a validação da primeira versão do Controle Remoto Mobile (`/remote`), foram coletados feedbacks práticos de ergonomia e usabilidade via toque:
1. **Navegação Bíblica:** A visualização em linha única com rolagem horizontal tornava a seleção de livros e capítulos lenta e propensa a erros de toque. É necessária uma navegação por abas dedicadas (**Livros**, **Capítulos**, **Versículos**) com exibição em grade (*grid*) touch-friendly.
2. **Entrada de Busca e Versão:** A seleção da tradução bíblica precisa de um grupo de entrada unificado (*Input Group*) e um Drawer/Bottom Sheet no padrão Shadcn UI em vez de um `<select>` nativo básico.
3. **Ergonomia e Área Segura (Safe-Area iOS):** Em dispositivos Apple e telas com entalhe/barra de início inferior (*Home Indicator*), os elementos das barras superior e inferior ficavam colados nas bordas, e o toque na barra de navegação era interceptado pelo sistema. É essencial aplicar `env(safe-area-inset-top)` e `env(safe-area-inset-bottom)`.
4. **Header e Branding Oficial:** Aumentar a área de toque no cabeçalho e substituir textos genéricos pelo logotipo oficial do OpenPresenter Minimal com o badge "Remoto".
5. **Card de Controle ao Vivo Retrátil (Collapsible):** Permitir colapsar o card persistente do topo para maximizar o espaço útil de visualização da Bíblia, Músicas e Setlist em telas de smartphones.
6. **Estados Ativos e Feedback Visual:** Destacar estrofes de músicas e versículos bíblicos que estão em exibição ao vivo no projetor público (`live`), além de sinalizar na listagem de músicas quando um item está ao vivo.
7. **Correção do Carregamento da Setlist:** Corrigir a requisição da setlist ativa carregando o detalhe completo via `/api/setlists/{id}` para obter o array de itens.
8. **Suporte a Progressive Web App (PWA):** Disponibilizar manifesto web (`manifest.webmanifest`), ícones de alta resolução, meta tags de tela cheia e Service Worker mínimo para que o OpenPresenter (Remoto, Painel e Telas) possa ser instalado como aplicativo standalone no computador e celular (Android e iOS).

---

## Escopo

### Incluído
- Criação do manifesto web PWA (`frontend/public/manifest.webmanifest`) e Service Worker (`frontend/public/sw.js`).
- Adição de ícones PWA oficiais (192x192 e 512x512) e meta tags de standalone no `index.html`.
- Atualização do título dinâmico da página e favicon garantido na rota `/remote`.
- Correção de carregamento da Setlist em `RemoteView.svelte` buscando os itens reais em `/api/setlists/{id}`.
- Refatoração da aba Bíblia no Remoto com sub-abas (**Livros**, **Capítulos**, **Versículos**), grades touch-friendly e Drawer Shadcn para seleção de tradução bíblica.
- Implementação de *Input Group* agrupando a seleção de tradução e a busca por versículos/palavras.
- Opção de colapsar (*collapse/expand*) no Card de Controle ao Vivo do topo.
- Destaque ativo (*active state*) em versículos e estrofes de músicas projetados ao vivo.
- Ajustes de layout com suporte obrigatório a `safe-area-inset-top` e `safe-area-inset-bottom` para iOS Safari e Android.
- Aumento da área de toque (*touch targets* mínimos de 44px) no header e navegação.

### Não Incluído
- Edição de letras de músicas ou administração de banco de dados diretamente pelo celular (o Remoto é focado em operação ao vivo).

---

## Critérios de Aceite

1. **Acesso à Setlist:** Ao abrir o Remoto e tocar na aba Setlist, os itens cadastrados no culto ativo são listados corretamente sem falhas.
2. **Navegação Bíblica Fluida:**
   - A aba Bíblia permite alternar entre Livros, Capítulos e Versículos.
   - Livros e capítulos são dispostos em grade ampla com toques fáceis.
   - O toque num livro avança automaticamente para capítulos; o toque num capítulo avança para versículos.
   - Ao tocar num versículo, ele é projetado ao vivo e recebe destaque verde esmeralda.
3. **Drawer de Traduções:** O seletor de versão bíblica abre um Drawer de baixo para cima com as traduções disponíveis e seleção rápida.
4. **Card Retrátil:** O card superior pode ser encolhido com um toque, liberando espaço vertical na tela e mantendo atalhos rápidos.
5. **Respeito à Safe-Area:** No iOS e smartphones modernos, nenhum botão fica sob a Home Bar ou sobreposto pelo entalhe superior da câmera.
6. **Instalação PWA:** O navegador oferece a opção "Adicionar à Tela de Início" / "Instalar Aplicativo" com o ícone oficial e tema escuro.
7. **Portões de Qualidade:** `cargo fmt`, `clippy`, `cargo test`, `npm run check` e `npm run build` aprovados sem advertências.

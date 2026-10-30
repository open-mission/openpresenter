# Especificação: Suporte a Sub-pastas na Aba de Imagens e Varredura Hierárquica de Mídias

## Contexto e Motivação
Atualmente, ao mapear uma pasta externa ou utilizar a pasta de imagens interna do aplicativo, o OpenPresenter realiza apenas a leitura direta (`std::fs::read_dir`) na raiz da pasta mapeada. Arquivos contidos em sub-pastas não são descobertos e as sub-pastas não aparecem na árvore de navegação lateral nem na galeria, fazendo com que pastas organizadas hierarquicamente (ex.: `Culto/Avisos`, `Culto/Banners`, `Culto/Pregacao`) pareçam vazias para o usuário.

## Escopo
1. **Varredura Hierárquica no Backend (`crates/op-admin` e `crates/op-core`):**
   - Estender `MediaFolder` e a listagem de pastas para incluir sub-pastas descobertas dinamicamente no sistema de arquivos local (`subfolders: Vec<MediaSubfolder>`).
   - Atualizar a função `list_images` para varrer diretórios recursivamente (respeitando profundidade razoável, ignorando pastas ocultas), preenchendo o campo `subfolder` e construindo URLs relativas seguras para o endpoint `serve_external_file` e `serve_media_file`.
   - Garantir que `serve_external_file` atenda caminhos com sub-pastas (ex.: `file=Avisos/foto.png`) sem risco de directory traversal (`..` ou `/` inicial).
2. **Navegação e Interface no Frontend (`BottomPanel.svelte`, `ImageSelectorModal.svelte`):**
   - **Árvore de Navegação Lateral:** Pastas que possuam sub-pastas exibem indicador expansível e lista das sub-pastas com ícone e contagem de itens.
   - **Grade e Cards de Sub-pastas:** Quando o usuário seleciona uma pasta que possui sub-pastas, tiles/cards de pastas são exibidos no topo da grade da galeria, permitindo clicar para navegar diretamente para a sub-pasta.
   - **Breadcrumbs:** Navegação por migalhas de pão navegável (`Mídias > Imagens > Pasta Raiz > Sub-pasta`), permitindo retornar a qualquer nível com facilidade.
   - **Filtro de Sub-pasta:** Selecionar uma sub-pasta filtra as imagens da galeria estritamente para o diretório selecionado.
   - **Modal de Seleção de Imagens (`ImageSelectorModal.svelte`):** Suporte a filtragem por sub-pastas ao selecionar imagens para a setlist.

## Decisões Técnicas
- **Zero-Copy e Dinamismo:** Sub-pastas não precisam de migração no SQLite nem de persistência redundante; são lidas sob demanda no sistema de arquivos local, garantindo sincronia imediata quando o usuário cria ou renomeia pastas no sistema operacional.
- **Normalização de Caminhos:** No Windows e Unix, os caminhos relativos de sub-pastas são normalizados com barras normais (`/`), garantindo consistência em URLs e consultas HTTP.
- **Segurança de Servir Arquivos:** O endpoint `serve_external_file` continua validando rigorosamente a ausência de `..` e de barras iniciais antes de fazer `join` com o caminho absoluto da pasta raiz cadastrada.

## Critérios de Aceite
- Ao mapear uma pasta contendo sub-pastas com imagens, todas as sub-pastas e seus respectivos arquivos são descobertos pelo OpenPresenter.
- Na aba de Imagens do painel inferior, as sub-pastas aparecem na árvore de navegação e como cards navegáveis na galeria.
- Clicar em uma sub-pasta exibe apenas os arquivos daquela sub-pasta, com breadcrumbs atualizados.
- Imagens em sub-pastas podem ser projetadas ao vivo (duplo clique ou botão Apresentar), adicionadas à setlist e visualizadas no preview sem erros 404.
- Portões de qualidade aprovados: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm --prefix frontend run check && npm --prefix frontend run build`.

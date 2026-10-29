# Spec — Múltiplas Setlists, Context Menu, Papel de Parede e Armazenamento de Mídias

## Contexto e Motivação

O OpenPresenter evoluiu para um sistema web completo com suporte a busca local via SQLite FTS5, importação de letras da internet e controle de apresentação desacoplado. Para operação real em cultos e eventos, o operador necessita de:
1. **Organização flexível da Setlist:** capacidade de manipular itens diretamente com o botão direito (Renomear, Remover, Editar, Fixar no topo) e adicionar divisores de seção (ex: "Momento de Louvor", "Palavra", "Avisos").
2. **Múltiplas Setlists:** cadastrar e alternar entre diferentes cultos e programações (ex: "Domingo Manhã", "Domingo Noite", "Culto de Jovens"), cada qual com seu título, descrição e papel de parede exclusivo.
3. **Papel de Parede da Projeção com Ajuste:** exibir a identidade visual do evento ou papel de parede configurado na tela Public quando nenhum slide estiver ativo ou em repouso, com modos de exibição (Preencher / Cover, Ajustar / Contain, Centralizar / Center, Repetir / Repeat).
4. **Armazenamento de Mídias em Pasta do Sistema:** guardar arquivos e papéis de parede em um diretório acessível do sistema operacional (`<data_dir>/media/wallpapers/`), permitindo cópia, backup e migração simples de arquivos.
5. **Correção de Identidade Visual no Header:** eliminar a duplicação do logotipo no cabeçalho e torná-lo proeminente e nítido.
6. **Paleta Neutra Shadcn-UI:** adotar a escala Neutra (`neutral`) como padrão do aplicativo, com opção de customização na aba de aparência.

## Escopo

### No Escopo
- **Menu de Contexto nos Itens da Setlist:**
  - Clique direito em item da setlist abre menu flutuante com:
    - *Fixar no topo / Subir:* move o item para o início da setlist ativa.
    - *Editar:* abre o editor de música se for louvor, ou diálogo de edição se for texto/seção.
    - *Renomear:* altera o título exibido na setlist.
    - *Remover da Setlist:* remove o item da setlist ativa (e para projeção se estava no ar).
- **Menu de Contexto em Área Vazia e Ação do Botão `+`:**
  - Clique direito em espaço vazio da setlist abre menu com: Adicionar Música, Adicionar Seção, Adicionar Texto Livre.
  - Clique no botão `+` abre menu suspenso com as mesmas opções de criação rápida.
- **Múltiplas Setlists:**
  - Nova migração `0004_setlists.sql` no SQLite com tabela `setlists`.
  - Operações CRUD completas via API REST e WebSocket.
  - Dropdown no cabeçalho da setlist para alternar setlist ativa, criar nova e editar metadados (Título, Descrição, Papel de Parede e Modo de Ajuste).
- **Papel de Parede da Setlist no Public:**
  - Projeção do papel de parede ativo na rota `/public` em repouso e em modo `cleartext`.
  - Modos de ajuste CSS: `cover`, `contain`, `center`, `repeat`.
  - Configuração de modo padrão em `SettingsModal.svelte`.
- **Armazenamento no Sistema de Arquivos:**
  - Subpasta `<data_dir>/media/wallpapers/` criada automaticamente com papéis de parede padrão.
  - Endpoint `POST /api/media/upload` e serviço de entrega estática `GET /api/media/files/*`.
- **Correção do Logo e Paleta Neutra:**
  - Remoção de ícone duplicado no header e ajuste para altura proeminente e nítida.
  - Padrão neutro (`neutral`) adotado no CSS, com alternância de paletas em Configurações.

### Fora do Escopo
- Importação completa de Bíblias inteiras (marcado como "Em breve" no menu de adição).
- Transições de vídeo 3D aceleradas por GPU para troca de papel de parede.

## Decisões Técnicas

1. **Migração 0004:** Tabela `setlists` armazenando `items_json TEXT` para garantir que a ordenação e os divisores de seção sejam persistidos de forma atômica e resiliente.
2. **Extensão de `MediaKind`:** Adicionar `MediaKind::Header` para representar divisores visuais de seção na setlist (ex.: "Momento de Louvor").
3. **Serviço de Mídia Local:** Axum serve `<data_dir>/media/` sob o prefixo `/api/media/files/`, garantindo segurança de caminho (impedindo path traversal) e headers de cache.
4. **WebSocket Sync:** `PresentationState` inclui `active_setlist_id`, `wallpaper_path` e `wallpaper_mode`, permitindo que o `/public` reaja instantaneamente a trocas de setlist ou de papel de parede.

## Critérios de Aceite

- [ ] Clicar com botão direito em qualquer item da setlist exibe menu com "Fixar no topo", "Editar", "Renomear" e "Remover da Setlist".
- [ ] Remover item da setlist retira-o da lista em tempo real; se estivesse ao vivo, encerra a apresentação.
- [ ] Clicar com botão direito em área vazia da setlist exibe menu com opções para adicionar Seção, Música ou Texto.
- [ ] Clicar no botão `+` da setlist exibe dropdown com opções correspondentes.
- [ ] É possível cadastrar mais de uma setlist com título, descrição e papel de parede.
- [ ] Alternar a setlist ativa carrega seus itens e aplica seu papel de parede no `/public`.
- [ ] A tela pública exibe o papel de parede quando não há slide ativo, respeitando o modo de ajuste selecionado (preencher, ajustar, centralizar, repetir).
- [ ] Os arquivos de mídia e papéis de parede ficam salvos no diretório do sistema operacional.
- [ ] O logo no header não aparece duplicado e está com tamanho nítido e legível.
- [ ] As cores padrão do shadcn-ui adotam tons neutros e podem ser configuradas.
- [ ] Todos os testes de `cargo test`, `cargo clippy`, `cargo fmt` e `bun run check` passam sem erros.

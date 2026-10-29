# ADR 0007 — Múltiplas Setlists, Context Menu e Armazenamento de Mídias

## Contexto

A versão 0.5.0 introduziu o controle de apresentação desacoplado e configurações avançadas. Para enriquecer a experiência do operador e atender a demandas práticas de congregações, é necessário gerenciar múltiplas programações de culto (setlists), manipular dinamicamente seus itens com menu de contexto (clique direito), exibir papéis de parede personalizados na tela pública e salvar arquivos de mídia no sistema de arquivos para fácil portabilidade.

## Decisão

1. **Persistência de Setlists no SQLite (Migração 0004):**
   - Criação da tabela `setlists` contendo `id`, `title`, `description`, `wallpaper_path`, `wallpaper_mode`, `items_json`, `is_active`, `created_at` e `updated_at`.
   - O armazenamento de itens ordenados com metadados em `items_json` permite flexibilidade imediata para cabeçalhos divisores (`MediaKind::Header`), mídias avulsas e louvores sem necessidade de reconstrução de grafo relacional a cada reordenação.
2. **Extensão de Tipos de Mídia (`MediaKind::Header`):**
   - Adição do tipo `Header` aos tipos de mídia (`Song`, `Scripture`, `Announcement`, `Media`, `Header`) para representar seções/divisores na setlist.
3. **Armazenamento de Mídias em Pasta do Sistema Operacional:**
   - As mídias e papéis de parede ficam armazenados na pasta de dados do aplicativo (`<data_dir>/media/wallpapers/`), podendo ser movidos, sincronizados via nuvem ou copiados via pendrive.
   - O servidor Axum expõe os arquivos de forma segura sob `/api/media/files/*` com detecção de tipo MIME e prevenção de path traversal.
4. **Menus de Contexto Customizados:**
   - Criação de menu de contexto dedicado acionado por clique direito nos itens da setlist (Fixar no topo, Editar, Renomear, Remover da Setlist) e na área vazia (Adicionar Seção, Adicionar Música, Adicionar Texto Livre).
5. **Correção de Identidade Visual e Paleta Neutra:**
   - Correção do logotipo no cabeçalho eliminando ícone duplicado e redimensionando proporcionalmente.
   - Adoção dos tons neutros (`neutral`) do shadcn-ui como primários no aplicativo com suporte a customização nas configurações.

## Consequências

- **Positivas:**
  - O operador pode planejar cultos diferentes com antecedência e alternar entre eles com um clique.
  - A tela pública pode projetar a identidade visual do culto ou igreja sem necessidade de operador segurar um slide em branco.
  - O usuário tem total autonomia para gerenciar seus arquivos de papel de parede no disco.
  - O código e a interface permanecem limpos, modernos e de alta performance.
- **Negativas:**
  - O schema do banco avança para v4, requerendo aplicação automática da migração aditiva.

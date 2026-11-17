# Especificação — Correção da Sobrescrita do Catálogo Outros

## 1. Contexto e Motivação

No servidor da igreja, agendas criadas pelo operador desapareciam da aba
"Outros" ao longo dos dias. A investigação com dados reais do servidor
confirmou que os itens continuavam existindo nas setlists, mas não no catálogo
persistido em `app_settings` com a chave `custom_media_items`.

### 1.1 Causa Raiz

1. `saveOrUpdateCustomItem` montava o array a ser gravado a partir de
   `get(customItemsStore)`, um store em memória que começa vazio a cada carga da
   página e só é preenchido quando a aba "Outros" é aberta.
2. Salvar um item antes dessa consulta enviava ao servidor um array contendo
   apenas aquele item, sobrescrevendo todo o catálogo. Foi o que aconteceu com a
   agenda das 18:40 de 15/11, apagada quando a seção "Jovens" foi salva às 20:44.
3. Os handlers de agenda/texto/aviso em `AdminView.svelte` chamavam
   `saveOrUpdateCustomItem(...).catch(() => {})`, engolindo falhas de gravação.
4. `loadCustomItems` reinterpretava um array vazio válido (`"[]"`) como
   "nenhum catálogo salvo" e reinseria os quatro modelos padrão.

## 2. Intenção

Tornar a gravação do catálogo atômica no servidor, eliminando a
read-modify-write no cliente como fonte de perda de dados, e dar feedback
explícito quando a persistência falhar.

## 3. Escopo

### 3.1 Dentro do escopo

- Backend (`crates/op-admin/src/routes.rs`): endpoints de item
  (`POST /api/custom-items`, `DELETE /api/custom-items/{id}`), importação
  (`POST /api/custom-items/import`), listagem (`GET /api/custom-items`) e
  leitura/gravação do catálogo sob o mesmo lock do banco.
- Frontend (`customItems.ts`, `AdminView.svelte`): uso dos endpoints atômicos,
  carregamento do catálogo no startup, correção do array vazio e não supressão
  de erros.
- Testes de integração e documentação.

### 3.2 Fora do escopo

- Migrações de schema (a chave `custom_media_items` permanece a mesma).
- Mudanças visuais na aba "Outros".
- Publicação/build/deploy no servidor da igreja.

## 4. Contrato da API

Todos os endpoints retornam o catálogo completo atualizado como
`Vec<MediaItem>` e respondem `503 Service Unavailable` quando o banco não está
disponível.

| Método | Rota | Corpo | Efeito |
| --- | --- | --- | --- |
| `GET` | `/api/custom-items` | — | Lista o catálogo atual |
| `POST` | `/api/custom-items` | `{ "item": MediaItem }` | Insere ou substitui por `id` |
| `DELETE` | `/api/custom-items/{id}` | — | Remove o item com esse `id` |
| `POST` | `/api/custom-items/import` | `{ "items": MediaItem[] }` | Adiciona os itens ainda inexistentes (dedupe por `id`) |

A leitura e a escrita ocorrem dentro do mesmo `Mutex<Database>`, e a alteração
é transmitida às telas conectadas via `WsMessage::SettingUpdated` com a chave
`custom_media_items`.

## 5. Critérios de Aceite

1. Inserir um item nunca remove os itens já persistidos, mesmo que o cliente
   jamais tenha carregado o catálogo.
2. Atualizar um item substitui apenas o registro com o mesmo `id`.
3. Excluir um item remove somente o `id` informado.
4. Importar itens repetidos não duplica registros.
5. Um array vazio salvo anteriormente é preservado (não dispara re-seed).
6. Falha de persistência gera erro visível ao operador, nunca silencioso.
7. Sem banco disponível, os endpoints retornam `503`.
8. Portões `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D
   warnings`, `cargo test --workspace`, `npm run check`, `npm run build` e
   `npx impeccable detect --json` passam.

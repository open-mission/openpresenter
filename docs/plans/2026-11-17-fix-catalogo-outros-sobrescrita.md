# Plano — Correção da Sobrescrita do Catálogo Outros

## Etapa 1 — Backend (`crates/op-admin/src/routes.rs`)

- [x] Registrar rotas `GET/POST /api/custom-items`, `POST /api/custom-items/import` e `DELETE /api/custom-items/{id}`.
- [x] Extrair helpers `read_custom_items` e `persist_custom_items` sobre `app_settings.custom_media_items`.
- [x] Implementar upsert por `id`, remoção por `id` e importação com dedupe por `id`, todos dentro do mesmo lock do banco.
- [x] Broadcast de `SettingUpdated` após cada alteração.
- [x] Retornar `503 Service Unavailable` quando `state.db` for `None`.

## Etapa 2 — Testes (`crates/op-admin/src/main.rs`)

- [x] Teste de integração com banco real: base legada de 2 itens, inserção, atualização, exclusão, importação com duplicata e consistência entre `GET /api/custom-items` e a configuração persistida.
- [x] Teste de indisponibilidade de banco retornando `503`.

## Etapa 3 — Frontend (`customItems.ts`, `AdminView.svelte`)

- [ ] `saveOrUpdateCustomItem`, `deleteCustomItem` e `importItemsFromSetlist` passam a usar os endpoints atômicos e a substituir o store pelo array devolvido.
- [ ] `loadCustomItems` aceita array vazio como catálogo válido e semeia padrão somente quando `value` estiver ausente.
- [ ] `loadCustomItems()` chamado no startup do `AdminView`.
- [ ] Handlers de agenda/texto/aviso deixam de engolir erros e exibem `notifyError`.

## Etapa 4 — Documentação e portões

- [x] Spec e plano registrados e linkados em `docs/README.md`.
- [ ] `CHANGELOG.md` atualizado.
- [x] Portões backend: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (102 testes).
- [ ] Portões frontend: `npm run check`, `npm run build`, `npx impeccable detect --json`.

## Etapa 5 — Entrega

- [ ] Commits atômicos em GitFlow no branch `fix/catalog-items-and-remote-ip`.
- [ ] Recuperação dos itens perdidos já executada no servidor da igreja (2 agendas restauradas).
- [ ] Publicação/deploy no servidor da igreja mediante autorização explícita.

# ADR 0001 — Workspace, rusqlite e Slint Material vendorizado

- **Status:** aceito
- **Data:** 2026-10-28

## Contexto

O OpenPresenter Minimal precisa de uma fundação que separe domínio, persistência e interface sem exigir uma reestruturação quando o projetor e as funcionalidades de produto chegarem. A aplicação é desktop, local e prioriza desempenho e estabilidade.

## Decisão

- Organizar o código em um workspace Rust com `op-core`, `op-db` e `op-admin`; o futuro `op-output` ocupará a mesma fronteira.
- Usar `rusqlite` com a feature `bundled` e `rusqlite_migration`, em vez de um ORM ou runtime assíncrono.
- Fixar Slint em `1.18.1` e vendorizar os Material Components `1.1.0` em `vendor/material-1.1.0/`, carregados via `build.rs` com `with_library_paths`.

## Alternativas consideradas

- **Crate único com camadas internas:** menor cerimônia inicial, mas exigiria mover arquivos e ajustar dependências assim que a segunda responsabilidade real aparecesse.
- **`sqlx`:** traria runtime assíncrono e macros para consultas mínimas de uma aplicação local; sem ganho nesta escala.
- **Material como dependência do crates.io:** não existe; a biblioteca precisa ser vendorizada.

## Consequências

- Fronteiras explícitas e testes de banco isolados.
- Uma cópia versionada do Material, que deve ser atualizada manualmente e sem alterações locais.
- Atualizações do Material exigem compatibilidade com a série Slint 1.18.x; a versão do Material não deve ser trocada sem justificativa.

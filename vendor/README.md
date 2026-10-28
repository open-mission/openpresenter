# Slint Material Components vendorizado

- **Versão:** 1.1.0
- **Origem:** <https://material.slint.dev/zip/material-1.1.0.zip>
- **Data de extração:** 2026-10-28
- **Diretório:** `vendor/material-1.1.0/`

O conteúdo foi extraído do arquivo oficial e mantido **sem alterações**. Não edite arquivos dentro de `vendor/material-1.1.0/`; correções e customizações devem ficar no código do projeto.

## Licença

Os Material Components estão sob a licença MIT, conforme `vendor/material-1.1.0/LICENSE.md`. O Slint é tri-licenciado; este projeto usa a opção **GPL-3.0-only**, compatível com a licença do projeto.

## Como atualizar

1. Baixe a nova versão em `https://material.slint.dev/zip/material-<versão>.zip`.
2. Substitua o diretório `vendor/material-1.1.0/` pelo novo, mantendo o padrão `vendor/material-<versão>/`.
3. Ajuste o caminho em `crates/op-admin/build.rs`.
4. Recompile com `cargo build -p op-admin`.
5. Execute `cargo test --workspace` e valide a janela nos temas claro e escuro.
6. Atualize este README com a versão, a origem e a data.

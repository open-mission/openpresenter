# Spec: Ícone Oficial da Aplicação no System Tray e Executável

## Intenção
Garantir que a aplicação apresente a identidade visual oficial do OpenPresenter tanto no System Tray (Menu Bar / Barra de Tarefas) quanto no arquivo executável final (no Windows Explorer e demais sistemas), substituindo a renderização de pixels por carregamento do asset oficial e embutindo o recurso de ícone (`.ico`) no binário nativo.

## Escopo
- Gerar versões fiéis em PNG e ICO do ícone oficial `assets/branding/open-presenter-app-icon.svg`.
- Atualizar `crates/op-admin/src/tray.rs` para carregar o ícone oficial PNG usando o crate `image`, garantindo nitidez e proporções exatas da marca.
- Adicionar `crates/op-admin/build.rs` com `winresource` para embutir `open-presenter-app-icon.ico`, nome do produto e metadados no executável do Windows.
- Atualizar `assets/branding/README.md` documentando os novos formatos bitmap derivados do SVG oficial.

## Critérios de Aceite
1. O System Tray inicializa exibindo o ícone oficial do OpenPresenter com proporções nítidas e transparência adequada.
2. Em builds Windows, o arquivo `open-presenter.exe` possui o ícone oficial embutido exibido pelo Windows Explorer.
3. As regras de governança e branding (`assets/branding/README.md` e Regra 10 de `AGENTS.md`) são rigorosamente mantidas.
4. Testes do workspace e linters executam sem warnings ou falhas.

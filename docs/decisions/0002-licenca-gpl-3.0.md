# ADR 0002 — Licença GPL-3.0-only e política de marca

- **Status:** aceito
- **Data:** 2026-10-28

## Contexto

O projeto deve ser open source, mas não deve permitir que terceiros distribuam um clone proprietário do OpenPresenter. A licença do Slint e dos Material Components também precisa ser compatível com a distribuição do aplicativo.

## Decisão

- Licenciar o código sob **GPL-3.0-only**.
- Usar Slint sob a opção GPLv3; os Material Components são MIT, conforme o `LICENSE.md` vendorizado.
- Proteger o nome, o logotipo e a identidade visual por uma política separada em `TRADEMARK.md`.

## Alternativas consideradas

- **Licença permissiva (MIT/Apache-2.0):** permitiria que um derivado fosse fechado e distribuído como clone proprietário.
- **Licença proprietária:** contraria o objetivo de software livre.
- **AGPL:** adicionaria obrigações focadas em uso em rede, sem ganho para um aplicativo desktop local.

## Consequências

- Derivados distribuídos devem permanecer sob a GPL e ter o código disponível.
- A GPL não proíbe forks abertos — isso é inerente ao open source.
- Forks não podem usar o nome ou a identidade visual do OpenPresenter como se fossem oficiais; devem adotar marca própria.
- A licença não concede direitos de marca.

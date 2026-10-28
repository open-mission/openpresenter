# Fundação técnica do OpenPresenter Minimal — v0.1.0

- **Status:** aprovado
- **Data:** 2026-10-28
- **Versão do app:** 0.1.0
- **Versão do schema:** 1

## 1. Contexto

O OpenPresenter Minimal é um aplicativo desktop de projeção para igrejas e eventos. O painel administrativo será construído em Rust com Slint e os componentes Material, com SQLite como armazenamento local. O projeto valoriza desempenho, estabilidade e uma apresentação visual refinada.

Esta etapa entrega apenas a fundação: uma janela nativa que apresenta o aplicativo, um banco SQLite já inicializado com o estado necessário e a versão 0.1.0 configurada de forma organizada. Não há telas de produto nesta versão.

O repositório está vazio. Os assets de branding fornecidos ficam em `/Users/claudio/cafg-second-brain/Projects/OpenPresenter/media-assets` e serão copiados para o projeto.

## 2. Objetivos da v0.1.0

- Abrir uma janela nativa com a identidade visual do OpenPresenter.
- Mostrar as informações iniciais do app: nome, versão, proposta, stack, estado do banco, caminho do banco, versão do schema, primeira e última execução.
- Criar e migrar o banco SQLite no diretório de dados do sistema operacional.
- Persistir o estado inicial em `app_meta`, preservando a primeira execução entre sessões.
- Estabelecer a versão 0.1.0 como fonte única no workspace, exibida na interface e registrada no banco.
- Organizar o projeto em workspace com fronteiras claras entre domínio, banco e interface.
- Instituir a organização documental do projeto: specs, arquitetura, decisões, changelog e instruções para agentes em `AGENTS.md` e `GEMINI.md`.
- Deixar o projeto pronto para receber as próximas funcionalidades sem reestruturação.

## 3. Fora de escopo

- Cadastro de músicas, liturgias, cultos, eventos ou playlists.
- Projeção em segunda tela, janela de palco, retorno ou controle remoto.
- Importação de mídia, letras, Bíblia ou apresentações externas.
- Atalhos globais, agendamentos, OBS, NDI ou integrações externas.
- Tema personalizável pelo usuário, i18n completo ou múltiplos idiomas.
- Instaladores, assinatura, auto-update, empacotamento `.app`, `.msi` ou `.deb`.
- Sincronização em nuvem, telemetria ou contas de usuário.
- CI/CD. Os comandos de qualidade ficam documentados e podem virar CI depois.

## 4. Anti-objetivos

- UI web, Electron ou WebView: a interface deve ser nativa.
- Banco remoto ou ORM pesado para uma aplicação local de mesa.
- Tabelas de domínio especulativas antes de existir o fluxo de produto.
- Dependências grandes sem ganho concreto nesta etapa.
- Deixar a documentação para depois: spec e decisões fazem parte da entrega.

## 5. Restrições e premissas

- **Rust:** edição 2024, MSRV 1.95. Ambiente validado com Rust 1.96.0.
- **Slint:** 1.18.1 (o Material 1.1.0 exige Slint 1.18 ou superior).
- **Material Components:** 1.1.0, vendorizado em `vendor/material-1.1.0/`; não existe como dependência do crates.io. Deve ser carregado via `build.rs` com `with_library_paths`.
- **SQLite:** local, embutido via `rusqlite` com a feature `bundled`, para não depender de SQLite instalado na máquina.
- **Plataformas:** desenvolvimento em macOS; a base deve compilar em Windows e Linux. Recursos específicos de empacotamento ficam para depois.
- **Idioma:** documentação e textos de interface em português do Brasil; código, nomes de tipos e APIs em inglês.
- **Licença:** o código do projeto será licenciado sob **GPL-3.0-only**. É uma licença open source que exige que derivados permaneçam abertos, impedindo um clone proprietário. O Slint é usado sob a opção GPLv3; os Material Components estão sob a licença MIT. A GPL não proíbe forks abertos — isso é inerente ao open source. O nome e a identidade visual do OpenPresenter são protegidos separadamente por uma política de marca: forks devem usar nome e marca próprios.
- **Branding:** os SVGs fornecidos são a referência de marca. Serão copiados sem alteração para `assets/branding/`.

## 6. Abordagens consideradas

### A — Crate único com camadas internas

Menor cerimônia: `src/main.rs`, módulos de UI, banco e estado, com o Material vendorizado na raiz. Entrega rápida e suficiente para uma tela. O custo aparece quando o projetor e o domínio crescerem: a separação teria de ser feita depois, movendo arquivos e ajustando dependências.

### B — Workspace com crates por responsabilidade (escolhida)

`op-core` para o domínio, `op-db` para SQLite e migrações, `op-admin` para a interface Slint e, no futuro, `op-output` para o projetor. Fronteiras explícitas, testes de banco isolados e direção de dependências clara. Custa alguns arquivos e configuração inicial, mas evita uma refatoração estrutural assim que a segunda responsabilidade real aparecer.

**Decisão:** B. O usuário escolheu explicitamente essa opção.

### Alternativa descartada — `sqlx`

`sqlx` traria runtime assíncrono e macros de banco para uma aplicação desktop local. Não há ganho nesta escala; `rusqlite` é síncrono, leve e suficiente. A interface continuará responsiva porque as consultas desta versão são mínimas e ocorrem na inicialização.

## 7. Arquitetura

### 7.1 Estrutura do repositório

```text
open-presenter-minimal/
├── Cargo.toml                  # workspace, versão 0.1.0
├── Cargo.lock                  # versionado
├── README.md
├── CHANGELOG.md
├── LICENSE                  # GPL-3.0-only
├── TRADEMARK.md             # política de nome e identidade visual
├── AGENTS.md
├── GEMINI.md
├── .gitignore
├── assets/
│   └── branding/               # SVGs copiados de media-assets
│       └── README.md
├── vendor/
│   ├── README.md               # origem, versão e licença do Material
│   └── material-1.1.0/         # biblioteca vendorizada, sem alterações
├── crates/
│   ├── op-core/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── op-db/
│   │   ├── Cargo.toml
│   │   ├── migrations/
│   │   │   └── 0001_initial.sql
│   │   └── src/lib.rs
│   └── op-admin/
│       ├── Cargo.toml
│       ├── build.rs
│       ├── ui/
│       │   ├── main.slint
│       │   └── theme.slint
│       └── src/
│           ├── main.rs
│           ├── session.rs
│           └── ui.rs
└── docs/
    ├── README.md               # índice da documentação
    ├── specs/
    │   └── 2026-10-28-fundacao-tecnica.md
    ├── plans/
    │   └── 2026-10-28-fundacao-tecnica.md
    ├── architecture/
    │   └── overview.md
    └── decisions/
        ├── README.md
        ├── 0001-workspace-rusqlite-slint.md
        └── 0002-licenca-gpl-3.0.md
```

### 7.2 Fronteiras e dependências

- `op-core` não depende de Slint nem de SQLite. Contém tipos e constantes de domínio: `AppMeta`, `Theme`, `VERSION`, nome e tagline do app.
- `op-db` depende de `op-core` e encapsula `rusqlite`, migrações e SQL. Nenhuma API de UI vaza para este crate.
- `op-admin` depende de `op-core` e `op-db`; é o único crate que conhece Slint e resolve o diretório de dados.
- Direção: `op-admin` → `op-db` → `op-core`. Nunca o contrário.

### 7.3 Fluxo de inicialização

1. Resolver o diretório de dados (variável de ambiente ou diretório local do sistema) e criar a pasta se necessário.
2. Abrir `open-presenter.db`, aplicar PRAGMAs e rodar as migrações.
3. Ler o registro anterior de `app_meta`, se existir.
4. Gravar a sessão atual: preservar `first_run_at`, atualizar `last_run_at`, `app_version` e `updated_at`; preservar preferências.
5. Montar as informações da tela: versão do app, versão do schema, caminho do banco, primeira execução, última execução anterior, tema e idioma.
6. Construir a janela Slint e exibir.
7. Se qualquer etapa de banco falhar, a janela ainda abre em estado degradado, mostrando o erro e o caminho tentado; o app não fecha silenciosamente.

A última execução exibida é a da sessão anterior, lida antes da atualização. Na primeira abertura, a tela mostra “Esta é a primeira execução”.

### 7.4 Diretório e banco

- Override de desenvolvimento: `OPENPRESENTER_DATA_DIR`.
- Padrão: `directories::BaseDirs::data_local_dir()` + `OpenPresenter`.
  - macOS: `~/Library/Application Support/OpenPresenter`
  - Windows: `%LOCALAPPDATA%\OpenPresenter`
  - Linux: `~/.local/share/OpenPresenter`
- Arquivo: `open-presenter.db`.
- PRAGMAs na abertura: `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`.
- O banco nunca é versionado no repositório.

### 7.5 Schema inicial (versão 1)

```sql
CREATE TABLE app_meta (
    id           INTEGER PRIMARY KEY CHECK (id = 1),
    app_version  TEXT NOT NULL,
    first_run_at TEXT NOT NULL,
    last_run_at  TEXT NOT NULL,
    theme        TEXT NOT NULL DEFAULT 'system'
                 CHECK (theme IN ('system', 'light', 'dark')),
    locale       TEXT NOT NULL DEFAULT 'pt-BR',
    updated_at   TEXT NOT NULL
);
```

- Datas em RFC 3339 UTC com milissegundos.
- O registro é único (`id = 1`) e criado pelo Rust na primeira inicialização, porque contém dados variáveis.
- `app_version` registra a última versão que abriu o banco; `first_run_at` nunca é sobrescrito.
- `theme` e `locale` representam o estado de preferências desta versão: `system` e `pt-BR`.
- A versão do schema é controlada por `PRAGMA user_version` via `rusqlite_migration`; começa em 1. Não é a mesma coisa que a versão do app.

### 7.6 Migrações e versionamento

- Migrações ficam em `crates/op-db/migrations/` e são embutidas com `include_str!`.
- Migrações aplicadas são imutáveis. Qualquer mudança de schema cria `0002_...sql`, `0003_...sql` etc.
- A versão do app segue SemVer e vive em `[workspace.package] version = "0.1.0"`; todos os crates herdam.
- A versão exibida na UI vem de `op_core::VERSION` (`env!("CARGO_PKG_VERSION")`).
- `CHANGELOG.md` no formato Keep a Changelog, com a seção 0.1.0.
- Tag Git `v0.1.0` apenas quando o usuário tratar a versão como release; a fundação não cria a tag automaticamente.

### 7.7 Interface

Raiz `MaterialWindow` (herda `Window`), dimensões iniciais 960×640, título “OpenPresenter”, ícone de janela a partir de `assets/branding/open-presenter-app-icon.svg`.

Composição, de cima para baixo:

1. **Cabeçalho:** marca gráfica + wordmark (variante clara/escura conforme o tema do sistema) e um selo discreto `v0.1.0`.
2. **Hero:** título curto de boas-vindas e a tagline “Projeção para igrejas e eventos”, com uma frase de apoio indicando que a fundação está pronta.
3. **Cartão “Aplicação”:** versão, tema efetivo (claro/escuro, com preferência “Sistema”), idioma e stack “Rust · Slint · SQLite”.
4. **Cartão “Banco de dados”:** status (conectado ou indisponível), caminho do arquivo, versão do schema, primeira execução e última execução anterior.
5. **Rodapé:** “Dados salvos localmente” e a versão.

Estados visuais:

- **Pronto:** marcador verde discreto, textos normais.
- **Degradado:** marcador âmbar, aviso “Banco indisponível”, mensagem de erro e caminho tentado; os demais dados do app continuam visíveis.
- **Primeira execução:** “Esta é a primeira execução” no lugar da última execução.

Direção visual: monocromática e editorial, alinhada aos assets (`#111318`, branco, cinzas neutros), com bastante respiro, hierarquia tipográfica clara, bordas sutis e sem excesso de elevação. O tema segue `Palette.color_scheme` (claro/escuro do sistema). Nesta versão não se sobrescreve a paleta interna do Material; os componentes Material são usados em superfícies neutras e a identidade vem dos tokens próprios e do branding. Uma paleta Material customizada fica para a etapa de tema.

### 7.8 Branding

- Copiar para `assets/branding/`: `open-presenter-app-icon.svg`, `open-presenter-icon-dark.svg`, `open-presenter-icon-filled.svg`, `open-presenter-icon-inverted.svg`, `open-presenter-icon-light.svg`, `open-presenter-icon-monochrome.svg`, `open-presenter-wordmark.svg`, `open-presenter-wordmark-dark.svg`.
- Usar o wordmark claro sobre fundo escuro e o escuro sobre fundo claro.
- Usar o app icon como ícone da janela.
- Não recolorir, distorcer ou alterar proporções dos SVGs.
- `assets/branding/README.md` registra a origem, a data da cópia e a regra de não edição.

### 7.9 Tratamento de erros

- `op-db` expõe um erro tipado com contexto de abertura, migração e leitura/gravação.
- `op-admin` converte o erro em uma mensagem curta para a interface e registra o detalhe em `stderr`.
- Falha de banco não impede a janela de abrir; o estado degradado é parte do produto.
- O app não cria o banco se o diretório não puder ser criado; mostra o erro.

## 8. Documentação e governança

A documentação é parte da entrega, não um subproduto.

### 8.1 Estrutura

- `docs/README.md`: índice de toda a documentação.
- `docs/specs/`: uma spec por funcionalidade ou mudança relevante, nomeada `YYYY-MM-DD-slug.md`. Esta fundação é a primeira.
- `docs/plans/`: planos de implementação derivados das specs, nomeados `YYYY-MM-DD-slug.md` e linkados à spec correspondente.
- `docs/architecture/overview.md`: visão duradoura da arquitetura, dependências, fluxo de inicialização e banco.
- `docs/decisions/`: ADRs curtos, numerados (`0001-slug.md`). O primeiro registra a escolha do workspace, `rusqlite` e Material vendorizado; o segundo, a licença GPL-3.0-only e a política de marca.
- `README.md` na raiz: visão geral, pré-requisitos, comandos, caminho dos dados e licença.
- `CHANGELOG.md`: mudanças visíveis ao usuário, por versão.
- `LICENSE` e `TRADEMARK.md`: texto da GPL-3.0-only e política de uso do nome/identidade visual.
- `AGENTS.md` e `GEMINI.md`: instruções permanentes para agentes de IA.

### 8.2 Regras que devem constar em `AGENTS.md` e `GEMINI.md`

1. Antes de implementar qualquer funcionalidade ou mudança relevante, criar ou atualizar uma spec em `docs/specs/` com intenção, escopo, decisões e critérios de aceite; para mudanças com várias etapas, criar o plano correspondente em `docs/plans/`.
2. Manter `docs/README.md` atualizado e linkando todos os documentos.
3. Registrar decisões arquiteturais duradouras em `docs/decisions/` como ADR; ADRs aceitos não são editados, são substituídos por outro.
4. Atualizar `docs/architecture/overview.md` quando a estrutura, o fluxo ou as dependências mudarem.
5. Atualizar `CHANGELOG.md` para toda mudança visível ao usuário e seguir SemVer.
6. Nunca editar uma migração já aplicada; criar uma nova migração numerada. Versão do app e versão do schema são independentes.
7. Antes de declarar uma tarefa concluída, executar `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.
8. Escrever documentação e textos de interface em português do Brasil; código e identificadores em inglês.
9. Respeitar a direção de dependências: `op-admin` → `op-db` → `op-core`; `op-core` não conhece UI nem SQLite.
10. Usar os assets de `assets/branding/` sem alteração; novas peças de marca precisam ser adicionadas ao README de branding.
11. Não versionar banco local, arquivos `-wal`/`-shm`, segredos ou dados de usuário.
12. Manter `AGENTS.md` e `GEMINI.md` sincronizados; em caso de divergência, `AGENTS.md` é a fonte canônica.
13. Preservar a licença GPL-3.0-only e a política de marca: forks não podem usar o nome ou a identidade visual do OpenPresenter como se fossem oficiais.

## 9. Testes e critérios de aceite

### 9.1 Testes automatizados

- `op-db` com banco temporário:
  - primeira abertura cria `app_meta`, define `first_run_at`, `last_run_at`, `app_version` 0.1.0 e schema 1;
  - segunda abertura preserva `first_run_at`, retorna a última execução anterior e atualiza `last_run_at`/`app_version`;
  - reabrir o banco é idempotente em relação às migrações;
  - PRAGMAs essenciais estão ativos;
  - caminho inválido produz erro tipado, não pânico.
- `op-core`: constantes de versão e conversão de `Theme` (se houver serialização).
- `op-admin`: compilação da UI via build; sem teste visual automatizado nesta versão.

### 9.2 Critérios de aceite

- `cargo run -p op-admin` abre uma janela nativa intitulada “OpenPresenter”.
- A janela exibe marca, versão 0.1.0, tagline, stack, status do banco, caminho, schema 1, primeira execução e última execução anterior.
- O banco é criado no diretório de dados do sistema (ou no override) e o registro de `app_meta` existe após a primeira execução.
- Abrir o app uma segunda vez preserva `first_run_at` e mostra a execução anterior.
- Uma falha simulada de banco mantém a janela aberta em estado degradado, com mensagem e caminho.
- O tema acompanha claro/escuro do sistema e o wordmark correto é usado.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` passam.
- `docs/`, `docs/plans/`, `AGENTS.md`, `GEMINI.md`, `README.md`, `CHANGELOG.md`, `LICENSE`, `TRADEMARK.md`, `vendor/README.md` e `assets/branding/README.md` existem, estão coerentes e linkados.
- O workspace está em 0.1.0 e o `Cargo.lock` está versionado.
- O projeto está commitado no Git (commit inicial da fundação).

## 10. Riscos e decisões em aberto

- **Compatibilidade Material/Slint:** Material 1.1.0 exige Slint 1.18+; versões fixadas em 1.18.1. Se o build quebrar por incompatibilidade, a versão do Material deve ser mantida e a do Slint ajustada dentro da série 1.18.x, nunca o contrário sem justificativa.
- **Ícone no dock do macOS:** a propriedade `icon` cobre a janela; o ícone do bundle exige empacotamento futuro. Não é critério desta versão.
- **Licença e marca:** código sob GPL-3.0-only; a licença não concede direitos sobre o nome, logotipo ou identidade visual. A política está em `TRADEMARK.md`. A GPL permite forks abertos, mas derivados não podem se apresentar como OpenPresenter oficial.
- **Identificador de bundle:** será definido no empacotamento; não afeta o diretório de dados desta versão.
- **Paleta Material customizada:** fica para a etapa de tema; a v0.1 usa tokens próprios e componentes neutros.
- **Tag v0.1.0:** só após validação do usuário.

## 11. Liberdade do implementador

Pode decidir sem nova aprovação:

- espaçamento, tamanhos tipográficos e composição exata dentro da direção visual descrita;
- uso de `ElevatedCard`, `OutlinedCard` ou superfícies próprias, desde que o resultado seja monocromático e legível;
- textos auxiliares e mensagens de erro, desde que em pt-BR e coerentes com o tom do produto;
- organização interna de módulos, nomes de funções e uso de `thiserror`/`anyhow`;
- estratégia de testes e helpers;
- detalhes de formatação de datas (desde que legíveis e em pt-BR).

## 12. Aprovação

Aprovada em 2026-10-28. A licença foi definida como GPL-3.0-only, com política de marca separada. O próximo passo é o plano de implementação, seguido da execução com verificação.

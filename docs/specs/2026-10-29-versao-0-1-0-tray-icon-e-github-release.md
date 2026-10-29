# Especificação Técnica: Versão 0.1.0 — Executável Autocontido com Tray Icon, Frontend Embutido e CI/CD Multiplataforma

## 1. Intenção e Motivação

Preparar a primeira release oficial pública do OpenPresenter Minimal (**v0.1.0**) para distribuição simplificada a igrejas e operadores.
Em vez de exigir a instalação de dependências ou abertura manual de terminais de comando, o usuário faz o download de um arquivo executável para o seu sistema operacional (Windows, macOS ou Linux), executa com duplo clique e tem o sistema operando em segundo plano com um ícone nativo na área de notificação/menu bar (System Tray).

---

## 2. Escopo

1. **Unificação de Versão (0.1.0):**
   - Configurar a versão do workspace Rust (`Cargo.toml`), crates dependentes e frontend (`package.json`) para `0.1.0`.
   - Organizar a seção `## [0.1.0] - 2026-10-29` no `CHANGELOG.md` documentando todas as funcionalidades entregues.
2. **Servidor com Porta Segura por Padrão (18080):**
   - Alterar a porta padrão de `3000` para `18080` para evitar conflito com ferramentas de desenvolvimento web e outros softwares locais.
   - Ordem de resolução da porta:
     1. Variável de ambiente `PORT` (se definida).
     2. Configuração persistida no banco SQLite (`app_settings` com chave `server_port`).
     3. Valor padrão `18080`.
3. **Tray Icon e Menu de Contexto Nativo:**
   - Ícone gráfico da marca na bandeja do sistema operacional (Windows, macOS e Linux).
   - Menu com as opções:
     - **Abrir Painel de Controle** (`http://localhost:<porta>/`)
     - **Abrir Tela Pública (Public)** (`http://localhost:<porta>/public`)
     - **Abrir Retorno de Palco (Stage)** (`http://localhost:<porta>/stage`)
     - *Separador*
     - **Status: Servidor Ativo (Porta <porta>)** (item desabilitado informativo)
     - **Configurar Porta do Servidor...** (diálogo nativo para digitação de nova porta e salvamento)
     - *Separador*
     - **Sair do OpenPresenter** (finalização limpa do processo)
   - Suporte a flag de inicialização `--headless` para ambientes sem interface gráfica (servidores de teste, contêineres).
4. **Frontend Embutido no Binário (`rust-embed`):**
   - Incorporar os artefatos compilados de `frontend/dist` no binário compilado.
   - Em modo de desenvolvimento, manter a capacidade de leitura local do disco para recargas imediatas.
5. **Automação GitHub Actions para Build e Release (`.github/workflows/release.yml`):**
   - Matriz de compilação:
     - `windows-latest` -> `open-presenter-v0.1.0-windows-x64.zip`
     - `macos-latest` -> `open-presenter-v0.1.0-macos-universal.tar.gz` (Universal binary para Intel e Apple Silicon)
     - `ubuntu-latest` -> `open-presenter-v0.1.0-linux-x64.tar.gz`
   - Step final com `softprops/action-gh-release@v2` para publicação automática no GitHub Releases quando uma tag `v*` for enviada.

---

## 3. Decisões Técnicas

- **Bibliotecas Tray e Menu:** `tray-icon = "0.26"`, `muda = "0.21"`, `tao = "0.37"` e `open = "5.3"`.
- **Arquitetura de Threads:** A thread principal roda o loop de eventos nativo exigido pelo macOS (AppKit) e Windows (Win32), enquanto o Axum e a persistência SQLite rodam em uma runtime Tokio assíncrona desacoplada.
- **Diálogo Nativo de Porta:** Invocação de comandos de sistema leves e nativos de interface (`osascript` no macOS, `powershell` com `InputBox` no Windows, e `zenity`/`kdialog` no Linux) para entrada de texto sem puxar bibliotecas GUI pesadas e heterogêneas.

---

## 4. Critérios de Aceite

1. O binário `open-presenter` inicia o servidor web em `http://localhost:18080` e exibe o ícone de bandeja com o menu interativo.
2. As opções do menu abrem as URLs corretas no navegador padrão do sistema.
3. A alteração de porta salva a nova porta no SQLite e aplica no próximo ciclo.
4. O binário executável funciona sem a necessidade de uma pasta externa `frontend/dist` (assets embutidos via `rust-embed`).
5. A flag `--headless` permite rodar o servidor em modo puramente terminal.
6. O workflow `.github/workflows/release.yml` é válido e compila os artefatos para Windows, macOS e Linux.
7. Todos os quality gates passam: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `bun run build`.

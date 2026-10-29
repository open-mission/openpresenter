# ADR 0011: Versão 0.1.0 — Executável Autocontido com Tray Icon, Frontend Embutido e CI/CD Multiplataforma

## Status

Aceito.

## Contexto

O OpenPresenter Minimal atingiu maturidade de recursos para o seu primeiro lançamento oficial estável (**v0.1.0**), contemplando catálogo de músicas FTS5, importação Holyrics/Vagalume, editor de slides Markdown, apresentador de imagens com proporções e preenchimentos customizados, múltiplos layouts de monitor de retorno (Stage Display), cronômetro global em background, previews em tempo real fiéis e suporte completo a Bíblias em SQLite com busca textual estrita e atalhos rápidos.

Para a distribuição oficial aos usuários finais (igrejas, operadores, músicos e pregadores):
1. O usuário não deve precisar instalar Node.js, Bun ou ferramentas de terminal. O aplicativo deve ser distribuído como um único arquivo executável nativo para Windows (`.exe`), macOS e Linux.
2. Ao abrir o aplicativo com duplo clique, o servidor web local deve iniciar em segundo plano em uma porta segura e livre de conflitos (porta padrão `18080`), sem exibir janelas de terminal desnecessárias.
3. Deve haver um ícone na bandeja do sistema (**System Tray / Menu Bar**) permitindo abrir o painel principal, a tela pública, o retorno de palco, alterar a porta do servidor e encerrar o aplicativo com segurança.
4. O frontend compilado (`frontend/dist`) deve ser embutido diretamente dentro do binário Rust em tempo de compilação, permitindo que o executável funcione de forma 100% autocontida.
5. O repositório deve contar com automação no GitHub Actions para compilar em matriz multiplataforma (Windows, macOS e Linux) e publicar os artefatos diretamente na aba de Releases do GitHub.

## Decisão

1. **Versão Oficial 0.1.0:**
   - Unificar a versão do workspace Rust (`Cargo.toml`) e do frontend (`package.json`) em `0.1.0`.
2. **Tray Icon Multiplataforma (`tray-icon` + `muda` + `tao`):**
   - Utilizar as bibliotecas padrão do ecossistema Tauri (`tray-icon`, `muda` e `tao`) para gerenciar o ícone da bandeja e menus contextuais nativos no Windows, macOS e Linux.
   - A thread principal gerencia o loop de eventos nativo do sistema operacional (necessário para macOS Cocoa AppKit e Windows Win32 message loop), enquanto o servidor Axum roda em uma runtime Tokio assíncrona dedicada.
   - Fornecer opção `--headless` para permitir execução como serviço/daemon de servidor puro em ambientes sem interface gráfica (ex: servidores Linux, containers).
3. **Porta Padrão Segura e Diálogo de Configuração:**
   - Adotar a porta `18080` como padrão, evitando conflitos com serviços web populares (`3000`, `8080`, `8000`).
   - Permitir a personalização da porta via menu do Tray Icon com diálogo nativo de prompt, persistindo a preferência em banco SQLite (`app_settings.server_port`) ou variável `PORT`.
4. **Embutimento de Frontend com `rust-embed`:**
   - Embutir a pasta `frontend/dist` no binário compilado via `rust-embed`.
   - Em desenvolvimento local, manter fallback dinâmico para leitura direta do disco.
5. **Automação de Release no GitHub Actions (`.github/workflows/release.yml`):**
   - Workflow acionado em tags `v*` e via `workflow_dispatch`.
   - Matriz de compilação:
     - Windows: `windows-latest` gerando `open-presenter-v0.1.0-windows-x64.zip`.
     - macOS: `macos-latest` gerando binário universal `open-presenter-v0.1.0-macos-universal.tar.gz`.
     - Linux: `ubuntu-latest` gerando `open-presenter-v0.1.0-linux-x64.tar.gz`.
   - Publicação automatizada com release notes e checksums via `softprops/action-gh-release@v2`.

## Consequências

### Positivas
- Distribuição simplificada: download de um único arquivo pronto para uso sem pré-requisitos de runtime.
- Controle prático na bandeja do sistema operacional sem poluir a barra de tarefas.
- Porta padrão `18080` previne conflitos com servidores de desenvolvimento em máquinas de técnicos e voluntários.
- Pipeline de CI/CD totalmente automatizado para novas versões.

### Negativas / Cuidados
- No Linux, ambientes headless sem servidor X11/Wayland devem utilizar a flag `--headless`.
- Compilação do binário release requer a execução prévia do build do frontend (`bun run build`), o que é garantido pelo script de build e pelo workflow do GitHub Actions.

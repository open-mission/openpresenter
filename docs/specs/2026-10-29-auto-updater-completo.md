# Especificação: Sistema de Atualização Automática Completa (Auto-Updater)

## Intenção

Permitir que o OpenPresenter verifique automaticamente (e sob demanda) por novas versões oficiais publicadas no GitHub Releases (`open-mission/openpresenter`), notifique o operador no Painel de Controle e no System Tray com as notas da versão e realize o download e a substituição atômica do binário/aplicativo de forma segura em Windows, macOS e Linux, mantendo integralmente todos os dados do banco SQLite e arquivos de mídia.

## Escopo

### Incluído
1. **Verificação de Novas Versões via GitHub API:**
   - Consulta ao endpoint público `https://api.github.com/repos/open-mission/openpresenter/releases/latest`.
   - Comparação semântica de versões (`semver`) contra `op_core::VERSION`.
   - Identificação inteligente do asset correto de download conforme sistema operacional (`windows`, `macos`, `linux`) e arquitetura (`x86_64`, `aarch64`).
2. **Endpoints de API no Backend (`crates/op-admin`):**
   - `GET /api/updater/check`: Retorna se há atualização disponível, versão mais recente, changelog/release notes, data e URL do asset.
   - `POST /api/updater/install`: Dispara em background o download do pacote oficial compactado (`.zip` ou `.tar.gz`), extrai o binário e aplica a substituição atômica no caminho atual (`current_exe`).
   - `GET /api/updater/status`: Retorna o estado atual do processo (`idle`, `checking`, `downloading` com progresso percentual, `extracting`, `ready_to_restart`, `error`).
3. **Substituição Segura do Binário Multiplataforma:**
   - **Linux / macOS:** Substituição direta atômica de arquivo/inode com preservação de permissões executáveis (`0o755`).
   - **Windows:** Mecanismo de renomeação do executável em execução (`open-presenter.old.exe`) para permitir que o novo arquivo `open-presenter.exe` tome o lugar sem travamentos por locks do sistema de arquivos do Windows.
4. **Interface Visual e Notificações (Frontend e Tray):**
   - No **Painel de Controle**:
     - Notificação/badge discreto na barra superior quando houver atualização disponível.
     - Seção dedicada em **Configurações > Atualizações / Sobre**:
       - Exibição da versão instalada e da última versão disponível.
       - Botão "Verificar Atualizações".
       - Exibição formatada das notas da versão (Changelog).
       - Botão "Baixar e Atualizar Agora" com barra de progresso visual.
       - Aviso claro orientando o reinício da aplicação após a conclusão.
   - No **System Tray**:
     - Item de menu que avisa quando há uma nova versão disponível (`★ Nova versão disponível`).
5. **Garantia de Preservação de Dados:**
   - O banco de dados SQLite (`open-presenter.db`), mídias e bíblias permanecem isolados na pasta de dados do sistema (`BaseDirs::data_local_dir`), sendo 100% imunes à substituição do binário.

### Não Incluído
- Modificação forçada de arquivos sem consentimento explícito do usuário (o operador decide quando acionar a instalação).
- Canais de atualização inseguros via HTTP sem validação TLS.

## Decisões Técnicas

1. **Uso da API do GitHub Releases:** Utiliza o repositório canônico `open-mission/openpresenter` para checagem do último release estável via HTTPS.
2. **Descompressão em Memória / Arquivo Temporário:** Pacotes `.tar.gz` (macOS/Linux) são descompactados via `flate2` + `tar`. Pacotes `.zip` (Windows) são descompactados via crate `zip`.
3. **Resolução Dinâmica do Executável Atual:** Identificação de `std::env::current_exe()` para garantir que o arquivo correto seja atualizado, mesmo se renomeado ou movido.
4. **Tratamento de Falhas e Rate Limiting:** Em caso de timeout ou falta de internet, o updater falha de forma graciosa sem impactar a apresentação ou a projeção em andamento.

## Critérios de Aceite

1. O endpoint `/api/updater/check` retorna corretamente se há versão mais recente ou se o app está atualizado.
2. Ao simular uma atualização ou acionar a instalação, o arquivo é baixado e verificado com relatório de progresso em `/api/updater/status`.
3. A interface web em Svelte exibe o modal/aba de atualizações com checagem manual, changelog e progresso de download.
4. O processo do binário não trava o SQLite nem sobrescreve dados na pasta de dados do sistema operacional.
5. Todos os portões de qualidade (`cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`) passam sem erros.

# ADR 0012 — Sistema de Auto-Updater Completo, Verificação de Releases e Substituição Atômica Multiplataforma

## Contexto

Com o lançamento da versão `0.1.0` do OpenPresenter e a publicação de artefatos multiplataforma no GitHub Releases (`open-mission/openpresenter`), tornou-se essencial fornecer aos usuários um fluxo contínuo de atualização de novas versões sem exigir download e substituição manual de arquivos pelo navegador.

Ao mesmo tempo, era imperativo garantir que:
1. O banco SQLite (`open-presenter.db`), mídias e bíblias salvas na pasta de dados do sistema operacional (`directories::BaseDirs::data_local_dir`) não fossem tocados ou corrompidos durante uma atualização.
2. O processo de atualização fosse confiável em diferentes sistemas operacionais (Windows, macOS e Linux), lidando com particularidades como locks de arquivos abertos no Windows e permissões executáveis (`0o755`) no Unix.

## Decisão

Adotou-se uma arquitetura de atualização automática integrada diretamente no backend Rust e exposta via API REST para o frontend Svelte e o System Tray:

1. **Fonte Canônica de Versões:** As atualizações são verificadas diretamente contra a API pública do GitHub Releases no repositório `open-mission/openpresenter`.
2. **Comparação Semântica de Versões:** Uso do parser `semver` para validar releases e determinar com precisão se a release remota é estritamente mais recente que `op_core::VERSION`.
3. **Mapeamento de Assets:** A engine detecta o sistema operacional (`std::env::consts::OS`) e a arquitetura (`std::env::consts::ARCH`) em execução e seleciona o asset exato (`.zip` no Windows, `.tar.gz` no macOS e Linux).
4. **Substituição Atômica Segura:**
   - O arquivo compactado é descarregado em um diretório temporário isolado.
   - O binário novo é extraído e validado.
   - No Windows, o executável em execução é renomeado para `<nome>.old.exe` e o novo executável é movido para o local original.
   - No Unix (macOS e Linux), as permissões de execução são ajustadas e o arquivo é substituído de forma atômica.
5. **Transparência e Controle do Operador:** O sistema notifica visualmente o operador com o changelog da versão e só executa o download e instalação sob comando explícito.

## Alternativas consideradas

- **Instaladores externos (InnoSetup, PKG, etc.):** Descartado para a versão mínima, pois aumentaria a complexidade do pipeline e exigiria privilégios administrativos adicionais do sistema em cada atualização simples.
- **Auto-updater via Crate Externa Pesada:** Descartado para manter dependências leves e evitar conflitos com a arquitetura Axum/Tokio já estabelecida.
- **Apenas notificação sem download:** Descartado a pedido explícito do usuário, que necessitava de atualização completa in-app.

## Consequências

- **Positivas:**
  - Usuários podem atualizar o OpenPresenter com um clique a partir do painel de controle.
  - 100% de integridade dos dados locais (músicas, setlists, bíblias, configurações).
  - Experiência moderna comparável a softwares profissionais de projeção.
- **Atenção:**
  - Em ambientes corporativos restritos ou com proxy sem acesso ao GitHub, a checagem falha graciosamente mantendo a versão atual estável.

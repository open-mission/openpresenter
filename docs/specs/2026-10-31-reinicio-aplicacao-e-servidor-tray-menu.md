# Spec: Reinício da Aplicação/Servidor após Atualização e no System Tray

## Status
Aprovada

## Intenção
Oferecer ao operador a capacidade de reiniciar o OpenPresenter de forma automática ou sob demanda tanto diretamente pelo menu nativo do System Tray (menu bar / barra de tarefas) quanto pela interface web (especialmente após o auto-updater aplicar uma nova versão com sucesso).

## Escopo
1. **Backend (`crates/op-admin`)**:
   - Função utilitária multiplataforma `restart_application()` para spawn do executável com os mesmos argumentos da linha de comando atual, compatível com macOS, Linux e Windows.
   - Endpoint `POST /api/system/restart` que registra a ação nos logs de auditoria, retorna `{ "status": "restarting" }` e agenda a reinicialização limpa após breve intervalo para entrega da resposta HTTP.
   - Atualização do menu do System Tray (`crates/op-admin/src/tray.rs`): adição do item "Reiniciar OpenPresenter" antes do separador de saída.
   - Manipulação do evento de reinício no EventLoop em `crates/op-admin/src/main.rs`.
2. **Frontend (`frontend`)**:
   - Função `triggerSystemRestart()` no módulo `frontend/src/lib/updater.ts`.
   - Botão de ação "Reiniciar Aplicativo Agora" na aba do Atualizador em `SettingsModal.svelte` quando a atualização estiver concluída (`ReadyToRestart`).
   - Feedback visual com overlay de reinício e polling automático de saúde do servidor até o retorno bem-sucedido seguido de reload da página.

## Decisões Técnicas
- **Spawn desanexado do novo processo**:
  - No Windows, utilizar `CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS` para assegurar que a terminação do processo pai não encerre o novo processo.
  - No macOS e Linux, chamar `std::process::Command::new(exe).args(args).spawn()`.
- **Delay no endpoint HTTP**:
  - Introduzir delay de 500ms no tokio runtime antes do `exit(0)` para garantir que a resposta HTTP 200 seja completamente transmitida pela conexão TCP ao navegador.
- **Rastreabilidade**:
  - Gravar entrada no log de auditoria (`system`, `restart_requested`) antes de finalizar.

## Critérios de Aceite
1. O menu do tray exibe "Reiniciar OpenPresenter".
2. O endpoint `POST /api/system/restart` responde com sucesso e agenda o reinício.
3. A interface web apresenta o botão "Reiniciar Aplicativo Agora" ao atingir o estado `ReadyToRestart`.
4. Quality gates: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `npm --prefix frontend run check` 100% limpos.

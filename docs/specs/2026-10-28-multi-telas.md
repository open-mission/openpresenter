# Multi-telas: saídas Public e Stage — v0.2.0

- **Status:** aprovado
- **Data:** 2026-10-28
- **Versão do app:** 0.2.0
- **Versão do schema:** 2
- **Substitui:** nada; complementa a [fundação técnica](2026-10-28-fundacao-tecnica.md)

## 1. Contexto

A fundação v0.1.0 entregou uma única janela administrativa, banco SQLite com schema v1 e a identidade visual do OpenPresenter. A próxima etapa abre o aplicativo para o uso real em projeção: duas saídas independentes, **Public** (visível ao público) e **Stage** (retorno para o palco/apresentador).

Nesta versão as saídas exibem **placeholders de marca**, não conteúdo de apresentação. O objetivo é validar o mecanismo de janelas, monitores, configuração e persistência antes de existir um modelo de culto, letra ou mídia.

O cenário principal é um computador com um monitor externo: o painel administrativo fica na tela do computador e o Public no monitor externo. O Stage é opcional e só aparece se for habilitado nas configurações.

## 2. Objetivos

- Abrir duas janelas de saída independentes, Public e Stage, com placeholders distintos.
- Oferecer uma janela de **Configurações** acessível pelo menu do aplicativo, com a aba **Saídas**.
- Permitir, por saída: ativar/desativar, escolher o monitor e escolher o modo de exibição.
- Restaurar as saídas habilitadas na inicialização, cada uma no monitor configurado.
- Aplicar mudanças de configuração imediatamente, sem reiniciar o aplicativo.
- Persistir a configuração no SQLite com uma migração v2, preservando o schema v1.
- Funcionar com um único monitor, abrindo as saídas como janelas normais.
- Não impedir a abertura do aplicativo quando o banco ou a enumeração de monitores falhar.
- Manter a direção de dependências `op-admin` → `op-db` → `op-core` e a qualidade da fundação.

## 3. Fora de escopo

- Conteúdo real de apresentação: músicas, letras, Bíblia, liturgia, mídia, playlists ou eventos.
- Controle de projeção, avanço de itens, cronômetros de culto e notas do apresentador.
- Escalonamento, resolução, ROI, correção de cor ou layout customizável por saída.
- NDI, OBS, streaming, rede, controle remoto ou múltiplas máquinas.
- Menu nativo do sistema operacional (macOS/Windows); o menu é interno ao aplicativo.
- Atalhos globais, bandeja do sistema, inicialização automática com o SO.
- Detecção de hot-plug em tempo real; a lista de monitores é atualizada ao abrir as Configurações e na inicialização.
- Empacotamento, assinatura, auto-update e ícone de bundle.
- i18n completo; os textos continuam em português do Brasil.

## 4. Anti-objetivos

- Criar tabelas de apresentação ou um modelo de conteúdo especulativo só para preencher as saídas.
- Persistir índice de monitor (posição na lista) em vez de identidade; a ordem pode mudar.
- Introduzir trabalho longo ou perceptível na thread da interface. A enumeração de monitores e a gravação da configuração são chamadas curtas e síncronas na thread principal; a enumeração precisa disso no macOS (`NSScreen`/`MainThreadMarker`).
- Duplicar a lógica de posicionamento em cada janela; ela deve ser uma função testável.
- Editar a migração `0001_initial.sql`; toda mudança cria uma nova migração.
- Fazer o Stage aparecer por padrão; ele é opcional.
- Deixar a janela de Configurações como modal que cobre o painel administrativo.

## 5. Restrições e premissas

- **Stack:** Rust 2024 (MSRV 1.95), Slint 1.18.1, Material Components 1.1.0 vendorizado, `rusqlite` 0.40 com `bundled`, `rusqlite_migration` 2.6.
- **Monitores:** o Slint 1.18 não expõe enumeração de monitores. Será usada a crate `display-info` 0.5 (Apache-2.0, compatível com GPL-3.0-only) em `op-admin`.
- **Coordenadas:** `display-info` devolve coordenadas lógicas no macOS e no Linux (X11 e Wayland) e físicas no Windows. O projeto normaliza tudo para pixels físicos antes de chamar a API do Slint, que trabalha com posição física.
- **Slint:** várias janelas são suportadas criando componentes independentes e chamando `show()` em cada um; os handles fortes precisam viver enquanto a janela existir.
- **Banco:** schema atual é 1; a migração v2 é aditiva e não altera `app_meta`.
- **Plataformas:** desenvolvimento e validação visual no macOS; o código deve compilar em Windows e Linux.
- **Versão:** a entrega é a 0.2.0; a versão do schema (2) é independente da versão do app.

## 6. Abordagens consideradas

### 6.1 Interface de configuração

**A — Janela de Configurações independente (escolhida).** O menu abre uma janela própria, com barra lateral de abas. Permite manter as configurações abertas enquanto as saídas são ajustadas, pode ser movida para outro monitor e cresce para futuras abas (Geral, Aparência, Atalhos). Custa mais código de ciclo de vida (criar, mostrar, fechar e manter o handle).

**B — Modal sobre o painel.** Um diálogo Material com a mesma aba. Menos código, mas cobre o painel, não pode ser movido e fica pesado quando novas abas forem adicionadas.

**Decisão:** A. O usuário escolheu explicitamente a janela independente.

### 6.2 Enumeração de monitores

**A — `display-info` (escolhida).** Enumera monitores em macOS, Windows, X11 e Wayland, com posição, tamanho, escala, principal e builtin. Uma dependência pequena e focada.

**B — APIs nativas por sistema.** Mais controle, mas exige três implementações e manutenção específica de plataforma.

**C — `winit` diretamente.** O Slint já usa winit internamente, mas criar um segundo event loop/`EventLoop` no mesmo processo é frágil e conflita com o loop do Slint.

**Decisão:** A, com fallback para o comportamento de um monitor quando a enumeração falhar.

### 6.3 Persistência

**A — SQLite, schema v2 (escolhida).** Consistente com o restante do app, transacional e testável. Cria a tabela `output_config`.

**B — Arquivo JSON ao lado do banco.** Simples, mas introduz uma segunda fonte de verdade, tratamento de corrupção e escaping próprios.

**Decisão:** A.

## 7. Experiência do usuário

### 7.1 Menu do aplicativo

- Um botão de menu (ícone ☰) entra no cabeçalho da janela administrativa, à direita, antes do selo de versão.
- O botão abre um `PopupMenu` Material com o item **Configurações…**.
- Selecionar o item abre a janela de Configurações; o menu fecha.
- O menu é interno ao app. Não há menu nativo do macOS nesta versão.

### 7.2 Janela de Configurações

- Janela independente, título **Configurações — OpenPresenter**, tamanho inicial 820×600, mínimo 720×540.
- Segue o tema claro/escuro do sistema, como o painel.
- Cabeçalho com título, botão de fechar e, à esquerda, a marca compacta.
- Barra lateral com a aba **Saídas** selecionada. Outras abas não aparecem nesta versão.
- Conteúdo da aba:
  - Texto de apoio: “Escolha onde cada saída deve aparecer. As alterações são aplicadas imediatamente.”
  - Um cartão para **Public** e outro para **Stage**, com título, descrição curta e estado.
  - Controles por cartão: chave **Ativa**, seletor **Monitor**, seletor **Modo** e ação **Abrir agora/Fechar**.
  - Rodapé com o botão **Fechar** e a mensagem “As alterações são aplicadas imediatamente.”
- Estado de cada saída: `Desativada`, `Aberta`, `Oculta`, `Monitor indisponível — usando o principal` ou `Falha ao abrir`.
- Com um único monitor, aparece a nota: “Com um único monitor, as saídas abrem como janelas.”
- Se nenhum monitor for detectado, aparece um aviso e as saídas abrem como janelas na posição padrão do sistema.
- Se o banco estiver indisponível, a janela mostra o erro, os controles ficam desabilitados e nenhuma saída é aberta automaticamente.

### 7.3 Saída Public (placeholder)

- Janela própria, tema escuro fixo (independente do tema do painel), fundo `#0B0D12`.
- Wordmark claro centralizado, etiqueta “SAÍDA PÚBLICA” e a frase “Aguardando conteúdo da apresentação”.
- Sem controles interativos; não há campos focáveis. A janela pode receber foco do sistema ao ser mostrada ou entrar em tela cheia.
- Título da janela: “OpenPresenter — Público”.

### 7.4 Saída Stage (placeholder)

- Janela própria, tema escuro fixo, fundo `#0B0D12`.
- Cabeçalho “RETORNO DE PALCO”, relógio grande `HH:MM:SS` atualizado a cada segundo, data por extenso.
- Linha de evento: “Nenhum evento carregado”.
- Linha de status: “Aguardando apresentação”.
- Sem controles interativos; não há campos focáveis. A janela pode receber foco do sistema ao ser mostrada ou entrar em tela cheia.
- Título da janela: “OpenPresenter — Palco”.

### 7.5 Ciclo de vida das janelas

- **Inicialização:** carrega a configuração, enumera monitores, mostra o painel e depois abre cada saída habilitada. Saídas abertas não impedem o painel de aparecer.
- **Ativar:** abre a janela imediatamente e persiste `enabled = true`.
- **Desativar:** fecha a janela e persiste `enabled = false`.
- **Fechar a janela de saída pelo X:** apenas oculta a saída nesta sessão; `enabled` continua verdadeiro e ela volta na próxima inicialização. O estado “Oculta” aparece nas Configurações.
- **Abrir agora:** reabre uma saída habilitada que foi ocultada.
- **Mudar monitor ou modo:** persiste e, se a janela estiver visível, aplica na hora. Se estiver oculta, a próxima abertura usa a nova configuração.
- **Fechar o painel administrativo:** encerra o aplicativo e todas as saídas.
- A janela de Configurações pode ser fechada sem afetar as saídas.
- Em **tela cheia** no único monitor, a saída cobre o painel; para voltar, o operador usa os atalhos do sistema (Cmd+` no macOS, Alt+Tab no Windows/Linux). Não há atalho local nesta versão.

## 8. Modelo de monitores e posicionamento

### 8.1 MonitorInfo

Representação interna em `op-admin`, produzida a partir de `display-info`:

```rust
pub struct MonitorInfo {
    pub key: String,        // id nativo do display como texto
    pub name: String,       // nome amigável; fallback para o nome nativo
    pub label: String,      // nome amigável + resolução
    pub x: i32,             // pixels físicos
    pub y: i32,             // pixels físicos
    pub width: u32,         // pixels físicos
    pub height: u32,        // pixels físicos
    pub scale_factor: f32,
    pub is_primary: bool,
    pub is_builtin: bool,
}
```

- `x`, `y`, `width` e `height` são normalizados para pixels físicos: no macOS/Linux multiplica-se pela escala; no Windows usa-se o valor recebido.
- `name` usa o `friendly_name` do `display-info` quando não vazio e cai para o `name` nativo. É o nome salvo junto da preferência e serve de identidade secundária.
- `label` combina `name` e resolução, por exemplo `DELL U2720Q — 3840×2160`; recebe o sufixo `(principal)` no monitor principal.
- A identidade é **best-effort** entre plataformas. No macOS o `id` é estável durante a sessão e na maioria dos reinícios; no Windows o `HMONITOR` pode mudar ao trocar porta ou configuração, e o nome atua como fallback. A restauração no monitor salvo é validada no macOS; Windows e Linux ficam documentados como best-effort.

### 8.2 Preferência de monitor

```rust
pub enum MonitorSelection {
    Auto,
    Primary,
    Display { id: String, name: String },
}
```

- `Auto`: primeiro monitor externo (não principal e não builtin); se não houver, o principal; se não houver principal, o primeiro da lista.
- `Primary`: monitor principal; se não existir, o primeiro.
- `Display`: monitor cujo `id` coincide; se não existir, tenta pelo `name` salvo; se ainda assim não existir, usa o principal e sinaliza aviso.
- A preferência específica não é sobrescrita quando o monitor desaparece; ela volta a valer quando o monitor reaparece.
- A lista é esvaziada quando a enumeração falha; nesse caso, as saídas usam a posição padrão do sistema e as Configurações mostram um aviso.

### 8.3 Modo de exibição

```rust
pub enum OutputMode {
    Auto,
    Windowed,
    Maximized,
    Fullscreen,
}
```

- `Auto` para Public: tela cheia quando o monitor resolvido não é o principal; janela quando é o principal.
- `Auto` para Stage: sempre janela.
- `Windowed`: tamanho lógico de 960×540 para Public e 720×480 para Stage. O cálculo converte para pixels físicos pela escala do monitor de destino, centraliza no retângulo físico e aplica `PhysicalSize`/`PhysicalPosition`. Quando o destino é o principal, a janela é deslocada em cascata de 64 px (Public) e 128 px (Stage) lógicos, também convertidos para físicos.
- `Maximized`: posiciona no monitor de destino e maximiza.
- `Fullscreen`: posiciona no monitor de destino e entra em tela cheia com `set_fullscreen(true)`.
- Todas as dimensões de projeto (janela de Configurações 820×600, mínimos, saídas e cascata) são **lógicas**; somente as coordenadas de monitor são físicas.
- A posição e o tamanho são aplicados depois que a janela existe (no loop de eventos), fora do caminho crítico de inicialização.
- Mudar de tela cheia para janela desfaz o fullscreen antes de reposicionar.
- No macOS, a tela cheia nativa cria um Space e ativa a janela; isso é aceito nesta versão.

## 9. Dados e persistência

### 9.1 Tipos de domínio (`op-core`)

```rust
pub enum OutputKind { Public, Stage }
pub enum OutputMode { Auto, Windowed, Maximized, Fullscreen }
pub enum MonitorSelection { Auto, Primary, Display { id: String, name: String } }

pub struct OutputConfig {
    pub kind: OutputKind,
    pub enabled: bool,
    pub monitor: MonitorSelection,
    pub mode: OutputMode,
}

pub struct OutputsConfig {
    pub public: OutputConfig,
    pub stage: OutputConfig,
}
```

- `OutputKind::ALL`, `as_str`, `from_db_str`, `label_pt_br`, `description_pt_br`.
- `OutputMode::as_str`, `from_db_str`, `label_pt_br`.
- `MonitorSelection::as_db_key` (`auto`, `primary`, `id:<id>`), `from_db_key(key, name)`, `label_pt_br`.
- `OutputConfig::default_for(kind)`.
- `OutputsConfig::get(kind)` e `iter()`.

Padrões:

| Saída | Ativa | Monitor | Modo |
|-------|-------|---------|------|
| Public | sim | Auto | Auto |
| Stage | não | Principal | Janela |

### 9.2 Migração v2

```sql
CREATE TABLE output_config (
    output       TEXT PRIMARY KEY CHECK (output IN ('public', 'stage')),
    enabled      INTEGER NOT NULL CHECK (enabled IN (0, 1)),
    monitor_key  TEXT NOT NULL,
    monitor_name TEXT NOT NULL DEFAULT '',
    mode         TEXT NOT NULL CHECK (mode IN ('auto', 'windowed', 'maximized', 'fullscreen')),
    updated_at   TEXT NOT NULL
);
```

- Nenhuma linha é semeada; a ausência de linha significa “usar o padrão da saída”.
- `load_outputs` sempre devolve uma configuração completa, preenchendo com os padrões.
- `save_output` faz upsert por `output`.
- Valores inválidos em `mode` ou `monitor_key` (possíveis apenas por edição manual) caem para `Auto`, preservando a abertura.
- `updated_at` em RFC 3339 UTC com milissegundos, como em `app_meta`.

### 9.3 API de banco (`op-db`)

```rust
pub fn load_outputs(&self) -> Result<OutputsConfig, DbError>;
pub fn save_output(&self, config: &OutputConfig, now: DateTime<Utc>) -> Result<(), DbError>;
```

- Reutiliza `DbError::Read` e `DbError::Write`.
- A migração v1 → v2 é aplicada automaticamente na abertura; `app_meta` e `first_run_at` são preservados.
- `schema_version()` passa a devolver 2.

## 10. Arquitetura de `op-admin`

### 10.1 Módulos

```text
crates/op-admin/src/
├── main.rs        # ponto de entrada; cria o controlador
├── session.rs     # carrega sessão e mantém a conexão aberta
├── monitors.rs    # enumeração, normalização e resolução de monitores
├── outputs.rs     # controlador das janelas Public/Stage e do relógio
├── settings.rs    # estado e callbacks da janela de Configurações
└── ui.rs          # monta o painel, o menu e liga os callbacks
```

### 10.2 Sessão e banco

- `session::load_session` deixa de descartar a conexão: devolve a visão da sessão e um `Option<Database>`.
- O `Database` fica vivo no controlador durante toda a execução para ler/gravar a configuração de saídas.
- Em estado degradado (`db = None`), o painel abre normalmente, mas as saídas ficam desabilitadas e as Configurações mostram o erro do banco.
- A sessão continua registrando `first_run_at` e `last_run_at` como antes.

### 10.3 Controlador de saídas

- `OutputsController` mantém:
  - `OutputsConfig` em memória;
  - `Option<PublicWindow>` e `Option<StageWindow>`;
  - o estado de visibilidade de cada saída;
  - um `slint::Timer` de 1 s para o relógio do Stage, ativo apenas enquanto o Stage existe.
- Operações: `start`, `apply_config`, `set_enabled`, `set_monitor`, `set_mode`, `show_now`, `hide`, `close_all`.
- O fechamento pelo X usa `on_close_requested` para esconder a janela e atualizar o estado; não encerra o app.
- Erros de criação de janela (`PlatformError`) são registrados em `stderr` e refletidos no estado da saída; não derrubam o painel.

### 10.4 Monitores

- `monitors::list()` chama `DisplayInfo::all()`, normaliza para `MonitorInfo` e ordena por `x`, depois `y`.
- `list()` é síncrono e roda na thread principal, depois que o painel aparece. A enumeração é rápida e o macOS exige a thread principal.
- `monitors::resolve(selection, &[MonitorInfo]) -> Resolution` devolve o monitor escolhido, o modo efetivo e um aviso opcional (`Option<MonitorWarning>`).
- Funções puras de resolução e de cálculo de retângulo ficam em `monitors.rs` e são testadas sem abrir janelas.

### 10.5 Configurações

- A janela é criada sob demanda na primeira abertura do menu e mantida viva enquanto estiver aberta.
- Ao abrir, o controlador relê a lista de monitores e recalcula os estados.
- Callbacks da UI chamam o controlador; o controlador atualiza a memória, persiste e aplica.
- A gravação é um upsert SQLite curto e síncrono, executado no próprio callback; não há worker nem estado assíncrono nesta versão.
- Falha de persistência mostra uma faixa de erro na janela; a configuração em memória continua aplicada nesta sessão.
- O botão “Fechar” esconde a janela; o app continua.

## 11. Interface Slint

### 11.1 Arquivos

- `ui/main.slint`: botão de menu, `PopupMenu` e callback `open_settings()`.
- `ui/settings.slint`: `SettingsWindow` independente.
- `ui/outputs.slint`: `PublicWindow` e `StageWindow`.
- `ui/theme.slint`: tokens adicionais para as superfícies de saída.
- `ui/main.slint` reexporta os novos componentes para que `slint::include_modules!()` os gere.

### 11.2 Propriedades e callbacks principais

`SettingsWindow` recebe do Rust:

- `public-enabled`, `stage-enabled`;
- `public-monitor-items: [MenuItem]`, `public-monitor-index`;
- `public-mode-index`, `stage-mode-index`;
- `public-status`, `public-status-kind`, `public-visible`;
- equivalentes para `stage`;
- `single-monitor: bool`, `monitor-warning: string`, `database-error: string`.

Emite para o Rust:

- `public_enabled_changed(bool)`, `stage_enabled_changed(bool)`;
- `public_monitor_selected(int)`, `stage_monitor_selected(int)`;
- `public_mode_selected(int)`, `stage_mode_selected(int)`;
- `public_visibility_toggled()`, `stage_visibility_toggled()`;
- `close_requested()`.

`MainWindow` emite `open_settings()`.

### 11.3 Menu

- O menu usa `PopupMenu` com um único `MenuItem`: “Configurações…”.
- O popup é ancorado abaixo do botão e fecha ao clicar fora.
- O item não depende de ícone; usa texto e o ícone de menu no botão.

## 12. Fluxo de inicialização

1. Resolver o diretório de dados, abrir o banco e aplicar migrações (schema v2).
2. Registrar a sessão em `app_meta` como hoje.
3. Carregar `OutputsConfig` do banco (ou padrões).
4. Criar e mostrar o painel administrativo; ligar menu e callbacks.
5. Depois de mostrar o painel, enumerar monitores e abrir as saídas habilitadas. A chamada é curta e síncrona; se a enumeração falhar, seguir com lista vazia.
6. Criar as saídas habilitadas, com placeholders e handles fortes.
7. Aplicar posicionamento/modo depois que as janelas existem.
8. Iniciar o timer do relógio se o Stage estiver aberto.
9. Rodar o loop de eventos do Slint.

## 13. Erros e estados degradados

| Situação | Comportamento |
|----------|---------------|
| Banco indisponível | Painel abre; saídas não abrem; Configurações mostram o erro e ficam desabilitadas |
| Falha ao enumerar monitores | Lista vazia; saídas abrem como janelas na posição padrão; aviso nas Configurações |
| Monitor salvo ausente | Usa o principal; aviso no cartão; preferência original preservada |
| Falha ao criar janela | Registra em `stderr`; estado “Falha ao abrir”; painel continua |
| Falha ao salvar configuração | Faixa de erro nas Configurações; configuração em memória continua aplicada |
| Modo/monitor inválido no banco | Cai para `Auto`, sem impedir a abertura |
| Fechar saída pelo X | Apenas oculta; `enabled` permanece; volta na próxima execução |

## 14. Testes

### 14.1 `op-core`

- Round-trip de `OutputKind`, `OutputMode` e `MonitorSelection` por string de banco.
- Rótulos em português.
- Padrões de Public e Stage.
- `OutputsConfig::get` e iteração.

### 14.2 `op-db`

- Banco novo cria schema v2 com `output_config`.
- Banco v1 existente migra para v2 preservando `app_meta` e `first_run_at`.
- `load_outputs` sem linhas devolve os padrões.
- Upsert e leitura de Public e Stage.
- Atualização de uma saída não altera a outra.
- Valores inválidos de modo/monitor caem para `Auto` (teste com SQL direto).
- `schema_version()` devolve 2.

### 14.3 `op-admin`

- Resolução `Auto` escolhe externo; sem externo, principal; sem principal, primeiro.
- `Primary` escolhe o principal.
- `Display` por id; fallback por nome; fallback para principal com aviso.
- Lista vazia não entra em pânico e produz posição padrão.
- Normalização de coordenadas por plataforma (função pura testável).
- Conversão de tamanho lógico para físico pela escala do monitor, incluindo cascata e centralização.
- Retângulo de janela: centralizado no monitor de destino; cascata no principal; clamp dentro dos limites.
- Configuração padrão abre Public e não abre Stage.
- Desativar remove a janela; ativar cria.
- Fechar pelo X mantém `enabled`.
- Falha de banco não abre saídas.
- O timer do Stage não é criado quando o Stage está fechado.

### 14.4 Manual/visual

- Abrir o menu e a janela de Configurações.
- Com um monitor: ativar Public e Stage; alternar Janela/Maximizada/Tela cheia; fechar pelo X e reabrir; reiniciar o app e confirmar restauração; sair da tela cheia pelos atalhos do sistema.
- Com dois monitores (quando disponível): Public no monitor secundário em tela cheia automática; Stage no monitor escolhido; trocar monitor com as janelas abertas.
- Tema claro/escuro do painel; saídas permanecem legíveis no tema escuro fixo.
- Estado degradado: banco indisponível; salvar configuração com erro.
- Reiniciar com monitor salvo desconectado: aviso e fallback para o principal.

## 15. Critérios de aceite

1. O cabeçalho do painel tem um menu que abre **Configurações**.
2. A janela de Configurações tem a aba **Saídas** com cartões Public e Stage.
3. Cada cartão permite ativar, escolher monitor e escolher modo.
4. Public vem ativo, em monitor automático e modo automático; Stage vem desativado.
5. Com um monitor, Public e Stage abrem como janelas; maximizar e tela cheia funcionam.
6. Com mais de um monitor, é possível escolher o monitor de cada saída e a janela abre nele.
7. As saídas habilitadas reabrem na inicialização, no monitor salvo.
8. Mudanças de configuração são aplicadas sem reiniciar.
9. Fechar uma saída pelo X apenas a oculta; ela não é desativada.
10. Monitor ausente cai para o principal com aviso, sem apagar a preferência.
11. Falha de enumeração de monitores não impede o app de abrir.
12. Banco indisponível não abre saídas e mostra o erro nas Configurações, sem pânico.
13. A migração v1 → v2 preserva `app_meta` e `first_run_at`.
14. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` passam.
15. Documentação, changelog, ADR e versão 0.2.0 atualizados.

## 16. Riscos e decisões em aberto

- **Coordenadas por plataforma:** a normalização para pixels físicos é a principal fonte de erro em multi-monitor. As dimensões de janela são lógicas e convertidas pela escala do monitor; a função será isolada e testada. A validação visual será feita no macOS, e Windows/Linux ficam cobertos por compilação e testes da lógica.
- **Identidade de monitor:** a restauração é best-effort por `id` e depois por nome. No Windows, o handle e o nome podem mudar com a porta ou a configuração; o app cai para o principal com aviso e preserva a preferência.
- **Threading:** enumeração e gravação são síncronas e curtas na thread principal. Se a enumeração se mostrar lenta em alguma plataforma, a saída é movê-la para um worker apenas onde a API permitir; o macOS continua exigindo a thread principal.
- **Fullscreen no monitor correto:** o Slint entra em tela cheia no monitor onde a janela está. A sequência será posicionar, mostrar e só então aplicar fullscreen; se o sistema ignorar a posição inicial, o fallback é abrir maximizado e registrar o aviso.
- **Wayland:** posicionamento de janelas pode ser ignorado pelo compositor. O app não quebra; a janela abre onde o sistema decidir.
- **Hot-plug:** não há watcher em tempo real nesta versão. A lista é recalculada ao abrir as Configurações e na inicialização.
- **Múltiplas saídas no mesmo monitor:** é permitido; as janelas em modo janela usam cascata para não ficarem exatamente sobrepostas.
- **Dependência `display-info`:** revisar licença e manutenção antes de fixar a versão; manter o fallback para lista vazia.

## 17. Decisões deixadas ao construtor

- Layout exato dos placeholders e tipografia dentro dos tokens existentes.
- Tamanho e posição exatos da janela de Configurações, desde que respeitem o mínimo.
- Estratégia de animação/transição ao abrir/fechar saídas (pode ser imediata nesta versão).
- Formato exato do relógio e da data por extenso.
- Como agrupar os monitores no seletor quando dois têm o mesmo nome (sufixo com resolução ou id).
- Detalhes de acessibilidade dos novos controles, seguindo os padrões do Material.

## 18. Respostas da checagem de construtor

1. **Threading.** Enumeração e gravação são síncronas e curtas na thread principal; a enumeração é executada depois que o painel é mostrado. O anti-objetivo foi reescrito para proibir apenas trabalho longo ou perceptível, não qualquer chamada síncrona.
2. **Foco e tela cheia.** “Sem foco” significa ausência de campos focáveis, não impedir que o sistema ative a janela. A tela cheia usa o `set_fullscreen` nativo; no macOS isso cria um Space, comportamento aceito nesta versão.
3. **Saída da tela cheia com um monitor.** O modo é uma escolha explícita do operador. Para voltar, usam-se os atalhos do sistema (Cmd+` no macOS, Alt+Tab no Windows/Linux); não há atalho local porque as saídas não têm teclado.
4. **Identidade do monitor.** A preferência guarda `id` e nome amigável; a resolução tenta `id`, depois nome, depois principal com aviso. A restauração garantida é validada no macOS; Windows/Linux são best-effort documentado.
5. **Unidades.** Tamanhos de janela e cascata são lógicos e convertidos para físicos pela escala do monitor de destino; posições e retângulos de monitor são físicos. A conversão fica em função pura testável.

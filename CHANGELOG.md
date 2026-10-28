# Changelog

Todas as mudanças relevantes deste projeto são registradas neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e o projeto adere a [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Adicionado

- Novo painel principal operacional com 3 áreas integradas: Sidebar de Mídias, Painel de Programação (ordem do culto) e Painel de Preview de Mídias com controles de projeção (Ao Vivo, Limpar Tela, Ocultar Texto, Logotipo).
- Command Palette acionada por `⌘K` (macOS) ou `Ctrl+K` (Windows/Linux) ou clique no header, com busca rápida, categorias (Músicas, Bíblia, Comandos) e arquitetura extensível de comandos.
- Abertura da janela principal maximizada cobrindo o espaço de trabalho disponível do monitor primário.
- Isolamento coordenado na inicialização entre a janela administrativa e as janelas de saída de tela cheia.

### Corrigido

- Correção do cancelamento involuntário de tela cheia ao configurar saídas: `reposition` passa a ser ciente de estado e não força saída de tela cheia se a janela já estiver no modo desejado.
- Aplicação de configuração isolada por saída (`apply_config_for`), evitando reprocessar e resetar o modo de outras saídas abertas.
- Conteúdo das janelas de saída Public e Stage agora é responsivo e proporcional às dimensões da tela, preenchendo o espaço em tela cheia e resoluções elevadas.
- Retorno de palco (`Stage`) com layout distribuído: cabeçalho e data no topo, relógio ampliado em destaque central e rodapé de evento/apresentação.

## [0.2.0] - 2026-10-28

### Adicionado

- Saídas **Public** e **Stage** como janelas Slint independentes, com placeholders de marca; o Stage inclui relógio e data por extenso.
- Janela de **Configurações** independente, aberta pelo menu **☰ → Configurações…**, com a aba **Saídas**.
- Ativação, escolha de monitor e escolha de modo de exibição por saída, com aplicação imediata.
- Restauração das saídas habilitadas na inicialização, cada uma no monitor salvo.
- Migração v2 com a tabela `output_config` e as operações `load_outputs`/`save_output`.
- Dependência `display-info` 0.5 para enumeração de monitores em `op-admin`.
- Estados degradados para banco indisponível, falha ao salvar, falha de enumeração de monitores e falha ao abrir janelas.

### Alterado

- O schema do banco passa de v1 para v2, preservando `app_meta` e `first_run_at`.
- O painel principal ganha o menu interno e a nova copy da versão.
- Fechar uma saída pelo X apenas a oculta; desativar fecha a janela e persiste `enabled = false`.

## [0.1.0] - 2026-10-28

### Adicionado

- Workspace Rust com `op-core`, `op-db` e `op-admin`.
- Janela nativa Slint Material com a identidade visual do OpenPresenter.
- Banco SQLite local com schema v1 e tabela `app_meta`.
- Registro de primeira e última execução, versão do app, tema e idioma.
- Estado degradado na interface quando o banco não está disponível.
- Documentação de fundação, arquitetura, decisões e política de marca.
- Licença GPL-3.0-only.

# Especificação Técnica: Versão 0.2.0 — Mídias em Vídeo, Busca Bíblica Global, Conexões em Tempo Real e Refinamento de Craft

## 1. Intenção e Motivação

Preparar a release estável **v0.2.0** do OpenPresenter Minimal para distribuição aos usuários e igrejas.
Esta versão representa um salto qualitativo significativo sobre a versão inaugural v0.1.0, incorporando:
- Suporte a reprodução de clipes de vídeo em loop para planos de fundo de louvor;
- Sistema dinâmico e responsivo de auto-scale de fontes para projeção pública e retorno de palco;
- Busca unificada e instantânea de passagens bíblicas por referência e palavras-chave no Command Palette global (`Ctrl+K` / `Cmd+K`);
- Indicador flutuante no cabeçalho com listagem em tempo real de conexões WebSocket ativas (dispositivos, IPs e rotas);
- Refinamento completo de acessibilidade (WAI-ARIA), conformidade com `prefers-reduced-motion` e eliminação de todos os anti-patterns de interface detectados pela auditoria Impeccable.

---

## 2. Escopo do Lançamento v0.2.0

1. **Unificação de Versão (0.2.0):**
   - Atualizar a versão do workspace Rust em `Cargo.toml` (`[workspace.package] version = "0.2.0"`).
   - Atualizar a versão do pacote do frontend em `frontend/package.json` (`"version": "0.2.0"`).
   - Atualizar os fallbacks na interface do operador (`SettingsModal.svelte`).
   - Atualizar a tag de fallback no fluxo de release do GitHub Actions (`.github/workflows/release.yml`).
   - Fechar a versão `## [0.2.0] - 2026-10-30` no `CHANGELOG.md` e reabrir a seção vazia `## [Unreleased]`.

2. **Novos Recursos e Funcionalidades Principais:**
   - **Planos de Fundo em Vídeo e Customização CSS:** Importação de mídias de vídeo (.mp4, .webm, .mov) para planos de fundo contínuos e silenciosos na tela pública e previews, além de editor visual de CSS e cor de letras nas Configurações.
   - **Busca Bíblica no Command Palette Global:** Localização instantânea de textos por referência (`João 3:16`, `Sl 23`) e busca textual por termos com projeção direta (`Enter`) ou adição à setlist (`Shift+Enter`).
   - **Conexões WebSocket Ativas:** Painel dropdown no botão "Online" exibindo operador, telas públicas e monitores de palco conectados com IP e tempo de conexão.
   - **Navegação Hierárquica de Imagens:** Varredura recursiva de diretórios locais com árvore de navegação e breadcrumbs para sub-pastas.
   - **Auto-Scale Dinâmico em Telas de Saída:**
     - Projeção Pública (`/public`): Auto-scale inteligente sem quebras indesejadas e com limites de fonte máxima e mínima configuráveis.
     - Monitor de Palco (`/stage`): Padronização no preset **Grid Moderno Pro** com maximização da área útil e ausência de transbordamento.
   - **Modo Standby no Monitor de Palco:** Relógio gigante e cronômetro limpo em repouso quando não há apresentação de slides ativa.
   - **Acessibilidade & Theming Impeccable:** Padrão WAI-ARIA tablist completo, handle resizer com navegação por teclado, transições suaves com redução de movimento (`prefers-reduced-motion`) e tokens semânticos de alto contraste.

3. **Correções e Estabilidade:**
   - Ícone do System Tray oficial com transparência no macOS Menu Bar e suporte a templates claro/escuro.
   - Limpeza do cabeçalho da Setlist e prevenção de limpezas acidentais de itens.
   - Correção do dimensionamento no catálogo de músicas com painel de prévia aberto.
   - Correção de tela preta e erros JavaScript no modo standby do monitor de palco.

---

## 3. Decisões Técnicas

- **SemVer:** Incremento de minor (`0.1.0` -> `0.2.0`) refletindo a introdução de novos recursos substanciais mantendo compatibilidade retroativa com os bancos de dados SQLite v7 e arquivos do usuário.
- **Isolamento de Publicação:** O processo de preparação prepara todos os arquivos e builds localmente sem executar push remoto (`git push`), aguardando o comando explícito do mantenedor.

---

## 4. Critérios de Aceite

1. `Cargo.toml` declara `version = "0.2.0"`.
2. `frontend/package.json` declara `"version": "0.2.0"`.
3. `op_core::VERSION` compila e reporta `"0.2.0"`.
4. `CHANGELOG.md` documenta todas as entregas sob `## [0.2.0] - 2026-10-30`.
5. Todos os portões de qualidade Rust (`cargo fmt`, `cargo clippy`, `cargo test`) e frontend (`npm check`, `npm build`) passam com zero erros.
6. As alterações são commitadas localmente sem envio (`git push`).

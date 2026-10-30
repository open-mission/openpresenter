# Plano: Integração com Google Cast (Chromecast) para Telas Pública e Retorno de Palco

Plano de execução para a especificação [`docs/specs/2026-10-30-integracao-google-cast.md`](../specs/2026-10-30-integracao-google-cast.md).

## Fases de Implementação

### Fase 1: Módulo Central do Google Cast (`frontend/src/lib/cast.ts`)
- [ ] Criar gerenciador de Cast com carregamento assíncrono do SDK Google Cast Sender (`cast_sender.js`).
- [ ] Criar store reativo (`castStore`) com estado da sessão (disponibilidade, conectado, conectando, nome do dispositivo, alvo ativo).
- [ ] Implementar métodos `startCasting(target: 'public' | 'stage')` e `stopCasting()`.
- [ ] Implementar comunicação com o Web Receiver enviando o redirecionamento para o IP local obtido de `/api/system/network-info`.

### Fase 2: Componente de Interface e Controle (`frontend/src/lib/components/CastControl.svelte`)
- [ ] Desenvolver componente visual com opções de transmissão para Tela Pública e Monitor de Palco.
- [ ] Adicionar status visual em tempo real (dispositivo conectado, tempo de sessão, botão de desconectar).
- [ ] Adicionar guia passo a passo para contingência de espelhamento nativo de aba via Google Chrome / Edge.

### Fase 3: Integração no Modal de Saídas e Cabeçalho do Operador
- [ ] Adicionar aba "Chromecast" em [`ShareOutputsModal.svelte`](../../frontend/src/lib/components/ShareOutputsModal.svelte).
- [ ] Incluir atalho de Cast no menu de Saídas do cabeçalho em [`AdminView.svelte`](../../frontend/src/lib/views/AdminView.svelte).
- [ ] Exibir indicador visual de tela transmitida via Cast quando ativa.

### Fase 4: Portões de Qualidade e Documentação
- [ ] Executar `cargo fmt --all`.
- [ ] Executar `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Executar `cargo test --workspace`.
- [ ] Executar checagem do frontend `npm run check`.
- [ ] Atualizar `CHANGELOG.md` e `docs/README.md`.

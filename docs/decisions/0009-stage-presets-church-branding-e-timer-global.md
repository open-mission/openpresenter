# ADR 0009: Presets de Stage Display, Identidade da Igreja e Timer Global com Overtime

- **Status:** Aceito
- **Data:** 2026-10-29
- **Decisores:** Antigravity & Usuário
- **Substitui:** -
- **Substituído por:** -

---

## Contexto

Ambientes de projeção para igrejas e eventos demandam controle refinado tanto da experiência visual do público no templo (projeção no telão/projetor principal) quanto da experiência de palco da equipe de ministração, louvor e pregação (Stage Display / monitores de retorno).
Especificamente:
1. **Controle de Aproveitamento e Margens no Public:** Letras com diferentes comprimentos precisam de preenchimento inteligente (auto-fit), controle opcional de quebra de linha (para manter versos intactos em uma só linha) e controle de exibição de metadados do louvor, além de margens perimetrais ajustáveis para compensar distorções ópticas ou áreas de borda de projetores.
2. **Identidade Visual da Igreja:** Capacidade de exibir o logotipo institucional da igreja sobreposto nos slides de projeção pública com variações (principal, dark, light e completa), posicionamento em 4 cantos e controle de opacidade.
3. **Padrões de Stage Display do Setor:** O retorno de palco necessita de múltiplos layouts pré-configurados (Presets) inspirados nas ferramentas de referência da indústria (ProPresenter, Proclaim): Split 50/50 com bordas arredondadas e etiquetas, Modern Grid, Foco em Vídeo/Contagem e Horizontal Stack, além de personalização de cores para slide atual, próximo slide, bordas e textos.
4. **Timer Global e Relógio com Overtime:** O operador precisa de um painel de cronometragem acessível diretamente na cabine com presets ágeis (5 min, 10 min, 15 min, 30 min, etc.) e controles de Play, Pause e Reset. Quando o tempo de ministração esgota, o display deve entrar imediatamente em contagem negativa em vermelho pulsante (Overtime, ex: `-02:30`) para alertar o ministro sem interromper agressivamente a transmissão.

---

## Decisões

1. **Persistência de Preferências em Tabela SQLite Dedicada (Schema v6):**
   - Criação da migração `0006_app_settings.sql` com tabela de chave-valor `app_settings` para persistência no SQLite de configurações de Public, Marca e Stage.
2. **TimerState Integrado ao Estado Central da Apresentação:**
   - O `TimerState` (`running`, `duration_seconds`, `remaining_seconds`, `label`, `stage_message`, `warning_threshold_seconds`) é mantido pelo servidor backend Axum e sincronizado em tempo real via WebSocket com todos os clientes conectados (Admin, Stage, Public).
3. **Lógica de Overtime Negativo e Alerta Visual:**
   - O timer não para no zero: caso ultrapasse a duração planejada, a contagem continua em valores negativos (`remaining_seconds < 0`), disparando classes CSS de alerta pulsante (`animate-pulse text-red-500 font-bold`).
4. **Painel de Controle Dedicado na Sidebar do Operador:**
   - Adicionar o `TimerControlPanel` diretamente na coluna lateral direita do painel Admin, abaixo das caixas de preview de monitores.
5. **Favicon Oficial:**
   - Vinculação de `open-presenter-app-icon.svg` como favicon oficial do app.

---

## Consequências

- **Positivas:**
  - Telas de retorno e projeção com acabamento visual e usabilidade de nível profissional.
  - Sincronização impecável do tempo de culto/palestra entre cabine e palco.
  - Flexibilidade para igrejas exibirem sua marca com elegância e ajustarem projetores com bordas personalizadas.
- **Negativas / Mitigações:**
  - O loop de contagem de segundos precisa ser resiliente a suspensões do navegador. Mitigado usando timestamps de epoch no backend e sincronização WebSocket imediata.

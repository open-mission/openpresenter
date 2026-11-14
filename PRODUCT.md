# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

- **Operador de mídia ao vivo (usuário primário, prioridade máxima)**: responsável por operar projeções, letras, passagens bíblicas, mídias, avisos e cronômetro na mesa de som/projeção durante cultos e eventos ao vivo. Precisa de resposta instantânea, controle confiável pelo teclado e previsibilidade absoluta no ar.
- **Audiências secundárias (consumo e apoio)**:
  - **Congregação / plateia** (`/public` e Google Cast): acompanha letras, versículos, imagens, vídeos, avisos de agenda e timer com legibilidade a distância.
  - **Equipe de palco — cantores, músicos e pregador** (`/stage`): acompanha slide atual, próximo slide, relógio, cronômetro com overtime e avisos técnicos.
  - **Operação de transmissão ao vivo** (`/lower-third`): integra legendas e banners transparentes em softwares como OBS Studio e vMix.
  - **Voluntários e líderes de ministério** (`/remote` + Teams): consultam a escala do culto, confirmam presença no celular e, quando autorizados por PIN/permissão, apoiam a operação remotamente.

## Product Purpose

Permitir que igrejas e eventos conduzam projeções multi-tela ao vivo (telão público, retorno de palco, transmissão OBS e controle remoto mobile) de forma rápida, estável e totalmente local, mantendo a experiência da mesa do operador como centro das decisões e usando os fluxos remotos e de equipe como apoio.

## Positioning

Diferencia-se de soluções tradicionais (como Holyrics e ProPresenter) por entregar **leveza em binário único Rust + SQLite**, operação **100% offline e local** sem dependência de nuvem, mapeamento **zero-copy** de pastas de mídia sem duplicar arquivos em disco e **multi-tela instantânea via navegador/PWA**, tudo sob licença aberta GPL-3.0-only.

## Operating Context

- **Ambiente de uso**: cultos e eventos ao vivo em rede local (LAN) ou máquina dedicada, servidos localmente por padrão na porta `18080` com sincronização em tempo real via WebSocket (`/ws`).
- **Fluxo da mesa ao vivo**: disparo desacoplado (preparar sem interromper o que está no ar, projetar com `Enter`, parar com `Esc`), atalhos diretos de seções (`R`, `V`, `P`, ordinais), modos instantâneos (`live`, `blackout`, `cleartext`, papel de parede) e visualização em item focado ou fluxo contínuo (*Show Flow*).
- **Implantação e operação**: executável desktop com ícone na bandeja do sistema (*System Tray*) ou servidor *headless* em contêiner Docker.
- **Idioma**: textos de interface e documentação operacional em português do Brasil (`pt-BR`).

## Capabilities and Constraints

- **Capacidades confirmadas no sistema**:
  - **Setlists e Apresentação**: múltiplas setlists com drag & drop, seções agrupadoras, itens de música, Bíblia, imagem, pasta de imagens, vídeo, áudio, texto livre, aviso, agenda (*Schedule*) e timer.
  - **Músicas e Letras**: busca instantânea SQLite FTS5 sem acentos, tags, parser de seções estilo Holyrics, editor Markdown/alinhamento, áudio vinculado (playback), importação (JSON/TXT/MD), instalação offline da Harpa Cristã e busca online (Holyrics/Vagalume).
  - **Bíblias**: módulos `.sqlite` locais, busca exata e parser de referências bíblicas.
  - **Mídias e Fundos**: árvores de pastas do app e pastas externas mapeadas (*zero-copy*), streaming `HTTP Range` (206) para vídeos e áudios, player independente e fundos CSS/mesh/animados.
  - **Saídas Especializadas**: `/public` (auto-scale, safe area, marca da igreja, waveforms de áudio, timer overlay, Google Cast), `/stage` (preset único `Grid Moderno Pro` em 2 layouts: `side-column` e 3 linhas empilhadas, além do modo descanso e alerta fullscreen pop-in) e `/lower-third` (4 estilos com canal alfa transparente).
  - **Remoto, Equipes e Sistema**: PWA `/remote` com login local por voluntário/PIN e permissões granulares; módulo Teams com escalas em grafo (`@xyflow/svelte`) e grade + exportação WhatsApp; telemetria de hardware; logs de auditoria; API docs offline (`/docs`, `/swagger`); backup/restauração `.opbackup` e auto-updater via GitHub Releases.
- **Restrições duráveis**:
  - Direção de dependências estrita: `frontend → op-admin → op-db → op-core` (`op-core` sem UI nem SQLite).
  - Nenhuma funcionalidade crítica de culto pode exigir conexão com a internet.
  - Migrações SQLite já aplicadas são imutáveis (apenas novas migrações aditivas numeradas); manter `AGENTS.md` e `GEMINI.md` sincronizados.
  - Nota de alinhamento em aberto: `README.md` ainda menciona `v0.7.0` e "4 presets" de retorno de palco, mas o estado vigente confirmado segue o código atual (`v0.6.0` e preset único de Stage em 2 layouts).

## Brand Commitments

- **Nome oficial e licença**: **OpenPresenter** (edição atual **OpenPresenter Minimal**), código sob `GPL-3.0-only`, com nome e identidade visual protegidos por `TRADEMARK.md` (derivados/forks devem usar identidade própria).
- **Assets oficiais vinculantes**: arquivos em `docs/brand/` e `frontend/public/branding/` (`open-presenter-app-icon.svg`, `open-presenter-wordmark.svg`, `open-presenter-wordmark-dark.svg` e variantes de ícone) não devem ser editados, recoloridos, distorcidos ou alterados em proporção; novas peças precisam ser documentadas em `docs/brand/README.md`.

## Evidence on Hand

- Código-fonte completo do workspace (`crates/op-core`, `crates/op-db`, `crates/op-admin`, `frontend/`), 13 migrações SQLite, specs/planos/ADRs em `docs/`, marca oficial em `docs/brand/` e `frontend/public/branding/`, capturas de referência em `docs/screenshots/` (`main.png`, `public.png`, `stage.png`) e especificação OpenAPI offline.
- **Ausências registradas (não inventar)**: não há métricas de adoção em campo, depoimentos de igrejas, contagens de instalações ou benchmarks formais de latência/carga no repositório.

## Product Principles

1. **Mesa ao vivo acima de tudo**: em qualquer conflito de interface, velocidade de disparo, clareza visual e operação por teclado na mesa têm prioridade sobre recursos secundários.
2. **Offline-first e zero-copy por padrão**: o culto não pode depender de rede externa e mídias locais devem rodar direto do disco sem cópias redundantes.
3. **Separação estrita de superfícies**: o que é controle operacional fica no Admin/Remoto; telas públicas, palco e transmissão OBS permanecem livres de ruído.
4. **Enxuto e estável em qualquer máquina**: preservar a leveza do "Minimal" evitando inchaço de menus, fluxos burocráticos ou dependências pesadas.

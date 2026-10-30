# Spec: Modo de Descanso (Standby) com Relógio e Timer no Monitor de Palco (Stage View)

## Intenção
Oferecer uma experiência profissional no monitor de palco (`/stage`) durante os períodos de repouso (quando nenhuma música, texto ou versículo bíblico estiver em exibição). Em vez de manter caixas vazias de slides, a tela do palco comuta automaticamente para o Modo de Descanso, exibindo as horas em tipografia gigante e elegante, o cronômetro/timer centralizado e a mensagem do pastor/líder se houver.

## Escopo
- Frontend (`StageView.svelte`):
  - Detectar quando não há slide ativo (`!activeSlide`).
  - Renderizar uma visualização imersiva e de altíssima legibilidade focada em:
    - Relógio de horas/minutos/segundos em formato gigante no centro.
    - Data por extenso em português.
    - Widget do Cronômetro/Timer (em repouso ou contagem regressiva) em destaque com indicação de progresso e alertas visuais de aviso e overtime.
    - Mensagem de palco (`stageMessage`) em banner de destaque caso preenchida pelo operador.
    - Título da Setlist / Culto ativo e identificação visual discreta.
  - Ao iniciar qualquer apresentação de slide, alternar com transição suave para o layout normal de palco selecionado (`split-50-50`, `modern-grid`, etc.).
- Frontend (`StagePreviewBox.svelte`):
  - Espelhar o Modo de Descanso na prévia do operador dentro do painel lateral.

## Decisões
- O Modo de Descanso é acionado automaticamente em repouso (quando o papel de parede da setlist está em exibição e não há slide ao vivo).
- Garante total visibilidade do horário e tempo para pastores, músicos e pregadores no altar mesmo antes do início do culto ou entre as ministrações.

## Critérios de Aceite
1. Quando nenhuma mídia estiver sendo projetada, o monitor `/stage` exibe o relógio e cronômetro gigantes centralizados.
2. Ao disparar uma música ou versículo, o `/stage` exibe os slides Atual e Próximo conforme o preset configurado.
3. Ao cancelar a apresentação (Esc / Parar / Limpar), o `/stage` retorna imediatamente ao Modo de Descanso.
4. O preview no painel do operador reflete fielmente o estado de descanso.

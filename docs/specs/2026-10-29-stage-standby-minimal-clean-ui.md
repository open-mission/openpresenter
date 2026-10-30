# Spec: Interface Minimalista e Limpa no Modo Standby do Stage

## Intenção
Refinar o Modo de Descanso (Standby) do monitor de palco (`/stage`) para remover elementos visuais desnecessários de interface administrativa (pills, badges e avisos de rodapé) e condicionar a exibição do cronômetro exclusivamente aos momentos em que o timer estiver em execução (`running`) ou em tempo excedido (`overtime`).

## Escopo
- `frontend/src/lib/views/StageView.svelte`:
  - Remover a barra/badge superior de identificação `"Modo de Descanso (Standby)"`.
  - Posicionar o controle de tela cheia discretamente no canto superior direito.
  - Condicionar a renderização do bloco do cronômetro à condição `timer && (timer.running || timerIsOvertime)`.
  - Remover a barra de texto de orientação do rodapé (`"Aguardando início de slide ou louvor pelo operador"`).
- `frontend/src/lib/components/StagePreviewBox.svelte`:
  - Alinhar o preview do operador para refletir a mesma limpeza visual (relógio em destaque e cronômetro apenas quando ativo).

## Critérios de Aceite
1. Quando não há apresentação ativa e o cronômetro está parado, o monitor de palco exibe apenas o relógio gigante e a data por extenso centralizados.
2. Ao iniciar o cronômetro no painel do operador, o bloco do cronômetro aparece imediatamente abaixo do relógio com animação e cores de status (tempo normal, alerta e overtime).
3. Não são exibidos textos como "Modo de Descanso (Standby)" ou "Aguardando início de slide ou louvor pelo operador".
4. O botão de tela cheia permanece acessível de forma discreta no canto superior direito ou via tecla 'F' / duplo clique.

# Spec: Opção de Exibir Textos nos Botões de Controle de Saída

## Intenção
Permitir que os botões de controle de projeção (Ao Vivo, Parar, Blackout, Limpar Texto e Fundo/Wallpaper) na barra lateral de saídas operem por padrão em modo compacto (somente ícones) para economizar espaço vertical e horizontal, disponibilizando uma opção na aba de Aparência das Configurações para que o operador possa ativar os textos descritivos caso prefira.

## Escopo
- Frontend: Adicionar configuração reativa e persistente `previewLabelsStore` em `settings.ts`.
- Frontend: Adicionar controle do tipo switch toggle na aba "Aparência & Cores" em `SettingsModal.svelte`.
- Frontend: Atualizar os botões em `AdminView.svelte` para renderizar apenas os ícones quando a opção estiver desativada (padrão) e com textos quando ativada.

## Decisões
- O valor padrão é `false` (somente ícones), atendendo à solicitação do operador de ganhar espaço.
- Manter tooltips (`title`) completos em cada botão em ambos os modos para total clareza e acessibilidade (indicando as teclas de atalho: Esc, ., ,, /).
- Persistência em `localStorage` sob a chave `open_presenter_preview_labels`.

## Critérios de Aceite
1. Na visualização inicial do painel do operador, os botões de controle de projeção exibem apenas os ícones centralizados.
2. Ao passar o mouse sobre cada botão, o tooltip explicativo com atalhos de teclado é exibido.
3. Ao abrir "Configurações" -> "Aparência & Cores", há um switch para ativar/desativar os rótulos de texto.
4. Ao ativar o switch, os rótulos aparecem imediatamente nos botões sem necessidade de recarregar a página.
5. A preferência é salva e restaurada ao reiniciar o navegador.

# Spec: Modo Compacto e Exibição de Rótulos nos Botões da Interface

## Intenção
Permitir que os botões da interface do operador (barra de navegação dos slides, barra lateral de saídas e cabeçalho de atalhos rápidos) operem por padrão em modo compacto (somente ícones) para economizar espaço de tela útil, disponibilizando uma opção na aba de Aparência das Configurações para que o operador possa ativar os textos descritivos (labels) caso prefira.

## Escopo
- Frontend: Configuração reativa e persistente `previewLabelsStore` em `settings.ts`.
- Frontend: Controle switch toggle na aba "Aparência & Cores" em `SettingsModal.svelte`.
- Frontend: Barra de slides em `AdminView.svelte` (Apresentar, Anterior, Próximo, Parar) operando em modo compacto por padrão e alternando condicionalmente com labels.
- Frontend: Botões do cabeçalho em `AdminView.svelte` (Configurações, Iniciar Public, Iniciar Stage, QR Code) operando em modo compacto por padrão e alternando condicionalmente com labels.
- Frontend: Controles de projeção da barra lateral de saídas (Ao Vivo, Parar, Blackout, Limpar Texto e Fundo).
- Frontend: Ajuste no botão de ação "Apresentar" para usar variante com destaque visual estável quando em repouso e variante `live` (com pulso) apenas quando a mídia já se encontra transmitindo ao vivo.

## Decisões
- O valor padrão é `false` (somente ícones), maximizando a visibilidade do conteúdo dos slides e a largura de títulos.
- Manter tooltips (`title`) completos em cada botão em ambos os modos para total clareza e acessibilidade (com atalhos de teclado: Enter, Esc, setas, etc.).
- Persistência em `localStorage` sob a chave `open_presenter_preview_labels`.

## Critérios de Aceite
1. Na visualização inicial do painel do operador, os botões da barra de navegação de slides, da barra de saídas e do cabeçalho exibem apenas ícones centralizados e elegantes.
2. Ao passar o mouse sobre qualquer botão, o tooltip explicativo completo é exibido.
3. Ao abrir "Configurações" -> "Aparência & Cores", o switch permite ativar/desativar os textos (labels).
4. Ao ativar o switch, os textos aparecem imediatamente em todos os botões correspondentes sem recarregar a tela.
5. O botão "Apresentar" utiliza variante `default` sóbria quando a mídia não está ao vivo, mudando para `live` apenas quando a mídia ativa estiver sendo transmitida.
6. A preferência é salva e restaurada ao reiniciar o navegador.

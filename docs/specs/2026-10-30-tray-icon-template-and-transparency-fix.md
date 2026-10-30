# Spec: Correção de Transparência e Template do Ícone no System Tray

## Intenção
Corrigir o visual do ícone do OpenPresenter na bandeja do sistema operacional (System Tray / macOS Menu Bar). O ícone anterior apresentava cantos brancos e fundo escuro retangular sólido em formato de quadrado recortado, parecendo estranho e desconectado da barra de status do sistema operacional. A solução adota um bitmap com transparência total (sem moldura retangular de fundo) e ativa o modo template nativo (`with_icon_templated` no macOS), permitindo que o sistema operacional aplique a tintura correta (clara ou escura) de acordo com a aparência do sistema.

## Contexto e Diagnóstico
1. **Causa Raiz:**
   - Na tentativa anterior de remover o fundo branco do app icon, foi gerado `open-presenter-icon-dark-32.png` a partir do SVG que continha uma caixa de fundo retangular (`<rect fill="#111318"/>`). Além disso, fora do retângulo arredondado, os cantos foram preenchidos com branco opaco.
   - No System Tray (especialmente na barra de menus do macOS), ícones de bandeja nunca devem possuir molduras retangulares sólidas nem cantos opacos: eles devem ser glifos com fundo 100% transparente.
   - No macOS, a API nativa AppKit requer que imagens da Menu Bar sejam marcadas com `isTemplate = true`. No crate `tray-icon`, essa propriedade é configurada através de `TrayIconBuilder::with_icon_templated(icon)`, fazendo com que o macOS renderize a silhueta em branco em temas escuros e em preto em temas claros, além de gerenciar estados de foco/seleção.
2. **Decisões:**
   - Renderizar bitmaps 32x32 e 64x64 com fundo transparente a partir do glifo vetorial oficial preenchido (`open-presenter-icon-filled.svg`), com tela e borda sólidas e o botão play em espaço negativo vazado (alpha = 0).
   - Gerar `assets/branding/open-presenter-icon-tray-32.png` e `assets/branding/open-presenter-icon-tray-64.png`, atualizando também `assets/branding/open-presenter-icon-dark-32.png` com o mesmo bitmap limpo e transparente.
   - Atualizar `assets/branding/README.md` registrando as novas peças de marca.
   - Configurar `crates/op-admin/src/tray.rs` para carregar `open-presenter-icon-tray-32.png` e, no macOS, construir o `TrayIcon` via `.with_icon_templated(icon)`.
   - Adicionar teste unitário validando que o bitmap do ícone possui proporção adequada de transparência (cantos transparentes) e opacidade (glifo presente), impedindo regressões com caixas opacas ou bitmaps vazios.

## Critérios de Aceite
1. O ícone carregado pelo System Tray possui fundo 100% transparente, sem bordas ou molduras retangulares sólidas em volta da marca.
2. No macOS, o ícone é configurado como template (`with_icon_templated`), permitindo renderização monocromática adaptativa conforme o tema do sistema.
3. Teste unitário em `tray.rs` valida que o ícone carregado possui dimensões 32x32 e distribuição esperada de canais alfa (com transparência nas bordas e opacidade no centro da forma).
4. `assets/branding/README.md` documenta os novos assets de tray.
5. Todos os testes unitários e de integração do workspace continuam passando (`cargo test --workspace`).

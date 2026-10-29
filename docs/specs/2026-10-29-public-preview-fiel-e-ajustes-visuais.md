# Especificação: Preview Fiel da Tela Pública e Ajustes Visuais

## 1. Intenção

Garantir fidelidade visual absoluta entre a projeção pública real (`PublicView.svelte`) e o preview operacional exibido no painel de administração (`AdminView.svelte`), além de corrigir a legibilidade de referências bíblicas na tela pública e tornar o escurecimento de plano de fundo configurável (com opção de caixa de fundo para texto).

## 2. Escopo

1. **Escala da Referência Bíblica na Projeção Pública**:
   - Aumentar a escala tipográfica da referência bíblica (ex: `João 3:16` com sigla `[ARA]`), garantindo visibilidade clara para o público em telões e projetores de alta resolução.
2. **Backdrop e Placa de Leitura Configuráveis**:
   - Configuração `darken_background` (Padrão: `false`): película escura translúcida sobre todo o papel de parede quando houver slide ativo.
   - Configuração `text_box_background` (Padrão: `false`): placa translúcida com cantos arredondados posicionada diretamente atrás do texto/versículo para contraste seletivo.
   - Interface de configurações no modal (`SettingsModal.svelte`) com textos explicativos claros e profissionais em português (pt-BR).
3. **Componente Reutilizável `PublicPreviewBox.svelte`**:
   - Criação de um componente de preview com aspect ratio 16:9 idêntico ao layout de `PublicView.svelte`.
   - Suporte a papel de parede (cores CSS ou imagem com modo `cover`, `contain`, `repeat`, `center`), blackout, clear text, ajuste de imagens (`image_fit`), margens proporcionais, logo da igreja (`churchBrand`) e quebra/preenchimento de texto.
   - Substituição do preview simplificado em `AdminView.svelte` pelo `PublicPreviewBox`.
4. **Sincronização Reativa e Ajuste de Proporções do Stage**:
   - Garantir que alterações nos sliders de distribuição de proporções e cores do Stage reflitam dinamicamente no preview ativo do modal e no painel admin.

## 3. Decisões Arquiteturais

1. **Configuração Persistida em `PublicDisplaySettings`**:
   - Inclusão dos campos opcionais `darken_background?: boolean` e `text_box_background?: boolean`.
   - Valores padrão definidos como `false`, respeitando a solicitação expressa do usuário de que por padrão a tela pública venha sem o backdrop escuro sobre o wallpaper.
2. **Componente de Preview Espelho (`PublicPreviewBox.svelte`)**:
   - Segue o mesmo padrão estabelecido pelo `StagePreviewBox.svelte`, centralizando toda a lógica de renderização visual em miniatura proporcional sem duplicar regras ad-hoc.
3. **Reatividade em Svelte 5**:
   - Utilização de reatribuição imutável (`localStage = { ...localStage, [key]: val }`) para assegurar que modificações disparem atualizações imediatas em stores Svelte e nós reativos.

## 4. Critérios de Aceite

- [x] Referência bíblica na tela pública e no preview é grande, legível e destacada (ex: `text-2xl sm:text-3xl md:text-4xl font-extrabold`).
- [x] O papel de parede público não é escurecido por padrão (`darken_background: false`).
- [x] O usuário pode ativar nas configurações o escurecimento geral de fundo e/ou a placa de fundo nas caixas de texto.
- [x] O preview do Public no painel admin reproduz fielmente a tela pública (wallpapers, margens, quebra de texto, imagens e logo da igreja).
- [x] O preview do Stage no painel e na modal reflete as proporções e presets ativos em tempo real.
- [x] Portões de qualidade passam: `cargo fmt --all`, `cargo clippy`, `cargo test` e `bun run check` + `bun run build`.

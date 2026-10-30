# Spec: Ajuste de Margem para Logotipo da Igreja na Tela Pública

## Intenção
Permitir que o operador ajuste livremente o distanciamento (margem horizontal e vertical em pixels) do logotipo da igreja em relação às bordas da tela pública, possibilitando posicionamento preciso nos 4 cantos e evitando cortes em projetores ou televisores com overscan.

## Escopo
- Frontend: Adicionar `logo_margin_horizontal` e `logo_margin_vertical` (padrão 24px) na interface `ChurchBrandSettings` em `types.ts` e `DEFAULT_CHURCH_BRAND` em `displaySettings.ts`.
- Frontend: Adicionar controles interativos de slider na aba "Marca da Igreja" do modal de configurações (`SettingsModal.svelte`).
- Frontend: Atualizar o cálculo de posicionamento em `PublicView.svelte` e no preview proporcional de `PublicPreviewBox.svelte`.

## Critérios de Aceite
1. O modal de configurações em "Marca da Igreja" exibe controles deslizantes para "Margem Horizontal" e "Margem Vertical" quando a sobreposição do logo estiver ativada.
2. A alteração das margens reflete instantaneamente no preview da tela pública dentro do painel do operador.
3. Na tela `/public`, o logotipo é renderizado respeitando os valores exatos em pixels de distância dos limites da tela.
4. Os valores são salvos no banco SQLite de configurações e restaurados ao recarregar.

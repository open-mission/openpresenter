# Spec — Tipo de Mídia: Agenda (Schedule)

**Data:** 2026-10-31  
**Status:** Aprovado  
**Autor:** Antigravity / OpenPresenter Team  
**Plano correspondente:** [`docs/plans/2026-10-31-tipo-de-midia-agenda-schedule.md`](../plans/2026-10-31-tipo-de-midia-agenda-schedule.md)  
**Índice geral:** [`docs/README.md`](../README.md)

---

## 1. Intenção e Problema

Nas igrejas e eventos, é prática indispensável projetar a **Programação Semanal dos Cultos** (ex.: Terça de Oração, Quinta de Doutrina, Sábado dos Jovens, Domingo da Família) ou avisos de **Festividades e Eventos Futuros** (ex.: Congressos, Conferências, Vigílias e Retiros) com dias e horários detalhados.

Atualmente, para exibir essa agenda, o operador precisa recorrer a textos simples e desestruturados ou a imagens estáticas pré-editadas no Photoshop/Canva, sem flexibilidade de alteração rápida de última hora e sem integração com o Monitor de Palco.

Esta especificação define o novo tipo de mídia **Agenda (`MediaKind::Schedule`)**, oferecendo:
1. **Editor Ágil com Presets:** Tabela interativa para preenchimento de Dia/Data, Horário, Nome do Culto e Local/Detalhes, com atalhos de um clique para dias da semana (`+ Domingo`, `+ Quarta`, etc.).
2. **Projeção Pública Harmonizada (`/public`):** Renderização limpa com papel de parede ativo da setlist, badges de dia e horário com ícone, e título auto-dimensionado via `autoScale`.
3. **Sincronização Sequencial no Palco (`/stage`):** Cada evento compõe um slide individual da agenda. Quando o operador projeta o evento de **Segunda-feira**, o palco exibe **Segunda-feira** no bloco *Slide Atual* e **Terça-feira** no bloco *A Seguir (Próximo)*, mantendo os músicos e pregador cientes da sequência.

---

## 2. Escopo

### Dentro do escopo
- Extensão do enum `MediaKind` no backend (`crates/op-core/src/lib.rs`) com a variante `Schedule` (`"schedule"`), rótulo em português `"Agenda"`, e testes de serialização.
- Atualização do tipo `MediaKind` em `frontend/src/lib/types.ts`.
- Componente `ScheduleEditorModal.svelte` permitindo adicionar, reordenar, editar e excluir eventos da agenda.
- Opção "Agenda / Programação..." no menu `+` da setlist em `AdminView.svelte` e no menu de contexto de itens para reedição.
- Estilização de alto impacto na Tela Pública (`PublicView.svelte`) para itens de agenda, usando o papel de parede selecionado.
- Estilização no Monitor de Palco (`StageView.svelte`) e preview do operador (`StagePreviewBox.svelte`) destacando dia e horário atuais e seguintes.

### Fora do escopo
- Sincronização com calendários externos (Google Calendar, Outlook iCal) nesta fase inicial.

---

## 3. Decisões Arquiteturais e Modelo de Dados

### 3.1 Modelo de Dados (`MediaItem`)
O item da agenda é persistido como um `MediaItem` padrão:
```typescript
{
  id: "schedule-1730390000",
  title: "Programação Semanal dos Cultos",
  artist_or_ref: "Agenda Semanal",
  kind: "schedule",
  slides: [
    {
      id: "slide-1",
      label: "Segunda-feira",
      content: "19h30\nCulto de Oração e Doutrina\nTemplo Central",
      align_h: "center",
      align_v: "center"
    },
    {
      id: "slide-2",
      label: "Quarta-feira",
      content: "19h30\nCulto de Ensino\nTemplo Central e Online",
      align_h: "center",
      align_v: "center"
    }
  ]
}
```

Cada evento da programação é serializado como um slide:
- `slide.label`: Dia da semana ou data comemorativa (ex.: `"Domingo"`, `"15 a 17 de Nov"`).
- `slide.content`: Linha 1 = Horário; Linha 2 = Título do Culto/Evento; Linhas seguintes = Descrição/Detalhes.

Essa estrutura garante retrocompatibilidade e permite que qualquer visualizador (inclusive aplicativos remotos antigos) exiba o texto de forma legível.

---

## 4. Critérios de Aceite

1. `cargo test --workspace` compila com a variante `MediaKind::Schedule` e valida round-trips de serialização.
2. O operador pode abrir o menu `+` da setlist no painel principal e selecionar "Agenda / Programação...".
3. O modal `ScheduleEditorModal.svelte` abre com formulário dinâmico, presets rápidos de dias e validação de título.
4. Ao salvar, a agenda é adicionada à setlist ativa com a tag de tipo `"schedule"` e número correspondente de slides.
5. Na Tela Pública (`/public`), cada slide de agenda renderiza com o fundo da setlist, badges de destaque (dia e horário) e tipografia fluida com autoScale.
6. No Monitor de Palco (`/stage`), quando o slide de um dia estiver ativo, o slide atual mostra os dados daquele dia e o próximo slide mostra o dia seguinte da programação.
7. O operador pode clicar com o botão direito no item da setlist e escolher "Editar Agenda..." para modificar horários ou cultos existentes.

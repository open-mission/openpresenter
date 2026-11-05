# Spec — Proteção de Modais contra Fechamento Acidental e Configurações de Notificações

## 1. Intenção

1. Prevenir a perda acidental de conteúdo durante a criação ou edição de músicas, textos/apresentações rápidas, agendas e avisos/divisores:
   - Desabilitar o fechamento do modal ao clicar na área externa (backdrop/overlay).
   - Permitir fechar o modal exclusivamente via botão "Fechar" (X), botão "Cancelar" ou pressionando a tecla `Esc`.
   - Se houver dados preenchidos ou alterações não salvas (dirty state), solicitar confirmação explícita do usuário antes de descartar o conteúdo e fechar.
2. Fornecer controle total sobre as notificações do sistema (toasts) no painel de configurações:
   - Opção de **Modo Não Perturbe (Silenciar Notificações)**: quando ativada, suprime a exibição de toasts flutuantes na interface do operador para evitar distrações durante o culto/transmissão.
   - Opção de **Posição das Notificações na Tela**: escolha entre Superior Esquerdo (`top-left`), Superior Direito (`top-right`), Inferior Direito (`bottom-right`) e Inferior Esquerdo (`bottom-left`).

---

## 2. Contexto e Problema

1. **Perda de trabalho em editores:**
   - Ao criar uma música extensa, formatar slides de texto ou montar uma agenda detalhada, qualquer clique inadvertido fora da caixa do modal fechava imediatamente a janela sem salvar, descartando todo o trabalho do operador.
2. **Notificações intrusivas durante o culto:**
   - Durante a operação de um culto ou evento ao vivo, mensagens de toast avisando sobre clientes conectados, início de mídias ou ações podem ser inconvenientes ou cobrir elementos vitais do painel central.
   - Cada operador ou igreja possui um layout de tela preferido (ou monitores ultra-wide), de modo que ter a posição das notificações fixa no canto superior direito nem sempre é a escolha ergonômica ideal.

---

## 3. Escopo e Solução

### 3.1 Proteção contra Fechamento Acidental nos Modais

Modais abrangidos:
1. `SongEditorModal.svelte` (Modo Música e Modo Texto / Apresentação Rápida).
2. `ScheduleEditorModal.svelte` (Modo Agenda de Programação Semanal / Eventos).
3. `QuickItemModal.svelte` (Modo Aviso e Modo Divisor de Seção).

Comportamento:
- **Clique no Backdrop:** O evento `onclick` na área cinza externa ao modal não mais fechará o diálogo (`e.stopPropagation()` no container e remoção de chamadas de fechamento no clique do overlay).
- **Detecção de Dirty State:**
  - Ao abrir o modal, um snapshot dos campos é registrado (`initialSnapshot`).
  - Durante a edição, compara-se o estado corrente com o snapshot inicial.
- **Fechamento via Tecla `Esc` ou Botões (X / Cancelar):**
  - Se `isDirty === false`, fecha imediatamente.
  - Se `isDirty === true`, exibe diálogo nativo de confirmação (`confirm('Existem alterações não salvas. Deseja realmente descartar e fechar?')`).
  - Caso o operador cancele a confirmação, o modal permanece aberto intacto.
  - Se o salvamento for concluído com sucesso (`handleSave` / `saveContent`), o modal fecha diretamente sem perguntar confirmação.

### 3.2 Configurações de Notificações (Não Perturbe e Posição)

Armazenamento em `localStorage` e Stores reativas em `settings.ts`:
- `STORAGE_TOAST_DND_KEY = 'open_presenter_toast_dnd'` (`boolean`, padrão: `false`)
- `STORAGE_TOAST_POSITION_KEY = 'open_presenter_toast_position'` (`ToastPosition = 'top-left' | 'top-right' | 'bottom-right' | 'bottom-left'`, padrão: `'top-right'`)
- Stores exportadas: `toastDndStore` e `toastPositionStore`.
- Funções exportadas: `setToastDnd(enabled: boolean)` e `setToastPosition(position: ToastPosition)`.

Integração no Toast e Sonner:
- `toast.ts`:
  - `isToastDnd()` consulta o estado atual de Não Perturbe.
  - Funções de notificação (`notifySuccess`, `notifyError`, `notifyInfo`, `notifyWarning`, `handleActionLogNotification`) e métodos do objeto `toast` (`success`, `error`, `info`, `warning`, `message`) retornam silenciosamente quando Não Perturbe estiver ativo.
- `App.svelte`:
  - O componente `<Sonner />` recebe reativamente `position={$toastPositionStore}` no painel do operador.

Interface de Configurações (`SettingsModal.svelte`):
- Seção "Notificações do Sistema" adicionada na aba **Aparência & Interface**:
  - Switch interativo para ativar/desativar o Modo Não Perturbe com descrição clara.
  - Seletor em grade de 4 botões para as posições (`Superior Esquerdo`, `Superior Direito`, `Inferior Direito`, `Inferior Esquerdo`).
  - Botão de teste prático "Disparar Notificação de Teste" para visualizar a posição imediatamente na tela.

---

## 4. Critérios de Aceite

1. Clicar fora da caixa de qualquer modal de edição (Música, Texto, Agenda, Aviso, Divisor) não fecha a janela sob nenhuma hipótese.
2. Pressionar `Esc`, clicar no "X" ou no botão "Cancelar" sem ter modificado nada fecha o editor de imediato.
3. Se algum campo foi digitado ou alterado, `Esc`, "X" ou "Cancelar" solicita confirmação antes de descartar.
4. Salvar com sucesso fecha o modal sem exibir alerta de confirmação.
5. No modal de configurações, a aba Aparência & Interface exibe os controles de Não Perturbe e seletor de posição de notificações.
6. A escolha de posição altera a localização dos toasts em tempo real e é persistida no `localStorage`.
7. O modo Não Perturbe silencia todos os toasts de notificação enquanto estiver ativado.
8. Todos os portões de qualidade (`cargo fmt`, `cargo clippy`, `cargo test`, `bun run build`) passam com 100% de sucesso.

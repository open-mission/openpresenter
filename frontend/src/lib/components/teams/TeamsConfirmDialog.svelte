<script lang="ts">
  import {
    deleteDepartment,
    deleteMember,
    deleteRoster,
    removeRosterItem,
  } from '$lib/teams';
  import { notifyError, notifySuccess } from '$lib/toast';
  import ConfirmModal from '../bottom-panel/ConfirmModal.svelte';
  import { getConfirmCopy, type ConfirmTarget } from './teamConstants';

  interface Props {
    target: ConfirmTarget | null;
    onclose: () => void;
    oncompleted: () => void | Promise<void>;
  }

  let { target, onclose, oncompleted }: Props = $props();

  let isProcessing = $state(false);

  const copy = $derived(getConfirmCopy(target));
  const confirmLabel = $derived(target?.kind === 'item' ? 'Remover' : 'Excluir');

  async function handleConfirm() {
    if (!target || isProcessing) return;
    const current = target;
    isProcessing = true;
    try {
      if (current.kind === 'roster') {
        await deleteRoster(current.id);
        notifySuccess('Escala excluída', current.label);
      } else if (current.kind === 'member') {
        await deleteMember(current.id);
        notifySuccess('Voluntário excluído', current.label);
      } else if (current.kind === 'department') {
        await deleteDepartment(current.id);
        notifySuccess('Departamento excluído', current.label);
      } else {
        await removeRosterItem(current.rosterId, current.itemId);
        notifySuccess('Voluntário removido da escala', current.label);
      }
      onclose();
      await oncompleted();
    } catch (err) {
      notifyError(
        'Não foi possível concluir a ação',
        err instanceof Error && err.message ? err.message : 'Tente novamente.'
      );
    } finally {
      isProcessing = false;
    }
  }
</script>

<ConfirmModal
  open={target !== null}
  title={copy.title}
  message={copy.message}
  {confirmLabel}
  isProcessing={isProcessing}
  onconfirm={handleConfirm}
  onclose={() => {
    if (!isProcessing) onclose();
  }}
/>

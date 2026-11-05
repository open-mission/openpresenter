import { toast as sonnerToast } from 'svelte-sonner';
import { getInstanceId } from './instance';
import type { ActionLog } from './types';
import { loadInitialToastDnd, toastDndStore } from './settings';

let isDndActive = loadInitialToastDnd();
if (typeof window !== 'undefined') {
  toastDndStore.subscribe((val) => {
    isDndActive = val;
    if (val) {
      sonnerToast.dismiss();
    }
  });
}

export function isToastDnd(): boolean {
  return isDndActive;
}

const wrappedToast = {
  ...sonnerToast,
  success: (message: any, data?: any) => {
    if (isDndActive) return '';
    return sonnerToast.success(message, data);
  },
  error: (message: any, data?: any) => {
    if (isDndActive) return '';
    return sonnerToast.error(message, data);
  },
  info: (message: any, data?: any) => {
    if (isDndActive) return '';
    return sonnerToast.info(message, data);
  },
  warning: (message: any, data?: any) => {
    if (isDndActive) return '';
    return sonnerToast.warning(message, data);
  },
  message: (message: any, data?: any) => {
    if (isDndActive) return '';
    return sonnerToast.message(message, data);
  },
};

const toast = wrappedToast;
export { toast };

/**
 * Notificação simples de sucesso
 */
export function notifySuccess(message: string, description?: string) {
  if (isDndActive) return;
  sonnerToast.success(message, { description, duration: 3500 });
}

/**
 * Notificação simples de erro
 */
export function notifyError(message: string, description?: string) {
  if (isDndActive) return;
  sonnerToast.error(message, { description, duration: 5000 });
}

/**
 * Notificação simples informativa
 */
export function notifyInfo(message: string, description?: string) {
  if (isDndActive) return;
  sonnerToast.info(message, { description, duration: 3500 });
}

/**
 * Notificação de aviso
 */
export function notifyWarning(message: string, description?: string) {
  if (isDndActive) return;
  sonnerToast.warning(message, { description, duration: 4000 });
}

// Histórico recente para deduplicação rápida de toasts em milissegundos
const recentToasts = new Set<string>();

/**
 * Processa um ActionLog recebido via WebSocket ou gerado localmente
 * e exibe um toast contextual apropriado se relevante para o operador.
 */
export function handleActionLogNotification(log: ActionLog) {
  if (isDndActive) {
    return;
  }

  // Telas públicas e de palco nunca devem exibir notificações toast sobrepostas
  if (typeof window !== 'undefined') {
    const p = window.location.pathname;
    if (
      p.includes('/public') ||
      p.includes('/stage') ||
      p.includes('/lower-third') ||
      p.includes('/lower') ||
      p.includes('/obs')
    ) {
      return;
    }
  }

  const isSelf = !!log.instance_id && log.instance_id === getInstanceId();
  const originLabel = log.device_info || (isSelf ? 'Este computador' : 'Dispositivo remoto');

  // Chave de deduplicação (ação + detalhes) para evitar disparos duplicados em menos de 1.5s
  const dedupKey = `${log.action}:${log.details}`;
  if (recentToasts.has(dedupKey)) {
    return;
  }
  recentToasts.add(dedupKey);
  setTimeout(() => recentToasts.delete(dedupKey), 1500);

  switch (log.action) {
    case 'client_connected': {
      // Dispara toast para alertar sobre novos aparelhos na rede
      const label = log.device_info || 'Novo aparelho';
      const ip = log.ip_address ? ` (${log.ip_address})` : '';
      if (!isSelf) {
        toast.info(`Novo dispositivo conectado`, {
          description: `${label}${ip} conectou-se ao OpenPresenter.`,
          duration: 4000,
        });
      }
      break;
    }

    case 'client_disconnected': {
      if (!isSelf) {
        toast.info(`Dispositivo desconectado`, {
          description: log.details,
          duration: 3500,
        });
      }
      break;
    }

    case 'presentation_start':
    case 'quick_present_start': {
      if (!isSelf) {
        toast.info(`Apresentação iniciada remotamente`, {
          description: `${log.details} por ${originLabel}`,
          duration: 3500,
        });
      } else {
        toast.success(`Apresentação iniciada`, {
          description: log.details,
          duration: 2500,
        });
      }
      break;
    }

    case 'presentation_stop': {
      if (!isSelf) {
        toast.warning(`Apresentação parada remotamente`, {
          description: `Apresentação finalizada por ${originLabel}.`,
          duration: 3500,
        });
      } else {
        toast.info(`Apresentação finalizada`, {
          duration: 2500,
        });
      }
      break;
    }

    case 'song_saved': {
      toast.success(`Música salva no catálogo`, {
        description: log.details,
        duration: 3500,
      });
      break;
    }

    case 'setlist_item_added': {
      if (!isSelf) {
        toast.info(`Item adicionado à setlist`, {
          description: `${log.details} por ${originLabel}`,
          duration: 3000,
        });
      }
      break;
    }

    case 'setlist_cleared': {
      if (!isSelf) {
        toast.info(`Setlist limpa remotamente`, {
          description: `Todos os itens foram removidos por ${originLabel}`,
          duration: 3500,
        });
      }
      break;
    }

    case 'setlist_activated': {
      if (!isSelf) {
        toast.info(`Setlist ativada`, {
          description: `${log.details} por ${originLabel}`,
          duration: 3000,
        });
      }
      break;
    }

    case 'display_mode_change': {
      if (!isSelf) {
        toast.info(`Modo de exibição alterado`, {
          description: `${log.details} (${originLabel})`,
          duration: 3000,
        });
      }
      break;
    }

    case 'stage_message_change': {
      toast.info(`Aviso para o palco`, {
        description: log.details,
        duration: 3500,
      });
      break;
    }

    default:
      // Outras ações não precisam de toast intrusivo
      break;
  }
}

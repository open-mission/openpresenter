import { writable } from 'svelte/store';

export interface CastState {
  isSdkLoaded: boolean;
  isAvailable: boolean;
  isConnected: boolean;
  isConnecting: boolean;
  deviceName: string | null;
  activeTarget: 'public' | 'stage' | null;
  error: string | null;
}

const INITIAL_CAST_STATE: CastState = {
  isSdkLoaded: false,
  isAvailable: false,
  isConnected: false,
  isConnecting: false,
  deviceName: null,
  activeTarget: null,
  error: null,
};

export const castStore = writable<CastState>(INITIAL_CAST_STATE);

// App ID do Receiver Comunitário mantido para projeção direta de URLs no Chromecast
export const DEFAULT_URL_CAST_APP_ID = '5CB45E5A';
export const CAST_NAMESPACE_URL = 'urn:x-cast:com.url.cast';

let sdkLoadPromise: Promise<boolean> | null = null;
let currentAppId = DEFAULT_URL_CAST_APP_ID;

/**
 * Carrega a SDK do Google Cast Sender sob demanda de forma segura e não-bloqueante
 */
export function loadCastSdk(): Promise<boolean> {
  if (typeof window === 'undefined') return Promise.resolve(false);

  // Já carregado e inicializado
  if ((window as any).cast?.framework) {
    castStore.update((s) => ({ ...s, isSdkLoaded: true, isAvailable: true }));
    return Promise.resolve(true);
  }

  if (sdkLoadPromise) return sdkLoadPromise;

  sdkLoadPromise = new Promise<boolean>((resolve) => {
    // Callback obrigatório disparado pelo cast_sender.js
    (window as any)['__onGCastApiAvailable'] = (isAvailable: boolean) => {
      if (isAvailable && (window as any).cast?.framework) {
        initCastContext();
        castStore.update((s) => ({
          ...s,
          isSdkLoaded: true,
          isAvailable: true,
          error: null,
        }));
        resolve(true);
      } else {
        castStore.update((s) => ({
          ...s,
          isSdkLoaded: true,
          isAvailable: false,
          error: 'Nenhum dispositivo Google Cast detectado ou API indisponível.',
        }));
        resolve(false);
      }
    };

    // Injeta o script se não existir no DOM
    const scriptId = 'google-cast-sender-sdk';
    if (!document.getElementById(scriptId)) {
      const script = document.createElement('script');
      script.id = scriptId;
      script.src = 'https://www.gstatic.com/cv/js/sender/v1/cast_sender.js?loadCastFramework=1';
      script.async = true;
      script.onerror = () => {
        console.warn('[Cast] Falha ao carregar script do Google Cast SDK (modo offline ou sem conexão externa)');
        castStore.update((s) => ({
          ...s,
          isSdkLoaded: false,
          isAvailable: false,
          error: 'Não foi possível carregar a biblioteca Google Cast (verifique a conexão com a internet).',
        }));
        resolve(false);
      };
      document.head.appendChild(script);
    } else {
      // Script já inserido mas aguardando
      setTimeout(() => {
        if ((window as any).cast?.framework) {
          initCastContext();
          resolve(true);
        } else {
          resolve(false);
        }
      }, 1500);
    }
  });

  return sdkLoadPromise;
}

/**
 * Inicializa o CastContext com o App ID configurado e escuta mudanças de sessão
 */
function initCastContext() {
  try {
    const cast = (window as any).cast;
    const chrome = (window as any).chrome;
    if (!cast?.framework || !chrome?.cast) return;

    const context = cast.framework.CastContext.getInstance();
    context.setOptions({
      receiverApplicationId: currentAppId,
      autoJoinPolicy: chrome.cast.AutoJoinPolicy.ORIGIN_SCOPED,
    });

    context.addEventListener(
      cast.framework.CastContextEventType.SESSION_STATE_CHANGED,
      (event: any) => {
        const sessionState = event.sessionState;
        const session = context.getCurrentSession();

        if (sessionState === cast.framework.SessionState.SESSION_STARTED || sessionState === cast.framework.SessionState.SESSION_RESUMED) {
          const deviceName = session?.getCastDevice()?.friendlyName || 'Chromecast';
          castStore.update((s) => ({
            ...s,
            isConnected: true,
            isConnecting: false,
            deviceName,
            error: null,
          }));
        } else if (sessionState === cast.framework.SessionState.SESSION_STARTING) {
          castStore.update((s) => ({
            ...s,
            isConnecting: true,
            error: null,
          }));
        } else if (sessionState === cast.framework.SessionState.SESSION_ENDED || sessionState === cast.framework.SessionState.NO_SESSION) {
          castStore.update((s) => ({
            ...s,
            isConnected: false,
            isConnecting: false,
            deviceName: null,
            activeTarget: null,
          }));
        } else if (sessionState === cast.framework.SessionState.SESSION_START_FAILED) {
          castStore.update((s) => ({
            ...s,
            isConnected: false,
            isConnecting: false,
            error: 'Não foi possível conectar ao dispositivo selecionado.',
          }));
        }
      }
    );

    // Verifica se já havia uma sessão ativa
    const currentSession = context.getCurrentSession();
    if (currentSession) {
      castStore.update((s) => ({
        ...s,
        isConnected: true,
        deviceName: currentSession.getCastDevice()?.friendlyName || 'Chromecast',
      }));
    }
  } catch (err) {
    console.error('[Cast] Erro ao inicializar CastContext:', err);
  }
}

/**
 * Obtém a URL da tela na rede local (usando o IP do servidor em vez de localhost)
 */
async function resolveOutputUrl(target: 'public' | 'stage'): Promise<string> {
  try {
    const res = await fetch('/api/system/network-info');
    if (res.ok) {
      const data = await res.json();
      return target === 'stage' ? data.stage_url : data.public_url;
    }
  } catch {
    // fallback se o endpoint falhar
  }

  const hostname = window.location.hostname;
  const port = window.location.port ? `:${window.location.port}` : '';
  const protocol = window.location.protocol;
  return `${protocol}//${hostname}${port}/${target}`;
}

/**
 * Dispara a transmissão da tela para o Chromecast selecionado pelo usuário
 */
export async function startCasting(target: 'public' | 'stage'): Promise<boolean> {
  castStore.update((s) => ({ ...s, isConnecting: true, error: null }));

  const loaded = await loadCastSdk();
  if (!loaded) {
    castStore.update((s) => ({
      ...s,
      isConnecting: false,
      error: 'Google Cast SDK indisponível no momento. Use o menu Transmitir do navegador Chrome.',
    }));
    return false;
  }

  const cast = (window as any).cast;
  if (!cast?.framework) {
    castStore.update((s) => ({
      ...s,
      isConnecting: false,
      error: 'API do Google Cast não suportada neste navegador.',
    }));
    return false;
  }

  const context = cast.framework.CastContext.getInstance();
  const targetUrl = await resolveOutputUrl(target);

  try {
    // Solicita ao navegador a abertura do seletor nativo de dispositivos Cast
    const errorCode = await context.requestSession();
    if (errorCode) {
      console.warn('[Cast] requestSession retornou erro:', errorCode);
      castStore.update((s) => ({
        ...s,
        isConnecting: false,
        error: `Erro ao iniciar sessão Cast: ${errorCode}`,
      }));
      return false;
    }

    const session = context.getCurrentSession();
    if (!session) {
      castStore.update((s) => ({
        ...s,
        isConnecting: false,
        error: 'Sessão Cast não iniciada.',
      }));
      return false;
    }

    // Envia o comando de carregamento da URL para o Web Receiver
    const payload = {
      type: 'loc',
      url: targetUrl,
    };

    await session.sendMessage(CAST_NAMESPACE_URL, payload);

    castStore.update((s) => ({
      ...s,
      isConnected: true,
      isConnecting: false,
      deviceName: session.getCastDevice()?.friendlyName || 'Chromecast',
      activeTarget: target,
      error: null,
    }));

    return true;
  } catch (err: any) {
    console.error('[Cast] Falha ao iniciar transmissão:', err);
    castStore.update((s) => ({
      ...s,
      isConnecting: false,
      error: err?.message || 'Falha ao conectar com o Chromecast.',
    }));
    return false;
  }
}

/**
 * Interrompe a transmissão e desconecta o Chromecast
 */
export function stopCasting() {
  try {
    const cast = (window as any).cast;
    if (cast?.framework) {
      const context = cast.framework.CastContext.getInstance();
      context.endCurrentSession(true);
    }
  } catch (err) {
    console.error('[Cast] Erro ao encerrar sessão Cast:', err);
  } finally {
    castStore.update((s) => ({
      ...s,
      isConnected: false,
      isConnecting: false,
      deviceName: null,
      activeTarget: null,
      error: null,
    }));
  }
}

/**
 * Permite configurar um App ID customizado
 */
export function setCustomReceiverAppId(appId: string) {
  currentAppId = appId.trim() || DEFAULT_URL_CAST_APP_ID;
  if (typeof window !== 'undefined' && (window as any).cast?.framework) {
    initCastContext();
  }
}

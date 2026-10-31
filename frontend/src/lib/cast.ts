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
let cachedServerOrigin: string | null = null;

/**
 * Obtém a origem HTTP do servidor (ex: "http://192.168.1.100:18080")
 * priorizando parâmetros de URL (?server= ou ?origin=), IP em cache de network-info
 * ou a origem do navegador atual (caso não seja sandboxed chrome-extension / gstatic).
 */
export function getServerOrigin(): string {
  if (cachedServerOrigin) return cachedServerOrigin;

  if (typeof window === 'undefined') return '';

  // 1. Prioriza parâmetro explícito na URL
  const params = new URLSearchParams(window.location.search);
  const serverParam = params.get('server') || params.get('origin');
  if (serverParam && (serverParam.startsWith('http://') || serverParam.startsWith('https://'))) {
    return serverParam.replace(/\/+$/, '');
  }

  // 2. Se a origem atual for HTTP/HTTPS válida (e não for chrome-extension ou sandbox externa do Cast)
  const origin = window.location.origin;
  if (
    origin &&
    (origin.startsWith('http://') || origin.startsWith('https://')) &&
    !origin.startsWith('chrome-extension://') &&
    !origin.includes('gstatic.com')
  ) {
    return origin.replace(/\/+$/, '');
  }

  // 3. Fallback com hostname e porta
  const hostname = window.location.hostname || '127.0.0.1';
  const port = window.location.port ? `:${window.location.port}` : '';
  const protocol = window.location.protocol.startsWith('http') ? window.location.protocol : 'http:';
  return `${protocol}//${hostname}${port}`.replace(/\/+$/, '');
}

/**
 * Consulta a API de rede para obter o IP de LAN do servidor e armazenar em cache
 */
export async function fetchServerOrigin(): Promise<string> {
  if (cachedServerOrigin) return cachedServerOrigin;
  try {
    const res = await fetch('/api/system/network-info');
    if (res.ok) {
      const data = await res.json();
      if (data.local_ip && data.port && data.local_ip !== '127.0.0.1' && data.local_ip !== '0.0.0.0') {
        cachedServerOrigin = `http://${data.local_ip}:${data.port}`;
        return cachedServerOrigin;
      }
    }
  } catch {}
  return getServerOrigin();
}

/**
 * Normaliza qualquer URL de mídia ou background para endereço HTTP absoluto.
 * Concatena a origem real do servidor local onde o backend está rodando,
 * prevenindo que o Web Receiver do Chromecast (que roda em sandbox ou chrome-extension://)
 * tente resolver caminhos relativos como /media/... ou ./backgrounds/... em hosts incorretos.
 */
export function resolveMediaUrl(url: string | null | undefined): string {
  if (!url) return '';
  const trimmed = url.trim();
  if (!trimmed) return '';

  // Se já for absoluta ou inline data/blob, retorna inalterada
  if (
    trimmed.startsWith('http://') ||
    trimmed.startsWith('https://') ||
    trimmed.startsWith('data:') ||
    trimmed.startsWith('blob:')
  ) {
    return trimmed;
  }

  // Concatena a origem real do servidor local onde o backend está rodando
  if (typeof window !== 'undefined') {
    const params = new URLSearchParams(window.location.search);
    const serverParam = params.get('server') || params.get('origin');
    const base = serverParam && (serverParam.startsWith('http://') || serverParam.startsWith('https://'))
      ? serverParam.replace(/\/+$/, '')
      : window.location.origin;

    const cleanPath = trimmed.startsWith('./') ? trimmed.slice(2) : trimmed;
    return `${base}${cleanPath.startsWith('/') ? '' : '/'}${cleanPath}`;
  }

  return trimmed;
}

export const normalizeMediaUrl = resolveMediaUrl;

/**
 * Carrega a SDK do Google Cast Sender sob demanda de forma segura e não-bloqueante
 */
export function loadCastSdk(): Promise<boolean> {
  if (typeof window === 'undefined') return Promise.resolve(false);

  // Já carregado e inicializado
  if ((window as any).cast?.framework) {
    initCastContext();
    castStore.update((s) => ({ ...s, isSdkLoaded: true, isAvailable: true, error: null }));
    return Promise.resolve(true);
  }

  if (sdkLoadPromise) return sdkLoadPromise;

  const promise = new Promise<boolean>((resolve) => {
    let checkInterval: any = null;
    let timeoutId: any = null;

    const cleanup = () => {
      if (checkInterval) clearInterval(checkInterval);
      if (timeoutId) clearTimeout(timeoutId);
    };

    // Callback obrigatório disparado pelo cast_sender.js
    (window as any)['__onGCastApiAvailable'] = (isAvailable: boolean) => {
      cleanup();
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

    // Sonda periódica a cada 250ms caso o callback global tenha disparado antes ou haja atraso
    checkInterval = setInterval(() => {
      if ((window as any).cast?.framework) {
        cleanup();
        initCastContext();
        castStore.update((s) => ({
          ...s,
          isSdkLoaded: true,
          isAvailable: true,
          error: null,
        }));
        resolve(true);
      }
    }, 250);

    // Timeout de 7 segundos para aguardar a biblioteca
    timeoutId = setTimeout(() => {
      cleanup();
      if ((window as any).cast?.framework) {
        initCastContext();
        resolve(true);
      } else {
        resolve(false);
      }
    }, 7000);

    // Injeta o script se não existir no DOM
    const scriptId = 'google-cast-sender-sdk';
    if (!document.getElementById(scriptId)) {
      const script = document.createElement('script');
      script.id = scriptId;
      script.src = 'https://www.gstatic.com/cv/js/sender/v1/cast_sender.js?loadCastFramework=1';
      script.async = true;
      script.onerror = () => {
        cleanup();
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
    }
  });

  sdkLoadPromise = promise.then((ok) => {
    if (!ok) {
      // Se não carregou, limpa a promise para que o próximo clique do usuário possa tentar novamente
      sdkLoadPromise = null;
    }
    return ok;
  }).catch(() => {
    sdkLoadPromise = null;
    return false;
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
export async function resolveOutputUrl(target: 'public' | 'stage'): Promise<string> {
  const origin = await fetchServerOrigin();
  return `${origin}/${target}?server=${encodeURIComponent(origin)}&view=${target}`;
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

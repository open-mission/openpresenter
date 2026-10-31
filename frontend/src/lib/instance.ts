/**
 * Utilitários para rastreamento de instâncias e identificação amigável de dispositivos.
 */

export function getInstanceId(): string {
  if (typeof window === 'undefined' || !window.localStorage) {
    return 'inst-headless';
  }

  let id = localStorage.getItem('op_instance_id');
  if (!id) {
    id = `inst-${Math.random().toString(36).slice(2, 9)}-${Date.now().toString(36)}`;
    try {
      localStorage.setItem('op_instance_id', id);
    } catch {
      // Ignora se localStorage estiver bloqueado
    }
  }
  return id;
}

export function getDeviceInfo(): string {
  if (typeof window === 'undefined' || !navigator?.userAgent) {
    return 'Desktop';
  }

  const ua = navigator.userAgent;

  // Detecção de Plataforma/SO
  let os = 'Dispositivo';
  if (/iPhone/i.test(ua)) {
    os = 'iPhone';
  } else if (/iPad/i.test(ua)) {
    os = 'iPad';
  } else if (/Android/i.test(ua)) {
    os = 'Android';
  } else if (/Mac OS X|Macintosh/i.test(ua)) {
    os = 'macOS';
  } else if (/Windows/i.test(ua)) {
    os = 'Windows';
  } else if (/Linux/i.test(ua)) {
    os = 'Linux';
  }

  // Detecção de Navegador
  let browser = 'Navegador';
  if (/CriOS/i.test(ua)) {
    browser = 'Chrome iOS';
  } else if (/FxiOS/i.test(ua)) {
    browser = 'Firefox iOS';
  } else if (/Edg/i.test(ua)) {
    browser = 'Edge';
  } else if (/Chrome|Chromium/i.test(ua)) {
    browser = 'Chrome';
  } else if (/Safari/i.test(ua)) {
    browser = 'Safari';
  } else if (/Firefox/i.test(ua)) {
    browser = 'Firefox';
  }

  return `${browser} no ${os}`;
}

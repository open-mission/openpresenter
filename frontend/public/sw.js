// Service Worker para OpenPresenter PWA
const CACHE_NAME = 'open-presenter-cache-v1';

self.addEventListener('install', (event) => {
  // Ativação imediata da nova versão
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames
          .filter((name) => name !== CACHE_NAME)
          .map((name) => caches.delete(name))
      );
    }).then(() => self.clients.claim())
  );
});

// Estratégia Network-First para garantir que comandos em tempo real e websockets fluam direto
self.addEventListener('fetch', (event) => {
  // Ignora requisições de API, WebSocket e métodos não-GET
  if (
    event.request.method !== 'GET' ||
    event.request.url.includes('/api/') ||
    event.request.url.includes('/ws')
  ) {
    return;
  }

  event.respondWith(
    fetch(event.request).catch(() => {
      return caches.match(event.request);
    })
  );
});

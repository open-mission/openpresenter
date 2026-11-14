<script lang="ts">
  import { onMount } from 'svelte';
  import AdminView from '$lib/views/AdminView.svelte';
  import PublicView from '$lib/views/PublicView.svelte';
  import StageView from '$lib/views/StageView.svelte';
  import RemoteView from '$lib/views/RemoteView.svelte';
  import LowerThirdView from '$lib/views/LowerThirdView.svelte';
  import Sonner from '$lib/components/ui/Sonner.svelte';
  import { initAppTheme, themeStore, toastPositionStore } from '$lib/settings';

  let currentPath = $state(window.location.pathname);

  onMount(() => {
    // Redirecionamento inteligente: se aberto em modo standalone (PWA adicionado à tela de início no celular)
    // na raiz '/', direciona automaticamente para o controle remoto '/remote'
    const isStandalone =
      window.matchMedia('(display-mode: standalone)').matches ||
      (window.navigator as any).standalone === true;
    const isMobileScreen = window.innerWidth <= 768;

    if (isStandalone && isMobileScreen && window.location.pathname === '/') {
      window.history.replaceState({}, '', '/remote');
      currentPath = '/remote';
    }

    if (
      !window.location.pathname.startsWith('/public') &&
      !window.location.pathname.startsWith('/stage') &&
      !window.location.pathname.startsWith('/lower') &&
      !window.location.pathname.startsWith('/obs')
    ) {
      initAppTheme();
    }

    const handlePopState = () => {
      currentPath = window.location.pathname;
    };

    const handleStorage = (event: StorageEvent) => {
      if (
        event.key === 'openpresenter:theme' ||
        event.key === 'openpresenter:base-color' ||
        event.key === 'openpresenter:accent-theme'
      ) {
        initAppTheme();
      }
    };

    window.addEventListener('popstate', handlePopState);
    window.addEventListener('storage', handleStorage);
    return () => {
      window.removeEventListener('popstate', handlePopState);
      window.removeEventListener('storage', handleStorage);
    };
  });

  const viewMode = $derived.by<'admin' | 'public' | 'stage' | 'remote' | 'lower-third'>(() => {
    // Suporte prioritário a query params (ex: ?view=public para Web Receivers do Cast/iframes)
    if (typeof window !== 'undefined') {
      const searchParams = new URLSearchParams(window.location.search);
      const queryView = searchParams.get('view');
      if (queryView === 'public') return 'public';
      if (queryView === 'stage') return 'stage';
      if (queryView === 'remote') return 'remote';
      if (queryView === 'lower-third' || queryView === 'lower' || queryView === 'obs') {
        return 'lower-third';
      }
    }

    if (currentPath.startsWith('/public')) {
      return 'public';
    }
    if (currentPath.startsWith('/stage')) {
      return 'stage';
    }
    if (currentPath.startsWith('/remote')) {
      return 'remote';
    }
    if (
      currentPath.startsWith('/lower-third') ||
      currentPath.startsWith('/lower') ||
      currentPath.startsWith('/obs')
    ) {
      return 'lower-third';
    }
    return 'admin';
  });

  // Atualiza manifesto PWA dinâmico e classes de escopo global
  $effect(() => {
    if (typeof document === 'undefined') return;

    const manifestLink = document.querySelector<HTMLLinkElement>('link[rel="manifest"]');
    const appleTitleMeta = document.querySelector<HTMLMetaElement>('meta[name="apple-mobile-web-app-title"]');

    if (viewMode === 'remote') {
      if (manifestLink) manifestLink.href = '/manifest-remote.webmanifest';
      if (appleTitleMeta) appleTitleMeta.content = 'Remoto';
      document.title = 'OpenPresenter Remoto';
    } else {
      if (manifestLink) manifestLink.href = '/manifest.webmanifest';
      if (appleTitleMeta) appleTitleMeta.content = 'OpenPresenter';
      if (viewMode === 'lower-third') {
        document.title = 'OpenPresenter — Lower Third (OBS)';
      } else if (viewMode === 'public') {
        document.title = 'OpenPresenter — Projeção Pública';
      } else if (viewMode === 'stage') {
        document.title = 'OpenPresenter — Monitor de Palco';
      } else {
        document.title = 'OpenPresenter — Projeção para igrejas e eventos';
      }
    }

    // Classes no html e body para transparência na rota Lower Third
    if (viewMode === 'lower-third') {
      document.documentElement.classList.add('is-lower-third');
      document.body.classList.add('is-lower-third');
    } else {
      document.documentElement.classList.remove('is-lower-third');
      document.body.classList.remove('is-lower-third');
    }

    const themeColorMeta = document.querySelector<HTMLMetaElement>('meta[name="theme-color"]');
    if (themeColorMeta && (viewMode === 'remote' || viewMode === 'admin')) {
      const currentTheme = $themeStore;
      const prefersDark =
        typeof window !== 'undefined' &&
        window.matchMedia('(prefers-color-scheme: dark)').matches;
      const isDark =
        currentTheme === 'dark' || (currentTheme === 'system' && prefersDark);
      themeColorMeta.content = isDark ? '#0a0a0a' : '#fafafa';
    }
  });
</script>

{#if viewMode === 'public'}
  <PublicView />
{:else if viewMode === 'stage'}
  <StageView />
{:else if viewMode === 'remote'}
  <RemoteView />
  <Sonner position="top-center" richColors closeButton />
{:else if viewMode === 'lower-third'}
  <LowerThirdView />
{:else}
  <AdminView />
  <Sonner position={$toastPositionStore} richColors closeButton />
{/if}

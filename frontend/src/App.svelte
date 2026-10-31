<script lang="ts">
  import { onMount } from 'svelte';
  import AdminView from '$lib/views/AdminView.svelte';
  import PublicView from '$lib/views/PublicView.svelte';
  import StageView from '$lib/views/StageView.svelte';
  import RemoteView from '$lib/views/RemoteView.svelte';
  import LowerThirdView from '$lib/views/LowerThirdView.svelte';
  import Sonner from '$lib/components/ui/Sonner.svelte';

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

    const handlePopState = () => {
      currentPath = window.location.pathname;
    };

    window.addEventListener('popstate', handlePopState);
    return () => {
      window.removeEventListener('popstate', handlePopState);
    };
  });

  const viewMode = $derived.by<'admin' | 'public' | 'stage' | 'remote' | 'lower-third'>(() => {
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
  <Sonner position="top-right" richColors closeButton />
{/if}

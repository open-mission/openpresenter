<script lang="ts">
  import { onMount } from 'svelte';
  import AdminView from '$lib/views/AdminView.svelte';
  import PublicView from '$lib/views/PublicView.svelte';
  import StageView from '$lib/views/StageView.svelte';
  import RemoteView from '$lib/views/RemoteView.svelte';
  import LowerThirdView from '$lib/views/LowerThirdView.svelte';

  let currentPath = $state(window.location.pathname);

  onMount(() => {
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
</script>

{#if viewMode === 'public'}
  <PublicView />
{:else if viewMode === 'stage'}
  <StageView />
{:else if viewMode === 'remote'}
  <RemoteView />
{:else if viewMode === 'lower-third'}
  <LowerThirdView />
{:else}
  <AdminView />
{/if}

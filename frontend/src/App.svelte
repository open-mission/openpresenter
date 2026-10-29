<script lang="ts">
  import { onMount } from 'svelte';
  import AdminView from '$lib/views/AdminView.svelte';
  import PublicView from '$lib/views/PublicView.svelte';
  import StageView from '$lib/views/StageView.svelte';

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

  const viewMode = $derived.by<'admin' | 'public' | 'stage'>(() => {
    if (currentPath.startsWith('/public')) {
      return 'public';
    }
    if (currentPath.startsWith('/stage')) {
      return 'stage';
    }
    return 'admin';
  });
</script>

{#if viewMode === 'public'}
  <PublicView />
{:else if viewMode === 'stage'}
  <StageView />
{:else}
  <AdminView />
{/if}

<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'default' | 'secondary' | 'outline' | 'destructive' | 'live' | 'success';
    class?: string;
    children?: Snippet;
  }

  let {
    variant = 'default',
    class: className = '',
    children,
  }: Props = $props();

  const variantClasses = {
    default: 'bg-zinc-800 text-zinc-200 border-zinc-700',
    secondary: 'bg-zinc-900 text-zinc-400 border-zinc-800',
    outline: 'text-zinc-300 border-zinc-700 bg-transparent',
    destructive: 'bg-red-950/80 text-red-300 border-red-800/80',
    live: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/40 font-semibold',
    success: 'bg-emerald-950/80 text-emerald-300 border-emerald-800/80',
  };
</script>

<span
  class={cn(
    'inline-flex items-center gap-1 px-2 py-0.5 text-xs font-medium rounded-full border transition-colors select-none',
    variantClasses[variant],
    className
  )}
>
  {#if children}
    {@render children()}
  {/if}
</span>

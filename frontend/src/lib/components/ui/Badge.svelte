<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'default' | 'secondary' | 'outline' | 'destructive' | 'live' | 'success';
    class?: string;
    title?: string;
    children?: Snippet;
  }

  let {
    variant = 'default',
    class: className = '',
    title,
    children,
  }: Props = $props();

  const variantClasses = {
    default: 'bg-muted text-foreground border-border',
    secondary: 'bg-secondary text-secondary-foreground border-border',
    outline: 'text-foreground border-border bg-transparent',
    destructive: 'bg-red-500/15 text-red-700 dark:text-red-300 border-red-600/35 dark:border-red-500/40',
    live: 'bg-primary/15 text-primary border-primary/35 font-semibold',
    success: 'bg-emerald-500/15 text-emerald-800 dark:text-emerald-300 border-emerald-600/35 dark:border-emerald-500/40',
  };
</script>

<span
  {title}
  class={cn(
    'inline-flex items-center gap-1 px-2 py-0.5 text-xs font-medium rounded-full border transition-colors select-none whitespace-nowrap shrink-0',
    variantClasses[variant],
    className
  )}
>
  {#if children}
    {@render children()}
  {/if}
</span>

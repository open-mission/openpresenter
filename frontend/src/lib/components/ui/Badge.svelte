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
    default: 'bg-muted text-foreground border-border',
    secondary: 'bg-secondary text-secondary-foreground border-border',
    outline: 'text-foreground border-border bg-transparent',
    destructive: 'bg-destructive/20 text-destructive border-destructive/40',
    live: 'bg-primary/20 text-primary border-primary/40 font-semibold',
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

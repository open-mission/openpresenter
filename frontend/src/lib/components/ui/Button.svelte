<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'default' | 'secondary' | 'outline' | 'destructive' | 'ghost' | 'live' | 'subtle';
    size?: 'sm' | 'md' | 'lg' | 'icon';
    class?: string;
    disabled?: boolean;
    type?: 'button' | 'submit' | 'reset';
    title?: string;
    onclick?: (e: MouseEvent) => void;
    children?: Snippet;
  }

  let {
    variant = 'default',
    size = 'md',
    class: className = '',
    disabled = false,
    type = 'button',
    title,
    onclick,
    children,
  }: Props = $props();

  const variantClasses = {
    default: 'bg-primary text-primary-foreground hover:opacity-90 shadow-sm font-medium',
    secondary: 'bg-secondary text-secondary-foreground hover:bg-muted border border-border font-medium',
    outline: 'border border-border text-foreground hover:bg-muted hover:text-foreground',
    destructive: 'bg-destructive text-destructive-foreground hover:opacity-90 shadow-sm font-medium',
    ghost: 'text-muted-foreground hover:bg-muted hover:text-foreground',
    live: 'bg-primary text-primary-foreground font-semibold ring-2 ring-ring/60 shadow-md animate-pulse',
    subtle: 'bg-card text-foreground hover:bg-muted border border-border',
  };

  const sizeClasses = {
    sm: 'h-8 px-2.5 text-xs rounded-md',
    md: 'h-9 px-3.5 text-sm rounded-md',
    lg: 'h-11 px-6 text-base rounded-lg',
    icon: 'h-9 w-9 p-0 flex items-center justify-center rounded-md',
  };
</script>

<button
  {type}
  {title}
  {disabled}
  {onclick}
  class={cn(
    'inline-flex items-center justify-center gap-2 font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 cursor-pointer select-none',
    variantClasses[variant],
    sizeClasses[size],
    className
  )}
>
  {#if children}
    {@render children()}
  {/if}
</button>

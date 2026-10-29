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
    default: 'bg-emerald-600 text-white hover:bg-emerald-500 shadow-sm font-medium',
    secondary: 'bg-zinc-800 text-zinc-100 hover:bg-zinc-700 border border-zinc-700/60 font-medium',
    outline: 'border border-zinc-700 text-zinc-200 hover:bg-zinc-800 hover:text-white',
    destructive: 'bg-red-600 text-white hover:bg-red-500 shadow-sm font-medium',
    ghost: 'text-zinc-300 hover:bg-zinc-800/80 hover:text-white',
    live: 'bg-emerald-500 text-black font-semibold ring-2 ring-emerald-400 shadow-md animate-pulse',
    subtle: 'bg-zinc-900 text-zinc-300 hover:bg-zinc-800 hover:text-white border border-zinc-800',
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
    'inline-flex items-center justify-center gap-2 font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-500 disabled:pointer-events-none disabled:opacity-50 cursor-pointer select-none',
    variantClasses[variant],
    sizeClasses[size],
    className
  )}
>
  {#if children}
    {@render children()}
  {/if}
</button>

<script lang="ts">
  import { Toaster as Sonner, type ToasterProps } from 'svelte-sonner';
  import { themeStore } from '$lib/settings';

  let props: ToasterProps = $props();
</script>

<Sonner
  theme={$themeStore === 'light' ? 'light' : 'dark'}
  class="toaster group"
  toastOptions={{
    classes: {
      toast:
        'group toast group-[.toaster]:bg-neutral-900 group-[.toaster]:text-neutral-100 group-[.toaster]:border-neutral-800 group-[.toaster]:shadow-2xl group-[.toaster]:rounded-xl font-sans',
      description: 'group-[.toast]:text-neutral-400 text-xs',
      actionButton:
        'group-[.toast]:bg-primary group-[.toast]:text-primary-foreground text-xs font-semibold',
      cancelButton:
        'group-[.toast]:bg-neutral-800 group-[.toast]:text-neutral-300 text-xs',
      error: '!border-rose-800/60 !bg-rose-950/90 !text-rose-100',
      success: '!border-emerald-800/60 !bg-emerald-950/90 !text-emerald-100',
      warning: '!border-amber-800/60 !bg-amber-950/90 !text-amber-100',
      info: '!border-sky-800/60 !bg-sky-950/90 !text-sky-100',
    },
  }}
  {...props}
/>

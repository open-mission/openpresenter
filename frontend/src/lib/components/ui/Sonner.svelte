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
        'group toast group-[.toaster]:bg-card group-[.toaster]:text-card-foreground group-[.toaster]:border-border group-[.toaster]:shadow-2xl group-[.toaster]:rounded-xl font-sans',
      description: 'group-[.toast]:text-muted-foreground text-xs',
      actionButton:
        'group-[.toast]:bg-primary group-[.toast]:text-primary-foreground text-xs font-semibold',
      cancelButton:
        'group-[.toast]:bg-secondary group-[.toast]:text-secondary-foreground text-xs',
      error: '!border-red-600/35 dark:!border-rose-800/60 !bg-red-50 dark:!bg-rose-950/90 !text-red-900 dark:!text-rose-100',
      success: '!border-emerald-600/35 dark:!border-emerald-800/60 !bg-emerald-50 dark:!bg-emerald-950/90 !text-emerald-900 dark:!text-emerald-100',
      warning: '!border-amber-600/35 dark:!border-amber-800/60 !bg-amber-50 dark:!bg-amber-950/90 !text-amber-900 dark:!text-amber-100',
      info: '!border-sky-600/35 dark:!border-sky-800/60 !bg-sky-50 dark:!bg-sky-950/90 !text-sky-900 dark:!text-sky-100',
    },
  }}
  {...props}
/>

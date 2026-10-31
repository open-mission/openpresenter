<script lang="ts">
  import { PaneGroup, type PaneGroupProps } from 'paneforge';
  import { cn } from '$lib/utils';

  let { class: className, children, ...restProps }: PaneGroupProps = $props();
</script>

<PaneGroup class={cn('flex h-full w-full data-[direction=vertical]:flex-col', className)} {...restProps}>
  {@render children?.()}
</PaneGroup>

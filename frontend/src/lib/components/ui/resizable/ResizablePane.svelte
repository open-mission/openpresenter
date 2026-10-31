<script lang="ts">
  import { Pane, type PaneProps } from 'paneforge';
  import { cn } from '$lib/utils';

  let { class: className, children, ...restProps }: PaneProps = $props();
</script>

<Pane class={cn('', className)} {...restProps}>
  {@render children?.()}
</Pane>

export interface AutoScaleOptions {
  minSize?: number;
  maxSize?: number;
}

export function autoScale(node: HTMLElement, options: AutoScaleOptions = {}) {
  let currentOptions = { ...options };
  let animationFrameId: number | null = null;
  let isAdjusting = false;

  function adjust() {
    if (!node || !node.parentElement) return;
    const parent = node.parentElement;

    const computedParent = window.getComputedStyle(parent);
    const padX = (parseFloat(computedParent.paddingLeft) || 0) + (parseFloat(computedParent.paddingRight) || 0);
    const padY = (parseFloat(computedParent.paddingTop) || 0) + (parseFloat(computedParent.paddingBottom) || 0);

    const availableWidth = parent.clientWidth - padX;
    const availableHeight = parent.clientHeight - padY;

    if (availableWidth <= 4 || availableHeight <= 4) return;

    const minSize = Math.max(8, currentOptions.minSize ?? 20);
    const maxSize = Math.max(minSize, currentOptions.maxSize ?? 240);

    isAdjusting = true;

    let low = minSize;
    let high = maxSize;
    let best = minSize;

    // Busca binária para encontrar o maior font-size que cabe sem overflow
    while (low <= high) {
      const mid = Math.floor((low + high) / 2);
      node.style.fontSize = `${mid}px`;

      const overflowX = node.scrollWidth > availableWidth + 1;
      const overflowY = node.scrollHeight > availableHeight + 1;

      if (!overflowX && !overflowY) {
        best = mid;
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }

    node.style.fontSize = `${best}px`;
    isAdjusting = false;
  }

  function scheduleAdjust() {
    if (isAdjusting) return;
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
    }
    animationFrameId = window.requestAnimationFrame(() => {
      animationFrameId = null;
      adjust();
    });
  }

  const resizeObserver = new ResizeObserver(() => {
    scheduleAdjust();
  });

  const mutationObserver = new MutationObserver(() => {
    if (!isAdjusting) {
      scheduleAdjust();
    }
  });

  if (node.parentElement) {
    resizeObserver.observe(node.parentElement);
  }

  mutationObserver.observe(node, {
    childList: true,
    characterData: true,
    subtree: true,
  });

  scheduleAdjust();

  return {
    update(newOptions: AutoScaleOptions) {
      currentOptions = { ...newOptions };
      scheduleAdjust();
    },
    destroy() {
      if (animationFrameId !== null) {
        cancelAnimationFrame(animationFrameId);
      }
      resizeObserver.disconnect();
      mutationObserver.disconnect();
    },
  };
}

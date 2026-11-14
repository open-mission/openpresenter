const FOCUSABLE_SELECTOR = [
  'a[href]',
  'button:not([disabled])',
  'input:not([disabled]):not([type="hidden"])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[contenteditable="true"]',
  '[tabindex]:not([tabindex="-1"])',
].join(', ');

/**
 * Indica se o elemento está visível para receber foco.
 * Elementos com `position: fixed` têm `offsetParent === null` mesmo visíveis,
 * por isso a checagem de estilo computado complementa a de layout.
 */
function isVisible(el: HTMLElement): boolean {
  if (el.offsetParent !== null) return true;
  return getComputedStyle(el).position === 'fixed';
}

/**
 * Lista os elementos focáveis (e visíveis) dentro de um contêiner, na ordem
 * do DOM. Ignora desabilitados, `tabindex="-1"`, `aria-hidden` e itens ocultos.
 */
export function getFocusableElements(container: HTMLElement): HTMLElement[] {
  return Array.from(container.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)).filter(
    (el) => el.tabIndex >= 0 && el.getAttribute('aria-hidden') !== 'true' && isVisible(el)
  );
}

/**
 * Move o foco para o primeiro elemento que corresponde a `preferredSelector`
 * (ex.: a aba selecionada ou o primeiro campo do formulário). Se nada
 * corresponder, tenta o primeiro focável do contêiner, depois `fallback` e,
 * por fim, o próprio contêiner (que deve ter `tabindex="-1"`).
 * Usa `preventScroll` para não deslocar a página ao abrir um diálogo.
 */
export function focusInitialElement(
  container: HTMLElement,
  preferredSelector?: string,
  fallback: HTMLElement | null = null
): void {
  const preferred = preferredSelector
    ? Array.from(container.querySelectorAll<HTMLElement>(preferredSelector)).find(isVisible)
    : undefined;
  const target = preferred ?? getFocusableElements(container)[0] ?? fallback ?? container;
  target.focus({ preventScroll: true });
}

/**
 * Mantém o foco (Tab e Shift+Tab) dentro do contêiner do diálogo.
 * Sem elementos focáveis, prende o foco no próprio contêiner.
 * Deve ser chamado apenas quando o diálogo está aberto e é a camada do topo.
 */
export function trapTabKey(e: KeyboardEvent, container: HTMLElement): void {
  if (e.key !== 'Tab') return;
  const focusable = getFocusableElements(container);
  if (focusable.length === 0) {
    e.preventDefault();
    container.focus({ preventScroll: true });
    return;
  }
  const first = focusable[0];
  const last = focusable[focusable.length - 1];
  const active = document.activeElement as HTMLElement | null;
  if (!active || !container.contains(active)) {
    e.preventDefault();
    (e.shiftKey ? last : first).focus({ preventScroll: true });
    return;
  }
  if (e.shiftKey && (active === first || active === container)) {
    e.preventDefault();
    last.focus({ preventScroll: true });
  } else if (!e.shiftKey && active === last) {
    e.preventDefault();
    first.focus({ preventScroll: true });
  }
}

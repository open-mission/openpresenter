/**
 * Classes usadas pelos destaques do renderizador. O editor pode substituí-las
 * para manter contraste no tema Claro sem alterar a projeção pública.
 */
export interface FormatOptions {
  strongClass?: string;
  markClass?: string;
}

const DEFAULT_STRONG_CLASS = 'font-bold text-white';
const DEFAULT_MARK_CLASS =
  'bg-yellow-400/30 text-yellow-200 px-1 py-0.5 rounded font-semibold';

/**
 * Renderizador de formatação rica e typeset para slides de louvor e apresentações.
 * Suporta:
 * - Negrito: **texto**
 * - Itálico: *texto* ou _texto_
 * - Sublinhado: <u>texto</u> ou __texto__
 * - Highlight / Marca-texto: ==texto== ou <mark>texto</mark>
 * - Quebras de linha: \n -> <br/>
 */
export function renderFormattedText(text: string, options: FormatOptions = {}): string {
  if (!text) return '';

  const strongClass = options.strongClass ?? DEFAULT_STRONG_CLASS;
  const markClass = options.markClass ?? DEFAULT_MARK_CLASS;

  // Sanitiza caracteres especiais
  let html = text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');

  // Highlight (marca-texto): ==texto== ou <mark>texto</mark>
  html = html.replace(/==(.*?)==/g, `<mark class="${markClass}">$1</mark>`);
  html = html.replace(
    /&lt;mark&gt;(.*?)&lt;\/mark&gt;/g,
    `<mark class="${markClass}">$1</mark>`
  );

  // Sublinhado: <u>texto</u> ou __texto__
  html = html.replace(
    /&lt;u&gt;(.*?)&lt;\/u&gt;/g,
    '<u class="underline decoration-2 underline-offset-4">$1</u>'
  );
  html = html.replace(
    /__([^_]+)__/g,
    '<u class="underline decoration-2 underline-offset-4">$1</u>'
  );

  // Negrito: **texto**
  html = html.replace(/\*\*([^*]+)\*\*/g, `<strong class="${strongClass}">$1</strong>`);

  // Itálico: *texto* ou _texto_
  html = html.replace(/\*([^*]+)\*/g, '<em class="italic">$1</em>');
  html = html.replace(/_([^_]+)_/g, '<em class="italic">$1</em>');

  // Quebras de linha
  return html.replace(/\n/g, '<br/>');
}

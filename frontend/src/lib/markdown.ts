/**
 * Renderizador de formatação rica e typeset para slides de louvor e apresentações.
 * Suporta:
 * - Negrito: **texto**
 * - Itálico: *texto* ou _texto_
 * - Sublinhado: <u>texto</u> ou __texto__
 * - Highlight / Marca-texto: ==texto== ou <mark>texto</mark>
 * - Quebras de linha: \n -> <br/>
 */
export function renderFormattedText(text: string): string {
  if (!text) return '';

  // Sanitiza caracteres especiais
  let html = text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');

  // Highlight (marca-texto): ==texto== ou <mark>texto</mark>
  html = html.replace(
    /==(.*?)==/g,
    '<mark class="bg-yellow-400/30 text-yellow-200 px-1 py-0.5 rounded font-semibold">$1</mark>'
  );
  html = html.replace(
    /&lt;mark&gt;(.*?)&lt;\/mark&gt;/g,
    '<mark class="bg-yellow-400/30 text-yellow-200 px-1 py-0.5 rounded font-semibold">$1</mark>'
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
  html = html.replace(/\*\*([^*]+)\*\*/g, '<strong class="font-bold text-white">$1</strong>');

  // Itálico: *texto* ou _texto_
  html = html.replace(/\*([^*]+)\*/g, '<em class="italic">$1</em>');
  html = html.replace(/_([^_]+)_/g, '<em class="italic">$1</em>');

  // Quebras de linha
  return html.replace(/\n/g, '<br/>');
}

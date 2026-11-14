export interface HighlightSegment {
  text: string;
  match: boolean;
}

/**
 * Divide um texto em segmentos destacáveis a partir do termo buscado.
 * Comparação sem distinção de maiúsculas/minúsculas; termos curtos não destacam.
 */
export function highlightSegments(text: string, query: string): HighlightSegment[] {
  const value = text ?? '';
  const term = query.trim();
  if (!value || term.length < 2) {
    return [{ text: value, match: false }];
  }

  const lowerValue = value.toLocaleLowerCase('pt-BR');
  const lowerTerm = term.toLocaleLowerCase('pt-BR');
  const segments: HighlightSegment[] = [];
  let cursor = 0;

  let hit = lowerValue.indexOf(lowerTerm, cursor);
  while (hit !== -1) {
    if (hit > cursor) {
      segments.push({ text: value.slice(cursor, hit), match: false });
    }
    segments.push({ text: value.slice(hit, hit + lowerTerm.length), match: true });
    cursor = hit + lowerTerm.length;
    hit = lowerValue.indexOf(lowerTerm, cursor);
  }

  if (cursor < value.length) {
    segments.push({ text: value.slice(cursor), match: false });
  }

  return segments.length > 0 ? segments : [{ text: value, match: false }];
}

/**
 * Converte o snippet FTS5 (`<b>termo</b>`) em HTML seguro com `<mark>`.
 * Todo o restante é escapado antes de reintroduzir apenas o destaque.
 */
export function snippetToHtml(raw: string): string {
  const escaped = raw
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');
  return escaped.replace(/&lt;b&gt;/g, '<mark>').replace(/&lt;\/b&gt;/g, '</mark>');
}

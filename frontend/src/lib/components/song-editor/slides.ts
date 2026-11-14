export interface SlideLocation {
  slideIndex: number;
  sectionLabel: string;
  sectionHeaderStart: number;
  sectionHeaderEnd: number;
  isFirstInSec: boolean;
  blockStart: number;
  blockEnd: number;
}

/**
 * Mapeia cada slide gerado pelo parser de volta ao intervalo de texto bruto
 * que o originou, incluindo o cabeçalho `## Seção` quando aplicável.
 */
export function getSlideLocations(raw: string): SlideLocation[] {
  const locations: SlideLocation[] = [];
  const lineRegex = /([^\r\n]*)(\r?\n|$)/g;
  let match: RegExpExecArray | null;

  let currentSectionLabel = 'Verso 1';
  let currentSecHeaderStart = -1;
  let currentSecHeaderEnd = -1;
  let isFirstInSec = true;

  let currentBlockStart = -1;
  let currentBlockEnd = -1;
  let inBlock = false;

  while ((match = lineRegex.exec(raw)) !== null) {
    if (match.index === raw.length && match[0] === '') break;

    const line = match[1];
    const lineStart = match.index;
    const lineEnd = lineStart + line.length;
    const trimmed = line.trim();

    if (trimmed.startsWith('##') || trimmed.startsWith('# ')) {
      if (inBlock) {
        locations.push({
          slideIndex: locations.length,
          sectionLabel: currentSectionLabel,
          sectionHeaderStart: currentSecHeaderStart,
          sectionHeaderEnd: currentSecHeaderEnd,
          isFirstInSec,
          blockStart: currentBlockStart,
          blockEnd: currentBlockEnd,
        });
        inBlock = false;
        isFirstInSec = false;
      }
      currentSectionLabel = trimmed.replace(/^#+\s*/, '').trim() || 'Seção';
      currentSecHeaderStart = lineStart;
      currentSecHeaderEnd = lineEnd;
      isFirstInSec = true;
    } else if (trimmed === '') {
      if (inBlock) {
        locations.push({
          slideIndex: locations.length,
          sectionLabel: currentSectionLabel,
          sectionHeaderStart: currentSecHeaderStart,
          sectionHeaderEnd: currentSecHeaderEnd,
          isFirstInSec,
          blockStart: currentBlockStart,
          blockEnd: currentBlockEnd,
        });
        inBlock = false;
        isFirstInSec = false;
      }
    } else {
      if (!inBlock) {
        inBlock = true;
        currentBlockStart = lineStart;
      }
      currentBlockEnd = lineEnd;
    }

    if (match[2] === '') break;
  }

  if (inBlock) {
    locations.push({
      slideIndex: locations.length,
      sectionLabel: currentSectionLabel,
      sectionHeaderStart: currentSecHeaderStart,
      sectionHeaderEnd: currentSecHeaderEnd,
      isFirstInSec,
      blockStart: currentBlockStart,
      blockEnd: currentBlockEnd,
    });
  }

  return locations;
}

/** Renomeia a seção de um slide, reaproveitando o cabeçalho existente quando possível. */
export function setSlideSection(raw: string, slideIdx: number, newLabel: string): string | null {
  const loc = getSlideLocations(raw)[slideIdx];
  if (!loc) return null;

  if (loc.isFirstInSec && loc.sectionHeaderStart >= 0) {
    return raw.slice(0, loc.sectionHeaderStart) + `## ${newLabel}` + raw.slice(loc.sectionHeaderEnd);
  }

  const before = raw.slice(0, loc.blockStart).trimEnd();
  const after = raw.slice(loc.blockStart);
  return `${before}\n\n## ${newLabel}\n${after}`;
}

/** Duplica o bloco de texto de um slide imediatamente após o original. */
export function duplicateSlide(raw: string, slideIdx: number): string | null {
  const loc = getSlideLocations(raw)[slideIdx];
  if (!loc) return null;

  const slideContent = raw.slice(loc.blockStart, loc.blockEnd);
  const before = raw.slice(0, loc.blockEnd);
  const after = raw.slice(loc.blockEnd);
  return `${before}\n\n${slideContent}${after}`;
}

/** Remove um slide e o cabeçalho de seção quando ele era o único bloco daquela seção. */
export function deleteSlide(raw: string, slideIdx: number): string | null {
  const locations = getSlideLocations(raw);
  const loc = locations[slideIdx];
  if (!loc) return null;

  const hasSiblingInSec = locations.some(
    (item, idx) => idx !== slideIdx && item.sectionHeaderStart === loc.sectionHeaderStart
  );

  let deleteStart = loc.blockStart;
  if (!hasSiblingInSec && loc.sectionHeaderStart >= 0) {
    deleteStart = loc.sectionHeaderStart;
  }

  let deleteEnd = loc.blockEnd;
  while (deleteEnd < raw.length && (raw[deleteEnd] === '\n' || raw[deleteEnd] === '\r')) {
    deleteEnd++;
  }

  const before = raw.slice(0, deleteStart).trimEnd();
  const after = raw.slice(deleteEnd).trimStart();

  if (before && after) {
    const joiner = before.split('\n').pop()?.startsWith('#') ? '\n' : '\n\n';
    return `${before}${joiner}${after}`;
  }

  return before || after || '';
}

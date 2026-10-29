import type { SectionType, Slide, SongSection } from './types';

export function getSectionType(name: string): SectionType {
  const lower = name.toLowerCase().trim();
  if (lower.includes('pre-refr') || lower.includes('pré-refr') || lower.includes('pre-chorus')) {
    return 'pre-chorus';
  }
  if (lower.includes('refr') || lower.includes('coro') || lower.includes('chorus')) {
    return 'chorus';
  }
  if (lower.includes('ponte') || lower.includes('bridge')) {
    return 'bridge';
  }
  if (lower.includes('intro') || lower.includes('início') || lower.includes('introducao')) {
    return 'intro';
  }
  if (lower.includes('final') || lower.includes('outro')) {
    return 'outro';
  }
  if (lower.includes('instrumental') || lower.includes('solo')) {
    return 'instrumental';
  }
  if (lower.includes('tag')) {
    return 'tag';
  }
  if (lower.includes('encerramento') || lower.includes('ending')) {
    return 'ending';
  }
  if (lower.includes('verso') || lower.includes('verse') || lower.includes('estrofe')) {
    return 'verse';
  }
  return 'other';
}

export function getSectionColor(name: string): { bg: string; text: string; border: string; accent: string } {
  const lower = name.toLowerCase();
  if (lower.includes('refr') || lower.includes('coro')) {
    return {
      bg: 'bg-emerald-950/40',
      text: 'text-emerald-300',
      border: 'border-emerald-600/40',
      accent: 'bg-emerald-500',
    };
  }
  if (lower.includes('ponte') || lower.includes('bridge')) {
    return {
      bg: 'bg-amber-950/40',
      text: 'text-amber-300',
      border: 'border-amber-600/40',
      accent: 'bg-amber-500',
    };
  }
  if (lower.includes('intro') || lower.includes('início')) {
    return {
      bg: 'bg-cyan-950/40',
      text: 'text-cyan-300',
      border: 'border-cyan-600/40',
      accent: 'bg-cyan-500',
    };
  }
  if (lower.includes('final') || lower.includes('outro')) {
    return {
      bg: 'bg-purple-950/40',
      text: 'text-purple-300',
      border: 'border-purple-600/40',
      accent: 'bg-purple-500',
    };
  }
  if (lower.includes('pre-refr') || lower.includes('pré-refr')) {
    return {
      bg: 'bg-teal-950/40',
      text: 'text-teal-300',
      border: 'border-teal-600/40',
      accent: 'bg-teal-500',
    };
  }
  // Padrão para Verso / Outros
  return {
    bg: 'bg-blue-950/40',
    text: 'text-blue-300',
    border: 'border-blue-600/40',
    accent: 'bg-blue-500',
  };
}

export function parseRawLyrics(raw: string): { sections: SongSection[]; slides: Slide[] } {
  const lines = raw.split(/\r?\n/);
  const sections: SongSection[] = [];
  const slides: Slide[] = [];

  interface RawSection {
    label: string;
    blocks: string[];
  }

  const rawSections: RawSection[] = [];
  let currentLabel = 'Verso 1';
  let currentBlockLines: string[] = [];
  let currentBlocks: string[] = [];

  const flushBlock = () => {
    if (currentBlockLines.length > 0) {
      const text = currentBlockLines.join('\n').trim();
      if (text.length > 0) {
        currentBlocks.push(text);
      }
      currentBlockLines = [];
    }
  };

  const flushSection = () => {
    flushBlock();
    if (currentBlocks.length > 0) {
      rawSections.push({
        label: currentLabel,
        blocks: [...currentBlocks],
      });
      currentBlocks = [];
    }
  };

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith('##') || trimmed.startsWith('# ')) {
      flushSection();
      const rawTitle = trimmed.replace(/^#+\s*/, '').trim();
      currentLabel = rawTitle || 'Seção';
    } else if (trimmed === '') {
      flushBlock();
    } else {
      currentBlockLines.push(line);
    }
  }

  flushSection();

  let globalSlideIdx = 1;

  for (const sec of rawSections) {
    const totalBlocks = sec.blocks.length;
    const lines = sec.blocks.flatMap((b) => b.split('\n'));

    sections.push({
      section_type: getSectionType(sec.label),
      label: sec.label,
      lines,
    });

    sec.blocks.forEach((block, idx) => {
      const label = totalBlocks > 1 ? `${sec.label} (${idx + 1}/${totalBlocks})` : sec.label;
      slides.push({
        id: `slide-${globalSlideIdx++}`,
        label,
        content: block,
      });
    });
  }

  return { sections, slides };
}

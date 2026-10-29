import type { ParsedReference } from './types';

interface CanonicalBook {
  id: number;
  testament: number;
  name: string;
  abbreviation: string;
  aliases: string[];
}

export const CANONICAL_BOOKS: CanonicalBook[] = [
  // Antigo Testamento (1..39)
  { id: 1, testament: 1, name: 'Gênesis', abbreviation: 'Gn', aliases: ['genesis', 'gn', 'gen'] },
  { id: 2, testament: 1, name: 'Êxodo', abbreviation: 'Ex', aliases: ['exodo', 'ex', 'exo'] },
  { id: 3, testament: 1, name: 'Levítico', abbreviation: 'Lv', aliases: ['levitico', 'lv', 'lev'] },
  { id: 4, testament: 1, name: 'Números', abbreviation: 'Nm', aliases: ['numeros', 'nm', 'num'] },
  { id: 5, testament: 1, name: 'Deuteronômio', abbreviation: 'Dt', aliases: ['deuteronomio', 'dt', 'deut'] },
  { id: 6, testament: 1, name: 'Josué', abbreviation: 'Js', aliases: ['josue', 'js', 'jos'] },
  { id: 7, testament: 1, name: 'Juízes', abbreviation: 'Jz', aliases: ['juizes', 'jz', 'jui'] },
  { id: 8, testament: 1, name: 'Rute', abbreviation: 'Rt', aliases: ['rute', 'rt', 'rut'] },
  { id: 9, testament: 1, name: '1 Samuel', abbreviation: '1Sm', aliases: ['1 samuel', '1samuel', '1 sm', '1sm', '1sam', 'i samuel', 'i sm'] },
  { id: 10, testament: 1, name: '2 Samuel', abbreviation: '2Sm', aliases: ['2 samuel', '2samuel', '2 sm', '2sm', '2sam', 'ii samuel', 'ii sm'] },
  { id: 11, testament: 1, name: '1 Reis', abbreviation: '1Rs', aliases: ['1 reis', '1reis', '1 rs', '1rs', '1re', 'i reis', 'i rs'] },
  { id: 12, testament: 1, name: '2 Reis', abbreviation: '2Rs', aliases: ['2 reis', '2reis', '2 rs', '2rs', '2re', 'ii reis', 'ii rs'] },
  { id: 13, testament: 1, name: '1 Crônicas', abbreviation: '1Cr', aliases: ['1 cronicas', '1cronicas', '1 cr', '1cr', '1cro', 'i cronicas', 'i cr'] },
  { id: 14, testament: 1, name: '2 Crônicas', abbreviation: '2Cr', aliases: ['2 cronicas', '2cronicas', '2 cr', '2cr', '2cro', 'ii cronicas', 'ii cr'] },
  { id: 15, testament: 1, name: 'Esdras', abbreviation: 'Ed', aliases: ['esdras', 'ed', 'esd'] },
  { id: 16, testament: 1, name: 'Neemias', abbreviation: 'Ne', aliases: ['neemias', 'ne', 'neem'] },
  { id: 17, testament: 1, name: 'Ester', abbreviation: 'Et', aliases: ['ester', 'et', 'est'] },
  { id: 18, testament: 1, name: 'Jó', abbreviation: 'Jó', aliases: ['job'] },
  { id: 19, testament: 1, name: 'Salmos', abbreviation: 'Sl', aliases: ['salmos', 'salmo', 'sl', 'sal', 'psalms', 'ps'] },
  { id: 20, testament: 1, name: 'Provérbios', abbreviation: 'Pv', aliases: ['proverbios', 'pv', 'prov'] },
  { id: 21, testament: 1, name: 'Eclesiastes', abbreviation: 'Ec', aliases: ['eclesiastes', 'ec', 'ecl'] },
  { id: 22, testament: 1, name: 'Cânticos', abbreviation: 'Ct', aliases: ['canticos', 'cantares', 'cantares de salomao', 'cantico dos canticos', 'ct', 'cant'] },
  { id: 23, testament: 1, name: 'Isaías', abbreviation: 'Is', aliases: ['isaias', 'is', 'isa'] },
  { id: 24, testament: 1, name: 'Jeremias', abbreviation: 'Jr', aliases: ['jeremias', 'jr', 'jer'] },
  { id: 25, testament: 1, name: 'Lamentações', abbreviation: 'Lm', aliases: ['lamentacoes', 'lamentacoes de jeremias', 'lm', 'lam'] },
  { id: 26, testament: 1, name: 'Ezequiel', abbreviation: 'Ez', aliases: ['ezequiel', 'ez', 'eze'] },
  { id: 27, testament: 1, name: 'Daniel', abbreviation: 'Dn', aliases: ['daniel', 'dn', 'dan'] },
  { id: 28, testament: 1, name: 'Oseias', abbreviation: 'Os', aliases: ['oseias', 'os', 'hos'] },
  { id: 29, testament: 1, name: 'Joel', abbreviation: 'Jl', aliases: ['joel', 'jl', 'joe'] },
  { id: 30, testament: 1, name: 'Amós', abbreviation: 'Am', aliases: ['amos', 'am', 'amo'] },
  { id: 31, testament: 1, name: 'Obadias', abbreviation: 'Ob', aliases: ['obadias', 'ob', 'oba'] },
  { id: 32, testament: 1, name: 'Jonas', abbreviation: 'Jn', aliases: ['jonas', 'jn', 'jon'] },
  { id: 33, testament: 1, name: 'Miqueias', abbreviation: 'Mq', aliases: ['miqueias', 'mq', 'miq'] },
  { id: 34, testament: 1, name: 'Naum', abbreviation: 'Na', aliases: ['naum', 'na', 'nah'] },
  { id: 35, testament: 1, name: 'Habacuque', abbreviation: 'Hc', aliases: ['habacuque', 'hc', 'hab'] },
  { id: 36, testament: 1, name: 'Sofonias', abbreviation: 'Sf', aliases: ['sofonias', 'sf', 'sof'] },
  { id: 37, testament: 1, name: 'Ageu', abbreviation: 'Ag', aliases: ['ageu', 'ag', 'hag'] },
  { id: 38, testament: 1, name: 'Zacarias', abbreviation: 'Zc', aliases: ['zacarias', 'zc', 'zac'] },
  { id: 39, testament: 1, name: 'Malaquias', abbreviation: 'Ml', aliases: ['malaquias', 'ml', 'mal'] },

  // Novo Testamento (40..66)
  { id: 40, testament: 2, name: 'Mateus', abbreviation: 'Mt', aliases: ['mateus', 'mt', 'mat'] },
  { id: 41, testament: 2, name: 'Marcos', abbreviation: 'Mc', aliases: ['marcos', 'mc', 'mar', 'mk'] },
  { id: 42, testament: 2, name: 'Lucas', abbreviation: 'Lc', aliases: ['lucas', 'lc', 'luc', 'lk'] },
  { id: 43, testament: 2, name: 'João', abbreviation: 'Jo', aliases: ['joao', 'jo', 'jhn', 'john'] },
  { id: 44, testament: 2, name: 'Atos', abbreviation: 'At', aliases: ['atos', 'atos dos apostolos', 'at', 'act'] },
  { id: 45, testament: 2, name: 'Romanos', abbreviation: 'Rm', aliases: ['romanos', 'rm', 'rom'] },
  { id: 46, testament: 2, name: '1 Coríntios', abbreviation: '1Co', aliases: ['1 corintios', '1corintios', '1 co', '1co', '1cor', 'i corintios', 'i co'] },
  { id: 47, testament: 2, name: '2 Coríntios', abbreviation: '2Co', aliases: ['2 corintios', '2corintios', '2 co', '2co', '2cor', 'ii corintios', 'ii co'] },
  { id: 48, testament: 2, name: 'Gálatas', abbreviation: 'Gl', aliases: ['galatas', 'gl', 'gal'] },
  { id: 49, testament: 2, name: 'Efésios', abbreviation: 'Ef', aliases: ['efesios', 'ef', 'efe', 'eph'] },
  { id: 50, testament: 2, name: 'Filipenses', abbreviation: 'Fp', aliases: ['filipenses', 'fp', 'fil', 'php'] },
  { id: 51, testament: 2, name: 'Colossenses', abbreviation: 'Cl', aliases: ['colossenses', 'cl', 'col'] },
  { id: 52, testament: 2, name: '1 Tessalonicenses', abbreviation: '1Ts', aliases: ['1 tessalonicenses', '1tessalonicenses', '1 ts', '1ts', '1tes', 'i tessalonicenses', 'i ts'] },
  { id: 53, testament: 2, name: '2 Tessalonicenses', abbreviation: '2Ts', aliases: ['2 tessalonicenses', '2tessalonicenses', '2 ts', '2ts', '2tes', 'ii tessalonicenses', 'ii ts'] },
  { id: 54, testament: 2, name: '1 Timóteo', abbreviation: '1Tm', aliases: ['1 timoteo', '1timoteo', '1 tm', '1tm', '1tim', 'i timoteo', 'i tm'] },
  { id: 55, testament: 2, name: '2 Timóteo', abbreviation: '2Tm', aliases: ['2 timoteo', '2timoteo', '2 tm', '2tm', '2tim', 'ii timoteo', 'ii tm'] },
  { id: 56, testament: 2, name: 'Tito', abbreviation: 'Tt', aliases: ['tito', 'tt', 'tit'] },
  { id: 57, testament: 2, name: 'Filemom', abbreviation: 'Fm', aliases: ['filemom', 'fm', 'flm', 'filem'] },
  { id: 58, testament: 2, name: 'Hebreus', abbreviation: 'Hb', aliases: ['hebreus', 'hb', 'heb'] },
  { id: 59, testament: 2, name: 'Tiago', abbreviation: 'Tg', aliases: ['tiago', 'tg', 'tia', 'jas'] },
  { id: 60, testament: 2, name: '1 Pedro', abbreviation: '1Pe', aliases: ['1 pedro', '1pedro', '1 pe', '1pe', '1ped', 'i pedro', 'i pe'] },
  { id: 61, testament: 2, name: '2 Pedro', abbreviation: '2Pe', aliases: ['2 pedro', '2pedro', '2 pe', '2pe', '2ped', 'ii pedro', 'ii pe'] },
  { id: 62, testament: 2, name: '1 João', abbreviation: '1Jo', aliases: ['1 joao', '1joao', '1 jo', '1jo', 'i joao', 'i jo'] },
  { id: 63, testament: 2, name: '2 João', abbreviation: '2Jo', aliases: ['2 joao', '2joao', '2 jo', '2jo', 'ii joao', 'ii jo'] },
  { id: 64, testament: 2, name: '3 João', abbreviation: '3Jo', aliases: ['3 joao', '3joao', '3 jo', '3jo', 'iii joao', 'iii jo'] },
  { id: 65, testament: 2, name: 'Judas', abbreviation: 'Jd', aliases: ['judas', 'jd', 'jud'] },
  { id: 66, testament: 2, name: 'Apocalipse', abbreviation: 'Ap', aliases: ['apocalipse', 'ap', 'apoc', 'revelation', 'rev'] },
];

export function normalizeString(str: string): string {
  return str
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .trim();
}

export function findCanonicalBook(input: string): CanonicalBook | null {
  const trimmed = input.trim();
  if (!trimmed) return null;

  if (trimmed.toLowerCase() === 'job' || trimmed === 'jó' || trimmed === 'Jó') {
    return CANONICAL_BOOKS.find((b) => b.id === 18) || null;
  }
  if (trimmed.toLowerCase() === 'jo') {
    return CANONICAL_BOOKS.find((b) => b.id === 43) || null;
  }

  const norm = normalizeString(trimmed);

  for (const book of CANONICAL_BOOKS) {
    for (const alias of book.aliases) {
      if (norm === alias) return book;
    }
    if (norm === normalizeString(book.name) || norm === normalizeString(book.abbreviation)) {
      return book;
    }
  }

  return null;
}

export function parseBibleReference(raw: string): ParsedReference | null {
  const trimmed = raw.trim();
  if (!trimmed) return null;

  let sawAlpha = false;
  let splitIdx = -1;

  for (let i = 0; i < trimmed.length; i++) {
    const c = trimmed[i];
    if (/[a-zA-Z\u00C0-\u024F]/.test(c)) {
      sawAlpha = true;
    } else if (sawAlpha && /[0-9]/.test(c)) {
      splitIdx = i;
      break;
    }
  }

  if (splitIdx === -1) return null;

  const bookPart = trimmed.substring(0, splitIdx).trim();
  const cvPart = trimmed.substring(splitIdx).trim();

  if (!bookPart || !cvPart) return null;

  const book = findCanonicalBook(bookPart);
  if (!book) return null;

  const cvClean = cvPart.replace(/[:.,]/g, ' ');
  const tokens = cvClean.split(/\s+/).filter(Boolean);

  if (tokens.length === 0) return null;

  // Se o último token contiver apenas letras alfabéticas (ex: "ARA", "NVI", "ARC"), tratamos como versão bíblica
  let version: string | undefined = undefined;
  if (tokens.length >= 2 && /^[a-zA-Z\u00C0-\u024F]+$/.test(tokens[tokens.length - 1])) {
    version = tokens.pop()!.toUpperCase();
  }

  if (tokens.length === 0) return null;

  const chapter = parseInt(tokens[0], 10);
  if (isNaN(chapter)) return null;

  let verseStart: number | undefined = 1;
  let verseEnd: number | undefined = 1;

  if (tokens.length >= 2) {
    const verseToken = tokens[1];
    if (verseToken.includes('-')) {
      const parts = verseToken.split('-');
      verseStart = parseInt(parts[0], 10) || 1;
      verseEnd = parseInt(parts[1], 10) || verseStart;
    } else {
      verseStart = parseInt(verseToken, 10) || 1;
      verseEnd = verseStart;
    }
  } else {
    // Apenas capítulo! (Ex: "Salmos 91" ou "Salmos 91 ARA") -> foca no verso 1
    verseStart = 1;
    verseEnd = 1;
  }

  return {
    book_id: book.id,
    book_name: book.name,
    book_abbreviation: book.abbreviation,
    chapter,
    verse_start: verseStart,
    verse_end: verseEnd,
    version,
  };
}

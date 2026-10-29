use serde::{Deserialize, Serialize};

/// Resumo de uma versão da Bíblia cadastrada no aplicativo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BibleVersionSummary {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
    pub filename: String,
    pub is_default: bool,
    #[serde(default)]
    pub book_count: usize,
    #[serde(default)]
    pub verse_count: usize,
    pub created_at: String,
    pub updated_at: String,
}

/// Livro bíblico com metadados e contagem de capítulos.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BibleBook {
    pub id: i64,
    pub book_reference_id: i64,
    pub testament_reference_id: i64, // 1 = Antigo Testamento, 2 = Novo Testamento
    pub name: String,
    pub abbreviation: String,
    pub chapter_count: i64,
}

/// Versículo bíblico individual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BibleVerse {
    pub id: i64,
    pub book_id: i64,
    pub chapter: i64,
    pub verse: i64,
    pub text: String,
}

/// Resultado de busca textual por versículo bíblico.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BibleSearchResult {
    pub book_id: i64,
    pub book_name: String,
    pub book_abbreviation: String,
    pub chapter: i64,
    pub verse: i64,
    pub text: String,
}

/// Referência bíblica interpretada pelo parser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedReference {
    pub book_id: i64,
    pub book_name: String,
    pub book_abbreviation: String,
    pub chapter: i64,
    pub verse_start: Option<i64>,
    pub verse_end: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Definição canônica estática de um livro da Bíblia.
#[derive(Debug, Clone)]
pub struct CanonicalBook {
    pub id: i64,
    pub testament: i64, // 1 = AT, 2 = NT
    pub name: &'static str,
    pub abbreviation: &'static str,
    pub aliases: &'static [&'static str],
}

pub static CANONICAL_BOOKS: &[CanonicalBook] = &[
    // Antigo Testamento (1..39)
    CanonicalBook {
        id: 1,
        testament: 1,
        name: "Gênesis",
        abbreviation: "Gn",
        aliases: &["genesis", "gn", "gen"],
    },
    CanonicalBook {
        id: 2,
        testament: 1,
        name: "Êxodo",
        abbreviation: "Ex",
        aliases: &["exodo", "ex", "exo"],
    },
    CanonicalBook {
        id: 3,
        testament: 1,
        name: "Levítico",
        abbreviation: "Lv",
        aliases: &["levitico", "lv", "lev"],
    },
    CanonicalBook {
        id: 4,
        testament: 1,
        name: "Números",
        abbreviation: "Nm",
        aliases: &["numeros", "nm", "num"],
    },
    CanonicalBook {
        id: 5,
        testament: 1,
        name: "Deuteronômio",
        abbreviation: "Dt",
        aliases: &["deuteronomio", "dt", "deut"],
    },
    CanonicalBook {
        id: 6,
        testament: 1,
        name: "Josué",
        abbreviation: "Js",
        aliases: &["josue", "js", "jos"],
    },
    CanonicalBook {
        id: 7,
        testament: 1,
        name: "Juízes",
        abbreviation: "Jz",
        aliases: &["juizes", "jz", "jui"],
    },
    CanonicalBook {
        id: 8,
        testament: 1,
        name: "Rute",
        abbreviation: "Rt",
        aliases: &["rute", "rt", "rut"],
    },
    CanonicalBook {
        id: 9,
        testament: 1,
        name: "1 Samuel",
        abbreviation: "1Sm",
        aliases: &[
            "1 samuel", "1samuel", "1 sm", "1sm", "1sam", "i samuel", "i sm",
        ],
    },
    CanonicalBook {
        id: 10,
        testament: 1,
        name: "2 Samuel",
        abbreviation: "2Sm",
        aliases: &[
            "2 samuel",
            "2samuel",
            "2 sm",
            "2sm",
            "2sam",
            "ii samuel",
            "ii sm",
        ],
    },
    CanonicalBook {
        id: 11,
        testament: 1,
        name: "1 Reis",
        abbreviation: "1Rs",
        aliases: &["1 reis", "1reis", "1 rs", "1rs", "1re", "i reis", "i rs"],
    },
    CanonicalBook {
        id: 12,
        testament: 1,
        name: "2 Reis",
        abbreviation: "2Rs",
        aliases: &["2 reis", "2reis", "2 rs", "2rs", "2re", "ii reis", "ii rs"],
    },
    CanonicalBook {
        id: 13,
        testament: 1,
        name: "1 Crônicas",
        abbreviation: "1Cr",
        aliases: &[
            "1 cronicas",
            "1cronicas",
            "1 cr",
            "1cr",
            "1cro",
            "i cronicas",
            "i cr",
        ],
    },
    CanonicalBook {
        id: 14,
        testament: 1,
        name: "2 Crônicas",
        abbreviation: "2Cr",
        aliases: &[
            "2 cronicas",
            "2cronicas",
            "2 cr",
            "2cr",
            "2cro",
            "ii cronicas",
            "ii cr",
        ],
    },
    CanonicalBook {
        id: 15,
        testament: 1,
        name: "Esdras",
        abbreviation: "Ed",
        aliases: &["esdras", "ed", "esd"],
    },
    CanonicalBook {
        id: 16,
        testament: 1,
        name: "Neemias",
        abbreviation: "Ne",
        aliases: &["neemias", "ne", "neem"],
    },
    CanonicalBook {
        id: 17,
        testament: 1,
        name: "Ester",
        abbreviation: "Et",
        aliases: &["ester", "et", "est"],
    },
    CanonicalBook {
        id: 18,
        testament: 1,
        name: "Jó",
        abbreviation: "Jó",
        aliases: &["job"],
    },
    CanonicalBook {
        id: 19,
        testament: 1,
        name: "Salmos",
        abbreviation: "Sl",
        aliases: &["salmos", "salmo", "sl", "sal", "psalms", "ps"],
    },
    CanonicalBook {
        id: 20,
        testament: 1,
        name: "Provérbios",
        abbreviation: "Pv",
        aliases: &["proverbios", "pv", "prov"],
    },
    CanonicalBook {
        id: 21,
        testament: 1,
        name: "Eclesiastes",
        abbreviation: "Ec",
        aliases: &["eclesiastes", "ec", "ecl"],
    },
    CanonicalBook {
        id: 22,
        testament: 1,
        name: "Cânticos",
        abbreviation: "Ct",
        aliases: &[
            "canticos",
            "cantares",
            "cantares de salomao",
            "cantico dos canticos",
            "ct",
            "cant",
        ],
    },
    CanonicalBook {
        id: 23,
        testament: 1,
        name: "Isaías",
        abbreviation: "Is",
        aliases: &["isaias", "is", "isa"],
    },
    CanonicalBook {
        id: 24,
        testament: 1,
        name: "Jeremias",
        abbreviation: "Jr",
        aliases: &["jeremias", "jr", "jer"],
    },
    CanonicalBook {
        id: 25,
        testament: 1,
        name: "Lamentações",
        abbreviation: "Lm",
        aliases: &["lamentacoes", "lamentacoes de jeremias", "lm", "lam"],
    },
    CanonicalBook {
        id: 26,
        testament: 1,
        name: "Ezequiel",
        abbreviation: "Ez",
        aliases: &["ezequiel", "ez", "eze"],
    },
    CanonicalBook {
        id: 27,
        testament: 1,
        name: "Daniel",
        abbreviation: "Dn",
        aliases: &["daniel", "dn", "dan"],
    },
    CanonicalBook {
        id: 28,
        testament: 1,
        name: "Oseias",
        abbreviation: "Os",
        aliases: &["oseias", "os", "hos"],
    },
    CanonicalBook {
        id: 29,
        testament: 1,
        name: "Joel",
        abbreviation: "Jl",
        aliases: &["joel", "jl", "joe"],
    },
    CanonicalBook {
        id: 30,
        testament: 1,
        name: "Amós",
        abbreviation: "Am",
        aliases: &["amos", "am", "amo"],
    },
    CanonicalBook {
        id: 31,
        testament: 1,
        name: "Obadias",
        abbreviation: "Ob",
        aliases: &["obadias", "ob", "oba"],
    },
    CanonicalBook {
        id: 32,
        testament: 1,
        name: "Jonas",
        abbreviation: "Jn",
        aliases: &["jonas", "jn", "jon"],
    },
    CanonicalBook {
        id: 33,
        testament: 1,
        name: "Miqueias",
        abbreviation: "Mq",
        aliases: &["miqueias", "mq", "miq"],
    },
    CanonicalBook {
        id: 34,
        testament: 1,
        name: "Naum",
        abbreviation: "Na",
        aliases: &["naum", "na", "nah"],
    },
    CanonicalBook {
        id: 35,
        testament: 1,
        name: "Habacuque",
        abbreviation: "Hc",
        aliases: &["habacuque", "hc", "hab"],
    },
    CanonicalBook {
        id: 36,
        testament: 1,
        name: "Sofonias",
        abbreviation: "Sf",
        aliases: &["sofonias", "sf", "sof"],
    },
    CanonicalBook {
        id: 37,
        testament: 1,
        name: "Ageu",
        abbreviation: "Ag",
        aliases: &["ageu", "ag", "hag"],
    },
    CanonicalBook {
        id: 38,
        testament: 1,
        name: "Zacarias",
        abbreviation: "Zc",
        aliases: &["zacarias", "zc", "zac"],
    },
    CanonicalBook {
        id: 39,
        testament: 1,
        name: "Malaquias",
        abbreviation: "Ml",
        aliases: &["malaquias", "ml", "mal"],
    },
    // Novo Testamento (40..66)
    CanonicalBook {
        id: 40,
        testament: 2,
        name: "Mateus",
        abbreviation: "Mt",
        aliases: &["mateus", "mt", "mat"],
    },
    CanonicalBook {
        id: 41,
        testament: 2,
        name: "Marcos",
        abbreviation: "Mc",
        aliases: &["marcos", "mc", "mar", "mk"],
    },
    CanonicalBook {
        id: 42,
        testament: 2,
        name: "Lucas",
        abbreviation: "Lc",
        aliases: &["lucas", "lc", "luc", "lk"],
    },
    CanonicalBook {
        id: 43,
        testament: 2,
        name: "João",
        abbreviation: "Jo",
        aliases: &["joao", "jo", "jhn", "john"],
    },
    CanonicalBook {
        id: 44,
        testament: 2,
        name: "Atos",
        abbreviation: "At",
        aliases: &["atos", "atos dos apostolos", "at", "act"],
    },
    CanonicalBook {
        id: 45,
        testament: 2,
        name: "Romanos",
        abbreviation: "Rm",
        aliases: &["romanos", "rm", "rom"],
    },
    CanonicalBook {
        id: 46,
        testament: 2,
        name: "1 Coríntios",
        abbreviation: "1Co",
        aliases: &[
            "1 corintios",
            "1corintios",
            "1 co",
            "1co",
            "1cor",
            "i corintios",
            "i co",
        ],
    },
    CanonicalBook {
        id: 47,
        testament: 2,
        name: "2 Coríntios",
        abbreviation: "2Co",
        aliases: &[
            "2 corintios",
            "2corintios",
            "2 co",
            "2co",
            "2cor",
            "ii corintios",
            "ii co",
        ],
    },
    CanonicalBook {
        id: 48,
        testament: 2,
        name: "Gálatas",
        abbreviation: "Gl",
        aliases: &["galatas", "gl", "gal"],
    },
    CanonicalBook {
        id: 49,
        testament: 2,
        name: "Efésios",
        abbreviation: "Ef",
        aliases: &["efesios", "ef", "efe", "eph"],
    },
    CanonicalBook {
        id: 50,
        testament: 2,
        name: "Filipenses",
        abbreviation: "Fp",
        aliases: &["filipenses", "fp", "fil", "php"],
    },
    CanonicalBook {
        id: 51,
        testament: 2,
        name: "Colossenses",
        abbreviation: "Cl",
        aliases: &["colossenses", "cl", "col"],
    },
    CanonicalBook {
        id: 52,
        testament: 2,
        name: "1 Tessalonicenses",
        abbreviation: "1Ts",
        aliases: &[
            "1 tessalonicenses",
            "1tessalonicenses",
            "1 ts",
            "1ts",
            "1tes",
            "i tessalonicenses",
            "i ts",
        ],
    },
    CanonicalBook {
        id: 53,
        testament: 2,
        name: "2 Tessalonicenses",
        abbreviation: "2Ts",
        aliases: &[
            "2 tessalonicenses",
            "2tessalonicenses",
            "2 ts",
            "2ts",
            "2tes",
            "ii tessalonicenses",
            "ii ts",
        ],
    },
    CanonicalBook {
        id: 54,
        testament: 2,
        name: "1 Timóteo",
        abbreviation: "1Tm",
        aliases: &[
            "1 timoteo",
            "1timoteo",
            "1 tm",
            "1tm",
            "1tim",
            "i timoteo",
            "i tm",
        ],
    },
    CanonicalBook {
        id: 55,
        testament: 2,
        name: "2 Timóteo",
        abbreviation: "2Tm",
        aliases: &[
            "2 timoteo",
            "2timoteo",
            "2 tm",
            "2tm",
            "2tim",
            "ii timoteo",
            "ii tm",
        ],
    },
    CanonicalBook {
        id: 56,
        testament: 2,
        name: "Tito",
        abbreviation: "Tt",
        aliases: &["tito", "tt", "tit"],
    },
    CanonicalBook {
        id: 57,
        testament: 2,
        name: "Filemom",
        abbreviation: "Fm",
        aliases: &["filemom", "fm", "flm", "filem"],
    },
    CanonicalBook {
        id: 58,
        testament: 2,
        name: "Hebreus",
        abbreviation: "Hb",
        aliases: &["hebreus", "hb", "heb"],
    },
    CanonicalBook {
        id: 59,
        testament: 2,
        name: "Tiago",
        abbreviation: "Tg",
        aliases: &["tiago", "tg", "tia", "jas"],
    },
    CanonicalBook {
        id: 60,
        testament: 2,
        name: "1 Pedro",
        abbreviation: "1Pe",
        aliases: &[
            "1 pedro", "1pedro", "1 pe", "1pe", "1ped", "i pedro", "i pe",
        ],
    },
    CanonicalBook {
        id: 61,
        testament: 2,
        name: "2 Pedro",
        abbreviation: "2Pe",
        aliases: &[
            "2 pedro", "2pedro", "2 pe", "2pe", "2ped", "ii pedro", "ii pe",
        ],
    },
    CanonicalBook {
        id: 62,
        testament: 2,
        name: "1 João",
        abbreviation: "1Jo",
        aliases: &["1 joao", "1joao", "1 jo", "1jo", "i joao", "i jo"],
    },
    CanonicalBook {
        id: 63,
        testament: 2,
        name: "2 João",
        abbreviation: "2Jo",
        aliases: &["2 joao", "2joao", "2 jo", "2jo", "ii joao", "ii jo"],
    },
    CanonicalBook {
        id: 64,
        testament: 2,
        name: "3 João",
        abbreviation: "3Jo",
        aliases: &["3 joao", "3joao", "3 jo", "3jo", "iii joao", "iii jo"],
    },
    CanonicalBook {
        id: 65,
        testament: 2,
        name: "Judas",
        abbreviation: "Jd",
        aliases: &["judas", "jd", "jud"],
    },
    CanonicalBook {
        id: 66,
        testament: 2,
        name: "Apocalipse",
        abbreviation: "Ap",
        aliases: &["apocalipse", "ap", "apoc", "revelation", "rev"],
    },
];

/// Remove acentuação e converte para minúsculas.
pub fn normalize_string(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        let lower = c.to_lowercase();
        for lc in lower {
            match lc {
                'á' | 'à' | 'ã' | 'â' | 'ä' => out.push('a'),
                'é' | 'è' | 'ê' | 'ë' => out.push('e'),
                'í' | 'ì' | 'î' | 'ï' => out.push('i'),
                'ó' | 'ò' | 'õ' | 'ô' | 'ö' => out.push('o'),
                'ú' | 'ù' | 'û' | 'ü' => out.push('u'),
                'ç' => out.push('c'),
                _ => out.push(lc),
            }
        }
    }
    out
}

/// Encontra um livro canônico a partir de um nome ou abreviação.
pub fn find_canonical_book(input: &str) -> Option<&'static CanonicalBook> {
    let raw_trimmed = input.trim();
    if raw_trimmed.is_empty() {
        return None;
    }

    // Desambiguação entre Jó (id 18) e João (id 43):
    // "jó" com acento ou "job" -> Jó (id 18).
    // "jo" sem acento, "joao", "joão" -> João (id 43).
    if raw_trimmed.eq_ignore_ascii_case("job") || raw_trimmed == "jó" || raw_trimmed == "Jó" {
        return CANONICAL_BOOKS.iter().find(|b| b.id == 18);
    }
    if raw_trimmed.eq_ignore_ascii_case("jo") {
        return CANONICAL_BOOKS.iter().find(|b| b.id == 43);
    }

    let norm = normalize_string(raw_trimmed);

    // Busca exata por aliases normalizados
    for book in CANONICAL_BOOKS {
        for alias in book.aliases {
            if norm == *alias {
                return Some(book);
            }
        }
        if norm == normalize_string(book.name) || norm == normalize_string(book.abbreviation) {
            return Some(book);
        }
    }

    None
}

/// Obtém a abreviação canônica de um livro pelo ID (1 a 66).
pub fn get_book_abbreviation(book_id: i64) -> &'static str {
    CANONICAL_BOOKS
        .iter()
        .find(|b| b.id == book_id)
        .map(|b| b.abbreviation)
        .unwrap_or("")
}

/// Obtém o nome canônico de um livro pelo ID (1 a 66).
pub fn get_book_name(book_id: i64) -> &'static str {
    CANONICAL_BOOKS
        .iter()
        .find(|b| b.id == book_id)
        .map(|b| b.name)
        .unwrap_or("")
}

/// Interpreta uma referência textual bíblica nos padrões do português.
///
/// Exemplos suportados:
/// - `joao 3.16`
/// - `joão 3:16`
/// - `jo 3 16`
/// - `1 co 13:4-7`
/// - `Salmos 91` ou `sl 91` (capítulo isolado, foca no verso 1)
/// - `genesis 1:1`
pub fn parse_bible_reference(raw: &str) -> Option<ParsedReference> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Encontra o primeiro dígito que aparece após termos visto letras alfabéticas.
    // Em livros bíblicos (como "1 João", "2 Reis"), os números do livro ocorrem antes das letras.
    // Qualquer dígito após as letras marca o início do capítulo.
    let mut saw_alpha = false;
    let mut split_byte_idx = None;

    for (idx, c) in trimmed.char_indices() {
        if c.is_alphabetic() {
            saw_alpha = true;
        } else if saw_alpha && c.is_ascii_digit() {
            split_byte_idx = Some(idx);
            break;
        }
    }

    let split_pos = split_byte_idx?;
    let (book_part_raw, cv_part_raw) = trimmed.split_at(split_pos);
    let book_part = book_part_raw.trim();
    let chapter_verse_part = cv_part_raw.trim();

    if book_part.is_empty() || chapter_verse_part.is_empty() {
        return None;
    }

    let book = find_canonical_book(book_part)?;

    // Faz parse do capítulo, versículo e possível versão
    // Padrões de chapter_verse_part:
    // "3.16", "3:16", "3 16", "3:16-18", "91", "13:4-7", "91 ARA", "3:16 NVI", "13:4-7 arc"
    let cv_clean = chapter_verse_part.replace([':', '.', ','], " ");
    let mut tokens: Vec<&str> = cv_clean.split_whitespace().collect();

    if tokens.is_empty() {
        return None;
    }

    // Se o último token contiver apenas letras alfabéticas (ex: "ARA", "NVI", "ARC"), tratamos como versão bíblica
    let version = if tokens.len() >= 2
        && tokens
            .last()
            .is_some_and(|t| t.chars().all(|c| c.is_alphabetic()))
    {
        Some(tokens.pop().unwrap().to_uppercase())
    } else {
        None
    };

    if tokens.is_empty() {
        return None;
    }

    let chapter: i64 = tokens[0].parse().ok()?;

    let (verse_start, verse_end) = if tokens.len() >= 2 {
        let verse_token = tokens[1];
        if verse_token.contains('-') {
            let sub: Vec<&str> = verse_token.split('-').collect();
            let start = sub.first().and_then(|s| s.parse::<i64>().ok());
            let end = sub.get(1).and_then(|s| s.parse::<i64>().ok());
            (start, end.or(start))
        } else {
            let start = verse_token.parse::<i64>().ok();
            (start, start)
        }
    } else {
        // Apenas capítulo! (Ex: "Salmos 91" ou "Salmos 91 ARA") -> Foca no versículo 1
        (Some(1), Some(1))
    };

    Some(ParsedReference {
        book_id: book.id,
        book_name: book.name.to_string(),
        book_abbreviation: book.abbreviation.to_string(),
        chapter,
        verse_start,
        verse_end,
        version,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_various_reference_formats() {
        // joao 3.16
        let r1 = parse_bible_reference("joao 3.16").expect("parse joao 3.16");
        assert_eq!(r1.book_id, 43);
        assert_eq!(r1.book_abbreviation, "Jo");
        assert_eq!(r1.chapter, 3);
        assert_eq!(r1.verse_start, Some(16));
        assert_eq!(r1.verse_end, Some(16));

        // joão 3:16
        let r2 = parse_bible_reference("joão 3:16").expect("parse joão 3:16");
        assert_eq!(r2.book_id, 43);
        assert_eq!(r2.chapter, 3);
        assert_eq!(r2.verse_start, Some(16));

        // jo 3 16
        let r3 = parse_bible_reference("jo 3 16").expect("parse jo 3 16");
        assert_eq!(r3.book_id, 43);
        assert_eq!(r3.chapter, 3);
        assert_eq!(r3.verse_start, Some(16));

        // 1 co 13:4-7
        let r4 = parse_bible_reference("1 co 13:4-7").expect("parse 1 co 13:4-7");
        assert_eq!(r4.book_id, 46);
        assert_eq!(r4.book_abbreviation, "1Co");
        assert_eq!(r4.chapter, 13);
        assert_eq!(r4.verse_start, Some(4));
        assert_eq!(r4.verse_end, Some(7));

        // Salmos 91
        let r5 = parse_bible_reference("Salmos 91").expect("parse Salmos 91");
        assert_eq!(r5.book_id, 19);
        assert_eq!(r5.book_name, "Salmos");
        assert_eq!(r5.chapter, 91);
        assert_eq!(r5.verse_start, Some(1));

        // sl 23
        let r6 = parse_bible_reference("sl 23").expect("parse sl 23");
        assert_eq!(r6.book_id, 19);
        assert_eq!(r6.chapter, 23);
        assert_eq!(r6.verse_start, Some(1));

        // Gn 1:1
        let r7 = parse_bible_reference("Gn 1:1").expect("parse Gn 1:1");
        assert_eq!(r7.book_id, 1);
        assert_eq!(r7.chapter, 1);
        assert_eq!(r7.verse_start, Some(1));

        // Jó 1:1
        let r8 = parse_bible_reference("Jó 1:1").expect("parse Jó 1:1");
        assert_eq!(r8.book_id, 18);
        assert_eq!(r8.chapter, 1);

        // 1jo 1:9
        let r9 = parse_bible_reference("1jo 1:9").expect("parse 1jo 1:9");
        assert_eq!(r9.book_id, 62);
        assert_eq!(r9.book_abbreviation, "1Jo");
        assert_eq!(r9.chapter, 1);
        assert_eq!(r9.verse_start, Some(9));

        // 2 joao 1:5
        let r10 = parse_bible_reference("2 joao 1:5").expect("parse 2 joao 1:5");
        assert_eq!(r10.book_id, 63);

        // 3jo 1:2
        let r11 = parse_bible_reference("3jo 1:2").expect("parse 3jo 1:2");
        assert_eq!(r11.book_id, 64);

        // 1 corintios 13
        let r12 = parse_bible_reference("1 corintios 13").expect("parse 1 corintios 13");
        assert_eq!(r12.book_id, 46);
        assert_eq!(r12.chapter, 13);
        assert_eq!(r12.verse_start, Some(1));

        // apocalipse 22:20
        let r13 = parse_bible_reference("apocalipse 22:20").expect("parse apocalipse 22:20");
        assert_eq!(r13.book_id, 66);
        assert_eq!(r13.chapter, 22);
        assert_eq!(r13.verse_start, Some(20));
        assert_eq!(r13.version, None);

        // Salmos 91 ARA (com versão)
        let r14 = parse_bible_reference("Salmos 91 ARA").expect("parse Salmos 91 ARA");
        assert_eq!(r14.book_id, 19);
        assert_eq!(r14.chapter, 91);
        assert_eq!(r14.verse_start, Some(1));
        assert_eq!(r14.version, Some("ARA".to_string()));

        // joao 3:16 nvi (com versão e minúsculo)
        let r15 = parse_bible_reference("joao 3:16 nvi").expect("parse joao 3:16 nvi");
        assert_eq!(r15.book_id, 43);
        assert_eq!(r15.chapter, 3);
        assert_eq!(r15.verse_start, Some(16));
        assert_eq!(r15.version, Some("NVI".to_string()));

        // 1 co 13:4-7 arc
        let r16 = parse_bible_reference("1 co 13:4-7 arc").expect("parse 1 co 13:4-7 arc");
        assert_eq!(r16.book_id, 46);
        assert_eq!(r16.chapter, 13);
        assert_eq!(r16.verse_start, Some(4));
        assert_eq!(r16.verse_end, Some(7));
        assert_eq!(r16.version, Some("ARC".to_string()));

        // string inválida
        assert!(parse_bible_reference("texto qualquer sem sentido").is_none());
        assert!(parse_bible_reference("").is_none());
    }
}

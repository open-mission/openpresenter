use std::io::Read;

use flate2::read::GzDecoder;
use op_core::{OnlineLyricHit, Song, parse_raw_lyrics};
use serde::{Deserialize, Serialize};
use tracing::debug;

const HOLYRICS_SEARCH_URL: &str = "https://lyrics.holyrics.com.br/search.php";
const HOLYRICS_CDN_BASE: &str = "https://lyrics-cache.holyricscdn.com";
const HOLYRICS_HMAC_KEY: &[u8] = b"zjz8FXCFa8gE4jXebl7ETqd6IOrRoQZr";
const HOLYRICS_USER_AGENT: &str = "Holyrics Desktop v2.30.0";

const VAGALUME_BASE_URL: &str = "https://api.vagalume.com.br";
const VAGALUME_API_KEY: &str = "34db1ee757d414ed544882cb36bdfd6f";

#[derive(Deserialize)]
struct HolyricsSearchResponse {
    items: Option<Vec<HolyricsSearchItem>>,
}

#[derive(Deserialize)]
struct HolyricsSearchItem {
    id: String,
    title: String,
    artist: String,
}

#[derive(Deserialize)]
struct HolyricsSongPayload {
    title: Option<String>,
    artist: Option<String>,
    author: Option<String>,
    lyrics: Option<String>,
    key: Option<String>,
    bpm: Option<String>,
}

#[derive(Deserialize)]
struct VagalumeExcerptResponse {
    response: Option<VagalumeExcerptDocs>,
}

#[derive(Deserialize)]
struct VagalumeExcerptDocs {
    docs: Option<Vec<VagalumeDocItem>>,
}

#[derive(Deserialize)]
struct VagalumeDocItem {
    id: String,
    title: String,
    band: Option<String>,
}

#[derive(Deserialize)]
struct VagalumeMusResponse {
    art: Option<VagalumeArtInfo>,
    mus: Option<Vec<VagalumeMusItem>>,
}

#[derive(Deserialize)]
struct VagalumeArtInfo {
    name: Option<String>,
}

#[derive(Deserialize)]
struct VagalumeMusItem {
    name: Option<String>,
    text: Option<String>,
}

/// Resposta retornada ao buscar ou importar letra de provedor online.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchedOnlineSong {
    pub title: String,
    pub artist: String,
    pub composer: Option<String>,
    pub song_key: Option<String>,
    pub bpm: Option<i32>,
    pub lyrics_raw: String,
}

impl FetchedOnlineSong {
    pub fn into_song(self) -> Song {
        let id = format!("song-{}", uuid_simple());
        let (sections, _) = parse_raw_lyrics(&self.lyrics_raw);
        Song {
            id,
            title: self.title,
            artist: self.artist,
            album: None,
            composer: self.composer,
            song_key: self.song_key,
            bpm: self.bpm,
            sections,
            lyrics_raw: self.lyrics_raw,
        }
    }
}

/// Busca em paralelo nos provedores online (Holyrics e Vagalume).
pub async fn search_online(client: &reqwest::Client, query: &str) -> Vec<OnlineLyricHit> {
    let clean = query.trim();
    if clean.is_empty() {
        return Vec::new();
    }

    let holyrics_fut = search_holyrics(client, clean);
    let vagalume_fut = search_vagalume(client, clean);

    let (holyrics_res, vagalume_res) = tokio::join!(holyrics_fut, vagalume_fut);

    let mut results = Vec::new();
    // Prioriza Holyrics para repertório de igrejas
    results.extend(holyrics_res);
    results.extend(vagalume_res);
    results
}

async fn search_holyrics(client: &reqwest::Client, query: &str) -> Vec<OnlineLyricHit> {
    let res = match client
        .get(HOLYRICS_SEARCH_URL)
        .header("User-Agent", HOLYRICS_USER_AGENT)
        .header("lang", "pt")
        .header("device_type", "pc")
        .query(&[
            ("q", query),
            ("t", "1"),
            ("a", "1"),
            ("l", "1"),
            ("uid", "987654321"),
            ("sl", "1"),
        ])
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            debug!("Busca no Holyrics falhou: {err}");
            return Vec::new();
        }
    };

    if !res.status().is_success() {
        return Vec::new();
    }

    let data: HolyricsSearchResponse = match res.json().await {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    data.items
        .unwrap_or_default()
        .into_iter()
        .map(|item| OnlineLyricHit {
            track_name: item.title,
            artist_name: item.artist,
            album_name: None,
            provider: Some("holyrics".into()),
            provider_id: Some(item.id),
        })
        .collect()
}

async fn search_vagalume(client: &reqwest::Client, query: &str) -> Vec<OnlineLyricHit> {
    let url = format!("{}/search.excerpt", VAGALUME_BASE_URL);
    let res = match client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .query(&[("q", query), ("limit", "15")])
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            debug!("Busca no Vagalume falhou: {err}");
            return Vec::new();
        }
    };

    if !res.status().is_success() {
        return Vec::new();
    }

    let data: VagalumeExcerptResponse = match res.json().await {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    let docs = data.response.and_then(|r| r.docs).unwrap_or_default();
    docs.into_iter()
        .map(|doc| OnlineLyricHit {
            track_name: doc.title,
            artist_name: doc.band.unwrap_or_default(),
            album_name: None,
            provider: Some("vagalume".into()),
            provider_id: Some(doc.id),
        })
        .collect()
}

/// Baixa e normaliza a letra do provedor online indicado.
pub async fn fetch_online_song(
    client: &reqwest::Client,
    provider: &str,
    provider_id: &str,
) -> anyhow::Result<FetchedOnlineSong> {
    match provider {
        "holyrics" => fetch_holyrics(client, provider_id).await,
        "vagalume" => fetch_vagalume(client, provider_id).await,
        other => anyhow::bail!("provedor não suportado: {}", other),
    }
}

async fn fetch_holyrics(
    client: &reqwest::Client,
    song_id: &str,
) -> anyhow::Result<FetchedOnlineSong> {
    let key = format!("{};S4", song_id.trim().to_lowercase());
    let step1 = md5_hex(key.as_bytes());
    let step2 = hmac_sha512_hex(HOLYRICS_HMAC_KEY, step1.as_bytes());
    let final_hash = md5_hex(step2.as_bytes());
    let prefix = &final_hash[..3];

    let url = format!("{}/{}/{}.txt.gz", HOLYRICS_CDN_BASE, prefix, final_hash);
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("música não encontrada no Holyrics");
    }

    let gz_bytes = resp.bytes().await?;
    let mut decoder = GzDecoder::new(&gz_bytes[..]);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;

    let payload: HolyricsSongPayload = serde_json::from_str(&decompressed)?;

    let title = payload.title.unwrap_or_default();
    let artist = payload.artist.unwrap_or_default();
    let composer = payload.author.filter(|s| !s.trim().is_empty());
    let song_key = payload.key.filter(|s| !s.trim().is_empty());
    let bpm = payload.bpm.and_then(|s| s.parse::<i32>().ok());

    let raw_lyrics = payload
        .lyrics
        .unwrap_or_default()
        .replace("\n.\n", "\n\n")
        .trim()
        .to_string();

    Ok(FetchedOnlineSong {
        title,
        artist,
        composer,
        song_key,
        bpm,
        lyrics_raw: raw_lyrics,
    })
}

async fn fetch_vagalume(
    client: &reqwest::Client,
    song_id: &str,
) -> anyhow::Result<FetchedOnlineSong> {
    let url = format!("{}/search.php", VAGALUME_BASE_URL);
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .query(&[("musid", song_id), ("apikey", VAGALUME_API_KEY)])
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("música não encontrada no Vagalume");
    }

    let data: VagalumeMusResponse = resp.json().await?;
    let artist = data.art.and_then(|a| a.name).unwrap_or_default();
    let mus = data
        .mus
        .and_then(|m| m.into_iter().next())
        .ok_or_else(|| anyhow::anyhow!("letra não encontrada no Vagalume"))?;

    let title = mus.name.unwrap_or_default();
    let text = mus.text.unwrap_or_default().trim().to_string();

    Ok(FetchedOnlineSong {
        title,
        artist,
        composer: None,
        song_key: None,
        bpm: None,
        lyrics_raw: text,
    })
}

fn hmac_sha512_hex(key: &[u8], data: &[u8]) -> String {
    let s_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA512, key);
    let tag = ring::hmac::sign(&s_key, data);
    let mut out = String::with_capacity(128);
    for b in tag.as_ref() {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

fn md5_hex(input: &[u8]) -> String {
    let digest = md5_digest(input);
    let mut out = String::with_capacity(32);
    for b in digest {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

fn md5_digest(initial_msg: &[u8]) -> [u8; 16] {
    let mut msg = initial_msg.to_vec();
    let bit_len = (initial_msg.len() as u64) * 8;

    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());

    let mut a: u32 = 0x67452301;
    let mut b: u32 = 0xefcdab89;
    let mut c: u32 = 0x98badcfe;
    let mut d: u32 = 0x10325476;

    let s = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    let k = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 16];
        for (i, w_val) in w.iter_mut().enumerate() {
            let start = i * 4;
            *w_val = u32::from_le_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ]);
        }

        let mut aa = a;
        let mut bb = b;
        let mut cc = c;
        let mut dd = d;

        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((bb & cc) | (!bb & dd), i),
                16..=31 => ((dd & bb) | (!dd & cc), (5 * i + 1) % 16),
                32..=47 => (bb ^ cc ^ dd, (3 * i + 5) % 16),
                _ => (cc ^ (bb | !dd), (7 * i) % 16),
            };

            let temp = dd;
            dd = cc;
            cc = bb;
            bb = bb.wrapping_add(
                aa.wrapping_add(f)
                    .wrapping_add(k[i])
                    .wrapping_add(w[g])
                    .rotate_left(s[i]),
            );
            aa = temp;
        }

        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(&a.to_le_bytes());
    result[4..8].copy_from_slice(&b.to_le_bytes());
    result[8..12].copy_from_slice(&c.to_le_bytes());
    result[12..16].copy_from_slice(&d.to_le_bytes());
    result
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", nanos)
}

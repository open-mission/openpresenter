CREATE TABLE IF NOT EXISTS songs (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    artist TEXT NOT NULL DEFAULT '',
    album TEXT NOT NULL DEFAULT '',
    composer TEXT NOT NULL DEFAULT '',
    song_key TEXT NOT NULL DEFAULT '',
    bpm INTEGER,
    lyrics_raw TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS song_sections (
    song_id TEXT NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    section_type TEXT,
    label TEXT NOT NULL,
    PRIMARY KEY (song_id, position)
);

CREATE TABLE IF NOT EXISTS song_lyrics (
    song_id TEXT NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    section_position INTEGER NOT NULL,
    line_position INTEGER NOT NULL,
    line TEXT NOT NULL,
    PRIMARY KEY (song_id, section_position, line_position)
);

CREATE VIRTUAL TABLE IF NOT EXISTS songs_fts USING fts5(
    song_id UNINDEXED,
    title,
    artist,
    lyrics,
    tokenize = 'unicode61 remove_diacritics 2'
);

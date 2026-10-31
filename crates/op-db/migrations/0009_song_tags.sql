CREATE TABLE IF NOT EXISTS song_tags (
    song_id TEXT NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    tag TEXT NOT NULL,
    PRIMARY KEY (song_id, tag)
);

CREATE INDEX IF NOT EXISTS idx_song_tags_tag ON song_tags(tag);
CREATE INDEX IF NOT EXISTS idx_song_tags_song_id ON song_tags(song_id);

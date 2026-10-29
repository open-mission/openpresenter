-- 0004_setlists.sql
-- Tabela para múltiplas setlists com papéis de parede, descrição e itens serializados em JSON.

CREATE TABLE IF NOT EXISTS setlists (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    wallpaper_path TEXT,
    wallpaper_mode TEXT DEFAULT 'cover',
    items_json TEXT NOT NULL DEFAULT '[]',
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_setlists_is_active ON setlists(is_active);
CREATE INDEX IF NOT EXISTS idx_setlists_updated_at ON setlists(updated_at DESC);

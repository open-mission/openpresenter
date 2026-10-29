-- 0005_media_folders.sql
-- Tabela para gerenciar pastas locais de mídias mapeadas pelo usuário (zero-copy).

CREATE TABLE IF NOT EXISTS media_folders (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    kind TEXT NOT NULL DEFAULT 'images',
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_media_folders_kind ON media_folders(kind);

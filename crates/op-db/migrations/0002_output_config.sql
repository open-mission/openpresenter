CREATE TABLE output_config (
    output       TEXT PRIMARY KEY CHECK (output IN ('public', 'stage')),
    enabled      INTEGER NOT NULL CHECK (enabled IN (0, 1)),
    monitor_key  TEXT NOT NULL,
    monitor_name TEXT NOT NULL DEFAULT '',
    mode         TEXT NOT NULL CHECK (mode IN ('auto', 'windowed', 'maximized', 'fullscreen')),
    updated_at   TEXT NOT NULL
);

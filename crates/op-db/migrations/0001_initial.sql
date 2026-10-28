CREATE TABLE app_meta (
    id           INTEGER PRIMARY KEY CHECK (id = 1),
    app_version  TEXT NOT NULL,
    first_run_at TEXT NOT NULL,
    last_run_at  TEXT NOT NULL,
    theme        TEXT NOT NULL DEFAULT 'system'
                 CHECK (theme IN ('system', 'light', 'dark')),
    locale       TEXT NOT NULL DEFAULT 'pt-BR',
    updated_at   TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS todo (
    id                   INTEGER PRIMARY KEY,
    completed            BOOL DEFAULT FALSE,
    text                 TEXT NOT NULL,
    created_at           DATE DEFAULT (datetime('now','localtime')),
    updated_at           DATE DEFAULT (datetime('now','localtime'))
);

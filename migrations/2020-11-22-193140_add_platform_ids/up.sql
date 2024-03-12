CREATE TABLE platform_id_links (
    id INTEGER PRIMARY KEY NOT NULL,
    platform TEXT NOT NULL,
    platform_id TEXT NOT NULL,
    account_id INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE
);

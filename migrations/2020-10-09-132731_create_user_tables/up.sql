CREATE TABLE accounts (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    displayname TEXT NOT NULL,
    xp INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE friendships (
    id INTEGER PRIMARY KEY NOT NULL,
    account_id_1 INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    account_id_2 INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,

    favorite_1 BOOLEAN NOT NULL DEFAULT 0,
    favorite_2 BOOLEAN NOT NULL DEFAULT 0,

    initiated_by_1 BOOLEAN NOT NULL,
    
    initiated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    accepted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE friendship_lookup (
    account_id_1 INTEGER NOT NULL,
    account_id_2 INTEGER NOT NULL,
    friendship_id INTEGER NOT NULL REFERENCES friendships(id) ON DELETE CASCADE,

    PRIMARY KEY (account_id_1, account_id_2)
);


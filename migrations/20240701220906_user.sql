-- Store data about users
CREATE TABLE IF NOT EXISTS Users (
    discord_id INTEGER PRIMARY KEY NOT NULL,
    family_fame INTEGER NOT NULL,
    value_pack INTEGER NOT NULL,
    merchant_ring INTEGER NOT NULL,
    cron_cost INTEGER NOT NULL,
    region INTEGER NOT NULL
);

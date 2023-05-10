CREATE TABLE events (
    id TEXT PRIMARY KEY NOT NULL,
    ts TEXT NOT NULL DEFAULT datetime(),
    data TEXT NOT NULL
);

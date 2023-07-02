-- Your SQL goes here
CREATE TABLE "source_files" (
    path TEXT PRIMARY KEY NOT NULL,
    hash TEXT NOT NULL,
    size INTEGER NOT NULL,
    date_created TEXT NOT NULL
);

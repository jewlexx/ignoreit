-- Add migration script here
CREATE TABLE gitignores (
    key TEXT NOT NULL PRIMARY KEY,
    contents TEXT NOT NULL,
    file_name TEXT NOT NULL,
    name TEXT NOT NULL
);
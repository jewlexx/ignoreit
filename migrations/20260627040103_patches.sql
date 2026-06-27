-- Add migration script here
ALTER TABLE gitignores
ADD COLUMN is_patch INTEGER NOT NULL;
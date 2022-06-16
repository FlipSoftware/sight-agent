-- Add up migration script here
CREATE TABLE IF NOT EXISTS reply (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    corresponding_question integer REFERENCES kb
);
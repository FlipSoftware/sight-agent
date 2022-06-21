-- Add up migration script here
ALTER TABLE reply
ADD COLUMN account_id serial;
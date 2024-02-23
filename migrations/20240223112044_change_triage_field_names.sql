-- Add migration script here

ALTER TABLE triage
RENAME COLUMN moderator_channel_id TO mod_channel_id;

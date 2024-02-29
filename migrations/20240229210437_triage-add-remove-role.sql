-- Add migration script here

ALTER TABLE triage
ADD COLUMN remove_role_id bigint;

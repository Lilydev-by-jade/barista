-- Add migration script here

ALTER TABLE triage
RENAME COLUMN member_role_id TO access_role_id;
